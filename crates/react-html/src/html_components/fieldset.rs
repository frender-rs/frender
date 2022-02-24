use crate::HtmlCommonSharedPropsBuilder;

crate::macros::def_intrinsic_component! {
    "fieldset"
    FieldSetComponent(FieldSetComponentProps) {
        FieldSetComponentProps
        : FieldSetComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlFieldSetElement]
        {
            disabled: Option<bool>,
            form: Option<&str>,
            name: Option<&str>,
        }
    }
}
