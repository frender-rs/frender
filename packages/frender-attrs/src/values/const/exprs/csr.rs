use crate::{
    csr::RenderAttributes, parser::attrs::AttributesForRendering,
    values::r#const::HasConstAttributes,
};

use super::super::csr::CsrConstAttributes;

impl<const ATTRS: usize, const SSR_STRING_CAP: usize> CsrConstAttributes
    for AttributesForRendering<'_, ATTRS, SSR_STRING_CAP>
{
    fn remove_all<T: ?Sized + HasConstAttributes<Attributes = Self>>(
        renderer: &mut impl RenderAttributes,
    ) {
        // TODO: test performance and bundle size if not mapping just names
        const {
            let attrs = T::ATTRIBUTES.attributes();
            let mut names = [""; ATTRS];

            let mut i = 0;

            while i < ATTRS {
                names[i] = attrs[i].0;
                i += 1;
            }

            names
        }
        .into_iter()
        .for_each(|name| renderer.remove_attribute(name));
    }

    fn set_all<T: ?Sized + HasConstAttributes<Attributes = Self>>(
        renderer: &mut impl RenderAttributes,
    ) {
        // TODO: test performance vs `&[].iter()`
        const { *T::ATTRIBUTES.attributes() }
            .into_iter()
            .for_each(|(name, value)| renderer.set_attribute(name, value));
    }
}
