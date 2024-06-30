pub use to_element::{csr::Kind, SyncedCollectionToElement};
use weak_vec1::RcWithKey;

use std::{
    cell::RefCell,
    ops::{Deref, Index, IndexMut},
    rc::Weak,
};

mod weak_vec1 {
    use std::rc::{Rc, Weak};

    #[derive(Clone, Copy)]
    pub(super) struct Key(usize);

    impl Key {
        pub(super) const STACK: Self = Self(usize::MAX);
    }

    impl Key {
        fn into_index(self) -> Option<usize> {
            let Self(n) = self;
            (n != usize::MAX).then_some(n)
        }
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

    fn weak_is_of_rc<T: ?Sized, U: ?Sized>(weak: &Weak<T>, rc: &Rc<U>) -> bool {
        std::ptr::addr_eq(Weak::as_ptr(weak), Rc::as_ptr(rc))
    }

    fn is_empty_weak<T: ?Sized>(v: &Option<Weak<T>>) -> bool {
        if let Some(v) = v {
            v.strong_count() == 0
        } else {
            true
        }
    }

    impl<T: ?Sized> WeakVec1<T> {
        pub(super) const DEFAULT: Self = Self(None, Vec::new());
        pub(super) fn contains<U: ?Sized>(&self, v: &RcWithKey<U>) -> bool {
            let weak = match v.key.into_index() {
                None => self.0.as_ref(),
                Some(i) => self.1.get(i).and_then(Option::as_ref),
            };
            weak.map_or(false, |weak| weak_is_of_rc(weak, &v.rc))
        }

        pub(super) fn push_to_empty(&mut self, old_key: Key, v: Weak<T>) -> Key {
            let stack = &mut self.0;

            match old_key.into_index() {
                None => {
                    if is_empty_weak(stack) {
                        *stack = Some(v);
                        Key::STACK
                    } else {
                        let i = self.1.len();
                        self.1.push(None);
                        self.1[i] = Some(v);
                        Key(i)
                    }
                }
                Some(index) => match self.1.get_mut(index) {
                    Some(weak) if is_empty_weak(weak) => {
                        *weak = Some(v);
                        old_key
                    }
                    _ => {
                        if is_empty_weak(stack) {
                            *stack = Some(v);
                            Key::STACK
                        } else {
                            let i = self.1.len();
                            self.1.push(None);
                            self.1[i] = Some(v);
                            Key(i)
                        }
                    }
                },
            }
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
}

impl AllStates {
    fn make_rc_states_with_old_key_hint<S: States + 'static>(
        &mut self,
        old_key_hint: weak_vec1::Key,
        weak: Weak<RefCell<S>>,
    ) -> weak_vec1::Key {
        let key = self
            .0
            .push_to_empty(old_key_hint, weak as Weak<RefCell<dyn States>>);
        key
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
}

trait States: StatesCommon + StatesLikeVec {}

impl<S: ?Sized + StatesCommon + StatesLikeVec> States for S {}

/// All mutations are synced to the registered render states so that before elements `render_update`,
/// the render states can reposition and only `render_update`s the updated elements.
///
/// - [`IndexMut<usize>`] would record a update at the index.
///
///   `elements[i] = new_element` or even just `&mut elements[i]` is a update mutation at `i`.
#[derive(Debug, Default)]
pub struct SyncedCollection<ES> {
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
        Outdated,
        OutdatedAndMoved,
        // UpdateToDateButMoved,
    }

    impl MountState {
        fn mark_as_moved(&mut self) {
            *self = Self::OutdatedAndMoved
        }

        fn mark_as_outdated(&mut self) {
            *self = match self {
                MountState::MountedAndUpToDate => Self::Outdated,
                MountState::Outdated => Self::Outdated,
                MountState::OutdatedAndMoved => MountState::OutdatedAndMoved,
            }
        }
    }

    pub(super) struct Stated<S> {
        pub(super) render_state: S,
        pub(super) mount_state: MountState,
    }

