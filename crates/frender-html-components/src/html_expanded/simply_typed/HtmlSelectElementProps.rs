def_props_type!(
    #[derive(Debug, Clone, Copy, Default)]
    HtmlSelectElementProps(
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
        auto_complete: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        disabled: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>],
        form: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        multiple: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>],
        name: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        required: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>],
        size: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<u32>],
    )
);
#[cfg(feature = "csr")]
mod impl_dom_for_props {
    #![allow(unused_variables)]
    #[allow(unused_imports)]
    use super::super::*;
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlSelectElement>,
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
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
            E: ::core::convert::AsRef<web_sys::HtmlSelectElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::disabled<V>
    {
        type State = super::props::disabled<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::disabled(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_disabled(*v),
                    || dom_element.remove_attribute("disabled").unwrap(),
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
                |v| element.set_disabled(*v),
                || dom_element.remove_attribute("disabled").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlSelectElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::form<V>
    {
        type State = super::props::form<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::form(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "form",
                    ),
                    || dom_element.remove_attribute("form").unwrap(),
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
                    "form",
                ),
                || dom_element.remove_attribute("form").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
            E: ::core::convert::AsRef<web_sys::HtmlSelectElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::multiple<V>
    {
        type State = super::props::multiple<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::multiple(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_multiple(*v),
                    || dom_element.remove_attribute("multiple").unwrap(),
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
                |v| element.set_multiple(*v),
                || dom_element.remove_attribute("multiple").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlSelectElement>,
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
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
            E: ::core::convert::AsRef<web_sys::HtmlSelectElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::required<V>
    {
        type State = super::props::required<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::required(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_required(*v),
                    || dom_element.remove_attribute("required").unwrap(),
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
                |v| element.set_required(*v),
                || dom_element.remove_attribute("required").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<u32>,
            E: ::core::convert::AsRef<web_sys::HtmlSelectElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::size<V>
    {
        type State = super::props::size<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::size(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    u32,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_size(*v),
                    || dom_element.remove_attribute("size").unwrap(),
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
                u32,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| element.set_size(*v),
                || dom_element.remove_attribute("size").unwrap(),
            );
        }
    }
}
#[cfg(feature = "ssr")]
mod impl_ssr_for_props {
    #![allow(unused_variables)]
    #[allow(unused_imports)]
    use super::super::*;
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
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::disabled<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("disabled"),
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
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::form<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("form"),
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
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::multiple<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("multiple"),
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
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::required<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("required"),
                    attr_value
                        .map_or(
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue,
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::String,
                        ),
                ))
                .into_iter()
        }
    }
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<u32>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::size<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("size"),
                    attr_value
                        .map_or(
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue,
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::String,
                        ),
                ))
                .into_iter()
        }
    }
}
mod imports {
    #[allow(unused_imports)]
    use super::super::*;
    pub(super) use crate::imports::frender_html_simple::def_props_type;
}
use imports::def_props_type;
