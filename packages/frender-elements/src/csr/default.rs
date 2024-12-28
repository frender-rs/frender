//! The problem is how to deal with the following two cases with one algorithm:
//!
//! - Case I : `1 2 3 4 5 6 7        => 1 *4* 2 3 4 5 6 7`
//! - Case II: `1 2 3 4 5 6 7 8 9... => 1  4  5 6 7 8 9 ... *2 3*`
//!
//! In case I, it's more performant to just move *4* to after 2
//! (force_reposition *4* and go on as normal).
//!
//! In case II, it's more performant to move *2 3* to the end.
//! (mark *2 3* as MightUnmount and go on as normal.
//! When *2* is met, force_reposition *2*.
//! When *3* is met, force_reposition *3*.).
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
//! The render state `Element::UnpinnedRenderState` might NOT get dropped when unmounted.
//! [`RenderState::unmount`] will always run when unmounted.

use std::{cmp::Ordering, hash::Hash, marker::PhantomData, pin::Pin, task::Poll};

use indexmap::IndexMap;

use frender_html::{
    dom::{
        render::RenderContext,
        ui_handle::{UiHandle, UnmountedUiHandle},
    },
    elements::option::UiHandleMaybe,
    experimental::{RenderStates, UnpinnedRenderStateKind, UnpinnedRenderStateKindPollRender},
    ui_handles::CursorPlaceholdersSurrounded,
    CsrElement, HtmlRenderContext, RenderHtml, StateUnmount,
};

use crate::{DefaultElementsAlgorithm, Keyed};

use super::{ElementsAlgorithm, KeyedElementsAlgorithm};