    pub(super) struct RenderStates<S> {
        pub(super) states: Vec<Stated<S>>,
        // last `ready_to_unmount_count` states should be unmounted on next render_update
        pub(super) ready_to_unmount_count: usize,
    }

    impl<S> RenderStates<S> {
        pub(super) fn real_len(&self) -> usize {
            self.states.len() - self.ready_to_unmount_count
        }
        fn states_mut(&mut self) -> &mut [Stated<S>] {
            let real_len = self.real_len();
            &mut self.states[..real_len]
        }
        pub(super) fn clean<R: ?Sized>(&mut self, renderer: &mut R) -> &mut Vec<Stated<S>>
        where
            S: RenderState<R> + Unpin,
        {
            if self.ready_to_unmount_count > 0 {
                let real_len = self.real_len();
                self.states[real_len..]
                    .iter_mut()
                    .for_each(|state| S::unmount(Pin::new(&mut state.render_state), renderer));
                self.states.truncate(real_len);
                self.ready_to_unmount_count = 0;
            }
            &mut self.states
        }

        fn insert_many_at(&mut self, at: usize, len: usize)
        where
            S: Default,
        {
            self.extend(len);
            self.states_mut()[at..].rotate_right(len);
        }
    }

    impl<S> StatesLikeVec for RenderStates<S> {
        fn clear(&mut self) {
            self.ready_to_unmount_count = self.states.len();
        }

        fn swap(&mut self, a: usize, b: usize) {
            let states = self.states_mut();

            states.swap(a, b);
            states[a].mount_state.mark_as_moved();
            states[b].mount_state.mark_as_moved();
        }

        fn remove(&mut self, index: usize) {
            // not real remove
            self.states_mut()[index..].rotate_left(1);
            self.ready_to_unmount_count += 1;
        }

        fn swap_remove(&mut self, index: usize) {
            // not real remove
            let states = self.states_mut();
            states.swap(index, states.len() - 1);
            states[index].mount_state.mark_as_moved();
            self.ready_to_unmount_count += 1;
        }
    }

    impl<S: Default> StatesCommon for RenderStates<S> {
        fn mark_index_as_updated(&mut self, i: usize) {
            self.states_mut()[i].mount_state.mark_as_outdated()
        }

        fn extend(&mut self, len: usize) {
            if len > self.ready_to_unmount_count {
                self.ready_to_unmount_count = 0;
                let new_count = len - self.ready_to_unmount_count;
                self.states.extend(
                    std::iter::repeat_with(|| Stated {
                        render_state: S::default(),
                        mount_state: MountState::Outdated,
                    })
                    .take(new_count),
                );
            } else {
                // the items should already be marked as outdated
                //
                // let from = self.real_len();
                // self.states[from..(from + len)]
                //     .iter_mut()
                //     .for_each(|s| s.mount_state.mark_as_outdated());

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
                    .for_each(|state| state.mount_state.mark_as_outdated());

                self.drain(drain_start..(range.end))
            } else {
                let end = range.end;
                self.states[range]
                    .iter_mut()
                    .for_each(|state| state.mount_state.mark_as_outdated());

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
                    .for_each(|state| state.mount_state = MountState::OutdatedAndMoved);
                states.rotate_left(removed_len);
            } else {
                states
                    .iter_mut()
                    .for_each(|state| state.mount_state.mark_as_outdated());
            }
            self.ready_to_unmount_count += removed_len;
        }
    }

    impl<S> Default for RenderStates<S> {
        fn default() -> Self {
            Self {
                states: Vec::new(),
                ready_to_unmount_count: 0,
            }
        }
    }
}

mod state {
    use std::{cell::RefCell, pin::Pin, task::Poll};

    use frender_csr::RenderState;

    use super::{render_states::RenderStates, RcWithKey};

    pub struct State<S> {
        pub(super) render_states: Option<RcWithKey<RefCell<RenderStates<S>>>>,
        pub(super) state_unmounted: bool,
    }

    impl<S> Default for State<S> {
        fn default() -> Self {
            Self {
                render_states: None,
                state_unmounted: false,
            }
        }
    }

