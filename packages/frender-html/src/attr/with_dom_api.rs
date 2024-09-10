use frender_attr_value::csr::ValueKind;

pub(crate) trait SetAttributeWithDomApi: ValueKind {
    type DomApiValue<'a>;

    fn set_attribute_with_dom_api(api: impl FnOnce(Self::DomApiValue<'_>), value: Self::Value<'_>);
}

impl SetAttributeWithDomApi for str {
    type DomApiValue<'a> = &'a str;

    fn set_attribute_with_dom_api(api: impl FnOnce(Self::DomApiValue<'_>), value: Self::Value<'_>) {
        api(value)
    }
}

impl SetAttributeWithDomApi for bool {
    type DomApiValue<'a> = bool;

    fn set_attribute_with_dom_api(api: impl FnOnce(Self::DomApiValue<'_>), (): Self::Value<'_>) {
        api(true)
    }
}

pub(crate) trait KnownSimpleValueKind: for<'a> ValueKind<Value<'a> = Self> {}

impl KnownSimpleValueKind for i32 {}

impl KnownSimpleValueKind for u32 {}

impl KnownSimpleValueKind for f64 {}

impl<T: KnownSimpleValueKind> SetAttributeWithDomApi for T {
    type DomApiValue<'a> = T;

    fn set_attribute_with_dom_api(api: impl FnOnce(Self::DomApiValue<'_>), value: Self::Value<'_>) {
        api(value)
    }
}
