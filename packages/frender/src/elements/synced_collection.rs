pub use to_element::{csr::Kind, SyncedCollectionToElement};
use weak_vec1::RcWithKey;

use std::{
    cell::RefCell,
    ops::{Deref, Index, IndexMut},
    rc::{Rc, Weak},
};

use crate::fn_traits::FnMut1;

mod weak_vec1 {
    use std::rc::{Rc, Weak};

    use super::weak_is_of_rc;

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(super) struct Key(usize);

    impl Key {
        pub(super) const STACK: Self = Self(usize::MAX);
    }

    pub(super) struct RcWithKey<T: ?Sized> {
        pub(super) rc: Rc<T>,
        pub(super) key: Key,
    }

    impl<T: ?Sized> RcWithKey<std::cell::RefCell<T>> {
        pub(super) fn borrow_mut(&self) -> std::cell::RefMut<T> {
            self.rc.borrow_mut()
        }
    }

    // Option<Weak> has the same size as Weak
    pub(super) struct WeakVec1<T: ?Sized>(Option<Weak<T>>, Vec<Option<Weak<T>>>);

    impl<T: ?Sized> std::fmt::Debug for WeakVec1<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_list()
                .entries(Some(&self.0).into_iter().chain(self.1.iter()))
                .finish()
        }
    }

    impl<T: ?Sized> Default for WeakVec1<T> {
        fn default() -> Self {
            Self::DEFAULT
        }
    }

    fn upgrade_or_set_none<T: ?Sized>(weak: &mut Option<Weak<T>>) -> Option<Rc<T>> {
        if let Some(v) = weak {
            if let Some(v) = v.upgrade() {
                Some(v)
            } else {
                *weak = None;
                None
            }
        } else {
            None
        }
    }

    fn is_empty_weak<T: ?Sized>(v: &Option<Weak<T>>) -> bool {
        if let Some(v) = v {
            v.strong_count() == 0
        } else {
            true
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

    impl<T: ?Sized> WeakVec1<T> {
        pub(super) const DEFAULT: Self = Self(None, Vec::new());
        pub(super) fn contains<U: ?Sized>(&self, v: &RcWithKey<U>) -> bool {
            let weak = match v.key {
                Key::STACK => self.0.as_ref(),
                Key(i) => self.1.get(i).and_then(Option::as_ref),
            };
            weak.map_or(false, |weak| weak_is_of_rc(weak, &v.rc))
        }

        pub(crate) fn put_into_old_available_or_append<R: ?Sized>(
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

        pub(super) fn for_each_alive(&mut self, mut f: impl FnMut(Rc<T>)) {
            if let Some(v) = upgrade_or_set_none(&mut self.0) {
                f(v);
            }

            let continuous_empty_count = self.1.iter_mut().fold(0usize, |n, weak| {
                if let Some(v) = upgrade_or_set_none(weak) {
                    f(v);
                    0
                } else {
                    n + 1
                }
            });

            self.1.truncate(self.1.len() - continuous_empty_count);
        }
    }
}

// In most cases, signal of ElementsSynced is only rendered in one place, requiring only one Weak<RefCell<dyn States>>
#[derive(Debug, Default)]
struct AllStates(weak_vec1::WeakVec1<RefCell<dyn States>>);

impl AllStates {
    const fn new() -> Self {
        Self(weak_vec1::WeakVec1::DEFAULT)
    }
}

impl StatesCommon for AllStates {
    fn mark_index_as_updated(&mut self, i: usize) {
        self.for_each_alive_mut(|states| states.mark_index_as_updated(i))
    }
    fn extend(&mut self, len: usize) {
        self.for_each_alive_mut(|states| states.extend(len))
    }
    fn splice(&mut self, range: std::ops::Range<usize>, len: usize) {
        self.for_each_alive_mut(|states| states.splice(range.clone(), len))
    }
    fn drain(&mut self, range: std::ops::Range<usize>) {
        self.for_each_alive_mut(|states| states.drain(range.clone()))
    }
    fn mark_all_as_outdated(&mut self) {
        self.for_each_alive_mut(|states| states.mark_all_as_outdated())
    }
    fn mark_range_as_outdated(&mut self, range: &std::ops::Range<usize>) {
        self.for_each_alive_mut(|states| states.mark_range_as_outdated(range))
    }
}

fn weak_is_of_rc<T: ?Sized, U: ?Sized>(weak: &Weak<T>, rc: &Rc<U>) -> bool {
    std::ptr::addr_eq(Weak::as_ptr(weak), Rc::as_ptr(rc))
}

impl AllStates {
    fn put_rc_states_with_old_key_hint<S: States + 'static>(
        &mut self,
        old_key_hint: weak_vec1::Key,
        rc: &Rc<RefCell<S>>,
    ) -> weak_vec1::Key {
        self.0
            .put_into_old_available_or_append(old_key_hint, rc, weak_is_of_rc, |rc| {
                Rc::downgrade(rc) as _
            })
    }

    fn for_each_alive_mut(&mut self, mut f: impl FnMut(&mut dyn States)) {
        self.0.for_each_alive(|rc| f(&mut *rc.borrow_mut()))
    }
}

trait StatesCommon {
    fn mark_index_as_updated(&mut self, i: usize);
    fn extend(&mut self, len: usize);
    fn splice(&mut self, range: std::ops::Range<usize>, len: usize);
    fn drain(&mut self, range: std::ops::Range<usize>);

    fn mark_all_as_outdated(&mut self);
    fn mark_range_as_outdated(&mut self, range: &std::ops::Range<usize>);
}

trait States: StatesCommon + StatesLikeVec {}

impl<S: ?Sized + StatesCommon + StatesLikeVec> States for S {}

/// All mutations are synced to the registered render states so that before elements `render_update`,
/// the render states can reposition and only `render_update`s the updated elements.
///
/// Currently all elements will render_update if any one element is updated.
/// This might change in the future.
///
/// - [`IndexMut<usize>`] would record a update at the index.
///
///   `elements[i] = new_element` or even just `&mut elements[i]` is a update mutation at `i`.
#[derive(Debug, Default)]
pub struct SyncedCollection<ES> {
    /// SyncedCollection maintains weak references of
    /// all the states (ui handles and states) which it has been rendered with.
    /// When the collection's items move/swap/remove/insert, the states do the same to keep positions synced.
    /// When the collection's item `items[i]` update, all the corresponding states get marked as outdated,
    /// so that it gets re-rendered on next render_update().
    ///
    /// ```no_compile
    /// self.all_states.for_each( |states| states[i].mark_as_outdated() )
    /// ```
    ///
    /// That's why this collection is *synced* with its rendering states.
    ///
    /// - ToElement only has &self, so [`RefCell`] is required.
    /// - The collection itself doesn't know how the items will be rendered, so [`dyn States`] is required.
    ///
    /// [`dyn States`]: States
    all_states: RefCell<AllStates>,
    items: ES,
}

impl<ES> SyncedCollection<ES> {
    pub const fn new(items: ES) -> Self {
        Self {
            all_states: RefCell::new(AllStates::new()),
            items,
        }
    }

    pub fn mark_all_as_outdated(&mut self) {
        self.all_states.get_mut().mark_all_as_outdated()
    }
}

macro_rules! like_vec {
    (
        type Item = $Item:ident;

        trait $StatesLikeVec:ident {
            $(
                fn $fn_name:ident(&mut $_self:ident $(, $arg:ident : $arg_ty:ty)* $(,)?) $(-> $output:ty)? ;
            )*
        }

        impl<__> $impl_for:ty {
            proxy!($proxy_for:expr);
            real_vec!($real_vec:expr);
        }

        impl<__> $AllStates:ty {
            proxy_with!($AllStates_proxy_with:ident);
        }
    ) => {
        trait $StatesLikeVec {
            $(
                fn $fn_name(&mut $_self $(, $arg : $arg_ty)*); // no output type
            )*
        }

        impl<$Item> $impl_for {
            $(
                pub fn $fn_name(&mut $_self $(, $arg : $arg_ty)*) $(-> $output)? {
                    $proxy_for.$fn_name($($arg),*);
                    $real_vec.$fn_name($($arg),*)
                }
            )*
        }

        impl $StatesLikeVec for $AllStates {
            $(
                fn $fn_name(&mut self $(, $arg : $arg_ty)*) {
                    self.$AllStates_proxy_with(|states| states.$fn_name($($arg),*))
                }
            )*
        }
    };
}

like_vec!(
    type Item = E;

    trait StatesLikeVec {
        fn clear(&mut self);
        fn swap(&mut self, a: usize, b: usize);
        fn remove(&mut self, index: usize) -> E;
        fn swap_remove(&mut self, index: usize) -> E;
    }

    impl<__> SyncedCollection<Vec<E>> {
        proxy!(self.all_states.get_mut());
        real_vec!(self.items);
    }

    impl<__> AllStates {
        proxy_with!(for_each_alive_mut);
    }
);

/// Only [`Deref`] is implemented.
impl<ES> Deref for SyncedCollection<ES> {
    type Target = ES;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl<ES: Index<Idx>, Idx> Index<Idx> for SyncedCollection<ES> {
    type Output = ES::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        self.items.index(index)
    }
}

impl<ES: IndexMut<usize>> IndexMut<usize> for SyncedCollection<ES> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.all_states.get_mut().mark_index_as_updated(index);
        self.items.index_mut(index)
    }
}