    impl<S: RenderState<R> + Unpin, R: ?Sized> RenderState<R> for State<S> {
        fn unmount(self: std::pin::Pin<&mut Self>, renderer: &mut R) {
            let this = self.get_mut();
            if let Some(render_states) = &mut this.render_states {
                render_states
                    .borrow_mut()
                    .states
                    .iter_mut()
                    .for_each(|state| S::unmount(Pin::new(&mut state.render_state), renderer));
            }

            *this = Default::default();
        }

        fn state_unmount(self: std::pin::Pin<&mut Self>) {
            let this = self.get_mut();

            if this.state_unmounted {
                return;
            }

            if let Some(render_states) = &mut this.render_states {
                render_states
                    .borrow_mut()
                    .states
                    .iter_mut()
                    .for_each(|state| S::state_unmount(Pin::new(&mut state.render_state)));
                this.state_unmounted = true;
            }
        }

        fn poll_render(
            self: std::pin::Pin<&mut Self>,
            renderer: &mut R,
            cx: &mut std::task::Context<'_>,
        ) -> Poll<()> {
            match self.get_mut() {
                Self {
                    render_states: Some(render_states),
                    state_unmounted: false,
                } => render_states
                    .borrow_mut()
                    // on poll_render, if !state_unmounted, ready_to_unmount states are unmounted
                    .clean(renderer)
                    .iter_mut()
                    .fold(Poll::Ready(()), |res, state| {
                        match S::poll_render(Pin::new(&mut state.render_state), renderer, cx) {
                            Poll::Ready(()) => res,
                            Poll::Pending => Poll::Pending,
                        }
                    }),
                _ => Poll::Ready(()),
            }
        }
    }
}

mod to_element {
    use std::cell::RefCell;

    use frender_html::Element;

    use crate::ToElement;

    use super::AllStates;

    pub trait MapItemToElement<Item> {
        type ItemToElement: Element;
        fn map_item_to_element(&mut self, item: Item) -> Self::ItemToElement;
    }

    impl<F, Item, E: Element> MapItemToElement<Item> for F
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
        use std::{cell::RefCell, pin::Pin, rc::Rc};

        use frender_html::{
            Element, RenderStateKindPinned, RenderStateKindUnpinned, UnpinnedRenderStateOfContext,
        };

        use crate::elements::synced_collection::weak_vec1::{self, RcWithKey};

        use super::{
            super::{
                render_states::{MountState, RenderStates, Stated},
                state::State,
            },
            MapItemToElement, SyncedCollectionToElement,
        };

        enum Never {}
        pub struct Kind<K>(Never, std::marker::PhantomData<K>);

        impl<K: RenderStateKindUnpinned> RenderStateKindPinned for Kind<K> {
            type RenderState<R: frender_html::RenderHtml + ?Sized> =
                State<K::UnpinnedRenderState<R>>;
        }

        impl<K: RenderStateKindUnpinned> RenderStateKindUnpinned for Kind<K> {
            type UnpinnedRenderState<R: frender_html::RenderHtml + ?Sized> =
                State<K::UnpinnedRenderState<R>>;
        }

