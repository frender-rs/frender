use std::collections::HashMap;

use crate::{RenderState, UpdateRenderState};

/// TODO: `Keyed(element, key)`
///     - improve performance.
///       Currently, all elements are unmounted and then new elements update the states.
///     - impl UpdateRenderState<Ctx> for T where T: IntoIterator<Keyed<E, K>>
pub struct Keyed<K, E>(pub K, pub E);

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

    fn poll_reactive(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        let mut res = std::task::Poll::Ready(());

        let values = self.get_mut().states.values_mut();
        for state in values {
            match S::poll_reactive(std::pin::Pin::new(state), cx) {
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

impl<Ctx, K, E> UpdateRenderState<Ctx> for Vec<Keyed<K, E>>
where
    K: std::hash::Hash + Eq,
    E: UpdateRenderState<Ctx>,
    E::State: Unpin,
{
    type State = KeyedElementsState<K, E::State>;

    fn initialize_render_state(self, ctx: &mut Ctx) -> Self::State {
        KeyedElementsState {
            states: self
                .into_iter()
                .map(|Keyed(k, v)| (k, v.initialize_render_state(ctx)))
                .collect(),
        }
    }

    fn update_render_state(self, ctx: &mut Ctx, mut state: std::pin::Pin<&mut Self::State>) {
        state.as_mut().unmount();
        let states = &mut state.get_mut().states;
        let mut old_states = std::mem::replace(states, HashMap::with_capacity(self.len()));

        for Keyed(key, element) in self {
            if let Some(state) = old_states.remove(&key) {
                // TODO: assert entry is vacant
                let state = states.entry(key).or_insert(state);
                E::update_render_state(element, ctx, std::pin::Pin::new(state));
            } else {
                states.insert(key, element.initialize_render_state(ctx)); // TODO: assert returned is None
            }
        }
    }
}
