//! The problem is how to deal with the following two cases with one algorithm:
//!
//! - Case I : `1 2 3 4 5 6 7        => 1 *4* 2 3 4 5 6 7`
//! - Case II: `1 2 3 4 5 6 7 8 9... => 1  4  5 6 7 8 9 ... *2 3*`
//!
//! In case I, it's more performant to just move *4* to after 1
//! (reposition *4* and go on as normal).
//!
//! In case II, it's more performant to move *2 3* to the end.
//! (mark *2 3* as MightUnmount and go on as normal.
//! When *2* is met, reposition *2*.
//! When *3* is met, reposition *3*.).
//!
//! The problem is: while iterating elements, we meet *1* followed by *4*.
//! We can't decide this is case I or case II.
//!
//! The current algorithm is: If there were more elements between 1 and 4 than the elements after 4,
//! we assume this is case I. Otherwise, we assume it's case II.
//! With this algorithm, the above case I will be assumed as case II.
//! But we just force_positioned one more element which is ok.
//!
//! # Implementation details
//!
//! The render state [`UnpinnedState`](UnpinnedRenderStateKind::UnpinnedState) might NOT get dropped when unmounted.
//! [`StateUnmount::state_unmount`] will always run when unmounted.

use std::{cmp::Ordering, hash::Hash, marker::PhantomData, pin::Pin, task::Poll};

use indexmap::{map::Entry, IndexMap};

use frender_html::{
    dom::{
        render::RenderContext,
        ui_handle::{UiHandle, UnmountedUiHandle},
    },
    experimental::{self, UnpinnedRenderStateKind, UnpinnedRenderStateKindPollRender},
    ui_handles::CursorPlaceholdersSurrounded,
    CsrElement, RenderHtml, StateUnmount,
};

use crate::{DefaultAlgorithm, Keyed};

use super::KeyedElementsAlgorithm;

struct StatedUiHandle<U> {
    // The following fields could have a smaller representation like `usize`.
    order: usize,
    update_state: UpdateState,

    // TODO: We rely on in-place collect <https://doc.rust-lang.org/src/alloc/vec/in_place_collect.rs.html>
    // since in current implementation MountedUiHandle and UnmountedUiHandle are the same type.
    //
    // This might be implemented with UiHandleMaybe<M, U> to avoid re-alloc when unmount and mount
    // with the cost of larger size and some runtime overhead.
    //
    // This might be optimized with union.
    ui_handle: U,
}

impl<U> StatedUiHandle<U> {
    fn map_ui_handle<T>(self, f: impl FnOnce(U) -> T) -> StatedUiHandle<T> {
        let Self {
            order,
            update_state,
            ui_handle,
        } = self;
        StatedUiHandle {
            order,
            update_state,
            ui_handle: f(ui_handle),
        }
    }

    fn sort(this: &mut [Self]) {
        let mut cur = 0;

        while let Some(cur_u) = this.get(cur) {
            if cur_u.order != cur {
                this.swap(cur, cur_u.order);
            } else {
                cur += 1;
            }
        }

        if cfg!(debug_assertions) {
            if !(this.iter().enumerate().all(|(i, u)| i == u.order)) {
                let unsorted = this.iter().map(|u| u.order).collect::<Vec<_>>();
                panic!("Failed to sort StatedUiHandles: {:?}", unsorted.as_slice())
            }
        }
    }
}

// TODO: implement with bit fields for simplicity
#[derive(Clone, Copy)]
enum UpdateState {
    MightUnmounted,
    Mounted,
    MightUnmountedAndOldMovedLeft,
    MountedAndOldMovedLeft,
}

impl UpdateState {
    fn is_marked_as_might_unmounted(self) -> bool {
        matches!(
            self,
            Self::MightUnmounted | Self::MightUnmountedAndOldMovedLeft
        )
    }

    fn mark_as_mounted(&mut self) {
        match self {
            UpdateState::MightUnmounted => *self = UpdateState::Mounted,
            UpdateState::MightUnmountedAndOldMovedLeft => {
                *self = UpdateState::MountedAndOldMovedLeft
            }
            _ => {}
        }
    }

    fn mark_as_old_moved_left(&mut self) {
        match self {
            UpdateState::MightUnmounted => *self = UpdateState::MightUnmountedAndOldMovedLeft,
            UpdateState::Mounted => *self = UpdateState::MountedAndOldMovedLeft,
            _ => {}
        }
    }

