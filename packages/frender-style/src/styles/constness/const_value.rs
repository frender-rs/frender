use frender_common::define_phantom_wrapper;

mod reactive_value;

define_phantom_wrapper!(
    #[always_derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ConstValue<T: ?Sized + HasConstValue>;
);

pub trait HasConstValue {
    type Value;
    const VALUE: Self::Value;
}

pub trait ConstBorrowStr<'a>: std::borrow::Borrow<str> {
    type HasConstValueStr<T: ?Sized + HasConstValue<Value = Self>>: ?Sized
        + HasConstValue<Value = &'a str>;
}

impl<'a> ConstBorrowStr<'a> for &'a str {
    type HasConstValueStr<T: ?Sized + HasConstValue<Value = Self>> = T;
}
