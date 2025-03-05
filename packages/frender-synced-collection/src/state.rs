use std::{cell::RefCell, pin::Pin, rc::Rc, task::Poll};

use frender_csr::{
    CsrElement, StateUnmount, UiHandle, UnmountedUiHandle,
    experimental::{
        HtmlRenderContext, RenderHtml, UnpinnedRenderStateKind, UnpinnedRenderStateKindPollRender,
    },
    render::{RenderContext, RenderWithContext},
};

use super::{
    RcWithKey, StatesCommon, StatesLikeVec,
    render_states::{MountState, NeedsReposition},
};

pub(super) enum State<M, U, S> {
    /// The SyncedCollection has inserted an item, but its ui handle hasn't been rendered for real.
    BeforeMounted,
    Mounted {
        state: S,
        ui_handle: M,
        mount_state: MountState,
    },
    Unmounted {
        state: S,
        ui_handle: U,
    },
}

impl<M, U, S> State<M, U, S> {
    // region: fn mark
    fn mark_as_moved(&mut self) {
        if let Self::Mounted { mount_state, .. } = self {
            mount_state.mark_as_moved()
        }
    }
    fn mark_as_outdated_and_moved(&mut self) {
        if let Self::Mounted { mount_state, .. } = self {
            mount_state.mark_as_outdated_and_moved()
        }
    }
    fn mark_as_outdated(&mut self) {
        if let Self::Mounted { mount_state, .. } = self {
            mount_state.mark_as_outdated()
        }
    }
    fn mark_previous_was_skipped(&mut self) {
        if let Self::Mounted { mount_state, .. } = self {
            mount_state.mark_previous_was_skipped()
        }
    }
    // endregion

    pub(crate) fn from_render_states((state, ui_handle): (S, M)) -> Self {
        Self::Mounted {
            state,
            ui_handle,
            mount_state: MountState::MountedAndUpToDate,
        }
    }

    /// Doesn't trust mount_state.
    pub(super) fn force_render_init_or_update_with<E: CsrElement, Ctx: ?Sized + HtmlRenderContext>(
        &mut self,
        element: E,
        render_context: &mut Ctx,
        reposition: impl FnOnce(
            &mut M,
            &MountState,
            &mut <Ctx::Renderer as RenderWithContext>::RenderContext<'_>,
        ),
    ) where
        M: UiHandle<Ctx::Renderer, Unmounted = U>,
        U: UnmountedUiHandle<Ctx::Renderer, Mounted = M>,
        E::RenderStateKind: UnpinnedRenderStateKind<
                UnpinnedUiHandle<Ctx::Renderer> = M,
                UnpinnedState<Ctx::Renderer> = S,
            >,
    {
        self.render_init_or_update_with(
            || element,
            render_context,
            |get_element, state, ui_handle, mount_state, render_context| {
                reposition(ui_handle, mount_state, render_context);

                // render_update
                get_element().unpinned_render_update(
                    render_context.renderer_mut(),
                    state,
                    ui_handle,
                );

                *mount_state = MountState::MountedAndUpToDate;
            },
        )
    }

    pub(super) fn render_init_or_update_with<
        //
        E: CsrElement,
        Ctx: ?Sized + HtmlRenderContext,
        G: FnOnce() -> E,
    >(
        &mut self,
        get_element: G,
        render_context: &mut Ctx,
        process_mounted: impl FnOnce(
            G,
            &mut S,
            &mut M,
            &mut MountState,
            &mut <Ctx::Renderer as RenderWithContext>::RenderContext<'_>,
        ),
    ) where
        M: UiHandle<Ctx::Renderer, Unmounted = U>,
        U: UnmountedUiHandle<Ctx::Renderer, Mounted = M>,
        E::RenderStateKind: UnpinnedRenderStateKind<
                UnpinnedUiHandle<Ctx::Renderer> = M,
                UnpinnedState<Ctx::Renderer> = S,
            >,
    {
        match self {
            State::BeforeMounted => {
                *self = {
                    let (state, ui_handle) = get_element().unpinned_render_init(render_context);
                    Self::Mounted {
                        state,
                        ui_handle,
                        mount_state: MountState::MountedAndUpToDate,
                    }
                }
            }
            State::Mounted {
                state,
                ui_handle,
                mount_state,
            } => render_context.map_mut_render_context(|render_context| {
                process_mounted(get_element, state, ui_handle, mount_state, render_context)
            }),
            State::Unmounted { .. } => {
                let State::Unmounted {
                    mut state,
                    ui_handle,
                } = self.take()
                else {
                    unreachable!()
                };

                let ui_handle = get_element().unpinned_render_init_by_reusing(
                    render_context,
                    &mut state,
                    ui_handle,
                );

                *self = Self::Mounted {
                    state,
                    ui_handle,
                    mount_state: MountState::MountedAndUpToDate,
                };
            }
        }
    }

