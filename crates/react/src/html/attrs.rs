use wasm_bindgen::JsValue;

pub enum Inheritable<T> {
    Value(T),
    Inherit,
}

/// 'none' | 'text' | 'tel' | 'url' | 'email' | 'numeric' | 'decimal' | 'search'
pub enum HtmlInputMode {
    None,
    Text,
    Tel,
    Url,
    Email,
    Numeric,
    Decimal,
    Search,
}

pub struct HtmlAttributes<V> {
    // React-specific Attributes
    default_checked: Option<bool>,
    default_value: Option<V>,
    suppress_content_editable_warning: Option<bool>,
    suppress_hydration_warning: Option<bool>,

    // Standard HTML Attributes
    access_key: Option<String>,
    class_name: Option<String>,
    content_editable: Inheritable<bool>,
    context_menu: Option<String>,
    dir: Option<String>,
    draggable: Option<bool>,
    hidden: Option<bool>,
    id: Option<String>,
    lang: Option<String>,
    placeholder: Option<String>,
    slot: Option<String>,
    spell_check: Option<bool>,
    style: Option<Box<super::css::CssProperties>>,
    tab_index: Option<i32>,
    title: Option<String>,
    translate: Option<bool>, // TODO: ser: yes | no

    // Unknown
    radio_group: Option<String>, // <command>, <menuitem>

    // WAI-ARIA
    role: Option<super::aria::AriaRole>,

    // RDFa Attributes
    about: Option<String>,
    datatype: Option<String>,
    inlist: JsValue,
    prefix: Option<String>,
    property: Option<String>,
    resource: Option<String>,
    kind_of: Option<String>, // TODO: ser typeof
    vocab: Option<String>,

    // Non-standard Attributes
    auto_capitalize: Option<String>,
    auto_correct: Option<String>,
    auto_save: Option<String>,
    color: Option<String>,
    item_prop: Option<String>,
    item_scope: Option<bool>,
    item_type: Option<String>,
    item_id: Option<String>,
    item_ref: Option<String>,
    results: Option<i32>,
    security: Option<String>,
    unselectable: Option<bool>, // TODO: ser: 'on' | 'off' | undefined;

    // Living Standard
    /**
     * Hints at the type of data that might be entered by the user while editing the element or its contents
     * @see https://html.spec.whatwg.org/multipage/interaction.html#input-modalities:-the-inputmode-attribute
     */
    input_mode: Option<HtmlInputMode>,
    /**
     * Specify that a standard HTML element should behave like a defined custom built-in element
     * @see https://html.spec.whatwg.org/multipage/custom-elements.html#attr-is
     */
    is: Option<String>,
}
