use std::{collections::HashMap, fmt::Debug, hash::Hash, pin::Pin};

use indexmap::{IndexMap, IndexSet};

use frender_common::{Elements, Keyed};

use crate::{Element, RenderState};

pub struct KeyedElementsState<K: Hash + Eq, S: Unpin> {
    imp: link::ListAndSorted<K, S, link::RealIndexMap<K, S>>,
}

mod link {
    use std::{
        collections::{BinaryHeap, HashMap},
        hash::Hash,
        marker::PhantomData,
        pin::Pin,
    };

    use frender_common::Keyed;

    struct CursorOfLinkedItems {
        first_index: usize,
    }

    struct ItemInfo {
        index: usize,
        next: usize,
    }

    struct IndexMapWithHashMap<K, V> {
        map: HashMap<K, ItemInfo>,
        vec: Vec<V>,
    }

    pub struct Node<Value, Index = usize> {
        value: Value,
        prev: Index,
        next: Index,
    }

    type Position<Index = usize> = Node<(), Index>;

    impl<Value, Index> Node<Value, Index> {
        fn as_mut(&mut self) -> Node<&mut Value, &mut Index> {
            Node {
                value: &mut self.value,
                prev: &mut self.prev,
                next: &mut self.next,
            }
        }

        fn as_mut_position(&mut self) -> Node<(), &mut Index> {
            Node {
                value: (),
                prev: &mut self.prev,
                next: &mut self.next,
            }
        }

        fn as_ref(&self) -> Node<&Value, &Index> {
            Node {
                value: &self.value,
                prev: &self.prev,
                next: &self.next,
            }
        }

        fn as_position(&self) -> Position<Index>
        where
            Index: Copy,
        {
            Node {
                value: (),
                prev: self.prev,
                next: self.next,
            }
        }

        fn as_position_deref_copied(&self) -> Position<Index::Target>
        where
            Index: std::ops::Deref,
            Index::Target: Copy,
        {
            Position {
                value: (),
                prev: *self.prev,
                next: *self.next,
            }
        }
    }

    pub(super) struct RealIndexMap<K, V> {
        map: indexmap::IndexMap<K, Node<V>>,
    }

    impl<K, V> GetMutPositionByIndex for RealIndexMap<K, V> {
        fn get_mut_position_by_index(&mut self, index: usize) -> Node<(), &mut usize> {
            self.map.get_index_mut(index).unwrap().1.as_mut_position()
        }

        fn get_position_by_index(&self, index: usize) -> Node<(), usize> {
            self.map.get_index(index).unwrap().1.as_position()
        }

        fn get_mut_next_by_index(&mut self, index: usize) -> &mut usize {
            &mut self.map.get_index_mut(index).unwrap().1.next
        }

        fn get_mut_prev_by_index(&mut self, index: usize) -> &mut usize {
            &mut self.map.get_index_mut(index).unwrap().1.prev
        }
    }

    #[cfg(test)]
    mod tests {
        use std::borrow::Cow;

        use super::*;

        type Key = Cow<'static, str>;
        type Item = u32;
        type State = u32;

        const ORIGINAL: [(Key, Item); 5] = [
            (Cow::Borrowed("a"), 0),
            (Cow::Borrowed("b"), 1),
            (Cow::Borrowed("c"), 2),
            (Cow::Borrowed("d"), 3),
            (Cow::Borrowed("e"), 4),
        ];

        fn get_original_state<Impl: IndexMapForStates<Key, State>>(
        ) -> ListAndSorted<Key, State, Impl> {
            ListAndSorted::<Key, State, Impl>::from_entries(ORIGINAL)
        }

        fn clone_key_state<K: Clone, S: Clone>((key, state): (&K, &S)) -> (K, S) {
            (key.clone(), state.clone())
        }

        mod asserts {
            use super::*;

            #[derive(Debug, PartialEq, Eq)]
            struct Updated {
                old_state: State,
                new_item: Item,
                force_reposition: bool,
            }

            #[derive(Debug, Default, PartialEq)]
            struct Records {
                into: Vec<Item>,
                updated: Vec<Updated>,
                unmounted: Vec<State>,
            }

