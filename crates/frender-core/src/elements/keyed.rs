use std::collections::HashMap;

use crate::{RenderState, UpdateRenderState};

pub struct Keyed<E, K>(pub E, pub K);

pub struct KeyedElementsState<K, S> {
    states: HashMap<K, S>,
}

impl<K, S> Unpin for KeyedElementsState<K, S> {}

impl<K, S: RenderState + Unpin> RenderState for KeyedElementsState<K, S> {
    fn new_uninitialized() -> Self
    where
        Self: Sized,
    {
        Self {
            states: Default::default(),
        }
    }

    fn unmount(self: std::pin::Pin<&mut Self>) {
        for state in self.get_mut().states.values_mut().map(std::pin::Pin::new) {
            S::unmount(state)
        }
    }

    fn poll_reactive(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<bool> {
        let mut res = std::task::Poll::Ready(false);

        let values = self.get_mut().states.values_mut();
        for state in values {
            match S::poll_reactive(std::pin::Pin::new(state), cx) {
                std::task::Poll::Ready(false) => {}
                v @ std::task::Poll::Ready(true) => res = v,
                v @ std::task::Poll::Pending => {
                    if let std::task::Poll::Ready(false) = res {
                        res = v;
                    }
                }
            }
        }

        res
    }
}

impl<Ctx, E, K> UpdateRenderState<Ctx> for Vec<Keyed<E, K>>
where
    E: UpdateRenderState<Ctx>,
    E::State: Unpin,
    K: std::hash::Hash + Eq,
{
    type State = KeyedElementsState<K, E::State>;

    fn update_render_state(self, ctx: &mut Ctx, mut state: std::pin::Pin<&mut Self::State>) {
        state.as_mut().unmount();
        let states = &mut state.get_mut().states;
        let mut old_states = std::mem::replace(states, HashMap::with_capacity(self.len()));

        for Keyed(element, key) in self {
            let state = old_states
                .remove(&key)
                .unwrap_or_else(E::State::new_uninitialized);

            let state = states.entry(key).or_insert(state);

            E::update_render_state(element, ctx, std::pin::Pin::new(state));
        }
    }
}
