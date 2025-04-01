use chtml::encode::attribute_value::AttributeValueForRendering;

use crate::{
    html::AttrKindOfContentEditable,
    values::r#const::value::{ConstAttrValueValue, ConstAttrValueValueOfKind},
    AttrKindOfStr,
};

impl<'a, const CAP: usize> ConstAttrValueValue for AttributeValueForRendering<'a, CAP> {}
impl<'a, const CAP: usize> ConstAttrValueValueOfKind<AttrKindOfStr>
    for AttributeValueForRendering<'a, CAP>
{
}
impl<'a, const CAP: usize> ConstAttrValueValueOfKind<AttrKindOfContentEditable>
    for AttributeValueForRendering<'a, CAP>
{
}

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;
