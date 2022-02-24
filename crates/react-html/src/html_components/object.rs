use crate::HtmlCommonSharedPropsBuilder;

crate::macros::def_intrinsic_component! {
    "object"
    ObjectComponent(ObjectComponentProps) {
        ObjectComponentProps
        : ObjectComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlObjectElement]
        {
            class_id: Option<&str>,
            data: Option<&str>,
            form: Option<&str>,
            height['a, T: Into<crate::NumOrStr<'a>>]: Option<T> { into? |v| v.into() },
            name: Option<&str>,
            html_type@"type": Option<&str>,
            use_map: Option<&str>,
            width['a, T: Into<crate::NumOrStr<'a>>]: Option<T> { into? |v| v.into() },
            wmode: Option<&str>,
        }
    }
}