            impl Records {
                fn record_while_updating_states<
                    States: IndexMapForStates<Key, State>,
                    E: Iterator<Item = Keyed<Key, Item>>,
                >(
                    &mut self,
                    states: &mut ListAndSorted<Key, State, States>,
                    entries: E,
                ) {
                    states.update(
                        entries,
                        |v, _| {
                            self.into.push(v);
                            v
                        },
                        |v, _, state, force_reposition| {
                            self.updated.push(Updated {
                                old_state: *state,
                                new_item: v,
                                force_reposition,
                            });
                            *state.get_mut() = v;
                        },
                        |state| self.unmounted.push(*state),
                        &mut (),
                    )
                }
            }

            fn original<Impl: IndexMapForStates<Key, State>>() {
                let values = get_original_state::<Impl>()
                    .iter_ordered()
                    .map(|(k, &v)| (k.clone(), v))
                    .collect::<Vec<_>>();

                assert_eq!(values, ORIGINAL);
            }

            fn update_with_unchanged<Impl: IndexMapForStates<Key, State>>() {
                let mut state = get_original_state::<Impl>();
                let mut into = vec![];
                let mut updated = vec![];
                let mut unmounted = vec![];
                state.update(
                    ORIGINAL.map(|(k, v)| Keyed(k, v)).into_iter(),
                    |v, _| {
                        into.push(v);
                        v
                    },
                    |v, _, state, force_reposition| {
                        updated.push(Updated {
                            old_state: *state,
                            new_item: v,
                            force_reposition,
                        });
                        *state.get_mut() = v;
                    },
                    |v| unmounted.push(*v),
                    &mut (),
                );

                assert_eq!(
                    state
                        .iter_ordered()
                        .map(clone_key_state)
                        .collect::<Vec<_>>(),
                    ORIGINAL
                );

                assert!(into.is_empty());
                assert_eq!(
                    updated,
                    ORIGINAL.map(|(key, v)| Updated {
                        old_state: v,
                        new_item: v,
                        force_reposition: false
                    })
                );
                assert_eq!(unmounted, [] as [u32; 0]);
            }

            fn append<Impl: IndexMapForStates<Key, State>>() {
                let mut states = get_original_state::<Impl>();
                let mut records = Records::default();

                let entries = ORIGINAL
                    .into_iter()
                    .chain([(ORIGINAL.len().to_string().into(), ORIGINAL.len() as Item)])
                    .map(Keyed::from_tuple);

                records.record_while_updating_states(&mut states, entries);

                assert_eq!(
                    states
                        .iter_ordered()
                        .map(clone_key_state)
                        .collect::<Vec<_>>(),
                    ORIGINAL
                        .into_iter()
                        .chain([(ORIGINAL.len().to_string().into(), ORIGINAL.len() as Item)])
                        .collect::<Vec<_>>()
                );

                assert_eq!(
                    records,
                    Records {
                        into: vec![ORIGINAL.len() as Item],
                        updated: ORIGINAL
                            .map(|(_, item)| Updated {
                                old_state: item,
                                new_item: item,
                                force_reposition: false
                            })
                            .into_iter()
                            .collect(),
                        unmounted: vec![]
                    }
                );
            }

            fn prepend<Impl: IndexMapForStates<Key, State>>() {
                let mut states = get_original_state::<Impl>();
                let mut records = Records::default();

                records.record_while_updating_states(
                    &mut states,
                    [(ORIGINAL.len().to_string().into(), ORIGINAL.len() as Item)]
                        .into_iter()
                        .chain(ORIGINAL.into_iter())
                        .map(Keyed::from_tuple),
                );

                assert_eq!(
                    states.clone_ordered(),
                    [(ORIGINAL.len().to_string().into(), ORIGINAL.len() as Item)]
                        .into_iter()
                        .chain(ORIGINAL.into_iter())
                        .collect::<Vec<_>>()
                );

                assert_eq!(
                    records,
                    Records {
                        into: vec![ORIGINAL.len() as Item],
                        updated: ORIGINAL
                            .into_iter()
                            .map(|(_, item)| Updated {
                                old_state: item,
                                new_item: item,
                                force_reposition: false
                            })
                            .collect::<Vec<_>>(),
                        unmounted: vec![]
                    }
                );
            }

