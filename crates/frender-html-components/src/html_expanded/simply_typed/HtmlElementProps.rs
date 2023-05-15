def_props_type!(
    #[derive(Debug, Clone, Copy, Default)]
    HtmlElementProps(
        ..ElementProps(
            children,
            class: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
            id: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
            part: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
            on_cancel: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
            on_error: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
            on_scroll: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
            on_security_policy_violation:
                bounds![
                    crate::imports::frender_events::MaybeHandleEvent<
                        events::SecurityPolicyViolationEvent,
                    >
                ],
            on_select: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
            on_wheel: bounds![crate::imports::frender_events::MaybeHandleEvent<events::WheelEvent>],
            on_copy: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
            on_cut: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
            on_paste: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
            on_composition_end:
                bounds![crate::imports::frender_events::MaybeHandleEvent<events::CompositionEvent>],
            on_composition_start:
                bounds![crate::imports::frender_events::MaybeHandleEvent<events::CompositionEvent>],
            on_composition_update:
                bounds![crate::imports::frender_events::MaybeHandleEvent<events::CompositionEvent>],
            on_blur: bounds![crate::imports::frender_events::MaybeHandleEvent<events::FocusEvent>],
            on_focus: bounds![crate::imports::frender_events::MaybeHandleEvent<events::FocusEvent>],
            on_focus_in:
                bounds![crate::imports::frender_events::MaybeHandleEvent<events::FocusEvent>],
            on_focus_out:
                bounds![crate::imports::frender_events::MaybeHandleEvent<events::FocusEvent>],
            on_fullscreen_change:
                bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
            on_fullscreen_error:
                bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
            on_key_down:
                bounds![crate::imports::frender_events::MaybeHandleEvent<events::KeyboardEvent>],
            on_key_up:
                bounds![crate::imports::frender_events::MaybeHandleEvent<events::KeyboardEvent>],
            on_aux_click:
                bounds![crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>],
            on_click: bounds![crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>],
            on_context_menu:
                bounds![crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>],
            on_double_click:
                bounds![crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>],
            on_mouse_down:
                bounds![crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>],
            on_mouse_enter:
                bounds![crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>],
            on_mouse_leave:
                bounds![crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>],
            on_mouse_move:
                bounds![crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>],
            on_mouse_out:
                bounds![crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>],
            on_mouse_over:
                bounds![crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>],
            on_mouse_up:
                bounds![crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>],
            on_touch_cancel:
                bounds![crate::imports::frender_events::MaybeHandleEvent<events::TouchEvent>],
            on_touch_end:
                bounds![crate::imports::frender_events::MaybeHandleEvent<events::TouchEvent>],
            on_touch_move:
                bounds![crate::imports::frender_events::MaybeHandleEvent<events::TouchEvent>],
            on_touch_start:
                bounds![crate::imports::frender_events::MaybeHandleEvent<events::TouchEvent>],
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
        on_invalid: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
        on_animation_cancel:
            bounds![crate::imports::frender_events::MaybeHandleEvent<events::AnimationEvent>],
        on_animation_end:
            bounds![crate::imports::frender_events::MaybeHandleEvent<events::AnimationEvent>],
        on_animation_iteration:
            bounds![crate::imports::frender_events::MaybeHandleEvent<events::AnimationEvent>],
        on_animation_start:
            bounds![crate::imports::frender_events::MaybeHandleEvent<events::AnimationEvent>],
        on_before_input:
            bounds![crate::imports::frender_events::MaybeHandleEvent<events::InputEvent>],
        on_input: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
        on_change: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
        on_got_pointer_capture:
            bounds![crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>],
        on_lost_pointer_capture:
            bounds![crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>],
        on_pointer_cancel:
            bounds![crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>],
        on_pointer_down:
            bounds![crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>],
        on_pointer_enter:
            bounds![crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>],
        on_pointer_leave:
            bounds![crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>],
        on_pointer_move:
            bounds![crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>],
        on_pointer_out:
            bounds![crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>],
        on_pointer_over:
            bounds![crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>],
        on_pointer_up:
            bounds![crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>],
        on_transition_cancel:
            bounds![crate::imports::frender_events::MaybeHandleEvent<events::TransitionEvent>],
        on_transition_end:
            bounds![crate::imports::frender_events::MaybeHandleEvent<events::TransitionEvent>],
        on_transition_run:
            bounds![crate::imports::frender_events::MaybeHandleEvent<events::TransitionEvent>],
        on_transition_start:
            bounds![crate::imports::frender_events::MaybeHandleEvent<events::TransitionEvent>],
        on_drag: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
        on_drag_end: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
        on_drag_enter: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
        on_drag_leave: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
        on_drag_over: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
        on_drag_start: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
        on_drop: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
    )
);
#[cfg(feature = "csr")]
mod impl_dom_for_props {
    #![allow(unused_variables)]
    #[allow(unused_imports)]
    use super::super::*;
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::access_key<V>
    {
        type State = super::props::access_key<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
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
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
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
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::auto_capitalize<V>
    {
        type State = super::props::auto_capitalize<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::auto_capitalize(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
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
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
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
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::auto_focus<V>
    {
        type State = super::props::auto_focus<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::auto_focus(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
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
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                bool,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
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
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::context_menu<V>
    {
        type State = super::props::context_menu<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::context_menu(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
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
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
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
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E> for super::props::dir<V>
    {
        type State = super::props::dir<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
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
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
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
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::draggable<V>
    {
        type State = super::props::draggable<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
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
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
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
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::enter_key_hint<V>
    {
        type State = super::props::enter_key_hint<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::enter_key_hint(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
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
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
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
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::hidden<V>
    {
        type State = super::props::hidden<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
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
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
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
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::inert<V>
    {
        type State = super::props::inert<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::inert(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
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
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                bool,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
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
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::input_mode<V>
    {
        type State = super::props::input_mode<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::input_mode(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
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
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
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
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E> for super::props::is<V>
    {
        type State = super::props::is<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::is(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
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
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
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
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::item_id<V>
    {
        type State = super::props::item_id<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::item_id(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
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
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
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
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::item_prop<V>
    {
        type State = super::props::item_prop<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::item_prop(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
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
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
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
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::item_ref<V>
    {
        type State = super::props::item_ref<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::item_ref(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
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
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
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
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::item_scope<V>
    {
        type State = super::props::item_scope<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::item_scope(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
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
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
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
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::item_type<V>
    {
        type State = super::props::item_type<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::item_type(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
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
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
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
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::lang<V>
    {
        type State = super::props::lang<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
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
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
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
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::nonce<V>
    {
        type State = super::props::nonce<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::nonce(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
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
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
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
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::role<V>
    {
        type State = super::props::role<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::role(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
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
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
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
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::slot<V>
    {
        type State = super::props::slot<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::slot(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
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
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
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
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::spellcheck<V>
    {
        type State = super::props::spellcheck<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
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
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
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
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::style<V>
    {
        type State = super::props::style<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::style(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
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
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
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
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::tab_index<V>
    {
        type State = super::props::tab_index<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
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
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
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
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::title<V>
    {
        type State = super::props::title<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
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
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
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
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::translate<V>
    {
        type State = super::props::translate<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::translate(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
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
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
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
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::virtual_keyboard_policy<V>
    {
        type State = super::props::virtual_keyboard_policy<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::virtual_keyboard_policy(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
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
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "virtualkeyboardpolicy",
                ),
                || dom_element.remove_attribute("virtualkeyboardpolicy").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_invalid<V>
    {
        type State = super::props::on_invalid<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_invalid(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "invalid",
                    callable.clone(),
                )
            }))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_handle_event_state(this.0, &mut state.get_mut().0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "invalid",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::AnimationEvent>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_animation_cancel<V>
    {
        type State = super::props::on_animation_cancel<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_animation_cancel(V::initialize_handle_event_state(
                this.0,
                |callable| {
                    crate::imports::frender_events::EventListener::new(
                        dom_element,
                        "animationcancel",
                        callable.clone(),
                    )
                },
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_handle_event_state(this.0, &mut state.get_mut().0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "animationcancel",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::AnimationEvent>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_animation_end<V>
    {
        type State = super::props::on_animation_end<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_animation_end(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "animationend",
                    callable.clone(),
                )
            }))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_handle_event_state(this.0, &mut state.get_mut().0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "animationend",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::AnimationEvent>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_animation_iteration<V>
    {
        type State = super::props::on_animation_iteration<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_animation_iteration(V::initialize_handle_event_state(
                this.0,
                |callable| {
                    crate::imports::frender_events::EventListener::new(
                        dom_element,
                        "animationiteration",
                        callable.clone(),
                    )
                },
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_handle_event_state(this.0, &mut state.get_mut().0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "animationiteration",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::AnimationEvent>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_animation_start<V>
    {
        type State = super::props::on_animation_start<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_animation_start(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "animationstart",
                    callable.clone(),
                )
            }))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_handle_event_state(this.0, &mut state.get_mut().0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "animationstart",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::InputEvent>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_before_input<V>
    {
        type State = super::props::on_before_input<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_before_input(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "beforeinput",
                    callable.clone(),
                )
            }))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_handle_event_state(this.0, &mut state.get_mut().0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "beforeinput",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_input<V>
    {
        type State = super::props::on_input<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_input(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "input",
                    callable.clone(),
                )
            }))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_handle_event_state(this.0, &mut state.get_mut().0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "input",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_change<V>
    {
        type State = super::props::on_change<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_change(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "change",
                    callable.clone(),
                )
            }))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_handle_event_state(this.0, &mut state.get_mut().0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "change",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_got_pointer_capture<V>
    {
        type State = super::props::on_got_pointer_capture<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_got_pointer_capture(V::initialize_handle_event_state(
                this.0,
                |callable| {
                    crate::imports::frender_events::EventListener::new(
                        dom_element,
                        "gotpointercapture",
                        callable.clone(),
                    )
                },
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_handle_event_state(this.0, &mut state.get_mut().0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "gotpointercapture",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_lost_pointer_capture<V>
    {
        type State = super::props::on_lost_pointer_capture<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_lost_pointer_capture(V::initialize_handle_event_state(
                this.0,
                |callable| {
                    crate::imports::frender_events::EventListener::new(
                        dom_element,
                        "lostpointercapture",
                        callable.clone(),
                    )
                },
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_handle_event_state(this.0, &mut state.get_mut().0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "lostpointercapture",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_pointer_cancel<V>
    {
        type State = super::props::on_pointer_cancel<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_pointer_cancel(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "pointercancel",
                    callable.clone(),
                )
            }))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_handle_event_state(this.0, &mut state.get_mut().0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "pointercancel",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_pointer_down<V>
    {
        type State = super::props::on_pointer_down<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_pointer_down(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "pointerdown",
                    callable.clone(),
                )
            }))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_handle_event_state(this.0, &mut state.get_mut().0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "pointerdown",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_pointer_enter<V>
    {
        type State = super::props::on_pointer_enter<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_pointer_enter(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "pointerenter",
                    callable.clone(),
                )
            }))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_handle_event_state(this.0, &mut state.get_mut().0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "pointerenter",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_pointer_leave<V>
    {
        type State = super::props::on_pointer_leave<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_pointer_leave(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "pointerleave",
                    callable.clone(),
                )
            }))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_handle_event_state(this.0, &mut state.get_mut().0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "pointerleave",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_pointer_move<V>
    {
        type State = super::props::on_pointer_move<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_pointer_move(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "pointermove",
                    callable.clone(),
                )
            }))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_handle_event_state(this.0, &mut state.get_mut().0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "pointermove",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_pointer_out<V>
    {
        type State = super::props::on_pointer_out<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_pointer_out(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "pointerout",
                    callable.clone(),
                )
            }))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_handle_event_state(this.0, &mut state.get_mut().0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "pointerout",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_pointer_over<V>
    {
        type State = super::props::on_pointer_over<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_pointer_over(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "pointerover",
                    callable.clone(),
                )
            }))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_handle_event_state(this.0, &mut state.get_mut().0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "pointerover",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_pointer_up<V>
    {
        type State = super::props::on_pointer_up<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_pointer_up(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "pointerup",
                    callable.clone(),
                )
            }))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_handle_event_state(this.0, &mut state.get_mut().0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "pointerup",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::TransitionEvent>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_transition_cancel<V>
    {
        type State = super::props::on_transition_cancel<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_transition_cancel(V::initialize_handle_event_state(
                this.0,
                |callable| {
                    crate::imports::frender_events::EventListener::new(
                        dom_element,
                        "transitioncancel",
                        callable.clone(),
                    )
                },
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_handle_event_state(this.0, &mut state.get_mut().0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "transitioncancel",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::TransitionEvent>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_transition_end<V>
    {
        type State = super::props::on_transition_end<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_transition_end(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "transitionend",
                    callable.clone(),
                )
            }))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_handle_event_state(this.0, &mut state.get_mut().0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "transitionend",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::TransitionEvent>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_transition_run<V>
    {
        type State = super::props::on_transition_run<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_transition_run(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "transitionrun",
                    callable.clone(),
                )
            }))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_handle_event_state(this.0, &mut state.get_mut().0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "transitionrun",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::TransitionEvent>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_transition_start<V>
    {
        type State = super::props::on_transition_start<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_transition_start(V::initialize_handle_event_state(
                this.0,
                |callable| {
                    crate::imports::frender_events::EventListener::new(
                        dom_element,
                        "transitionstart",
                        callable.clone(),
                    )
                },
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_handle_event_state(this.0, &mut state.get_mut().0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "transitionstart",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_drag<V>
    {
        type State = super::props::on_drag<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_drag(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "drag",
                    callable.clone(),
                )
            }))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_handle_event_state(this.0, &mut state.get_mut().0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "drag",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_drag_end<V>
    {
        type State = super::props::on_drag_end<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_drag_end(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "dragend",
                    callable.clone(),
                )
            }))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_handle_event_state(this.0, &mut state.get_mut().0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "dragend",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_drag_enter<V>
    {
        type State = super::props::on_drag_enter<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_drag_enter(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "dragenter",
                    callable.clone(),
                )
            }))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_handle_event_state(this.0, &mut state.get_mut().0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "dragenter",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_drag_leave<V>
    {
        type State = super::props::on_drag_leave<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_drag_leave(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "dragleave",
                    callable.clone(),
                )
            }))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_handle_event_state(this.0, &mut state.get_mut().0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "dragleave",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_drag_over<V>
    {
        type State = super::props::on_drag_over<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_drag_over(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "dragover",
                    callable.clone(),
                )
            }))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_handle_event_state(this.0, &mut state.get_mut().0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "dragover",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_drag_start<V>
    {
        type State = super::props::on_drag_start<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_drag_start(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "dragstart",
                    callable.clone(),
                )
            }))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_handle_event_state(this.0, &mut state.get_mut().0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "dragstart",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_drop<V>
    {
        type State = super::props::on_drop<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_drop(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "drop",
                    callable.clone(),
                )
            }))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_handle_event_state(this.0, &mut state.get_mut().0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "drop",
                    callable.clone(),
                )
            })
        }
    }
}
#[cfg(feature = "ssr")]
mod impl_ssr_for_props {
    #![allow(unused_variables)]
    #[allow(unused_imports)]
    use super::super::*;
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::access_key<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("accesskey"),
                    attr_value
                        .map_or(
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue,
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::String,
                        ),
                ))
                .into_iter()
        }
    }
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::auto_capitalize<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("autocapitalize"),
                    attr_value
                        .map_or(
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue,
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::String,
                        ),
                ))
                .into_iter()
        }
    }
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::auto_focus<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("autofocus"),
                    attr_value
                        .map_or(
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue,
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::String,
                        ),
                ))
                .into_iter()
        }
    }
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::context_menu<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("contextmenu"),
                    attr_value
                        .map_or(
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue,
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::String,
                        ),
                ))
                .into_iter()
        }
    }
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::dir<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("dir"),
                    attr_value
                        .map_or(
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue,
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::String,
                        ),
                ))
                .into_iter()
        }
    }
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::draggable<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("draggable"),
                    attr_value
                        .map_or(
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue,
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::String,
                        ),
                ))
                .into_iter()
        }
    }
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::enter_key_hint<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("enterkeyhint"),
                    attr_value
                        .map_or(
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue,
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::String,
                        ),
                ))
                .into_iter()
        }
    }
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::hidden<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("hidden"),
                    attr_value
                        .map_or(
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue,
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::String,
                        ),
                ))
                .into_iter()
        }
    }
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::inert<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("inert"),
                    attr_value
                        .map_or(
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue,
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::String,
                        ),
                ))
                .into_iter()
        }
    }
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::input_mode<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("inputmode"),
                    attr_value
                        .map_or(
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue,
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::String,
                        ),
                ))
                .into_iter()
        }
    }
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::is<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("is"),
                    attr_value
                        .map_or(
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue,
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::String,
                        ),
                ))
                .into_iter()
        }
    }
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::item_id<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("itemid"),
                    attr_value
                        .map_or(
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue,
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::String,
                        ),
                ))
                .into_iter()
        }
    }
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::item_prop<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("itemprop"),
                    attr_value
                        .map_or(
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue,
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::String,
                        ),
                ))
                .into_iter()
        }
    }
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::item_ref<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("itemref"),
                    attr_value
                        .map_or(
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue,
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::String,
                        ),
                ))
                .into_iter()
        }
    }
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::item_scope<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("itemscope"),
                    attr_value
                        .map_or(
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue,
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::String,
                        ),
                ))
                .into_iter()
        }
    }
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::item_type<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("itemtype"),
                    attr_value
                        .map_or(
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue,
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::String,
                        ),
                ))
                .into_iter()
        }
    }
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::lang<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("lang"),
                    attr_value
                        .map_or(
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue,
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::String,
                        ),
                ))
                .into_iter()
        }
    }
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::nonce<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("nonce"),
                    attr_value
                        .map_or(
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue,
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::String,
                        ),
                ))
                .into_iter()
        }
    }
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::role<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("role"),
                    attr_value
                        .map_or(
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue,
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::String,
                        ),
                ))
                .into_iter()
        }
    }
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::slot<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("slot"),
                    attr_value
                        .map_or(
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue,
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::String,
                        ),
                ))
                .into_iter()
        }
    }
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::spellcheck<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("spellcheck"),
                    attr_value
                        .map_or(
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue,
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::String,
                        ),
                ))
                .into_iter()
        }
    }
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::style<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("style"),
                    attr_value
                        .map_or(
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue,
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::String,
                        ),
                ))
                .into_iter()
        }
    }
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<i32>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::tab_index<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("tabindex"),
                    attr_value
                        .map_or(
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue,
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::String,
                        ),
                ))
                .into_iter()
        }
    }
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::title<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("title"),
                    attr_value
                        .map_or(
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue,
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::String,
                        ),
                ))
                .into_iter()
        }
    }
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::translate<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("translate"),
                    attr_value
                        .map_or(
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue,
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::String,
                        ),
                ))
                .into_iter()
        }
    }
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::virtual_keyboard_policy<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("virtualkeyboardpolicy"),
                    attr_value
                        .map_or(
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue,
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::String,
                        ),
                ))
                .into_iter()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_invalid<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::AnimationEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_animation_cancel<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::AnimationEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_animation_end<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::AnimationEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_animation_iteration<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::AnimationEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_animation_start<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::InputEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_before_input<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_input<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_change<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_got_pointer_capture<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_lost_pointer_capture<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_pointer_cancel<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_pointer_down<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_pointer_enter<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_pointer_leave<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_pointer_move<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_pointer_out<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_pointer_over<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_pointer_up<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::TransitionEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_transition_cancel<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::TransitionEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_transition_end<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::TransitionEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_transition_run<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::TransitionEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_transition_start<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_drag<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_drag_end<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_drag_enter<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_drag_leave<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_drag_over<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_drag_start<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_drop<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
}
mod imports {
    #[allow(unused_imports)]
    use super::super::*;
    pub(super) use crate::imports::frender_html_simple::def_props_type;
}
use imports::def_props_type;
