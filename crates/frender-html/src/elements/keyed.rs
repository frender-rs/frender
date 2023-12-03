use std::pin::Pin;

use frender_common::{Elements, Keyed};

use crate::{Element, RenderHtml, RenderState};

pub trait ElementsAlgorithm<K, E> {
    type CsrState<R: RenderHtml>: RenderState<R> + Unpin + Default;

    fn keyed_elements_update_csr_state<I: IntoIterator<Item = Keyed<K, E>>, R: RenderHtml>(self, keyed_elements: I, renderer: &mut R, state: Pin<&mut Self::CsrState<R>>);

    /// The element needs to be repositioned (re-add to the ctx)
    fn keyed_elements_update_csr_state_force_reposition<I: IntoIterator<Item = Keyed<K, E>>, R: RenderHtml>(self, keyed_elements: I, renderer: &mut R, state: Pin<&mut Self::CsrState<R>>);

    fn keyed_elements_update_csr_state_maybe_reposition<I: IntoIterator<Item = Keyed<K, E>>, R: RenderHtml>(self, keyed_elements: I, renderer: &mut R, state: Pin<&mut Self::CsrState<R>>, force_reposition: bool)
    where
        Self: Sized,
    {
        if force_reposition {
            self.keyed_elements_update_csr_state_force_reposition(keyed_elements, renderer, state)
        } else {
            self.keyed_elements_update_csr_state(keyed_elements, renderer, state)
        }
    }
}

pub mod default {
    use std::{collections::HashMap, hash::Hash, pin::Pin};

    use frender_common::{DefaultElementsAlgorithm, Keyed};
    use indexmap::IndexMap;

    use crate::{Element, RenderHtml, RenderState};

    use super::ElementsAlgorithm;

    pub struct States<K, S>(IndexMap<K, S>);

    impl<K, S> Default for States<K, S> {
        fn default() -> Self {
            Self(Default::default())
        }
    }

    impl<K, S> Unpin for States<K, S> {}

    impl<K, S: RenderState<R> + Unpin, R> RenderState<R> for States<K, S> {
        fn unmount(self: Pin<&mut Self>, renderer: &mut R) {
            let this = self.get_mut();
            for state in this.0.values_mut() {
                S::unmount(Pin::new(state), renderer)
            }
            this.0 = Default::default();
        }

        fn state_unmount(self: Pin<&mut Self>) {
            for state in self.get_mut().0.values_mut() {
                S::state_unmount(Pin::new(state))
            }
        }

        fn poll_render(self: Pin<&mut Self>, renderer: &mut R, cx: &mut std::task::Context<'_>) -> std::task::Poll<()> {
            let mut res = std::task::Poll::Ready(());

            for state in self.get_mut().0.values_mut() {
                match S::poll_render(std::pin::Pin::new(state), renderer, cx) {
                    std::task::Poll::Ready(()) => {}
                    v @ std::task::Poll::Pending => {
                        res = v;
                    }
                }
            }

            res
        }
    }

    impl<K: Hash + Eq, E: Element> ElementsAlgorithm<K, E> for DefaultElementsAlgorithm {
        type CsrState<R: RenderHtml> = States<K, E::UnpinnedRenderState<R>>;

        fn keyed_elements_update_csr_state<I: IntoIterator<Item = Keyed<K, E>>, R: RenderHtml>(self, keyed_elements: I, renderer: &mut R, state: Pin<&mut Self::CsrState<R>>) {
            let state = state.get_mut();

            let elements = keyed_elements.into_iter();

            // TODO: specialize for ExactSizeIterator
            // if elements.len() == 0 {
            //     return state.clear();
            // }

            let states = &mut state.0;

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

                                debug_assert!(*key_old == key);

                                element.unpinned_render_update_force_reposition(renderer, old_state);

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
                    let state_old: &mut <E as Element>::UnpinnedRenderState<R> = &mut old_states[old_idx];
                    element.unpinned_render_update(renderer, state_old);
                    cur += 1;
                } else {
                    states.extend(old_states.drain(..cur));
                    cur = 0;
                    state_vacant_and_then(states, key, |entry| {
                        let old_state = if let Some(old_state) = removed.remove(entry.key()) { old_state } else { Default::default() };
                        let state = entry.insert(old_state);
                        element.unpinned_render_update_force_reposition(renderer, state)
                    });
                }
            }