trait CollectionWithCount {
    fn count(&self) -> usize;
}

macro_rules! impl_for_collection {
    (
        impl<$T:ident> $Trait:ident for
            each_of![$($for_ty:ty),+ $(,)?]
        $impl_block:tt
    ) => {
        $(
            impl<$T> $Trait for $for_ty
            $impl_block
        )+
    };
}

impl_for_collection!(
    impl<T> CollectionWithCount
        for each_of![
            Vec<T>,
            std::collections::VecDeque<T>,
            std::collections::LinkedList<T>,
        ]
    {
        #[inline(always)]
        fn count(&self) -> usize {
            self.len()
        }
    }
);

impl<ES: CollectionWithCount + Extend<A>, A> Extend<A> for SyncedCollection<ES> {
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        let old_len = self.items.count();
        self.items.extend(iter);
        let new_len = self.items.count();
        let count = new_len - old_len;
        self.all_states.get_mut().extend(count);
    }
}

/// copied from [std::slice::range]
fn parse_range<R>(range: &R, len: usize) -> std::ops::Range<usize>
where
    R: std::ops::RangeBounds<usize>,
{
    use std::ops;
    let start = match range.start_bound() {
        ops::Bound::Included(&start) => start,
        ops::Bound::Excluded(start) => start.checked_add(1).unwrap(),
        ops::Bound::Unbounded => 0,
    };

    let end = match range.end_bound() {
        ops::Bound::Included(end) => end.checked_add(1).unwrap(),
        ops::Bound::Excluded(&end) => end,
        ops::Bound::Unbounded => len,
    };

    start..end
}

impl<T> SyncedCollection<Vec<T>> {
    pub fn push(&mut self, value: T) {
        self.items.push(value);
        self.all_states.get_mut().extend(1);
    }

    pub fn splice<R, I>(
        &mut self,
        range: R,
        replace_with: I,
    ) -> std::vec::Splice<'_, splice::SpliceReplaceWith<'_, I::IntoIter>>
    where
        R: std::ops::RangeBounds<usize>,
        I: IntoIterator<Item = T>,
    {
        let parsed_range = parse_range(&range, self.items.len());

        self.items.splice(
            range,
            splice::SpliceReplaceWith {
                iter: replace_with.into_iter(),
                all_states: &mut self.all_states,
                range: parsed_range,
                count: 0,
            },
        )
    }

    pub fn drain<R>(&mut self, range: R) -> drain::Drain<'_, T>
    where
        R: std::ops::RangeBounds<usize>,
    {
        let drain_range = parse_range(&range, self.items.len());

        drain::Drain::new(&mut self.all_states, self.items.drain(range), drain_range)
    }

    pub fn as_mut_slice_and_mark_all_outdated(&mut self) -> &mut [T] {
        self.mark_all_as_outdated();
        self.items.as_mut_slice()
    }

    pub fn as_mut_slice_range_and_mark_outdated<R: std::ops::RangeBounds<usize>>(
        &mut self,
        range: R,
    ) -> &mut [T] {
        let range = parse_range(&range, self.items.len());
        let slice = &mut self.items[range.clone()];

        self.all_states.get_mut().mark_range_as_outdated(&range);

        slice
    }

    pub fn get2_mut(&mut self, a: usize, b: usize) -> Option<(&mut T, &mut T)> {
        #[inline(always)]
        fn get2_mut_impl<T>(
            items: &mut [T],
            less: usize,
            greater: usize,
        ) -> Option<(&mut T, &mut T)> {
            if greater < items.len() {
                let (first, second) = items.split_at_mut(greater);
                Some((&mut first[less], &mut second[0]))
            } else {
                None
            }
        }

        let res = match a.cmp(&b) {
            std::cmp::Ordering::Less => get2_mut_impl(&mut self.items, a, b),
            std::cmp::Ordering::Greater => get2_mut_impl(&mut self.items, b, a),
            std::cmp::Ordering::Equal => None,
        };

        if res.is_some() {
            let all_states = self.all_states.get_mut();
            all_states.mark_index_as_updated(a);
            all_states.mark_index_as_updated(b);
        }

        res
    }
}

