use std::rc::Weak;
use std::{cell::RefCell, marker::PhantomData, pin::Pin, rc::Rc};

use frender_csr::{
    CsrElement, UiHandle, UnmountedUiHandle,
    experimental::{
        self, HtmlRenderContext, PinnedRenderStateKind, PinnedRenderStateKindPollRender,
        PinnedStateOfKind, RenderHtml, RenderInitPinned, UnpinnedRenderStateKind,
        UnpinnedRenderStateKindPollRender, UnpinnedStateOfKind, UnpinnedUiHandleOfKind,
        behaviors::{NodeRenderSelf, NodeWithRenderContextAfterSelf as _},
    },
    render::RenderContext as _,
};

use crate::weak_vec1::WeakVec1;

use self::{
    render_states::MountState,
    state::{PinnedStates, ReactiveStates, State, StatesOfKind, UiHandles, UnmountedUiHandles},
};

use super::super::{AllStates, States};
use super::{MapItemToElement, SyncedCollectionToElement};

mod render_states;
mod state;

fn weak_is_of_rc<T: ?Sized, U: ?Sized>(weak: &Weak<T>, rc: &Rc<U>) -> bool {
    std::ptr::addr_eq(Weak::as_ptr(weak), Rc::as_ptr(rc))
}

impl AllStates {
    fn put_rc_states_with_old_key_hint<S: States + 'static>(
        &mut self,
        old_key_hint: Key,
        rc: &Rc<RefCell<S>>,
    ) -> Key {
        self.0
            .put_into_old_available_or_append(old_key_hint, rc, weak_is_of_rc, |rc| {
                Rc::downgrade(rc) as _
            })
    }
}

impl<T: ?Sized> WeakVec1<T> {
    fn contains<U: ?Sized>(&self, v: &RcWithKey<U>) -> bool {
        let weak = match v.key {
            Key::STACK => self.0.as_ref(),
            Key(i) => self.1.get(i).and_then(Option::as_ref),
        };
        weak.map_or(false, |weak| weak_is_of_rc(weak, &v.rc))
    }

    fn put_into_old_available_or_append<R: ?Sized>(
        &mut self,
        old_key: Key,
        rc: &R,
        // We require Copy because we can.
        weak_has_same_addr_of: impl Copy + FnOnce(&Weak<T>, &R) -> bool,
        to_weak: impl Copy + FnOnce(&R) -> Weak<T>,
    ) -> Key {
        let stack = &mut self.0;

        match old_key {
            Key::STACK => {}
            Key(index) => match self.1.get_mut(index) {
                Some(weak) => {
                    if put_into_available_weak(weak, rc, weak_has_same_addr_of, to_weak) {
                        return old_key;
                    }
                }
                None => {}
            },
        }

        if put_into_available_weak(stack, rc, weak_has_same_addr_of, to_weak) {
            return Key::STACK;
        }

        let i = self.1.len();
        assert_ne!(i, Key::STACK.0);
        self.1.push(Some(to_weak(rc)));
        Key(i)
    }
}

fn weak_is_empty<T: ?Sized>(v: &Weak<T>) -> bool {
    v.strong_count() == 0
}

/// Returns `true` if `v` is available.
fn put_into_available_weak<T: ?Sized, R: ?Sized>(
    v: &mut Option<Weak<T>>,
    rc: &R,
    weak_has_same_addr_of: impl FnOnce(&Weak<T>, &R) -> bool,
    to_weak: impl FnOnce(&R) -> Weak<T>,
) -> bool {
    match v {
        None => {}
        Some(v) if weak_is_empty(v) => {}
        Some(v) if weak_has_same_addr_of(v, rc) => {
            // the weak matched the rc so it doesn't need to be updated
            return true;
        }
        _ => {
            // the weak is alive and doesn't match the rc
            return false;
        }
    }

    *v = Some(to_weak(rc));

    true
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Key(usize);

impl Key {
    const STACK: Self = Self(usize::MAX);
}

struct RcWithKey<T: ?Sized> {
    rc: Rc<T>,
    key: Key,
}

enum Never {}
pub struct Kind<K>(Never, PhantomData<K>);

impl<K: UnpinnedRenderStateKind> UnpinnedRenderStateKind for Kind<K> {
    type UnpinnedUiHandle<R: RenderHtml + ?Sized> = UiHandles<
        R::CursorPlaceholder,
        K::UnpinnedUiHandle<R>,
        <K::UnpinnedUiHandle<R> as UiHandle<R>>::Unmounted,
        K::UnpinnedState<R>,
    >;

    type UnpinnedState<R: RenderHtml + ?Sized> = ReactiveStates<K::UnpinnedState<R>>;
}

impl<K: UnpinnedRenderStateKindPollRender> UnpinnedRenderStateKindPollRender for Kind<K> {
    fn unpinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        state: &mut Self::UnpinnedState<R>,
        ui_handle: &mut Self::UnpinnedUiHandle<R>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        let _ = state; // TODO: debug assert state and ui_handle are the same rc
        ui_handle.poll_render::<K, R>(renderer, cx)
    }
}