    /// Doesn't trust mount_state
    pub(crate) fn force_render_with<E: CsrElement, Ctx: ?Sized + HtmlRenderContext>(
        &mut self,
        element: E,
        render_context: &mut Ctx,
    ) where
        M: UiHandle<Ctx::Renderer, Unmounted = U>,
        U: UnmountedUiHandle<Ctx::Renderer, Mounted = M>,
        E::RenderStateKind: UnpinnedRenderStateKind<
                UnpinnedUiHandle<Ctx::Renderer> = M,
                UnpinnedState<Ctx::Renderer> = S,
            >,
    {
        self.force_render_init_or_update_with(
            element,
            render_context,
            |ui_handle, _, render_context| ui_handle.reposition(render_context),
        )
    }

    /// Trusts only position info of mount_state
    pub(crate) fn force_render_with_but_trust_position<
        E: CsrElement,
        Ctx: ?Sized + HtmlRenderContext,
    >(
        &mut self,
        element: E,
        render_context: &mut Ctx,
    ) where
        M: UiHandle<Ctx::Renderer, Unmounted = U>,
        U: UnmountedUiHandle<Ctx::Renderer, Mounted = M>,
        E::RenderStateKind: UnpinnedRenderStateKind<
                UnpinnedUiHandle<Ctx::Renderer> = M,
                UnpinnedState<Ctx::Renderer> = S,
            >,
    {
        self.force_render_init_or_update_with(
            element,
            render_context,
            |ui_handle, mount_state, render_context| {
                // reposition or check_and_move_cursor
                match mount_state.needs_reposition() {
                    NeedsReposition::Yes => ui_handle.reposition(render_context),
                    NeedsReposition::No { previous_skipped } => {
                        if previous_skipped {
                            render_context.mark_cursor_skipped();
                        }

                        ui_handle.check_and_move_cursor(render_context)
                    }
                }
            },
        )
    }

    pub(crate) fn take(&mut self) -> Self {
        std::mem::replace(self, Self::BeforeMounted)
    }
    fn mount_in_place<R: ?Sized + RenderWithContext>(
        &mut self,
        render_context: &mut R::RenderContext<'_>,
    ) where
        U: UnmountedUiHandle<R, Mounted = M>,
    {
        match self.take() {
            State::BeforeMounted => {}
            State::Mounted { .. } => unreachable!(),
            State::Unmounted { state, ui_handle } => {
                *self = Self::Mounted {
                    state,
                    ui_handle: ui_handle.mount(render_context),
                    mount_state: MountState::Outdated, // mounted at the correct position but still outdated
                }
            }
        }
    }

    fn unmount_in_place<R: ?Sized>(&mut self, renderer: &mut R)
    where
        M: UiHandle<R, Unmounted = U>,
    {
        match self.take() {
            State::BeforeMounted => {}
            State::Mounted {
                state,
                ui_handle,
                mount_state: _,
            } => {
                *self = Self::Unmounted {
                    state,
                    ui_handle: ui_handle.unmount(renderer),
                }
            }
            State::Unmounted { .. } => unreachable!(),
        }
    }
}

pub(crate) struct States<M, U, S> {
    pub(super) states: Vec<State<M, U, S>>,
    // last `ready_to_unmount_count` states should be unmounted on next render_update
    pub(super) ready_to_unmount_count: usize,
    pub(super) all_outdated: bool,
}

impl<M, U, S> States<M, U, S> {
    pub(super) const fn new(states: Vec<State<M, U, S>>) -> Self {
        Self {
            states,
            ready_to_unmount_count: 0,
            all_outdated: false,
        }
    }

