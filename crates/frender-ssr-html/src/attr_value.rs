use frender_common::AsyncStrIterator;

use crate::{encode::Encode, escape_safe::DoubleQuotedAttribute};

frender_common::Strings!(
    enum AttrEqValueState {}
    pub struct AttrEqValue<V: AsyncStrIterator>(
        eq_double_quote!("=\""),
        value!(Encode<DoubleQuotedAttribute, V>),
        double_quote!("\""),
    );
);

impl<V: AsyncStrIterator> AttrEqValue<V> {
    pub fn new(value: V) -> Self {
        Self {
            _state: AttrEqValueState(),
            eq_double_quote: (),
            value: Encode::new(DoubleQuotedAttribute, value),
            double_quote: (),
        }
    }
}

#[allow(non_snake_case)]
pub fn AttrEqValue<V: AsyncStrIterator>(value: V) -> AttrEqValue<V> {
    AttrEqValue::new(value)
}
