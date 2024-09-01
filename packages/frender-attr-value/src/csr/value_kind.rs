pub trait ValueKind: 'static {
    type Value<'a>;
}

impl ValueKind for str {
    type Value<'a> = &'a str;
}

impl ValueKind for bool {
    type Value<'a> = bool;
}

frender_common::impl_many!(
    impl<__> ValueKind
        for each_of![i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64]
    {
        type Value<'a> = Self;
    }
);

/*
impl ValueKind for crate::content_editable::ContentEditable<'static> {
    type Value<'a> = crate::content_editable::ContentEditable<'a>;
}

impl ValueKind for crate::Spellcheck {
    type Value<'a> = crate::Spellcheck;
}
*/
