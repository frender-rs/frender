def_props_type!(
    #[derive(Debug, Clone, Copy, Default)]
    HtmlElementProps(
        ..ElementProps(
            children,
            class: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
            id: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
            part: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
            on_cancel,
            on_error,
            on_scroll,
            on_security_policy_violation,
            on_select,
            on_wheel,
            on_copy,
            on_cut,
            on_paste,
            on_composition_end,
            on_composition_start,
            on_composition_update,
            on_blur,
            on_focus,
            on_focus_in,
            on_focus_out,
            on_fullscreen_change,
            on_fullscreen_error,
            on_key_down,
            on_key_up,
            on_aux_click,
            on_click,
            on_context_menu,
            on_double_click,
            on_mouse_down,
            on_mouse_enter,
            on_mouse_leave,
            on_mouse_move,
            on_mouse_out,
            on_mouse_over,
            on_mouse_up,
            on_touch_cancel,
            on_touch_end,
            on_touch_move,
            on_touch_start,
        ),
        access_key: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        auto_capitalize:
            bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        auto_focus: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>],
        context_menu: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        dir: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        draggable: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>],
        enter_key_hint:
            bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        hidden: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>],
        inert: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>],
        input_mode: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        is: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        item_id: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        item_prop: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        item_ref: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        item_scope: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        item_type: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        lang: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        nonce: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        role: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        slot: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        spellcheck: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>],
        style: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        tab_index: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<i32>],
        title: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        translate: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        virtual_keyboard_policy:
            bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        on_invalid,
        on_animation_cancel,
        on_animation_end,
        on_animation_iteration,
        on_animation_start,
        on_before_input,
        on_input,
        on_change,
        on_got_pointer_capture,
        on_lost_pointer_capture,
        on_pointer_cancel,
        on_pointer_down,
        on_pointer_enter,
        on_pointer_leave,
        on_pointer_move,
        on_pointer_out,
        on_pointer_over,
        on_pointer_up,
        on_transition_cancel,
        on_transition_end,
        on_transition_run,
        on_transition_start,
        on_drag,
        on_drag_end,
        on_drag_enter,
        on_drag_leave,
        on_drag_over,
        on_drag_start,
        on_drop,
    )
);
#[cfg(feature = "dom")]
mod impl_dom_for_props {
    #![allow(unused_variables)]
    #[allow(unused_imports)]
    use super::super::*;
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::access_key<V>
    {
        type State = super::props::access_key<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::access_key(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_access_key(v),
                    || dom_element.remove_attribute("accesskey").unwrap(),
                ),
            )
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| element.set_access_key(v),
                || dom_element.remove_attribute("accesskey").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::auto_capitalize<V>
    {
        type State = super::props::auto_capitalize<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::auto_capitalize(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "autocapitalize",
                    ),
                    || dom_element.remove_attribute("autocapitalize").unwrap(),
                ),
            )
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "autocapitalize",
                ),
                || dom_element.remove_attribute("autocapitalize").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::auto_focus<V>
    {
        type State = super::props::auto_focus<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::auto_focus(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        *v,
                        dom_element,
                        "autofocus",
                    ),
                    || dom_element.remove_attribute("autofocus").unwrap(),
                ),
            )
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                bool,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                    *v,
                    dom_element,
                    "autofocus",
                ),
                || dom_element.remove_attribute("autofocus").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::context_menu<V>
    {
        type State = super::props::context_menu<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::context_menu(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "contextmenu",
                    ),
                    || dom_element.remove_attribute("contextmenu").unwrap(),
                ),
            )
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "contextmenu",
                ),
                || dom_element.remove_attribute("contextmenu").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E> for super::props::dir<V>
    {
        type State = super::props::dir<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::dir(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_dir(v),
                    || dom_element.remove_attribute("dir").unwrap(),
                ),
            )
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| element.set_dir(v),
                || dom_element.remove_attribute("dir").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::draggable<V>
    {
        type State = super::props::draggable<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::draggable(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_draggable(*v),
                    || dom_element.remove_attribute("draggable").unwrap(),
                ),
            )
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                bool,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| element.set_draggable(*v),
                || dom_element.remove_attribute("draggable").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::enter_key_hint<V>
    {
        type State = super::props::enter_key_hint<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::enter_key_hint(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "enterkeyhint",
                    ),
                    || dom_element.remove_attribute("enterkeyhint").unwrap(),
                ),
            )
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "enterkeyhint",
                ),
                || dom_element.remove_attribute("enterkeyhint").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::hidden<V>
    {
        type State = super::props::hidden<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::hidden(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_hidden(*v),
                    || dom_element.remove_attribute("hidden").unwrap(),
                ),
            )
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                bool,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| element.set_hidden(*v),
                || dom_element.remove_attribute("hidden").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::inert<V>
    {
        type State = super::props::inert<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::inert(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        *v,
                        dom_element,
                        "inert",
                    ),
                    || dom_element.remove_attribute("inert").unwrap(),
                ),
            )
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                bool,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                    *v,
                    dom_element,
                    "inert",
                ),
                || dom_element.remove_attribute("inert").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::input_mode<V>
    {
        type State = super::props::input_mode<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::input_mode(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "inputmode",
                    ),
                    || dom_element.remove_attribute("inputmode").unwrap(),
                ),
            )
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "inputmode",
                ),
                || dom_element.remove_attribute("inputmode").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E> for super::props::is<V>
    {
        type State = super::props::is<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::is(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "is",
                    ),
                    || dom_element.remove_attribute("is").unwrap(),
                ),
            )
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "is",
                ),
                || dom_element.remove_attribute("is").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::item_id<V>
    {
        type State = super::props::item_id<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::item_id(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "itemid",
                    ),
                    || dom_element.remove_attribute("itemid").unwrap(),
                ),
            )
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "itemid",
                ),
                || dom_element.remove_attribute("itemid").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::item_prop<V>
    {
        type State = super::props::item_prop<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::item_prop(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "itemprop",
                    ),
                    || dom_element.remove_attribute("itemprop").unwrap(),
                ),
            )
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "itemprop",
                ),
                || dom_element.remove_attribute("itemprop").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::item_ref<V>
    {
        type State = super::props::item_ref<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::item_ref(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "itemref",
                    ),
                    || dom_element.remove_attribute("itemref").unwrap(),
                ),
            )
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "itemref",
                ),
                || dom_element.remove_attribute("itemref").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::item_scope<V>
    {
        type State = super::props::item_scope<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::item_scope(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "itemscope",
                    ),
                    || dom_element.remove_attribute("itemscope").unwrap(),
                ),
            )
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "itemscope",
                ),
                || dom_element.remove_attribute("itemscope").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::item_type<V>
    {
        type State = super::props::item_type<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::item_type(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "itemtype",
                    ),
                    || dom_element.remove_attribute("itemtype").unwrap(),
                ),
            )
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "itemtype",
                ),
                || dom_element.remove_attribute("itemtype").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::lang<V>
    {
        type State = super::props::lang<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::lang(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_lang(v),
                    || dom_element.remove_attribute("lang").unwrap(),
                ),
            )
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| element.set_lang(v),
                || dom_element.remove_attribute("lang").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::nonce<V>
    {
        type State = super::props::nonce<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::nonce(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "nonce",
                    ),
                    || dom_element.remove_attribute("nonce").unwrap(),
                ),
            )
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "nonce",
                ),
                || dom_element.remove_attribute("nonce").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::role<V>
    {
        type State = super::props::role<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::role(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "role",
                    ),
                    || dom_element.remove_attribute("role").unwrap(),
                ),
            )
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "role",
                ),
                || dom_element.remove_attribute("role").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::slot<V>
    {
        type State = super::props::slot<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::slot(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "slot",
                    ),
                    || dom_element.remove_attribute("slot").unwrap(),
                ),
            )
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "slot",
                ),
                || dom_element.remove_attribute("slot").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::spellcheck<V>
    {
        type State = super::props::spellcheck<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::spellcheck(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_spellcheck(*v),
                    || dom_element.remove_attribute("spellcheck").unwrap(),
                ),
            )
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                bool,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| element.set_spellcheck(*v),
                || dom_element.remove_attribute("spellcheck").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::style<V>
    {
        type State = super::props::style<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::style(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "style",
                    ),
                    || dom_element.remove_attribute("style").unwrap(),
                ),
            )
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "style",
                ),
                || dom_element.remove_attribute("style").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<i32>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::tab_index<V>
    {
        type State = super::props::tab_index<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::tab_index(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    i32,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_tab_index(*v),
                    || dom_element.remove_attribute("tabindex").unwrap(),
                ),
            )
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                i32,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| element.set_tab_index(*v),
                || dom_element.remove_attribute("tabindex").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::title<V>
    {
        type State = super::props::title<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::title(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_title(v),
                    || dom_element.remove_attribute("title").unwrap(),
                ),
            )
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| element.set_title(v),
                || dom_element.remove_attribute("title").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::translate<V>
    {
        type State = super::props::translate<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::translate(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "translate",
                    ),
                    || dom_element.remove_attribute("translate").unwrap(),
                ),
            )
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "translate",
                ),
                || dom_element.remove_attribute("translate").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::virtual_keyboard_policy<V>
    {
        type State = super::props::virtual_keyboard_policy<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::virtual_keyboard_policy(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "virtualkeyboardpolicy",
                    ),
                    || dom_element.remove_attribute("virtualkeyboardpolicy").unwrap(),
                ),
            )
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "virtualkeyboardpolicy",
                ),
                || dom_element.remove_attribute("virtualkeyboardpolicy").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::Invalid>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_invalid<V>
    {
        type State = super::props::on_invalid<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_invalid(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::AnimationCancel>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_animation_cancel<V>
    {
        type State = super::props::on_animation_cancel<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_animation_cancel(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::AnimationEnd>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_animation_end<V>
    {
        type State = super::props::on_animation_end<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_animation_end(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::AnimationIteration>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_animation_iteration<V>
    {
        type State = super::props::on_animation_iteration<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_animation_iteration(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::AnimationStart>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_animation_start<V>
    {
        type State = super::props::on_animation_start<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_animation_start(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::BeforeInput>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_before_input<V>
    {
        type State = super::props::on_before_input<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_before_input(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::Input>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_input<V>
    {
        type State = super::props::on_input<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_input(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::Change>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_change<V>
    {
        type State = super::props::on_change<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_change(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::GotPointerCapture>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_got_pointer_capture<V>
    {
        type State = super::props::on_got_pointer_capture<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_got_pointer_capture(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::LostPointerCapture>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_lost_pointer_capture<V>
    {
        type State = super::props::on_lost_pointer_capture<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_lost_pointer_capture(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::PointerCancel>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_pointer_cancel<V>
    {
        type State = super::props::on_pointer_cancel<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_pointer_cancel(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::PointerDown>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_pointer_down<V>
    {
        type State = super::props::on_pointer_down<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_pointer_down(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::PointerEnter>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_pointer_enter<V>
    {
        type State = super::props::on_pointer_enter<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_pointer_enter(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::PointerLeave>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_pointer_leave<V>
    {
        type State = super::props::on_pointer_leave<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_pointer_leave(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::PointerMove>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_pointer_move<V>
    {
        type State = super::props::on_pointer_move<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_pointer_move(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::PointerOut>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_pointer_out<V>
    {
        type State = super::props::on_pointer_out<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_pointer_out(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::PointerOver>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_pointer_over<V>
    {
        type State = super::props::on_pointer_over<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_pointer_over(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::PointerUp>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_pointer_up<V>
    {
        type State = super::props::on_pointer_up<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_pointer_up(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::TransitionCancel>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_transition_cancel<V>
    {
        type State = super::props::on_transition_cancel<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_transition_cancel(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::TransitionEnd>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_transition_end<V>
    {
        type State = super::props::on_transition_end<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_transition_end(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::TransitionRun>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_transition_run<V>
    {
        type State = super::props::on_transition_run<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_transition_run(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::TransitionStart>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_transition_start<V>
    {
        type State = super::props::on_transition_start<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_transition_start(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::Drag>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_drag<V>
    {
        type State = super::props::on_drag<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_drag(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::DragEnd>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_drag_end<V>
    {
        type State = super::props::on_drag_end<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_drag_end(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::DragEnter>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_drag_enter<V>
    {
        type State = super::props::on_drag_enter<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_drag_enter(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::DragLeave>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_drag_leave<V>
    {
        type State = super::props::on_drag_leave<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_drag_leave(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::DragOver>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_drag_over<V>
    {
        type State = super::props::on_drag_over<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_drag_over(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::DragStart>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_drag_start<V>
    {
        type State = super::props::on_drag_start<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_drag_start(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::Drop>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_drop<V>
    {
        type State = super::props::on_drop<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_drop(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
}
#[cfg(feature = "ssr")]
mod impl_ssr_for_props {
    #![allow(unused_variables)]
    #[allow(unused_imports)]
    use super::super::*;
}
mod imports {
    #[allow(unused_imports)]
    use super::super::*;
    pub(super) use crate::imports::frender_html_simple::def_props_type;
}
use imports::def_props_type;
