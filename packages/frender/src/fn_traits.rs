pub trait FnOnce1<Arg>: FnOnce(Arg) -> Self::Output_ {
    type Output_;
}

impl<F: ?Sized + FnOnce(Arg) -> Out, Arg, Out> FnOnce1<Arg> for F {
    type Output_ = Out;
}

pub trait FnMut1<Arg>: FnOnce1<Arg> + FnMut(Arg) -> Self::Output_ {}
impl<F: ?Sized + FnMut(Arg) -> Out, Arg, Out> FnMut1<Arg> for F {}

pub trait Fn1<Arg>: FnMut1<Arg> + Fn(Arg) -> Self::Output_ {}
impl<F: ?Sized + Fn(Arg) -> Out, Arg, Out> Fn1<Arg> for F {}

pub trait FnOnce2<Arg0, Arg1>: FnOnce(Arg0, Arg1) -> Self::Output_ {
    type Output_;
}

impl<F: ?Sized + FnOnce(Arg0, Arg1) -> Out, Arg0, Arg1, Out> FnOnce2<Arg0, Arg1> for F {
    type Output_ = Out;
}

pub trait FnMut2<Arg0, Arg1>: FnOnce2<Arg0, Arg1> + FnMut(Arg0, Arg1) -> Self::Output_ {}
impl<F: ?Sized + FnMut(Arg0, Arg1) -> Out, Arg0, Arg1, Out> FnMut2<Arg0, Arg1> for F {}

#[allow(dead_code)]
pub trait Fn2<Arg0, Arg1>: FnMut2<Arg0, Arg1> + Fn(Arg0, Arg1) -> Self::Output_ {}
impl<F: ?Sized + Fn(Arg0, Arg1) -> Out, Arg0, Arg1, Out> Fn2<Arg0, Arg1> for F {}
