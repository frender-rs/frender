def_props_type!(
    #[derive(Debug, Clone, Copy, Default)]
    HtmlFormElementProps(
        ..HtmlElementProps(
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
                on_wheel:
                    bounds![crate::imports::frender_events::MaybeHandleEvent<events::WheelEvent>],
                on_copy: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
                on_cut: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
                on_paste: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
                on_composition_end:
                    bounds![
                        crate::imports::frender_events::MaybeHandleEvent<events::CompositionEvent>
                    ],
                on_composition_start:
                    bounds![
                        crate::imports::frender_events::MaybeHandleEvent<events::CompositionEvent>
                    ],
                on_composition_update:
                    bounds![
                        crate::imports::frender_events::MaybeHandleEvent<events::CompositionEvent>
                    ],
                on_blur:
                    bounds![crate::imports::frender_events::MaybeHandleEvent<events::FocusEvent>],
                on_focus:
                    bounds![crate::imports::frender_events::MaybeHandleEvent<events::FocusEvent>],
                on_focus_in:
                    bounds![crate::imports::frender_events::MaybeHandleEvent<events::FocusEvent>],
                on_focus_out:
                    bounds![crate::imports::frender_events::MaybeHandleEvent<events::FocusEvent>],
                on_fullscreen_change:
                    bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
                on_fullscreen_error:
                    bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
                on_key_down:
                    bounds![
                        crate::imports::frender_events::MaybeHandleEvent<events::KeyboardEvent>
                    ],
                on_key_up:
                    bounds![
                        crate::imports::frender_events::MaybeHandleEvent<events::KeyboardEvent>
                    ],
                on_aux_click:
                    bounds![crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>],
                on_click:
                    bounds![crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>],
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
            access_key:
                bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
            auto_capitalize:
                bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
            auto_focus:
                bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>],
            context_menu:
                bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
            dir: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
            draggable:
                bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>],
            enter_key_hint:
                bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
            hidden: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>],
            inert: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>],
            input_mode:
                bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
            is: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
            item_id: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
            item_prop: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
            item_ref: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
            item_scope:
                bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
            item_type: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
            lang: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
            nonce: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
            role: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
            slot: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
            spellcheck:
                bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>],
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
        ),
        accept: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        accept_charset:
            bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        auto_complete: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        name: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        rel: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        action: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        enc_type: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        method: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        no_validate: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>],
        target: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        on_form_data: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
        on_reset: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
        on_submit: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
    )
);
#[cfg(feature = "csr")]
mod impl_dom_for_props {
    #![allow(unused_variables)]
    #[allow(unused_imports)]
    use super::super::*;
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlFormElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::accept<V>
    {
        type State = super::props::accept<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::accept(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "accept",
                    ),
                    || dom_element.remove_attribute("accept").unwrap(),
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
                    "accept",
                ),
                || dom_element.remove_attribute("accept").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlFormElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::accept_charset<V>
    {
        type State = super::props::accept_charset<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::accept_charset(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_accept_charset(v),
                    || dom_element.remove_attribute("accept-charset").unwrap(),
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
                |v| element.set_accept_charset(v),
                || dom_element.remove_attribute("accept-charset").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlFormElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::auto_complete<V>
    {
        type State = super::props::auto_complete<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::auto_complete(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_autocomplete(v),
                    || dom_element.remove_attribute("autocomplete").unwrap(),
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
                |v| element.set_autocomplete(v),
                || dom_element.remove_attribute("autocomplete").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlFormElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::name<V>
    {
        type State = super::props::name<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::name(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_name(v),
                    || dom_element.remove_attribute("name").unwrap(),
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
                |v| element.set_name(v),
                || dom_element.remove_attribute("name").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlFormElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E> for super::props::rel<V>
    {
        type State = super::props::rel<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::rel(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "rel",
                    ),
                    || dom_element.remove_attribute("rel").unwrap(),
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
                    "rel",
                ),
                || dom_element.remove_attribute("rel").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlFormElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::action<V>
    {
        type State = super::props::action<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::action(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_action(v),
                    || dom_element.remove_attribute("action").unwrap(),
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
                |v| element.set_action(v),
                || dom_element.remove_attribute("action").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlFormElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::enc_type<V>
    {
        type State = super::props::enc_type<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::enc_type(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_enctype(v),
                    || dom_element.remove_attribute("enctype").unwrap(),
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
                |v| element.set_enctype(v),
                || dom_element.remove_attribute("enctype").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlFormElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::method<V>
    {
        type State = super::props::method<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::method(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_method(v),
                    || dom_element.remove_attribute("method").unwrap(),
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
                |v| element.set_method(v),
                || dom_element.remove_attribute("method").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
            E: ::core::convert::AsRef<web_sys::HtmlFormElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::no_validate<V>
    {
        type State = super::props::no_validate<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::no_validate(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_no_validate(*v),
                    || dom_element.remove_attribute("novalidate").unwrap(),
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
                |v| element.set_no_validate(*v),
                || dom_element.remove_attribute("novalidate").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlFormElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::target<V>
    {
        type State = super::props::target<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::target(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_target(v),
                    || dom_element.remove_attribute("target").unwrap(),
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
                |v| element.set_target(v),
                || dom_element.remove_attribute("target").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
            E: ::core::convert::AsRef<web_sys::HtmlFormElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_form_data<V>
    {
        type State = super::props::on_form_data<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_form_data(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "formdata",
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
                    "formdata",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
            E: ::core::convert::AsRef<web_sys::HtmlFormElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_reset<V>
    {
        type State = super::props::on_reset<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_reset(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "reset",
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
                    "reset",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
            E: ::core::convert::AsRef<web_sys::HtmlFormElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_submit<V>
    {
        type State = super::props::on_submit<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_submit(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "submit",
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
                    "submit",
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
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::accept<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("accept"),
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
        for super::props::accept_charset<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("accept-charset"),
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
        for super::props::auto_complete<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("autocomplete"),
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
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::name<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("name"),
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
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::rel<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("rel"),
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
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::action<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("action"),
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
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::enc_type<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("enctype"),
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
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::method<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("method"),
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
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::no_validate<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("novalidate"),
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
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::target<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("target"),
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
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_form_data<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_reset<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_submit<V>
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
