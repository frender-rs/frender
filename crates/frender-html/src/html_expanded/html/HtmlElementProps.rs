#[allow(non_snake_case)]
pub fn HtmlElementProps() -> Building<TypesInitial> {
    #[allow(unused_imports)]
    use super::*;
    self::Building(self::Data {
        ElementProps: ElementProps::build(ElementProps()),
        access_key: (),
        auto_capitalize: (),
        auto_focus: (),
        content_editable: (),
        context_menu: (),
        dir: (),
        draggable: (),
        enter_key_hint: (),
        hidden: (),
        inert: (),
        input_mode: (),
        is: (),
        item_id: (),
        item_prop: (),
        item_ref: (),
        item_scope: (),
        item_type: (),
        lang: (),
        nonce: (),
        role: (),
        slot: (),
        spellcheck: (),
        style: (),
        tab_index: (),
        title: (),
        translate: (),
        virtual_keyboard_policy: (),
        on_click: (),
    })
}
pub mod prelude {}
pub mod overwrite {
    #![allow(non_camel_case_types)]
    pub type ElementProps<TypeDefs, Value> = dyn super::Types<
        ElementProps = Value,
        access_key = <TypeDefs as super::Types>::access_key,
        auto_capitalize = <TypeDefs as super::Types>::auto_capitalize,
        auto_focus = <TypeDefs as super::Types>::auto_focus,
        content_editable = <TypeDefs as super::Types>::content_editable,
        context_menu = <TypeDefs as super::Types>::context_menu,
        dir = <TypeDefs as super::Types>::dir,
        draggable = <TypeDefs as super::Types>::draggable,
        enter_key_hint = <TypeDefs as super::Types>::enter_key_hint,
        hidden = <TypeDefs as super::Types>::hidden,
        inert = <TypeDefs as super::Types>::inert,
        input_mode = <TypeDefs as super::Types>::input_mode,
        is = <TypeDefs as super::Types>::is,
        item_id = <TypeDefs as super::Types>::item_id,
        item_prop = <TypeDefs as super::Types>::item_prop,
        item_ref = <TypeDefs as super::Types>::item_ref,
        item_scope = <TypeDefs as super::Types>::item_scope,
        item_type = <TypeDefs as super::Types>::item_type,
        lang = <TypeDefs as super::Types>::lang,
        nonce = <TypeDefs as super::Types>::nonce,
        role = <TypeDefs as super::Types>::role,
        slot = <TypeDefs as super::Types>::slot,
        spellcheck = <TypeDefs as super::Types>::spellcheck,
        style = <TypeDefs as super::Types>::style,
        tab_index = <TypeDefs as super::Types>::tab_index,
        title = <TypeDefs as super::Types>::title,
        translate = <TypeDefs as super::Types>::translate,
        virtual_keyboard_policy = <TypeDefs as super::Types>::virtual_keyboard_policy,
        on_click = <TypeDefs as super::Types>::on_click,
    >;
    pub type children<TypeDefs, Value> = self::ElementProps<
        TypeDefs,
        super::super::ElementProps::overwrite::children<
            <TypeDefs as super::Types>::ElementProps,
            Value,
        >,
    >;
    pub type class<TypeDefs, Value> = self::ElementProps<
        TypeDefs,
        super::super::ElementProps::overwrite::class<
            <TypeDefs as super::Types>::ElementProps,
            Value,
        >,
    >;
    pub type id<TypeDefs, Value> = self::ElementProps<
        TypeDefs,
        super::super::ElementProps::overwrite::id<<TypeDefs as super::Types>::ElementProps, Value>,
    >;
    pub type part<TypeDefs, Value> = self::ElementProps<
        TypeDefs,
        super::super::ElementProps::overwrite::part<
            <TypeDefs as super::Types>::ElementProps,
            Value,
        >,
    >;
    pub type access_key<TypeDefs, Value> = dyn super::Types<
        ElementProps = <TypeDefs as super::Types>::ElementProps,
        access_key = Value,
        auto_capitalize = <TypeDefs as super::Types>::auto_capitalize,
        auto_focus = <TypeDefs as super::Types>::auto_focus,
        content_editable = <TypeDefs as super::Types>::content_editable,
        context_menu = <TypeDefs as super::Types>::context_menu,
        dir = <TypeDefs as super::Types>::dir,
        draggable = <TypeDefs as super::Types>::draggable,
        enter_key_hint = <TypeDefs as super::Types>::enter_key_hint,
        hidden = <TypeDefs as super::Types>::hidden,
        inert = <TypeDefs as super::Types>::inert,
        input_mode = <TypeDefs as super::Types>::input_mode,
        is = <TypeDefs as super::Types>::is,
        item_id = <TypeDefs as super::Types>::item_id,
        item_prop = <TypeDefs as super::Types>::item_prop,
        item_ref = <TypeDefs as super::Types>::item_ref,
        item_scope = <TypeDefs as super::Types>::item_scope,
        item_type = <TypeDefs as super::Types>::item_type,
        lang = <TypeDefs as super::Types>::lang,
        nonce = <TypeDefs as super::Types>::nonce,
        role = <TypeDefs as super::Types>::role,
        slot = <TypeDefs as super::Types>::slot,
        spellcheck = <TypeDefs as super::Types>::spellcheck,
        style = <TypeDefs as super::Types>::style,
        tab_index = <TypeDefs as super::Types>::tab_index,
        title = <TypeDefs as super::Types>::title,
        translate = <TypeDefs as super::Types>::translate,
        virtual_keyboard_policy = <TypeDefs as super::Types>::virtual_keyboard_policy,
        on_click = <TypeDefs as super::Types>::on_click,
    >;
    pub type auto_capitalize<TypeDefs, Value> = dyn super::Types<
        ElementProps = <TypeDefs as super::Types>::ElementProps,
        access_key = <TypeDefs as super::Types>::access_key,
        auto_capitalize = Value,
        auto_focus = <TypeDefs as super::Types>::auto_focus,
        content_editable = <TypeDefs as super::Types>::content_editable,
        context_menu = <TypeDefs as super::Types>::context_menu,
        dir = <TypeDefs as super::Types>::dir,
        draggable = <TypeDefs as super::Types>::draggable,
        enter_key_hint = <TypeDefs as super::Types>::enter_key_hint,
        hidden = <TypeDefs as super::Types>::hidden,
        inert = <TypeDefs as super::Types>::inert,
        input_mode = <TypeDefs as super::Types>::input_mode,
        is = <TypeDefs as super::Types>::is,
        item_id = <TypeDefs as super::Types>::item_id,
        item_prop = <TypeDefs as super::Types>::item_prop,
        item_ref = <TypeDefs as super::Types>::item_ref,
        item_scope = <TypeDefs as super::Types>::item_scope,
        item_type = <TypeDefs as super::Types>::item_type,
        lang = <TypeDefs as super::Types>::lang,
        nonce = <TypeDefs as super::Types>::nonce,
        role = <TypeDefs as super::Types>::role,
        slot = <TypeDefs as super::Types>::slot,
        spellcheck = <TypeDefs as super::Types>::spellcheck,
        style = <TypeDefs as super::Types>::style,
        tab_index = <TypeDefs as super::Types>::tab_index,
        title = <TypeDefs as super::Types>::title,
        translate = <TypeDefs as super::Types>::translate,
        virtual_keyboard_policy = <TypeDefs as super::Types>::virtual_keyboard_policy,
        on_click = <TypeDefs as super::Types>::on_click,
    >;
    pub type auto_focus<TypeDefs, Value> = dyn super::Types<
        ElementProps = <TypeDefs as super::Types>::ElementProps,
        access_key = <TypeDefs as super::Types>::access_key,
        auto_capitalize = <TypeDefs as super::Types>::auto_capitalize,
        auto_focus = Value,
        content_editable = <TypeDefs as super::Types>::content_editable,
        context_menu = <TypeDefs as super::Types>::context_menu,
        dir = <TypeDefs as super::Types>::dir,
        draggable = <TypeDefs as super::Types>::draggable,
        enter_key_hint = <TypeDefs as super::Types>::enter_key_hint,
        hidden = <TypeDefs as super::Types>::hidden,
        inert = <TypeDefs as super::Types>::inert,
        input_mode = <TypeDefs as super::Types>::input_mode,
        is = <TypeDefs as super::Types>::is,
        item_id = <TypeDefs as super::Types>::item_id,
        item_prop = <TypeDefs as super::Types>::item_prop,
        item_ref = <TypeDefs as super::Types>::item_ref,
        item_scope = <TypeDefs as super::Types>::item_scope,
        item_type = <TypeDefs as super::Types>::item_type,
        lang = <TypeDefs as super::Types>::lang,
        nonce = <TypeDefs as super::Types>::nonce,
        role = <TypeDefs as super::Types>::role,
        slot = <TypeDefs as super::Types>::slot,
        spellcheck = <TypeDefs as super::Types>::spellcheck,
        style = <TypeDefs as super::Types>::style,
        tab_index = <TypeDefs as super::Types>::tab_index,
        title = <TypeDefs as super::Types>::title,
        translate = <TypeDefs as super::Types>::translate,
        virtual_keyboard_policy = <TypeDefs as super::Types>::virtual_keyboard_policy,
        on_click = <TypeDefs as super::Types>::on_click,
    >;
    pub type content_editable<TypeDefs, Value> = dyn super::Types<
        ElementProps = <TypeDefs as super::Types>::ElementProps,
        access_key = <TypeDefs as super::Types>::access_key,
        auto_capitalize = <TypeDefs as super::Types>::auto_capitalize,
        auto_focus = <TypeDefs as super::Types>::auto_focus,
        content_editable = Value,
        context_menu = <TypeDefs as super::Types>::context_menu,
        dir = <TypeDefs as super::Types>::dir,
        draggable = <TypeDefs as super::Types>::draggable,
        enter_key_hint = <TypeDefs as super::Types>::enter_key_hint,
        hidden = <TypeDefs as super::Types>::hidden,
        inert = <TypeDefs as super::Types>::inert,
        input_mode = <TypeDefs as super::Types>::input_mode,
        is = <TypeDefs as super::Types>::is,
        item_id = <TypeDefs as super::Types>::item_id,
        item_prop = <TypeDefs as super::Types>::item_prop,
        item_ref = <TypeDefs as super::Types>::item_ref,
        item_scope = <TypeDefs as super::Types>::item_scope,
        item_type = <TypeDefs as super::Types>::item_type,
        lang = <TypeDefs as super::Types>::lang,
        nonce = <TypeDefs as super::Types>::nonce,
        role = <TypeDefs as super::Types>::role,
        slot = <TypeDefs as super::Types>::slot,
        spellcheck = <TypeDefs as super::Types>::spellcheck,
        style = <TypeDefs as super::Types>::style,
        tab_index = <TypeDefs as super::Types>::tab_index,
        title = <TypeDefs as super::Types>::title,
        translate = <TypeDefs as super::Types>::translate,
        virtual_keyboard_policy = <TypeDefs as super::Types>::virtual_keyboard_policy,
        on_click = <TypeDefs as super::Types>::on_click,
    >;
    pub type context_menu<TypeDefs, Value> = dyn super::Types<
        ElementProps = <TypeDefs as super::Types>::ElementProps,
        access_key = <TypeDefs as super::Types>::access_key,
        auto_capitalize = <TypeDefs as super::Types>::auto_capitalize,
        auto_focus = <TypeDefs as super::Types>::auto_focus,
        content_editable = <TypeDefs as super::Types>::content_editable,
        context_menu = Value,
        dir = <TypeDefs as super::Types>::dir,
        draggable = <TypeDefs as super::Types>::draggable,
        enter_key_hint = <TypeDefs as super::Types>::enter_key_hint,
        hidden = <TypeDefs as super::Types>::hidden,
        inert = <TypeDefs as super::Types>::inert,
        input_mode = <TypeDefs as super::Types>::input_mode,
        is = <TypeDefs as super::Types>::is,
        item_id = <TypeDefs as super::Types>::item_id,
        item_prop = <TypeDefs as super::Types>::item_prop,
        item_ref = <TypeDefs as super::Types>::item_ref,
        item_scope = <TypeDefs as super::Types>::item_scope,
        item_type = <TypeDefs as super::Types>::item_type,
        lang = <TypeDefs as super::Types>::lang,
        nonce = <TypeDefs as super::Types>::nonce,
        role = <TypeDefs as super::Types>::role,
        slot = <TypeDefs as super::Types>::slot,
        spellcheck = <TypeDefs as super::Types>::spellcheck,
        style = <TypeDefs as super::Types>::style,
        tab_index = <TypeDefs as super::Types>::tab_index,
        title = <TypeDefs as super::Types>::title,
        translate = <TypeDefs as super::Types>::translate,
        virtual_keyboard_policy = <TypeDefs as super::Types>::virtual_keyboard_policy,
        on_click = <TypeDefs as super::Types>::on_click,
    >;
    pub type dir<TypeDefs, Value> = dyn super::Types<
        ElementProps = <TypeDefs as super::Types>::ElementProps,
        access_key = <TypeDefs as super::Types>::access_key,
        auto_capitalize = <TypeDefs as super::Types>::auto_capitalize,
        auto_focus = <TypeDefs as super::Types>::auto_focus,
        content_editable = <TypeDefs as super::Types>::content_editable,
        context_menu = <TypeDefs as super::Types>::context_menu,
        dir = Value,
        draggable = <TypeDefs as super::Types>::draggable,
        enter_key_hint = <TypeDefs as super::Types>::enter_key_hint,
        hidden = <TypeDefs as super::Types>::hidden,
        inert = <TypeDefs as super::Types>::inert,
        input_mode = <TypeDefs as super::Types>::input_mode,
        is = <TypeDefs as super::Types>::is,
        item_id = <TypeDefs as super::Types>::item_id,
        item_prop = <TypeDefs as super::Types>::item_prop,
        item_ref = <TypeDefs as super::Types>::item_ref,
        item_scope = <TypeDefs as super::Types>::item_scope,
        item_type = <TypeDefs as super::Types>::item_type,
        lang = <TypeDefs as super::Types>::lang,
        nonce = <TypeDefs as super::Types>::nonce,
        role = <TypeDefs as super::Types>::role,
        slot = <TypeDefs as super::Types>::slot,
        spellcheck = <TypeDefs as super::Types>::spellcheck,
        style = <TypeDefs as super::Types>::style,
        tab_index = <TypeDefs as super::Types>::tab_index,
        title = <TypeDefs as super::Types>::title,
        translate = <TypeDefs as super::Types>::translate,
        virtual_keyboard_policy = <TypeDefs as super::Types>::virtual_keyboard_policy,
        on_click = <TypeDefs as super::Types>::on_click,
    >;
    pub type draggable<TypeDefs, Value> = dyn super::Types<
        ElementProps = <TypeDefs as super::Types>::ElementProps,
        access_key = <TypeDefs as super::Types>::access_key,
        auto_capitalize = <TypeDefs as super::Types>::auto_capitalize,
        auto_focus = <TypeDefs as super::Types>::auto_focus,
        content_editable = <TypeDefs as super::Types>::content_editable,
        context_menu = <TypeDefs as super::Types>::context_menu,
        dir = <TypeDefs as super::Types>::dir,
        draggable = Value,
        enter_key_hint = <TypeDefs as super::Types>::enter_key_hint,
        hidden = <TypeDefs as super::Types>::hidden,
        inert = <TypeDefs as super::Types>::inert,
        input_mode = <TypeDefs as super::Types>::input_mode,
        is = <TypeDefs as super::Types>::is,
        item_id = <TypeDefs as super::Types>::item_id,
        item_prop = <TypeDefs as super::Types>::item_prop,
        item_ref = <TypeDefs as super::Types>::item_ref,
        item_scope = <TypeDefs as super::Types>::item_scope,
        item_type = <TypeDefs as super::Types>::item_type,
        lang = <TypeDefs as super::Types>::lang,
        nonce = <TypeDefs as super::Types>::nonce,
        role = <TypeDefs as super::Types>::role,
        slot = <TypeDefs as super::Types>::slot,
        spellcheck = <TypeDefs as super::Types>::spellcheck,
        style = <TypeDefs as super::Types>::style,
        tab_index = <TypeDefs as super::Types>::tab_index,
        title = <TypeDefs as super::Types>::title,
        translate = <TypeDefs as super::Types>::translate,
        virtual_keyboard_policy = <TypeDefs as super::Types>::virtual_keyboard_policy,
        on_click = <TypeDefs as super::Types>::on_click,
    >;
    pub type enter_key_hint<TypeDefs, Value> = dyn super::Types<
        ElementProps = <TypeDefs as super::Types>::ElementProps,
        access_key = <TypeDefs as super::Types>::access_key,
        auto_capitalize = <TypeDefs as super::Types>::auto_capitalize,
        auto_focus = <TypeDefs as super::Types>::auto_focus,
        content_editable = <TypeDefs as super::Types>::content_editable,
        context_menu = <TypeDefs as super::Types>::context_menu,
        dir = <TypeDefs as super::Types>::dir,
        draggable = <TypeDefs as super::Types>::draggable,
        enter_key_hint = Value,
        hidden = <TypeDefs as super::Types>::hidden,
        inert = <TypeDefs as super::Types>::inert,
        input_mode = <TypeDefs as super::Types>::input_mode,
        is = <TypeDefs as super::Types>::is,
        item_id = <TypeDefs as super::Types>::item_id,
        item_prop = <TypeDefs as super::Types>::item_prop,
        item_ref = <TypeDefs as super::Types>::item_ref,
        item_scope = <TypeDefs as super::Types>::item_scope,
        item_type = <TypeDefs as super::Types>::item_type,
        lang = <TypeDefs as super::Types>::lang,
        nonce = <TypeDefs as super::Types>::nonce,
        role = <TypeDefs as super::Types>::role,
        slot = <TypeDefs as super::Types>::slot,
        spellcheck = <TypeDefs as super::Types>::spellcheck,
        style = <TypeDefs as super::Types>::style,
        tab_index = <TypeDefs as super::Types>::tab_index,
        title = <TypeDefs as super::Types>::title,
        translate = <TypeDefs as super::Types>::translate,
        virtual_keyboard_policy = <TypeDefs as super::Types>::virtual_keyboard_policy,
        on_click = <TypeDefs as super::Types>::on_click,
    >;
    pub type hidden<TypeDefs, Value> = dyn super::Types<
        ElementProps = <TypeDefs as super::Types>::ElementProps,
        access_key = <TypeDefs as super::Types>::access_key,
        auto_capitalize = <TypeDefs as super::Types>::auto_capitalize,
        auto_focus = <TypeDefs as super::Types>::auto_focus,
        content_editable = <TypeDefs as super::Types>::content_editable,
        context_menu = <TypeDefs as super::Types>::context_menu,
        dir = <TypeDefs as super::Types>::dir,
        draggable = <TypeDefs as super::Types>::draggable,
        enter_key_hint = <TypeDefs as super::Types>::enter_key_hint,
        hidden = Value,
        inert = <TypeDefs as super::Types>::inert,
        input_mode = <TypeDefs as super::Types>::input_mode,
        is = <TypeDefs as super::Types>::is,
        item_id = <TypeDefs as super::Types>::item_id,
        item_prop = <TypeDefs as super::Types>::item_prop,
        item_ref = <TypeDefs as super::Types>::item_ref,
        item_scope = <TypeDefs as super::Types>::item_scope,
        item_type = <TypeDefs as super::Types>::item_type,
        lang = <TypeDefs as super::Types>::lang,
        nonce = <TypeDefs as super::Types>::nonce,
        role = <TypeDefs as super::Types>::role,
        slot = <TypeDefs as super::Types>::slot,
        spellcheck = <TypeDefs as super::Types>::spellcheck,
        style = <TypeDefs as super::Types>::style,
        tab_index = <TypeDefs as super::Types>::tab_index,
        title = <TypeDefs as super::Types>::title,
        translate = <TypeDefs as super::Types>::translate,
        virtual_keyboard_policy = <TypeDefs as super::Types>::virtual_keyboard_policy,
        on_click = <TypeDefs as super::Types>::on_click,
    >;
    pub type inert<TypeDefs, Value> = dyn super::Types<
        ElementProps = <TypeDefs as super::Types>::ElementProps,
        access_key = <TypeDefs as super::Types>::access_key,
        auto_capitalize = <TypeDefs as super::Types>::auto_capitalize,
        auto_focus = <TypeDefs as super::Types>::auto_focus,
        content_editable = <TypeDefs as super::Types>::content_editable,
        context_menu = <TypeDefs as super::Types>::context_menu,
        dir = <TypeDefs as super::Types>::dir,
        draggable = <TypeDefs as super::Types>::draggable,
        enter_key_hint = <TypeDefs as super::Types>::enter_key_hint,
        hidden = <TypeDefs as super::Types>::hidden,
        inert = Value,
        input_mode = <TypeDefs as super::Types>::input_mode,
        is = <TypeDefs as super::Types>::is,
        item_id = <TypeDefs as super::Types>::item_id,
        item_prop = <TypeDefs as super::Types>::item_prop,
        item_ref = <TypeDefs as super::Types>::item_ref,
        item_scope = <TypeDefs as super::Types>::item_scope,
        item_type = <TypeDefs as super::Types>::item_type,
        lang = <TypeDefs as super::Types>::lang,
        nonce = <TypeDefs as super::Types>::nonce,
        role = <TypeDefs as super::Types>::role,
        slot = <TypeDefs as super::Types>::slot,
        spellcheck = <TypeDefs as super::Types>::spellcheck,
        style = <TypeDefs as super::Types>::style,
        tab_index = <TypeDefs as super::Types>::tab_index,
        title = <TypeDefs as super::Types>::title,
        translate = <TypeDefs as super::Types>::translate,
        virtual_keyboard_policy = <TypeDefs as super::Types>::virtual_keyboard_policy,
        on_click = <TypeDefs as super::Types>::on_click,
    >;
    pub type input_mode<TypeDefs, Value> = dyn super::Types<
        ElementProps = <TypeDefs as super::Types>::ElementProps,
        access_key = <TypeDefs as super::Types>::access_key,
        auto_capitalize = <TypeDefs as super::Types>::auto_capitalize,
        auto_focus = <TypeDefs as super::Types>::auto_focus,
        content_editable = <TypeDefs as super::Types>::content_editable,
        context_menu = <TypeDefs as super::Types>::context_menu,
        dir = <TypeDefs as super::Types>::dir,
        draggable = <TypeDefs as super::Types>::draggable,
        enter_key_hint = <TypeDefs as super::Types>::enter_key_hint,
        hidden = <TypeDefs as super::Types>::hidden,
        inert = <TypeDefs as super::Types>::inert,
        input_mode = Value,
        is = <TypeDefs as super::Types>::is,
        item_id = <TypeDefs as super::Types>::item_id,
        item_prop = <TypeDefs as super::Types>::item_prop,
        item_ref = <TypeDefs as super::Types>::item_ref,
        item_scope = <TypeDefs as super::Types>::item_scope,
        item_type = <TypeDefs as super::Types>::item_type,
        lang = <TypeDefs as super::Types>::lang,
        nonce = <TypeDefs as super::Types>::nonce,
        role = <TypeDefs as super::Types>::role,
        slot = <TypeDefs as super::Types>::slot,
        spellcheck = <TypeDefs as super::Types>::spellcheck,
        style = <TypeDefs as super::Types>::style,
        tab_index = <TypeDefs as super::Types>::tab_index,
        title = <TypeDefs as super::Types>::title,
        translate = <TypeDefs as super::Types>::translate,
        virtual_keyboard_policy = <TypeDefs as super::Types>::virtual_keyboard_policy,
        on_click = <TypeDefs as super::Types>::on_click,
    >;
    pub type is<TypeDefs, Value> = dyn super::Types<
        ElementProps = <TypeDefs as super::Types>::ElementProps,
        access_key = <TypeDefs as super::Types>::access_key,
        auto_capitalize = <TypeDefs as super::Types>::auto_capitalize,
        auto_focus = <TypeDefs as super::Types>::auto_focus,
        content_editable = <TypeDefs as super::Types>::content_editable,
        context_menu = <TypeDefs as super::Types>::context_menu,
        dir = <TypeDefs as super::Types>::dir,
        draggable = <TypeDefs as super::Types>::draggable,
        enter_key_hint = <TypeDefs as super::Types>::enter_key_hint,
        hidden = <TypeDefs as super::Types>::hidden,
        inert = <TypeDefs as super::Types>::inert,
        input_mode = <TypeDefs as super::Types>::input_mode,
        is = Value,
        item_id = <TypeDefs as super::Types>::item_id,
        item_prop = <TypeDefs as super::Types>::item_prop,
        item_ref = <TypeDefs as super::Types>::item_ref,
        item_scope = <TypeDefs as super::Types>::item_scope,
        item_type = <TypeDefs as super::Types>::item_type,
        lang = <TypeDefs as super::Types>::lang,
        nonce = <TypeDefs as super::Types>::nonce,
        role = <TypeDefs as super::Types>::role,
        slot = <TypeDefs as super::Types>::slot,
        spellcheck = <TypeDefs as super::Types>::spellcheck,
        style = <TypeDefs as super::Types>::style,
        tab_index = <TypeDefs as super::Types>::tab_index,
        title = <TypeDefs as super::Types>::title,
        translate = <TypeDefs as super::Types>::translate,
        virtual_keyboard_policy = <TypeDefs as super::Types>::virtual_keyboard_policy,
        on_click = <TypeDefs as super::Types>::on_click,
    >;
    pub type item_id<TypeDefs, Value> = dyn super::Types<
        ElementProps = <TypeDefs as super::Types>::ElementProps,
        access_key = <TypeDefs as super::Types>::access_key,
        auto_capitalize = <TypeDefs as super::Types>::auto_capitalize,
        auto_focus = <TypeDefs as super::Types>::auto_focus,
        content_editable = <TypeDefs as super::Types>::content_editable,
        context_menu = <TypeDefs as super::Types>::context_menu,
        dir = <TypeDefs as super::Types>::dir,
        draggable = <TypeDefs as super::Types>::draggable,
        enter_key_hint = <TypeDefs as super::Types>::enter_key_hint,
        hidden = <TypeDefs as super::Types>::hidden,
        inert = <TypeDefs as super::Types>::inert,
        input_mode = <TypeDefs as super::Types>::input_mode,
        is = <TypeDefs as super::Types>::is,
        item_id = Value,
        item_prop = <TypeDefs as super::Types>::item_prop,
        item_ref = <TypeDefs as super::Types>::item_ref,
        item_scope = <TypeDefs as super::Types>::item_scope,
        item_type = <TypeDefs as super::Types>::item_type,
        lang = <TypeDefs as super::Types>::lang,
        nonce = <TypeDefs as super::Types>::nonce,
        role = <TypeDefs as super::Types>::role,
        slot = <TypeDefs as super::Types>::slot,
        spellcheck = <TypeDefs as super::Types>::spellcheck,
        style = <TypeDefs as super::Types>::style,
        tab_index = <TypeDefs as super::Types>::tab_index,
        title = <TypeDefs as super::Types>::title,
        translate = <TypeDefs as super::Types>::translate,
        virtual_keyboard_policy = <TypeDefs as super::Types>::virtual_keyboard_policy,
        on_click = <TypeDefs as super::Types>::on_click,
    >;
    pub type item_prop<TypeDefs, Value> = dyn super::Types<
        ElementProps = <TypeDefs as super::Types>::ElementProps,
        access_key = <TypeDefs as super::Types>::access_key,
        auto_capitalize = <TypeDefs as super::Types>::auto_capitalize,
        auto_focus = <TypeDefs as super::Types>::auto_focus,
        content_editable = <TypeDefs as super::Types>::content_editable,
        context_menu = <TypeDefs as super::Types>::context_menu,
        dir = <TypeDefs as super::Types>::dir,
        draggable = <TypeDefs as super::Types>::draggable,
        enter_key_hint = <TypeDefs as super::Types>::enter_key_hint,
        hidden = <TypeDefs as super::Types>::hidden,
        inert = <TypeDefs as super::Types>::inert,
        input_mode = <TypeDefs as super::Types>::input_mode,
        is = <TypeDefs as super::Types>::is,
        item_id = <TypeDefs as super::Types>::item_id,
        item_prop = Value,
        item_ref = <TypeDefs as super::Types>::item_ref,
        item_scope = <TypeDefs as super::Types>::item_scope,
        item_type = <TypeDefs as super::Types>::item_type,
        lang = <TypeDefs as super::Types>::lang,
        nonce = <TypeDefs as super::Types>::nonce,
        role = <TypeDefs as super::Types>::role,
        slot = <TypeDefs as super::Types>::slot,
        spellcheck = <TypeDefs as super::Types>::spellcheck,
        style = <TypeDefs as super::Types>::style,
        tab_index = <TypeDefs as super::Types>::tab_index,
        title = <TypeDefs as super::Types>::title,
        translate = <TypeDefs as super::Types>::translate,
        virtual_keyboard_policy = <TypeDefs as super::Types>::virtual_keyboard_policy,
        on_click = <TypeDefs as super::Types>::on_click,
    >;
    pub type item_ref<TypeDefs, Value> = dyn super::Types<
        ElementProps = <TypeDefs as super::Types>::ElementProps,
        access_key = <TypeDefs as super::Types>::access_key,
        auto_capitalize = <TypeDefs as super::Types>::auto_capitalize,
        auto_focus = <TypeDefs as super::Types>::auto_focus,
        content_editable = <TypeDefs as super::Types>::content_editable,
        context_menu = <TypeDefs as super::Types>::context_menu,
        dir = <TypeDefs as super::Types>::dir,
        draggable = <TypeDefs as super::Types>::draggable,
        enter_key_hint = <TypeDefs as super::Types>::enter_key_hint,
        hidden = <TypeDefs as super::Types>::hidden,
        inert = <TypeDefs as super::Types>::inert,
        input_mode = <TypeDefs as super::Types>::input_mode,
        is = <TypeDefs as super::Types>::is,
        item_id = <TypeDefs as super::Types>::item_id,
        item_prop = <TypeDefs as super::Types>::item_prop,
        item_ref = Value,
        item_scope = <TypeDefs as super::Types>::item_scope,
        item_type = <TypeDefs as super::Types>::item_type,
        lang = <TypeDefs as super::Types>::lang,
        nonce = <TypeDefs as super::Types>::nonce,
        role = <TypeDefs as super::Types>::role,
        slot = <TypeDefs as super::Types>::slot,
        spellcheck = <TypeDefs as super::Types>::spellcheck,
        style = <TypeDefs as super::Types>::style,
        tab_index = <TypeDefs as super::Types>::tab_index,
        title = <TypeDefs as super::Types>::title,
        translate = <TypeDefs as super::Types>::translate,
        virtual_keyboard_policy = <TypeDefs as super::Types>::virtual_keyboard_policy,
        on_click = <TypeDefs as super::Types>::on_click,
    >;
    pub type item_scope<TypeDefs, Value> = dyn super::Types<
        ElementProps = <TypeDefs as super::Types>::ElementProps,
        access_key = <TypeDefs as super::Types>::access_key,
        auto_capitalize = <TypeDefs as super::Types>::auto_capitalize,
        auto_focus = <TypeDefs as super::Types>::auto_focus,
        content_editable = <TypeDefs as super::Types>::content_editable,
        context_menu = <TypeDefs as super::Types>::context_menu,
        dir = <TypeDefs as super::Types>::dir,
        draggable = <TypeDefs as super::Types>::draggable,
        enter_key_hint = <TypeDefs as super::Types>::enter_key_hint,
        hidden = <TypeDefs as super::Types>::hidden,
        inert = <TypeDefs as super::Types>::inert,
        input_mode = <TypeDefs as super::Types>::input_mode,
        is = <TypeDefs as super::Types>::is,
        item_id = <TypeDefs as super::Types>::item_id,
        item_prop = <TypeDefs as super::Types>::item_prop,
        item_ref = <TypeDefs as super::Types>::item_ref,
        item_scope = Value,
        item_type = <TypeDefs as super::Types>::item_type,
        lang = <TypeDefs as super::Types>::lang,
        nonce = <TypeDefs as super::Types>::nonce,
        role = <TypeDefs as super::Types>::role,
        slot = <TypeDefs as super::Types>::slot,
        spellcheck = <TypeDefs as super::Types>::spellcheck,
        style = <TypeDefs as super::Types>::style,
        tab_index = <TypeDefs as super::Types>::tab_index,
        title = <TypeDefs as super::Types>::title,
        translate = <TypeDefs as super::Types>::translate,
        virtual_keyboard_policy = <TypeDefs as super::Types>::virtual_keyboard_policy,
        on_click = <TypeDefs as super::Types>::on_click,
    >;
    pub type item_type<TypeDefs, Value> = dyn super::Types<
        ElementProps = <TypeDefs as super::Types>::ElementProps,
        access_key = <TypeDefs as super::Types>::access_key,
        auto_capitalize = <TypeDefs as super::Types>::auto_capitalize,
        auto_focus = <TypeDefs as super::Types>::auto_focus,
        content_editable = <TypeDefs as super::Types>::content_editable,
        context_menu = <TypeDefs as super::Types>::context_menu,
        dir = <TypeDefs as super::Types>::dir,
        draggable = <TypeDefs as super::Types>::draggable,
        enter_key_hint = <TypeDefs as super::Types>::enter_key_hint,
        hidden = <TypeDefs as super::Types>::hidden,
        inert = <TypeDefs as super::Types>::inert,
        input_mode = <TypeDefs as super::Types>::input_mode,
        is = <TypeDefs as super::Types>::is,
        item_id = <TypeDefs as super::Types>::item_id,
        item_prop = <TypeDefs as super::Types>::item_prop,
        item_ref = <TypeDefs as super::Types>::item_ref,
        item_scope = <TypeDefs as super::Types>::item_scope,
        item_type = Value,
        lang = <TypeDefs as super::Types>::lang,
        nonce = <TypeDefs as super::Types>::nonce,
        role = <TypeDefs as super::Types>::role,
        slot = <TypeDefs as super::Types>::slot,
        spellcheck = <TypeDefs as super::Types>::spellcheck,
        style = <TypeDefs as super::Types>::style,
        tab_index = <TypeDefs as super::Types>::tab_index,
        title = <TypeDefs as super::Types>::title,
        translate = <TypeDefs as super::Types>::translate,
        virtual_keyboard_policy = <TypeDefs as super::Types>::virtual_keyboard_policy,
        on_click = <TypeDefs as super::Types>::on_click,
    >;
    pub type lang<TypeDefs, Value> = dyn super::Types<
        ElementProps = <TypeDefs as super::Types>::ElementProps,
        access_key = <TypeDefs as super::Types>::access_key,
        auto_capitalize = <TypeDefs as super::Types>::auto_capitalize,
        auto_focus = <TypeDefs as super::Types>::auto_focus,
        content_editable = <TypeDefs as super::Types>::content_editable,
        context_menu = <TypeDefs as super::Types>::context_menu,
        dir = <TypeDefs as super::Types>::dir,
        draggable = <TypeDefs as super::Types>::draggable,
        enter_key_hint = <TypeDefs as super::Types>::enter_key_hint,
        hidden = <TypeDefs as super::Types>::hidden,
        inert = <TypeDefs as super::Types>::inert,
        input_mode = <TypeDefs as super::Types>::input_mode,
        is = <TypeDefs as super::Types>::is,
        item_id = <TypeDefs as super::Types>::item_id,
        item_prop = <TypeDefs as super::Types>::item_prop,
        item_ref = <TypeDefs as super::Types>::item_ref,
        item_scope = <TypeDefs as super::Types>::item_scope,
        item_type = <TypeDefs as super::Types>::item_type,
        lang = Value,
        nonce = <TypeDefs as super::Types>::nonce,
        role = <TypeDefs as super::Types>::role,
        slot = <TypeDefs as super::Types>::slot,
        spellcheck = <TypeDefs as super::Types>::spellcheck,
        style = <TypeDefs as super::Types>::style,
        tab_index = <TypeDefs as super::Types>::tab_index,
        title = <TypeDefs as super::Types>::title,
        translate = <TypeDefs as super::Types>::translate,
        virtual_keyboard_policy = <TypeDefs as super::Types>::virtual_keyboard_policy,
        on_click = <TypeDefs as super::Types>::on_click,
    >;
    pub type nonce<TypeDefs, Value> = dyn super::Types<
        ElementProps = <TypeDefs as super::Types>::ElementProps,
        access_key = <TypeDefs as super::Types>::access_key,
        auto_capitalize = <TypeDefs as super::Types>::auto_capitalize,
        auto_focus = <TypeDefs as super::Types>::auto_focus,
        content_editable = <TypeDefs as super::Types>::content_editable,
        context_menu = <TypeDefs as super::Types>::context_menu,
        dir = <TypeDefs as super::Types>::dir,
        draggable = <TypeDefs as super::Types>::draggable,
        enter_key_hint = <TypeDefs as super::Types>::enter_key_hint,
        hidden = <TypeDefs as super::Types>::hidden,
        inert = <TypeDefs as super::Types>::inert,
        input_mode = <TypeDefs as super::Types>::input_mode,
        is = <TypeDefs as super::Types>::is,
        item_id = <TypeDefs as super::Types>::item_id,
        item_prop = <TypeDefs as super::Types>::item_prop,
        item_ref = <TypeDefs as super::Types>::item_ref,
        item_scope = <TypeDefs as super::Types>::item_scope,
        item_type = <TypeDefs as super::Types>::item_type,
        lang = <TypeDefs as super::Types>::lang,
        nonce = Value,
        role = <TypeDefs as super::Types>::role,
        slot = <TypeDefs as super::Types>::slot,
        spellcheck = <TypeDefs as super::Types>::spellcheck,
        style = <TypeDefs as super::Types>::style,
        tab_index = <TypeDefs as super::Types>::tab_index,
        title = <TypeDefs as super::Types>::title,
        translate = <TypeDefs as super::Types>::translate,
        virtual_keyboard_policy = <TypeDefs as super::Types>::virtual_keyboard_policy,
        on_click = <TypeDefs as super::Types>::on_click,
    >;
    pub type role<TypeDefs, Value> = dyn super::Types<
        ElementProps = <TypeDefs as super::Types>::ElementProps,
        access_key = <TypeDefs as super::Types>::access_key,
        auto_capitalize = <TypeDefs as super::Types>::auto_capitalize,
        auto_focus = <TypeDefs as super::Types>::auto_focus,
        content_editable = <TypeDefs as super::Types>::content_editable,
        context_menu = <TypeDefs as super::Types>::context_menu,
        dir = <TypeDefs as super::Types>::dir,
        draggable = <TypeDefs as super::Types>::draggable,
        enter_key_hint = <TypeDefs as super::Types>::enter_key_hint,
        hidden = <TypeDefs as super::Types>::hidden,
        inert = <TypeDefs as super::Types>::inert,
        input_mode = <TypeDefs as super::Types>::input_mode,
        is = <TypeDefs as super::Types>::is,
        item_id = <TypeDefs as super::Types>::item_id,
        item_prop = <TypeDefs as super::Types>::item_prop,
        item_ref = <TypeDefs as super::Types>::item_ref,
        item_scope = <TypeDefs as super::Types>::item_scope,
        item_type = <TypeDefs as super::Types>::item_type,
        lang = <TypeDefs as super::Types>::lang,
        nonce = <TypeDefs as super::Types>::nonce,
        role = Value,
        slot = <TypeDefs as super::Types>::slot,
        spellcheck = <TypeDefs as super::Types>::spellcheck,
        style = <TypeDefs as super::Types>::style,
        tab_index = <TypeDefs as super::Types>::tab_index,
        title = <TypeDefs as super::Types>::title,
        translate = <TypeDefs as super::Types>::translate,
        virtual_keyboard_policy = <TypeDefs as super::Types>::virtual_keyboard_policy,
        on_click = <TypeDefs as super::Types>::on_click,
    >;
    pub type slot<TypeDefs, Value> = dyn super::Types<
        ElementProps = <TypeDefs as super::Types>::ElementProps,
        access_key = <TypeDefs as super::Types>::access_key,
        auto_capitalize = <TypeDefs as super::Types>::auto_capitalize,
        auto_focus = <TypeDefs as super::Types>::auto_focus,
        content_editable = <TypeDefs as super::Types>::content_editable,
        context_menu = <TypeDefs as super::Types>::context_menu,
        dir = <TypeDefs as super::Types>::dir,
        draggable = <TypeDefs as super::Types>::draggable,
        enter_key_hint = <TypeDefs as super::Types>::enter_key_hint,
        hidden = <TypeDefs as super::Types>::hidden,
        inert = <TypeDefs as super::Types>::inert,
        input_mode = <TypeDefs as super::Types>::input_mode,
        is = <TypeDefs as super::Types>::is,
        item_id = <TypeDefs as super::Types>::item_id,
        item_prop = <TypeDefs as super::Types>::item_prop,
        item_ref = <TypeDefs as super::Types>::item_ref,
        item_scope = <TypeDefs as super::Types>::item_scope,
        item_type = <TypeDefs as super::Types>::item_type,
        lang = <TypeDefs as super::Types>::lang,
        nonce = <TypeDefs as super::Types>::nonce,
        role = <TypeDefs as super::Types>::role,
        slot = Value,
        spellcheck = <TypeDefs as super::Types>::spellcheck,
        style = <TypeDefs as super::Types>::style,
        tab_index = <TypeDefs as super::Types>::tab_index,
        title = <TypeDefs as super::Types>::title,
        translate = <TypeDefs as super::Types>::translate,
        virtual_keyboard_policy = <TypeDefs as super::Types>::virtual_keyboard_policy,
        on_click = <TypeDefs as super::Types>::on_click,
    >;
    pub type spellcheck<TypeDefs, Value> = dyn super::Types<
        ElementProps = <TypeDefs as super::Types>::ElementProps,
        access_key = <TypeDefs as super::Types>::access_key,
        auto_capitalize = <TypeDefs as super::Types>::auto_capitalize,
        auto_focus = <TypeDefs as super::Types>::auto_focus,
        content_editable = <TypeDefs as super::Types>::content_editable,
        context_menu = <TypeDefs as super::Types>::context_menu,
        dir = <TypeDefs as super::Types>::dir,
        draggable = <TypeDefs as super::Types>::draggable,
        enter_key_hint = <TypeDefs as super::Types>::enter_key_hint,
        hidden = <TypeDefs as super::Types>::hidden,
        inert = <TypeDefs as super::Types>::inert,
        input_mode = <TypeDefs as super::Types>::input_mode,
        is = <TypeDefs as super::Types>::is,
        item_id = <TypeDefs as super::Types>::item_id,
        item_prop = <TypeDefs as super::Types>::item_prop,
        item_ref = <TypeDefs as super::Types>::item_ref,
        item_scope = <TypeDefs as super::Types>::item_scope,
        item_type = <TypeDefs as super::Types>::item_type,
        lang = <TypeDefs as super::Types>::lang,
        nonce = <TypeDefs as super::Types>::nonce,
        role = <TypeDefs as super::Types>::role,
        slot = <TypeDefs as super::Types>::slot,
        spellcheck = Value,
        style = <TypeDefs as super::Types>::style,
        tab_index = <TypeDefs as super::Types>::tab_index,
        title = <TypeDefs as super::Types>::title,
        translate = <TypeDefs as super::Types>::translate,
        virtual_keyboard_policy = <TypeDefs as super::Types>::virtual_keyboard_policy,
        on_click = <TypeDefs as super::Types>::on_click,
    >;
    pub type style<TypeDefs, Value> = dyn super::Types<
        ElementProps = <TypeDefs as super::Types>::ElementProps,
        access_key = <TypeDefs as super::Types>::access_key,
        auto_capitalize = <TypeDefs as super::Types>::auto_capitalize,
        auto_focus = <TypeDefs as super::Types>::auto_focus,
        content_editable = <TypeDefs as super::Types>::content_editable,
        context_menu = <TypeDefs as super::Types>::context_menu,
        dir = <TypeDefs as super::Types>::dir,
        draggable = <TypeDefs as super::Types>::draggable,
        enter_key_hint = <TypeDefs as super::Types>::enter_key_hint,
        hidden = <TypeDefs as super::Types>::hidden,
        inert = <TypeDefs as super::Types>::inert,
        input_mode = <TypeDefs as super::Types>::input_mode,
        is = <TypeDefs as super::Types>::is,
        item_id = <TypeDefs as super::Types>::item_id,
        item_prop = <TypeDefs as super::Types>::item_prop,
        item_ref = <TypeDefs as super::Types>::item_ref,
        item_scope = <TypeDefs as super::Types>::item_scope,
        item_type = <TypeDefs as super::Types>::item_type,
        lang = <TypeDefs as super::Types>::lang,
        nonce = <TypeDefs as super::Types>::nonce,
        role = <TypeDefs as super::Types>::role,
        slot = <TypeDefs as super::Types>::slot,
        spellcheck = <TypeDefs as super::Types>::spellcheck,
        style = Value,
        tab_index = <TypeDefs as super::Types>::tab_index,
        title = <TypeDefs as super::Types>::title,
        translate = <TypeDefs as super::Types>::translate,
        virtual_keyboard_policy = <TypeDefs as super::Types>::virtual_keyboard_policy,
        on_click = <TypeDefs as super::Types>::on_click,
    >;
    pub type tab_index<TypeDefs, Value> = dyn super::Types<
        ElementProps = <TypeDefs as super::Types>::ElementProps,
        access_key = <TypeDefs as super::Types>::access_key,
        auto_capitalize = <TypeDefs as super::Types>::auto_capitalize,
        auto_focus = <TypeDefs as super::Types>::auto_focus,
        content_editable = <TypeDefs as super::Types>::content_editable,
        context_menu = <TypeDefs as super::Types>::context_menu,
        dir = <TypeDefs as super::Types>::dir,
        draggable = <TypeDefs as super::Types>::draggable,
        enter_key_hint = <TypeDefs as super::Types>::enter_key_hint,
        hidden = <TypeDefs as super::Types>::hidden,
        inert = <TypeDefs as super::Types>::inert,
        input_mode = <TypeDefs as super::Types>::input_mode,
        is = <TypeDefs as super::Types>::is,
        item_id = <TypeDefs as super::Types>::item_id,
        item_prop = <TypeDefs as super::Types>::item_prop,
        item_ref = <TypeDefs as super::Types>::item_ref,
        item_scope = <TypeDefs as super::Types>::item_scope,
        item_type = <TypeDefs as super::Types>::item_type,
        lang = <TypeDefs as super::Types>::lang,
        nonce = <TypeDefs as super::Types>::nonce,
        role = <TypeDefs as super::Types>::role,
        slot = <TypeDefs as super::Types>::slot,
        spellcheck = <TypeDefs as super::Types>::spellcheck,
        style = <TypeDefs as super::Types>::style,
        tab_index = Value,
        title = <TypeDefs as super::Types>::title,
        translate = <TypeDefs as super::Types>::translate,
        virtual_keyboard_policy = <TypeDefs as super::Types>::virtual_keyboard_policy,
        on_click = <TypeDefs as super::Types>::on_click,
    >;
    pub type title<TypeDefs, Value> = dyn super::Types<
        ElementProps = <TypeDefs as super::Types>::ElementProps,
        access_key = <TypeDefs as super::Types>::access_key,
        auto_capitalize = <TypeDefs as super::Types>::auto_capitalize,
        auto_focus = <TypeDefs as super::Types>::auto_focus,
        content_editable = <TypeDefs as super::Types>::content_editable,
        context_menu = <TypeDefs as super::Types>::context_menu,
        dir = <TypeDefs as super::Types>::dir,
        draggable = <TypeDefs as super::Types>::draggable,
        enter_key_hint = <TypeDefs as super::Types>::enter_key_hint,
        hidden = <TypeDefs as super::Types>::hidden,
        inert = <TypeDefs as super::Types>::inert,
        input_mode = <TypeDefs as super::Types>::input_mode,
        is = <TypeDefs as super::Types>::is,
        item_id = <TypeDefs as super::Types>::item_id,
        item_prop = <TypeDefs as super::Types>::item_prop,
        item_ref = <TypeDefs as super::Types>::item_ref,
        item_scope = <TypeDefs as super::Types>::item_scope,
        item_type = <TypeDefs as super::Types>::item_type,
        lang = <TypeDefs as super::Types>::lang,
        nonce = <TypeDefs as super::Types>::nonce,
        role = <TypeDefs as super::Types>::role,
        slot = <TypeDefs as super::Types>::slot,
        spellcheck = <TypeDefs as super::Types>::spellcheck,
        style = <TypeDefs as super::Types>::style,
        tab_index = <TypeDefs as super::Types>::tab_index,
        title = Value,
        translate = <TypeDefs as super::Types>::translate,
        virtual_keyboard_policy = <TypeDefs as super::Types>::virtual_keyboard_policy,
        on_click = <TypeDefs as super::Types>::on_click,
    >;
    pub type translate<TypeDefs, Value> = dyn super::Types<
        ElementProps = <TypeDefs as super::Types>::ElementProps,
        access_key = <TypeDefs as super::Types>::access_key,
        auto_capitalize = <TypeDefs as super::Types>::auto_capitalize,
        auto_focus = <TypeDefs as super::Types>::auto_focus,
        content_editable = <TypeDefs as super::Types>::content_editable,
        context_menu = <TypeDefs as super::Types>::context_menu,
        dir = <TypeDefs as super::Types>::dir,
        draggable = <TypeDefs as super::Types>::draggable,
        enter_key_hint = <TypeDefs as super::Types>::enter_key_hint,
        hidden = <TypeDefs as super::Types>::hidden,
        inert = <TypeDefs as super::Types>::inert,
        input_mode = <TypeDefs as super::Types>::input_mode,
        is = <TypeDefs as super::Types>::is,
        item_id = <TypeDefs as super::Types>::item_id,
        item_prop = <TypeDefs as super::Types>::item_prop,
        item_ref = <TypeDefs as super::Types>::item_ref,
        item_scope = <TypeDefs as super::Types>::item_scope,
        item_type = <TypeDefs as super::Types>::item_type,
        lang = <TypeDefs as super::Types>::lang,
        nonce = <TypeDefs as super::Types>::nonce,
        role = <TypeDefs as super::Types>::role,
        slot = <TypeDefs as super::Types>::slot,
        spellcheck = <TypeDefs as super::Types>::spellcheck,
        style = <TypeDefs as super::Types>::style,
        tab_index = <TypeDefs as super::Types>::tab_index,
        title = <TypeDefs as super::Types>::title,
        translate = Value,
        virtual_keyboard_policy = <TypeDefs as super::Types>::virtual_keyboard_policy,
        on_click = <TypeDefs as super::Types>::on_click,
    >;
    pub type virtual_keyboard_policy<TypeDefs, Value> = dyn super::Types<
        ElementProps = <TypeDefs as super::Types>::ElementProps,
        access_key = <TypeDefs as super::Types>::access_key,
        auto_capitalize = <TypeDefs as super::Types>::auto_capitalize,
        auto_focus = <TypeDefs as super::Types>::auto_focus,
        content_editable = <TypeDefs as super::Types>::content_editable,
        context_menu = <TypeDefs as super::Types>::context_menu,
        dir = <TypeDefs as super::Types>::dir,
        draggable = <TypeDefs as super::Types>::draggable,
        enter_key_hint = <TypeDefs as super::Types>::enter_key_hint,
        hidden = <TypeDefs as super::Types>::hidden,
        inert = <TypeDefs as super::Types>::inert,
        input_mode = <TypeDefs as super::Types>::input_mode,
        is = <TypeDefs as super::Types>::is,
        item_id = <TypeDefs as super::Types>::item_id,
        item_prop = <TypeDefs as super::Types>::item_prop,
        item_ref = <TypeDefs as super::Types>::item_ref,
        item_scope = <TypeDefs as super::Types>::item_scope,
        item_type = <TypeDefs as super::Types>::item_type,
        lang = <TypeDefs as super::Types>::lang,
        nonce = <TypeDefs as super::Types>::nonce,
        role = <TypeDefs as super::Types>::role,
        slot = <TypeDefs as super::Types>::slot,
        spellcheck = <TypeDefs as super::Types>::spellcheck,
        style = <TypeDefs as super::Types>::style,
        tab_index = <TypeDefs as super::Types>::tab_index,
        title = <TypeDefs as super::Types>::title,
        translate = <TypeDefs as super::Types>::translate,
        virtual_keyboard_policy = Value,
        on_click = <TypeDefs as super::Types>::on_click,
    >;
    pub type on_click<TypeDefs, Value> = dyn super::Types<
        ElementProps = <TypeDefs as super::Types>::ElementProps,
        access_key = <TypeDefs as super::Types>::access_key,
        auto_capitalize = <TypeDefs as super::Types>::auto_capitalize,
        auto_focus = <TypeDefs as super::Types>::auto_focus,
        content_editable = <TypeDefs as super::Types>::content_editable,
        context_menu = <TypeDefs as super::Types>::context_menu,
        dir = <TypeDefs as super::Types>::dir,
        draggable = <TypeDefs as super::Types>::draggable,
        enter_key_hint = <TypeDefs as super::Types>::enter_key_hint,
        hidden = <TypeDefs as super::Types>::hidden,
        inert = <TypeDefs as super::Types>::inert,
        input_mode = <TypeDefs as super::Types>::input_mode,
        is = <TypeDefs as super::Types>::is,
        item_id = <TypeDefs as super::Types>::item_id,
        item_prop = <TypeDefs as super::Types>::item_prop,
        item_ref = <TypeDefs as super::Types>::item_ref,
        item_scope = <TypeDefs as super::Types>::item_scope,
        item_type = <TypeDefs as super::Types>::item_type,
        lang = <TypeDefs as super::Types>::lang,
        nonce = <TypeDefs as super::Types>::nonce,
        role = <TypeDefs as super::Types>::role,
        slot = <TypeDefs as super::Types>::slot,
        spellcheck = <TypeDefs as super::Types>::spellcheck,
        style = <TypeDefs as super::Types>::style,
        tab_index = <TypeDefs as super::Types>::tab_index,
        title = <TypeDefs as super::Types>::title,
        translate = <TypeDefs as super::Types>::translate,
        virtual_keyboard_policy = <TypeDefs as super::Types>::virtual_keyboard_policy,
        on_click = Value,
    >;
}
mod trait_types {
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub trait Types {
        type ElementProps: ?::core::marker::Sized + ElementProps::Types;
        type access_key: crate::MaybeUpdateValueByRef<str>;
        type auto_capitalize: crate::MaybeUpdateValueByRef<str>;
        type auto_focus: crate::MaybeUpdateValue<bool>;
        type content_editable: crate::props::MaybeInherit<bool>;
        type context_menu: crate::MaybeUpdateValueByRef<str>;
        type dir: crate::MaybeUpdateValueByRef<str>;
        type draggable: crate::MaybeUpdateValue<bool>;
        type enter_key_hint: crate::MaybeUpdateValueByRef<str>;
        type hidden: crate::MaybeUpdateValue<bool>;
        type inert: crate::MaybeUpdateValue<bool>;
        type input_mode: crate::MaybeUpdateValueByRef<str>;
        type is: crate::MaybeUpdateValueByRef<str>;
        type item_id: crate::MaybeUpdateValueByRef<str>;
        type item_prop: crate::MaybeUpdateValueByRef<str>;
        type item_ref: crate::MaybeUpdateValueByRef<str>;
        type item_scope: crate::MaybeUpdateValueByRef<str>;
        type item_type: crate::MaybeUpdateValueByRef<str>;
        type lang: crate::MaybeUpdateValueByRef<str>;
        type nonce: crate::MaybeUpdateValueByRef<str>;
        type role: crate::MaybeUpdateValueByRef<str>;
        type slot: crate::MaybeUpdateValueByRef<str>;
        type spellcheck: crate::MaybeUpdateValue<bool>;
        type style: crate::MaybeUpdateValueByRef<str>;
        type tab_index: crate::MaybeUpdateValue<i32>;
        type title: crate::MaybeUpdateValueByRef<str>;
        type translate: crate::MaybeUpdateValueByRef<str>;
        type virtual_keyboard_policy: crate::MaybeUpdateValueByRef<str>;
        type on_click;
    }
}
pub use trait_types::Types;
pub use trait_types::Types as ValidTypes;
pub mod data_struct {
    #[non_exhaustive]
    pub struct HtmlElementProps<TypeDefs: super::Types + ?::core::marker::Sized> {
        pub ElementProps: super::super::ElementProps::Data<TypeDefs::ElementProps>,
        pub access_key: TypeDefs::access_key,
        pub auto_capitalize: TypeDefs::auto_capitalize,
        pub auto_focus: TypeDefs::auto_focus,
        pub content_editable: TypeDefs::content_editable,
        pub context_menu: TypeDefs::context_menu,
        pub dir: TypeDefs::dir,
        pub draggable: TypeDefs::draggable,
        pub enter_key_hint: TypeDefs::enter_key_hint,
        pub hidden: TypeDefs::hidden,
        pub inert: TypeDefs::inert,
        pub input_mode: TypeDefs::input_mode,
        pub is: TypeDefs::is,
        pub item_id: TypeDefs::item_id,
        pub item_prop: TypeDefs::item_prop,
        pub item_ref: TypeDefs::item_ref,
        pub item_scope: TypeDefs::item_scope,
        pub item_type: TypeDefs::item_type,
        pub lang: TypeDefs::lang,
        pub nonce: TypeDefs::nonce,
        pub role: TypeDefs::role,
        pub slot: TypeDefs::slot,
        pub spellcheck: TypeDefs::spellcheck,
        pub style: TypeDefs::style,
        pub tab_index: TypeDefs::tab_index,
        pub title: TypeDefs::title,
        pub translate: TypeDefs::translate,
        pub virtual_keyboard_policy: TypeDefs::virtual_keyboard_policy,
        pub on_click: TypeDefs::on_click,
    }
}
pub use data_struct::HtmlElementProps as Data;
pub struct Building<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
pub struct Replacing<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
mod types_initial {
    #[allow(unused_imports)]
    use super::super::*;
    pub type TypesInitial = dyn super::Types<
        ElementProps = ElementProps::TypesInitial,
        access_key = (),
        auto_capitalize = (),
        auto_focus = (),
        content_editable = (),
        context_menu = (),
        dir = (),
        draggable = (),
        enter_key_hint = (),
        hidden = (),
        inert = (),
        input_mode = (),
        is = (),
        item_id = (),
        item_prop = (),
        item_ref = (),
        item_scope = (),
        item_type = (),
        lang = (),
        nonce = (),
        role = (),
        slot = (),
        spellcheck = (),
        style = (),
        tab_index = (),
        title = (),
        translate = (),
        virtual_keyboard_policy = (),
        on_click = (),
    >;
}
pub use types_initial::TypesInitial;
pub type DataInitial = Data<TypesInitial>;
#[cfg(feature = "dom")]
pub mod render_state {
    #[allow(non_camel_case_types)]
    pub trait RenderStateTypes {
        type ElementProps: ::core::default::Default + crate::props::IntrinsicComponentPollReactive;
        type on_click: ::core::default::Default;
    }
    pub struct RenderState<TypeDefs: RenderStateTypes>
    where
        TypeDefs: ?::core::marker::Sized,
    {
        pub ElementProps: TypeDefs::ElementProps,
        pub on_click: TypeDefs::on_click,
    }
    #[allow(dead_code)]
    #[allow(single_use_lifetimes)]
    #[allow(clippy::unknown_clippy_lints)]
    #[allow(clippy::mut_mut)]
    #[allow(clippy::redundant_pub_crate)]
    #[allow(clippy::ref_option_ref)]
    #[allow(clippy::type_repetition_in_bounds)]
    pub(crate) struct RenderStateProj<'__pin, TypeDefs: RenderStateTypes>
    where
        RenderState<TypeDefs>: '__pin,
        TypeDefs: ?::core::marker::Sized,
    {
        pub ElementProps: ::pin_project_lite::__private::Pin<&'__pin mut (TypeDefs::ElementProps)>,
        pub on_click: &'__pin mut (TypeDefs::on_click),
    }
    #[allow(explicit_outlives_requirements)]
    #[allow(single_use_lifetimes)]
    #[allow(clippy::unknown_clippy_lints)]
    #[allow(clippy::redundant_pub_crate)]
    #[allow(clippy::used_underscore_binding)]
    const _: () = {
        #[allow(dead_code)]
        #[allow(single_use_lifetimes)]
        #[allow(clippy::unknown_clippy_lints)]
        #[allow(clippy::mut_mut)]
        #[allow(clippy::redundant_pub_crate)]
        #[allow(clippy::ref_option_ref)]
        #[allow(clippy::type_repetition_in_bounds)]
        pub(crate) struct ProjectionRef<'__pin, TypeDefs: RenderStateTypes>
        where
            RenderState<TypeDefs>: '__pin,
            TypeDefs: ?::core::marker::Sized,
        {
            pub ElementProps: ::pin_project_lite::__private::Pin<&'__pin (TypeDefs::ElementProps)>,
            pub on_click: &'__pin (TypeDefs::on_click),
        }
        impl<TypeDefs: RenderStateTypes> RenderState<TypeDefs>
        where
            TypeDefs: ?::core::marker::Sized,
        {
            pub(crate) fn project<'__pin>(
                self: ::pin_project_lite::__private::Pin<&'__pin mut Self>,
            ) -> RenderStateProj<'__pin, TypeDefs> {
                unsafe {
                    let Self {
                        ElementProps,
                        on_click,
                    } = self.get_unchecked_mut();
                    RenderStateProj {
                        ElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            ElementProps,
                        ),
                        on_click: on_click,
                    }
                }
            }
            pub(crate) fn project_ref<'__pin>(
                self: ::pin_project_lite::__private::Pin<&'__pin Self>,
            ) -> ProjectionRef<'__pin, TypeDefs> {
                unsafe {
                    let Self {
                        ElementProps,
                        on_click,
                    } = self.get_ref();
                    ProjectionRef {
                        ElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            ElementProps,
                        ),
                        on_click: on_click,
                    }
                }
            }
        }
        #[allow(non_snake_case)]
        pub struct __Origin<'__pin, TypeDefs: RenderStateTypes>
        where
            TypeDefs: ?::core::marker::Sized,
        {
            __dummy_lifetime: ::pin_project_lite::__private::PhantomData<&'__pin ()>,
            ElementProps: TypeDefs::ElementProps,
            on_click: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::on_click>,
        }
        impl<'__pin, TypeDefs: RenderStateTypes> ::pin_project_lite::__private::Unpin
            for RenderState<TypeDefs>
        where
            __Origin<'__pin, TypeDefs>: ::pin_project_lite::__private::Unpin,
            TypeDefs: ?::core::marker::Sized,
        {
        }
        trait MustNotImplDrop {}
        #[allow(clippy::drop_bounds, drop_bounds)]
        impl<T: ::pin_project_lite::__private::Drop> MustNotImplDrop for T {}
        impl<TypeDefs: RenderStateTypes> MustNotImplDrop for RenderState<TypeDefs> where
            TypeDefs: ?::core::marker::Sized
        {
        }
        #[forbid(unaligned_references, safe_packed_borrows)]
        fn __assert_not_repr_packed<TypeDefs: RenderStateTypes>(this: &RenderState<TypeDefs>)
        where
            TypeDefs: ?::core::marker::Sized,
        {
            let _ = &this.ElementProps;
            let _ = &this.on_click;
        }
    };
    impl<TypeDefs: ?::core::marker::Sized + RenderStateTypes> RenderState<TypeDefs> {
        #[inline]
        pub(crate) fn pin_project(
            self: ::core::pin::Pin<&mut Self>,
        ) -> RenderStateProj<'_, TypeDefs> {
            self.project()
        }
    }
    impl<TypeDefs: ?::core::marker::Sized + RenderStateTypes> ::core::default::Default
        for RenderState<TypeDefs>
    {
        fn default() -> Self {
            Self {
                ElementProps: ::core::default::Default::default(),
                on_click: ::core::default::Default::default(),
            }
        }
    }
    impl<TypeDefs: ?::core::marker::Sized + RenderStateTypes>
        crate::props::IntrinsicComponentPollReactive for RenderState<TypeDefs>
    {
        #[inline]
        fn intrinsic_component_poll_reactive(
            self: ::core::pin::Pin<&mut Self>,
            cx: &mut ::core::task::Context<'_>,
        ) -> ::core::task::Poll<bool> {
            crate::props::IntrinsicComponentPollReactive::intrinsic_component_poll_reactive(
                self.project().ElementProps,
                cx,
            )
        }
    }
}
#[inline]
pub fn build<TypeDefs: ?::core::marker::Sized + Types>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    building.0
}
mod builder_and_replacer {
    #[allow(unused_imports)]
    use super::super::*;
    impl<TypeDefs: super::Types + ?::core::marker::Sized> super::Building<TypeDefs> {
        #[doc = "See [`ElementProps::children`]"]
        #[inline]
        pub fn children<V>(
            self,
            children: V,
        ) -> super::Building<super::overwrite::children<TypeDefs, V>> {
            super::Building(super::Data {
                ElementProps: ElementProps::build(
                    ElementProps::Building(self.0.ElementProps).children(children),
                ),
                access_key: self.0.access_key,
                auto_capitalize: self.0.auto_capitalize,
                auto_focus: self.0.auto_focus,
                content_editable: self.0.content_editable,
                context_menu: self.0.context_menu,
                dir: self.0.dir,
                draggable: self.0.draggable,
                enter_key_hint: self.0.enter_key_hint,
                hidden: self.0.hidden,
                inert: self.0.inert,
                input_mode: self.0.input_mode,
                is: self.0.is,
                item_id: self.0.item_id,
                item_prop: self.0.item_prop,
                item_ref: self.0.item_ref,
                item_scope: self.0.item_scope,
                item_type: self.0.item_type,
                lang: self.0.lang,
                nonce: self.0.nonce,
                role: self.0.role,
                slot: self.0.slot,
                spellcheck: self.0.spellcheck,
                style: self.0.style,
                tab_index: self.0.tab_index,
                title: self.0.title,
                translate: self.0.translate,
                virtual_keyboard_policy: self.0.virtual_keyboard_policy,
                on_click: self.0.on_click,
            })
        }
        #[doc = "See [`ElementProps::class`]"]
        #[inline]
        pub fn class<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            class: V,
        ) -> super::Building<super::overwrite::class<TypeDefs, V>> {
            super::Building(super::Data {
                ElementProps: ElementProps::build(
                    ElementProps::Building(self.0.ElementProps).class(class),
                ),
                access_key: self.0.access_key,
                auto_capitalize: self.0.auto_capitalize,
                auto_focus: self.0.auto_focus,
                content_editable: self.0.content_editable,
                context_menu: self.0.context_menu,
                dir: self.0.dir,
                draggable: self.0.draggable,
                enter_key_hint: self.0.enter_key_hint,
                hidden: self.0.hidden,
                inert: self.0.inert,
                input_mode: self.0.input_mode,
                is: self.0.is,
                item_id: self.0.item_id,
                item_prop: self.0.item_prop,
                item_ref: self.0.item_ref,
                item_scope: self.0.item_scope,
                item_type: self.0.item_type,
                lang: self.0.lang,
                nonce: self.0.nonce,
                role: self.0.role,
                slot: self.0.slot,
                spellcheck: self.0.spellcheck,
                style: self.0.style,
                tab_index: self.0.tab_index,
                title: self.0.title,
                translate: self.0.translate,
                virtual_keyboard_policy: self.0.virtual_keyboard_policy,
                on_click: self.0.on_click,
            })
        }
        #[doc = "See [`ElementProps::id`]"]
        #[inline]
        pub fn id<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            id: V,
        ) -> super::Building<super::overwrite::id<TypeDefs, V>> {
            super::Building(super::Data {
                ElementProps: ElementProps::build(
                    ElementProps::Building(self.0.ElementProps).id(id),
                ),
                access_key: self.0.access_key,
                auto_capitalize: self.0.auto_capitalize,
                auto_focus: self.0.auto_focus,
                content_editable: self.0.content_editable,
                context_menu: self.0.context_menu,
                dir: self.0.dir,
                draggable: self.0.draggable,
                enter_key_hint: self.0.enter_key_hint,
                hidden: self.0.hidden,
                inert: self.0.inert,
                input_mode: self.0.input_mode,
                is: self.0.is,
                item_id: self.0.item_id,
                item_prop: self.0.item_prop,
                item_ref: self.0.item_ref,
                item_scope: self.0.item_scope,
                item_type: self.0.item_type,
                lang: self.0.lang,
                nonce: self.0.nonce,
                role: self.0.role,
                slot: self.0.slot,
                spellcheck: self.0.spellcheck,
                style: self.0.style,
                tab_index: self.0.tab_index,
                title: self.0.title,
                translate: self.0.translate,
                virtual_keyboard_policy: self.0.virtual_keyboard_policy,
                on_click: self.0.on_click,
            })
        }
        #[doc = "See [`ElementProps::part`]"]
        #[inline]
        pub fn part<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            part: V,
        ) -> super::Building<super::overwrite::part<TypeDefs, V>> {
            super::Building(super::Data {
                ElementProps: ElementProps::build(
                    ElementProps::Building(self.0.ElementProps).part(part),
                ),
                access_key: self.0.access_key,
                auto_capitalize: self.0.auto_capitalize,
                auto_focus: self.0.auto_focus,
                content_editable: self.0.content_editable,
                context_menu: self.0.context_menu,
                dir: self.0.dir,
                draggable: self.0.draggable,
                enter_key_hint: self.0.enter_key_hint,
                hidden: self.0.hidden,
                inert: self.0.inert,
                input_mode: self.0.input_mode,
                is: self.0.is,
                item_id: self.0.item_id,
                item_prop: self.0.item_prop,
                item_ref: self.0.item_ref,
                item_scope: self.0.item_scope,
                item_type: self.0.item_type,
                lang: self.0.lang,
                nonce: self.0.nonce,
                role: self.0.role,
                slot: self.0.slot,
                spellcheck: self.0.spellcheck,
                style: self.0.style,
                tab_index: self.0.tab_index,
                title: self.0.title,
                translate: self.0.translate,
                virtual_keyboard_policy: self.0.virtual_keyboard_policy,
                on_click: self.0.on_click,
            })
        }
        #[inline]
        pub fn access_key<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            access_key: V,
        ) -> super::Building<super::overwrite::access_key<TypeDefs, V>> {
            super::Building(super::Data {
                ElementProps: self.0.ElementProps,
                access_key,
                auto_capitalize: self.0.auto_capitalize,
                auto_focus: self.0.auto_focus,
                content_editable: self.0.content_editable,
                context_menu: self.0.context_menu,
                dir: self.0.dir,
                draggable: self.0.draggable,
                enter_key_hint: self.0.enter_key_hint,
                hidden: self.0.hidden,
                inert: self.0.inert,
                input_mode: self.0.input_mode,
                is: self.0.is,
                item_id: self.0.item_id,
                item_prop: self.0.item_prop,
                item_ref: self.0.item_ref,
                item_scope: self.0.item_scope,
                item_type: self.0.item_type,
                lang: self.0.lang,
                nonce: self.0.nonce,
                role: self.0.role,
                slot: self.0.slot,
                spellcheck: self.0.spellcheck,
                style: self.0.style,
                tab_index: self.0.tab_index,
                title: self.0.title,
                translate: self.0.translate,
                virtual_keyboard_policy: self.0.virtual_keyboard_policy,
                on_click: self.0.on_click,
            })
        }
        #[inline]
        pub fn auto_capitalize<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            auto_capitalize: V,
        ) -> super::Building<super::overwrite::auto_capitalize<TypeDefs, V>> {
            super::Building(super::Data {
                ElementProps: self.0.ElementProps,
                access_key: self.0.access_key,
                auto_capitalize,
                auto_focus: self.0.auto_focus,
                content_editable: self.0.content_editable,
                context_menu: self.0.context_menu,
                dir: self.0.dir,
                draggable: self.0.draggable,
                enter_key_hint: self.0.enter_key_hint,
                hidden: self.0.hidden,
                inert: self.0.inert,
                input_mode: self.0.input_mode,
                is: self.0.is,
                item_id: self.0.item_id,
                item_prop: self.0.item_prop,
                item_ref: self.0.item_ref,
                item_scope: self.0.item_scope,
                item_type: self.0.item_type,
                lang: self.0.lang,
                nonce: self.0.nonce,
                role: self.0.role,
                slot: self.0.slot,
                spellcheck: self.0.spellcheck,
                style: self.0.style,
                tab_index: self.0.tab_index,
                title: self.0.title,
                translate: self.0.translate,
                virtual_keyboard_policy: self.0.virtual_keyboard_policy,
                on_click: self.0.on_click,
            })
        }
        #[inline]
        pub fn auto_focus<V: crate::MaybeUpdateValue<bool>>(
            self,
            auto_focus: V,
        ) -> super::Building<super::overwrite::auto_focus<TypeDefs, V>> {
            super::Building(super::Data {
                ElementProps: self.0.ElementProps,
                access_key: self.0.access_key,
                auto_capitalize: self.0.auto_capitalize,
                auto_focus,
                content_editable: self.0.content_editable,
                context_menu: self.0.context_menu,
                dir: self.0.dir,
                draggable: self.0.draggable,
                enter_key_hint: self.0.enter_key_hint,
                hidden: self.0.hidden,
                inert: self.0.inert,
                input_mode: self.0.input_mode,
                is: self.0.is,
                item_id: self.0.item_id,
                item_prop: self.0.item_prop,
                item_ref: self.0.item_ref,
                item_scope: self.0.item_scope,
                item_type: self.0.item_type,
                lang: self.0.lang,
                nonce: self.0.nonce,
                role: self.0.role,
                slot: self.0.slot,
                spellcheck: self.0.spellcheck,
                style: self.0.style,
                tab_index: self.0.tab_index,
                title: self.0.title,
                translate: self.0.translate,
                virtual_keyboard_policy: self.0.virtual_keyboard_policy,
                on_click: self.0.on_click,
            })
        }
        #[inline]
        pub fn content_editable<V: crate::props::MaybeInherit<bool>>(
            self,
            content_editable: V,
        ) -> super::Building<super::overwrite::content_editable<TypeDefs, V>> {
            super::Building(super::Data {
                ElementProps: self.0.ElementProps,
                access_key: self.0.access_key,
                auto_capitalize: self.0.auto_capitalize,
                auto_focus: self.0.auto_focus,
                content_editable,
                context_menu: self.0.context_menu,
                dir: self.0.dir,
                draggable: self.0.draggable,
                enter_key_hint: self.0.enter_key_hint,
                hidden: self.0.hidden,
                inert: self.0.inert,
                input_mode: self.0.input_mode,
                is: self.0.is,
                item_id: self.0.item_id,
                item_prop: self.0.item_prop,
                item_ref: self.0.item_ref,
                item_scope: self.0.item_scope,
                item_type: self.0.item_type,
                lang: self.0.lang,
                nonce: self.0.nonce,
                role: self.0.role,
                slot: self.0.slot,
                spellcheck: self.0.spellcheck,
                style: self.0.style,
                tab_index: self.0.tab_index,
                title: self.0.title,
                translate: self.0.translate,
                virtual_keyboard_policy: self.0.virtual_keyboard_policy,
                on_click: self.0.on_click,
            })
        }
        #[deprecated = "See https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/contextMenu"]
        #[inline]
        pub fn context_menu<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            context_menu: V,
        ) -> super::Building<super::overwrite::context_menu<TypeDefs, V>> {
            super::Building(super::Data {
                ElementProps: self.0.ElementProps,
                access_key: self.0.access_key,
                auto_capitalize: self.0.auto_capitalize,
                auto_focus: self.0.auto_focus,
                content_editable: self.0.content_editable,
                context_menu,
                dir: self.0.dir,
                draggable: self.0.draggable,
                enter_key_hint: self.0.enter_key_hint,
                hidden: self.0.hidden,
                inert: self.0.inert,
                input_mode: self.0.input_mode,
                is: self.0.is,
                item_id: self.0.item_id,
                item_prop: self.0.item_prop,
                item_ref: self.0.item_ref,
                item_scope: self.0.item_scope,
                item_type: self.0.item_type,
                lang: self.0.lang,
                nonce: self.0.nonce,
                role: self.0.role,
                slot: self.0.slot,
                spellcheck: self.0.spellcheck,
                style: self.0.style,
                tab_index: self.0.tab_index,
                title: self.0.title,
                translate: self.0.translate,
                virtual_keyboard_policy: self.0.virtual_keyboard_policy,
                on_click: self.0.on_click,
            })
        }
        #[inline]
        pub fn dir<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            dir: V,
        ) -> super::Building<super::overwrite::dir<TypeDefs, V>> {
            super::Building(super::Data {
                ElementProps: self.0.ElementProps,
                access_key: self.0.access_key,
                auto_capitalize: self.0.auto_capitalize,
                auto_focus: self.0.auto_focus,
                content_editable: self.0.content_editable,
                context_menu: self.0.context_menu,
                dir,
                draggable: self.0.draggable,
                enter_key_hint: self.0.enter_key_hint,
                hidden: self.0.hidden,
                inert: self.0.inert,
                input_mode: self.0.input_mode,
                is: self.0.is,
                item_id: self.0.item_id,
                item_prop: self.0.item_prop,
                item_ref: self.0.item_ref,
                item_scope: self.0.item_scope,
                item_type: self.0.item_type,
                lang: self.0.lang,
                nonce: self.0.nonce,
                role: self.0.role,
                slot: self.0.slot,
                spellcheck: self.0.spellcheck,
                style: self.0.style,
                tab_index: self.0.tab_index,
                title: self.0.title,
                translate: self.0.translate,
                virtual_keyboard_policy: self.0.virtual_keyboard_policy,
                on_click: self.0.on_click,
            })
        }
        #[inline]
        pub fn draggable<V: crate::MaybeUpdateValue<bool>>(
            self,
            draggable: V,
        ) -> super::Building<super::overwrite::draggable<TypeDefs, V>> {
            super::Building(super::Data {
                ElementProps: self.0.ElementProps,
                access_key: self.0.access_key,
                auto_capitalize: self.0.auto_capitalize,
                auto_focus: self.0.auto_focus,
                content_editable: self.0.content_editable,
                context_menu: self.0.context_menu,
                dir: self.0.dir,
                draggable,
                enter_key_hint: self.0.enter_key_hint,
                hidden: self.0.hidden,
                inert: self.0.inert,
                input_mode: self.0.input_mode,
                is: self.0.is,
                item_id: self.0.item_id,
                item_prop: self.0.item_prop,
                item_ref: self.0.item_ref,
                item_scope: self.0.item_scope,
                item_type: self.0.item_type,
                lang: self.0.lang,
                nonce: self.0.nonce,
                role: self.0.role,
                slot: self.0.slot,
                spellcheck: self.0.spellcheck,
                style: self.0.style,
                tab_index: self.0.tab_index,
                title: self.0.title,
                translate: self.0.translate,
                virtual_keyboard_policy: self.0.virtual_keyboard_policy,
                on_click: self.0.on_click,
            })
        }
        #[inline]
        pub fn enter_key_hint<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            enter_key_hint: V,
        ) -> super::Building<super::overwrite::enter_key_hint<TypeDefs, V>> {
            super::Building(super::Data {
                ElementProps: self.0.ElementProps,
                access_key: self.0.access_key,
                auto_capitalize: self.0.auto_capitalize,
                auto_focus: self.0.auto_focus,
                content_editable: self.0.content_editable,
                context_menu: self.0.context_menu,
                dir: self.0.dir,
                draggable: self.0.draggable,
                enter_key_hint,
                hidden: self.0.hidden,
                inert: self.0.inert,
                input_mode: self.0.input_mode,
                is: self.0.is,
                item_id: self.0.item_id,
                item_prop: self.0.item_prop,
                item_ref: self.0.item_ref,
                item_scope: self.0.item_scope,
                item_type: self.0.item_type,
                lang: self.0.lang,
                nonce: self.0.nonce,
                role: self.0.role,
                slot: self.0.slot,
                spellcheck: self.0.spellcheck,
                style: self.0.style,
                tab_index: self.0.tab_index,
                title: self.0.title,
                translate: self.0.translate,
                virtual_keyboard_policy: self.0.virtual_keyboard_policy,
                on_click: self.0.on_click,
            })
        }
        #[inline]
        pub fn hidden<V: crate::MaybeUpdateValue<bool>>(
            self,
            hidden: V,
        ) -> super::Building<super::overwrite::hidden<TypeDefs, V>> {
            super::Building(super::Data {
                ElementProps: self.0.ElementProps,
                access_key: self.0.access_key,
                auto_capitalize: self.0.auto_capitalize,
                auto_focus: self.0.auto_focus,
                content_editable: self.0.content_editable,
                context_menu: self.0.context_menu,
                dir: self.0.dir,
                draggable: self.0.draggable,
                enter_key_hint: self.0.enter_key_hint,
                hidden,
                inert: self.0.inert,
                input_mode: self.0.input_mode,
                is: self.0.is,
                item_id: self.0.item_id,
                item_prop: self.0.item_prop,
                item_ref: self.0.item_ref,
                item_scope: self.0.item_scope,
                item_type: self.0.item_type,
                lang: self.0.lang,
                nonce: self.0.nonce,
                role: self.0.role,
                slot: self.0.slot,
                spellcheck: self.0.spellcheck,
                style: self.0.style,
                tab_index: self.0.tab_index,
                title: self.0.title,
                translate: self.0.translate,
                virtual_keyboard_policy: self.0.virtual_keyboard_policy,
                on_click: self.0.on_click,
            })
        }
        #[inline]
        pub fn inert<V: crate::MaybeUpdateValue<bool>>(
            self,
            inert: V,
        ) -> super::Building<super::overwrite::inert<TypeDefs, V>> {
            super::Building(super::Data {
                ElementProps: self.0.ElementProps,
                access_key: self.0.access_key,
                auto_capitalize: self.0.auto_capitalize,
                auto_focus: self.0.auto_focus,
                content_editable: self.0.content_editable,
                context_menu: self.0.context_menu,
                dir: self.0.dir,
                draggable: self.0.draggable,
                enter_key_hint: self.0.enter_key_hint,
                hidden: self.0.hidden,
                inert,
                input_mode: self.0.input_mode,
                is: self.0.is,
                item_id: self.0.item_id,
                item_prop: self.0.item_prop,
                item_ref: self.0.item_ref,
                item_scope: self.0.item_scope,
                item_type: self.0.item_type,
                lang: self.0.lang,
                nonce: self.0.nonce,
                role: self.0.role,
                slot: self.0.slot,
                spellcheck: self.0.spellcheck,
                style: self.0.style,
                tab_index: self.0.tab_index,
                title: self.0.title,
                translate: self.0.translate,
                virtual_keyboard_policy: self.0.virtual_keyboard_policy,
                on_click: self.0.on_click,
            })
        }
        #[inline]
        pub fn input_mode<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            input_mode: V,
        ) -> super::Building<super::overwrite::input_mode<TypeDefs, V>> {
            super::Building(super::Data {
                ElementProps: self.0.ElementProps,
                access_key: self.0.access_key,
                auto_capitalize: self.0.auto_capitalize,
                auto_focus: self.0.auto_focus,
                content_editable: self.0.content_editable,
                context_menu: self.0.context_menu,
                dir: self.0.dir,
                draggable: self.0.draggable,
                enter_key_hint: self.0.enter_key_hint,
                hidden: self.0.hidden,
                inert: self.0.inert,
                input_mode,
                is: self.0.is,
                item_id: self.0.item_id,
                item_prop: self.0.item_prop,
                item_ref: self.0.item_ref,
                item_scope: self.0.item_scope,
                item_type: self.0.item_type,
                lang: self.0.lang,
                nonce: self.0.nonce,
                role: self.0.role,
                slot: self.0.slot,
                spellcheck: self.0.spellcheck,
                style: self.0.style,
                tab_index: self.0.tab_index,
                title: self.0.title,
                translate: self.0.translate,
                virtual_keyboard_policy: self.0.virtual_keyboard_policy,
                on_click: self.0.on_click,
            })
        }
        #[inline]
        pub fn is<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            is: V,
        ) -> super::Building<super::overwrite::is<TypeDefs, V>> {
            super::Building(super::Data {
                ElementProps: self.0.ElementProps,
                access_key: self.0.access_key,
                auto_capitalize: self.0.auto_capitalize,
                auto_focus: self.0.auto_focus,
                content_editable: self.0.content_editable,
                context_menu: self.0.context_menu,
                dir: self.0.dir,
                draggable: self.0.draggable,
                enter_key_hint: self.0.enter_key_hint,
                hidden: self.0.hidden,
                inert: self.0.inert,
                input_mode: self.0.input_mode,
                is,
                item_id: self.0.item_id,
                item_prop: self.0.item_prop,
                item_ref: self.0.item_ref,
                item_scope: self.0.item_scope,
                item_type: self.0.item_type,
                lang: self.0.lang,
                nonce: self.0.nonce,
                role: self.0.role,
                slot: self.0.slot,
                spellcheck: self.0.spellcheck,
                style: self.0.style,
                tab_index: self.0.tab_index,
                title: self.0.title,
                translate: self.0.translate,
                virtual_keyboard_policy: self.0.virtual_keyboard_policy,
                on_click: self.0.on_click,
            })
        }
        #[inline]
        pub fn item_id<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            item_id: V,
        ) -> super::Building<super::overwrite::item_id<TypeDefs, V>> {
            super::Building(super::Data {
                ElementProps: self.0.ElementProps,
                access_key: self.0.access_key,
                auto_capitalize: self.0.auto_capitalize,
                auto_focus: self.0.auto_focus,
                content_editable: self.0.content_editable,
                context_menu: self.0.context_menu,
                dir: self.0.dir,
                draggable: self.0.draggable,
                enter_key_hint: self.0.enter_key_hint,
                hidden: self.0.hidden,
                inert: self.0.inert,
                input_mode: self.0.input_mode,
                is: self.0.is,
                item_id,
                item_prop: self.0.item_prop,
                item_ref: self.0.item_ref,
                item_scope: self.0.item_scope,
                item_type: self.0.item_type,
                lang: self.0.lang,
                nonce: self.0.nonce,
                role: self.0.role,
                slot: self.0.slot,
                spellcheck: self.0.spellcheck,
                style: self.0.style,
                tab_index: self.0.tab_index,
                title: self.0.title,
                translate: self.0.translate,
                virtual_keyboard_policy: self.0.virtual_keyboard_policy,
                on_click: self.0.on_click,
            })
        }
        #[inline]
        pub fn item_prop<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            item_prop: V,
        ) -> super::Building<super::overwrite::item_prop<TypeDefs, V>> {
            super::Building(super::Data {
                ElementProps: self.0.ElementProps,
                access_key: self.0.access_key,
                auto_capitalize: self.0.auto_capitalize,
                auto_focus: self.0.auto_focus,
                content_editable: self.0.content_editable,
                context_menu: self.0.context_menu,
                dir: self.0.dir,
                draggable: self.0.draggable,
                enter_key_hint: self.0.enter_key_hint,
                hidden: self.0.hidden,
                inert: self.0.inert,
                input_mode: self.0.input_mode,
                is: self.0.is,
                item_id: self.0.item_id,
                item_prop,
                item_ref: self.0.item_ref,
                item_scope: self.0.item_scope,
                item_type: self.0.item_type,
                lang: self.0.lang,
                nonce: self.0.nonce,
                role: self.0.role,
                slot: self.0.slot,
                spellcheck: self.0.spellcheck,
                style: self.0.style,
                tab_index: self.0.tab_index,
                title: self.0.title,
                translate: self.0.translate,
                virtual_keyboard_policy: self.0.virtual_keyboard_policy,
                on_click: self.0.on_click,
            })
        }
        #[inline]
        pub fn item_ref<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            item_ref: V,
        ) -> super::Building<super::overwrite::item_ref<TypeDefs, V>> {
            super::Building(super::Data {
                ElementProps: self.0.ElementProps,
                access_key: self.0.access_key,
                auto_capitalize: self.0.auto_capitalize,
                auto_focus: self.0.auto_focus,
                content_editable: self.0.content_editable,
                context_menu: self.0.context_menu,
                dir: self.0.dir,
                draggable: self.0.draggable,
                enter_key_hint: self.0.enter_key_hint,
                hidden: self.0.hidden,
                inert: self.0.inert,
                input_mode: self.0.input_mode,
                is: self.0.is,
                item_id: self.0.item_id,
                item_prop: self.0.item_prop,
                item_ref,
                item_scope: self.0.item_scope,
                item_type: self.0.item_type,
                lang: self.0.lang,
                nonce: self.0.nonce,
                role: self.0.role,
                slot: self.0.slot,
                spellcheck: self.0.spellcheck,
                style: self.0.style,
                tab_index: self.0.tab_index,
                title: self.0.title,
                translate: self.0.translate,
                virtual_keyboard_policy: self.0.virtual_keyboard_policy,
                on_click: self.0.on_click,
            })
        }
        #[inline]
        pub fn item_scope<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            item_scope: V,
        ) -> super::Building<super::overwrite::item_scope<TypeDefs, V>> {
            super::Building(super::Data {
                ElementProps: self.0.ElementProps,
                access_key: self.0.access_key,
                auto_capitalize: self.0.auto_capitalize,
                auto_focus: self.0.auto_focus,
                content_editable: self.0.content_editable,
                context_menu: self.0.context_menu,
                dir: self.0.dir,
                draggable: self.0.draggable,
                enter_key_hint: self.0.enter_key_hint,
                hidden: self.0.hidden,
                inert: self.0.inert,
                input_mode: self.0.input_mode,
                is: self.0.is,
                item_id: self.0.item_id,
                item_prop: self.0.item_prop,
                item_ref: self.0.item_ref,
                item_scope,
                item_type: self.0.item_type,
                lang: self.0.lang,
                nonce: self.0.nonce,
                role: self.0.role,
                slot: self.0.slot,
                spellcheck: self.0.spellcheck,
                style: self.0.style,
                tab_index: self.0.tab_index,
                title: self.0.title,
                translate: self.0.translate,
                virtual_keyboard_policy: self.0.virtual_keyboard_policy,
                on_click: self.0.on_click,
            })
        }
        #[inline]
        pub fn item_type<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            item_type: V,
        ) -> super::Building<super::overwrite::item_type<TypeDefs, V>> {
            super::Building(super::Data {
                ElementProps: self.0.ElementProps,
                access_key: self.0.access_key,
                auto_capitalize: self.0.auto_capitalize,
                auto_focus: self.0.auto_focus,
                content_editable: self.0.content_editable,
                context_menu: self.0.context_menu,
                dir: self.0.dir,
                draggable: self.0.draggable,
                enter_key_hint: self.0.enter_key_hint,
                hidden: self.0.hidden,
                inert: self.0.inert,
                input_mode: self.0.input_mode,
                is: self.0.is,
                item_id: self.0.item_id,
                item_prop: self.0.item_prop,
                item_ref: self.0.item_ref,
                item_scope: self.0.item_scope,
                item_type,
                lang: self.0.lang,
                nonce: self.0.nonce,
                role: self.0.role,
                slot: self.0.slot,
                spellcheck: self.0.spellcheck,
                style: self.0.style,
                tab_index: self.0.tab_index,
                title: self.0.title,
                translate: self.0.translate,
                virtual_keyboard_policy: self.0.virtual_keyboard_policy,
                on_click: self.0.on_click,
            })
        }
        #[inline]
        pub fn lang<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            lang: V,
        ) -> super::Building<super::overwrite::lang<TypeDefs, V>> {
            super::Building(super::Data {
                ElementProps: self.0.ElementProps,
                access_key: self.0.access_key,
                auto_capitalize: self.0.auto_capitalize,
                auto_focus: self.0.auto_focus,
                content_editable: self.0.content_editable,
                context_menu: self.0.context_menu,
                dir: self.0.dir,
                draggable: self.0.draggable,
                enter_key_hint: self.0.enter_key_hint,
                hidden: self.0.hidden,
                inert: self.0.inert,
                input_mode: self.0.input_mode,
                is: self.0.is,
                item_id: self.0.item_id,
                item_prop: self.0.item_prop,
                item_ref: self.0.item_ref,
                item_scope: self.0.item_scope,
                item_type: self.0.item_type,
                lang,
                nonce: self.0.nonce,
                role: self.0.role,
                slot: self.0.slot,
                spellcheck: self.0.spellcheck,
                style: self.0.style,
                tab_index: self.0.tab_index,
                title: self.0.title,
                translate: self.0.translate,
                virtual_keyboard_policy: self.0.virtual_keyboard_policy,
                on_click: self.0.on_click,
            })
        }
        #[inline]
        pub fn nonce<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            nonce: V,
        ) -> super::Building<super::overwrite::nonce<TypeDefs, V>> {
            super::Building(super::Data {
                ElementProps: self.0.ElementProps,
                access_key: self.0.access_key,
                auto_capitalize: self.0.auto_capitalize,
                auto_focus: self.0.auto_focus,
                content_editable: self.0.content_editable,
                context_menu: self.0.context_menu,
                dir: self.0.dir,
                draggable: self.0.draggable,
                enter_key_hint: self.0.enter_key_hint,
                hidden: self.0.hidden,
                inert: self.0.inert,
                input_mode: self.0.input_mode,
                is: self.0.is,
                item_id: self.0.item_id,
                item_prop: self.0.item_prop,
                item_ref: self.0.item_ref,
                item_scope: self.0.item_scope,
                item_type: self.0.item_type,
                lang: self.0.lang,
                nonce,
                role: self.0.role,
                slot: self.0.slot,
                spellcheck: self.0.spellcheck,
                style: self.0.style,
                tab_index: self.0.tab_index,
                title: self.0.title,
                translate: self.0.translate,
                virtual_keyboard_policy: self.0.virtual_keyboard_policy,
                on_click: self.0.on_click,
            })
        }
        #[inline]
        pub fn role<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            role: V,
        ) -> super::Building<super::overwrite::role<TypeDefs, V>> {
            super::Building(super::Data {
                ElementProps: self.0.ElementProps,
                access_key: self.0.access_key,
                auto_capitalize: self.0.auto_capitalize,
                auto_focus: self.0.auto_focus,
                content_editable: self.0.content_editable,
                context_menu: self.0.context_menu,
                dir: self.0.dir,
                draggable: self.0.draggable,
                enter_key_hint: self.0.enter_key_hint,
                hidden: self.0.hidden,
                inert: self.0.inert,
                input_mode: self.0.input_mode,
                is: self.0.is,
                item_id: self.0.item_id,
                item_prop: self.0.item_prop,
                item_ref: self.0.item_ref,
                item_scope: self.0.item_scope,
                item_type: self.0.item_type,
                lang: self.0.lang,
                nonce: self.0.nonce,
                role,
                slot: self.0.slot,
                spellcheck: self.0.spellcheck,
                style: self.0.style,
                tab_index: self.0.tab_index,
                title: self.0.title,
                translate: self.0.translate,
                virtual_keyboard_policy: self.0.virtual_keyboard_policy,
                on_click: self.0.on_click,
            })
        }
        #[inline]
        pub fn slot<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            slot: V,
        ) -> super::Building<super::overwrite::slot<TypeDefs, V>> {
            super::Building(super::Data {
                ElementProps: self.0.ElementProps,
                access_key: self.0.access_key,
                auto_capitalize: self.0.auto_capitalize,
                auto_focus: self.0.auto_focus,
                content_editable: self.0.content_editable,
                context_menu: self.0.context_menu,
                dir: self.0.dir,
                draggable: self.0.draggable,
                enter_key_hint: self.0.enter_key_hint,
                hidden: self.0.hidden,
                inert: self.0.inert,
                input_mode: self.0.input_mode,
                is: self.0.is,
                item_id: self.0.item_id,
                item_prop: self.0.item_prop,
                item_ref: self.0.item_ref,
                item_scope: self.0.item_scope,
                item_type: self.0.item_type,
                lang: self.0.lang,
                nonce: self.0.nonce,
                role: self.0.role,
                slot,
                spellcheck: self.0.spellcheck,
                style: self.0.style,
                tab_index: self.0.tab_index,
                title: self.0.title,
                translate: self.0.translate,
                virtual_keyboard_policy: self.0.virtual_keyboard_policy,
                on_click: self.0.on_click,
            })
        }
        #[inline]
        pub fn spellcheck<V: crate::MaybeUpdateValue<bool>>(
            self,
            spellcheck: V,
        ) -> super::Building<super::overwrite::spellcheck<TypeDefs, V>> {
            super::Building(super::Data {
                ElementProps: self.0.ElementProps,
                access_key: self.0.access_key,
                auto_capitalize: self.0.auto_capitalize,
                auto_focus: self.0.auto_focus,
                content_editable: self.0.content_editable,
                context_menu: self.0.context_menu,
                dir: self.0.dir,
                draggable: self.0.draggable,
                enter_key_hint: self.0.enter_key_hint,
                hidden: self.0.hidden,
                inert: self.0.inert,
                input_mode: self.0.input_mode,
                is: self.0.is,
                item_id: self.0.item_id,
                item_prop: self.0.item_prop,
                item_ref: self.0.item_ref,
                item_scope: self.0.item_scope,
                item_type: self.0.item_type,
                lang: self.0.lang,
                nonce: self.0.nonce,
                role: self.0.role,
                slot: self.0.slot,
                spellcheck,
                style: self.0.style,
                tab_index: self.0.tab_index,
                title: self.0.title,
                translate: self.0.translate,
                virtual_keyboard_policy: self.0.virtual_keyboard_policy,
                on_click: self.0.on_click,
            })
        }
        #[inline]
        pub fn style<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            style: V,
        ) -> super::Building<super::overwrite::style<TypeDefs, V>> {
            super::Building(super::Data {
                ElementProps: self.0.ElementProps,
                access_key: self.0.access_key,
                auto_capitalize: self.0.auto_capitalize,
                auto_focus: self.0.auto_focus,
                content_editable: self.0.content_editable,
                context_menu: self.0.context_menu,
                dir: self.0.dir,
                draggable: self.0.draggable,
                enter_key_hint: self.0.enter_key_hint,
                hidden: self.0.hidden,
                inert: self.0.inert,
                input_mode: self.0.input_mode,
                is: self.0.is,
                item_id: self.0.item_id,
                item_prop: self.0.item_prop,
                item_ref: self.0.item_ref,
                item_scope: self.0.item_scope,
                item_type: self.0.item_type,
                lang: self.0.lang,
                nonce: self.0.nonce,
                role: self.0.role,
                slot: self.0.slot,
                spellcheck: self.0.spellcheck,
                style,
                tab_index: self.0.tab_index,
                title: self.0.title,
                translate: self.0.translate,
                virtual_keyboard_policy: self.0.virtual_keyboard_policy,
                on_click: self.0.on_click,
            })
        }
        #[inline]
        pub fn tab_index<V: crate::MaybeUpdateValue<i32>>(
            self,
            tab_index: V,
        ) -> super::Building<super::overwrite::tab_index<TypeDefs, V>> {
            super::Building(super::Data {
                ElementProps: self.0.ElementProps,
                access_key: self.0.access_key,
                auto_capitalize: self.0.auto_capitalize,
                auto_focus: self.0.auto_focus,
                content_editable: self.0.content_editable,
                context_menu: self.0.context_menu,
                dir: self.0.dir,
                draggable: self.0.draggable,
                enter_key_hint: self.0.enter_key_hint,
                hidden: self.0.hidden,
                inert: self.0.inert,
                input_mode: self.0.input_mode,
                is: self.0.is,
                item_id: self.0.item_id,
                item_prop: self.0.item_prop,
                item_ref: self.0.item_ref,
                item_scope: self.0.item_scope,
                item_type: self.0.item_type,
                lang: self.0.lang,
                nonce: self.0.nonce,
                role: self.0.role,
                slot: self.0.slot,
                spellcheck: self.0.spellcheck,
                style: self.0.style,
                tab_index,
                title: self.0.title,
                translate: self.0.translate,
                virtual_keyboard_policy: self.0.virtual_keyboard_policy,
                on_click: self.0.on_click,
            })
        }
        #[inline]
        pub fn title<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            title: V,
        ) -> super::Building<super::overwrite::title<TypeDefs, V>> {
            super::Building(super::Data {
                ElementProps: self.0.ElementProps,
                access_key: self.0.access_key,
                auto_capitalize: self.0.auto_capitalize,
                auto_focus: self.0.auto_focus,
                content_editable: self.0.content_editable,
                context_menu: self.0.context_menu,
                dir: self.0.dir,
                draggable: self.0.draggable,
                enter_key_hint: self.0.enter_key_hint,
                hidden: self.0.hidden,
                inert: self.0.inert,
                input_mode: self.0.input_mode,
                is: self.0.is,
                item_id: self.0.item_id,
                item_prop: self.0.item_prop,
                item_ref: self.0.item_ref,
                item_scope: self.0.item_scope,
                item_type: self.0.item_type,
                lang: self.0.lang,
                nonce: self.0.nonce,
                role: self.0.role,
                slot: self.0.slot,
                spellcheck: self.0.spellcheck,
                style: self.0.style,
                tab_index: self.0.tab_index,
                title,
                translate: self.0.translate,
                virtual_keyboard_policy: self.0.virtual_keyboard_policy,
                on_click: self.0.on_click,
            })
        }
        #[inline]
        pub fn translate<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            translate: V,
        ) -> super::Building<super::overwrite::translate<TypeDefs, V>> {
            super::Building(super::Data {
                ElementProps: self.0.ElementProps,
                access_key: self.0.access_key,
                auto_capitalize: self.0.auto_capitalize,
                auto_focus: self.0.auto_focus,
                content_editable: self.0.content_editable,
                context_menu: self.0.context_menu,
                dir: self.0.dir,
                draggable: self.0.draggable,
                enter_key_hint: self.0.enter_key_hint,
                hidden: self.0.hidden,
                inert: self.0.inert,
                input_mode: self.0.input_mode,
                is: self.0.is,
                item_id: self.0.item_id,
                item_prop: self.0.item_prop,
                item_ref: self.0.item_ref,
                item_scope: self.0.item_scope,
                item_type: self.0.item_type,
                lang: self.0.lang,
                nonce: self.0.nonce,
                role: self.0.role,
                slot: self.0.slot,
                spellcheck: self.0.spellcheck,
                style: self.0.style,
                tab_index: self.0.tab_index,
                title: self.0.title,
                translate,
                virtual_keyboard_policy: self.0.virtual_keyboard_policy,
                on_click: self.0.on_click,
            })
        }
        #[inline]
        pub fn virtual_keyboard_policy<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            virtual_keyboard_policy: V,
        ) -> super::Building<super::overwrite::virtual_keyboard_policy<TypeDefs, V>> {
            super::Building(super::Data {
                ElementProps: self.0.ElementProps,
                access_key: self.0.access_key,
                auto_capitalize: self.0.auto_capitalize,
                auto_focus: self.0.auto_focus,
                content_editable: self.0.content_editable,
                context_menu: self.0.context_menu,
                dir: self.0.dir,
                draggable: self.0.draggable,
                enter_key_hint: self.0.enter_key_hint,
                hidden: self.0.hidden,
                inert: self.0.inert,
                input_mode: self.0.input_mode,
                is: self.0.is,
                item_id: self.0.item_id,
                item_prop: self.0.item_prop,
                item_ref: self.0.item_ref,
                item_scope: self.0.item_scope,
                item_type: self.0.item_type,
                lang: self.0.lang,
                nonce: self.0.nonce,
                role: self.0.role,
                slot: self.0.slot,
                spellcheck: self.0.spellcheck,
                style: self.0.style,
                tab_index: self.0.tab_index,
                title: self.0.title,
                translate: self.0.translate,
                virtual_keyboard_policy,
                on_click: self.0.on_click,
            })
        }
        #[inline]
        pub fn on_click<V>(
            self,
            on_click: V,
        ) -> super::Building<super::overwrite::on_click<TypeDefs, V>> {
            super::Building(super::Data {
                ElementProps: self.0.ElementProps,
                access_key: self.0.access_key,
                auto_capitalize: self.0.auto_capitalize,
                auto_focus: self.0.auto_focus,
                content_editable: self.0.content_editable,
                context_menu: self.0.context_menu,
                dir: self.0.dir,
                draggable: self.0.draggable,
                enter_key_hint: self.0.enter_key_hint,
                hidden: self.0.hidden,
                inert: self.0.inert,
                input_mode: self.0.input_mode,
                is: self.0.is,
                item_id: self.0.item_id,
                item_prop: self.0.item_prop,
                item_ref: self.0.item_ref,
                item_scope: self.0.item_scope,
                item_type: self.0.item_type,
                lang: self.0.lang,
                nonce: self.0.nonce,
                role: self.0.role,
                slot: self.0.slot,
                spellcheck: self.0.spellcheck,
                style: self.0.style,
                tab_index: self.0.tab_index,
                title: self.0.title,
                translate: self.0.translate,
                virtual_keyboard_policy: self.0.virtual_keyboard_policy,
                on_click,
            })
        }
    }
}
#[cfg(feature = "dom")]
mod impl_update_element {
    #[allow(unused_imports)]
    use super::super::*;
    impl<TypeDefs: ?::core::marker::Sized + super::Types>
        crate::props::UpdateElement<web_sys::HtmlElement> for super::Data<TypeDefs>
    where
        ElementProps::Data<TypeDefs::ElementProps>: crate::props::UpdateElement<web_sys::Element>,
        TypeDefs::on_click: crate::props::UpdateDomEventListener<crate::props::events::Click>,
    {
        type State = super :: render_state :: RenderState < dyn super :: render_state :: RenderStateTypes < ElementProps = < ElementProps :: Data < TypeDefs :: ElementProps , > as crate :: props :: UpdateElement < web_sys :: Element > > :: State , on_click = < TypeDefs :: on_click as crate :: props :: UpdateDomEventListener < crate :: props :: events :: Click , > > :: State , > , > ;
        fn update_element(
            this: Self,
            element: &web_sys::HtmlElement,
            children_ctx: &mut ::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let state = state.pin_project();
            let dom_element: &::web_sys::Element = element.as_ref();
            crate::props::UpdateElement::update_element(
                this.ElementProps,
                element.as_ref(),
                children_ctx,
                state.ElementProps,
            );
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "accesskey";
                < TypeDefs :: access_key as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . access_key , | v | element . set_access_key (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "autocapitalize";
                < TypeDefs :: auto_capitalize as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . auto_capitalize , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "autofocus";
                < TypeDefs :: auto_focus as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (this . auto_focus , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "contextmenu";
                < TypeDefs :: context_menu as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . context_menu , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "dir";
                < TypeDefs :: dir as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . dir , | v | element . set_dir (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "draggable";
                < TypeDefs :: draggable as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (this . draggable , | v | element . set_draggable (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "enterkeyhint";
                < TypeDefs :: enter_key_hint as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . enter_key_hint , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "hidden";
                < TypeDefs :: hidden as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (this . hidden , | v | element . set_hidden (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "inert";
                < TypeDefs :: inert as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (this . inert , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "inputmode";
                < TypeDefs :: input_mode as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . input_mode , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "is";
                < TypeDefs :: is as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . is , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "itemid";
                < TypeDefs :: item_id as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . item_id , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "itemprop";
                < TypeDefs :: item_prop as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . item_prop , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "itemref";
                < TypeDefs :: item_ref as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . item_ref , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "itemscope";
                < TypeDefs :: item_scope as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . item_scope , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "itemtype";
                < TypeDefs :: item_type as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . item_type , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "lang";
                < TypeDefs :: lang as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . lang , | v | element . set_lang (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "nonce";
                < TypeDefs :: nonce as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . nonce , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "role";
                < TypeDefs :: role as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . role , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "slot";
                < TypeDefs :: slot as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . slot , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "spellcheck";
                < TypeDefs :: spellcheck as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (this . spellcheck , | v | element . set_spellcheck (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "style";
                < TypeDefs :: style as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . style , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "tabindex";
                < TypeDefs :: tab_index as :: frender_dom :: props :: MaybeUpdateValue < i32 , > > :: maybe_update_value (this . tab_index , | v | element . set_tab_index (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "title";
                < TypeDefs :: title as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . title , | v | element . set_title (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "translate";
                < TypeDefs :: translate as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . translate , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "virtualkeyboardpolicy";
                <TypeDefs::virtual_keyboard_policy as ::frender_dom::props::MaybeUpdateValueByRef<
                    str,
                >>::maybe_update_value_by_ref(
                    &this.virtual_keyboard_policy,
                    |v| {
                        crate::props::UpdateElementAttribute::update_element_attribute(
                            v,
                            dom_element,
                            ATTR_NAME,
                        )
                    },
                    || dom_element.remove_attribute(ATTR_NAME).unwrap(),
                )
            }
            crate :: props :: UpdateDomEventListener :: < crate :: props :: events :: Click , > :: update_dom_event_listener (this . on_click , element , state . on_click) ;
        }
    }
}