impl<K: UnpinnedRenderStateKind> PinnedRenderStateKind for Kind<K> {
    type PinnedUiHandle<R: RenderHtml + ?Sized> =
        <Self as UnpinnedRenderStateKind>::UnpinnedUiHandle<R>;
    type PinnedState<R: RenderHtml + ?Sized> = PinnedStates<K::UnpinnedState<R>>;
}

impl<K: UnpinnedRenderStateKindPollRender> PinnedRenderStateKindPollRender for Kind<K> {
    fn pinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        state: Pin<&mut Self::PinnedState<R>>,
        ui_handle: &mut Self::PinnedUiHandle<R>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        Self::unpinned_poll_render(renderer, state.get_mut().as_mut_states(), ui_handle, cx)
    }
}

pub struct RenderInit<'a, ES: Iterator, F: MapItemToElement<ES::Item>>(
    SyncedCollectionToElement<'a, ES, F>,
);

impl<'a, ES: Iterator, F: MapItemToElement<ES::Item>, Ctx: ?Sized + HtmlRenderContext>
    RenderInitPinned<
        &mut Ctx,
        PinnedStateOfKind<Ctx::Renderer, Kind<<F::ItemToElement as CsrElement>::RenderStateKind>>,
    > for RenderInit<'a, ES, F>
where
    F::ItemToElement: CsrElement,
    // TODO: make this implied in RenderStateKind, or make RenderState and UnpinnedRenderState 'static
    <F::ItemToElement as CsrElement>::RenderStateKind: 'static,
{
    type Output = UnpinnedUiHandleOfKind<
        Ctx::Renderer,
        Kind<<F::ItemToElement as CsrElement>::RenderStateKind>,
    >;

    fn render_init_pinned(
        self,
        render_context: &mut Ctx,
        state: Pin<
            &mut PinnedStateOfKind<
                Ctx::Renderer,
                Kind<<F::ItemToElement as CsrElement>::RenderStateKind>,
            >,
        >,
    ) -> Self::Output {
        let (state_init, ui_handle) = self.0.unpinned_render_init(render_context);
        state.get_mut().set(state_init);
        ui_handle
    }
}

impl<'a, ES: Iterator, F: MapItemToElement<ES::Item>> CsrElement
    for SyncedCollectionToElement<'a, ES, F>
