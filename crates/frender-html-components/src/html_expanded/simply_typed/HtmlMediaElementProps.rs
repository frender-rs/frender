def_props_type!(
    #[derive(Debug, Clone, Copy, Default)]
    HtmlMediaElementProps(
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
        auto_play: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>],
        controls: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>],
        cross_origin: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        r#loop: alias![loop_]
            + bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>],
        muted: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>],
        preload: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        src: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
        on_abort: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
        on_can_play: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
        on_can_play_through:
            bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
        on_duration_change:
            bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
        on_emptied: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
        on_ended: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
        on_loaded_data: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
        on_loaded_metadata:
            bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
        on_load_start: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
        on_pause: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
        on_play: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
        on_playing: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
        on_progress: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
        on_rate_change: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
        on_resize: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
        on_seeked: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
        on_seeking: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
        on_stalled: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
        on_suspend: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
        on_time_update: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
        on_volume_change: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
        on_waiting: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
    )
);
#[cfg(feature = "csr")]
mod impl_dom_for_props {
    #![allow(unused_variables)]
    #[allow(unused_imports)]
    use super::super::*;
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
            E: ::core::convert::AsRef<web_sys::HtmlMediaElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::auto_play<V>
    {
        type State = super::props::auto_play<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::auto_play(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_autoplay(*v),
                    || dom_element.remove_attribute("autoplay").unwrap(),
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
                |v| element.set_autoplay(*v),
                || dom_element.remove_attribute("autoplay").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
            E: ::core::convert::AsRef<web_sys::HtmlMediaElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::controls<V>
    {
        type State = super::props::controls<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::controls(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_controls(*v),
                    || dom_element.remove_attribute("controls").unwrap(),
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
                |v| element.set_controls(*v),
                || dom_element.remove_attribute("controls").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlMediaElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::cross_origin<V>
    {
        type State = super::props::cross_origin<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
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
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
            E: ::core::convert::AsRef<web_sys::HtmlMediaElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::r#loop<V>
    {
        type State = super::props::r#loop<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::r#loop(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_loop(*v),
                    || dom_element.remove_attribute("loop").unwrap(),
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
                |v| element.set_loop(*v),
                || dom_element.remove_attribute("loop").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
            E: ::core::convert::AsRef<web_sys::HtmlMediaElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::muted<V>
    {
        type State = super::props::muted<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::muted(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_muted(*v),
                    || dom_element.remove_attribute("muted").unwrap(),
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
                |v| element.set_muted(*v),
                || dom_element.remove_attribute("muted").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlMediaElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::preload<V>
    {
        type State = super::props::preload<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::preload(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_preload(v),
                    || dom_element.remove_attribute("preload").unwrap(),
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
                |v| element.set_preload(v),
                || dom_element.remove_attribute("preload").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlMediaElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E> for super::props::src<V>
    {
        type State = super::props::src<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
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
                |v| element.set_src(v),
                || dom_element.remove_attribute("src").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
            E: ::core::convert::AsRef<web_sys::HtmlMediaElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_abort<V>
    {
        type State = super::props::on_abort<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_abort(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "abort",
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
                    "abort",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
            E: ::core::convert::AsRef<web_sys::HtmlMediaElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_can_play<V>
    {
        type State = super::props::on_can_play<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_can_play(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "canplay",
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
                    "canplay",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
            E: ::core::convert::AsRef<web_sys::HtmlMediaElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_can_play_through<V>
    {
        type State = super::props::on_can_play_through<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_can_play_through(V::initialize_handle_event_state(
                this.0,
                |callable| {
                    crate::imports::frender_events::EventListener::new(
                        dom_element,
                        "canplaythrough",
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
                    "canplaythrough",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
            E: ::core::convert::AsRef<web_sys::HtmlMediaElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_duration_change<V>
    {
        type State = super::props::on_duration_change<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_duration_change(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "durationchange",
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
                    "durationchange",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
            E: ::core::convert::AsRef<web_sys::HtmlMediaElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_emptied<V>
    {
        type State = super::props::on_emptied<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_emptied(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "emptied",
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
                    "emptied",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
            E: ::core::convert::AsRef<web_sys::HtmlMediaElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_ended<V>
    {
        type State = super::props::on_ended<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_ended(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "ended",
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
                    "ended",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
            E: ::core::convert::AsRef<web_sys::HtmlMediaElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_loaded_data<V>
    {
        type State = super::props::on_loaded_data<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_loaded_data(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "loadeddata",
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
                    "loadeddata",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
            E: ::core::convert::AsRef<web_sys::HtmlMediaElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_loaded_metadata<V>
    {
        type State = super::props::on_loaded_metadata<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_loaded_metadata(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "loadedmetadata",
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
                    "loadedmetadata",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
            E: ::core::convert::AsRef<web_sys::HtmlMediaElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_load_start<V>
    {
        type State = super::props::on_load_start<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_load_start(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "loadstart",
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
                    "loadstart",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
            E: ::core::convert::AsRef<web_sys::HtmlMediaElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_pause<V>
    {
        type State = super::props::on_pause<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_pause(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "pause",
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
                    "pause",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
            E: ::core::convert::AsRef<web_sys::HtmlMediaElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_play<V>
    {
        type State = super::props::on_play<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_play(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "play",
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
                    "play",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
            E: ::core::convert::AsRef<web_sys::HtmlMediaElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_playing<V>
    {
        type State = super::props::on_playing<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_playing(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "playing",
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
                    "playing",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
            E: ::core::convert::AsRef<web_sys::HtmlMediaElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_progress<V>
    {
        type State = super::props::on_progress<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_progress(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "progress",
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
                    "progress",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
            E: ::core::convert::AsRef<web_sys::HtmlMediaElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_rate_change<V>
    {
        type State = super::props::on_rate_change<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_rate_change(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "ratechange",
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
                    "ratechange",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
            E: ::core::convert::AsRef<web_sys::HtmlMediaElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_resize<V>
    {
        type State = super::props::on_resize<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_resize(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "resize",
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
                    "resize",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
            E: ::core::convert::AsRef<web_sys::HtmlMediaElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_seeked<V>
    {
        type State = super::props::on_seeked<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_seeked(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "seeked",
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
                    "seeked",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
            E: ::core::convert::AsRef<web_sys::HtmlMediaElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_seeking<V>
    {
        type State = super::props::on_seeking<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_seeking(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "seeking",
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
                    "seeking",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
            E: ::core::convert::AsRef<web_sys::HtmlMediaElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_stalled<V>
    {
        type State = super::props::on_stalled<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_stalled(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "stalled",
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
                    "stalled",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
            E: ::core::convert::AsRef<web_sys::HtmlMediaElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_suspend<V>
    {
        type State = super::props::on_suspend<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_suspend(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "suspend",
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
                    "suspend",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
            E: ::core::convert::AsRef<web_sys::HtmlMediaElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_time_update<V>
    {
        type State = super::props::on_time_update<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_time_update(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "timeupdate",
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
                    "timeupdate",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
            E: ::core::convert::AsRef<web_sys::HtmlMediaElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_volume_change<V>
    {
        type State = super::props::on_volume_change<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_volume_change(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "volumechange",
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
                    "volumechange",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
            E: ::core::convert::AsRef<web_sys::HtmlMediaElement>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_waiting<V>
    {
        type State = super::props::on_waiting<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_waiting(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "waiting",
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
                    "waiting",
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
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::auto_play<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("autoplay"),
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
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::controls<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("controls"),
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
        for super::props::cross_origin<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("crossorigin"),
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
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::r#loop<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("loop"),
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
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::muted<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("muted"),
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
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::preload<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("preload"),
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
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::src<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("src"),
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
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_abort<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_can_play<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_can_play_through<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_duration_change<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_emptied<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_ended<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_loaded_data<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_loaded_metadata<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_load_start<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_pause<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_play<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_playing<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_progress<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_rate_change<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_resize<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_seeked<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_seeking<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_stalled<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_suspend<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_time_update<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_volume_change<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_waiting<V>
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
