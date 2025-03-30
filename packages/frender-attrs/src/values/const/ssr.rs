use frender_ssr_html::assert::SpaceAndHtmlAttributesOrEmpty;

use crate::ssr::SsrAttributes;

use super::{ConstAttributes, HasConstAttributes};

pub trait SsrConstAttributes {
    type SsrAttributes: SpaceAndHtmlAttributesOrEmpty;
    type IntoConstSsrAttributes<T: ?Sized + HasConstAttributes<Attributes = Self>>: HasConstSsrAttributes<SsrAttributes = Self::SsrAttributes>;
}

pub trait HasConstSsrAttributes {
    type SsrAttributes: SpaceAndHtmlAttributesOrEmpty;
    const SSR_ATTRIBUTES: Self::SsrAttributes;
}

impl<T: ?Sized + HasConstAttributes> SsrAttributes for ConstAttributes<T> {
    type IntoSsrAttributes = <T::Attributes as super::ssr::SsrConstAttributes>::SsrAttributes;

    fn into_ssr_attributes(self) -> Self::IntoSsrAttributes {
        <<T::Attributes as super::ssr::SsrConstAttributes>::IntoConstSsrAttributes<T> as HasConstSsrAttributes>::SSR_ATTRIBUTES
    }
}
