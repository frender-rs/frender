use convert_js::ToJs;
use react::SafeIntoJsRuntime;
use wasm_bindgen::JsValue;

pub trait IntrinsicComponent {
    const INTRINSIC_TAG: &'static str;
}

crate::macros::def_props_trait! {
    [TElement, TValue: ToJs] HtmlBaseProps [TElement, TValue]
    : HtmlBasePropsBuilder
    {
        children[TNode: react::Node]: Option<TNode> {
            impl |this, v| {
                use react::any_js_props::AnyJsPropsBuilder;
                use react::Node;
                this.as_mut().set_children(v.into_react_children_js());
                this
            }
        },
        ref_el[TWriteRef: 'static + react::WriteRef<TElement> + react::SafeIntoJsRuntime]: Option<TWriteRef> { safe_into_js_runtime? },
        default_checked: Option<bool>,
        default_value: Option<TValue>,
        class@"className": Option<&str>,
        draggable: Option<bool>,
        hidden: Option<bool>,
        id: Option<&str>,
        lang: Option<&str>,
        placeholder: Option<&str>,
        style: Option<&crate::css::CssProperties> {
            impl |this, v| todo!()
        },
        tab_index: Option<i32>,
        title: Option<&str>,

        // React-specific Attributes
        suppress_content_editable_warning: Option<bool>,
        suppress_hydration_warning: Option<bool>,

        // Standard HTML Attributes
        access_key: Option<&str>,
        content_editable: Option<crate::Inheritable<bool>>,
        context_menu: Option<&str>,
        dir: Option<&str>,
        slot: Option<&str>,
        spell_check: Option<bool>,
        translate: Option<&str>, // TODO: ser: yes | no

        // Unknown
        radio_group: Option<&str>, // <command>, <menuitem>

        // WAI-ARIA
        role: Option<crate::aria::Role>,

        // RDFa Attributes
        about: Option<&str>,
        datatype: Option<&str>,
        inlist: Option<&wasm_bindgen::JsValue>,
        prefix: Option<&str>,
        property: Option<&str>,
        resource: Option<&str>,
        type_of@"typeof": Option<&str>,
        vocab: Option<&str>,

        // Non-standard Attributes
        auto_capitalize: Option<&str>,
        auto_correct: Option<&str>,
        auto_save: Option<&str>,
        color: Option<&str>,
        item_prop: Option<&str>,
        item_scope: Option<bool>,
        item_type: Option<&str>,
        item_id: Option<&str>,
        item_ref: Option<&str>,
        results: Option<i32>,
        security: Option<&str>,
        unselectable: Option<&str>, // TODO: ser: 'on' | 'off' | undefined;

        // Living Standard
        input_mode: Option<crate::HtmlInputMode>,
        is: Option<&str>,

        // events
        // TODO: def all events
        on_click[F: 'static + Fn(JsValue)]: Option<react::WrapFn<F, (JsValue,)>> {
            impl |this, v| {
                if let Some(v) = v {
                    this.as_mut().set_static_prop_and_persist("onClick", v.safe_into_js_runtime());
                } else {
                    use react::any_js_props::AnyJsPropsBuilder;
                    this.as_mut().remove_prop("onClick");
                }
                this
            }
        },
    }
}
