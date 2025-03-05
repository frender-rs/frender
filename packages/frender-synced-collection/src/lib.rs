pub use to_element::SyncedCollectionToElement;

use std::{
    cell::RefCell,
    ops::{Deref, Index, IndexMut},
};

use frender_fn_traits::FnMut1;

pub mod drain;
pub mod splice;

mod weak_vec1;

mod to_element;

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

impl AllStates {
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

impl<Items: FromIterator<A>, A> FromIterator<A> for SyncedCollection<Items> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        Self::new(Items::from_iter(iter))
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

    /// An identity fn
    #[inline(always)]
    pub const fn make_fn_mut<
        V: ?Sized,
        F: for<'a> FnMut(
            &'a V,
        )
            -> SyncedCollectionToElement<'a, <&'a ES as IntoIterator>::IntoIter, F2>,
        F2: for<'a> FnMut1<<&'a ES as IntoIterator>::Item>,
    >(
        f: F,
    ) -> F {
        f
    }
}