            fn prepend_and_update<Impl: IndexMapForStates<Key, State>>() {
                let mut states = get_original_state::<Impl>();
                let mut records = Records::default();
                let expected = [(ORIGINAL.len().to_string().into(), ORIGINAL.len() as Item)]
                    .into_iter()
                    .chain(ORIGINAL.into_iter().map(|(key, item)| (key, item + 1)))
                    .collect::<Vec<_>>();

                records.record_while_updating_states(
                    &mut states,
                    expected.clone().into_iter().map(Keyed::from_tuple),
                );

                assert_eq!(states.clone_ordered(), expected);

                assert_eq!(
                    records,
                    Records {
                        into: vec![ORIGINAL.len() as Item],
                        updated: ORIGINAL
                            .into_iter()
                            .map(|(_, item)| Updated {
                                old_state: item,
                                new_item: item + 1,
                                force_reposition: false
                            })
                            .collect::<Vec<_>>(),
                        unmounted: vec![]
                    }
                );
            }

            pub(super) fn swap<Impl: IndexMapForStates<Key, State>>() {
                let mut states = get_original_state::<Impl>();
                let mut records = Records::default();

                let swapped = {
                    let mut swapped = ORIGINAL;
                    swapped.swap(1, ORIGINAL.len() - 2);
                    swapped
                };

                records.record_while_updating_states(
                    &mut states,
                    swapped.clone().map(Keyed::from_tuple).into_iter(),
                );

                assert_eq!(states.clone_ordered(), swapped);

                assert_eq!(
                    records,
                    Records {
                        into: vec![ORIGINAL.len() as Item],
                        updated: ORIGINAL
                            .into_iter()
                            .map(|(_, item)| Updated {
                                old_state: item,
                                new_item: item + 1,
                                force_reposition: false
                            })
                            .collect::<Vec<_>>(),
                        unmounted: vec![]
                    }
                );
            }

            pub(super) fn all<Impl: IndexMapForStates<Key, State>>() {
                original::<Impl>();
                update_with_unchanged::<Impl>();
                append::<Impl>();
                prepend::<Impl>();
                prepend_and_update::<Impl>();
                swap::<Impl>();
            }
        }

        #[test]
        fn real_index_map() {
            asserts::swap::<RealIndexMap<_, _>>()
        }
    }

    pub(super) struct RealIndexMapIntoIterOrdered<K, V> {
        inner: indexmap::map::IntoIter<K, (V, usize)>,
    }

    impl<K, V> Iterator for RealIndexMapIntoIterOrdered<K, V> {
        type Item = (K, V);

        fn next(&mut self) -> Option<Self::Item> {
            self.inner.next().map(|(k, (v, _))| (k, v))
        }
    }

