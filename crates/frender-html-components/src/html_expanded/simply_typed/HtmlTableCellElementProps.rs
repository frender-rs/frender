def_props_type!(
    #[derive(Debug, Clone, Copy, Default)]
    HtmlTableCellElementProps(
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
        align: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        bg_color: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        char: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        char_off: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        v_align: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        col_span: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<u32>],
        headers: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        row_span: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<u32>],
        abbr: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        axis: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        height: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        scope: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        width: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
    )
);
#[cfg(feature = "csr")]
mod impl_dom_for_props {
    #![allow(unused_variables)]
    #[allow(unused_imports)]
    use super::super::*;
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlTableCellElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::align<V>
    {
        type State = super::props::align<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::align(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_align(v),
                    || dom_element.remove_attribute("align").unwrap(),
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
                |v| element.set_align(v),
                || dom_element.remove_attribute("align").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlTableCellElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::bg_color<V>
    {
        type State = super::props::bg_color<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::bg_color(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "bgcolor",
                    ),
                    || dom_element.remove_attribute("bgcolor").unwrap(),
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
                    "bgcolor",
                ),
                || dom_element.remove_attribute("bgcolor").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlTableCellElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::char<V>
    {
        type State = super::props::char<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::char(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_ch(v),
                    || dom_element.remove_attribute("char").unwrap(),
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
                |v| element.set_ch(v),
                || dom_element.remove_attribute("char").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlTableCellElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::char_off<V>
    {
        type State = super::props::char_off<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::char_off(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_ch_off(v),
                    || dom_element.remove_attribute("charoff").unwrap(),
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
                |v| element.set_ch_off(v),
                || dom_element.remove_attribute("charoff").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlTableCellElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::v_align<V>
    {
        type State = super::props::v_align<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::v_align(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_v_align(v),
                    || dom_element.remove_attribute("valign").unwrap(),
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
                |v| element.set_v_align(v),
                || dom_element.remove_attribute("valign").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<u32>,
            E: ::core::convert::AsRef<web_sys::HtmlTableCellElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::col_span<V>
    {
        type State = super::props::col_span<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::col_span(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    u32,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_col_span(*v),
                    || dom_element.remove_attribute("colspan").unwrap(),
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
                |v| element.set_col_span(*v),
                || dom_element.remove_attribute("colspan").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlTableCellElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::headers<V>
    {
        type State = super::props::headers<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::headers(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_headers(v),
                    || dom_element.remove_attribute("headers").unwrap(),
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
                |v| element.set_headers(v),
                || dom_element.remove_attribute("headers").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<u32>,
            E: ::core::convert::AsRef<web_sys::HtmlTableCellElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::row_span<V>
    {
        type State = super::props::row_span<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::row_span(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    u32,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_row_span(*v),
                    || dom_element.remove_attribute("rowspan").unwrap(),
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
                |v| element.set_row_span(*v),
                || dom_element.remove_attribute("rowspan").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlTableCellElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::abbr<V>
    {
        type State = super::props::abbr<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::abbr(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "abbr",
                    ),
                    || dom_element.remove_attribute("abbr").unwrap(),
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
                    "abbr",
                ),
                || dom_element.remove_attribute("abbr").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlTableCellElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::axis<V>
    {
        type State = super::props::axis<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::axis(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_axis(v),
                    || dom_element.remove_attribute("axis").unwrap(),
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
                |v| element.set_axis(v),
                || dom_element.remove_attribute("axis").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlTableCellElement>,
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
                |v| element.set_height(v),
                || dom_element.remove_attribute("height").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlTableCellElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::scope<V>
    {
        type State = super::props::scope<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::scope(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "scope",
                    ),
                    || dom_element.remove_attribute("scope").unwrap(),
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
                    "scope",
                ),
                || dom_element.remove_attribute("scope").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlTableCellElement>,
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
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::align<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("align"),
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
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::bg_color<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("bgcolor"),
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
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::char<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("char"),
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
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::char_off<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("charoff"),
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
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::v_align<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("valign"),
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
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::col_span<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("colspan"),
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
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::headers<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("headers"),
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
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::row_span<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("rowspan"),
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
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::abbr<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("abbr"),
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
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::axis<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("axis"),
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
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::scope<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("scope"),
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