            old_states.drain(cur..).map(|(_, v)| v).chain(removed.into_values()).for_each(|ref mut state| Pin::new(state).unmount(renderer));

            if states.is_empty() {
                *states = old_states;
            } else {
                states.extend(old_states);
            }
        }

        fn keyed_elements_update_csr_state_force_reposition<I: IntoIterator<Item = Keyed<K, E>>, R: RenderHtml>(self, keyed_elements: I, renderer: &mut R, state: Pin<&mut Self::CsrState<R>>) {
            let states = &mut state.get_mut().0;

            let elements = keyed_elements.into_iter();

            let mut old_states = std::mem::replace(states, IndexMap::new());
            // TODO: optimize perf with IndexMap::with_capacity(elements.len());

            for Keyed(key, element) in elements {
                let state = old_states.remove(&key);

                let entry = states.entry(key);

                debug_assert!(matches!(entry, indexmap::map::Entry::Vacant(_)));

                let render_state = entry.or_insert(state.unwrap_or_default());

                E::unpinned_render_update_force_reposition(element, renderer, render_state);
            }

            for state in old_states.values_mut().map(std::pin::Pin::new) {
                state.unmount(renderer)
            }
        }
    }

    /// If `states` contains `key`, then warns.
    /// Else, call f.
    fn state_vacant_and_then<'a, K, S>(states: &'a mut IndexMap<K, S>, key: K, f: impl FnOnce(indexmap::map::VacantEntry<'a, K, S>))
    where
        K: std::hash::Hash + Eq,
    {
        match states.entry(key) {
            indexmap::map::Entry::Vacant(entry) => f(entry),
            indexmap::map::Entry::Occupied(_) => {
                if cfg!(all(debug_assertions, target_arch = "wasm32")) {
                    #[cfg(not_working_yet)]
                    gloo::console::warn!("the same key has been inserted so the latter element is ignored");
                }
            }
        }
    }
}

#[cfg(not_working_yet)]
pub mod linked_vec {
    use std::{hash::Hash, marker::PhantomData, pin::Pin};

    use frender_common::Keyed;

    use crate::{Element, RenderState};

    use super::ElementsAlgorithm;

    pub use LinkedVecAlgorithm as Algorithm;

    pub struct LinkedVecAlgorithm<Impl> {
        __phantom: std::marker::PhantomData<Impl>,
    }

    impl<Impl> Default for LinkedVecAlgorithm<Impl> {
        fn default() -> Self {
            Self { __phantom: std::marker::PhantomData }
        }
    }