pub mod drain;
pub mod splice;

impl<Items: FromIterator<A>, A> FromIterator<A> for SyncedCollection<Items> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        Self::new(Items::from_iter(iter))
    }
}

mod render_states {
    use std::pin::Pin;

    use frender_csr::RenderState;

    use super::{StatesCommon, StatesLikeVec};

    pub(super) enum MountState {
        MountedAndUpToDate,
        MountedAndUpToDateButPreviousWasSkipped,
        Outdated,
        OutdatedAndPreviousWasSkipped,
        OutdatedAndMoved,
        // UpdateToDateButMoved,
    }

    impl MountState {
        // caller should mark the next one as previous_was_skipped
        // There might be a UpdateToDateButMoved variant so this method is different from mark_as_outdated_and_moved
        pub(crate) fn mark_as_moved(&mut self) {
            *self = Self::OutdatedAndMoved
        }

        // caller should mark the next one as previous_was_skipped
        pub(crate) fn mark_as_outdated_and_moved(&mut self) {
            *self = Self::OutdatedAndMoved
        }

        pub(crate) fn mark_as_outdated(&mut self) {
            *self = match self {
                MountState::MountedAndUpToDate => Self::Outdated,
                MountState::Outdated => Self::Outdated,
                MountState::OutdatedAndMoved => MountState::OutdatedAndMoved,
                MountState::MountedAndUpToDateButPreviousWasSkipped => {
                    MountState::OutdatedAndPreviousWasSkipped
                }
                MountState::OutdatedAndPreviousWasSkipped => {
                    MountState::OutdatedAndPreviousWasSkipped
                }
            }
        }

        pub(crate) fn mark_previous_was_skipped(&mut self) {
            match self {
                MountState::MountedAndUpToDate => {
                    *self = MountState::MountedAndUpToDateButPreviousWasSkipped
                }
                MountState::Outdated => *self = MountState::OutdatedAndPreviousWasSkipped,
                _ => {}
            }
        }

        pub(crate) fn needs_reposition(&self) -> NeedsReposition {
            match self {
                MountState::MountedAndUpToDate => NeedsReposition::No {
                    previous_skipped: false,
                },
                MountState::MountedAndUpToDateButPreviousWasSkipped => NeedsReposition::No {
                    previous_skipped: true,
                },
                MountState::Outdated => NeedsReposition::No {
                    previous_skipped: false,
                },
                MountState::OutdatedAndPreviousWasSkipped => NeedsReposition::No {
                    previous_skipped: true,
                },
                MountState::OutdatedAndMoved => NeedsReposition::Yes,
            }
        }
    }

    pub(crate) enum NeedsReposition {
        Yes,
        No { previous_skipped: bool },
    }

    const _: () = assert!(std::mem::size_of::<NeedsReposition>() == 1);

    pub(super) struct Stated<S> {
        pub(super) render_state: S,
        pub(super) mount_state: MountState,
    }
}

mod state {
    use std::{
        any::Any,
        cell::RefCell,
        pin::Pin,
        rc::{Rc, Weak},
        task::Poll,
    };

    use frender_csr::{
        render::{RenderContext, RenderWithContext},
        RenderState, StateUnmount,
    };
    use frender_html::{
        dom::ui_handle::{UiHandle, UnmountedUiHandle},
        experimental::{
            self, RenderStates, UnpinnedRenderStateKind, UnpinnedRenderStateKindPollRender,
        },
        CsrElement, HtmlRenderContext, RenderHtml,
    };

    use super::{
        render_states::{MountState, NeedsReposition},
        RcWithKey, StatesCommon, StatesLikeVec,
    };

    pub(crate) enum State<M, U, NRS, RS> {
        /// The SyncedCollection has inserted an item, but its ui handle hasn't been rendered for real.
        BeforeMounted,
        Mounted {
            non_reactive_state: NRS,
            reactive_state: RS,
            ui_handle: M,
            mount_state: MountState,
        },
        Unmounted {
            non_reactive_state: NRS,
            reactive_state: RS,
            ui_handle: U,
        },
    }

    impl<M, U, NRS, RS> State<M, U, NRS, RS> {
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

        pub(crate) fn from_render_states(
            experimental::RenderStates {
                non_reactive_state,
                reactive_state,
                ui_handle,
            }: experimental::RenderStates<M, NRS, RS>,
        ) -> Self {
            Self::Mounted {
                non_reactive_state,
                reactive_state,
                ui_handle,
                mount_state: MountState::MountedAndUpToDate,
            }
        }

