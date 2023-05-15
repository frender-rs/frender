def_props_type!(
    #[derive(Debug, Clone, Copy, Default)]
    HtmlMeterElementProps(
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
        value: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<f64>],
        min: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<f64>],
        max: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<f64>],
        low: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<f64>],
        high: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<f64>],
        optimum: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<f64>],
    )
);
#[cfg(feature = "csr")]
mod impl_dom_for_props {
    #![allow(unused_variables)]
    #[allow(unused_imports)]
    use super::super::*;
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<f64>,
            E: ::core::convert::AsRef<web_sys::HtmlMeterElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::value<V>
    {
        type State = super::props::value<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::value(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    f64,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_value(*v),
                    || dom_element.remove_attribute("value").unwrap(),
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
                f64,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| element.set_value(*v),
                || dom_element.remove_attribute("value").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<f64>,
            E: ::core::convert::AsRef<web_sys::HtmlMeterElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E> for super::props::min<V>
    {
        type State = super::props::min<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::min(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    f64,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_min(*v),
                    || dom_element.remove_attribute("min").unwrap(),
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
                f64,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| element.set_min(*v),
                || dom_element.remove_attribute("min").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<f64>,
            E: ::core::convert::AsRef<web_sys::HtmlMeterElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E> for super::props::max<V>
    {
        type State = super::props::max<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::max(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    f64,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_max(*v),
                    || dom_element.remove_attribute("max").unwrap(),
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
                f64,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| element.set_max(*v),
                || dom_element.remove_attribute("max").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<f64>,
            E: ::core::convert::AsRef<web_sys::HtmlMeterElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E> for super::props::low<V>
    {
        type State = super::props::low<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::low(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    f64,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_low(*v),
                    || dom_element.remove_attribute("low").unwrap(),
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
                f64,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| element.set_low(*v),
                || dom_element.remove_attribute("low").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<f64>,
            E: ::core::convert::AsRef<web_sys::HtmlMeterElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::high<V>
    {
        type State = super::props::high<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::high(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    f64,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_high(*v),
                    || dom_element.remove_attribute("high").unwrap(),
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
                f64,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| element.set_high(*v),
                || dom_element.remove_attribute("high").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<f64>,
            E: ::core::convert::AsRef<web_sys::HtmlMeterElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::optimum<V>
    {
        type State = super::props::optimum<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::optimum(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    f64,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_optimum(*v),
                    || dom_element.remove_attribute("optimum").unwrap(),
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
                f64,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| element.set_optimum(*v),
                || dom_element.remove_attribute("optimum").unwrap(),
            );
        }
    }
}
#[cfg(feature = "ssr")]
mod impl_ssr_for_props {
    #![allow(unused_variables)]
    #[allow(unused_imports)]
    use super::super::*;
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<f64>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::value<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("value"),
                    attr_value
                        .map_or(
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue,
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::String,
                        ),
                ))
                .into_iter()
        }
    }
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<f64>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::min<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("min"),
                    attr_value
                        .map_or(
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue,
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::String,
                        ),
                ))
                .into_iter()
        }
    }
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<f64>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::max<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("max"),
                    attr_value
                        .map_or(
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue,
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::String,
                        ),
                ))
                .into_iter()
        }
    }
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<f64>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::low<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("low"),
                    attr_value
                        .map_or(
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue,
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::String,
                        ),
                ))
                .into_iter()
        }
    }
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<f64>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::high<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("high"),
                    attr_value
                        .map_or(
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue,
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::String,
                        ),
                ))
                .into_iter()
        }
    }
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<f64>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::optimum<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("optimum"),
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
