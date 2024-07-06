use std::borrow::Borrow;

/// The non generic version of [`Borrow`].
pub trait PrimarilyBorrow: Borrow<Self::Borrowed> {
    type Borrowed: ?Sized;
}

crate::impl_many!(
    impl<__> PrimarilyBorrow
        for each_of![
            i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, //
            f32, f64,  //
            bool, //
            char,
        ]
    {
        type Borrowed = Self;
    }
);

crate::impl_many!(
    impl<__> PrimarilyBorrow
        for each_of![
            str,
            String,
            &str,
            //
            std::rc::Rc<str>,
        ]
    {
        type Borrowed = str;
    }
);

impl<T: ?Sized + ToOwned> PrimarilyBorrow for std::borrow::Cow<'_, T> {
    type Borrowed = T;
}
