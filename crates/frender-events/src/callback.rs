mod traits;
pub use traits::*;

pub mod accept_anything;
pub mod accept_ref;
pub mod chain;
pub mod output;
pub mod output_cloned;
pub mod with_input_ref;
pub mod with_state;

pub fn with_state<IN, Out, State: Clone + PartialEq>(
    f: fn(IN, &State) -> Out,
    state: State,
) -> with_state::WithState<accept_ref::AcceptRef2<fn(IN, &State) -> Out>, State> {
    with_state::WithState {
        f: accept_ref::AcceptRef2(f),
        state,
    }
}

///
/// ```
/// # use frender_events::*;
/// let callback = callback::accept_ref(u8::clone).with_input_ref(8);
/// assert_eq!(callback.emit(()), 8u8);
/// ```
pub fn accept_ref<IN: ?Sized, Out>(f: fn(&IN) -> Out) -> accept_ref::AcceptRef<fn(&IN) -> Out> {
    accept_ref::AcceptRef(f)
}

/// Also accessible at <code>[accept_ref]::[with_state()](accept_ref::with_state)</code>
/// and <code>[with_state]::[accept_ref()](with_state::accept_ref)</code>.
pub fn accept_ref_with_state<IN, Out, State: Clone + PartialEq>(
    f: fn(&IN, &State) -> Out,
    state: State,
) -> with_state::WithState<accept_ref::AcceptRef<fn(&IN, &State) -> Out>, State> {
    with_state::WithState {
        f: accept_ref::AcceptRef(f),
        state,
    }
}

#[cfg(test)]
mod tests {
    use super::{Callback, CallbackExt};

    #[test]
    fn test_callback_ref() {
        fn asserts<T>(t: T) -> T
        where
            T: for<'i> Callback<&'i usize, Output = usize>,
        {
            t
        }

        let cbk = asserts(super::accept_ref(Clone::clone));
        assert_eq!(cbk.emit(&0), 0);
        assert_eq!(cbk.with_input_ref(8).emit(()), 8);
    }
}
