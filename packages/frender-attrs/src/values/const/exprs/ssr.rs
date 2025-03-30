use std::marker::PhantomData;

use chtml::attrs::ssr::SpaceAndAttributesStrIntoIter;

use crate::{
    parser::attrs::AttributesForRendering,
    values::r#const::{
        ssr::{HasConstSsrAttributes, SsrConstAttributes},
        HasConstAttributes,
    },
};

impl<'a, const ATTRS: usize, const SSR_STRING_CAP: usize> SsrConstAttributes
    for AttributesForRendering<'a, ATTRS, SSR_STRING_CAP>
{
    type SsrAttributes = SpaceAndAttributesStrIntoIter<'a>;

    type IntoConstSsrAttributes<T: ?Sized + HasConstAttributes<Attributes = Self>> =
        AttributesForRenderingIntoSsrAttributes<T>;
}

enum Never {}
pub struct AttributesForRenderingIntoSsrAttributes<T: ?Sized>(Never, PhantomData<T>);

impl<
        'a,
        const ATTRS: usize,
        const SSR_STRING_CAP: usize,
        T: ?Sized + HasConstAttributes<Attributes = AttributesForRendering<'a, ATTRS, SSR_STRING_CAP>>,
    > HasConstSsrAttributes for AttributesForRenderingIntoSsrAttributes<T>
{
    type SsrAttributes = SpaceAndAttributesStrIntoIter<'a>;

    const SSR_ATTRIBUTES: Self::SsrAttributes =
        SpaceAndAttributesStrIntoIter::new(T::ATTRIBUTES.ssr_str());
}