    pub(crate) fn real_len(&self) -> usize {
        self.states.len() - self.ready_to_unmount_count
    }
    fn states_mut(&mut self) -> &mut [State<M, U, S>] {
        let real_len = self.real_len();
        &mut self.states[..real_len]
    }

    fn insert_many_at(&mut self, at: usize, len: usize) {
        self.extend(len);
        self.states_mut()[at..].rotate_right(len);
    }

    pub(super) fn clean<R: ?Sized>(&mut self, renderer: &mut R) -> &mut Vec<State<M, U, S>>
    where
        M: UiHandle<R>,
        S: StateUnmount + Unpin,
    {
        if self.ready_to_unmount_count > 0 {
            let real_len = self.real_len();
            self.states.drain(real_len..).for_each(|state| match state {
                State::BeforeMounted => {}
                State::Mounted {
                    mut state,
                    ui_handle,
                    mount_state: _,
                } => {
                    Pin::new(&mut state).state_unmount();
                    _ = ui_handle.unmount(renderer);
                }
                State::Unmounted { .. } => {
                    // just drop
                }
            });
            self.ready_to_unmount_count = 0;
        }
        &mut self.states
    }
}

pub(crate) type StatesOfKind<K, R> = States<
    <K as UnpinnedRenderStateKind>::UnpinnedUiHandle<R>,
    <<K as UnpinnedRenderStateKind>::UnpinnedUiHandle<R> as UiHandle<R>>::Unmounted,
    <K as UnpinnedRenderStateKind>::UnpinnedState<R>,
>;

// No item is State::Unmounted
pub struct UiHandles<C, M, U, S> {
    pub(crate) ui_handles: Rc<RefCell<States<M, U, S>>>,
    pub(crate) cursor_placeholders: [C; 2],
}

