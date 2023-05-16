use std::{collections::HashMap, fmt::Debug, pin::Pin};

use indexmap::{IndexMap, IndexSet};

use frender_common::{Elements, Keyed};

use crate::{Element, RenderState};

pub struct KeyedElementsState<K, S> {
    // key -> state
    states: IndexMap<K, S>,
}

impl<K, S: RenderState + Unpin> KeyedElementsState<K, S> {
    fn unmount(&mut self) {
        for state in self.states.values_mut().map(std::pin::Pin::new) {
            S::unmount(state)
        }
    }
    fn clear(&mut self) {
        self.unmount();
        self.states = Default::default();
    }
}

impl<K, S> Unpin for KeyedElementsState<K, S> {}

impl<K, S: RenderState + Unpin> RenderState for KeyedElementsState<K, S> {
    fn unmount(self: std::pin::Pin<&mut Self>) {
        self.get_mut().unmount()
    }

    fn state_unmount(self: std::pin::Pin<&mut Self>) {
        for state in self.get_mut().states.values_mut().map(std::pin::Pin::new) {
            S::state_unmount(state)
        }
    }

    fn poll_csr(
        self: std::pin::Pin<&mut Self>,
        ctx: &mut crate::CsrContext,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        let mut res = std::task::Poll::Ready(());

        let values = self.get_mut().states.values_mut();
        for state in values {
            match S::poll_csr(std::pin::Pin::new(state), ctx, cx) {
                std::task::Poll::Ready(()) => {}
                v @ std::task::Poll::Pending => {
                    if let std::task::Poll::Ready(()) = res {
                        res = v;
                    }
                }
            }
        }

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
        KeyedElementsState {
            states: self
                .0
                .into_iter()
                .map(|Keyed(k, v)| (k, v.into_csr_state(ctx)))
                .collect(),
        }
    }

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

    fn update_csr_state_force_reposition(
        self,
        ctx: &mut crate::CsrContext,
        state: std::pin::Pin<&mut Self::CsrState>,
    ) {
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
            self.update_csr_state(ctx, state)
        } else {
            self.update_csr_state_force_reposition(ctx, state)
        }
    }
}

/// If `states` contains `key`, then warns.
/// Else, call f.
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

            #[cfg(debug_assertions)]
            gloo::console::warn!(format!(
                "key {:?} has been inserted so the latter element is ignored",
                entry.key()
            ));
        }
    }
}
