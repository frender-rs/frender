use super::{Callback, IsCallback};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WithInputRef<IN: Clone + PartialEq, Cbk: for<'input> Callback<&'input IN>> {
    pub(super) callback: Cbk,
    pub(super) input: IN,
}

impl<IN: Clone + PartialEq, Cbk: for<'input> Callback<&'input IN>> IsCallback
    for WithInputRef<IN, Cbk>
{
}

impl<IN: Clone + PartialEq, Out, Cbk: for<'input> Callback<&'input IN, Output = Out>> Callback<()>
    for WithInputRef<IN, Cbk>
{
    type Output = Out;

    fn emit(&self, _: ()) -> Self::Output {
        self.callback.emit(&self.input)
    }
}
