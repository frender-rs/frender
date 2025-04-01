use std::marker::PhantomData;

use const_core::convert::MarkerOfConstValue;
use frender_ssr_html::assert::HtmlAttributeEqValueOrEmpty;

use crate::{ssr::SsrAttrValue, AttrValueKind};

use super::{value::ConstAttrValueValueOfKind, ConstAttrValue, HasConstAttrValue};

pub trait ConstAttrValueSsrValue<AK: AttrValueKind> {
    type HtmlAttributeValue: HtmlAttributeEqValueOrEmpty;
    type MaybeIntoHtmlAttributeValue<This: ?Sized + HasConstAttrValue<AttrValue = Self>>: ?Sized + MarkerOfConstValue<Option<Self::HtmlAttributeValue>>;
}

impl<
        M: ?Sized + HasConstAttrValue<AttrValue: ConstAttrValueValueOfKind<AK>>,
        AK: AttrValueKind,
    > SsrAttrValue<AK> for ConstAttrValue<M>
{
    type HtmlAttributeValue = <M::AttrValue as ConstAttrValueSsrValue<AK>>::HtmlAttributeValue;

    fn maybe_into_html_attribute_value(_: Self) -> Option<Self::HtmlAttributeValue> {
        <M::AttrValue as ConstAttrValueSsrValue<AK>>::MaybeIntoHtmlAttributeValue::<M>::VALUE
    }
}

enum Never {}
pub struct MarkerOfSome<M: ?Sized>(Never, PhantomData<M>);

impl<M: ?Sized + MarkerOfConstValue<T>, T> MarkerOfConstValue<Option<T>> for MarkerOfSome<M> {
    const VALUE: Option<T> = Some(M::VALUE);
}
