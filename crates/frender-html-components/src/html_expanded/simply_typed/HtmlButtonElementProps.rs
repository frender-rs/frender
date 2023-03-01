def_props_type!(
    #[derive(Debug, Clone, Copy, Default)]
    HtmlButtonElementProps[..HtmlElementProps[..ElementProps[children, class :
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
    on_drop,], disabled : bounds![crate
    ::imports::frender_html::props::MaybeUpdateValueWithState < bool >], form :
    bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState < str >],
    form_action : bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState
    < str >], form_enc_type : bounds![crate
    ::imports::frender_html::props::MaybeUpdateValueWithState < str >], form_method :
    bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState < str >],
    form_no_validate : bounds![crate
    ::imports::frender_html::props::MaybeUpdateValueWithState < bool >], form_target :
    bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState < str >],
    name : bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState < str
    >], type_ : bounds![crate ::imports::frender_html::props::MaybeUpdateValueWithState <
    str >], value : bounds![crate
    ::imports::frender_html::props::MaybeUpdateValueWithState < str >],]
);
#[cfg(feature = "dom")]
mod impl_dom_for_props {
    #![allow(unused_variables)]
    #[allow(unused_imports)]
    use super::super::*;
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
            E: ::core::convert::AsRef<web_sys::HtmlButtonElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::disabled<V>
    {
        type State = super::props::disabled<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
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
                |v| element.set_disabled(*v),
                || dom_element.remove_attribute("disabled").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlButtonElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::form<V>
    {
        type State = super::props::form<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::form(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
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
                    "form",
                ),
                || dom_element.remove_attribute("form").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlButtonElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::form_action<V>
    {
        type State = super::props::form_action<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::form_action(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_form_action(v),
                    || dom_element.remove_attribute("formaction").unwrap(),
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
                |v| element.set_form_action(v),
                || dom_element.remove_attribute("formaction").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlButtonElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::form_enc_type<V>
    {
        type State = super::props::form_enc_type<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::form_enc_type(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_form_enctype(v),
                    || dom_element.remove_attribute("formenctype").unwrap(),
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
                |v| element.set_form_enctype(v),
                || dom_element.remove_attribute("formenctype").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlButtonElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::form_method<V>
    {
        type State = super::props::form_method<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::form_method(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_form_method(v),
                    || dom_element.remove_attribute("formmethod").unwrap(),
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
                |v| element.set_form_method(v),
                || dom_element.remove_attribute("formmethod").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
            E: ::core::convert::AsRef<web_sys::HtmlButtonElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::form_no_validate<V>
    {
        type State = super::props::form_no_validate<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::form_no_validate(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_form_no_validate(*v),
                    || dom_element.remove_attribute("formnovalidate").unwrap(),
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
                |v| element.set_form_no_validate(*v),
                || dom_element.remove_attribute("formnovalidate").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlButtonElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::form_target<V>
    {
        type State = super::props::form_target<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::form_target(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_form_target(v),
                    || dom_element.remove_attribute("formtarget").unwrap(),
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
                |v| element.set_form_target(v),
                || dom_element.remove_attribute("formtarget").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlButtonElement>,
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
            E: ::core::convert::AsRef<web_sys::HtmlButtonElement>,
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
            E: ::core::convert::AsRef<web_sys::HtmlButtonElement>,
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
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_value(v),
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
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| element.set_value(v),
                || dom_element.remove_attribute("value").unwrap(),
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
