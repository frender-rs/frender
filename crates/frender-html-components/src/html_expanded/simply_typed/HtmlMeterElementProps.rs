def_props_type!(
    #[derive(Debug, Clone, Copy, Default)]
    HtmlMeterElementProps(
        ..HtmlElementProps(
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
        ),
        value: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<f64>],
        min: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<f64>],
        max: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<f64>],
        low: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<f64>],
        high: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<f64>],
        optimum: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<f64>],
    )
);
#[cfg(feature = "dom")]
mod impl_dom_for_props {
    #![allow(unused_variables)]
    #[allow(unused_imports)]
    use super::super::*;
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<f64>,
            E: ::core::convert::AsRef<web_sys::HtmlMeterElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::value<V>
    {
        type State = super::props::value<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
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
            children_ctx: &mut crate::imports::frender_dom::Dom,
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
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E> for super::props::min<V>
    {
        type State = super::props::min<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
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
            children_ctx: &mut crate::imports::frender_dom::Dom,
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
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E> for super::props::max<V>
    {
        type State = super::props::max<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
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
            children_ctx: &mut crate::imports::frender_dom::Dom,
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
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E> for super::props::low<V>
    {
        type State = super::props::low<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
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
            children_ctx: &mut crate::imports::frender_dom::Dom,
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
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::high<V>
    {
        type State = super::props::high<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
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
            children_ctx: &mut crate::imports::frender_dom::Dom,
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
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::optimum<V>
    {
        type State = super::props::optimum<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
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
            children_ctx: &mut crate::imports::frender_dom::Dom,
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