    fn take_old_moved_left(&mut self) -> bool {
        match self {
            UpdateState::MightUnmountedAndOldMovedLeft => {
                *self = UpdateState::MightUnmounted;
                true
            }
            UpdateState::MountedAndOldMovedLeft => {
                *self = UpdateState::Mounted;
                true
            }
            _ => false,
        }
    }
}

// the first `key_to_index.len()` states are mounted.
pub struct UiHandles<U>(Vec<StatedUiHandle<U>>);

pub struct UnmountedUiHandles<U>(Vec<StatedUiHandle<U>>);

impl<U: UnmountedUiHandle<R>, R: ?Sized> UnmountedUiHandle<R> for UnmountedUiHandles<U> {
    type Mounted = UiHandles<U::Mounted>;

    fn mount(mut self, render_context: &mut <R>::RenderContext<'_>) -> Self::Mounted
    where
        R: frender_html::dom::render::RenderWithContext,
    {
        StatedUiHandle::sort(&mut self.0);
        UiHandles(
            self.0
                .into_iter()
                .map(|this| this.map_ui_handle(|u| u.mount(render_context)))
                .collect(),
        )
    }
}

impl<M: UiHandle<R>, R: ?Sized> UiHandle<R> for UiHandles<M> {
    type Unmounted = UnmountedUiHandles<M::Unmounted>;

    fn unmount(self, renderer: &mut R) -> Self::Unmounted {
        UnmountedUiHandles(
            self.0
                .into_iter()
                .map(|this| this.map_ui_handle(|u| u.unmount(renderer)))
                .collect(),
        )
    }