        impl<'a, ES: Iterator, F: MapItemToElement<ES::Item>> Element
            for SyncedCollectionToElement<'a, ES, F>
        where
            // TODO: make this implied in RenderStateKind, or make RenderState and UnpinnedRenderState 'static
            <F::ItemToElement as Element>::RenderStateKind: 'static,
        {
            type RenderStateKind = Kind<<F::ItemToElement as Element>::RenderStateKind>;

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
                >,
                force_reposition: bool,
            ) {
                let rc_with_old_key = if let Some(render_states) = &mut render_state.render_states {
                    if self.all_states.borrow().0.contains(&*render_states) {
                        // render_states is up-to-date as of it's order and count
                        let render_states = &mut *render_states.borrow_mut();
                        let render_states = render_states.clean(render_context.renderer_mut());

                        let mut render_states = render_states.iter_mut();
                        let mut elements = self.items;
                        let mut f = self.f;

                        let zip = render_states.by_ref().zip(elements.by_ref());

                        if force_reposition {
                            zip.for_each(|(render_state, el)| {
                                unpinned_render_update_force_reposition(
                                    f.map_item_to_element(el),
                                    render_context,
                                    render_state,
                                )
                            })
                        } else {
                            zip.for_each(|(render_state, el)| {
                                unpinned_render_update(
                                    f.map_item_to_element(el),
                                    render_context,
                                    render_state,
                                )
                            })
                        }

                        assert_eq!(render_states.len(), 0, "too many render states");
                        assert!(elements.next().is_none(), "too many elements");

                        return;
                    } else {
                        // the states are outdated
                        {
                            let states = Rc::get_mut(&mut render_states.rc).unwrap().get_mut();
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
                    };

                    render_state.render_states.insert(RcWithKey {
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

                render_state.state_unmounted = false;
            }
        }

        fn unpinned_render_update<Ctx: ?Sized + frender_html::HtmlRenderContext, E: Element>(
            el: E,
            render_context: &mut Ctx,
            Stated {
                render_state,
                mount_state,
            }: &mut Stated<UnpinnedRenderStateOfContext<E::RenderStateKind, Ctx>>,
        ) {
            let force_reposition = match mount_state {
                MountState::MountedAndUpToDate => return,
                MountState::Outdated => false,
                MountState::OutdatedAndMoved => true,
            };
            el.unpinned_render_update_maybe_reposition(
                render_context,
                render_state,
                force_reposition,
            );
            *mount_state = MountState::MountedAndUpToDate;
        }

        fn unpinned_render_update_force_reposition<
            Ctx: ?Sized + frender_html::HtmlRenderContext,
            E: Element,
        >(
            el: E,
            render_context: &mut Ctx,
            Stated {
                render_state,
                mount_state,
            }: &mut Stated<UnpinnedRenderStateOfContext<E::RenderStateKind, Ctx>>,
        ) {
            el.unpinned_render_update_force_reposition(render_context, render_state);
            *mount_state = MountState::MountedAndUpToDate;
        }

        fn new_unpinned_render_state<Ctx: ?Sized + frender_html::HtmlRenderContext, E: Element>(
            el: E,
            render_context: &mut Ctx,
        ) -> Stated<UnpinnedRenderStateOfContext<E::RenderStateKind, Ctx>> {
            let mut state = Stated {
                render_state: Default::default(),
                mount_state: MountState::MountedAndUpToDate,
            };
            el.unpinned_render_update(render_context, &mut state.render_state);

            state
        }
    }

    #[cfg(feature = "ToElement")]
    mod with_to_element {
        use crate::ToElement;

        use super::{
            super::SyncedCollection, csr::Kind, MapItemWithToElement, SyncedCollectionToElement,
        };

        impl<ES, E: ToElement> ToElement for SyncedCollection<ES>
        where
            for<'a> &'a ES: IntoIterator<Item = &'a E>,
            // TODO: make this implied in RenderStateKind, or make RenderState and UnpinnedRenderState 'static
            E::ToElementRenderStateKind: 'static,
        {
            type ToElement<'a> = SyncedCollectionToElement<'a, <&'a ES as IntoIterator>::IntoIter, MapItemWithToElement>
            where
                Self: 'a;

            fn to_element(&self) -> Self::ToElement<'_> {
                self.to_element_with(MapItemWithToElement)
            }

            type ToElementHtmlChildren =
                async_str_iter::flat::Flat<std::vec::IntoIter<E::ToElementHtmlChildren>>;

            type ToElementRenderStateKind = Kind<E::ToElementRenderStateKind>;
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
        E: crate::Element,
        F: FnMut(<&'a ES as IntoIterator>::Item) -> E,
    >(
        &'a self,
        f: F,
    ) -> SyncedCollectionToElement<'a, <&'a ES as IntoIterator>::IntoIter, F> {
        self.to_element_with(f)
    }
}
