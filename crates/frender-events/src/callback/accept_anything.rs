use super::{Callback, IsCallback};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AcceptAnything<Cbk: Callback<()>>(pub(crate) Cbk);

impl<Cbk: Callback<()>> IsCallback for AcceptAnything<Cbk> {}

impl<Cbk: Callback<()>, IN> Callback<IN> for AcceptAnything<Cbk> {
    type Output = Cbk::Output;

    fn emit(&self, _: IN) -> Self::Output {
        self.0.emit(())
    }
}