        /// Doesn't trust mount_state.
        /// `update` should reposition if needed
        pub(crate) fn force_render_init_or_update_with<
            E: CsrElement,
            Ctx: ?Sized + HtmlRenderContext,
        >(
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
                UnpinnedNonReactiveState<Ctx::Renderer> = NRS,
                UnpinnedReactiveState = RS,
            >,
        {
            self.render_init_or_update_with(
                || element,
                render_context,
                |get_element, states, mount_state, render_context| {
                    reposition(states.ui_handle, mount_state, render_context);

                    // render_update
                    get_element().unpinned_render_update(render_context, states);

                    *mount_state = MountState::MountedAndUpToDate;
                },
            )
        }

        pub(crate) fn render_init_or_update_with<
            //
            E: CsrElement,
            Ctx: ?Sized + HtmlRenderContext,
            G: FnOnce() -> E,
        >(
            &mut self,
            get_element: G,
            render_context: &mut Ctx,
            process_mounted: impl for<'s> FnOnce(
                G,
                experimental::RenderStates<&'s mut M, &'s mut NRS, &'s mut RS>,
                &mut MountState,
                &mut <Ctx::Renderer as RenderWithContext>::RenderContext<'_>,
            ),
        ) where
            M: UiHandle<Ctx::Renderer, Unmounted = U>,
            U: UnmountedUiHandle<Ctx::Renderer, Mounted = M>,
            E::RenderStateKind: UnpinnedRenderStateKind<
                UnpinnedUiHandle<Ctx::Renderer> = M,
                UnpinnedNonReactiveState<Ctx::Renderer> = NRS,
                UnpinnedReactiveState = RS,
            >,
        {
            match self {
                State::BeforeMounted => {
                    *self = {
                        let RenderStates {
                            ui_handle,
                            non_reactive_state,
                            reactive_state,
                        } = get_element().unpinned_render_init(render_context);
                        Self::Mounted {
                            non_reactive_state,
                            reactive_state,
                            ui_handle,
                            mount_state: MountState::MountedAndUpToDate,
                        }
                    }
                }
                State::Mounted {
                    non_reactive_state,
                    reactive_state,
                    ui_handle,
                    mount_state,
                } => render_context.map_mut_render_context(|render_context| {
                    process_mounted(
                        get_element,
                        experimental::RenderStates {
                            ui_handle,
                            non_reactive_state,
                            reactive_state,
                        },
                        mount_state,
                        render_context,
                    )
                }),
                State::Unmounted { .. } => {
                    let State::Unmounted {
                        mut non_reactive_state,
                        mut reactive_state,
                        ui_handle,
                    } = self.take()
                    else {
                        unreachable!()
                    };

                    let mut ui_handle = render_context
                        .map_mut_render_context(|render_context| ui_handle.mount(render_context));

                    // render_update
                    get_element().unpinned_render_update(
                        render_context,
                        RenderStates {
                            ui_handle: &mut ui_handle,
                            non_reactive_state: &mut non_reactive_state,
                            reactive_state: &mut reactive_state,
                        },
                    );

                    *self = Self::Mounted {
                        non_reactive_state,
                        reactive_state,
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
                UnpinnedNonReactiveState<Ctx::Renderer> = NRS,
                UnpinnedReactiveState = RS,
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
                UnpinnedNonReactiveState<Ctx::Renderer> = NRS,
                UnpinnedReactiveState = RS,
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

        fn take(&mut self) -> Self {
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
                State::Unmounted {
                    non_reactive_state,
                    reactive_state,
                    ui_handle,
                } => {
                    *self = Self::Mounted {
                        non_reactive_state,
                        reactive_state,
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
                    non_reactive_state,
                    reactive_state,
                    ui_handle,
                    mount_state: _,
                } => {
                    *self = Self::Unmounted {
                        non_reactive_state,
                        reactive_state,
                        ui_handle: ui_handle.unmount(renderer),
                    }
                }
                State::Unmounted { .. } => unreachable!(),
            }
        }
    }

    pub(crate) struct States<M, U, NRS, RS> {
        pub(super) states: Vec<State<M, U, NRS, RS>>,
        // last `ready_to_unmount_count` states should be unmounted on next render_update
        pub(super) ready_to_unmount_count: usize,
        pub(super) all_outdated: bool,
    }

    impl<M, U, NRS, RS> States<M, U, NRS, RS> {
        pub(crate) const fn new() -> Self {
            Self {
                states: Vec::new(),
                ready_to_unmount_count: 0,
                all_outdated: false,
            }
        }

        pub(crate) fn real_len(&self) -> usize {
            self.states.len() - self.ready_to_unmount_count
        }
        fn states_mut(&mut self) -> &mut [State<M, U, NRS, RS>] {
            let real_len = self.real_len();
            &mut self.states[..real_len]
        }

        fn insert_many_at(&mut self, at: usize, len: usize) {
            self.extend(len);
            self.states_mut()[at..].rotate_right(len);
        }

        pub(crate) fn clean<R: ?Sized>(
            &mut self,
            renderer: &mut R,
        ) -> &mut Vec<State<M, U, NRS, RS>>
        where
            M: UiHandle<R>,
            RS: StateUnmount + Unpin,
        {
            if self.ready_to_unmount_count > 0 {
                let real_len = self.real_len();
                self.states.drain(real_len..).for_each(|state| match state {
                    State::BeforeMounted => {}
                    State::Mounted {
                        non_reactive_state: _,
                        mut reactive_state,
                        ui_handle,
                        mount_state: _,
                    } => {
                        Pin::new(&mut reactive_state).state_unmount();
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
        <K as UnpinnedRenderStateKind>::UnpinnedNonReactiveState<R>,
        <K as UnpinnedRenderStateKind>::UnpinnedReactiveState,
    >;

    // No item is State::Unmounted
    pub struct UiHandles<C, M, U, NRS, RS> {
        pub(crate) ui_handles: Rc<RefCell<States<M, U, NRS, RS>>>,
        pub(crate) cursor_placeholders: [C; 2],
    }

    impl<C, M, U, NRS, RS> UiHandles<C, M, U, NRS, RS> {
        pub(crate) fn poll_render<K, R>(
            &mut self,
            renderer: &mut R,
            cx: &mut std::task::Context<'_>,
        ) -> Poll<()>
        where
            K: UnpinnedRenderStateKindPollRender<
                UnpinnedUiHandle<R> = M,
                UnpinnedNonReactiveState<R> = NRS,
                UnpinnedReactiveState = RS,
            >,
            R: ?Sized + RenderHtml,
        {
            let mut this = self.ui_handles.borrow_mut();
            let mut res = Poll::Ready(());
            let mounted_len = this.states.len() - this.ready_to_unmount_count;
            for state in &mut this.states[..mounted_len] {
                let State::Mounted {
                    ui_handle,
                    non_reactive_state,
                    reactive_state,
                    mount_state: _,
                } = state
                else {
                    unreachable!()
                };
                if let Poll::Pending = K::unpinned_poll_render(
                    renderer,
                    RenderStates {
                        ui_handle,
                        non_reactive_state,
                        reactive_state,
                    },
                    cx,
                ) {
                    res = Poll::Pending;
                }
            }
            res
        }
    }

    // No item is State::Mounted
    pub struct UnmountedUiHandles<C, M, U, NRS, RS> {
        ui_handles: Rc<RefCell<States<M, U, NRS, RS>>>,
        cursor_placeholders: [C; 2],
    }

    impl<C: UnmountedUiHandle<R>, U: UnmountedUiHandle<R>, NRS, RS, R: ?Sized> UnmountedUiHandle<R>
        for UnmountedUiHandles<C, U::Mounted, U, NRS, RS>
    {
        type Mounted = UiHandles<C::Mounted, U::Mounted, U, NRS, RS>;

        fn mount(self, render_context: &mut <R>::RenderContext<'_>) -> Self::Mounted
        where
            R: frender_html::dom::render::RenderWithContext,
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
                            unreachable!(
                                "UnmountedUiHandles shouldn't contain any Mounted ui handles"
                            )
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

    impl<C: UiHandle<R>, UH: UiHandle<R>, NRS, RS, R: ?Sized> UiHandle<R>
        for UiHandles<C, UH, UH::Unmounted, NRS, RS>
    {
        type Unmounted = UnmountedUiHandles<C::Unmounted, UH, UH::Unmounted, NRS, RS>;

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
            R: frender_html::dom::render::RenderWithContext,
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
            R: frender_html::dom::render::RenderWithContext,
        {
            let [cpa, cpb] = &self.cursor_placeholders;

            cpa.check_and_move_cursor(render_context);
            render_context.mark_cursor_skipped();
            cpb.check_and_move_cursor(render_context);
        }

        fn assert_cursor_is_at_self(&self, render_context: &<R>::RenderContext<'_>)
        where
            R: frender_html::dom::render::RenderWithContext,
        {
            self.cursor_placeholders[0].assert_cursor_is_at_self(render_context)
        }
    }

    pub(crate) trait StateUnmountWithRef {
        type ItemReactiveState;

        fn upcast_rc(self: Rc<Self>) -> Rc<dyn Any>
        where
            Self: 'static;

        fn state_unmount_with_ref(&self);
    }

    impl<M, U, NRS, RS: StateUnmount + Unpin> States<M, U, NRS, RS> {
        fn state_unmount_all(&mut self) {
            self.all_outdated = true; // TODO: is this needed?
            self.states.iter_mut().for_each(|state| {
                Pin::new(match state {
                    State::Mounted { reactive_state, .. } => reactive_state,
                    State::BeforeMounted => return,
                    State::Unmounted { reactive_state, .. } => reactive_state,
                })
                .state_unmount()
            });
        }
    }

    impl<M, U, NRS, RS: StateUnmount + Unpin> StateUnmountWithRef for RefCell<States<M, U, NRS, RS>> {
        type ItemReactiveState = RS;

        fn upcast_rc(self: Rc<Self>) -> Rc<dyn Any>
        where
            Self: 'static,
        {
            self
        }

        fn state_unmount_with_ref(&self) {
            self.borrow_mut().state_unmount_all()
        }
    }

    pub struct ReactiveStates<RS>(
        pub(crate) Option<RcWithKey<dyn StateUnmountWithRef<ItemReactiveState = RS>>>,
    );

    impl<RS> Default for ReactiveStates<RS> {
        fn default() -> Self {
            Self(None)
        }
    }

    impl<RS: StateUnmount + Unpin> StateUnmount for ReactiveStates<RS> {
        fn state_unmount(self: Pin<&mut Self>) {
            let Some(this) = &self.0 else {
                return;
            };
            this.rc.state_unmount_with_ref();
        }
    }

    impl<M, U, NRS, RS> StatesLikeVec for States<M, U, NRS, RS> {
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

    impl<M, U, NRS, RS> StatesCommon for States<M, U, NRS, RS> {
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

    #[cfg(todo)]
    impl<S> Default for States<S> {
        fn default() -> Self {
            Self {
                states: Vec::new(),
                ready_to_unmount_count: 0,
                all_outdated: false,
            }
        }
    }
}

mod to_element {
    use std::cell::RefCell;

    use frender_element::Element;

    use crate::ToElement;

    use super::AllStates;

    pub trait MapItemToElement<Item> {
        type ItemToElement;
        fn map_item_to_element(&mut self, item: Item) -> Self::ItemToElement;
    }

    impl<F, Item, E> MapItemToElement<Item> for F
    where
        F: FnMut(Item) -> E,
    {
        type ItemToElement = E;

        #[inline(always)]
        fn map_item_to_element(&mut self, item: Item) -> Self::ItemToElement {
            self(item)
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct MapItemWithToElement;

    impl<'a, Item: ToElement> MapItemToElement<&'a Item> for MapItemWithToElement {
        type ItemToElement = Item::ToElement<'a>;
        fn map_item_to_element(&mut self, item: &'a Item) -> Self::ItemToElement {
            item.to_element()
        }
    }

    pub struct SyncedCollectionToElement<
        'a,
        ES: Iterator,
        F: MapItemToElement<ES::Item> = MapItemWithToElement,
    > {
        pub(super) all_states: &'a RefCell<AllStates>,
        pub(super) items: ES,
        pub(super) f: F,
    }

    mod ssr {
        use frender_ssr::SsrElement;

        use super::{MapItemToElement, SyncedCollectionToElement};

        impl<'a, ES: Iterator, F: MapItemToElement<ES::Item>> SsrElement
            for SyncedCollectionToElement<'a, ES, F>
        where
            F::ItemToElement: SsrElement,
        {
            type HtmlChildren = async_str_iter::flat::Flat<
                std::vec::IntoIter<<F::ItemToElement as SsrElement>::HtmlChildren>,
            >;

            fn into_html_children(mut self) -> Self::HtmlChildren {
                let children = self
                    .items
                    .map(|el| self.f.map_item_to_element(el).into_html_children())
                    .collect::<Vec<_>>();
                async_str_iter::flat::Flat::new(children.into_iter())
            }
        }
    }

    pub(crate) mod csr {
        use std::{cell::RefCell, marker::PhantomData, pin::Pin, rc::Rc};

        use frender_html::{
            dom::{
                behaviors::{Node as _, NodeRenderSelf},
                ui_handle::UiHandle,
            },
            experimental::{
                self, PinnedRenderStateKind, PinnedRenderStateKindPollRender,
                UnpinnedRenderStateKind, UnpinnedRenderStateKindPollRender,
            },
            CsrElement,
        };

        use crate::elements::synced_collection::{
            state::{ReactiveStates, UiHandles},
            weak_vec1::{self, Key, RcWithKey},
        };

        use super::{
            super::{
                render_states::{MountState, Stated},
                state::{State, StatesOfKind},
            },
            MapItemToElement, SyncedCollectionToElement,
        };

        enum Never {}
        pub struct Kind<K>(Never, PhantomData<K>);

        impl<K: UnpinnedRenderStateKind> UnpinnedRenderStateKind for Kind<K> {
            type UnpinnedUiHandle<R: frender_html::RenderHtml + ?Sized> = UiHandles<
                R::CursorPlaceholder,
                K::UnpinnedUiHandle<R>,
                <K::UnpinnedUiHandle<R> as UiHandle<R>>::Unmounted,
                K::UnpinnedNonReactiveState<R>,
                K::UnpinnedReactiveState,
            >;

            type UnpinnedNonReactiveState<R: frender_html::RenderHtml + ?Sized> = ();

            type UnpinnedReactiveState = ReactiveStates<K::UnpinnedReactiveState>;
        }

        impl<K: UnpinnedRenderStateKindPollRender> UnpinnedRenderStateKindPollRender for Kind<K> {
            fn unpinned_poll_render<R: frender_html::RenderHtml + ?Sized>(
                //
                renderer: &mut R,
                experimental::RenderStates {
                    ui_handle,
                    non_reactive_state: (),
                    reactive_state: _,
                }: experimental::UnpinnedMutRenderStatesOfKind<Self, R>,
                cx: &mut std::task::Context<'_>,
            ) -> std::task::Poll<()> {
                ui_handle.poll_render::<K, R>(renderer, cx)
            }
        }

        impl<K: UnpinnedRenderStateKind> PinnedRenderStateKind for Kind<K> {
            type PinnedUiHandle<R: frender_html::RenderHtml + ?Sized> =
                <Self as UnpinnedRenderStateKind>::UnpinnedUiHandle<R>;
            type PinnedNonReactiveState<R: frender_html::RenderHtml + ?Sized> = ();
            type PinnedReactiveState = ReactiveStates<K::UnpinnedReactiveState>;
        }

        impl<K: UnpinnedRenderStateKindPollRender> PinnedRenderStateKindPollRender for Kind<K> {
            fn pinned_poll_render<R: frender_html::RenderHtml + ?Sized>(
                //
                renderer: &mut R,
                experimental::RenderStates {
                    ui_handle,
                    non_reactive_state,
                    reactive_state,
                }: experimental::PinnedMutRenderStatesOfKind<Self, R>,
                cx: &mut std::task::Context<'_>,
            ) -> std::task::Poll<()> {
                Self::unpinned_poll_render(
                    renderer,
                    experimental::RenderStates {
                        ui_handle,
                        non_reactive_state: non_reactive_state.get_mut(),
                        reactive_state: reactive_state.get_mut(),
                    },
                    cx,
                )
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

            #[cfg(todo)]
            fn render_update_maybe_reposition<Ctx: ?Sized + frender_html::HtmlRenderContext>(
                //
                self,
                render_context: &mut Ctx,
                render_state: Pin<
                    &mut frender_html::RenderStateOfContext<Self::RenderStateKind, Ctx>,
                >,
                force_reposition: bool,
            ) {
                self.unpinned_render_update_maybe_reposition(
                    render_context,
                    render_state.get_mut(),
                    force_reposition,
                )
            }

            #[cfg(todo)]
            fn unpinned_render_update_maybe_reposition<
                Ctx: ?Sized + frender_html::HtmlRenderContext,
            >(
                //
                self,
                render_context: &mut Ctx,
                render_state: &mut State<
                    UnpinnedRenderStateOfContext<
                        <F::ItemToElement as Element>::RenderStateKind,
                        Ctx,
                    >,
                    <Ctx::Renderer as frender_html::dom::render::Render>::CursorPlaceholder,
                >,
                force_reposition: bool,
            ) {
                render_state.state_unmounted = false;

                let cursor_placeholder_force_reposition =
                    force_reposition || render_state.render_states.is_none(); // Was unmounted, so must re-mount

                let State {
                    render_states,
                    state_unmounted: _,
                    cursor_placeholders,
                } = render_state;

                let cpb_or_new_cpa = render_context.map_mut_render_context(|render_context| {
                    if let Some((cpa, cpb)) = cursor_placeholders {
                        cpa.readd_self(render_context, cursor_placeholder_force_reposition);
                        Ok(cpb)
                    } else {
                        let new_cpa = NodeRenderSelf::render_self(render_context);
                        Err(new_cpa)
                    }
                });

                self.unpinned_impl(render_context, render_states, force_reposition);

                match cpb_or_new_cpa {
                    Ok(cpb) => render_context.map_mut_render_context(|render_context| {
                        cpb.readd_self(render_context, cursor_placeholder_force_reposition)
                    }),
                    Err(new_cpa) => {
                        let new_cpb = render_context.map_mut_render_context(|render_context| {
                            NodeRenderSelf::render_self(render_context)
                        });
                        *cursor_placeholders = Some((new_cpa, new_cpb));
                    }
                };
            }

            fn pinned_render_init<Ctx: ?Sized + frender_html::HtmlRenderContext>(
                //
                self,
                render_context: &mut Ctx,
                experimental::PinMutRenderInitStates {
                    non_reactive_state: _,
                    reactive_state,
                }: experimental::PinMutRenderInitStatesOfKind<
                    Self::RenderStateKind,
                    Ctx::Renderer,
                >,
            ) -> experimental::PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>
            {
                let reactive_states = reactive_state.get_mut();
                let Self {
                    all_states,
                    items,
                    mut f,
                } = self;
                let all_states = &mut *all_states.borrow_mut();

                let old_key_hint;

                let ui_handle = if let Some(RcWithKey { rc, key }) = reactive_states.0.take() {
                    old_key_hint = key;
                    // The ui handle got dropped (and should have been unmounted before getting dropped)
                    // but the reactive state didn't get dropped (might have state_unmounted).
                    // This case is rare and unexpected but we could make use of it.
                    match rc.upcast_rc().downcast::<RefCell<
                        StatesOfKind<
                            <F::ItemToElement as CsrElement>::RenderStateKind,
                            Ctx::Renderer,
                        >,
                    >>() {
                        Ok(rc) => Ok(rc),
                        Err(_ /* drop the wrong rc */) => Err(()),
                    }
                } else {
                    old_key_hint = Key::STACK;
                    Err(())
                };

                let cpa = render_context.map_mut_render_context(|render_context| {
                    NodeRenderSelf::render_self(render_context)
                });

                let mut ui_handle = ui_handle.or(const {
                    Err(<StatesOfKind<
                        <F::ItemToElement as CsrElement>::RenderStateKind,
                        Ctx::Renderer,
                    >>::new())
                });

                {
                    let ui_handle = match &mut ui_handle {
                        Ok(rc) => &mut *rc.borrow_mut(),
                        Err(v) => v,
                    };
                    ui_handle.ready_to_unmount_count = 0;
                    ui_handle.all_outdated = false;

                    let states = &mut ui_handle.states;

                    let mut unprocessed = 0usize;
                    let mut items = items;
                    for state in states.iter_mut() {
                        if let Some(item) = items.next() {
                            let element = f.map_item_to_element(item);
                            state.force_render_with(element, render_context);
                        } else {
                            unprocessed = states.len() + 1;
                            break;
                        }
                    }

                    if unprocessed == 0 {
                        // there might be remaining items
                        states.extend(items.map(|item| {
                            let element = f.map_item_to_element(item);
                            State::from_render_states(element.unpinned_render_init(render_context))
                        }));
                    } else {
                        // items are empty
                        let drain = states.drain((states.len() - unprocessed)..);
                        if cfg!(debug_assertions) {
                            for state in drain {
                                match state {
                                    State::Mounted { .. } => {
                                        unreachable!(
                                            "old states of SyncedCollectionToElement should have been unmounted earlier"
                                        )
                                    }
                                    _ => {}
                                }
                            }
                        } else {
                            drop(drain);
                        }
                    }
                }

                let ui_handle = ui_handle.unwrap_or_else(|states| Rc::new(RefCell::new(states)));

                let cpb = render_context.map_mut_render_context(|render_context| {
                    NodeRenderSelf::render_self(render_context)
                });

                let key = all_states.put_rc_states_with_old_key_hint(old_key_hint, &ui_handle);

                reactive_states.0 = Some(RcWithKey {
                    rc: ui_handle.clone(),
                    key,
                });
                UiHandles {
                    ui_handles: ui_handle,
                    cursor_placeholders: [cpa, cpb],
                }
            }

            fn pinned_render_update<Ctx: ?Sized + frender_html::HtmlRenderContext>(
                //
                self,
                render_context: &mut Ctx,
                experimental::RenderStates {
                    ui_handle,
                    non_reactive_state,
                    reactive_state,
                }: experimental::PinnedMutRenderStatesOfKind<
                    Self::RenderStateKind,
                    Ctx::Renderer,
                >,
            ) {
                self.unpinned_render_update(
                    render_context,
                    experimental::RenderStates {
                        ui_handle,
                        non_reactive_state: non_reactive_state.get_mut(),
                        reactive_state: reactive_state.get_mut(),
                    },
                )
            }

            fn unpinned_render_init<Ctx: ?Sized + frender_html::HtmlRenderContext>(
                //
                self,
                render_context: &mut Ctx,
            ) -> experimental::UnpinnedRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>
            {
                let mut reactive_state = Default::default();
                let ui_handle = self.pinned_render_init(
                    render_context,
                    experimental::PinMutRenderInitStates {
                        non_reactive_state: Pin::new(&mut ()),
                        reactive_state: Pin::new(&mut reactive_state),
                    },
                );

                experimental::RenderStates {
                    ui_handle,
                    non_reactive_state: (),
                    reactive_state,
                }
            }

            fn unpinned_render_update<Ctx: ?Sized + frender_html::HtmlRenderContext>(
                //
                self,
                render_context: &mut Ctx,
                experimental::RenderStates {
                    ui_handle:
                        UiHandles {
                            ui_handles,
                            cursor_placeholders: [cpa, cpb],
                        },
                    non_reactive_state: (),
                    reactive_state,
                }: experimental::UnpinnedMutRenderStatesOfKind<
                    Self::RenderStateKind,
                    Ctx::Renderer,
                >,
            ) {
                let ReactiveStates(Some(rc_with_key)) = reactive_state else {
                    panic!("ReactiveState of SyncedCollectionToElement have been dropped with the UiHandle not dropped");
                };

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
                                |get_element, states, mount_state, render_context| {
                                    enum SimpleMountState {
                                        UpToDate,
                                        Outdated,
                                        OutdatedAndMoved,
                                    }

                                    let (simple_mount_state, cursor_should_skip) = match mount_state
                                    {
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
                                        use frender_csr::render::RenderContext;
                                        render_context.mark_cursor_skipped()
                                    }

                                    match simple_mount_state {
                                        SimpleMountState::UpToDate => {
                                            states.ui_handle.check_and_move_cursor(render_context);
                                            return;
                                        }
                                        SimpleMountState::Outdated => {}
                                        SimpleMountState::OutdatedAndMoved => {
                                            states.ui_handle.reposition(render_context);
                                        }
                                    }

                                    get_element().unpinned_render_update(render_context, states);
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

                            mounted_state
                                .force_render_with_but_trust_position(element, render_context);
                        } else {
                            unprocessed_mounted = mounted.len() + 1;
                            break;
                        }
                    }

                    if unprocessed_mounted == 0 {
                        // there might be remaining items

                        // shadow
                        let unprocessed_mounted = ();
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

        #[cfg(todo)]
        impl<'a, ES: Iterator, F: MapItemToElement<ES::Item>> SyncedCollectionToElement<'a, ES, F>
        where
            // TODO: make this implied in RenderStateKind, or make RenderState and UnpinnedRenderState 'static
            <F::ItemToElement as CsrElement>::RenderStateKind: 'static,
        {
            // without caring about cursor placeholders
            fn unpinned_impl<Ctx: ?Sized + frender_html::HtmlRenderContext>(
                //
                self,
                render_context: &mut Ctx,
                render_states: &mut Option<
                    RcWithKey<
                        RefCell<
                            RenderStates<
                                UnpinnedRenderStateOfContext<
                                    <F::ItemToElement as Element>::RenderStateKind,
                                    Ctx,
                                >,
                            >,
                        >,
                    >,
                >,
                force_reposition: bool,
            ) {
                let rc_with_old_key = if let Some(render_states) = render_states {
                    if self.all_states.borrow().0.contains(&*render_states) {
                        /*
                        {
                            use frender_html::dom::render::Render;

                            let states = render_states.rc.borrow();

                            render_context.renderer_mut().log(
                                &states
                                    .states
                                    .iter()
                                    .map(|s| match s.mount_state {
                                        MountState::MountedAndUpToDate => "MountedAndUpToDate ",
                                        MountState::Outdated => "Outdated ",
                                        MountState::OutdatedAndMoved => "OutdatedAndMoved ",
                                        MountState::MountedAndUpToDateButPreviousWasSkipped => {
                                            "MountedAndUpToDateButPreviousWasSkipped "
                                        }
                                        MountState::OutdatedAndPreviousWasSkipped => {
                                            "OutdatedAndPreviousWasSkipped "
                                        }
                                    })
                                    .collect::<String>(),
                            );
                            render_context
                                .renderer_mut()
                                .log(&states.ready_to_unmount_count.to_string());
                        }
                        */

                        // render_states is properly synced
                        let render_states = &mut *render_states.borrow_mut();

                        let all_outdated = std::mem::take(&mut render_states.all_outdated);

                        let render_states = render_states.clean(render_context.renderer_mut());

                        let mut render_states = render_states.iter_mut();
                        let mut elements = self.items;
                        let mut f = self.f;

                        let zip = render_states.by_ref().zip(elements.by_ref());

                        if force_reposition {
                            // TODO: we could just unpinned_render_update_force_reposition outdated elements, and just force_reposition UpdateToDateButMoved elements, if not all_outdated
                            zip.for_each(|(render_state, el)| {
                                unpinned_render_update_force_reposition(
                                    f.map_item_to_element(el),
                                    render_context,
                                    render_state,
                                )
                            })
                        } else {
                            if all_outdated {
                                zip.for_each(|(render_state, el)| {
                                    unpinned_render_update(
                                        f.map_item_to_element(el),
                                        render_context,
                                        render_state,
                                    )
                                })
                            } else {
                                // only update outdated elements
                                zip.for_each(|(render_state, el): (&mut _, _)| {
                                    let Stated {
                                        render_state,
                                        mount_state,
                                    } = render_state;

                                    enum SimpleMountState {
                                        UpToDate,
                                        Outdated,
                                        OutdatedAndMoved,
                                    }

                                    let (simple_mount_state, cursor_should_skip) = match mount_state
                                    {
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

                                    if cursor_should_skip {
                                        render_context.mark_cursor_skipped()
                                    }

                                    *mount_state = MountState::MountedAndUpToDate;

                                    let force_reposition = match simple_mount_state {
                                        SimpleMountState::UpToDate => {
                                            render_context.map_mut_render_context(|render_context| {
                                                frender_html::RenderState::check_and_move_cursor(
                                                    render_state,
                                                    render_context,
                                                )
                                            });
                                            return;
                                        }
                                        SimpleMountState::Outdated => false,
                                        SimpleMountState::OutdatedAndMoved => true,
                                    };

                                    let el = f.map_item_to_element(el);

                                    el.unpinned_render_update_maybe_reposition(
                                        render_context,
                                        render_state,
                                        force_reposition,
                                    );
                                })
                            }
                        }

                        assert_eq!(render_states.len(), 0, "too many render states");
                        assert!(elements.next().is_none(), "too many elements");

                        return;
                    } else {
                        // the states are outdated
                        {
                            let states = Rc::get_mut(&mut render_states.rc).unwrap().get_mut();

                            // It should be set to false when finished.
                            // We can assume all_outdated=true in this branch so we set it earlier.
                            states.all_outdated = false;

                            let (mounted, unmounted) = {
                                let real_len = states.real_len();
                                states.states.split_at_mut(real_len)
                            };

                            let mut elements = self.items;
                            let mut f = self.f;

                            let mut mounted = mounted.iter_mut();
                            elements
                                .by_ref()
                                .zip(mounted.by_ref())
                                .for_each(|(el, state)| {
                                    unpinned_render_update(
                                        f.map_item_to_element(el),
                                        render_context,
                                        state,
                                    )
                                });

                            if mounted.len() > 0 {
                                states.ready_to_unmount_count += mounted.len();
                            } else {
                                let mut unmounted = unmounted.iter_mut();

                                elements.by_ref().zip(unmounted.by_ref()).for_each(
                                    |(el, state)| {
                                        unpinned_render_update_force_reposition(
                                            f.map_item_to_element(el),
                                            render_context,
                                            state,
                                        )
                                    },
                                );

                                if unmounted.len() > 0 {
                                    states.ready_to_unmount_count = unmounted.len();
                                } else {
                                    states.ready_to_unmount_count = 0;
                                    states.states.extend(elements.map(|el| {
                                        new_unpinned_render_state(
                                            f.map_item_to_element(el),
                                            render_context,
                                        )
                                    }))
                                }
                            }

                            states.clean(render_context.renderer_mut());
                        }

                        render_states
                    }
                } else {
                    // new
                    let mut f = self.f;
                    let states = RenderStates {
                        states: self
                            .items
                            .map(|el| {
                                new_unpinned_render_state(f.map_item_to_element(el), render_context)
                            })
                            .collect(),
                        ready_to_unmount_count: 0,
                        all_outdated: false,
                    };

                    render_states.insert(RcWithKey {
                        rc: Rc::new(RefCell::new(states)),
                        key: weak_vec1::Key::STACK,
                    })
                };

                rc_with_old_key.key = self
                    .all_states
                    .borrow_mut()
                    .make_rc_states_with_old_key_hint(
                        rc_with_old_key.key,
                        Rc::downgrade(&rc_with_old_key.rc),
                    );
            }
        }
    }

    #[cfg(feature = "ToElement")]
    mod with_to_element {
        use crate::ToElement;

        use super::{super::SyncedCollection, MapItemWithToElement, SyncedCollectionToElement};

        impl<ES, E: ToElement> ToElement for SyncedCollection<ES>
        where
            for<'a> &'a ES: IntoIterator<Item = &'a E>,
            // TODO: make this implied in RenderStateKind, or make RenderState and UnpinnedRenderState 'static
            // E::ToElementRenderStateKind: 'static,
        {
            type ToElement<'a> = SyncedCollectionToElement<'a, <&'a ES as IntoIterator>::IntoIter, MapItemWithToElement>
            where
                Self: 'a;

            fn to_element(&self) -> Self::ToElement<'_> {
                self.to_element_with(MapItemWithToElement)
            }
        }
    }
}

pub type SyncedVec<E> = SyncedCollection<Vec<E>>;

#[allow(non_snake_case)]
pub fn SyncedVec<E>(elements: Vec<E>) -> SyncedVec<E> {
    SyncedCollection::new(elements)
}

impl<ES> SyncedCollection<ES>
where
    for<'a> &'a ES: IntoIterator,
{
    #[inline(always)]
    fn to_element_with<'a, F: to_element::MapItemToElement<<&'a ES as IntoIterator>::Item>>(
        &'a self,
        f: F,
    ) -> SyncedCollectionToElement<'a, <&'a ES as IntoIterator>::IntoIter, F> {
        SyncedCollectionToElement {
            all_states: &self.all_states,
            items: IntoIterator::into_iter(&self.items),
            f,
        }
    }

    pub fn to_element_with_fn<
        'a,
        F: for<'e> FnMut1<<&'e ES as IntoIterator>::Item>,
        // `F` is more restricted than the following bounds but is more developer friendly
        // E: crate::Element,
        // F: FnMut(<&'a ES as IntoIterator>::Item) -> E,
    >(
        &'a self,
        f: F,
    ) -> SyncedCollectionToElement<'a, <&'a ES as IntoIterator>::IntoIter, F> {
        self.to_element_with(f)
    }
}

// TODO: rename to make_*
/// An identity fn
#[inline(always)]
pub const fn synced_collection_to_elements<
    ES,
    V: ?Sized,
    F: for<'a> FnMut(&'a V) -> SyncedCollectionToElement<'a, <&'a ES as IntoIterator>::IntoIter, F2>,
    F2: for<'a> FnMut1<<&'a ES as IntoIterator>::Item>,
>(
    f: F,
) -> F
where
    for<'a> &'a ES: IntoIterator,
{
    f
}

// TODO: rename to make_*
/// An identity fn
#[inline(always)]
pub const fn synced_vec_to_elements<
    T,
    V: ?Sized,
    F: for<'a> FnMut(&'a V) -> SyncedCollectionToElement<'a, std::slice::Iter<'a, T>, F2>,
    F2: for<'a> FnMut1<&'a T>,
>(
    f: F,
) -> F {
    synced_collection_to_elements::<Vec<T>, V, F, F2>(f)
}
