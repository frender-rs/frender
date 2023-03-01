def_props_type!(
    #[derive(Debug, Clone, Copy, Default)]
    HtmlIFrameElementProps[..HtmlElementProps[..ElementProps[children, class :
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
    on_drop,], allow : bounds![crate
    ::imports::frender_html::props::MaybeUpdateValueWithState < str >], allow_fullscreen
    : bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState < bool >],
    allow_payment_request : bounds![crate
    ::imports::frender_html::props::MaybeUpdateValueWithState < bool >], csp :
    bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState < str >],
    fetch_priority : bounds![crate
    ::imports::frender_html::props::MaybeUpdateValueWithState < str >], height :
    bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState < str >],
    loading : bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState <
    str >], name : bounds![crate
    ::imports::frender_html::props::MaybeUpdateValueWithState < str >], referrer_policy :
    bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState < str >],
    sandbox : bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState <
    str >], src : bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState
    < str >], src_doc : bounds![crate
    ::imports::frender_html::props::MaybeUpdateValueWithState < str >], width :
    bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState < str >],]
);
#[cfg(feature = "dom")]
mod impl_dom_for_props {
    #![allow(unused_variables)]
    #[allow(unused_imports)]
    use super::super::*;
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlIFrameElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::allow<V>
    {
        type State = super::props::allow<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::allow(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "allow",
                    ),
                    || dom_element.remove_attribute("allow").unwrap(),
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
                    "allow",
                ),
                || dom_element.remove_attribute("allow").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
            E: ::core::convert::AsRef<web_sys::HtmlIFrameElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::allow_fullscreen<V>
    {
        type State = super::props::allow_fullscreen<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::allow_fullscreen(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_allow_fullscreen(*v),
                    || dom_element.remove_attribute("allowfullscreen").unwrap(),
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
                |v| element.set_allow_fullscreen(*v),
                || dom_element.remove_attribute("allowfullscreen").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
            E: ::core::convert::AsRef<web_sys::HtmlIFrameElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::allow_payment_request<V>
    {
        type State = super::props::allow_payment_request<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::allow_payment_request(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_allow_payment_request(*v),
                    || dom_element.remove_attribute("allowpaymentrequest").unwrap(),
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
                |v| element.set_allow_payment_request(*v),
                || dom_element.remove_attribute("allowpaymentrequest").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlIFrameElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E> for super::props::csp<V>
    {
        type State = super::props::csp<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::csp(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "csp",
                    ),
                    || dom_element.remove_attribute("csp").unwrap(),
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
                    "csp",
                ),
                || dom_element.remove_attribute("csp").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlIFrameElement>,
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
            E: ::core::convert::AsRef<web_sys::HtmlIFrameElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::height<V>
    {
        type State = super::props::height<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::height(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_height(v),
                    || dom_element.remove_attribute("height").unwrap(),
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
                |v| element.set_height(v),
                || dom_element.remove_attribute("height").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlIFrameElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::loading<V>
    {
        type State = super::props::loading<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::loading(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "loading",
                    ),
                    || dom_element.remove_attribute("loading").unwrap(),
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
                    "loading",
                ),
                || dom_element.remove_attribute("loading").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlIFrameElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::name<V>
    {
        type State = super::props::name<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
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
                |v| element.set_name(v),
                || dom_element.remove_attribute("name").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlIFrameElement>,
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
            E: ::core::convert::AsRef<web_sys::HtmlIFrameElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::sandbox<V>
    {
        type State = super::props::sandbox<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::sandbox(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "sandbox",
                    ),
                    || dom_element.remove_attribute("sandbox").unwrap(),
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
                    "sandbox",
                ),
                || dom_element.remove_attribute("sandbox").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlIFrameElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E> for super::props::src<V>
    {
        type State = super::props::src<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::src(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_src(v),
                    || dom_element.remove_attribute("src").unwrap(),
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
                |v| element.set_src(v),
                || dom_element.remove_attribute("src").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlIFrameElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::src_doc<V>
    {
        type State = super::props::src_doc<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::src_doc(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_srcdoc(v),
                    || dom_element.remove_attribute("srcdoc").unwrap(),
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
                |v| element.set_srcdoc(v),
                || dom_element.remove_attribute("srcdoc").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlIFrameElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::width<V>
    {
        type State = super::props::width<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::width(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_width(v),
                    || dom_element.remove_attribute("width").unwrap(),
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
                |v| element.set_width(v),
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
}
mod imports {
    #[allow(unused_imports)]
    use super::super::*;
    pub(super) use crate::imports::frender_html_simple::def_props_type;
}
use imports::def_props_type;
