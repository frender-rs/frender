pub trait TryBehavior {
    fn unwrap_result<T, E>(&mut self, result: Result<T, E>) -> T
    where
        E: std::fmt::Debug;
    fn unwrap_option<T>(&mut self, option: Option<T>) -> T;
}

impl<TB: ?Sized + TryBehavior> TryBehavior for &mut TB {
    fn unwrap_result<T, E>(&mut self, result: Result<T, E>) -> T
    where
        E: std::fmt::Debug,
    {
        TB::unwrap_result(self, result)
    }

    fn unwrap_option<T>(&mut self, option: Option<T>) -> T {
        TB::unwrap_option(self, option)
    }
}

pub struct Unwrap;

impl TryBehavior for Unwrap {
    fn unwrap_result<T, E>(&mut self, result: Result<T, E>) -> T
    where
        E: std::fmt::Debug,
    {
        result.unwrap()
    }

    fn unwrap_option<T>(&mut self, option: Option<T>) -> T {
        option.unwrap()
    }
}

pub trait TryWithTryBehavior<TB: TryBehavior> {
    type TryOutput;
    fn unwrap_with_behavior(self, tb: &mut TB) -> Self::TryOutput;
}

impl<T, E: std::fmt::Debug, TB: TryBehavior> TryWithTryBehavior<TB> for Result<T, E> {
    type TryOutput = T;

    fn unwrap_with_behavior(self, tb: &mut TB) -> Self::TryOutput {
        tb.unwrap_result(self)
    }
}

impl<T, TB: TryBehavior> TryWithTryBehavior<TB> for Option<T> {
    type TryOutput = T;

    fn unwrap_with_behavior(self, tb: &mut TB) -> Self::TryOutput {
        tb.unwrap_option(self)
    }
}
