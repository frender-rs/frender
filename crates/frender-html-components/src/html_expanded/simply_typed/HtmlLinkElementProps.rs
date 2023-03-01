def_props_type!(
    #[derive(Debug, Clone, Copy, Default)]
    HtmlLinkElementProps[..HtmlElementProps[..ElementProps[children, class :
    bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState < str >], id
    : bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState < str >],
    part : bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState < str
    >], on_cancel, on_error, on_scroll, on_security_policy_violation, on_select,
    on_wheel, on_copy, on_cut, on_paste, on_composition_end, on_composition_start,
    on_composition_update, on_blur, on_focus, on_focus_in, on_focus_out,
    on_fullscreen_change, on_fullscreen_error, on_key_down, on_key_up, on_aux_click,
    on_click, on_context_menu, on_double_click, on_mouse_down, on_mouse_enter,
    on_mouse_leave, on_mouse_move, on_mouse_out, on_mouse_over, on_mouse_up,
    on_touch_cancel, on_touch_end, on_touch_move, on_touch_start,], access_key :
    bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState < str >],
    auto_capitalize : bounds![crate
    ::imports::frender_html::props::MaybeUpdateValueWithState < str >], auto_focus :
    bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState < bool >],
    context_menu : bounds![crate
    ::imports::frender_html::props::MaybeUpdateValueWithState < str >], dir :
    bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState < str >],
    draggable : bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState <
    bool >], enter_key_hint : bounds![crate
    ::imports::frender_html::props::MaybeUpdateValueWithState < str >], hidden :
    bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState < bool >],
    inert : bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState <
    bool >], input_mode : bounds![crate
    ::imports::frender_html::props::MaybeUpdateValueWithState < str >], is :
    bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState < str >],
    item_id : bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState <
    str >], item_prop : bounds![crate
    ::imports::frender_html::props::MaybeUpdateValueWithState < str >], item_ref :
    bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState < str >],
    item_scope : bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState
    < str >], item_type : bounds![crate
    ::imports::frender_html::props::MaybeUpdateValueWithState < str >], lang :
    bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState < str >],
    nonce : bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState < str
    >], role : bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState <
    str >], slot : bounds![crate
    ::imports::frender_html::props::MaybeUpdateValueWithState < str >], spellcheck :
    bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState < bool >],
    style : bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState < str
    >], tab_index : bounds![crate
    ::imports::frender_html::props::MaybeUpdateValueWithState < i32 >], title :
    bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState < str >],
    translate : bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState <
    str >], virtual_keyboard_policy : bounds![crate
    ::imports::frender_html::props::MaybeUpdateValueWithState < str >], on_invalid,
    on_animation_cancel, on_animation_end, on_animation_iteration, on_animation_start,
    on_before_input, on_input, on_change, on_got_pointer_capture,
    on_lost_pointer_capture, on_pointer_cancel, on_pointer_down, on_pointer_enter,
    on_pointer_leave, on_pointer_move, on_pointer_out, on_pointer_over, on_pointer_up,
    on_transition_cancel, on_transition_end, on_transition_run, on_transition_start,
    on_drag, on_drag_end, on_drag_enter, on_drag_leave, on_drag_over, on_drag_start,
    on_drop,], as_ : bounds![crate
    ::imports::frender_html::props::MaybeUpdateValueWithState < str >], cross_origin :
    bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState < str >],
    fetch_priority : bounds![crate
    ::imports::frender_html::props::MaybeUpdateValueWithState < str >], href :
    bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState < str >],
    href_lang : bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState <
    str >], image_sizes : bounds![crate
    ::imports::frender_html::props::MaybeUpdateValueWithState < str >], image_src_set :
    bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState < str >],
    integrity : bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState <
    str >], media : bounds![crate
    ::imports::frender_html::props::MaybeUpdateValueWithState < str >], prefetch :
    bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState < str >],
    referrer_policy : bounds![crate
    ::imports::frender_html::props::MaybeUpdateValueWithState < str >], rel :
    bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState < str >],
    sizes : bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState < str
    >], type_ : bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState <
    str >], blocking : bounds![crate
    ::imports::frender_html::props::MaybeUpdateValueWithState < str >],]
);
#[cfg(feature = "dom")]
mod impl_dom_for_props {
    #![allow(unused_variables)]
    #[allow(unused_imports)]
    use super::super::*;
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlLinkElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E> for super::props::as_<V>
    {
        type State = super::props::as_<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::as_(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_as(v),
                    || dom_element.remove_attribute("as").unwrap(),
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
                |v| element.set_as(v),
                || dom_element.remove_attribute("as").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlLinkElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::cross_origin<V>
    {
        type State = super::props::cross_origin<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::cross_origin(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    match element {
                        el => |v: &_| el.set_cross_origin(Some(v)),
                    },
                    match element {
                        el => || el.set_cross_origin(None),
                    },
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
                match element {
                    el => |v: &_| el.set_cross_origin(Some(v)),
                },
                match element {
                    el => || el.set_cross_origin(None),
                },
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlLinkElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::fetch_priority<V>
    {
        type State = super::props::fetch_priority<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::fetch_priority(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "fetchpriority",
                    ),
                    || dom_element.remove_attribute("fetchpriority").unwrap(),
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
                    "fetchpriority",
                ),
                || dom_element.remove_attribute("fetchpriority").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlLinkElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::href<V>
    {
        type State = super::props::href<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::href(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_href(v),
                    || dom_element.remove_attribute("href").unwrap(),
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
                |v| element.set_href(v),
                || dom_element.remove_attribute("href").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlLinkElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::href_lang<V>
    {
        type State = super::props::href_lang<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::href_lang(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_hreflang(v),
                    || dom_element.remove_attribute("hreflang").unwrap(),
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
                |v| element.set_hreflang(v),
                || dom_element.remove_attribute("hreflang").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlLinkElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::image_sizes<V>
    {
        type State = super::props::image_sizes<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::image_sizes(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "imagesizes",
                    ),
                    || dom_element.remove_attribute("imagesizes").unwrap(),
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
                    "imagesizes",
                ),
                || dom_element.remove_attribute("imagesizes").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlLinkElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::image_src_set<V>
    {
        type State = super::props::image_src_set<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::image_src_set(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "imagesrcset",
                    ),
                    || dom_element.remove_attribute("imagesrcset").unwrap(),
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
                    "imagesrcset",
                ),
                || dom_element.remove_attribute("imagesrcset").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlLinkElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::integrity<V>
    {
        type State = super::props::integrity<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::integrity(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_integrity(v),
                    || dom_element.remove_attribute("integrity").unwrap(),
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
                |v| element.set_integrity(v),
                || dom_element.remove_attribute("integrity").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlLinkElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::media<V>
    {
        type State = super::props::media<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::media(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_media(v),
                    || dom_element.remove_attribute("media").unwrap(),
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
                |v| element.set_media(v),
                || dom_element.remove_attribute("media").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlLinkElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::prefetch<V>
    {
        type State = super::props::prefetch<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::prefetch(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "prefetch",
                    ),
                    || dom_element.remove_attribute("prefetch").unwrap(),
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
                    "prefetch",
                ),
                || dom_element.remove_attribute("prefetch").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlLinkElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::referrer_policy<V>
    {
        type State = super::props::referrer_policy<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::referrer_policy(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_referrer_policy(v),
                    || dom_element.remove_attribute("referrerpolicy").unwrap(),
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
                |v| element.set_referrer_policy(v),
                || dom_element.remove_attribute("referrerpolicy").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlLinkElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E> for super::props::rel<V>
    {
        type State = super::props::rel<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::rel(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_rel(v),
                    || dom_element.remove_attribute("rel").unwrap(),
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
                |v| element.set_rel(v),
                || dom_element.remove_attribute("rel").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlLinkElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::sizes<V>
    {
        type State = super::props::sizes<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::sizes(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "sizes",
                    ),
                    || dom_element.remove_attribute("sizes").unwrap(),
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
                    "sizes",
                ),
                || dom_element.remove_attribute("sizes").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlLinkElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::type_<V>
    {
        type State = super::props::type_<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::type_(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_type(v),
                    || dom_element.remove_attribute("type").unwrap(),
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
                |v| element.set_type(v),
                || dom_element.remove_attribute("type").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlLinkElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::blocking<V>
    {
        type State = super::props::blocking<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::blocking(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "blocking",
                    ),
                    || dom_element.remove_attribute("blocking").unwrap(),
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
                    "blocking",
                ),
                || dom_element.remove_attribute("blocking").unwrap(),
            );
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