    pub(super) struct RealIndexMapIterOrdered<'a, K, V> {
        inner: indexmap::map::Iter<'a, K, (V, usize)>,
    }

    impl<'a, K, V> Iterator for RealIndexMapIterOrdered<'a, K, V> {
        type Item = (&'a K, &'a V);

        fn next(&mut self) -> Option<Self::Item> {
            self.inner.next().map(|(k, (v, _))| (k, v))
        }
    }

    impl<K: Hash + Eq, V: Unpin> IndexMapForStates<K, V> for RealIndexMap<K, V> {
        fn len(&self) -> usize {
            self.map.len()
        }

        fn is_empty(&self) -> bool {
            self.map.is_empty()
        }

        fn for_each_value_pin_mut<F: FnMut(Pin<&mut V>)>(&mut self, mut f: F) {
            for Node { value, .. } in self.map.values_mut() {
                f(Pin::new(value))
            }
        }

        fn clear(&mut self) {
            self.map.clear()
        }

        fn from_entries<E: IntoIterator<Item = (K, V)>>(entries: E) -> Self {
            let mut map = entries
                .into_iter()
                .enumerate()
                .map(|(i, (key, value))| {
                    (
                        key,
                        Node {
                            value,
                            prev: i.wrapping_sub(1),
                            next: i + 1,
                        },
                    )
                })
                .collect::<indexmap::IndexMap<_, _>>();
            if let Some((
                _,
                Node {
                    next: next_of_last, ..
                },
            )) = map.last_mut()
            {
                *next_of_last = usize::MAX;
            }
            Self { map }
        }

        fn get_index_by_key_with_index_hint(&self, key: &K, index_hint: usize) -> Option<usize> {
            self.map
                .get_index(index_hint)
                .and_then(|(k, _)| if *k == *key { Some(index_hint) } else { None })
                .or_else(|| self.map.get_index_of(key))
        }

        fn add_with_new_key(&mut self, key: K, node: Node<V>) -> usize {
            let res = self.map.insert_full(key, node);
            debug_assert!(res.1.is_none(), "key already exists");
            res.0
        }

        fn get_mut_value_by_index(&mut self, index: usize) -> &mut V {
            &mut self.map.get_index_mut(index).unwrap().1.value
        }

        fn get_mut_node_by_index(&mut self, index: usize) -> Node<&mut V, &mut usize> {
            self.map.get_index_mut(index).unwrap().1.as_mut()
        }

        /// `index` should be valid
        fn get_by_index(&self, index: usize) -> (&K, Node<&V, &usize>) {
            let (key, node) = self.map.get_index(index).unwrap();
            (key, node.as_ref())
        }
    }

    pub enum KeyOrIndex<'a, K> {
        Key(&'a K),
        Index(usize),
    }

    pub(super) enum MutKeyMatchedIndex<'a, K, V> {
        KeyExists {
            key: &'a K,
            value: &'a mut V,
            next: &'a mut usize,
            unmatched_index: Option<usize>,
        },
        KeyNotExists {
            value: &'a mut V,
            next: &'a mut usize,
        },
    }

    pub(super) struct GetMut<'a, V> {
        index: usize,
        value: &'a mut V,
        next_index: &'a mut usize,
    }

    pub(super) trait GetMutPositionByIndex {
        fn get_mut_position_by_index(&mut self, index: usize) -> Node<(), &mut usize>;
        fn get_position_by_index(&self, index: usize) -> Position<usize>;
        fn get_mut_next_by_index(&mut self, index: usize) -> &mut usize;
        fn get_mut_prev_by_index(&mut self, index: usize) -> &mut usize;
    }

    pub(super) trait IndexMapForStates<K, V>: GetMutPositionByIndex {
        fn len(&self) -> usize;
        fn is_empty(&self) -> bool;

        fn for_each_value_pin_mut<F: FnMut(Pin<&mut V>)>(&mut self, f: F);

        fn clear(&mut self);

        fn from_entries<E: IntoIterator<Item = (K, V)>>(entries: E) -> Self;

        // fn get_mut_and_index_by_key(&self, key: &K) -> Option<(&mut V, &mut usize, usize)>;

        // fn get_key_or_index(&self, key: &K, index: usize) -> KeyOrIndex<'_, K>;
        // fn get_mut_by_key_with_index_hint<'a>(
        //     &'a mut self,
        //     key: &K,
        //     index_hint: usize,
        // ) -> Option<GetMut<'a, V>>;

        fn get_index_by_key_with_index_hint(&self, key: &K, index_hint: usize) -> Option<usize>;
        // fn get_value_and_next_mut_by_index(&self, index: usize) -> Option<(&mut V, &mut usize)>;
        fn add_with_new_key(&mut self, key: K, node: Node<V>) -> usize;
        // fn update_value_by_index(&mut self, index: usize, update: impl FnOnce(&mut V));
        fn get_mut_node_by_index(&mut self, index: usize) -> Node<&mut V, &mut usize>;

        fn get_mut_value_by_index(&mut self, index: usize) -> &mut V;
        fn get_by_index(&self, index: usize) -> (&K, Node<&V, &usize>);
    }

    #[cfg(test)]
    pub(super) trait IndexMapForTesting<K, V>: IndexMapForStates<K, V> {
        type IntoIterOrdered: Iterator<Item = (K, V)>;
        fn into_iter_ordered(self) -> Self::IntoIterOrdered;

        type IterOrdered<'a>: Iterator<Item = (&'a K, &'a V)>
        where
            Self: 'a,
            K: 'a,
            V: 'a;

        fn iter_ordered(&self) -> Self::IterOrdered<'_>;
    }

    pub(super) struct ListAndSorted<K, V, IndexMapImpl: IndexMapForStates<K, V>> {
        map: IndexMapImpl,
        first_index: usize,
        _phantom: PhantomData<(K, V)>,
    }

    mod skipped_list {
        use super::*;

        pub(super) struct SkippedList {
            first: usize,
            tail: usize,
        }

        impl SkippedList {
            pub(super) fn new() -> Self {
                Self {
                    first: usize::MAX,
                    tail: usize::MAX,
                }
            }

            /// returns the old position of `index`
            pub(super) fn append(
                &mut self,
                index: usize,
                map: &mut impl GetMutPositionByIndex,
            ) -> Position {
                let pos = map.get_mut_position_by_index(index);
                let old_pos = pos.as_position_deref_copied();
                if self.first == usize::MAX {
                    // empty
                    debug_assert_eq!(self.tail, usize::MAX);
                    self.first = index;
                    self.tail = index;

                    *pos.prev = usize::MAX;
                    *pos.next = usize::MAX;
                } else {
                    *map.get_mut_next_by_index(self.tail) = index;
                    let pos = map.get_mut_position_by_index(index);
                    *pos.prev = self.tail;
                    *pos.next = usize::MAX;
                    self.tail = index;
                }

                if old_pos.prev != usize::MAX {
                    let next = map.get_mut_next_by_index(old_pos.prev);
                    debug_assert_eq!(*next, index);
                    *next = old_pos.next;
                }

                if old_pos.next != usize::MAX {
                    let prev = map.get_mut_prev_by_index(old_pos.next);
                    debug_assert_eq!(*prev, index);
                    *prev = old_pos.prev;
                }

                old_pos
            }

            pub(super) fn pop_front(
                &mut self,
                map: &mut impl GetMutPositionByIndex,
            ) -> Option<usize> {
                if self.first == usize::MAX {
                    // empty
                    debug_assert_eq!(self.tail, usize::MAX);
                    None
                } else {
                    let first = self.first;

                    debug_assert_eq!(*map.get_mut_prev_by_index(first), usize::MAX);

                    let next_of_index = map.get_mut_next_by_index(first);
                    let next = *next_of_index;

                    *next_of_index = usize::MAX;

                    let prev_of_next = map.get_mut_prev_by_index(next);
                    debug_assert_eq!(*prev_of_next, first);
                    *prev_of_next = usize::MAX;

                    Some(first)
                }
            }
        }
    }

    impl<K: Hash + Eq, V: Unpin, IndexMapImpl: IndexMapForStates<K, V>>
        ListAndSorted<K, V, IndexMapImpl>
    {
        pub(super) fn for_each_value_pin_mut<F: FnMut(Pin<&mut V>)>(&mut self, f: F) {
            self.map.for_each_value_pin_mut(f)
        }

        pub(super) fn clear(&mut self) {
            self.map.clear();
            self.first_index = usize::MAX;
        }

        pub(super) fn from_entries<E: IntoIterator<Item = (K, V)>>(entries: E) -> Self {
            let map = IndexMapImpl::from_entries(entries);

            debug_assert!(map.len() < usize::MAX);

            Self {
                map,
                first_index: 0,
                _phantom: PhantomData,
            }
        }

        pub(super) fn update<T, Ctx, E: Iterator<Item = Keyed<K, T>>>(
            &mut self,
            entries: E,
            mut item_into_value: impl FnMut(T, &mut Ctx) -> V,
            mut item_update_value: impl FnMut(T, &mut Ctx, Pin<&mut V>, bool),
            mut value_unmount: impl FnMut(Pin<&mut V>),
            ctx: &mut Ctx,
        ) {
            if self.map.is_empty() {
                self.map = IndexMapImpl::from_entries(
                    entries.map(|Keyed(key, item)| (key, item_into_value(item, ctx))),
                );
                debug_assert!(self.map.len() < usize::MAX);
                self.first_index = 0;

                return;
            }

            // TODO: The following code requires E to impl `ExactSizeIterator`,
            //       This bound might be too strict so this code is disabled for now.
            //       When specialization is stabilized, we can optimize when `E: ExactSizeIterator`
            #[cfg(all(feature = "", not(feature = "")))]
            if entries.len() == 0 {
                self.for_each_value_pin_mut(value_unmount);
                self.clear();
                return;
            }

            debug_assert_ne!(self.first_index, usize::MAX);
            debug_assert!(self.map.len() > self.first_index);

            #[derive(Debug)]
            struct Cursor<'a> {
                current: Option<usize>,
                first: &'a mut usize,
            }

            impl<'a> Cursor<'a> {
                fn current_as_index(&self) -> usize {
                    self.current.unwrap_or(usize::MAX)
                }

                fn get_mut_next_index_of_current<'b>(
                    &'b mut self,
                    map: &'b mut impl GetMutPositionByIndex,
                ) -> &'b mut usize {
                    if let Some(current) = self.current {
                        map.get_mut_next_by_index(current)
                    } else {
                        self.first
                    }
                }
            }

            let mut list = Cursor {
                current: None,
                first: &mut self.first_index,
            };

            #[cfg(all(debug_assertions, target_arch = "wasm32"))]
            gloo::console::warn!("list.first", *list.first);

            let mut skipped_list = skipped_list::SkippedList::new();

            let mut count = 0;

            for Keyed(key, item) in entries {
                count += 1;

                #[cfg(all(debug_assertions, target_arch = "wasm32"))]
                gloo::console::warn!("iter: list = ", format!("{:?}", list));

                let mut next_of_cursor = *list.get_mut_next_index_of_current(&mut self.map);
                if let Some(index) = self
                    .map
                    .get_index_by_key_with_index_hint(&key, next_of_cursor)
                {
                    let force_reposition;

                    // skip `next_of_cursor`
                    loop {
                        if next_of_cursor == usize::MAX {
                            force_reposition = true;
                            break;
                        }

                        if next_of_cursor == index {
                            force_reposition = false;
                            break;
                        };

                        let old_pos = skipped_list.append(next_of_cursor, &mut self.map);

                        next_of_cursor = old_pos.next;
                    }

                    if force_reposition {
                        let pos = self.map.get_position_by_index(index);
                        if pos.prev != usize::MAX {
                            *self.map.get_mut_next_by_index(pos.prev) = pos.next;
                        }
                        if pos.next != usize::MAX {
                            *self.map.get_mut_prev_by_index(pos.next) = pos.prev;
                        }
                    }

                    let node = self.map.get_mut_node_by_index(index);

                    item_update_value(item, ctx, Pin::new(node.value), force_reposition);

                    *node.prev = list.current_as_index();

                    *list.get_mut_next_index_of_current(&mut self.map) = index;

                    list.current = Some(index);
                } else {
                    #[cfg(all(debug_assertions, target_arch = "wasm32"))]
                    gloo::console::warn!("New Key, next_of_cursor = ", next_of_cursor);
                    let index = self.map.add_with_new_key(
                        key,
                        Node {
                            value: item_into_value(item, ctx),
                            prev: list.current_as_index(),
                            next: next_of_cursor,
                        },
                    );
                    *list.get_mut_next_index_of_current(&mut self.map) = index;
                    list.current = Some(index);
                };
            }

            if count == 0 {
                self.for_each_value_pin_mut(value_unmount);
                self.clear();
                return;
            }

            while let Some(skipped_index) = skipped_list.pop_front(&mut self.map) {
                let value = self.map.get_mut_value_by_index(skipped_index);
                value_unmount(Pin::new(value));

                // TODO: swap_remove skipped
            }
        }

        #[cfg(test)]
        fn iter_ordered(&self) -> IterOrdered<'_, K, V, IndexMapImpl> {
            IterOrdered {
                this: self,
                cursor: self.first_index,
            }
        }

        #[cfg(test)]
        fn clone_ordered(&self) -> Vec<(K, V)>
        where
            K: Clone,
            V: Clone,
        {
            self.iter_ordered()
                .map(|(key, state)| (key.clone(), state.clone()))
                .collect::<Vec<_>>()
        }
    }

    pub(super) struct IterOrdered<'a, K, V, IndexMapImpl: IndexMapForStates<K, V>> {
        this: &'a ListAndSorted<K, V, IndexMapImpl>,
        cursor: usize,
    }

    impl<'a, K, V, IndexMapImpl: IndexMapForStates<K, V>> Iterator
        for IterOrdered<'a, K, V, IndexMapImpl>
    {
        type Item = (&'a K, &'a V);

        fn next(&mut self) -> Option<Self::Item> {
            if self.cursor == usize::MAX {
                return None;
            }
            let (
                key,
                Node {
                    value,
                    prev: _,
                    next,
                },
            ) = self.this.map.get_by_index(self.cursor);
            self.cursor = *next;
            Some((key, value))
        }
    }
}

