use frender_dom::dom_tokens::DomTokens;
use frender_ssr::html::attr_value::AttrEqValue;

use crate::impl_bounds::ssr::SpaceAndHtmlAttributes;

type Haevoe<V> = AttrEqValue<<V as DomTokens>::DomTokensIntoAsyncStrIter>;

pub(crate) fn into_haevoe<V: DomTokens>(this: V) -> Haevoe<V> {
    Haevoe::<V>::new(V::dom_tokens_into_async_str_iter(this))
}

pub(crate) type Output<V> = SpaceAndHtmlAttributes<Haevoe<V>>;

// DomTokens attributes are always present if specified
pub(crate) use crate::impl_bounds::ssr::into_space_and_html_attributes as output;
