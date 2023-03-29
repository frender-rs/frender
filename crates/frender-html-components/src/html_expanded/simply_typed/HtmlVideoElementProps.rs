def_props_type!(
    #[derive(Debug, Clone, Copy, Default)]
    HtmlVideoElementProps(
        ..HtmlMediaElementProps(
            ..HtmlElementProps(
                ..ElementProps(
                    children,
                    class:
                        bounds![
                            crate::imports::frender_html::props::MaybeUpdateValueWithState<str>
                        ],
                    id: bounds![
                        crate::imports::frender_html::props::MaybeUpdateValueWithState<str>
                    ],
                    part: bounds![
                        crate::imports::frender_html::props::MaybeUpdateValueWithState<str>
                    ],
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
                hidden:
                    bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>],
                inert:
                    bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>],
                input_mode:
                    bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
                is: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
                item_id:
                    bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
                item_prop:
                    bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
                item_ref:
                    bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
                item_scope:
                    bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
                item_type:
                    bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
                lang: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
                nonce: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
                role: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
                slot: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
                spellcheck:
                    bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>],
                style: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
                tab_index:
                    bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<i32>],
                title: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
                translate:
                    bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
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
            auto_play:
                bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>],
            controls: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>],
            cross_origin:
                bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
            r#loop: alias![loop_]
                + bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>],
            muted: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>],
            preload: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
            src: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
            on_abort,
            on_can_play,
            on_can_play_through,
            on_duration_change,
            on_emptied,
            on_ended,
            on_loaded_data,
            on_loaded_metadata,
            on_load_start,
            on_pause,
            on_play,
            on_playing,
            on_progress,
            on_rate_change,
            on_resize,
            on_seeked,
            on_seeking,
            on_stalled,
            on_suspend,
            on_time_update,
            on_volume_change,
            on_waiting,
        ),
        height: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<u32>],
        plays_inline: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>],
        poster: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        width: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<u32>],
    )
);
#[cfg(feature = "csr")]
mod impl_dom_for_props {
    #![allow(unused_variables)]
    #[allow(unused_imports)]
    use super::super::*;
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<u32>,
            E: ::core::convert::AsRef<web_sys::HtmlVideoElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::height<V>
    {
        type State = super::props::height<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::height(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    u32,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_height(*v),
                    || dom_element.remove_attribute("height").unwrap(),
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
                |v| element.set_height(*v),
                || dom_element.remove_attribute("height").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
            E: ::core::convert::AsRef<web_sys::HtmlVideoElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::plays_inline<V>
    {
        type State = super::props::plays_inline<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::plays_inline(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
                        *v,
                        dom_element,
                        "playsinline",
                    ),
                    || dom_element.remove_attribute("playsinline").unwrap(),
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
                    "playsinline",
                ),
                || dom_element.remove_attribute("playsinline").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlVideoElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::poster<V>
    {
        type State = super::props::poster<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::poster(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_poster(v),
                    || dom_element.remove_attribute("poster").unwrap(),
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
                |v| element.set_poster(v),
                || dom_element.remove_attribute("poster").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<u32>,
            E: ::core::convert::AsRef<web_sys::HtmlVideoElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::width<V>
    {
        type State = super::props::width<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::width(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    u32,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_width(*v),
                    || dom_element.remove_attribute("width").unwrap(),
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
                |v| element.set_width(*v),
                || dom_element.remove_attribute("width").unwrap(),
            );
        }
    }
}
#[cfg(feature = "ssr")]
mod impl_ssr_for_props {
    #![allow(unused_variables)]
    #[allow(unused_imports)]
    use super::super::*;
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<u32>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::height<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("height"),
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
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::plays_inline<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("playsinline"),
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
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::poster<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("poster"),
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
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::width<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("width"),
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
