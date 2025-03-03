use crate::{
    csr::cached_some::{CsrAttrValueCachedSome, ImplCsrAttrValueWithCachedSome},
    known::KnownCsrStr,
};

use super::AttrKindOfStr;

impl<S: KnownCsrStr> ImplCsrAttrValueWithCachedSome for S {}
impl<S: KnownCsrStr> CsrAttrValueCachedSome<AttrKindOfStr> for S {}
