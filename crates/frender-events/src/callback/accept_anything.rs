use super::IsCallable;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AcceptAnything<Cbk: super::Callable<()>>(pub(crate) Cbk);

impl<Cbk: super::Callable<()>> IsCallable for AcceptAnything<Cbk> {}

impl<Cbk: super::Callable<()>, IN> super::Callable<(IN,)> for AcceptAnything<Cbk> {
    type Output = Cbk::Output;

    fn call_fn(&self, _: (IN,)) -> Self::Output {
        self.0.call_fn(())
    }
}
