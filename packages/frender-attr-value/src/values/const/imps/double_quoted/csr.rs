use chtml::encode::attribute_value::AttributeValueForRendering;
use const_core::convert::MarkerOfConstValue;

use crate::{csr::UpdateAttrValue, values::r#const::csr::ConstAttrValueCsrValue, AttrKindOfStr};

impl<'a, const CAP: usize> ConstAttrValueCsrValue<AttrKindOfStr>
    for AttributeValueForRendering<'a, CAP>
{
    fn render_init_on_absent_attribute<
        M: ?Sized + crate::values::r#const::HasConstAttrValue<AttrValue = Self>,
    >(
        updater: impl UpdateAttrValue<Kind = AttrKindOfStr>,
    ) {
        Self::render_init::<M>(updater)
    }

    fn render_init<M: ?Sized + crate::values::r#const::HasConstAttrValue<AttrValue = Self>>(
        updater: impl UpdateAttrValue<Kind = AttrKindOfStr>,
    ) {
        updater.set(const { M::ATTR_VALUE.as_str() });
    }

    type AttributeIsKnownAsAbsent<
        This: ?Sized + crate::values::r#const::HasConstAttrValue<AttrValue = Self>,
    > = ConstFalse;
}

pub enum ConstFalse {}

impl MarkerOfConstValue<bool> for ConstFalse {
    const VALUE: bool = false;
}

#[cfg(feature = "html")]
mod html {
    use chtml::encode::attribute_value::AttributeValueForRendering;

    use crate::{
        csr::UpdateAttrValue, html::AttrKindOfContentEditable,
        values::r#const::csr::ConstAttrValueCsrValue,
    };

    use super::ConstFalse;

    impl<'a, const CAP: usize> ConstAttrValueCsrValue<AttrKindOfContentEditable>
        for AttributeValueForRendering<'a, CAP>
    {
        fn render_init_on_absent_attribute<
            M: ?Sized + crate::values::r#const::HasConstAttrValue<AttrValue = Self>,
        >(
            updater: impl UpdateAttrValue<Kind = AttrKindOfContentEditable>,
        ) {
            Self::render_init::<M>(updater)
        }

        fn render_init<M: ?Sized + crate::values::r#const::HasConstAttrValue<AttrValue = Self>>(
            updater: impl UpdateAttrValue<Kind = AttrKindOfContentEditable>,
        ) {
            updater.set(const { M::ATTR_VALUE.as_str() })
        }

        type AttributeIsKnownAsAbsent<
            This: ?Sized + crate::values::r#const::HasConstAttrValue<AttrValue = Self>,
        > = ConstFalse;
    }
}
