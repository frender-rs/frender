use super::{Callback, IsCallback};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WithState<F: Clone + PartialEq, State: Clone + PartialEq> {
    pub(super) f: F,
    pub(super) state: State,
}

impl<State: Clone + PartialEq, IN, Out> Callback<IN> for WithState<fn(IN, &State) -> Out, State> {
    type Output = Out;

    fn emit(&self, input: IN) -> Self::Output {
        (self.f)(input, &self.state)
    }
}

impl<F: Clone + PartialEq, State: Clone + PartialEq> IsCallback for WithState<F, State> {}

impl<'i, State: Clone + PartialEq, IN, Out> Callback<&'i IN>
    for WithState<super::accept_ref::AcceptRef<fn(&IN, &State) -> Out>, State>
{
    type Output = Out;

    fn emit(&self, input: &'i IN) -> Self::Output {
        (self.f.0)(input, &self.state)
    }
}

impl<'i, State: Clone + PartialEq, IN, Out> Callback<IN>
    for WithState<super::accept_ref::AcceptRef2<fn(IN, &State) -> Out>, State>
{
    type Output = Out;

    fn emit(&self, input: IN) -> Self::Output {
        (self.f.0)(input, &self.state)
    }
}

pub use super::accept_ref_with_state as accept_ref;