struct State<M, U, S> {
    // TODO: UiHandleMaybe is used to avoid re-alloc when unmount and mount with the cost of larger size.
    // This might be optimized with one of the followings:
    // - union
    // - in place collect <https://doc.rust-lang.org/src/alloc/vec/in_place_collect.rs.html>
    ui_handle: UiHandleMaybe<M, U>,
    non_reactive_state: S,
    // The following fields could have a smaller representation like `usize`.
    order: usize,
    update_state: UpdateState,
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
pub struct UiHandlesWithNonReactiveStates<M, U, S>(Vec<State<M, U, S>>);

pub struct UnmountedUiHandlesWithNonReactiveStates<M, U, S>(Vec<State<M, U, S>>);

impl<U: UnmountedUiHandle<R>, S, R: ?Sized> UnmountedUiHandle<R>
    for UnmountedUiHandlesWithNonReactiveStates<U::Mounted, U, S>
{
    type Mounted = UiHandlesWithNonReactiveStates<U::Mounted, U, S>;

    fn mount(mut self, render_context: &mut <R>::RenderContext<'_>) -> Self::Mounted
    where
        R: frender_html::dom::render::RenderWithContext,
    {
        self.0
            .iter_mut()
            .for_each(|this| _ = this.ui_handle.mount_in_place(render_context));

        UiHandlesWithNonReactiveStates(self.0)
    }
}

impl<M: UiHandle<R>, S, R: ?Sized> UiHandle<R>
    for UiHandlesWithNonReactiveStates<M, M::Unmounted, S>
{
    type Unmounted = UnmountedUiHandlesWithNonReactiveStates<M, M::Unmounted, S>;

    fn unmount(mut self, renderer: &mut R) -> Self::Unmounted {
        for this in self.0.iter_mut() {
            let UiHandleMaybe::Mounted(mounted) = this.ui_handle.take() else {
                unreachable!()
            };
            this.ui_handle = UiHandleMaybe::Unmounted(mounted.unmount(renderer));
        }
        UnmountedUiHandlesWithNonReactiveStates(self.0)
    }

    fn reposition(&mut self, render_context: &mut <R>::RenderContext<'_>)
    where
        R: frender_html::dom::render::RenderWithContext,
    {
        for this in self.0.iter_mut() {
            match &mut this.ui_handle {
                UiHandleMaybe::Mounted(mounted) => mounted.reposition(render_context),
                _ => unreachable!(),
            }
        }
    }

    fn check_and_move_cursor(&self, render_context: &mut <R>::RenderContext<'_>)
    where
        R: frender_html::dom::render::RenderWithContext,
    {
        for this in self.0.iter() {
            match &this.ui_handle {
                UiHandleMaybe::Mounted(mounted) => mounted.check_and_move_cursor(render_context),
                _ => unreachable!(),
            }
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

pub struct KeyToIndex<K>(IndexMap<K, ()>);

pub struct ReactiveStates<S>(Vec<S>);

impl<S> Default for ReactiveStates<S> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<S: StateUnmount + Unpin> StateUnmount for ReactiveStates<S> {
    fn state_unmount(self: Pin<&mut Self>) {
        self.get_mut()
            .0
            .iter_mut()
            .map(Pin::new)
            .for_each(S::state_unmount)
    }
}

#[cfg(todo)]
fn insert_at_index<'s, K: Hash + Eq, S: Default>(
    states: &'s mut Vec<State<S>>,
    key_to_index: &mut IndexMap<K, ()>,
    key: K,
    index: usize,
) -> &'s mut State<S> {
    debug_assert!(index <= key_to_index.len());
    let old_index = match key_to_index.entry(key) {
        indexmap::map::Entry::Occupied(entry) => entry.index(),
        indexmap::map::Entry::Vacant(entry) => {
            let real_len = entry.index();
            entry.insert(());
            push_state_with_real_len(states, real_len);
            real_len
        }
    };

    if old_index != index {
        key_to_index.swap_indices(old_index, index);
        states.swap(old_index, index);
    }

    &mut states[index]
}

fn push_state_with_real_len<T: Default>(states: &mut Vec<T>, real_len: usize) {
    debug_assert!(states.len() >= real_len);

    if states.len() == real_len {
        states.push(Default::default());
    }
}

enum Never {}
pub struct Kind<K: Hash + Eq, EK: UnpinnedRenderStateKind>(Never, PhantomData<(K, EK)>);

impl<K: Hash + Eq, EK: UnpinnedRenderStateKind> UnpinnedRenderStateKind for Kind<K, EK> {
    type UnpinnedUiHandle<R: RenderHtml + ?Sized> = CursorPlaceholdersSurrounded<
        R::CursorPlaceholder,
        UiHandlesWithNonReactiveStates<
            EK::UnpinnedUiHandle<R>,
            <EK::UnpinnedUiHandle<R> as UiHandle<R>>::Unmounted,
            EK::UnpinnedNonReactiveState<R>,
        >,
    >;
    type UnpinnedNonReactiveState<R: RenderHtml + ?Sized> = KeyToIndex<K>;
    type UnpinnedReactiveState = ReactiveStates<EK::UnpinnedReactiveState>;
}

impl<K: Hash + Eq, EK: UnpinnedRenderStateKindPollRender> UnpinnedRenderStateKindPollRender
    for Kind<K, EK>
{
    fn unpinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        RenderStates {
            ui_handle,
            non_reactive_state: KeyToIndex(key_to_index),
            reactive_state: ReactiveStates(reactive_states),
        }: frender_html::experimental::UnpinnedMutRenderStatesOfKind<Self, R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        let UiHandlesWithNonReactiveStates(states) = ui_handle.surrounded_mut();
        debug_assert_eq!(states.len(), key_to_index.len());
        debug_assert_eq!(states.len(), reactive_states.len());

        let mut res = Poll::Ready(());
        for (
            State {
                ui_handle,
                non_reactive_state,
                ..
            },
            reactive_state,
        ) in states.iter_mut().zip(reactive_states.iter_mut())
        {
            let UiHandleMaybe::Mounted(ui_handle) = ui_handle else {
                unreachable!()
            };
            if let Poll::Pending = EK::unpinned_poll_render(
                renderer,
                RenderStates {
                    ui_handle,
                    non_reactive_state,
                    reactive_state,
                },
                cx,
            ) {
                res = Poll::Pending
            }
        }

        res
    }
}

impl<K: Hash + Eq, E: CsrElement> KeyedElementsAlgorithm<K, E> for DefaultElementsAlgorithm {
    type KeyedElementsRenderStateKind = Kind<K, E::RenderStateKind>;

    fn keyed_elements_render_init<
        I: IntoIterator<Item = Keyed<K, E>>,
        Ctx: ?Sized + frender_html::HtmlRenderContext,
    >(
        self,
        elements: I,
        render_context: &mut Ctx,
    ) -> frender_html::experimental::UnpinnedRenderStatesOfKind<
        Self::KeyedElementsRenderStateKind,
        Ctx::Renderer,
    > {
        let (ui_handle, (key_to_index, reactive_states)) =
            render_context.map_mut_render_context(|render_context| {
                CursorPlaceholdersSurrounded::surround_and_output(
                    render_context,
                    |render_context| {
                        let (ui_handles, rest) = elements
                            .into_iter()
                            .enumerate()
                            .map(|(order, Keyed(key, element))| {
                                let RenderStates {
                                    ui_handle,
                                    non_reactive_state,
                                    reactive_state,
                                } = E::unpinned_render_init(element, render_context);

                                (
                                    // ui_handles
                                    State {
                                        ui_handle: UiHandleMaybe::Mounted(ui_handle),
                                        non_reactive_state,
                                        order,
                                        update_state: UpdateState::Mounted,
                                    },
                                    (
                                        // key_to_index
                                        (key, ()),
                                        // reactive_states
                                        reactive_state,
                                    ),
                                )
                            })
                            .collect();

                        (UiHandlesWithNonReactiveStates(ui_handles), rest)
                    },
                )
            });
        RenderStates {
            ui_handle,
            non_reactive_state: KeyToIndex(key_to_index),
            reactive_state: ReactiveStates(reactive_states),
        }
    }

    fn keyed_elements_render_update<
        I: IntoIterator<Item = Keyed<K, E>>,
        Ctx: ?Sized + frender_html::HtmlRenderContext,
    >(
        self,
        elements: I,
        render_context: &mut Ctx,
        states: frender_html::experimental::UnpinnedMutRenderStatesOfKind<
            Self::KeyedElementsRenderStateKind,
            Ctx::Renderer,
        >,
    ) {
        use frender_html::dom::behaviors::{Node as _, NodeRenderSelf};

        let RenderStates {
            ui_handle,
            non_reactive_state,
            reactive_state,
        } = states;

        render_context.map_mut_render_context(|render_context| {
            ui_handle.map_mut_surrounded_with_render_context(
                render_context,
                |ui_handle, render_context| {
                    render_update(
                        elements,
                        render_context,
                        RenderStates {
                            ui_handle,
                            non_reactive_state,
                            reactive_state,
                        },
                    )
                },
            )
        })
    }
}

type UiHandlesOfKind<EK, R> = UiHandlesWithNonReactiveStates<
    <EK as UnpinnedRenderStateKind>::UnpinnedUiHandle<R>,
    <<EK as UnpinnedRenderStateKind>::UnpinnedUiHandle<R> as UiHandle<R>>::Unmounted,
    <EK as UnpinnedRenderStateKind>::UnpinnedNonReactiveState<R>,
>;

fn render_update<
    K: Hash + Eq,
    E: CsrElement,
    I: IntoIterator<Item = Keyed<K, E>>,
    R: ?Sized + RenderHtml,
>(
    elements: I,
    render_context: &mut R::RenderContext<'_>,
    RenderStates {
        ui_handle: UiHandlesWithNonReactiveStates(states),
        non_reactive_state: KeyToIndex(key_to_index),
        reactive_state: ReactiveStates(reactive_states),
    }: RenderStates<
        &mut UiHandlesOfKind<E::RenderStateKind, R>,
        &mut KeyToIndex<K>,
        &mut ReactiveStates<<E::RenderStateKind as UnpinnedRenderStateKind>::UnpinnedReactiveState>,
    >,
) {
    let elements = elements.into_iter();

    // TODO: specialize for ExactSizeIterator
    // if elements.len() == 0 {
    // }

    let mut cur = 0;
    let mut old_cur = 0;

    let old_mounted_count = key_to_index.len();

    states.iter_mut().for_each(|state| {
        state.update_state = UpdateState::MightUnmounted;
    });

    for Keyed(key, element) in elements {
        match key_to_index.entry(key) {
            indexmap::map::Entry::Occupied(entry) => {
                // old element old state, possible new position

                let index = entry.index();
                let State {
                    ui_handle,
                    non_reactive_state,
                    order,
                    update_state,
                } = &mut states[index];

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
                        // order = 4, old_cur = 2
                        // *4* is moved left or *2 3* is moved right
                        // 1 2 3 4 => 1 4 2 3
                        // 1 2 3 4 5 6 7 8 => 1 4 2 3 5 6 7 8
                        // 1 2 3 4 5 6 7 8 => 1 4 5 6 7 8 2 3
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

                let UiHandleMaybe::Mounted(ui_handle) = ui_handle else {
                    unreachable!()
                };

                {
                    let force_reposition =
                        matches!(strategy, Strategy::MoveLeft { .. } | Strategy::MoveRight);
                    if force_reposition {
                        ui_handle.reposition(render_context);
                    }
                }

                let reactive_state = &mut reactive_states[index];

                element.unpinned_render_update(
                    render_context,
                    RenderStates {
                        ui_handle,
                        non_reactive_state,
                        reactive_state,
                    },
                );

                update_state.mark_as_mounted();
                let old_order = *order;
                *order = cur;

                match strategy {
                    Strategy::NoMove | Strategy::Skip => {
                        old_cur = old_order + 1;
                        while states
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
                        states[old_order].update_state.mark_as_old_moved_left();
                        // old_cur doesn't change
                    }
                    Strategy::MoveRight => {
                        // old_cur doesn't change
                    }
                }
            }
            indexmap::map::Entry::Vacant(entry) => {
                let index = entry.index();
                debug_assert_eq!(states.len(), index);
                debug_assert_eq!(reactive_states.len(), index);
                // TODO: swap with the first MightUnmount state

                let RenderStates {
                    ui_handle,
                    non_reactive_state,
                    reactive_state,
                } = element.unpinned_render_init(render_context);
                states.push(State {
                    ui_handle: UiHandleMaybe::Mounted(ui_handle),
                    non_reactive_state,
                    order: cur,
                    update_state: UpdateState::Mounted,
                });

                reactive_states.push(reactive_state);

                entry.insert(());
            }
        };

        cur += 1;
    }

    let real_len = key_to_index.len();
    let mut mounted_count = 0;
    while mounted_count < key_to_index.len() {
        if states[mounted_count]
            .update_state
            .is_marked_as_might_unmounted()
        {
            key_to_index.swap_remove_index(mounted_count);
            states.swap(
                mounted_count,
                // this has been decremented
                key_to_index.len(),
            );
        } else {
            mounted_count += 1;
        }
    }

    let renderer = render_context.renderer_mut();

    states.drain(mounted_count..real_len).for_each(|state| {
        debug_assert!(state.update_state.is_marked_as_might_unmounted());
        let UiHandleMaybe::Mounted(ui_handle) = state.ui_handle else {
            unreachable!()
        };
        _ = ui_handle.unmount(renderer);
    });

    reactive_states
        .drain(mounted_count..real_len)
        .for_each(|mut state| Pin::new(&mut state).state_unmount());

    debug_assert_eq!(states.len(), key_to_index.len());
    debug_assert_eq!(reactive_states.len(), key_to_index.len());
}
