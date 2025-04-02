use chtml::encode::attribute_value::AttributeValueForRendering;

use crate::{
    values::r#const::value::{ConstAttrValueValue, ConstAttrValueValueOfKind},
    AttrKindOfStr,
};

impl<'a, const CAP: usize> ConstAttrValueValue for AttributeValueForRendering<'a, CAP> {}
impl<'a, const CAP: usize> ConstAttrValueValueOfKind<AttrKindOfStr>
    for AttributeValueForRendering<'a, CAP>
{
}

#[cfg(feature = "html")]
impl<'a, const CAP: usize> ConstAttrValueValueOfKind<crate::html::AttrKindOfContentEditable>
    for AttributeValueForRendering<'a, CAP>
{
}

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;
