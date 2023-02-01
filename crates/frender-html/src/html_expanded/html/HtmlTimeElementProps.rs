#[allow(non_snake_case)]
pub fn HtmlTimeElementProps() -> Building<TypesInitial> {
    #[allow(unused_imports)]
    use super::*;
    self::Building(self::Data {
        children: (),
        class: (),
        id: (),
        part: (),
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
        date_time: (),
    })
}
pub mod prelude {}
pub mod overwrite {
    #![allow(non_camel_case_types)]
    pub type children<TypeDefs, Value> = dyn super::Types<
        children = Value,
        class = <TypeDefs as super::Types>::class,
        id = <TypeDefs as super::Types>::id,
        part = <TypeDefs as super::Types>::part,
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
        date_time = <TypeDefs as super::Types>::date_time,
    >;
    pub type class<TypeDefs, Value> = dyn super::Types<
        children = <TypeDefs as super::Types>::children,
        class = Value,
        id = <TypeDefs as super::Types>::id,
        part = <TypeDefs as super::Types>::part,
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
        date_time = <TypeDefs as super::Types>::date_time,
    >;
    pub type id<TypeDefs, Value> = dyn super::Types<
        children = <TypeDefs as super::Types>::children,
        class = <TypeDefs as super::Types>::class,
        id = Value,
        part = <TypeDefs as super::Types>::part,
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
        date_time = <TypeDefs as super::Types>::date_time,
    >;
    pub type part<TypeDefs, Value> = dyn super::Types<
        children = <TypeDefs as super::Types>::children,
        class = <TypeDefs as super::Types>::class,
        id = <TypeDefs as super::Types>::id,
        part = Value,
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
        date_time = <TypeDefs as super::Types>::date_time,
    >;
    pub type access_key<TypeDefs, Value> = dyn super::Types<
        children = <TypeDefs as super::Types>::children,
        class = <TypeDefs as super::Types>::class,
        id = <TypeDefs as super::Types>::id,
        part = <TypeDefs as super::Types>::part,
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
        date_time = <TypeDefs as super::Types>::date_time,
    >;
    pub type auto_capitalize<TypeDefs, Value> = dyn super::Types<
        children = <TypeDefs as super::Types>::children,
        class = <TypeDefs as super::Types>::class,
        id = <TypeDefs as super::Types>::id,
        part = <TypeDefs as super::Types>::part,
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
        date_time = <TypeDefs as super::Types>::date_time,
    >;
    pub type auto_focus<TypeDefs, Value> = dyn super::Types<
        children = <TypeDefs as super::Types>::children,
        class = <TypeDefs as super::Types>::class,
        id = <TypeDefs as super::Types>::id,
        part = <TypeDefs as super::Types>::part,
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
        date_time = <TypeDefs as super::Types>::date_time,
    >;
    pub type content_editable<TypeDefs, Value> = dyn super::Types<
        children = <TypeDefs as super::Types>::children,
        class = <TypeDefs as super::Types>::class,
        id = <TypeDefs as super::Types>::id,
        part = <TypeDefs as super::Types>::part,
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
        date_time = <TypeDefs as super::Types>::date_time,
    >;
    pub type context_menu<TypeDefs, Value> = dyn super::Types<
        children = <TypeDefs as super::Types>::children,
        class = <TypeDefs as super::Types>::class,
        id = <TypeDefs as super::Types>::id,
        part = <TypeDefs as super::Types>::part,
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
        date_time = <TypeDefs as super::Types>::date_time,
    >;
    pub type dir<TypeDefs, Value> = dyn super::Types<
        children = <TypeDefs as super::Types>::children,
        class = <TypeDefs as super::Types>::class,
        id = <TypeDefs as super::Types>::id,
        part = <TypeDefs as super::Types>::part,
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
        date_time = <TypeDefs as super::Types>::date_time,
    >;
    pub type draggable<TypeDefs, Value> = dyn super::Types<
        children = <TypeDefs as super::Types>::children,
        class = <TypeDefs as super::Types>::class,
        id = <TypeDefs as super::Types>::id,
        part = <TypeDefs as super::Types>::part,
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
        date_time = <TypeDefs as super::Types>::date_time,
    >;
    pub type enter_key_hint<TypeDefs, Value> = dyn super::Types<
        children = <TypeDefs as super::Types>::children,
        class = <TypeDefs as super::Types>::class,
        id = <TypeDefs as super::Types>::id,
        part = <TypeDefs as super::Types>::part,
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
        date_time = <TypeDefs as super::Types>::date_time,
    >;
    pub type hidden<TypeDefs, Value> = dyn super::Types<
        children = <TypeDefs as super::Types>::children,
        class = <TypeDefs as super::Types>::class,
        id = <TypeDefs as super::Types>::id,
        part = <TypeDefs as super::Types>::part,
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
        date_time = <TypeDefs as super::Types>::date_time,
    >;
    pub type inert<TypeDefs, Value> = dyn super::Types<
        children = <TypeDefs as super::Types>::children,
        class = <TypeDefs as super::Types>::class,
        id = <TypeDefs as super::Types>::id,
        part = <TypeDefs as super::Types>::part,
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
        date_time = <TypeDefs as super::Types>::date_time,
    >;
    pub type input_mode<TypeDefs, Value> = dyn super::Types<
        children = <TypeDefs as super::Types>::children,
        class = <TypeDefs as super::Types>::class,
        id = <TypeDefs as super::Types>::id,
        part = <TypeDefs as super::Types>::part,
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
        date_time = <TypeDefs as super::Types>::date_time,
    >;
    pub type is<TypeDefs, Value> = dyn super::Types<
        children = <TypeDefs as super::Types>::children,
        class = <TypeDefs as super::Types>::class,
        id = <TypeDefs as super::Types>::id,
        part = <TypeDefs as super::Types>::part,
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
        date_time = <TypeDefs as super::Types>::date_time,
    >;
    pub type item_id<TypeDefs, Value> = dyn super::Types<
        children = <TypeDefs as super::Types>::children,
        class = <TypeDefs as super::Types>::class,
        id = <TypeDefs as super::Types>::id,
        part = <TypeDefs as super::Types>::part,
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
        date_time = <TypeDefs as super::Types>::date_time,
    >;
    pub type item_prop<TypeDefs, Value> = dyn super::Types<
        children = <TypeDefs as super::Types>::children,
        class = <TypeDefs as super::Types>::class,
        id = <TypeDefs as super::Types>::id,
        part = <TypeDefs as super::Types>::part,
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
        date_time = <TypeDefs as super::Types>::date_time,
    >;
    pub type item_ref<TypeDefs, Value> = dyn super::Types<
        children = <TypeDefs as super::Types>::children,
        class = <TypeDefs as super::Types>::class,
        id = <TypeDefs as super::Types>::id,
        part = <TypeDefs as super::Types>::part,
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
        date_time = <TypeDefs as super::Types>::date_time,
    >;
    pub type item_scope<TypeDefs, Value> = dyn super::Types<
        children = <TypeDefs as super::Types>::children,
        class = <TypeDefs as super::Types>::class,
        id = <TypeDefs as super::Types>::id,
        part = <TypeDefs as super::Types>::part,
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
        date_time = <TypeDefs as super::Types>::date_time,
    >;
    pub type item_type<TypeDefs, Value> = dyn super::Types<
        children = <TypeDefs as super::Types>::children,
        class = <TypeDefs as super::Types>::class,
        id = <TypeDefs as super::Types>::id,
        part = <TypeDefs as super::Types>::part,
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
        date_time = <TypeDefs as super::Types>::date_time,
    >;
    pub type lang<TypeDefs, Value> = dyn super::Types<
        children = <TypeDefs as super::Types>::children,
        class = <TypeDefs as super::Types>::class,
        id = <TypeDefs as super::Types>::id,
        part = <TypeDefs as super::Types>::part,
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
        date_time = <TypeDefs as super::Types>::date_time,
    >;
    pub type nonce<TypeDefs, Value> = dyn super::Types<
        children = <TypeDefs as super::Types>::children,
        class = <TypeDefs as super::Types>::class,
        id = <TypeDefs as super::Types>::id,
        part = <TypeDefs as super::Types>::part,
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
        date_time = <TypeDefs as super::Types>::date_time,
    >;
    pub type role<TypeDefs, Value> = dyn super::Types<
        children = <TypeDefs as super::Types>::children,
        class = <TypeDefs as super::Types>::class,
        id = <TypeDefs as super::Types>::id,
        part = <TypeDefs as super::Types>::part,
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
        date_time = <TypeDefs as super::Types>::date_time,
    >;
    pub type slot<TypeDefs, Value> = dyn super::Types<
        children = <TypeDefs as super::Types>::children,
        class = <TypeDefs as super::Types>::class,
        id = <TypeDefs as super::Types>::id,
        part = <TypeDefs as super::Types>::part,
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
        date_time = <TypeDefs as super::Types>::date_time,
    >;
    pub type spellcheck<TypeDefs, Value> = dyn super::Types<
        children = <TypeDefs as super::Types>::children,
        class = <TypeDefs as super::Types>::class,
        id = <TypeDefs as super::Types>::id,
        part = <TypeDefs as super::Types>::part,
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
        date_time = <TypeDefs as super::Types>::date_time,
    >;
    pub type style<TypeDefs, Value> = dyn super::Types<
        children = <TypeDefs as super::Types>::children,
        class = <TypeDefs as super::Types>::class,
        id = <TypeDefs as super::Types>::id,
        part = <TypeDefs as super::Types>::part,
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
        date_time = <TypeDefs as super::Types>::date_time,
    >;
    pub type tab_index<TypeDefs, Value> = dyn super::Types<
        children = <TypeDefs as super::Types>::children,
        class = <TypeDefs as super::Types>::class,
        id = <TypeDefs as super::Types>::id,
        part = <TypeDefs as super::Types>::part,
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
        date_time = <TypeDefs as super::Types>::date_time,
    >;
    pub type title<TypeDefs, Value> = dyn super::Types<
        children = <TypeDefs as super::Types>::children,
        class = <TypeDefs as super::Types>::class,
        id = <TypeDefs as super::Types>::id,
        part = <TypeDefs as super::Types>::part,
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
        date_time = <TypeDefs as super::Types>::date_time,
    >;
    pub type translate<TypeDefs, Value> = dyn super::Types<
        children = <TypeDefs as super::Types>::children,
        class = <TypeDefs as super::Types>::class,
        id = <TypeDefs as super::Types>::id,
        part = <TypeDefs as super::Types>::part,
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
        date_time = <TypeDefs as super::Types>::date_time,
    >;
    pub type virtual_keyboard_policy<TypeDefs, Value> = dyn super::Types<
        children = <TypeDefs as super::Types>::children,
        class = <TypeDefs as super::Types>::class,
        id = <TypeDefs as super::Types>::id,
        part = <TypeDefs as super::Types>::part,
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
        date_time = <TypeDefs as super::Types>::date_time,
    >;
    pub type on_click<TypeDefs, Value> = dyn super::Types<
        children = <TypeDefs as super::Types>::children,
        class = <TypeDefs as super::Types>::class,
        id = <TypeDefs as super::Types>::id,
        part = <TypeDefs as super::Types>::part,
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
        date_time = <TypeDefs as super::Types>::date_time,
    >;
    pub type date_time<TypeDefs, Value> = dyn super::Types<
        children = <TypeDefs as super::Types>::children,
        class = <TypeDefs as super::Types>::class,
        id = <TypeDefs as super::Types>::id,
        part = <TypeDefs as super::Types>::part,
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
        date_time = Value,
    >;
}
mod trait_types {
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub trait Types {
        type children;
        type class: ::frender_dom::props::MaybeUpdateValue<str>;
        type id: ::frender_dom::props::MaybeUpdateValue<str>;
        type part: ::frender_dom::props::MaybeUpdateValue<str>;
        type access_key: ::frender_dom::props::MaybeUpdateValue<str>;
        type auto_capitalize: ::frender_dom::props::MaybeUpdateValue<str>;
        type auto_focus: ::frender_dom::props::MaybeUpdateValue<bool>;
        type content_editable: crate::props::MaybeInherit<bool>;
        type context_menu: ::frender_dom::props::MaybeUpdateValue<str>;
        type dir: ::frender_dom::props::MaybeUpdateValue<str>;
        type draggable: ::frender_dom::props::MaybeUpdateValue<bool>;
        type enter_key_hint: ::frender_dom::props::MaybeUpdateValue<str>;
        type hidden: ::frender_dom::props::MaybeUpdateValue<bool>;
        type inert: ::frender_dom::props::MaybeUpdateValue<bool>;
        type input_mode: ::frender_dom::props::MaybeUpdateValue<str>;
        type is: ::frender_dom::props::MaybeUpdateValue<str>;
        type item_id: ::frender_dom::props::MaybeUpdateValue<str>;
        type item_prop: ::frender_dom::props::MaybeUpdateValue<str>;
        type item_ref: ::frender_dom::props::MaybeUpdateValue<str>;
        type item_scope: ::frender_dom::props::MaybeUpdateValue<str>;
        type item_type: ::frender_dom::props::MaybeUpdateValue<str>;
        type lang: ::frender_dom::props::MaybeUpdateValue<str>;
        type nonce: ::frender_dom::props::MaybeUpdateValue<str>;
        type role: ::frender_dom::props::MaybeUpdateValue<str>;
        type slot: ::frender_dom::props::MaybeUpdateValue<str>;
        type spellcheck: ::frender_dom::props::MaybeUpdateValue<bool>;
        type style: ::frender_dom::props::MaybeUpdateValue<str>;
        type tab_index: ::frender_dom::props::MaybeUpdateValue<i32>;
        type title: ::frender_dom::props::MaybeUpdateValue<str>;
        type translate: ::frender_dom::props::MaybeUpdateValue<str>;
        type virtual_keyboard_policy: ::frender_dom::props::MaybeUpdateValue<str>;
        type on_click;
        type date_time: ::frender_dom::props::MaybeUpdateValue<str>;
    }
}
pub use trait_types::Types;
pub use trait_types::Types as ValidTypes;
pub mod data_struct {
    #[non_exhaustive]
    pub struct HtmlTimeElementProps<TypeDefs: super::Types + ?::core::marker::Sized> {
        pub children: TypeDefs::children,
        pub class: TypeDefs::class,
        pub id: TypeDefs::id,
        pub part: TypeDefs::part,
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
        pub date_time: TypeDefs::date_time,
    }
}
pub use data_struct::HtmlTimeElementProps as Data;
pub struct Building<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
pub struct Replacing<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
mod types_initial {
    #[allow(unused_imports)]
    use super::super::*;
    pub type TypesInitial = dyn super::Types<
        children = (),
        class = (),
        id = (),
        part = (),
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
        date_time = (),
    >;
}
pub use types_initial::TypesInitial;
pub type DataInitial = Data<TypesInitial>;
#[cfg(feature = "dom")]
pub mod render_state {
    #[allow(non_camel_case_types)]
    pub trait RenderStateTypes {
        type children: ::frender_core::RenderState;
        type class: ::core::default::Default;
        type id: ::core::default::Default;
        type part: ::core::default::Default;
        type access_key: ::core::default::Default;
        type auto_capitalize: ::core::default::Default;
        type auto_focus: ::core::default::Default;
        type content_editable: ::core::default::Default;
        type context_menu: ::core::default::Default;
        type dir: ::core::default::Default;
        type draggable: ::core::default::Default;
        type enter_key_hint: ::core::default::Default;
        type hidden: ::core::default::Default;
        type inert: ::core::default::Default;
        type input_mode: ::core::default::Default;
        type is: ::core::default::Default;
        type item_id: ::core::default::Default;
        type item_prop: ::core::default::Default;
        type item_ref: ::core::default::Default;
        type item_scope: ::core::default::Default;
        type item_type: ::core::default::Default;
        type lang: ::core::default::Default;
        type nonce: ::core::default::Default;
        type role: ::core::default::Default;
        type slot: ::core::default::Default;
        type spellcheck: ::core::default::Default;
        type style: ::core::default::Default;
        type tab_index: ::core::default::Default;
        type title: ::core::default::Default;
        type translate: ::core::default::Default;
        type virtual_keyboard_policy: ::core::default::Default;
        type on_click: ::core::default::Default;
        type date_time: ::core::default::Default;
    }
    pub struct RenderState<TypeDefs: RenderStateTypes>
    where
        TypeDefs: ?::core::marker::Sized,
    {
        pub children: TypeDefs::children,
        pub class: TypeDefs::class,
        pub id: TypeDefs::id,
        pub part: TypeDefs::part,
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
        pub date_time: TypeDefs::date_time,
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
        pub children: ::pin_project_lite::__private::Pin<&'__pin mut (TypeDefs::children)>,
        pub class: &'__pin mut (TypeDefs::class),
        pub id: &'__pin mut (TypeDefs::id),
        pub part: &'__pin mut (TypeDefs::part),
        pub access_key: &'__pin mut (TypeDefs::access_key),
        pub auto_capitalize: &'__pin mut (TypeDefs::auto_capitalize),
        pub auto_focus: &'__pin mut (TypeDefs::auto_focus),
        pub content_editable: &'__pin mut (TypeDefs::content_editable),
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
        pub date_time: &'__pin mut (TypeDefs::date_time),
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
            pub children: ::pin_project_lite::__private::Pin<&'__pin (TypeDefs::children)>,
            pub class: &'__pin (TypeDefs::class),
            pub id: &'__pin (TypeDefs::id),
            pub part: &'__pin (TypeDefs::part),
            pub access_key: &'__pin (TypeDefs::access_key),
            pub auto_capitalize: &'__pin (TypeDefs::auto_capitalize),
            pub auto_focus: &'__pin (TypeDefs::auto_focus),
            pub content_editable: &'__pin (TypeDefs::content_editable),
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
            pub date_time: &'__pin (TypeDefs::date_time),
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
                        children,
                        class,
                        id,
                        part,
                        access_key,
                        auto_capitalize,
                        auto_focus,
                        content_editable,
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
                        date_time,
                    } = self.get_unchecked_mut();
                    RenderStateProj {
                        children: ::pin_project_lite::__private::Pin::new_unchecked(children),
                        class: class,
                        id: id,
                        part: part,
                        access_key: access_key,
                        auto_capitalize: auto_capitalize,
                        auto_focus: auto_focus,
                        content_editable: content_editable,
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
                        date_time: date_time,
                    }
                }
            }
            pub(crate) fn project_ref<'__pin>(
                self: ::pin_project_lite::__private::Pin<&'__pin Self>,
            ) -> ProjectionRef<'__pin, TypeDefs> {
                unsafe {
                    let Self {
                        children,
                        class,
                        id,
                        part,
                        access_key,
                        auto_capitalize,
                        auto_focus,
                        content_editable,
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
                        date_time,
                    } = self.get_ref();
                    ProjectionRef {
                        children: ::pin_project_lite::__private::Pin::new_unchecked(children),
                        class: class,
                        id: id,
                        part: part,
                        access_key: access_key,
                        auto_capitalize: auto_capitalize,
                        auto_focus: auto_focus,
                        content_editable: content_editable,
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
                        date_time: date_time,
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
            children: TypeDefs::children,
            class: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::class>,
            id: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::id>,
            part: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::part>,
            access_key: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::access_key>,
            auto_capitalize: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::auto_capitalize>,
            auto_focus: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::auto_focus>,
            content_editable:
                ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::content_editable>,
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
            date_time: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::date_time>,
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
            let _ = &this.children;
            let _ = &this.class;
            let _ = &this.id;
            let _ = &this.part;
            let _ = &this.access_key;
            let _ = &this.auto_capitalize;
            let _ = &this.auto_focus;
            let _ = &this.content_editable;
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
            let _ = &this.date_time;
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
                children: ::frender_core::RenderState::new_uninitialized(),
                class: ::core::default::Default::default(),
                id: ::core::default::Default::default(),
                part: ::core::default::Default::default(),
                access_key: ::core::default::Default::default(),
                auto_capitalize: ::core::default::Default::default(),
                auto_focus: ::core::default::Default::default(),
                content_editable: ::core::default::Default::default(),
                context_menu: ::core::default::Default::default(),
                dir: ::core::default::Default::default(),
                draggable: ::core::default::Default::default(),
                enter_key_hint: ::core::default::Default::default(),
                hidden: ::core::default::Default::default(),
                inert: ::core::default::Default::default(),
                input_mode: ::core::default::Default::default(),
                is: ::core::default::Default::default(),
                item_id: ::core::default::Default::default(),
                item_prop: ::core::default::Default::default(),
                item_ref: ::core::default::Default::default(),
                item_scope: ::core::default::Default::default(),
                item_type: ::core::default::Default::default(),
                lang: ::core::default::Default::default(),
                nonce: ::core::default::Default::default(),
                role: ::core::default::Default::default(),
                slot: ::core::default::Default::default(),
                spellcheck: ::core::default::Default::default(),
                style: ::core::default::Default::default(),
                tab_index: ::core::default::Default::default(),
                title: ::core::default::Default::default(),
                translate: ::core::default::Default::default(),
                virtual_keyboard_policy: ::core::default::Default::default(),
                on_click: ::core::default::Default::default(),
                date_time: ::core::default::Default::default(),
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
            ::frender_core::RenderState::poll_reactive(self.project().children, cx)
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
    use super::super::*;
    impl<TypeDefs: HtmlTimeElementProps::Types + ?::core::marker::Sized>
        HtmlTimeElementProps::Building<TypeDefs>
    {
        pub fn children<V>(
            self,
            children: V,
        ) -> HtmlTimeElementProps::Building<HtmlTimeElementProps::overwrite::children<TypeDefs, V>>
        {
            let __tmp_value = children;
            let HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            } = self.0;
            let _ = children;
            let children = __tmp_value;
            HtmlTimeElementProps::Building(HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            })
        }
        pub fn class<V: ::frender_dom::props::MaybeUpdateValue<str>>(
            self,
            class: V,
        ) -> HtmlTimeElementProps::Building<HtmlTimeElementProps::overwrite::class<TypeDefs, V>>
        {
            let __tmp_value = class;
            let HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            } = self.0;
            let _ = class;
            let class = __tmp_value;
            HtmlTimeElementProps::Building(HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            })
        }
        pub fn id<V: ::frender_dom::props::MaybeUpdateValue<str>>(
            self,
            id: V,
        ) -> HtmlTimeElementProps::Building<HtmlTimeElementProps::overwrite::id<TypeDefs, V>>
        {
            let __tmp_value = id;
            let HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            } = self.0;
            let _ = id;
            let id = __tmp_value;
            HtmlTimeElementProps::Building(HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            })
        }
        pub fn part<V: ::frender_dom::props::MaybeUpdateValue<str>>(
            self,
            part: V,
        ) -> HtmlTimeElementProps::Building<HtmlTimeElementProps::overwrite::part<TypeDefs, V>>
        {
            let __tmp_value = part;
            let HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            } = self.0;
            let _ = part;
            let part = __tmp_value;
            HtmlTimeElementProps::Building(HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            })
        }
        pub fn access_key<V: ::frender_dom::props::MaybeUpdateValue<str>>(
            self,
            access_key: V,
        ) -> HtmlTimeElementProps::Building<HtmlTimeElementProps::overwrite::access_key<TypeDefs, V>>
        {
            let __tmp_value = access_key;
            let HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            } = self.0;
            let _ = access_key;
            let access_key = __tmp_value;
            HtmlTimeElementProps::Building(HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            })
        }
        pub fn auto_capitalize<V: ::frender_dom::props::MaybeUpdateValue<str>>(
            self,
            auto_capitalize: V,
        ) -> HtmlTimeElementProps::Building<
            HtmlTimeElementProps::overwrite::auto_capitalize<TypeDefs, V>,
        > {
            let __tmp_value = auto_capitalize;
            let HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            } = self.0;
            let _ = auto_capitalize;
            let auto_capitalize = __tmp_value;
            HtmlTimeElementProps::Building(HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            })
        }
        pub fn auto_focus<V: ::frender_dom::props::MaybeUpdateValue<bool>>(
            self,
            auto_focus: V,
        ) -> HtmlTimeElementProps::Building<HtmlTimeElementProps::overwrite::auto_focus<TypeDefs, V>>
        {
            let __tmp_value = auto_focus;
            let HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            } = self.0;
            let _ = auto_focus;
            let auto_focus = __tmp_value;
            HtmlTimeElementProps::Building(HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            })
        }
        pub fn content_editable<V: crate::props::MaybeInherit<bool>>(
            self,
            content_editable: V,
        ) -> HtmlTimeElementProps::Building<
            HtmlTimeElementProps::overwrite::content_editable<TypeDefs, V>,
        > {
            let __tmp_value = content_editable;
            let HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            } = self.0;
            let _ = content_editable;
            let content_editable = __tmp_value;
            HtmlTimeElementProps::Building(HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            })
        }
        #[deprecated = "See https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/contextMenu"]
        pub fn context_menu<V: ::frender_dom::props::MaybeUpdateValue<str>>(
            self,
            context_menu: V,
        ) -> HtmlTimeElementProps::Building<
            HtmlTimeElementProps::overwrite::context_menu<TypeDefs, V>,
        > {
            let __tmp_value = context_menu;
            let HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            } = self.0;
            let _ = context_menu;
            let context_menu = __tmp_value;
            HtmlTimeElementProps::Building(HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            })
        }
        pub fn dir<V: ::frender_dom::props::MaybeUpdateValue<str>>(
            self,
            dir: V,
        ) -> HtmlTimeElementProps::Building<HtmlTimeElementProps::overwrite::dir<TypeDefs, V>>
        {
            let __tmp_value = dir;
            let HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            } = self.0;
            let _ = dir;
            let dir = __tmp_value;
            HtmlTimeElementProps::Building(HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            })
        }
        pub fn draggable<V: ::frender_dom::props::MaybeUpdateValue<bool>>(
            self,
            draggable: V,
        ) -> HtmlTimeElementProps::Building<HtmlTimeElementProps::overwrite::draggable<TypeDefs, V>>
        {
            let __tmp_value = draggable;
            let HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            } = self.0;
            let _ = draggable;
            let draggable = __tmp_value;
            HtmlTimeElementProps::Building(HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            })
        }
        pub fn enter_key_hint<V: ::frender_dom::props::MaybeUpdateValue<str>>(
            self,
            enter_key_hint: V,
        ) -> HtmlTimeElementProps::Building<
            HtmlTimeElementProps::overwrite::enter_key_hint<TypeDefs, V>,
        > {
            let __tmp_value = enter_key_hint;
            let HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            } = self.0;
            let _ = enter_key_hint;
            let enter_key_hint = __tmp_value;
            HtmlTimeElementProps::Building(HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            })
        }
        pub fn hidden<V: ::frender_dom::props::MaybeUpdateValue<bool>>(
            self,
            hidden: V,
        ) -> HtmlTimeElementProps::Building<HtmlTimeElementProps::overwrite::hidden<TypeDefs, V>>
        {
            let __tmp_value = hidden;
            let HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            } = self.0;
            let _ = hidden;
            let hidden = __tmp_value;
            HtmlTimeElementProps::Building(HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            })
        }
        pub fn inert<V: ::frender_dom::props::MaybeUpdateValue<bool>>(
            self,
            inert: V,
        ) -> HtmlTimeElementProps::Building<HtmlTimeElementProps::overwrite::inert<TypeDefs, V>>
        {
            let __tmp_value = inert;
            let HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            } = self.0;
            let _ = inert;
            let inert = __tmp_value;
            HtmlTimeElementProps::Building(HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            })
        }
        pub fn input_mode<V: ::frender_dom::props::MaybeUpdateValue<str>>(
            self,
            input_mode: V,
        ) -> HtmlTimeElementProps::Building<HtmlTimeElementProps::overwrite::input_mode<TypeDefs, V>>
        {
            let __tmp_value = input_mode;
            let HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            } = self.0;
            let _ = input_mode;
            let input_mode = __tmp_value;
            HtmlTimeElementProps::Building(HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            })
        }
        pub fn is<V: ::frender_dom::props::MaybeUpdateValue<str>>(
            self,
            is: V,
        ) -> HtmlTimeElementProps::Building<HtmlTimeElementProps::overwrite::is<TypeDefs, V>>
        {
            let __tmp_value = is;
            let HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            } = self.0;
            let _ = is;
            let is = __tmp_value;
            HtmlTimeElementProps::Building(HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            })
        }
        pub fn item_id<V: ::frender_dom::props::MaybeUpdateValue<str>>(
            self,
            item_id: V,
        ) -> HtmlTimeElementProps::Building<HtmlTimeElementProps::overwrite::item_id<TypeDefs, V>>
        {
            let __tmp_value = item_id;
            let HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            } = self.0;
            let _ = item_id;
            let item_id = __tmp_value;
            HtmlTimeElementProps::Building(HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            })
        }
        pub fn item_prop<V: ::frender_dom::props::MaybeUpdateValue<str>>(
            self,
            item_prop: V,
        ) -> HtmlTimeElementProps::Building<HtmlTimeElementProps::overwrite::item_prop<TypeDefs, V>>
        {
            let __tmp_value = item_prop;
            let HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            } = self.0;
            let _ = item_prop;
            let item_prop = __tmp_value;
            HtmlTimeElementProps::Building(HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            })
        }
        pub fn item_ref<V: ::frender_dom::props::MaybeUpdateValue<str>>(
            self,
            item_ref: V,
        ) -> HtmlTimeElementProps::Building<HtmlTimeElementProps::overwrite::item_ref<TypeDefs, V>>
        {
            let __tmp_value = item_ref;
            let HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            } = self.0;
            let _ = item_ref;
            let item_ref = __tmp_value;
            HtmlTimeElementProps::Building(HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            })
        }
        pub fn item_scope<V: ::frender_dom::props::MaybeUpdateValue<str>>(
            self,
            item_scope: V,
        ) -> HtmlTimeElementProps::Building<HtmlTimeElementProps::overwrite::item_scope<TypeDefs, V>>
        {
            let __tmp_value = item_scope;
            let HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            } = self.0;
            let _ = item_scope;
            let item_scope = __tmp_value;
            HtmlTimeElementProps::Building(HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            })
        }
        pub fn item_type<V: ::frender_dom::props::MaybeUpdateValue<str>>(
            self,
            item_type: V,
        ) -> HtmlTimeElementProps::Building<HtmlTimeElementProps::overwrite::item_type<TypeDefs, V>>
        {
            let __tmp_value = item_type;
            let HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            } = self.0;
            let _ = item_type;
            let item_type = __tmp_value;
            HtmlTimeElementProps::Building(HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            })
        }
        pub fn lang<V: ::frender_dom::props::MaybeUpdateValue<str>>(
            self,
            lang: V,
        ) -> HtmlTimeElementProps::Building<HtmlTimeElementProps::overwrite::lang<TypeDefs, V>>
        {
            let __tmp_value = lang;
            let HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            } = self.0;
            let _ = lang;
            let lang = __tmp_value;
            HtmlTimeElementProps::Building(HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            })
        }
        pub fn nonce<V: ::frender_dom::props::MaybeUpdateValue<str>>(
            self,
            nonce: V,
        ) -> HtmlTimeElementProps::Building<HtmlTimeElementProps::overwrite::nonce<TypeDefs, V>>
        {
            let __tmp_value = nonce;
            let HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            } = self.0;
            let _ = nonce;
            let nonce = __tmp_value;
            HtmlTimeElementProps::Building(HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            })
        }
        pub fn role<V: ::frender_dom::props::MaybeUpdateValue<str>>(
            self,
            role: V,
        ) -> HtmlTimeElementProps::Building<HtmlTimeElementProps::overwrite::role<TypeDefs, V>>
        {
            let __tmp_value = role;
            let HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            } = self.0;
            let _ = role;
            let role = __tmp_value;
            HtmlTimeElementProps::Building(HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            })
        }
        pub fn slot<V: ::frender_dom::props::MaybeUpdateValue<str>>(
            self,
            slot: V,
        ) -> HtmlTimeElementProps::Building<HtmlTimeElementProps::overwrite::slot<TypeDefs, V>>
        {
            let __tmp_value = slot;
            let HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            } = self.0;
            let _ = slot;
            let slot = __tmp_value;
            HtmlTimeElementProps::Building(HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            })
        }
        pub fn spellcheck<V: ::frender_dom::props::MaybeUpdateValue<bool>>(
            self,
            spellcheck: V,
        ) -> HtmlTimeElementProps::Building<HtmlTimeElementProps::overwrite::spellcheck<TypeDefs, V>>
        {
            let __tmp_value = spellcheck;
            let HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            } = self.0;
            let _ = spellcheck;
            let spellcheck = __tmp_value;
            HtmlTimeElementProps::Building(HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            })
        }
        pub fn style<V: ::frender_dom::props::MaybeUpdateValue<str>>(
            self,
            style: V,
        ) -> HtmlTimeElementProps::Building<HtmlTimeElementProps::overwrite::style<TypeDefs, V>>
        {
            let __tmp_value = style;
            let HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            } = self.0;
            let _ = style;
            let style = __tmp_value;
            HtmlTimeElementProps::Building(HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            })
        }
        pub fn tab_index<V: ::frender_dom::props::MaybeUpdateValue<i32>>(
            self,
            tab_index: V,
        ) -> HtmlTimeElementProps::Building<HtmlTimeElementProps::overwrite::tab_index<TypeDefs, V>>
        {
            let __tmp_value = tab_index;
            let HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            } = self.0;
            let _ = tab_index;
            let tab_index = __tmp_value;
            HtmlTimeElementProps::Building(HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            })
        }
        pub fn title<V: ::frender_dom::props::MaybeUpdateValue<str>>(
            self,
            title: V,
        ) -> HtmlTimeElementProps::Building<HtmlTimeElementProps::overwrite::title<TypeDefs, V>>
        {
            let __tmp_value = title;
            let HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            } = self.0;
            let _ = title;
            let title = __tmp_value;
            HtmlTimeElementProps::Building(HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            })
        }
        pub fn translate<V: ::frender_dom::props::MaybeUpdateValue<str>>(
            self,
            translate: V,
        ) -> HtmlTimeElementProps::Building<HtmlTimeElementProps::overwrite::translate<TypeDefs, V>>
        {
            let __tmp_value = translate;
            let HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            } = self.0;
            let _ = translate;
            let translate = __tmp_value;
            HtmlTimeElementProps::Building(HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            })
        }
        pub fn virtual_keyboard_policy<V: ::frender_dom::props::MaybeUpdateValue<str>>(
            self,
            virtual_keyboard_policy: V,
        ) -> HtmlTimeElementProps::Building<
            HtmlTimeElementProps::overwrite::virtual_keyboard_policy<TypeDefs, V>,
        > {
            let __tmp_value = virtual_keyboard_policy;
            let HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            } = self.0;
            let _ = virtual_keyboard_policy;
            let virtual_keyboard_policy = __tmp_value;
            HtmlTimeElementProps::Building(HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            })
        }
        pub fn on_click<V>(
            self,
            on_click: V,
        ) -> HtmlTimeElementProps::Building<HtmlTimeElementProps::overwrite::on_click<TypeDefs, V>>
        {
            let __tmp_value = on_click;
            let HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            } = self.0;
            let _ = on_click;
            let on_click = __tmp_value;
            HtmlTimeElementProps::Building(HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            })
        }
        pub fn date_time<V: ::frender_dom::props::MaybeUpdateValue<str>>(
            self,
            date_time: V,
        ) -> HtmlTimeElementProps::Building<HtmlTimeElementProps::overwrite::date_time<TypeDefs, V>>
        {
            let __tmp_value = date_time;
            let HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            } = self.0;
            let _ = date_time;
            let date_time = __tmp_value;
            HtmlTimeElementProps::Building(HtmlTimeElementProps::Data {
                children,
                class,
                id,
                part,
                access_key,
                auto_capitalize,
                auto_focus,
                content_editable,
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
                date_time,
            })
        }
    }
}
#[cfg(feature = "dom")]
mod impl_update_element {
    #[allow(unused_imports)]
    use super::super::*;
    impl<TypeDefs: ?::core::marker::Sized + super::Types>
        crate::props::UpdateElement<web_sys::HtmlTimeElement> for super::Data<TypeDefs>
    where
        TypeDefs::children: ::frender_core::UpdateRenderState<::frender_dom::Dom>,
        TypeDefs::on_click: crate::props::UpdateDomEventListener<crate::props::events::Click>,
    {
        type State = super :: render_state :: RenderState < dyn super :: render_state :: RenderStateTypes < children = < TypeDefs :: children as frender_core :: UpdateRenderState < frender_dom :: Dom , > > :: State , class = < TypeDefs :: class as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , id = < TypeDefs :: id as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , part = < TypeDefs :: part as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , access_key = < TypeDefs :: access_key as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , auto_capitalize = < TypeDefs :: auto_capitalize as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , auto_focus = < TypeDefs :: auto_focus as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: State , content_editable = () , context_menu = < TypeDefs :: context_menu as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , dir = < TypeDefs :: dir as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , draggable = < TypeDefs :: draggable as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: State , enter_key_hint = < TypeDefs :: enter_key_hint as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , hidden = < TypeDefs :: hidden as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: State , inert = < TypeDefs :: inert as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: State , input_mode = < TypeDefs :: input_mode as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , is = < TypeDefs :: is as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , item_id = < TypeDefs :: item_id as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , item_prop = < TypeDefs :: item_prop as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , item_ref = < TypeDefs :: item_ref as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , item_scope = < TypeDefs :: item_scope as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , item_type = < TypeDefs :: item_type as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , lang = < TypeDefs :: lang as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , nonce = < TypeDefs :: nonce as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , role = < TypeDefs :: role as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , slot = < TypeDefs :: slot as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , spellcheck = < TypeDefs :: spellcheck as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: State , style = < TypeDefs :: style as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , tab_index = < TypeDefs :: tab_index as :: frender_dom :: props :: MaybeUpdateValue < i32 , > > :: State , title = < TypeDefs :: title as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , translate = < TypeDefs :: translate as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , virtual_keyboard_policy = < TypeDefs :: virtual_keyboard_policy as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , on_click = < TypeDefs :: on_click as crate :: props :: UpdateDomEventListener < crate :: props :: events :: Click , > > :: State , date_time = < TypeDefs :: date_time as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , > , > ;
        fn update_element(
            this: Self,
            element: &web_sys::HtmlTimeElement,
            children_ctx: &mut ::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let state = state.pin_project();
            let dom_element: &::web_sys::Element = element.as_ref();
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.children,
                state: state.children,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    state,
                    children_ctx,
                    ..
                } => ::frender_core::UpdateRenderState::update_render_state(
                    data,
                    children_ctx,
                    state,
                ),
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.class,
                state: state.class,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "class";
                    < TypeDefs :: class as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.id,
                state: state.id,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "id";
                    < TypeDefs :: id as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | element . set_id (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.part,
                state: state.part,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "part";
                    < TypeDefs :: part as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.access_key,
                state: state.access_key,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "accesskey";
                    < TypeDefs :: access_key as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | element . set_access_key (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.auto_capitalize,
                state: state.auto_capitalize,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "autocapitalize";
                    < TypeDefs :: auto_capitalize as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.auto_focus,
                state: state.auto_focus,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "autofocus";
                    < TypeDefs :: auto_focus as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (data , state , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.content_editable,
                state: state.content_editable,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData { data, .. } => {}
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.context_menu,
                state: state.context_menu,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "contextmenu";
                    < TypeDefs :: context_menu as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.dir,
                state: state.dir,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "dir";
                    < TypeDefs :: dir as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | element . set_dir (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.draggable,
                state: state.draggable,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "draggable";
                    < TypeDefs :: draggable as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (data , state , | v | element . set_draggable (* v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.enter_key_hint,
                state: state.enter_key_hint,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "enterkeyhint";
                    < TypeDefs :: enter_key_hint as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.hidden,
                state: state.hidden,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "hidden";
                    < TypeDefs :: hidden as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (data , state , | v | element . set_hidden (* v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.inert,
                state: state.inert,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "inert";
                    < TypeDefs :: inert as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (data , state , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.input_mode,
                state: state.input_mode,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "inputmode";
                    < TypeDefs :: input_mode as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.is,
                state: state.is,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "is";
                    < TypeDefs :: is as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.item_id,
                state: state.item_id,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "itemid";
                    < TypeDefs :: item_id as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.item_prop,
                state: state.item_prop,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "itemprop";
                    < TypeDefs :: item_prop as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.item_ref,
                state: state.item_ref,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "itemref";
                    < TypeDefs :: item_ref as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.item_scope,
                state: state.item_scope,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "itemscope";
                    < TypeDefs :: item_scope as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.item_type,
                state: state.item_type,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "itemtype";
                    < TypeDefs :: item_type as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.lang,
                state: state.lang,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "lang";
                    < TypeDefs :: lang as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | element . set_lang (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.nonce,
                state: state.nonce,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "nonce";
                    < TypeDefs :: nonce as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.role,
                state: state.role,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "role";
                    < TypeDefs :: role as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.slot,
                state: state.slot,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "slot";
                    < TypeDefs :: slot as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.spellcheck,
                state: state.spellcheck,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "spellcheck";
                    < TypeDefs :: spellcheck as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (data , state , | v | element . set_spellcheck (* v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.style,
                state: state.style,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "style";
                    < TypeDefs :: style as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.tab_index,
                state: state.tab_index,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "tabindex";
                    < TypeDefs :: tab_index as :: frender_dom :: props :: MaybeUpdateValue < i32 , > > :: maybe_update_value (data , state , | v | element . set_tab_index (* v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.title,
                state: state.title,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "title";
                    < TypeDefs :: title as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | element . set_title (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.translate,
                state: state.translate,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "translate";
                    < TypeDefs :: translate as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.virtual_keyboard_policy,
                state: state.virtual_keyboard_policy,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "virtualkeyboardpolicy";
                    <TypeDefs::virtual_keyboard_policy as ::frender_dom::props::MaybeUpdateValue<
                        str,
                    >>::maybe_update_value(
                        data,
                        state,
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
            }
            # [allow (unused_variables)] match (crate :: props :: FieldData { data : this . on_click , state : state . on_click , element , dom_element , children_ctx : & mut * children_ctx , }) { crate :: props :: FieldData { data , state , element , .. } => { crate :: props :: UpdateDomEventListener :: < crate :: props :: events :: Click , > :: update_dom_event_listener (data , element , state) } }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.date_time,
                state: state.date_time,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "datetime";
                    < TypeDefs :: date_time as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | element . set_date_time (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
        }
    }
}