    impl<K: Hash + Eq, E: Element, Impl: IndexMapForStates<K, E::CsrState>> ElementsAlgorithm<K, E> for LinkedVecAlgorithm<Impl>
    where
        E::CsrState: Unpin,
    {
        type CsrState = States<K, E::CsrState, Impl>;

        fn keyed_elements_into_csr_state<I: IntoIterator<Item = Keyed<K, E>>>(self, keyed_elements: I, ctx: &mut crate::CsrContext) -> Self::CsrState {
            States::from_entries(keyed_elements.into_iter().map(|Keyed(key, element)| (key, element.into_csr_state(ctx))))
        }

        fn keyed_elements_update_csr_state<I: IntoIterator<Item = Keyed<K, E>>>(self, keyed_elements: I, ctx: &mut crate::CsrContext, state: Pin<&mut Self::CsrState>) {
            self.keyed_elements_update_csr_state_maybe_reposition(keyed_elements, ctx, state, false)
        }

        fn keyed_elements_update_csr_state_force_reposition<I: IntoIterator<Item = Keyed<K, E>>>(self, keyed_elements: I, ctx: &mut crate::CsrContext, state: Pin<&mut Self::CsrState>) {
            self.keyed_elements_update_csr_state_maybe_reposition(keyed_elements, ctx, state, true)
        }

        fn keyed_elements_update_csr_state_maybe_reposition<I: IntoIterator<Item = Keyed<K, E>>>(self, keyed_elements: I, ctx: &mut crate::CsrContext, state: Pin<&mut Self::CsrState>, force_reposition: bool)
        where
            Self: Sized,
        {
            state
                .get_mut()
                .update_maybe_reposition(keyed_elements.into_iter(), E::into_csr_state, E::update_csr_state_maybe_reposition, E::CsrState::unmount, ctx, force_reposition);
        }
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

        fn as_mut_position(&mut self) -> Position<&mut Index> {
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
    }

    pub struct RealIndexMap<K, V> {
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

        const ORIGINAL: [(Key, Item); 5] = [(Cow::Borrowed("a"), 0), (Cow::Borrowed("b"), 1), (Cow::Borrowed("c"), 2), (Cow::Borrowed("d"), 3), (Cow::Borrowed("e"), 4)];

        fn get_original_state<Impl: IndexMapForStates<Key, State>>() -> States<Key, State, Impl> {
            States::<Key, State, Impl>::from_entries(ORIGINAL)
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
                fn record_while_updating_states<Impl: IndexMapForStates<Key, State>, E: Iterator<Item = Keyed<Key, Item>>>(&mut self, states: &mut States<Key, State, Impl>, entries: E, force_reposition: bool) {
                    states.update_maybe_reposition(
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
                        force_reposition,
                    )
                }
            }

            fn original<Impl: IndexMapForStates<Key, State>>() {
                let values = get_original_state::<Impl>().iter_ordered().map(|(k, &v)| (k.clone(), v)).collect::<Vec<_>>();

                assert_eq!(values, ORIGINAL);
            }

            fn update_with_unchanged<Impl: IndexMapForStates<Key, State>>() {
                let mut states = get_original_state::<Impl>();
                let mut records = Records::default();
                records.record_while_updating_states(&mut states, ORIGINAL.map(|(k, v)| Keyed(k, v)).into_iter(), false);
                assert_eq!(states.iter_ordered().map(clone_key_state).collect::<Vec<_>>(), ORIGINAL);

                assert_eq!(
                    records,
                    Records {
                        into: vec![],
                        updated: ORIGINAL
                            .map(|(_, v)| Updated {
                                old_state: v,
                                new_item: v,
                                force_reposition: false
                            })
                            .into(),
                        unmounted: vec![]
                    }
                );
            }

            pub(super) fn append<Impl: IndexMapForStates<Key, State>>() {
                let mut states = get_original_state::<Impl>();
                let mut records = Records::default();

                let entries = ORIGINAL.into_iter().chain([(ORIGINAL.len().to_string().into(), ORIGINAL.len() as Item)]).map(Keyed::from_tuple);

                records.record_while_updating_states(&mut states, entries, false);

                assert_eq!(
                    states.iter_ordered().map(clone_key_state).collect::<Vec<_>>(),
                    ORIGINAL.into_iter().chain([(ORIGINAL.len().to_string().into(), ORIGINAL.len() as Item)]).collect::<Vec<_>>()
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

            pub(super) fn prepend<Impl: IndexMapForStates<Key, State>>() {
                let mut states = get_original_state::<Impl>();
                let mut records = Records::default();

                records.record_while_updating_states(
                    &mut states,
                    [(ORIGINAL.len().to_string().into(), ORIGINAL.len() as Item)].into_iter().chain(ORIGINAL.into_iter()).map(Keyed::from_tuple),
                    false,
                );

                assert_eq!(
                    states.clone_ordered(),
                    [(ORIGINAL.len().to_string().into(), ORIGINAL.len() as Item)].into_iter().chain(ORIGINAL.into_iter()).collect::<Vec<_>>()
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

                records.record_while_updating_states(&mut states, expected.clone().into_iter().map(Keyed::from_tuple), false);

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

                records.record_while_updating_states(&mut states, swapped.clone().map(Keyed::from_tuple).into_iter(), false);

                assert_eq!(states.clone_ordered(), swapped);

                assert_eq!(
                    records,
                    Records {
                        into: vec![],
                        updated: [
                            Updated {
                                old_state: 0,
                                new_item: 0,
                                force_reposition: false
                            },
                            Updated {
                                old_state: 3,
                                new_item: 3,
                                force_reposition: true
                            },
                            Updated {
                                old_state: 2,
                                new_item: 2,
                                force_reposition: true
                            },
                            Updated {
                                old_state: 1,
                                new_item: 1,
                                force_reposition: false
                            },
                            Updated {
                                old_state: 4,
                                new_item: 4,
                                force_reposition: false
                            },
                        ]
                        .into(),
                        unmounted: vec![]
                    }
                );
            }

            pub(super) fn remove_all<Impl: IndexMapForStates<Key, State>>() {
                let mut states = get_original_state::<Impl>();
                let mut records = Records::default();

                records.record_while_updating_states(&mut states, [].into_iter(), false);

                assert_eq!(states.clone_ordered(), []);

                assert_eq!(
                    records,
                    Records {
                        into: vec![],
                        updated: vec![],
                        unmounted: ORIGINAL.into_iter().map(|(_, state)| state).collect()
                    }
                );
            }

            pub(super) fn full_replace<Impl: IndexMapForStates<Key, State>>() {
                let mut states = get_original_state::<Impl>();
                let mut records = Records::default();

                const NEW: [(Key, Item); 2] = [(Cow::Borrowed("f"), 5), (Cow::Borrowed("g"), 6)];

                records.record_while_updating_states(&mut states, NEW.into_iter().map(Keyed::from_tuple), false);

                assert_eq!(states.clone_ordered(), NEW);

                assert_eq!(
                    records,
                    Records {
                        into: vec![5, 6],
                        updated: vec![],
                        unmounted: ORIGINAL.into_iter().map(|(_, state)| state).collect()
                    }
                );
            }

            pub(super) fn pop_front<Impl: IndexMapForStates<Key, State>>() {
                let mut states = get_original_state::<Impl>();
                let mut records = Records::default();

                records.record_while_updating_states(&mut states, ORIGINAL.into_iter().skip(1).map(Keyed::from_tuple), false);

                assert_eq!(states.clone_ordered(), ORIGINAL.into_iter().skip(1).collect::<Vec<_>>());

                assert_eq!(
                    records,
                    Records {
                        into: vec![],
                        updated: ORIGINAL
                            .into_iter()
                            .skip(1)
                            .map(|(_, v)| Updated {
                                old_state: v,
                                new_item: v,
                                force_reposition: true
                            })
                            .collect(),
                        unmounted: vec![ORIGINAL[0].1]
                    }
                );
            }

            pub(super) fn empty<Impl: IndexMapForStates<Key, State>>() {
                let mut states = States::<_, _, Impl>::from_entries([]);
                assert!(states.clone_ordered().is_empty());

                let mut records = Records::default();

                records.record_while_updating_states(&mut states, ORIGINAL.into_iter().map(Keyed::from_tuple), false);

                assert_eq!(states.clone_ordered(), ORIGINAL);
            }

            pub(super) fn create<Impl: IndexMapForStates<Key, State>>() {
                let states = States::<_, _, Impl>::from_entries([]);
                assert!(states.clone_ordered().is_empty());
            }

            pub(super) fn all<Impl: IndexMapForStates<Key, State>>() {
                original::<Impl>();
                update_with_unchanged::<Impl>();
                append::<Impl>();
                prepend::<Impl>();
                prepend_and_update::<Impl>();
                swap::<Impl>();
                remove_all::<Impl>();
                full_replace::<Impl>();
                pop_front::<Impl>();
                empty::<Impl>();
                create::<Impl>();
            }
        }

        #[test]
        fn real_index_map() {
            asserts::all::<RealIndexMap<_, _>>()
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
            if let Some((_, Node { next: next_of_last, .. })) = map.last_mut() {
                *next_of_last = usize::MAX;
            }

            if let Some((_, Node { prev: prev_of_first, .. })) = map.first_mut() {
                *prev_of_first = usize::MAX;
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

        fn remove_by_index(&mut self, index: usize, first_index: &mut usize) -> Node<V, usize> {
            let (_, mut node) = self.map.swap_remove_index(index).unwrap();

            let swapped_index = self.map.len();

            debug_assert_ne!(swapped_index, usize::MAX);

            if node.prev == swapped_index {
                node.prev = index;
            }

            if node.next == swapped_index {
                node.next = index;
            }

            *if node.prev == usize::MAX {
                debug_assert_ne!(*first_index, usize::MAX);
                &mut *first_index
            } else {
                self.get_mut_next_by_index(node.prev)
            } = node.next;

            if node.next != usize::MAX {
                *self.get_mut_prev_by_index(node.next) = node.prev;
            }

            if index != swapped_index {
                let Position { value: (), prev, next } = self.get_position_by_index(index);
                let next_of_prev = if prev == usize::MAX { &mut *first_index } else { self.get_mut_next_by_index(prev) };
                *next_of_prev = index;

                if next != usize::MAX {
                    let prev_of_next = self.get_mut_prev_by_index(next);
                    *prev_of_next = index;
                }
            }

            #[cfg(debug_assertions)]
            if index != swapped_index {
                let Position { value: (), prev, next } = self.get_position_by_index(index);
                assert_eq!(*if prev == usize::MAX { first_index } else { self.get_mut_next_by_index(prev) }, index);
                if next != usize::MAX {
                    assert_eq!(*self.get_mut_prev_by_index(next), index);
                }
            }

            node
        }
    }

    pub trait GetMutPositionByIndex {
        fn get_mut_position_by_index(&mut self, index: usize) -> Node<(), &mut usize>;
        fn get_position_by_index(&self, index: usize) -> Position<usize>;
        fn get_mut_next_by_index(&mut self, index: usize) -> &mut usize;
        fn get_mut_prev_by_index(&mut self, index: usize) -> &mut usize;
    }

    pub trait IndexMapForStates<K, V>: GetMutPositionByIndex {
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

        /// `index` should be valid. If not, implementation should panic.
        ///
        /// Implementation should make sure `prev` and `next` are correctly updated.
        fn remove_by_index(&mut self, index: usize, first_index: &mut usize) -> Node<V, usize>;
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

    pub struct States<K, V, IndexMapImpl: IndexMapForStates<K, V>> {
        map: IndexMapImpl,
        first_index: usize,
        _phantom: PhantomData<(K, V)>,
    }

    impl<K: Hash + Eq, V: Unpin, IndexMapImpl: IndexMapForStates<K, V>> States<K, V, IndexMapImpl> {
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

            let first_index = if map.is_empty() { usize::MAX } else { 0 };

            Self { map, first_index, _phantom: PhantomData }
        }

        pub(super) fn update_maybe_reposition<T, Ctx, E: Iterator<Item = Keyed<K, T>>>(
            &mut self,
            entries: E,
            mut item_into_value: impl FnMut(T, &mut Ctx) -> V,
            mut item_update_value: impl FnMut(T, &mut Ctx, Pin<&mut V>, bool),
            mut value_unmount: impl FnMut(Pin<&mut V>),
            ctx: &mut Ctx,
            force_reposition: bool,
        ) {
            if self.map.is_empty() {
                self.map = IndexMapImpl::from_entries(entries.map(|Keyed(key, item)| (key, item_into_value(item, ctx))));
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

            let mut prev = usize::MAX;
            let mut next_of_prev = self.first_index;

            let mut count = 0;

            for Keyed(key, item) in entries {
                let index;

                if let Some(i) = self.map.get_index_by_key_with_index_hint(&key, next_of_prev) {
                    index = i;

                    if index == next_of_prev {
                        let node = self.map.get_mut_node_by_index(index);
                        item_update_value(item, ctx, Pin::new(node.value), force_reposition);

                        next_of_prev = *node.next;

                        #[cfg(debug_assertions)]
                        if prev == usize::MAX {
                            debug_assert_eq!(self.first_index, index);
                        } else {
                            debug_assert_eq!(self.map.get_position_by_index(prev).next, index);
                        }
                    } else {
                        let node = self.map.get_mut_node_by_index(index);
                        item_update_value(item, ctx, Pin::new(node.value), true);

                        let prev_of_node = *node.prev;
                        let next_of_node = *node.next;
                        *node.prev = prev;
                        *node.next = next_of_prev;

                        if prev == usize::MAX {
                            self.first_index = index;
                        } else {
                            *self.map.get_mut_next_by_index(prev) = index;
                        }

                        if next_of_prev != usize::MAX {
                            *self.map.get_mut_prev_by_index(next_of_prev) = index;
                        }

                        {
                            let next = if prev_of_node == usize::MAX { &mut self.first_index } else { self.map.get_mut_next_by_index(prev_of_node) };
                            debug_assert_eq!(*next, index);
                            *next = next_of_node;
                        }

                        if next_of_node != usize::MAX {
                            let prev = self.map.get_mut_prev_by_index(next_of_node);
                            debug_assert_eq!(*prev, index);
                            *prev = prev_of_node;
                        }
                    }

                    prev = index;
                } else {
                    index = self.map.add_with_new_key(
                        key,
                        Node {
                            value: item_into_value(item, ctx),
                            prev,
                            next: next_of_prev,
                        },
                    );

                    {
                        let nop = if prev == usize::MAX { &mut self.first_index } else { self.map.get_mut_next_by_index(prev) };

                        debug_assert_eq!(*nop, next_of_prev);

                        *nop = index;
                    }

                    if next_of_prev != usize::MAX {
                        let pon = self.map.get_mut_prev_by_index(next_of_prev);
                        debug_assert_eq!(*pon, prev);
                        *pon = index;
                    }

                    prev = index;
                };

                #[cfg(debug_assertions)]
                if count == 0 {
                    assert_eq!(index, self.first_index);
                }

                count += 1;
            }

            if count == 0 {
                self.for_each_value_pin_mut(value_unmount);
                self.clear();
                return;
            }

            while next_of_prev != usize::MAX {
                let mut removed = self.map.remove_by_index(next_of_prev, &mut self.first_index);

                value_unmount(Pin::new(&mut removed.value));

                next_of_prev = removed.next;
            }
        }

        #[cfg(test)]
        fn iter_ordered(&self) -> IterOrdered<'_, K, V, IndexMapImpl> {
            IterOrdered { this: self, cursor: self.first_index }
        }

        #[cfg(test)]
        fn clone_ordered(&self) -> Vec<(K, V)>
        where
            K: Clone,
            V: Clone,
        {
            self.iter_ordered().map(|(key, state)| (key.clone(), state.clone())).collect::<Vec<_>>()
        }
    }

    impl<K: Hash + Eq, S: Unpin, Impl: IndexMapForStates<K, S>> Unpin for States<K, S, Impl> {}

    impl<K: Hash + Eq, S: RenderState + Unpin, Impl: IndexMapForStates<K, S>> RenderState for States<K, S, Impl> {
        fn unmount(self: std::pin::Pin<&mut Self>) {
            self.get_mut().for_each_value_pin_mut(S::unmount)
        }

        fn state_unmount(self: std::pin::Pin<&mut Self>) {
            self.get_mut().for_each_value_pin_mut(S::state_unmount)
        }

        fn poll_csr(self: std::pin::Pin<&mut Self>, ctx: &mut crate::CsrContext, cx: &mut std::task::Context<'_>) -> std::task::Poll<()> {
            let mut res = std::task::Poll::Ready(());

            self.get_mut().for_each_value_pin_mut(|state| match S::poll_csr(state, ctx, cx) {
                std::task::Poll::Ready(()) => {}
                std::task::Poll::Pending => {
                    res = std::task::Poll::Pending;
                }
            });

            res
        }
    }

    pub(super) struct IterOrdered<'a, K, V, IndexMapImpl: IndexMapForStates<K, V>> {
        this: &'a States<K, V, IndexMapImpl>,
        cursor: usize,
    }

    impl<'a, K, V, IndexMapImpl: IndexMapForStates<K, V>> Iterator for IterOrdered<'a, K, V, IndexMapImpl> {
        type Item = (&'a K, &'a V);

        fn next(&mut self) -> Option<Self::Item> {
            if self.cursor == usize::MAX {
                return None;
            }
            let (key, Node { value, prev: &prev, next: &next }) = self.this.map.get_by_index(self.cursor);

            assert_eq!(if prev == usize::MAX { self.this.first_index } else { self.this.map.get_position_by_index(prev).next }, self.cursor);

            if next != usize::MAX {
                assert_eq!(self.this.map.get_position_by_index(next).prev, self.cursor)
            }

            self.cursor = next;
            Some((key, value))
        }
    }
}

impl<K, E> Element for Vec<Keyed<K, E>>
where
    K: std::hash::Hash + Eq,
    E: Element,
{
    type RenderState<R: RenderHtml> = default::States<K, E::UnpinnedRenderState<R>>;

    fn render_update_maybe_reposition<Renderer: RenderHtml>(self, renderer: &mut Renderer, render_state: Pin<&mut Self::RenderState<Renderer>>, force_reposition: bool) {
        Elements(self).render_update_maybe_reposition(renderer, render_state, force_reposition)
    }

    fn render_update<Renderer: RenderHtml>(self, renderer: &mut Renderer, render_state: Pin<&mut Self::RenderState<Renderer>>) {
        Elements(self).render_update(renderer, render_state)
    }

    fn render_update_force_reposition<Renderer: RenderHtml>(self, renderer: &mut Renderer, render_state: Pin<&mut Self::RenderState<Renderer>>) {
        Elements(self).render_update_force_reposition(renderer, render_state)
    }

    crate::impl_unpinned_render_for_unpin! {}
}

impl<I, A, K, E> Element for Elements<I, A>
where
    I: IntoIterator<Item = Keyed<K, E>>,
    K: std::hash::Hash + Eq,
    E: Element,
    A: ElementsAlgorithm<K, E>,
{
    type RenderState<R: RenderHtml> = A::CsrState<R>;

    fn render_update<Renderer: RenderHtml>(self, renderer: &mut Renderer, render_state: Pin<&mut Self::RenderState<Renderer>>) {
        A::keyed_elements_update_csr_state(self.algorithm, self.iter, renderer, render_state)
    }

    fn render_update_force_reposition<Renderer: RenderHtml>(self, renderer: &mut Renderer, render_state: Pin<&mut Self::RenderState<Renderer>>) {
        A::keyed_elements_update_csr_state_force_reposition(self.algorithm, self.iter, renderer, render_state)
    }

    fn render_update_maybe_reposition<Renderer: RenderHtml>(self, renderer: &mut Renderer, render_state: Pin<&mut Self::RenderState<Renderer>>, force_reposition: bool) {
        A::keyed_elements_update_csr_state_maybe_reposition(self.algorithm, self.iter, renderer, render_state, force_reposition)
    }

    crate::impl_unpinned_render_for_unpin! {}
}

#[cfg(not_working_yet)]
pub type ElementsLinkedVec<I> = Elements<I, linked_vec::Algorithm<linked_vec::RealIndexMap<<<I as IntoIterator>::Item as IsKeyed>::Key, <<<I as IntoIterator>::Item as IsKeyed>::Element as Element>::CsrState>>>;

#[cfg(not_working_yet)]
#[allow(non_snake_case)]
pub fn ElementsLinkedVec<K, E: Element, I: IntoIterator<Item = Keyed<K, E>>>(iter: I) -> Elements<I, linked_vec::Algorithm<linked_vec::RealIndexMap<K, E::UnpinnedRenderState<R>>>> {
    Elements { iter, algorithm: Default::default() }
}
