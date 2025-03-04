use frender_macro_rules::impl_many;
use frender_reactive_value::{non_reactive::Uncached, temp_into_static::TempIntoStatic};

use super::ToElement;

// scalar
impl_many!(
    impl<__> ToElement
        for each_of![
            i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64, //
            char,
        ]
    {
        type ToElement<'a>
            = Self
        where
            Self: 'a;
        fn to_element(&self) -> Self {
            *self
        }
    }
);

// acts like `Uncached<TempIntoStatic<&'a str>>`
impl_many!(
    impl<__> ToElement
        for each_of![
            //
            str,
            String,
            std::borrow::Cow<'_, str>,
        ]
    {
        type ToElement<'a>
            = Uncached<TempIntoStatic<&'a str>>
        where
            Self: 'a;
        fn to_element(&self) -> Self::ToElement<'_> {
            Uncached(TempIntoStatic(self))
        }
    }
);

// CheapClone
impl_many!(
    impl<__> ToElement
        for each_of![
            std::rc::Rc<str>, //
            std::sync::Arc<str>,
        ]
    {
        type ToElement<'a>
            = Uncached<TempIntoStatic<&'a Self>>
        where
            Self: 'a;
        fn to_element(&self) -> Self::ToElement<'_> {
            Uncached(TempIntoStatic(self))
        }
    }
);