impl<K: Hash + Eq, S: Unpin> Unpin for KeyedElementsState<K, S> {}

impl<K: Hash + Eq, S: RenderState + Unpin> RenderState for KeyedElementsState<K, S> {
    fn unmount(self: std::pin::Pin<&mut Self>) {
        self.get_mut().imp.for_each_value_pin_mut(S::unmount)
    }

    fn state_unmount(self: std::pin::Pin<&mut Self>) {
        self.get_mut().imp.for_each_value_pin_mut(S::state_unmount)
    }

    fn poll_csr(
        self: std::pin::Pin<&mut Self>,
        ctx: &mut crate::CsrContext,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        let mut res = std::task::Poll::Ready(());

        self.get_mut()
            .imp
            .for_each_value_pin_mut(|state| match S::poll_csr(state, ctx, cx) {
                std::task::Poll::Ready(()) => {}
                std::task::Poll::Pending => {
                    res = std::task::Poll::Pending;
                }
            });

        res
    }
}

impl<K, E> Element for Vec<Keyed<K, E>>
where
    K: std::hash::Hash + Eq + Debug,
    E: Element,
    E::CsrState: Unpin,
{
    type CsrState = KeyedElementsState<K, E::CsrState>;

    fn into_csr_state(self, ctx: &mut crate::CsrContext) -> Self::CsrState {
        Elements(self).into_csr_state(ctx)
    }

    fn update_csr_state_maybe_reposition(
        self,
        ctx: &mut crate::CsrContext,
        state: Pin<&mut Self::CsrState>,
        force_reposition: bool,
    ) {
        Elements(self).update_csr_state_maybe_reposition(ctx, state, force_reposition)
    }

    fn update_csr_state(self, ctx: &mut crate::CsrContext, state: Pin<&mut Self::CsrState>)
    where
        Self: Sized,
    {
        Elements(self).update_csr_state(ctx, state)
    }

    fn update_csr_state_force_reposition(
        self,
        ctx: &mut crate::CsrContext,
        state: Pin<&mut Self::CsrState>,
    ) where
        Self: Sized,
    {
        Elements(self).update_csr_state_force_reposition(ctx, state)
    }
}

