use std::marker::PhantomData;

use chtml::encode::attribute_value::{ssr::EqValueStrIntoIter, AttributeValueForRendering};
use const_core::convert::MarkerOfConstValue;

use crate::{
    html::AttrKindOfContentEditable,
    values::r#const::{
        ssr::{ConstAttrValueSsrValue, MarkerOfSome},
        HasConstAttrValue,
    },
    AttrKindOfStr,
};

impl<'a, const CAP: usize> ConstAttrValueSsrValue<AttrKindOfStr>
    for AttributeValueForRendering<'a, CAP>
{
    type HtmlAttributeValue = EqValueStrIntoIter<'a>;
    type MaybeIntoHtmlAttributeValue<This: ?Sized + HasConstAttrValue<AttrValue = Self>> =
        MarkerOfSome<ToEqValueStrIntoIter<This>>;
}
impl<'a, const CAP: usize> ConstAttrValueSsrValue<AttrKindOfContentEditable>
    for AttributeValueForRendering<'a, CAP>
{
    type HtmlAttributeValue = EqValueStrIntoIter<'a>;
    type MaybeIntoHtmlAttributeValue<This: ?Sized + HasConstAttrValue<AttrValue = Self>> =
        MarkerOfSome<ToEqValueStrIntoIter<This>>;
}

enum Never {}
pub struct ToEqValueStrIntoIter<M: ?Sized + HasConstAttrValue>(Never, PhantomData<M>);

impl<
        'a,
        M: ?Sized + HasConstAttrValue<AttrValue = AttributeValueForRendering<'a, CAP>>,
        const CAP: usize,
    > MarkerOfConstValue<EqValueStrIntoIter<'a>> for ToEqValueStrIntoIter<M>
{
    const VALUE: EqValueStrIntoIter<'a> = EqValueStrIntoIter::new(M::ATTR_VALUE.as_eq_value_str());
}