where
    F::ItemToElement: CsrElement,
    // TODO: make this implied in RenderStateKind, or make RenderState and UnpinnedRenderState 'static
    <F::ItemToElement as CsrElement>::RenderStateKind: 'static,
{
    type RenderStateKind = Kind<<F::ItemToElement as CsrElement>::RenderStateKind>;
    type PinnedRenderInit<R: ?Sized + RenderHtml> = RenderInit<'a, ES, F>;

    fn pinned_render_init<Renderer: ?Sized + RenderHtml>(
        //
        self,
        _: &mut Renderer,
    ) -> (
        //
        experimental::PinnedStateOfKind<Renderer, Self::RenderStateKind>,
        Self::PinnedRenderInit<Renderer>,
    ) {
        (PinnedStates::DUMMY, RenderInit(self))
    }

    fn pinned_render_init_by_reusing<Ctx: ?Sized + HtmlRenderContext>(
        self,
        render_context: &mut Ctx,
        reused_state: Pin<
            &mut experimental::PinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>,
        >,
        unmounted_ui_handle: experimental::PinnedUnmountedUiHandleOfKind<
            Ctx::Renderer,
            Self::RenderStateKind,
        >,
    ) -> experimental::PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        self.unpinned_render_init_by_reusing(
            render_context,
            reused_state.get_mut().as_mut_states(),
            unmounted_ui_handle,
        )
    }

    fn pinned_render_update<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        state: Pin<&mut experimental::PinnedStateOfKind<Renderer, Self::RenderStateKind>>,
        ui_handle: &mut experimental::PinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    ) {
        self.unpinned_render_update(renderer, state.get_mut().as_mut_states(), ui_handle)
    }

    fn unpinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
    ) -> (
        //
        UnpinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>,
        UnpinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>,
    ) {
        let Self {
            all_states,
            items,
            mut f,
        } = self;

        let cpa = render_context.map_mut_render_context(NodeRenderSelf::render_self);

        let ui_handles = items
            .map(|item| {
                let element = f.map_item_to_element(item);
                State::from_render_states(element.unpinned_render_init(render_context))
            })
            .collect::<Vec<_>>();

        let cpb = render_context.map_mut_render_context(NodeRenderSelf::render_self);

        let ui_handles = Rc::new(RefCell::new(self::state::States::new(ui_handles)));

        let key = {
            let all_states = &mut *all_states.borrow_mut();
            all_states.put_rc_states_with_old_key_hint(Key::STACK, &ui_handles)
        };

        (
            ReactiveStates(RcWithKey {
                rc: ui_handles.clone(),
                key,
            }),
            UiHandles {
                ui_handles,
                cursor_placeholders: [cpa, cpb],
            },
        )
    }

    fn unpinned_render_init_by_reusing<Ctx: ?Sized + HtmlRenderContext>(
        self,
        render_context: &mut Ctx,
        ReactiveStates(rc_with_key): &mut UnpinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>,
        UnmountedUiHandles {
            ui_handles,
            cursor_placeholders: [cpa, cpb],
        }: experimental::UnpinnedUnmountedUiHandleOfKind<
            Ctx::Renderer,
            Self::RenderStateKind,
        >,
    ) -> UnpinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        let Self {
            all_states,
            items,
            mut f,
        } = self;

        debug_assert!(std::ptr::addr_eq(
            Rc::as_ptr(&rc_with_key.rc),
            Rc::as_ptr(&ui_handles)
        ));

        let cpa = render_context.map_mut_render_context(|render_context| cpa.mount(render_context));

        {
            let ui_handle = &mut *ui_handles.borrow_mut();
            ui_handle.ready_to_unmount_count = 0;
            ui_handle.all_outdated = false;

            let states = &mut ui_handle.states;

            let mut items = items.fuse();

            states.retain_mut(|state| {
                if let Some(item) = items.next() {
                    match state.take() {
                        State::BeforeMounted => false,
                        State::Mounted { .. } => unreachable!(), // UnmountedUiHandles must have been unmounted
                        State::Unmounted {
                            state: mut reused_state,
                            ui_handle,
                        } => {
                            let element = f.map_item_to_element(item);
                            let ui_handle = element.unpinned_render_init_by_reusing(
                                render_context,
                                &mut reused_state,
                                ui_handle,
                            );

                            *state = State::Mounted {
                                state: reused_state,
                                ui_handle,
                                mount_state: MountState::MountedAndUpToDate,
                            };

                            true
                        }
                    }
                } else {
                    false
                }
            });

            // there might be remaining items
            states.extend(items.map(|item| {
                let element = f.map_item_to_element(item);
                State::from_render_states(element.unpinned_render_init(render_context))
            }));
        }

        let cpb = render_context.map_mut_render_context(|render_context| cpb.mount(render_context));

        let old_key_hint = rc_with_key.key;

        let key = {
            let all_states = &mut *all_states.borrow_mut();
            all_states.put_rc_states_with_old_key_hint(old_key_hint, &ui_handles)
        };

        *rc_with_key = RcWithKey {
            rc: ui_handles.clone(),
            key,
        };
        UiHandles {
            ui_handles,
            cursor_placeholders: [cpa, cpb],
        }
    }

    fn unpinned_render_update<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        states: &mut UnpinnedStateOfKind<Renderer, Self::RenderStateKind>,
        UiHandles {
            ui_handles,
            cursor_placeholders: [cpa, cpb],
        }: &mut UnpinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    ) {
        cpa.with_render_context_after_self(renderer, |render_context| {
            self.render_update(render_context, states, ui_handles);
            cpb.check_and_move_cursor(render_context);
        })
    }
}

