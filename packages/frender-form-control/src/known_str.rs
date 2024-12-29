use frender_common::{
    strings::{CsrStr, SsrStr},
    IntoStaticStr, IntoStaticStrCache, TempStr,
};

trait KnownStaticStr: 'static + AsRef<str> + SsrStr + CsrStr {}

frender_common::impl_many!(
    impl<__> KnownStaticStr
        for each_of![
            &'static str,
            String,
            std::borrow::Cow<'static, str>,
            std::rc::Rc<str>,
            std::sync::Arc<str>,
        ]
    {
    }
);

pub(crate) trait KnownSsrStr: SsrStr {}

impl<S: KnownStaticStr> KnownSsrStr for S {}
impl<S: IntoStaticStr> KnownSsrStr for TempStr<S> {}

pub(crate) trait KnownCsrStr: CsrStr {}

impl<S: KnownStaticStr> KnownCsrStr for S {}
impl<S: IntoStaticStrCache> KnownCsrStr for TempStr<S> {}