impl<I, K, E> Element for Elements<I>
where
    I: IntoIterator<Item = Keyed<K, E>>,
    I::IntoIter: ExactSizeIterator,
    K: std::hash::Hash + Eq + Debug,
    E: Element,
    E::CsrState: Unpin,
{
    type CsrState = KeyedElementsState<K, E::CsrState>;

    fn into_csr_state(self, ctx: &mut crate::CsrContext) -> Self::CsrState {
        let states = self
            .0
            .into_iter()
            .map(|Keyed(k, v)| (k, v.into_csr_state(ctx)));
        KeyedElementsState {
            imp: link::ListAndSorted::from_entries(states),
        }
    }

    #[cfg(aaa)]
    #[cfg(not(frender_elements_update_by_swapping))]
    fn update_csr_state(
        self,
        ctx: &mut crate::CsrContext,
        state: std::pin::Pin<&mut Self::CsrState>,
    ) {
        let state = state.get_mut();

        if state.states.is_empty() {
            return *state = self.into_csr_state(ctx);
        }

        let elements = self.0.into_iter();
        if elements.len() == 0 {
            return state.clear();
        }

        let states = &mut state.states;

        let mut old_states = std::mem::replace(states, IndexMap::new());

        let mut removed = HashMap::new();

        let mut cur = 0;

        for Keyed(key, element) in elements {
            if let Some(mut old_idx) = old_states.get_index_of(&key) {
                match old_idx.cmp(&cur) {
                    std::cmp::Ordering::Equal => {}
                    std::cmp::Ordering::Less => unreachable!(),
                    std::cmp::Ordering::Greater => {
                        if old_states.len() - old_idx < old_idx - cur {
                            // drain tail

                            let mut to_remove = old_states.drain(old_idx..);
                            let old = to_remove.next().unwrap();
                            removed.extend(to_remove);

                            states.extend(old_states.drain(..cur).chain(Some(old)));

                            let (key_old, old_state) = states.last_mut().unwrap();

                            debug_assert_eq!(*key_old, key);

                            element.update_csr_state_force_reposition(ctx, Pin::new(old_state));

                            cur = 0;
                            continue;
                        } else {
                            // drain before this
                            let mut old_states_and_to_remove = old_states.drain(..old_idx);

                            states.extend((&mut old_states_and_to_remove).take(cur));
                            removed.extend(old_states_and_to_remove);
                            old_idx = 0;
                            cur = 0;
                        }
                    }
                }

                // "equal" or "greater but drained"
                let state_old = &mut old_states[old_idx];
                element.update_csr_state(ctx, Pin::new(state_old));
                cur += 1;
            } else {
                states.extend(old_states.drain(..cur));
                cur = 0;
                state_vacant_and_then(states, key, |entry| {
                    if let Some(old_state) = removed.remove(entry.key()) {
                        let state = entry.insert(old_state);
                        element.update_csr_state_force_reposition(ctx, Pin::new(state))
                    } else {
                        _ = entry.insert(element.into_csr_state(ctx))
                    }
                });
            }
        }

        old_states
            .drain(cur..)
            .map(|(_, v)| v)
            .chain(removed.into_values())
            .for_each(|ref mut state| Pin::new(state).unmount());

        if states.is_empty() {
            *states = old_states;
        } else {
            states.extend(old_states);
        }
    }

    // #[cfg(frender_elements_update_by_swapping)]
    fn update_csr_state(
        self,
        ctx: &mut crate::CsrContext,
        state: std::pin::Pin<&mut Self::CsrState>,
    ) {
        state.get_mut().imp.update(
            self.0.into_iter(),
            E::into_csr_state,
            E::update_csr_state_maybe_reposition,
            E::CsrState::unmount,
            ctx,
        );
    }

    fn update_csr_state_force_reposition(
        self,
        _: &mut crate::CsrContext,
        _: std::pin::Pin<&mut Self::CsrState>,
    ) {
        panic!("Elements repositioning")
    }

    #[cfg(aaaa)]
    fn update_csr_state_force_reposition(
        self,
        ctx: &mut crate::CsrContext,
        state: std::pin::Pin<&mut Self::CsrState>,
    ) {
        #[cfg(all(debug_assertions, target_arch = "wasm32"))]
        gloo::console::warn!("Elements repositioning");

        let states = &mut state.get_mut().states;

        let elements = self.0.into_iter();

        let mut old_states = std::mem::replace(states, IndexMap::with_capacity(elements.len()));

        for Keyed(key, element) in elements {
            if let Some(state) = old_states.remove(&key) {
                let entry = states.entry(key);

                debug_assert!(matches!(entry, indexmap::map::Entry::Vacant(_)));

                let state = entry.or_insert(state);
                E::update_csr_state_force_reposition(element, ctx, std::pin::Pin::new(state));
            } else {
                states.insert(key, element.into_csr_state(ctx)); // TODO: assert returned is None
            }
        }

        for state in old_states.values_mut().map(std::pin::Pin::new) {
            state.unmount()
        }
    }

    fn update_csr_state_maybe_reposition(
        self,
        ctx: &mut crate::CsrContext,
        state: std::pin::Pin<&mut Self::CsrState>,
        force_reposition: bool,
    ) {
        if force_reposition {
            self.update_csr_state_force_reposition(ctx, state)
        } else {
            self.update_csr_state(ctx, state)
        }
    }
}

/// If `states` contains `key`, then warns.
/// Else, call f.
#[cfg(not(frender_elements_update_by_swapping))]
fn state_vacant_and_then<'a, K, S>(
    states: &'a mut IndexMap<K, S>,
    key: K,
    f: impl FnOnce(indexmap::map::VacantEntry<'a, K, S>),
) where
    K: std::hash::Hash + Eq + Debug,
{
    match states.entry(key) {
        indexmap::map::Entry::Vacant(entry) => f(entry),
        indexmap::map::Entry::Occupied(entry) => {
            #[cfg(not(debug_assertions))]
            let _ = entry;

            #[cfg(all(debug_assertions, target_arch = "wasm32"))]
            gloo::console::warn!(format!(
                "key {:?} has been inserted so the latter element is ignored",
                entry.key()
            ));
        }
    }
}
