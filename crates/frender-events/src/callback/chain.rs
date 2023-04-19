use super::{Callback, IsCallback};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Chain<F1, F2>(pub F1, pub F2);

impl<F1: Clone + PartialEq, F2: Clone + PartialEq> IsCallback for Chain<F1, F2> {}

impl<IN, F1, F2> Callback<IN> for Chain<F1, F2>
where
    F1: Callback<IN>,
    F2: Callback<F1::Output>,
{
    type Output = F2::Output;

    fn emit(&self, input: IN) -> Self::Output {
        self.1.emit(self.0.emit(input))
    }
}
