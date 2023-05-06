use super::{Callable, CallableWithFixedArguments, Callback, IsCallable};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Chain<F1, F2>(pub F1, pub F2);

impl<F1, F2> IsCallable for Chain<F1, F2> {}
impl<
        F1: CallableWithFixedArguments,
        F2: for<'a> Callable<(
            <F1 as Callable<super::argument::ArgumentsOfTypes<'a, F1::FixedArgumentTypes>>>::Output,
        )>,
    > CallableWithFixedArguments for Chain<F1, F2>
{
    type FixedArgumentTypes = F1::FixedArgumentTypes;
}

impl<Args: super::sealed::Tuple, F1, F2> Callable<Args> for Chain<F1, F2>
where
    F1: Callable<Args>,
    F2: Callable<(F1::Output,)>,
{
    type Output = F2::Output;

    fn call_fn(&self, args: Args) -> Self::Output {
        self.1.emit(self.0.call_fn(args))
    }
}
