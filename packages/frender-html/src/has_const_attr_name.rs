#[cfg(feature = "ssr")]
pub(crate) use self::ssr::HasConstAttrNameSsr;

pub(crate) trait HasConstAttrName {
    const ATTR_NAME: &'static str;
}

#[cfg(feature = "ssr")]
mod ssr {
    use frender_ssr::html::attr::AssertSpaceAndHtmlAttributeName;

    use super::HasConstAttrName;

    pub(crate) trait HasConstAttrNameSsr: HasConstAttrName {
        const ASSERT_SPACE_AND_HTML_ATTRIBUTE_NAME: AssertSpaceAndHtmlAttributeName<&'static str>;
    }
}
