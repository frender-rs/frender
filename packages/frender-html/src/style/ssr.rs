pub use frender_style::Style as Bounds;

use frender_ssr::html::attr_value::AttrEqValue;
use frender_style::experimental::ssr::{SsrDeclarationList, SsrStyle};

use crate::impl_bounds::ssr::SpaceAndHtmlAttributes;

type Haevoe<V> = AttrEqValue<<<V as SsrStyle>::IntoSsrDeclarationList as SsrDeclarationList>::IntoDeclarationList>;

pub(crate) fn into_haevoe<V: SsrStyle>(this: V) -> Haevoe<V> {
    Haevoe::<V>::new(SsrDeclarationList::into_declaration_list(V::into_ssr_declaration_list(this)))
}

pub(crate) type Output<V> = SpaceAndHtmlAttributes<Haevoe<V>>;

// Style attribute is always present if specified
pub(crate) use crate::impl_bounds::ssr::into_space_and_html_attributes as output;