impl<'a, ES: Iterator, F: MapItemToElement<ES::Item>> SyncedCollectionToElement<'a, ES, F>
where
    F::ItemToElement: CsrElement,
    // TODO: make this implied in RenderStateKind, or make RenderState and UnpinnedRenderState 'static
    <F::ItemToElement as CsrElement>::RenderStateKind: 'static,
{
    fn render_update<Renderer: ?Sized + RenderHtml>(
        self,
        render_context: &mut Renderer::RenderContext<'_>,
        ReactiveStates(rc_with_key): &mut ReactiveStates<
            UnpinnedStateOfKind<Renderer, <F::ItemToElement as CsrElement>::RenderStateKind>,
        >,
        ui_handles: &mut Rc<
            RefCell<StatesOfKind<<F::ItemToElement as CsrElement>::RenderStateKind, Renderer>>,
        >,
    ) {
        assert!(std::ptr::addr_eq(
            Rc::as_ptr(&rc_with_key.rc),
            Rc::as_ptr(ui_handles)
        ));

        if self.all_states.borrow().0.contains(rc_with_key) {
            // render_states has been properly synced
            let states = &mut *ui_handles.borrow_mut();

            let all_outdated = std::mem::take(&mut states.all_outdated);

            let render_states = states.clean(render_context.renderer_mut());

            let mut render_states = render_states.iter_mut();
            let mut elements = self.items;
            let mut f = self.f;

            let zip = render_states.by_ref().zip(elements.by_ref());

            if all_outdated {
                zip.for_each(|(state, el)| {
                    state.force_render_with_but_trust_position(
                        f.map_item_to_element(el),
                        render_context,
                    )
                });
            } else {
                // only update outdated elements
                zip.for_each(|(state, el): (&mut _, _)| {
                    state.render_init_or_update_with(
                        || f.map_item_to_element(el),
                        render_context,
                        |get_element, state, ui_handle, mount_state, render_context| {
                            enum SimpleMountState {
                                UpToDate,
                                Outdated,
                                OutdatedAndMoved,
                            }

                            let (simple_mount_state, cursor_should_skip) = match mount_state {
                                MountState::MountedAndUpToDate => {
                                    (SimpleMountState::UpToDate, false)
                                }
                                MountState::MountedAndUpToDateButPreviousWasSkipped => {
                                    (SimpleMountState::UpToDate, true)
                                }
                                MountState::Outdated => (SimpleMountState::Outdated, false),
                                MountState::OutdatedAndPreviousWasSkipped => {
                                    (SimpleMountState::Outdated, true)
                                }
                                MountState::OutdatedAndMoved => {
                                    (SimpleMountState::OutdatedAndMoved, false)
                                }
                            };

                            *mount_state = MountState::MountedAndUpToDate;

                            if cursor_should_skip {
                                render_context.mark_cursor_skipped()
                            }

                            match simple_mount_state {
                                SimpleMountState::UpToDate => {
                                    ui_handle.check_and_move_cursor(render_context);
                                    return;
                                }
                                SimpleMountState::Outdated => {}
                                SimpleMountState::OutdatedAndMoved => {
                                    ui_handle.reposition(render_context)
                                }
                            }

                            get_element().unpinned_render_update(
                                render_context.renderer_mut(),
                                state,
                                ui_handle,
                            );
                        },
                    )
                })
            }

            assert_eq!(render_states.len(), 0, "too many render states");
            assert!(elements.next().is_none(), "too many elements");
        } else {
            // let RcWithKey { rc: _, key } = rc_with_key;

            // the states are outdated
            let states = &mut *ui_handles.borrow_mut();

            // It should be set to false when finished.
            // We can assume all_outdated=true in this branch so we set it earlier.
            states.all_outdated = false;

            let real_len = states.real_len();
            let (mounted, unmounted) = states.states.split_at_mut(real_len);

            let mut elements = self.items;
            let mut f = self.f;

            let mut unprocessed_mounted = 0;
            for mounted_state in mounted.iter_mut() {
                if let Some(item) = elements.next() {
                    let element = f.map_item_to_element(item);

                    mounted_state.force_render_with_but_trust_position(element, render_context);
                } else {
                    unprocessed_mounted = mounted.len() + 1;
                    break;
                }
            }

            if unprocessed_mounted == 0 {
                // there might be remaining items

                // shadow
                #[allow(unused_variables)]
                let unprocessed_mounted = ();
                #[allow(unused_variables)]
                let mounted = ();

                let mut unprocessed_unmounted = 0;
                for unmounted_state in unmounted.iter_mut() {
                    if let Some(item) = elements.next() {
                        let element = f.map_item_to_element(item);

                        // the position info should be correct but
                        // it should be marked as moved.
                        // So we just reposition the ui handle.
                        unmounted_state.force_render_with(element, render_context);
                    } else {
                        unprocessed_unmounted = unmounted.len() + 1;
                        break;
                    }
                }

                if unprocessed_unmounted == 0 {
                    // there might be remaining items
                    states.ready_to_unmount_count = 0;
                    states.states.extend(elements.map(|el| {
                        State::from_render_states(
                            f.map_item_to_element(el)
                                .unpinned_render_init(render_context),
                        )
                    }))
                } else {
                    // items are drained
                    states.ready_to_unmount_count = unprocessed_unmounted;
                }
            } else {
                // items are drained
                states.ready_to_unmount_count += unprocessed_mounted;
            }

            states.clean(render_context.renderer_mut());
        }
    }
}
