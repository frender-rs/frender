use std::collections::HashMap;

use frender_common::Keyed;

use crate::{Element, RenderState};

pub struct KeyedElementsState<K, S> {
    states: HashMap<K, S>,
}

impl<K, S> Unpin for KeyedElementsState<K, S> {}

impl<K, S: RenderState + Unpin> RenderState for KeyedElementsState<K, S> {
    fn unmount(self: std::pin::Pin<&mut Self>) {
        for state in self.get_mut().states.values_mut().map(std::pin::Pin::new) {
            S::unmount(state)
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
    K: std::hash::Hash + Eq,
    E: Element,
    E::CsrState: Unpin,
{
    type CsrState = KeyedElementsState<K, E::CsrState>;

    fn into_csr_state(self, ctx: &mut crate::CsrContext) -> Self::CsrState {
        KeyedElementsState {
            states: self
                .into_iter()
                .map(|Keyed(k, v)| (k, v.into_csr_state(ctx)))
                .collect(),
        }
    }

    fn update_csr_state(
        self,
        ctx: &mut crate::CsrContext,
        mut state: std::pin::Pin<&mut Self::CsrState>,
    ) {
        state.as_mut().unmount();
        let states = &mut state.get_mut().states;
        let mut old_states = std::mem::replace(states, HashMap::with_capacity(self.len()));

        for Keyed(key, element) in self {
            if let Some(state) = old_states.remove(&key) {
                // TODO: assert entry is vacant
                let state = states.entry(key).or_insert(state);
                E::update_csr_state(element, ctx, std::pin::Pin::new(state));
            } else {
                states.insert(key, element.into_csr_state(ctx)); // TODO: assert returned is None
            }
        }
    }
}
