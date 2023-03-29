def_props_type!(
    #[derive(Debug, Clone, Copy, Default)]
    ElementProps(
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
    )
);
#[cfg(feature = "csr")]
mod impl_dom_for_props {
    #![allow(unused_variables)]
    #[allow(unused_imports)]
    use super::super::*;
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::class<V>
    {
        type State = super::props::class<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::class(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "class",
                    ),
                    || dom_element.remove_attribute("class").unwrap(),
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
                    "class",
                ),
                || dom_element.remove_attribute("class").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E> for super::props::id<V>
    {
        type State = super::props::id<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::id(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_id(v),
                    || dom_element.remove_attribute("id").unwrap(),
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
                |v| element.set_id(v),
                || dom_element.remove_attribute("id").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::part<V>
    {
        type State = super::props::part<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::part(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "part",
                    ),
                    || dom_element.remove_attribute("part").unwrap(),
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
                    "part",
                ),
                || dom_element.remove_attribute("part").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::Cancel>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_cancel<V>
    {
        type State = super::props::on_cancel<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_cancel(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::Error>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_error<V>
    {
        type State = super::props::on_error<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_error(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::Scroll>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_scroll<V>
    {
        type State = super::props::on_scroll<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_scroll(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<
                events::SecurityPolicyViolation,
            >,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_security_policy_violation<V>
    {
        type State = super::props::on_security_policy_violation<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_security_policy_violation(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::Select>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_select<V>
    {
        type State = super::props::on_select<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_select(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::Wheel>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_wheel<V>
    {
        type State = super::props::on_wheel<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_wheel(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::Copy>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_copy<V>
    {
        type State = super::props::on_copy<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_copy(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::Cut>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_cut<V>
    {
        type State = super::props::on_cut<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_cut(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::Paste>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_paste<V>
    {
        type State = super::props::on_paste<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_paste(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::CompositionEnd>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_composition_end<V>
    {
        type State = super::props::on_composition_end<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_composition_end(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::CompositionStart>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_composition_start<V>
    {
        type State = super::props::on_composition_start<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_composition_start(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::CompositionUpdate>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_composition_update<V>
    {
        type State = super::props::on_composition_update<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_composition_update(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::Blur>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_blur<V>
    {
        type State = super::props::on_blur<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_blur(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::Focus>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_focus<V>
    {
        type State = super::props::on_focus<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_focus(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::FocusIn>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_focus_in<V>
    {
        type State = super::props::on_focus_in<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_focus_in(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::FocusOut>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_focus_out<V>
    {
        type State = super::props::on_focus_out<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_focus_out(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::FullscreenChange>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_fullscreen_change<V>
    {
        type State = super::props::on_fullscreen_change<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_fullscreen_change(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::FullscreenError>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_fullscreen_error<V>
    {
        type State = super::props::on_fullscreen_error<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_fullscreen_error(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::KeyDown>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_key_down<V>
    {
        type State = super::props::on_key_down<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_key_down(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::KeyUp>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_key_up<V>
    {
        type State = super::props::on_key_up<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_key_up(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::AuxClick>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_aux_click<V>
    {
        type State = super::props::on_aux_click<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_aux_click(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::Click>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_click<V>
    {
        type State = super::props::on_click<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_click(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::ContextMenu>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_context_menu<V>
    {
        type State = super::props::on_context_menu<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_context_menu(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::DoubleClick>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_double_click<V>
    {
        type State = super::props::on_double_click<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_double_click(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::MouseDown>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_mouse_down<V>
    {
        type State = super::props::on_mouse_down<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_mouse_down(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::MouseEnter>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_mouse_enter<V>
    {
        type State = super::props::on_mouse_enter<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_mouse_enter(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::MouseLeave>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_mouse_leave<V>
    {
        type State = super::props::on_mouse_leave<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_mouse_leave(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::MouseMove>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_mouse_move<V>
    {
        type State = super::props::on_mouse_move<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_mouse_move(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::MouseOut>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_mouse_out<V>
    {
        type State = super::props::on_mouse_out<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_mouse_out(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::MouseOver>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_mouse_over<V>
    {
        type State = super::props::on_mouse_over<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_mouse_over(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::MouseUp>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_mouse_up<V>
    {
        type State = super::props::on_mouse_up<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_mouse_up(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::TouchCancel>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_touch_cancel<V>
    {
        type State = super::props::on_touch_cancel<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_touch_cancel(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::TouchEnd>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_touch_end<V>
    {
        type State = super::props::on_touch_end<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_touch_end(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::TouchMove>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_touch_move<V>
    {
        type State = super::props::on_touch_move<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_touch_move(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::TouchStart>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_csr::props::UpdateElementNonReactive<E>
        for super::props::on_touch_start<V>
    {
        type State = super::props::on_touch_start<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_touch_start(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_csr::CsrContext,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
}
#[cfg(feature = "ssr")]
mod impl_ssr_for_props {
    #![allow(unused_variables)]
    #[allow(unused_imports)]
    use super::super::*;
    impl<'a, V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::class<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("class"),
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
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::id<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("id"),
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
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::part<V>
    {
        type IntoIterAttrs =
            ::core::option::IntoIter<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            V::maybe_into_html_attribute_value(this.0)
                .map(|attr_value| (
                    ::std::borrow::Cow::Borrowed("part"),
                    attr_value
                        .map_or(
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue,
                            crate::imports::frender_ssr::element::html::HtmlAttributeValue::String,
                        ),
                ))
                .into_iter()
        }
    }
    impl<'a, V> crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_cancel<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V> crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_error<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V> crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_scroll<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V> crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_security_policy_violation<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V> crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_select<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V> crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_wheel<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V> crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_copy<V> {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V> crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_cut<V> {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V> crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_paste<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V> crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_composition_end<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V> crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_composition_start<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V> crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_composition_update<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V> crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_blur<V> {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V> crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_focus<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V> crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_focus_in<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V> crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_focus_out<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V> crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_fullscreen_change<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V> crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_fullscreen_error<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V> crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_key_down<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V> crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_key_up<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V> crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_aux_click<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V> crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_click<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V> crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_context_menu<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V> crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_double_click<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V> crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_mouse_down<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V> crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_mouse_enter<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V> crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_mouse_leave<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V> crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_mouse_move<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V> crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_mouse_out<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V> crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_mouse_over<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V> crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_mouse_up<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V> crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_touch_cancel<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V> crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_touch_end<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V> crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_touch_move<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V> crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_touch_start<V>
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
