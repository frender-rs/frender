use std::borrow::Borrow;

use crate::{
    static_or_temp_ref::StaticOrTempRef,
    temp_into_static::{IntoStatic, TempIntoStatic},
    temp_ref::TempRef,
};

pub trait IntoBorrowStr {
    type IntoBorrowStr: Borrow<str>;
    fn into_borrow_str(self) -> Self::IntoBorrowStr;
}

impl<T: Borrow<str>> IntoBorrowStr for T {
    type IntoBorrowStr = T;

    fn into_borrow_str(self) -> Self::IntoBorrowStr {
        self
    }
}

impl<'a> IntoBorrowStr for TempRef<'a, str> {
    type IntoBorrowStr = &'a str;

    fn into_borrow_str(self) -> Self::IntoBorrowStr {
        self.0
    }
}

impl<'a> IntoBorrowStr for StaticOrTempRef<'a, str> {
    type IntoBorrowStr = &'a str;

    fn into_borrow_str(self) -> Self::IntoBorrowStr {
        self.to_ref()
    }
}

impl<T: IntoStatic<str>> IntoBorrowStr for TempIntoStatic<T> {
    type IntoBorrowStr = T::IntoStatic;

    fn into_borrow_str(self) -> Self::IntoBorrowStr {
        self.0.into_static()
    }
}
