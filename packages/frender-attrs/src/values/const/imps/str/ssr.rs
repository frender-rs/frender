use std::marker::PhantomData;

use chtml::attrs::ssr::SpaceAndAttributesStrIntoIter;

use crate::values::r#const::{
    ssr::{HasConstSsrAttributes, SsrConstAttributes},
    HasConstAttributes,
};

use super::Output;

impl<'a, const ATTRS: usize, const CSR: usize, const SSR: usize, const SSR_STRING_CAP: usize>
    SsrConstAttributes for Output<'a, ATTRS, CSR, SSR, SSR_STRING_CAP>
{
    type SsrAttributes = SpaceAndAttributesStrIntoIter<'a>;

    type IntoConstSsrAttributes<T: ?Sized + HasConstAttributes<Attributes = Self>> =
        OutputIntoSsrAttributes<T>;
}

enum Never {}
pub struct OutputIntoSsrAttributes<T: ?Sized>(Never, PhantomData<T>);

impl<
        'a,
        const ATTRS: usize,
        const CSR: usize,
        const SSR: usize,
        const SSR_STRING_CAP: usize,
        T: ?Sized + HasConstAttributes<Attributes = Output<'a, ATTRS, CSR, SSR, SSR_STRING_CAP>>,
    > HasConstSsrAttributes for OutputIntoSsrAttributes<T>
{
    type SsrAttributes = SpaceAndAttributesStrIntoIter<'a>;

    const SSR_ATTRIBUTES: Self::SsrAttributes =
        SpaceAndAttributesStrIntoIter::new(T::ATTRIBUTES.0.ssr_str());
}
