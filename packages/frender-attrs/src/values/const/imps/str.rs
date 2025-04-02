use crate::values::r#const::imp::ConstAttributesAttributes;

use super::super::input::str::Output;

impl<'a, const ATTRS: usize, const CSR: usize, const SSR: usize, const SSR_STRING_CAP: usize>
    ConstAttributesAttributes for Output<'a, ATTRS, CSR, SSR, SSR_STRING_CAP>
{
}

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;
