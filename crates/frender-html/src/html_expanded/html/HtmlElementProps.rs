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
        type access_key: crate::MaybeUpdateValueWithState<str>;
        type auto_capitalize: crate::MaybeUpdateValueWithState<str>;
        type auto_focus: crate::MaybeUpdateValueWithState<bool>;
        type content_editable: crate::props::MaybeInherit<bool>;
        type context_menu: crate::MaybeUpdateValueWithState<str>;
        type dir: crate::MaybeUpdateValueWithState<str>;
        type draggable: crate::MaybeUpdateValueWithState<bool>;
        type enter_key_hint: crate::MaybeUpdateValueWithState<str>;
        type hidden: crate::MaybeUpdateValueWithState<bool>;
        type inert: crate::MaybeUpdateValueWithState<bool>;
        type input_mode: crate::MaybeUpdateValueWithState<str>;
        type is: crate::MaybeUpdateValueWithState<str>;
        type item_id: crate::MaybeUpdateValueWithState<str>;
        type item_prop: crate::MaybeUpdateValueWithState<str>;
        type item_ref: crate::MaybeUpdateValueWithState<str>;
        type item_scope: crate::MaybeUpdateValueWithState<str>;
        type item_type: crate::MaybeUpdateValueWithState<str>;
        type lang: crate::MaybeUpdateValueWithState<str>;
        type nonce: crate::MaybeUpdateValueWithState<str>;
        type role: crate::MaybeUpdateValueWithState<str>;
        type slot: crate::MaybeUpdateValueWithState<str>;
        type spellcheck: crate::MaybeUpdateValueWithState<bool>;
        type style: crate::MaybeUpdateValueWithState<str>;
        type tab_index: crate::MaybeUpdateValueWithState<i32>;
        type title: crate::MaybeUpdateValueWithState<str>;
        type translate: crate::MaybeUpdateValueWithState<str>;
        type virtual_keyboard_policy: crate::MaybeUpdateValueWithState<str>;
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
        type ElementProps: crate::props::IntrinsicComponentPollReactive;
        type access_key;
        type auto_capitalize;
        type auto_focus;
        type context_menu;
        type dir;
        type draggable;
        type enter_key_hint;
        type hidden;
        type inert;
        type input_mode;
        type is;
        type item_id;
        type item_prop;
        type item_ref;
        type item_scope;
        type item_type;
        type lang;
        type nonce;
        type role;
        type slot;
        type spellcheck;
        type style;
        type tab_index;
        type title;
        type translate;
        type virtual_keyboard_policy;
        type on_click: std::any::Any;
    }
    pub struct RenderState<TypeDefs: RenderStateTypes>
    where
        TypeDefs: ?::core::marker::Sized,
    {
        pub ElementProps: TypeDefs::ElementProps,
        pub access_key: TypeDefs::access_key,
        pub auto_capitalize: TypeDefs::auto_capitalize,
        pub auto_focus: TypeDefs::auto_focus,
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
        pub access_key: &'__pin mut (TypeDefs::access_key),
        pub auto_capitalize: &'__pin mut (TypeDefs::auto_capitalize),
        pub auto_focus: &'__pin mut (TypeDefs::auto_focus),
        pub context_menu: &'__pin mut (TypeDefs::context_menu),
        pub dir: &'__pin mut (TypeDefs::dir),
        pub draggable: &'__pin mut (TypeDefs::draggable),
        pub enter_key_hint: &'__pin mut (TypeDefs::enter_key_hint),
        pub hidden: &'__pin mut (TypeDefs::hidden),
        pub inert: &'__pin mut (TypeDefs::inert),
        pub input_mode: &'__pin mut (TypeDefs::input_mode),
        pub is: &'__pin mut (TypeDefs::is),
        pub item_id: &'__pin mut (TypeDefs::item_id),
        pub item_prop: &'__pin mut (TypeDefs::item_prop),
        pub item_ref: &'__pin mut (TypeDefs::item_ref),
        pub item_scope: &'__pin mut (TypeDefs::item_scope),
        pub item_type: &'__pin mut (TypeDefs::item_type),
        pub lang: &'__pin mut (TypeDefs::lang),
        pub nonce: &'__pin mut (TypeDefs::nonce),
        pub role: &'__pin mut (TypeDefs::role),
        pub slot: &'__pin mut (TypeDefs::slot),
        pub spellcheck: &'__pin mut (TypeDefs::spellcheck),
        pub style: &'__pin mut (TypeDefs::style),
        pub tab_index: &'__pin mut (TypeDefs::tab_index),
        pub title: &'__pin mut (TypeDefs::title),
        pub translate: &'__pin mut (TypeDefs::translate),
        pub virtual_keyboard_policy: &'__pin mut (TypeDefs::virtual_keyboard_policy),
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
            pub access_key: &'__pin (TypeDefs::access_key),
            pub auto_capitalize: &'__pin (TypeDefs::auto_capitalize),
            pub auto_focus: &'__pin (TypeDefs::auto_focus),
            pub context_menu: &'__pin (TypeDefs::context_menu),
            pub dir: &'__pin (TypeDefs::dir),
            pub draggable: &'__pin (TypeDefs::draggable),
            pub enter_key_hint: &'__pin (TypeDefs::enter_key_hint),
            pub hidden: &'__pin (TypeDefs::hidden),
            pub inert: &'__pin (TypeDefs::inert),
            pub input_mode: &'__pin (TypeDefs::input_mode),
            pub is: &'__pin (TypeDefs::is),
            pub item_id: &'__pin (TypeDefs::item_id),
            pub item_prop: &'__pin (TypeDefs::item_prop),
            pub item_ref: &'__pin (TypeDefs::item_ref),
            pub item_scope: &'__pin (TypeDefs::item_scope),
            pub item_type: &'__pin (TypeDefs::item_type),
            pub lang: &'__pin (TypeDefs::lang),
            pub nonce: &'__pin (TypeDefs::nonce),
            pub role: &'__pin (TypeDefs::role),
            pub slot: &'__pin (TypeDefs::slot),
            pub spellcheck: &'__pin (TypeDefs::spellcheck),
            pub style: &'__pin (TypeDefs::style),
            pub tab_index: &'__pin (TypeDefs::tab_index),
            pub title: &'__pin (TypeDefs::title),
            pub translate: &'__pin (TypeDefs::translate),
            pub virtual_keyboard_policy: &'__pin (TypeDefs::virtual_keyboard_policy),
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
                        access_key,
                        auto_capitalize,
                        auto_focus,
                        context_menu,
                        dir,
                        draggable,
                        enter_key_hint,
                        hidden,
                        inert,
                        input_mode,
                        is,
                        item_id,
                        item_prop,
                        item_ref,
                        item_scope,
                        item_type,
                        lang,
                        nonce,
                        role,
                        slot,
                        spellcheck,
                        style,
                        tab_index,
                        title,
                        translate,
                        virtual_keyboard_policy,
                        on_click,
                    } = self.get_unchecked_mut();
                    RenderStateProj {
                        ElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            ElementProps,
                        ),
                        access_key: access_key,
                        auto_capitalize: auto_capitalize,
                        auto_focus: auto_focus,
                        context_menu: context_menu,
                        dir: dir,
                        draggable: draggable,
                        enter_key_hint: enter_key_hint,
                        hidden: hidden,
                        inert: inert,
                        input_mode: input_mode,
                        is: is,
                        item_id: item_id,
                        item_prop: item_prop,
                        item_ref: item_ref,
                        item_scope: item_scope,
                        item_type: item_type,
                        lang: lang,
                        nonce: nonce,
                        role: role,
                        slot: slot,
                        spellcheck: spellcheck,
                        style: style,
                        tab_index: tab_index,
                        title: title,
                        translate: translate,
                        virtual_keyboard_policy: virtual_keyboard_policy,
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
                        access_key,
                        auto_capitalize,
                        auto_focus,
                        context_menu,
                        dir,
                        draggable,
                        enter_key_hint,
                        hidden,
                        inert,
                        input_mode,
                        is,
                        item_id,
                        item_prop,
                        item_ref,
                        item_scope,
                        item_type,
                        lang,
                        nonce,
                        role,
                        slot,
                        spellcheck,
                        style,
                        tab_index,
                        title,
                        translate,
                        virtual_keyboard_policy,
                        on_click,
                    } = self.get_ref();
                    ProjectionRef {
                        ElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            ElementProps,
                        ),
                        access_key: access_key,
                        auto_capitalize: auto_capitalize,
                        auto_focus: auto_focus,
                        context_menu: context_menu,
                        dir: dir,
                        draggable: draggable,
                        enter_key_hint: enter_key_hint,
                        hidden: hidden,
                        inert: inert,
                        input_mode: input_mode,
                        is: is,
                        item_id: item_id,
                        item_prop: item_prop,
                        item_ref: item_ref,
                        item_scope: item_scope,
                        item_type: item_type,
                        lang: lang,
                        nonce: nonce,
                        role: role,
                        slot: slot,
                        spellcheck: spellcheck,
                        style: style,
                        tab_index: tab_index,
                        title: title,
                        translate: translate,
                        virtual_keyboard_policy: virtual_keyboard_policy,
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
            access_key: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::access_key>,
            auto_capitalize: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::auto_capitalize>,
            auto_focus: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::auto_focus>,
            context_menu: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::context_menu>,
            dir: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::dir>,
            draggable: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::draggable>,
            enter_key_hint: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::enter_key_hint>,
            hidden: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::hidden>,
            inert: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::inert>,
            input_mode: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::input_mode>,
            is: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::is>,
            item_id: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::item_id>,
            item_prop: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::item_prop>,
            item_ref: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::item_ref>,
            item_scope: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::item_scope>,
            item_type: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::item_type>,
            lang: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::lang>,
            nonce: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::nonce>,
            role: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::role>,
            slot: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::slot>,
            spellcheck: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::spellcheck>,
            style: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::style>,
            tab_index: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::tab_index>,
            title: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::title>,
            translate: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::translate>,
            virtual_keyboard_policy:
                ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::virtual_keyboard_policy>,
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
            let _ = &this.access_key;
            let _ = &this.auto_capitalize;
            let _ = &this.auto_focus;
            let _ = &this.context_menu;
            let _ = &this.dir;
            let _ = &this.draggable;
            let _ = &this.enter_key_hint;
            let _ = &this.hidden;
            let _ = &this.inert;
            let _ = &this.input_mode;
            let _ = &this.is;
            let _ = &this.item_id;
            let _ = &this.item_prop;
            let _ = &this.item_ref;
            let _ = &this.item_scope;
            let _ = &this.item_type;
            let _ = &this.lang;
            let _ = &this.nonce;
            let _ = &this.role;
            let _ = &this.slot;
            let _ = &this.spellcheck;
            let _ = &this.style;
            let _ = &this.tab_index;
            let _ = &this.title;
            let _ = &this.translate;
            let _ = &this.virtual_keyboard_policy;
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
        ///See [`ElementProps::children`]
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
        ///See [`ElementProps::class`]
        #[inline]
        pub fn class<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`ElementProps::id`]
        #[inline]
        pub fn id<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`ElementProps::part`]
        #[inline]
        pub fn part<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn access_key<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn auto_capitalize<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn auto_focus<V: crate::MaybeUpdateValueWithState<bool>>(
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
        pub fn context_menu<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn dir<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn draggable<V: crate::MaybeUpdateValueWithState<bool>>(
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
        pub fn enter_key_hint<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn hidden<V: crate::MaybeUpdateValueWithState<bool>>(
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
        pub fn inert<V: crate::MaybeUpdateValueWithState<bool>>(
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
        pub fn input_mode<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn is<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn item_id<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn item_prop<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn item_ref<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn item_scope<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn item_type<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn lang<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn nonce<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn role<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn slot<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn spellcheck<V: crate::MaybeUpdateValueWithState<bool>>(
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
        pub fn style<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn tab_index<V: crate::MaybeUpdateValueWithState<i32>>(
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
        pub fn title<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn translate<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn virtual_keyboard_policy<V: crate::MaybeUpdateValueWithState<str>>(
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
        type State = super::render_state::RenderState<
            dyn super::render_state::RenderStateTypes<
                ElementProps = <ElementProps::Data<
                    TypeDefs::ElementProps,
                > as crate::props::UpdateElement<web_sys::Element>>::State,
                access_key = <TypeDefs::access_key as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                auto_capitalize = <TypeDefs::auto_capitalize as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                auto_focus = <TypeDefs::auto_focus as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::State,
                context_menu = <TypeDefs::context_menu as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                dir = <TypeDefs::dir as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                draggable = <TypeDefs::draggable as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::State,
                enter_key_hint = <TypeDefs::enter_key_hint as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                hidden = <TypeDefs::hidden as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::State,
                inert = <TypeDefs::inert as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::State,
                input_mode = <TypeDefs::input_mode as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                is = <TypeDefs::is as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                item_id = <TypeDefs::item_id as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                item_prop = <TypeDefs::item_prop as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                item_ref = <TypeDefs::item_ref as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                item_scope = <TypeDefs::item_scope as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                item_type = <TypeDefs::item_type as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                lang = <TypeDefs::lang as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                nonce = <TypeDefs::nonce as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                role = <TypeDefs::role as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                slot = <TypeDefs::slot as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                spellcheck = <TypeDefs::spellcheck as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::State,
                style = <TypeDefs::style as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                tab_index = <TypeDefs::tab_index as ::frender_dom::props::MaybeUpdateValueWithState<
                    i32,
                >>::State,
                title = <TypeDefs::title as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                translate = <TypeDefs::translate as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                virtual_keyboard_policy = <TypeDefs::virtual_keyboard_policy as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                on_click = <TypeDefs::on_click as crate::props::UpdateDomEventListener<
                    crate::props::events::Click,
                >>::State,
            >,
        >;
        fn initialize_state(
            this: Self,
            element: &web_sys::HtmlElement,
            children_ctx: &mut ::frender_dom::Dom,
        ) -> Self::State {
            let dom_element: &::web_sys::Element = element.as_ref();
            super::render_state::RenderState {
                ElementProps: <ElementProps::Data<
                    TypeDefs::ElementProps,
                > as crate::props::UpdateElement<
                    web_sys::Element,
                >>::initialize_state(this.ElementProps, element, children_ctx),
                access_key: <TypeDefs::access_key as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.access_key,
                    |v| element.set_access_key(v),
                    || dom_element.remove_attribute("accesskey").unwrap(),
                ),
                auto_capitalize: <TypeDefs::auto_capitalize as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.auto_capitalize,
                    |v| crate::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "autocapitalize",
                    ),
                    || dom_element.remove_attribute("autocapitalize").unwrap(),
                ),
                auto_focus: <TypeDefs::auto_focus as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.auto_focus,
                    |v| crate::props::UpdateElementAttribute::update_element_attribute(
                        *v,
                        dom_element,
                        "autofocus",
                    ),
                    || dom_element.remove_attribute("autofocus").unwrap(),
                ),
                context_menu: <TypeDefs::context_menu as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.context_menu,
                    |v| crate::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "contextmenu",
                    ),
                    || dom_element.remove_attribute("contextmenu").unwrap(),
                ),
                dir: <TypeDefs::dir as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.dir,
                    |v| element.set_dir(v),
                    || dom_element.remove_attribute("dir").unwrap(),
                ),
                draggable: <TypeDefs::draggable as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.draggable,
                    |v| element.set_draggable(*v),
                    || dom_element.remove_attribute("draggable").unwrap(),
                ),
                enter_key_hint: <TypeDefs::enter_key_hint as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.enter_key_hint,
                    |v| crate::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "enterkeyhint",
                    ),
                    || dom_element.remove_attribute("enterkeyhint").unwrap(),
                ),
                hidden: <TypeDefs::hidden as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.hidden,
                    |v| element.set_hidden(*v),
                    || dom_element.remove_attribute("hidden").unwrap(),
                ),
                inert: <TypeDefs::inert as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.inert,
                    |v| crate::props::UpdateElementAttribute::update_element_attribute(
                        *v,
                        dom_element,
                        "inert",
                    ),
                    || dom_element.remove_attribute("inert").unwrap(),
                ),
                input_mode: <TypeDefs::input_mode as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.input_mode,
                    |v| crate::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "inputmode",
                    ),
                    || dom_element.remove_attribute("inputmode").unwrap(),
                ),
                is: <TypeDefs::is as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.is,
                    |v| crate::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "is",
                    ),
                    || dom_element.remove_attribute("is").unwrap(),
                ),
                item_id: <TypeDefs::item_id as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.item_id,
                    |v| crate::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "itemid",
                    ),
                    || dom_element.remove_attribute("itemid").unwrap(),
                ),
                item_prop: <TypeDefs::item_prop as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.item_prop,
                    |v| crate::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "itemprop",
                    ),
                    || dom_element.remove_attribute("itemprop").unwrap(),
                ),
                item_ref: <TypeDefs::item_ref as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.item_ref,
                    |v| crate::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "itemref",
                    ),
                    || dom_element.remove_attribute("itemref").unwrap(),
                ),
                item_scope: <TypeDefs::item_scope as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.item_scope,
                    |v| crate::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "itemscope",
                    ),
                    || dom_element.remove_attribute("itemscope").unwrap(),
                ),
                item_type: <TypeDefs::item_type as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.item_type,
                    |v| crate::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "itemtype",
                    ),
                    || dom_element.remove_attribute("itemtype").unwrap(),
                ),
                lang: <TypeDefs::lang as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.lang,
                    |v| element.set_lang(v),
                    || dom_element.remove_attribute("lang").unwrap(),
                ),
                nonce: <TypeDefs::nonce as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.nonce,
                    |v| crate::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "nonce",
                    ),
                    || dom_element.remove_attribute("nonce").unwrap(),
                ),
                role: <TypeDefs::role as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.role,
                    |v| crate::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "role",
                    ),
                    || dom_element.remove_attribute("role").unwrap(),
                ),
                slot: <TypeDefs::slot as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.slot,
                    |v| crate::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "slot",
                    ),
                    || dom_element.remove_attribute("slot").unwrap(),
                ),
                spellcheck: <TypeDefs::spellcheck as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.spellcheck,
                    |v| element.set_spellcheck(*v),
                    || dom_element.remove_attribute("spellcheck").unwrap(),
                ),
                style: <TypeDefs::style as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.style,
                    |v| crate::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "style",
                    ),
                    || dom_element.remove_attribute("style").unwrap(),
                ),
                tab_index: <TypeDefs::tab_index as ::frender_dom::props::MaybeUpdateValueWithState<
                    i32,
                >>::initialize_state_and_update(
                    this.tab_index,
                    |v| element.set_tab_index(*v),
                    || dom_element.remove_attribute("tabindex").unwrap(),
                ),
                title: <TypeDefs::title as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.title,
                    |v| element.set_title(v),
                    || dom_element.remove_attribute("title").unwrap(),
                ),
                translate: <TypeDefs::translate as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.translate,
                    |v| crate::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "translate",
                    ),
                    || dom_element.remove_attribute("translate").unwrap(),
                ),
                virtual_keyboard_policy: <TypeDefs::virtual_keyboard_policy as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.virtual_keyboard_policy,
                    |v| crate::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "virtualkeyboardpolicy",
                    ),
                    || { dom_element.remove_attribute("virtualkeyboardpolicy").unwrap() },
                ),
                on_click: crate::props::UpdateDomEventListener::<
                    crate::props::events::Click,
                >::initialize_dom_event_listener_state(this.on_click, element),
            }
        }
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
            <TypeDefs::access_key as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.access_key,
                state.access_key,
                |v| element.set_access_key(v),
                || dom_element.remove_attribute("accesskey").unwrap(),
            );
            <TypeDefs::auto_capitalize as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.auto_capitalize,
                state.auto_capitalize,
                |v| crate::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "autocapitalize",
                ),
                || dom_element.remove_attribute("autocapitalize").unwrap(),
            );
            <TypeDefs::auto_focus as ::frender_dom::props::MaybeUpdateValueWithState<
                bool,
            >>::maybe_update_value_with_state(
                this.auto_focus,
                state.auto_focus,
                |v| crate::props::UpdateElementAttribute::update_element_attribute(
                    *v,
                    dom_element,
                    "autofocus",
                ),
                || dom_element.remove_attribute("autofocus").unwrap(),
            );
            <TypeDefs::context_menu as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.context_menu,
                state.context_menu,
                |v| crate::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "contextmenu",
                ),
                || dom_element.remove_attribute("contextmenu").unwrap(),
            );
            <TypeDefs::dir as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.dir,
                state.dir,
                |v| element.set_dir(v),
                || dom_element.remove_attribute("dir").unwrap(),
            );
            <TypeDefs::draggable as ::frender_dom::props::MaybeUpdateValueWithState<
                bool,
            >>::maybe_update_value_with_state(
                this.draggable,
                state.draggable,
                |v| element.set_draggable(*v),
                || dom_element.remove_attribute("draggable").unwrap(),
            );
            <TypeDefs::enter_key_hint as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.enter_key_hint,
                state.enter_key_hint,
                |v| crate::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "enterkeyhint",
                ),
                || dom_element.remove_attribute("enterkeyhint").unwrap(),
            );
            <TypeDefs::hidden as ::frender_dom::props::MaybeUpdateValueWithState<
                bool,
            >>::maybe_update_value_with_state(
                this.hidden,
                state.hidden,
                |v| element.set_hidden(*v),
                || dom_element.remove_attribute("hidden").unwrap(),
            );
            <TypeDefs::inert as ::frender_dom::props::MaybeUpdateValueWithState<
                bool,
            >>::maybe_update_value_with_state(
                this.inert,
                state.inert,
                |v| crate::props::UpdateElementAttribute::update_element_attribute(
                    *v,
                    dom_element,
                    "inert",
                ),
                || dom_element.remove_attribute("inert").unwrap(),
            );
            <TypeDefs::input_mode as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.input_mode,
                state.input_mode,
                |v| crate::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "inputmode",
                ),
                || dom_element.remove_attribute("inputmode").unwrap(),
            );
            <TypeDefs::is as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.is,
                state.is,
                |v| crate::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "is",
                ),
                || dom_element.remove_attribute("is").unwrap(),
            );
            <TypeDefs::item_id as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.item_id,
                state.item_id,
                |v| crate::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "itemid",
                ),
                || dom_element.remove_attribute("itemid").unwrap(),
            );
            <TypeDefs::item_prop as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.item_prop,
                state.item_prop,
                |v| crate::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "itemprop",
                ),
                || dom_element.remove_attribute("itemprop").unwrap(),
            );
            <TypeDefs::item_ref as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.item_ref,
                state.item_ref,
                |v| crate::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "itemref",
                ),
                || dom_element.remove_attribute("itemref").unwrap(),
            );
            <TypeDefs::item_scope as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.item_scope,
                state.item_scope,
                |v| crate::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "itemscope",
                ),
                || dom_element.remove_attribute("itemscope").unwrap(),
            );
            <TypeDefs::item_type as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.item_type,
                state.item_type,
                |v| crate::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "itemtype",
                ),
                || dom_element.remove_attribute("itemtype").unwrap(),
            );
            <TypeDefs::lang as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.lang,
                state.lang,
                |v| element.set_lang(v),
                || dom_element.remove_attribute("lang").unwrap(),
            );
            <TypeDefs::nonce as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.nonce,
                state.nonce,
                |v| crate::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "nonce",
                ),
                || dom_element.remove_attribute("nonce").unwrap(),
            );
            <TypeDefs::role as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.role,
                state.role,
                |v| crate::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "role",
                ),
                || dom_element.remove_attribute("role").unwrap(),
            );
            <TypeDefs::slot as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.slot,
                state.slot,
                |v| crate::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "slot",
                ),
                || dom_element.remove_attribute("slot").unwrap(),
            );
            <TypeDefs::spellcheck as ::frender_dom::props::MaybeUpdateValueWithState<
                bool,
            >>::maybe_update_value_with_state(
                this.spellcheck,
                state.spellcheck,
                |v| element.set_spellcheck(*v),
                || dom_element.remove_attribute("spellcheck").unwrap(),
            );
            <TypeDefs::style as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.style,
                state.style,
                |v| crate::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "style",
                ),
                || dom_element.remove_attribute("style").unwrap(),
            );
            <TypeDefs::tab_index as ::frender_dom::props::MaybeUpdateValueWithState<
                i32,
            >>::maybe_update_value_with_state(
                this.tab_index,
                state.tab_index,
                |v| element.set_tab_index(*v),
                || dom_element.remove_attribute("tabindex").unwrap(),
            );
            <TypeDefs::title as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.title,
                state.title,
                |v| element.set_title(v),
                || dom_element.remove_attribute("title").unwrap(),
            );
            <TypeDefs::translate as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.translate,
                state.translate,
                |v| crate::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "translate",
                ),
                || dom_element.remove_attribute("translate").unwrap(),
            );
            <TypeDefs::virtual_keyboard_policy as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.virtual_keyboard_policy,
                state.virtual_keyboard_policy,
                |v| crate::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "virtualkeyboardpolicy",
                ),
                || dom_element.remove_attribute("virtualkeyboardpolicy").unwrap(),
            );
            crate::props::UpdateDomEventListener::<
                crate::props::events::Click,
            >::update_dom_event_listener(this.on_click, element, state.on_click);
        }
    }
}