impl<C, M, U, S> UiHandles<C, M, U, S> {
    pub(crate) fn poll_render<K, R>(
        &mut self,
        renderer: &mut R,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()>
    where
        K: UnpinnedRenderStateKindPollRender<UnpinnedUiHandle<R> = M, UnpinnedState<R> = S>,
        R: ?Sized + RenderHtml,
    {
        let mut this = self.ui_handles.borrow_mut();
        let mut res = Poll::Ready(());
        let mounted_len = this.states.len() - this.ready_to_unmount_count;
        for state in &mut this.states[..mounted_len] {
            let State::Mounted {
                state,
                ui_handle,
                mount_state: _,
            } = state
            else {
                unreachable!()
            };
            if let Poll::Pending = K::unpinned_poll_render(renderer, state, ui_handle, cx) {
                res = Poll::Pending;
            }
        }
        res
    }
}

// No item is State::Mounted
pub struct UnmountedUiHandles<C, M, U, S> {
    pub(crate) ui_handles: Rc<RefCell<States<M, U, S>>>,
    pub(crate) cursor_placeholders: [C; 2],
}

impl<C: UnmountedUiHandle<R>, U: UnmountedUiHandle<R>, S, R: ?Sized> UnmountedUiHandle<R>
    for UnmountedUiHandles<C, U::Mounted, U, S>
{
    type Mounted = UiHandles<C::Mounted, U::Mounted, U, S>;

    fn mount(self, render_context: &mut <R>::RenderContext<'_>) -> Self::Mounted
    where
        R: RenderWithContext,
    {
        let Self {
            ui_handles,
            cursor_placeholders: [cpa, cpb],
        } = self;

        let cpa = cpa.mount(render_context);

        {
            let this = &mut *ui_handles.borrow_mut();

            let states = &mut this.states;
            let valid_len = states.len() - this.ready_to_unmount_count;

            // remove the ready_to_unmount items
            if cfg!(debug_assertions) {
                states[valid_len..].iter().for_each(|state| match state {
                    State::BeforeMounted => {}
                    State::Mounted { .. } => {
                        unreachable!("UnmountedUiHandles shouldn't contain any Mounted ui handles")
                    }
                    State::Unmounted { .. } => {}
                });
            }
            states.truncate(valid_len);
            this.ready_to_unmount_count = 0;

            // mount
            states
                .iter_mut()
                .for_each(|state| state.mount_in_place(render_context));
        }

        let cpb = cpb.mount(render_context);

        UiHandles {
            ui_handles,
            cursor_placeholders: [cpa, cpb],
        }
    }
}

impl<C: UiHandle<R>, UH: UiHandle<R>, S, R: ?Sized> UiHandle<R>
    for UiHandles<C, UH, UH::Unmounted, S>
{
    type Unmounted = UnmountedUiHandles<C::Unmounted, UH, UH::Unmounted, S>;

    fn unmount(self, renderer: &mut R) -> Self::Unmounted {
        let Self {
            ui_handles,
            cursor_placeholders: [cpa, cpb],
        } = self;
        let cpa = cpa.unmount(renderer);
        {
            let this = &mut *ui_handles.borrow_mut();
            this.states
                .iter_mut()
                .for_each(|state| state.unmount_in_place(renderer));
        }
        let cpb = cpb.unmount(renderer);
        UnmountedUiHandles {
            ui_handles,
            cursor_placeholders: [cpa, cpb],
        }
    }

    fn reposition(&mut self, render_context: &mut <R>::RenderContext<'_>)
    where
        R: RenderWithContext,
    {
        let Self {
            ui_handles,
            cursor_placeholders: [cpa, cpb],
        } = self;

        cpa.reposition(render_context);

        let States {
            states,
            ready_to_unmount_count: _, // the ready_to_unmount ui handles also get repositioned,
            all_outdated: _,
        } = &mut *ui_handles.borrow_mut();
        states.iter_mut().for_each(|state| match state {
            State::BeforeMounted => {}
            State::Mounted {
                ui_handle,
                mount_state,
                ..
            } => {
                ui_handle.reposition(render_context);
                *mount_state = match mount_state {
                    MountState::MountedAndUpToDate => MountState::MountedAndUpToDate,
                    MountState::MountedAndUpToDateButPreviousWasSkipped => {
                        MountState::MountedAndUpToDate
                    }
                    MountState::Outdated => MountState::Outdated,
                    MountState::OutdatedAndPreviousWasSkipped => MountState::Outdated,
                    MountState::OutdatedAndMoved => MountState::Outdated,
                };
            }
            State::Unmounted { .. } => unreachable!(),
        });

        cpb.reposition(render_context);
    }

    fn check_and_move_cursor(&self, render_context: &mut <R>::RenderContext<'_>)
    where
        R: RenderWithContext,
    {
        let [cpa, cpb] = &self.cursor_placeholders;

        cpa.check_and_move_cursor(render_context);
        render_context.mark_cursor_skipped();
        cpb.check_and_move_cursor(render_context);
    }

    fn assert_cursor_is_at_self(&self, render_context: &<R>::RenderContext<'_>)
    where
        R: RenderWithContext,
    {
        self.cursor_placeholders[0].assert_cursor_is_at_self(render_context)
    }
}

pub(crate) trait StateUnmountWithRef {
    type ItemReactiveState;

    fn state_unmount_with_ref(&self);
}

impl<M, U, S: StateUnmount + Unpin> States<M, U, S> {
    fn state_unmount_all(&mut self) {
        self.all_outdated = true; // TODO: is this needed?
        self.states.iter_mut().for_each(|state| {
            Pin::new(match state {
                State::Mounted { state, .. } => state,
                State::BeforeMounted => return,
                State::Unmounted { state, .. } => state,
            })
            .state_unmount()
        });
    }
}

impl<M, U, S: StateUnmount + Unpin> StateUnmountWithRef for RefCell<States<M, U, S>> {
    type ItemReactiveState = S;

    fn state_unmount_with_ref(&self) {
        self.borrow_mut().state_unmount_all()
    }
}

pub struct ReactiveStates<RS>(
    pub(super) RcWithKey<dyn StateUnmountWithRef<ItemReactiveState = RS>>,
);

pub struct PinnedStates<S>(Option<ReactiveStates<S>>);

impl<S> PinnedStates<S> {
    pub(crate) const DUMMY: Self = Self(None);

    pub(crate) fn as_mut_states(&mut self) -> &mut ReactiveStates<S> {
        self.0.as_mut().unwrap()
    }

    pub(crate) fn set(&mut self, v: ReactiveStates<S>) {
        self.0 = Some(v)
    }
}

impl<RS: StateUnmount + Unpin> StateUnmount for ReactiveStates<RS> {
    fn state_unmount(self: Pin<&mut Self>) {
        self.0.rc.state_unmount_with_ref()
    }
}

impl<S: StateUnmount + Unpin> StateUnmount for PinnedStates<S> {
    fn state_unmount(self: Pin<&mut Self>) {
        Pin::new(self.get_mut().as_mut_states()).state_unmount()
    }
}

impl<M, U, S> StatesLikeVec for States<M, U, S> {
    fn clear(&mut self) {
        self.ready_to_unmount_count = self.states.len();
    }

    fn swap(&mut self, a: usize, b: usize) {
        let states = self.states_mut();

        states.swap(a, b);

        if a != b {
            states[a].mark_as_moved();
            states[b].mark_as_moved();

            // a,x,b -> b,x,a
            if a.abs_diff(b) > 1 {
                states[a.min(b) + 1].mark_previous_was_skipped()
            }
        }
    }

    fn remove(&mut self, index: usize) {
        // not real remove
        let states = &mut self.states_mut()[index..];
        states[0].mark_as_outdated_and_moved();
        if states.len() > 1 {
            states[1].mark_previous_was_skipped();
            states.rotate_left(1);
        }
        self.ready_to_unmount_count += 1;
    }

    fn swap_remove(&mut self, index: usize) {
        // not real remove
        let states = self.states_mut();
        let to_swap = states.len() - 1;
        if index < to_swap {
            states.swap(index, to_swap);
            states[index + 1].mark_previous_was_skipped();
        }
        states[index].mark_as_outdated_and_moved();

        states[index + 1].mark_as_moved();
        self.ready_to_unmount_count += 1;
    }
}

impl<M, U, S> StatesCommon for States<M, U, S> {
    fn mark_index_as_updated(&mut self, i: usize) {
        self.states_mut()[i].mark_as_outdated()
    }

    fn extend(&mut self, len: usize) {
        if len > self.ready_to_unmount_count {
            self.ready_to_unmount_count = 0;
            let new_count = len - self.ready_to_unmount_count;
            self.states
                .extend(std::iter::repeat_with(|| State::BeforeMounted).take(new_count));
        } else {
            // the items should have already been marked as outdated
            //
            // let from = self.real_len();
            // self.states[from..(from + len)]
            //     .iter_mut()
            //     .for_each(|s| s.mark_as_outdated());

            self.ready_to_unmount_count -= len;
        }
    }

    fn splice(&mut self, range: std::ops::Range<usize>, new_len: usize) {
        let real_len = self.real_len();
        assert!(range.start <= range.end);
        assert!(range.end <= real_len);

        let removed_len = range.end - range.start;

        if removed_len >= new_len {
            let drain_start = range.start + new_len;
            self.states[(range.start)..drain_start]
                .iter_mut()
                .for_each(|state| state.mark_as_outdated());

            self.drain(drain_start..(range.end))
        } else {
            let end = range.end;
            self.states[range]
                .iter_mut()
                .for_each(|state| state.mark_as_outdated());

            self.insert_many_at(end, new_len - removed_len);
        }
    }

    fn drain(&mut self, range: std::ops::Range<usize>) {
        let real_len = self.real_len();
        assert!(range.start <= range.end);
        assert!(range.end <= real_len);

        let states = &mut self.states_mut()[(range.start)..];
        let removed_len = range.end - range.start;

        if removed_len < states.len() {
            states[..removed_len]
                .iter_mut()
                .for_each(|state| state.mark_as_outdated_and_moved());
            states.rotate_left(removed_len);
            states[0].mark_previous_was_skipped();
        } else {
            states.iter_mut().for_each(|state| state.mark_as_outdated());
        }
        self.ready_to_unmount_count += removed_len;
    }

    fn mark_all_as_outdated(&mut self) {
        self.all_outdated = true;
    }

    fn mark_range_as_outdated(&mut self, range: &std::ops::Range<usize>) {
        if self.all_outdated {
            return;
        }
        if range.start == 0 && range.end == self.real_len() {
            self.mark_all_as_outdated()
        } else {
            self.states_mut()[range.clone()]
                .iter_mut()
                .for_each(|state| state.mark_as_outdated())
        }
    }
}