    fn reposition(&mut self, render_context: &mut <R>::RenderContext<'_>)
    where
        R: frender_html::dom::render::RenderWithContext,
    {
        for this in self.0.iter_mut() {
            this.ui_handle.reposition(render_context);
        }
    }

    fn check_and_move_cursor(&self, render_context: &mut <R>::RenderContext<'_>)
    where
        R: frender_html::dom::render::RenderWithContext,
    {
        for this in self.0.iter() {
            this.ui_handle.check_and_move_cursor(render_context);
        }
    }

    fn assert_cursor_is_at_self(&self, render_context: &<R>::RenderContext<'_>)
    where
        R: frender_html::dom::render::RenderWithContext,
    {
        if let Some(first) = self.0.first() {
            first.ui_handle.assert_cursor_is_at_self(render_context);
        }
    }
}

pub struct States<K, S>(IndexMap<K, S>);

impl<K, S> Unpin for States<K, S> {}

impl<K, S: StateUnmount + Unpin> StateUnmount for States<K, S> {
    fn state_unmount(self: Pin<&mut Self>) {
        self.get_mut()
            .0
            .values_mut()
            .map(Pin::new)
            .for_each(S::state_unmount)
    }
}

enum Never {}
pub struct Kind<K: Hash + Eq, EK: UnpinnedRenderStateKind>(Never, PhantomData<(K, EK)>);

impl<K: Hash + Eq, EK: UnpinnedRenderStateKind> UnpinnedRenderStateKind for Kind<K, EK> {
    type UnpinnedUiHandle<R: RenderHtml + ?Sized> =
        CursorPlaceholdersSurrounded<R::CursorPlaceholder, UiHandles<EK::UnpinnedUiHandle<R>>>;
    type UnpinnedState<R: RenderHtml + ?Sized> = States<K, EK::UnpinnedState<R>>;
}

impl<K: Hash + Eq, EK: UnpinnedRenderStateKindPollRender> UnpinnedRenderStateKindPollRender
    for Kind<K, EK>
{
    fn unpinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        States(states): &mut Self::UnpinnedState<R>,
        ui_handle: &mut Self::UnpinnedUiHandle<R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        let UiHandles(ui_handles) = ui_handle.surrounded_mut();
        debug_assert_eq!(ui_handles.len(), states.len());

        let mut res = Poll::Ready(());
        for (StatedUiHandle { ui_handle, .. }, state) in
            ui_handles.iter_mut().zip(states.values_mut())
        {
            if let Poll::Pending = EK::unpinned_poll_render(renderer, state, ui_handle, cx) {
                res = Poll::Pending
            }
        }

        res
    }
}

impl<K: Hash + Eq, E: CsrElement> KeyedElementsAlgorithm<K, E> for DefaultAlgorithm {
    type KeyedElementsRenderStateKind = Kind<K, E::RenderStateKind>;

    fn dummy_state<Renderer: ?Sized + RenderHtml>(
    ) -> experimental::UnpinnedStateOfKind<Renderer, Self::KeyedElementsRenderStateKind> {
        States(Default::default())
    }

    fn keyed_elements_render_init<
        I: IntoIterator<Item = Keyed<K, E>>,
        Ctx: ?Sized + frender_html::HtmlRenderContext,
    >(
        self,
        elements: I,
        render_context: &mut Ctx,
    ) -> (
        experimental::UnpinnedStateOfKind<Ctx::Renderer, Self::KeyedElementsRenderStateKind>,
        experimental::UnpinnedUiHandleOfKind<Ctx::Renderer, Self::KeyedElementsRenderStateKind>,
    ) {
        render_context.map_mut_render_context(|render_context| {
            CursorPlaceholdersSurrounded::output_and_surround(render_context, |render_context| {
                let (states, ui_handles) = elements
                    .into_iter()
                    .enumerate()
                    .map(|(order, Keyed(key, element))| {
                        let (state, ui_handle) = E::unpinned_render_init(element, render_context);

                        (
                            // key_to_state
                            (key, state),
                            // ui_handles
                            StatedUiHandle {
                                order,
                                update_state: UpdateState::Mounted,
                                ui_handle,
                            },
                        )
                    })
                    .collect();

                (States(states), UiHandles(ui_handles))
            })
        })
    }

    fn keyed_elements_render_init_by_reusing<
        I: IntoIterator<Item = Keyed<K, E>>,
        Ctx: ?Sized + frender_html::HtmlRenderContext,
    >(
        self,
        elements: I,
        render_context: &mut Ctx,
        States(key_to_state): &mut experimental::UnpinnedStateOfKind<
            Ctx::Renderer,
            Self::KeyedElementsRenderStateKind,
        >,
        unmounted_ui_handle: experimental::UnpinnedUnmountedUiHandleOfKind<
            Ctx::Renderer,
            Self::KeyedElementsRenderStateKind,
        >,
    ) -> experimental::UnpinnedUiHandleOfKind<Ctx::Renderer, Self::KeyedElementsRenderStateKind>
    {
        render_context
            .map_mut_render_context(|render_context| {
                unmounted_ui_handle.mount_and_map_and_output(
                    render_context,
                    |UnmountedUiHandles(unmounted_ui_handles), render_context| {
                        debug_assert_eq!(unmounted_ui_handles.len(), key_to_state.len());
                        render_context.map_mut_unrendered_render_context_and_then_reposition(
                            |render_context| {
                                let mut ui_handles = render_context.map_mut_cloned_render_context(
                                    |render_context| {
                                        // TODO: sort before mount?
                                        unmounted_ui_handles
                                            .into_iter()
                                            .enumerate()
                                            .map(|(i, u)| StatedUiHandle {
                                                order: i,
                                                update_state: UpdateState::MightUnmounted,
                                                ui_handle: u.ui_handle.mount(render_context),
                                            })
                                            .collect::<Vec<_>>()
                                    },
                                );

                                render_update(
                                    elements,
                                    render_context,
                                    key_to_state,
                                    &mut ui_handles,
                                );

                                ((), UiHandles(ui_handles))
                            },
                        )
                    },
                )
            })
            .1
    }

    fn keyed_elements_render_update<
        I: IntoIterator<Item = Keyed<K, E>>,
        Renderer: ?Sized + frender_html::RenderHtml,
    >(
        self,
        elements: I,
        renderer: &mut Renderer,
        States(key_to_state): &mut experimental::UnpinnedStateOfKind<
            Renderer,
            Self::KeyedElementsRenderStateKind,
        >,
        ui_handle: &mut experimental::UnpinnedUiHandleOfKind<
            Renderer,
            Self::KeyedElementsRenderStateKind,
        >,
    ) {
        ui_handle.use_surrounded_render_context(
            renderer,
            |UiHandles(ui_handles), render_context| {
                render_update(elements, render_context, key_to_state, ui_handles)
            },
        )
    }
}

fn render_update<
    K: Hash + Eq,
    E: CsrElement,
    I: IntoIterator<Item = Keyed<K, E>>,
    R: ?Sized + RenderHtml,
>(
    elements: I,
    render_context: &mut R::RenderContext<'_>,
    key_to_state: &mut IndexMap<K, experimental::UnpinnedStateOfKind<R, E::RenderStateKind>>,
    ui_handles: &mut Vec<
        StatedUiHandle<experimental::UnpinnedUiHandleOfKind<R, E::RenderStateKind>>,
    >,
) {
    // TODO: specialize for ExactSizeIterator
    // let elements = elements.into_iter();
    // if elements.len() == 0 {
    // }

    let mut cur = 0;
    let mut old_cur = 0;

    let old_mounted_count = key_to_state.len();

    ui_handles.iter_mut().for_each(|state| {
        state.update_state = UpdateState::MightUnmounted;
    });

    for Keyed(key, element) in elements {
        match key_to_state.entry(key) {
            Entry::Occupied(mut entry) => {
                // old element old state, possible new position

                let index = entry.index();
                let StatedUiHandle {
                    ui_handle,
                    order,
                    update_state,
                } = &mut ui_handles[index];

                enum Strategy {
                    // force_position = false
                    NoMove,
                    // force_position = false
                    Skip,
                    // force_position = true
                    MoveLeft { old_order: usize },
                    // force_position = true
                    MoveRight,
                }

                let strategy = match old_cur.cmp(order) {
                    Ordering::Equal => Strategy::NoMove,
                    Ordering::Less => {
                        let old_order = *order;
                        // old_order = 4, old_cur = 1
                        // *4* is moved left or *2 3* is moved right
                        // 1 2 3 4 => 1 4 2 3                  MoveLeft
                        // 1 2 3 4 5 6 7 8 => 1 4 2 3 5 6 7 8  Skip (*2 3* will move right)
                        // 1 2 3 4 5 6 7 8 => 1 4 5 6 7 8 2 3  Skip (*5 6 7 8* will NoMove, *2 3* will move right)
                        let before = old_order - old_cur;
                        let after = old_mounted_count - old_order;

                        if before < after {
                            // *2 3* were marked as MightUnmount (state_unmounted=true) at the start
                            // They will be removed or mounted later.
                            render_context.mark_cursor_skipped();
                            Strategy::Skip
                        } else {
                            // move *4* left
                            Strategy::MoveLeft { old_order }
                        }
                    }
                    Ordering::Greater => {
                        // this state were skipped before
                        Strategy::MoveRight
                    }
                };

                let state = entry.get_mut();

                element.unpinned_render_update(render_context.renderer_mut(), state, ui_handle);

                {
                    let force_reposition =
                        matches!(strategy, Strategy::MoveLeft { .. } | Strategy::MoveRight);
                    if force_reposition {
                        ui_handle.reposition(render_context);
                    } else {
                        ui_handle.check_and_move_cursor(render_context);
                    }
                }

                update_state.mark_as_mounted();
                let old_order = *order;
                *order = cur;

                match strategy {
                    Strategy::NoMove | Strategy::Skip => {
                        old_cur = old_order + 1;
                        while ui_handles
                            .get_mut(old_cur)
                            .map(|state| state.update_state.take_old_moved_left())
                            .unwrap_or(false)
                        {
                            old_cur += 1;
                        }
                    }
                    Strategy::MoveLeft { old_order } => {
                        debug_assert!(old_order > old_cur);
                        debug_assert!(old_order < old_mounted_count);
                        // mark state at old order `old_order` as moved left
                        ui_handles[old_order].update_state.mark_as_old_moved_left();
                        // old_cur doesn't change
                    }
                    Strategy::MoveRight => {
                        // old_cur doesn't change
                    }
                }
            }
            Entry::Vacant(entry) => {
                let index = entry.index();
                debug_assert_eq!(ui_handles.len(), index);
                // TODO: swap with the first MightUnmount state

                let (state, ui_handle) = element.unpinned_render_init(render_context);
                ui_handles.push(StatedUiHandle {
                    ui_handle,
                    order: cur,
                    update_state: UpdateState::Mounted,
                });

                entry.insert(state);
            }
        };

        cur += 1;
    }

    let real_len = key_to_state.len();
    let mut mounted_count = 0;
    while mounted_count < key_to_state.len() {
        if ui_handles[mounted_count]
            .update_state
            .is_marked_as_might_unmounted()
        {
            key_to_state.swap_remove_index(mounted_count);
            ui_handles.swap(
                mounted_count,
                // this has been decremented
                key_to_state.len(),
            );
        } else {
            mounted_count += 1;
        }
    }

    let renderer = render_context.renderer_mut();

    ui_handles.drain(mounted_count..real_len).for_each(|u| {
        debug_assert!(u.update_state.is_marked_as_might_unmounted());
        _ = u.ui_handle.unmount(renderer);
    });

    debug_assert_eq!(ui_handles.len(), key_to_state.len());
}
