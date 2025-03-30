use crate::{
    csr::cached_some::{CsrAttrValueCachedSome, ImplCsrAttrValueWithCachedSome},
    known::KnownStr,
};

use super::AttrKindOfStr;

impl<S: KnownStr> ImplCsrAttrValueWithCachedSome for S {}
impl<S: KnownStr> CsrAttrValueCachedSome<AttrKindOfStr> for S {}
