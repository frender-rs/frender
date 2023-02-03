#[allow(non_snake_case)]
pub fn HtmlFieldSetElementProps() -> Building<TypesInitial> {
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
        disabled: (),
        form: (),
        name: (),
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
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        name = <TypeDefs as super::Types>::name,
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
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        name = <TypeDefs as super::Types>::name,
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
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        name = <TypeDefs as super::Types>::name,
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
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        name = <TypeDefs as super::Types>::name,
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
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        name = <TypeDefs as super::Types>::name,
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
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        name = <TypeDefs as super::Types>::name,
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
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        name = <TypeDefs as super::Types>::name,
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
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        name = <TypeDefs as super::Types>::name,
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
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        name = <TypeDefs as super::Types>::name,
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
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        name = <TypeDefs as super::Types>::name,
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
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        name = <TypeDefs as super::Types>::name,
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
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        name = <TypeDefs as super::Types>::name,
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
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        name = <TypeDefs as super::Types>::name,
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
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        name = <TypeDefs as super::Types>::name,
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
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        name = <TypeDefs as super::Types>::name,
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
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        name = <TypeDefs as super::Types>::name,
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
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        name = <TypeDefs as super::Types>::name,
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
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        name = <TypeDefs as super::Types>::name,
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
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        name = <TypeDefs as super::Types>::name,
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
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        name = <TypeDefs as super::Types>::name,
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
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        name = <TypeDefs as super::Types>::name,
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
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        name = <TypeDefs as super::Types>::name,
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
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        name = <TypeDefs as super::Types>::name,
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
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        name = <TypeDefs as super::Types>::name,
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
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        name = <TypeDefs as super::Types>::name,
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
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        name = <TypeDefs as super::Types>::name,
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
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        name = <TypeDefs as super::Types>::name,
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
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        name = <TypeDefs as super::Types>::name,
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
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        name = <TypeDefs as super::Types>::name,
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
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        name = <TypeDefs as super::Types>::name,
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
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        name = <TypeDefs as super::Types>::name,
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
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        name = <TypeDefs as super::Types>::name,
    >;
    pub type disabled<TypeDefs, Value> = dyn super::Types<
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
        disabled = Value,
        form = <TypeDefs as super::Types>::form,
        name = <TypeDefs as super::Types>::name,
    >;
    pub type form<TypeDefs, Value> = dyn super::Types<
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
        disabled = <TypeDefs as super::Types>::disabled,
        form = Value,
        name = <TypeDefs as super::Types>::name,
    >;
    pub type name<TypeDefs, Value> = dyn super::Types<
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
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        name = Value,
    >;
}
mod trait_types {
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub trait Types {
        type children;
        type class: crate::MaybeUpdateValue<str>;
        type id: crate::MaybeUpdateValue<str>;
        type part: crate::MaybeUpdateValue<str>;
        type access_key: crate::MaybeUpdateValue<str>;
        type auto_capitalize: crate::MaybeUpdateValue<str>;
        type auto_focus: crate::MaybeUpdateValue<bool>;
        type content_editable: crate::props::MaybeInherit<bool>;
        type context_menu: crate::MaybeUpdateValue<str>;
        type dir: crate::MaybeUpdateValue<str>;
        type draggable: crate::MaybeUpdateValue<bool>;
        type enter_key_hint: crate::MaybeUpdateValue<str>;
        type hidden: crate::MaybeUpdateValue<bool>;
        type inert: crate::MaybeUpdateValue<bool>;
        type input_mode: crate::MaybeUpdateValue<str>;
        type is: crate::MaybeUpdateValue<str>;
        type item_id: crate::MaybeUpdateValue<str>;
        type item_prop: crate::MaybeUpdateValue<str>;
        type item_ref: crate::MaybeUpdateValue<str>;
        type item_scope: crate::MaybeUpdateValue<str>;
        type item_type: crate::MaybeUpdateValue<str>;
        type lang: crate::MaybeUpdateValue<str>;
        type nonce: crate::MaybeUpdateValue<str>;
        type role: crate::MaybeUpdateValue<str>;
        type slot: crate::MaybeUpdateValue<str>;
        type spellcheck: crate::MaybeUpdateValue<bool>;
        type style: crate::MaybeUpdateValue<str>;
        type tab_index: crate::MaybeUpdateValue<i32>;
        type title: crate::MaybeUpdateValue<str>;
        type translate: crate::MaybeUpdateValue<str>;
        type virtual_keyboard_policy: crate::MaybeUpdateValue<str>;
        type on_click;
        type disabled: crate::MaybeUpdateValue<bool>;
        type form: crate::MaybeUpdateValue<str>;
        type name: crate::MaybeUpdateValue<str>;
    }
}
pub use trait_types::Types;
pub use trait_types::Types as ValidTypes;
pub mod data_struct {
    #[non_exhaustive]
    pub struct HtmlFieldSetElementProps<TypeDefs: super::Types + ?::core::marker::Sized> {
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
        pub disabled: TypeDefs::disabled,
        pub form: TypeDefs::form,
        pub name: TypeDefs::name,
    }
}
pub use data_struct::HtmlFieldSetElementProps as Data;
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
        disabled = (),
        form = (),
        name = (),
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
        type disabled: ::core::default::Default;
        type form: ::core::default::Default;
        type name: ::core::default::Default;
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
        pub disabled: TypeDefs::disabled,
        pub form: TypeDefs::form,
        pub name: TypeDefs::name,
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
        pub disabled: &'__pin mut (TypeDefs::disabled),
        pub form: &'__pin mut (TypeDefs::form),
        pub name: &'__pin mut (TypeDefs::name),
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
            pub disabled: &'__pin (TypeDefs::disabled),
            pub form: &'__pin (TypeDefs::form),
            pub name: &'__pin (TypeDefs::name),
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
                        disabled,
                        form,
                        name,
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
                        disabled: disabled,
                        form: form,
                        name: name,
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
                        disabled,
                        form,
                        name,
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
                        disabled: disabled,
                        form: form,
                        name: name,
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
            disabled: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::disabled>,
            form: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::form>,
            name: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::name>,
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
            let _ = &this.disabled;
            let _ = &this.form;
            let _ = &this.name;
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
                disabled: ::core::default::Default::default(),
                form: ::core::default::Default::default(),
                name: ::core::default::Default::default(),
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
    #[allow(unused_imports)]
    use super::super::*;
    impl<TypeDefs: super::Types + ?::core::marker::Sized> super::Building<TypeDefs> {
        pub fn children<V>(
            self,
            children: V,
        ) -> super::Building<super::overwrite::children<TypeDefs, V>> {
            super::Building(super::Data {
                children,
                class: self.0.class,
                id: self.0.id,
                part: self.0.part,
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
                disabled: self.0.disabled,
                form: self.0.form,
                name: self.0.name,
            })
        }
        pub fn class<V: crate::MaybeUpdateValue<str>>(
            self,
            class: V,
        ) -> super::Building<super::overwrite::class<TypeDefs, V>> {
            super::Building(super::Data {
                children: self.0.children,
                class,
                id: self.0.id,
                part: self.0.part,
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
                disabled: self.0.disabled,
                form: self.0.form,
                name: self.0.name,
            })
        }
        pub fn id<V: crate::MaybeUpdateValue<str>>(
            self,
            id: V,
        ) -> super::Building<super::overwrite::id<TypeDefs, V>> {
            super::Building(super::Data {
                children: self.0.children,
                class: self.0.class,
                id,
                part: self.0.part,
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
                disabled: self.0.disabled,
                form: self.0.form,
                name: self.0.name,
            })
        }
        pub fn part<V: crate::MaybeUpdateValue<str>>(
            self,
            part: V,
        ) -> super::Building<super::overwrite::part<TypeDefs, V>> {
            super::Building(super::Data {
                children: self.0.children,
                class: self.0.class,
                id: self.0.id,
                part,
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
                disabled: self.0.disabled,
                form: self.0.form,
                name: self.0.name,
            })
        }
        pub fn access_key<V: crate::MaybeUpdateValue<str>>(
            self,
            access_key: V,
        ) -> super::Building<super::overwrite::access_key<TypeDefs, V>> {
            super::Building(super::Data {
                children: self.0.children,
                class: self.0.class,
                id: self.0.id,
                part: self.0.part,
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
                disabled: self.0.disabled,
                form: self.0.form,
                name: self.0.name,
            })
        }
        pub fn auto_capitalize<V: crate::MaybeUpdateValue<str>>(
            self,
            auto_capitalize: V,
        ) -> super::Building<super::overwrite::auto_capitalize<TypeDefs, V>> {
            super::Building(super::Data {
                children: self.0.children,
                class: self.0.class,
                id: self.0.id,
                part: self.0.part,
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
                disabled: self.0.disabled,
                form: self.0.form,
                name: self.0.name,
            })
        }
        pub fn auto_focus<V: crate::MaybeUpdateValue<bool>>(
            self,
            auto_focus: V,
        ) -> super::Building<super::overwrite::auto_focus<TypeDefs, V>> {
            super::Building(super::Data {
                children: self.0.children,
                class: self.0.class,
                id: self.0.id,
                part: self.0.part,
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
                disabled: self.0.disabled,
                form: self.0.form,
                name: self.0.name,
            })
        }
        pub fn content_editable<V: crate::props::MaybeInherit<bool>>(
            self,
            content_editable: V,
        ) -> super::Building<super::overwrite::content_editable<TypeDefs, V>> {
            super::Building(super::Data {
                children: self.0.children,
                class: self.0.class,
                id: self.0.id,
                part: self.0.part,
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
                disabled: self.0.disabled,
                form: self.0.form,
                name: self.0.name,
            })
        }
        #[deprecated = "See https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/contextMenu"]
        pub fn context_menu<V: crate::MaybeUpdateValue<str>>(
            self,
            context_menu: V,
        ) -> super::Building<super::overwrite::context_menu<TypeDefs, V>> {
            super::Building(super::Data {
                children: self.0.children,
                class: self.0.class,
                id: self.0.id,
                part: self.0.part,
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
                disabled: self.0.disabled,
                form: self.0.form,
                name: self.0.name,
            })
        }
        pub fn dir<V: crate::MaybeUpdateValue<str>>(
            self,
            dir: V,
        ) -> super::Building<super::overwrite::dir<TypeDefs, V>> {
            super::Building(super::Data {
                children: self.0.children,
                class: self.0.class,
                id: self.0.id,
                part: self.0.part,
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
                disabled: self.0.disabled,
                form: self.0.form,
                name: self.0.name,
            })
        }
        pub fn draggable<V: crate::MaybeUpdateValue<bool>>(
            self,
            draggable: V,
        ) -> super::Building<super::overwrite::draggable<TypeDefs, V>> {
            super::Building(super::Data {
                children: self.0.children,
                class: self.0.class,
                id: self.0.id,
                part: self.0.part,
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
                disabled: self.0.disabled,
                form: self.0.form,
                name: self.0.name,
            })
        }
        pub fn enter_key_hint<V: crate::MaybeUpdateValue<str>>(
            self,
            enter_key_hint: V,
        ) -> super::Building<super::overwrite::enter_key_hint<TypeDefs, V>> {
            super::Building(super::Data {
                children: self.0.children,
                class: self.0.class,
                id: self.0.id,
                part: self.0.part,
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
                disabled: self.0.disabled,
                form: self.0.form,
                name: self.0.name,
            })
        }
        pub fn hidden<V: crate::MaybeUpdateValue<bool>>(
            self,
            hidden: V,
        ) -> super::Building<super::overwrite::hidden<TypeDefs, V>> {
            super::Building(super::Data {
                children: self.0.children,
                class: self.0.class,
                id: self.0.id,
                part: self.0.part,
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
                disabled: self.0.disabled,
                form: self.0.form,
                name: self.0.name,
            })
        }
        pub fn inert<V: crate::MaybeUpdateValue<bool>>(
            self,
            inert: V,
        ) -> super::Building<super::overwrite::inert<TypeDefs, V>> {
            super::Building(super::Data {
                children: self.0.children,
                class: self.0.class,
                id: self.0.id,
                part: self.0.part,
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
                disabled: self.0.disabled,
                form: self.0.form,
                name: self.0.name,
            })
        }
        pub fn input_mode<V: crate::MaybeUpdateValue<str>>(
            self,
            input_mode: V,
        ) -> super::Building<super::overwrite::input_mode<TypeDefs, V>> {
            super::Building(super::Data {
                children: self.0.children,
                class: self.0.class,
                id: self.0.id,
                part: self.0.part,
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
                disabled: self.0.disabled,
                form: self.0.form,
                name: self.0.name,
            })
        }
        pub fn is<V: crate::MaybeUpdateValue<str>>(
            self,
            is: V,
        ) -> super::Building<super::overwrite::is<TypeDefs, V>> {
            super::Building(super::Data {
                children: self.0.children,
                class: self.0.class,
                id: self.0.id,
                part: self.0.part,
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
                disabled: self.0.disabled,
                form: self.0.form,
                name: self.0.name,
            })
        }
        pub fn item_id<V: crate::MaybeUpdateValue<str>>(
            self,
            item_id: V,
        ) -> super::Building<super::overwrite::item_id<TypeDefs, V>> {
            super::Building(super::Data {
                children: self.0.children,
                class: self.0.class,
                id: self.0.id,
                part: self.0.part,
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
                disabled: self.0.disabled,
                form: self.0.form,
                name: self.0.name,
            })
        }
        pub fn item_prop<V: crate::MaybeUpdateValue<str>>(
            self,
            item_prop: V,
        ) -> super::Building<super::overwrite::item_prop<TypeDefs, V>> {
            super::Building(super::Data {
                children: self.0.children,
                class: self.0.class,
                id: self.0.id,
                part: self.0.part,
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
                disabled: self.0.disabled,
                form: self.0.form,
                name: self.0.name,
            })
        }
        pub fn item_ref<V: crate::MaybeUpdateValue<str>>(
            self,
            item_ref: V,
        ) -> super::Building<super::overwrite::item_ref<TypeDefs, V>> {
            super::Building(super::Data {
                children: self.0.children,
                class: self.0.class,
                id: self.0.id,
                part: self.0.part,
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
                disabled: self.0.disabled,
                form: self.0.form,
                name: self.0.name,
            })
        }
        pub fn item_scope<V: crate::MaybeUpdateValue<str>>(
            self,
            item_scope: V,
        ) -> super::Building<super::overwrite::item_scope<TypeDefs, V>> {
            super::Building(super::Data {
                children: self.0.children,
                class: self.0.class,
                id: self.0.id,
                part: self.0.part,
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
                disabled: self.0.disabled,
                form: self.0.form,
                name: self.0.name,
            })
        }
        pub fn item_type<V: crate::MaybeUpdateValue<str>>(
            self,
            item_type: V,
        ) -> super::Building<super::overwrite::item_type<TypeDefs, V>> {
            super::Building(super::Data {
                children: self.0.children,
                class: self.0.class,
                id: self.0.id,
                part: self.0.part,
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
                disabled: self.0.disabled,
                form: self.0.form,
                name: self.0.name,
            })
        }
        pub fn lang<V: crate::MaybeUpdateValue<str>>(
            self,
            lang: V,
        ) -> super::Building<super::overwrite::lang<TypeDefs, V>> {
            super::Building(super::Data {
                children: self.0.children,
                class: self.0.class,
                id: self.0.id,
                part: self.0.part,
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
                disabled: self.0.disabled,
                form: self.0.form,
                name: self.0.name,
            })
        }
        pub fn nonce<V: crate::MaybeUpdateValue<str>>(
            self,
            nonce: V,
        ) -> super::Building<super::overwrite::nonce<TypeDefs, V>> {
            super::Building(super::Data {
                children: self.0.children,
                class: self.0.class,
                id: self.0.id,
                part: self.0.part,
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
                disabled: self.0.disabled,
                form: self.0.form,
                name: self.0.name,
            })
        }
        pub fn role<V: crate::MaybeUpdateValue<str>>(
            self,
            role: V,
        ) -> super::Building<super::overwrite::role<TypeDefs, V>> {
            super::Building(super::Data {
                children: self.0.children,
                class: self.0.class,
                id: self.0.id,
                part: self.0.part,
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
                disabled: self.0.disabled,
                form: self.0.form,
                name: self.0.name,
            })
        }
        pub fn slot<V: crate::MaybeUpdateValue<str>>(
            self,
            slot: V,
        ) -> super::Building<super::overwrite::slot<TypeDefs, V>> {
            super::Building(super::Data {
                children: self.0.children,
                class: self.0.class,
                id: self.0.id,
                part: self.0.part,
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
                disabled: self.0.disabled,
                form: self.0.form,
                name: self.0.name,
            })
        }
        pub fn spellcheck<V: crate::MaybeUpdateValue<bool>>(
            self,
            spellcheck: V,
        ) -> super::Building<super::overwrite::spellcheck<TypeDefs, V>> {
            super::Building(super::Data {
                children: self.0.children,
                class: self.0.class,
                id: self.0.id,
                part: self.0.part,
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
                disabled: self.0.disabled,
                form: self.0.form,
                name: self.0.name,
            })
        }
        pub fn style<V: crate::MaybeUpdateValue<str>>(
            self,
            style: V,
        ) -> super::Building<super::overwrite::style<TypeDefs, V>> {
            super::Building(super::Data {
                children: self.0.children,
                class: self.0.class,
                id: self.0.id,
                part: self.0.part,
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
                disabled: self.0.disabled,
                form: self.0.form,
                name: self.0.name,
            })
        }
        pub fn tab_index<V: crate::MaybeUpdateValue<i32>>(
            self,
            tab_index: V,
        ) -> super::Building<super::overwrite::tab_index<TypeDefs, V>> {
            super::Building(super::Data {
                children: self.0.children,
                class: self.0.class,
                id: self.0.id,
                part: self.0.part,
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
                disabled: self.0.disabled,
                form: self.0.form,
                name: self.0.name,
            })
        }
        pub fn title<V: crate::MaybeUpdateValue<str>>(
            self,
            title: V,
        ) -> super::Building<super::overwrite::title<TypeDefs, V>> {
            super::Building(super::Data {
                children: self.0.children,
                class: self.0.class,
                id: self.0.id,
                part: self.0.part,
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
                disabled: self.0.disabled,
                form: self.0.form,
                name: self.0.name,
            })
        }
        pub fn translate<V: crate::MaybeUpdateValue<str>>(
            self,
            translate: V,
        ) -> super::Building<super::overwrite::translate<TypeDefs, V>> {
            super::Building(super::Data {
                children: self.0.children,
                class: self.0.class,
                id: self.0.id,
                part: self.0.part,
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
                disabled: self.0.disabled,
                form: self.0.form,
                name: self.0.name,
            })
        }
        pub fn virtual_keyboard_policy<V: crate::MaybeUpdateValue<str>>(
            self,
            virtual_keyboard_policy: V,
        ) -> super::Building<super::overwrite::virtual_keyboard_policy<TypeDefs, V>> {
            super::Building(super::Data {
                children: self.0.children,
                class: self.0.class,
                id: self.0.id,
                part: self.0.part,
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
                disabled: self.0.disabled,
                form: self.0.form,
                name: self.0.name,
            })
        }
        pub fn on_click<V>(
            self,
            on_click: V,
        ) -> super::Building<super::overwrite::on_click<TypeDefs, V>> {
            super::Building(super::Data {
                children: self.0.children,
                class: self.0.class,
                id: self.0.id,
                part: self.0.part,
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
                disabled: self.0.disabled,
                form: self.0.form,
                name: self.0.name,
            })
        }
        pub fn disabled<V: crate::MaybeUpdateValue<bool>>(
            self,
            disabled: V,
        ) -> super::Building<super::overwrite::disabled<TypeDefs, V>> {
            super::Building(super::Data {
                children: self.0.children,
                class: self.0.class,
                id: self.0.id,
                part: self.0.part,
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
                disabled,
                form: self.0.form,
                name: self.0.name,
            })
        }
        pub fn form<V: crate::MaybeUpdateValue<str>>(
            self,
            form: V,
        ) -> super::Building<super::overwrite::form<TypeDefs, V>> {
            super::Building(super::Data {
                children: self.0.children,
                class: self.0.class,
                id: self.0.id,
                part: self.0.part,
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
                disabled: self.0.disabled,
                form,
                name: self.0.name,
            })
        }
        pub fn name<V: crate::MaybeUpdateValue<str>>(
            self,
            name: V,
        ) -> super::Building<super::overwrite::name<TypeDefs, V>> {
            super::Building(super::Data {
                children: self.0.children,
                class: self.0.class,
                id: self.0.id,
                part: self.0.part,
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
                disabled: self.0.disabled,
                form: self.0.form,
                name,
            })
        }
    }
}
#[cfg(feature = "dom")]
mod impl_update_element {
    #[allow(unused_imports)]
    use super::super::*;
    impl<TypeDefs: ?::core::marker::Sized + super::Types>
        crate::props::UpdateElement<web_sys::HtmlFieldSetElement> for super::Data<TypeDefs>
    where
        TypeDefs::children: ::frender_core::UpdateRenderState<::frender_dom::Dom>,
        TypeDefs::on_click: crate::props::UpdateDomEventListener<crate::props::events::Click>,
    {
        type State = super :: render_state :: RenderState < dyn super :: render_state :: RenderStateTypes < children = < TypeDefs :: children as frender_core :: UpdateRenderState < frender_dom :: Dom , > > :: State , class = < TypeDefs :: class as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , id = < TypeDefs :: id as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , part = < TypeDefs :: part as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , access_key = < TypeDefs :: access_key as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , auto_capitalize = < TypeDefs :: auto_capitalize as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , auto_focus = < TypeDefs :: auto_focus as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: State , content_editable = () , context_menu = < TypeDefs :: context_menu as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , dir = < TypeDefs :: dir as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , draggable = < TypeDefs :: draggable as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: State , enter_key_hint = < TypeDefs :: enter_key_hint as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , hidden = < TypeDefs :: hidden as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: State , inert = < TypeDefs :: inert as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: State , input_mode = < TypeDefs :: input_mode as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , is = < TypeDefs :: is as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , item_id = < TypeDefs :: item_id as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , item_prop = < TypeDefs :: item_prop as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , item_ref = < TypeDefs :: item_ref as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , item_scope = < TypeDefs :: item_scope as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , item_type = < TypeDefs :: item_type as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , lang = < TypeDefs :: lang as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , nonce = < TypeDefs :: nonce as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , role = < TypeDefs :: role as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , slot = < TypeDefs :: slot as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , spellcheck = < TypeDefs :: spellcheck as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: State , style = < TypeDefs :: style as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , tab_index = < TypeDefs :: tab_index as :: frender_dom :: props :: MaybeUpdateValue < i32 , > > :: State , title = < TypeDefs :: title as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , translate = < TypeDefs :: translate as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , virtual_keyboard_policy = < TypeDefs :: virtual_keyboard_policy as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , on_click = < TypeDefs :: on_click as crate :: props :: UpdateDomEventListener < crate :: props :: events :: Click , > > :: State , disabled = < TypeDefs :: disabled as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: State , form = < TypeDefs :: form as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , name = < TypeDefs :: name as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , > , > ;
        fn update_element(
            this: Self,
            element: &web_sys::HtmlFieldSetElement,
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
                data: this.disabled,
                state: state.disabled,
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
                    const ATTR_NAME: &::core::primitive::str = "disabled";
                    < TypeDefs :: disabled as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (data , state , | v | element . set_disabled (* v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.form,
                state: state.form,
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
                    const ATTR_NAME: &::core::primitive::str = "form";
                    < TypeDefs :: form as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.name,
                state: state.name,
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
                    const ATTR_NAME: &::core::primitive::str = "name";
                    < TypeDefs :: name as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | element . set_name (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
        }
    }
}
