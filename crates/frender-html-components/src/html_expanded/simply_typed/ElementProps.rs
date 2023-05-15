def_props_type!(
    #[derive(Debug, Clone, Copy, Default)]
    ElementProps(
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
        on_wheel: bounds![crate::imports::frender_events::MaybeHandleEvent<events::WheelEvent>],
        on_copy: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
        on_cut: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
        on_paste: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
        on_composition_end:
            bounds![crate::imports::frender_events::MaybeHandleEvent<events::CompositionEvent>],
        on_composition_start:
            bounds![crate::imports::frender_events::MaybeHandleEvent<events::CompositionEvent>],
        on_composition_update:
            bounds![crate::imports::frender_events::MaybeHandleEvent<events::CompositionEvent>],
        on_blur: bounds![crate::imports::frender_events::MaybeHandleEvent<events::FocusEvent>],
        on_focus: bounds![crate::imports::frender_events::MaybeHandleEvent<events::FocusEvent>],
        on_focus_in: bounds![crate::imports::frender_events::MaybeHandleEvent<events::FocusEvent>],
        on_focus_out: bounds![crate::imports::frender_events::MaybeHandleEvent<events::FocusEvent>],
        on_fullscreen_change:
            bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
        on_fullscreen_error:
            bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
        on_key_down:
            bounds![crate::imports::frender_events::MaybeHandleEvent<events::KeyboardEvent>],
        on_key_up: bounds![crate::imports::frender_events::MaybeHandleEvent<events::KeyboardEvent>],
        on_aux_click: bounds![crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>],
        on_click: bounds![crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>],
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
        on_mouse_out: bounds![crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>],
        on_mouse_over:
            bounds![crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>],
        on_mouse_up: bounds![crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>],
        on_touch_cancel:
            bounds![crate::imports::frender_events::MaybeHandleEvent<events::TouchEvent>],
        on_touch_end: bounds![crate::imports::frender_events::MaybeHandleEvent<events::TouchEvent>],
        on_touch_move:
            bounds![crate::imports::frender_events::MaybeHandleEvent<events::TouchEvent>],
        on_touch_start:
            bounds![crate::imports::frender_events::MaybeHandleEvent<events::TouchEvent>],
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
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
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
            super::props::on_cancel(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "cancel",
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
                    "cancel",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
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
            super::props::on_error(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "error",
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
                    "error",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
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
            super::props::on_scroll(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "scroll",
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
                    "scroll",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::SecurityPolicyViolationEvent>,
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
            super::props::on_security_policy_violation(V::initialize_handle_event_state(
                this.0,
                |callable| {
                    crate::imports::frender_events::EventListener::new(
                        dom_element,
                        "securitypolicyviolation",
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
                    "securitypolicyviolation",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
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
            super::props::on_select(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "select",
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
                    "select",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::WheelEvent>,
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
            super::props::on_wheel(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "wheel",
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
                    "wheel",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
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
            super::props::on_copy(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "copy",
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
                    "copy",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
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
            super::props::on_cut(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "cut",
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
                    "cut",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
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
            super::props::on_paste(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "paste",
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
                    "paste",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::CompositionEvent>,
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
            super::props::on_composition_end(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "compositionend",
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
                    "compositionend",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::CompositionEvent>,
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
            super::props::on_composition_start(V::initialize_handle_event_state(
                this.0,
                |callable| {
                    crate::imports::frender_events::EventListener::new(
                        dom_element,
                        "compositionstart",
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
                    "compositionstart",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::CompositionEvent>,
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
            super::props::on_composition_update(V::initialize_handle_event_state(
                this.0,
                |callable| {
                    crate::imports::frender_events::EventListener::new(
                        dom_element,
                        "compositionupdate",
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
                    "compositionupdate",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::FocusEvent>,
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
            super::props::on_blur(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "blur",
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
                    "blur",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::FocusEvent>,
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
            super::props::on_focus(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "focus",
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
                    "focus",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::FocusEvent>,
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
            super::props::on_focus_in(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "focusin",
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
                    "focusin",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::FocusEvent>,
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
            super::props::on_focus_out(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "focusout",
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
                    "focusout",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
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
            super::props::on_fullscreen_change(V::initialize_handle_event_state(
                this.0,
                |callable| {
                    crate::imports::frender_events::EventListener::new(
                        dom_element,
                        "fullscreenchange",
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
                    "fullscreenchange",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::Event>,
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
            super::props::on_fullscreen_error(V::initialize_handle_event_state(
                this.0,
                |callable| {
                    crate::imports::frender_events::EventListener::new(
                        dom_element,
                        "fullscreenerror",
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
                    "fullscreenerror",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::KeyboardEvent>,
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
            super::props::on_key_down(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "keydown",
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
                    "keydown",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::KeyboardEvent>,
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
            super::props::on_key_up(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "keyup",
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
                    "keyup",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>,
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
            super::props::on_aux_click(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "auxclick",
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
                    "auxclick",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>,
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
            super::props::on_click(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "click",
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
                    "click",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>,
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
            super::props::on_context_menu(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "contextmenu",
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
                    "contextmenu",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>,
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
            super::props::on_double_click(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "dblclick",
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
                    "dblclick",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>,
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
            super::props::on_mouse_down(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "mousedown",
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
                    "mousedown",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>,
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
            super::props::on_mouse_enter(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "mouseenter",
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
                    "mouseenter",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>,
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
            super::props::on_mouse_leave(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "mouseleave",
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
                    "mouseleave",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>,
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
            super::props::on_mouse_move(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "mousemove",
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
                    "mousemove",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>,
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
            super::props::on_mouse_out(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "mouseout",
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
                    "mouseout",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>,
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
            super::props::on_mouse_over(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "mouseover",
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
                    "mouseover",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>,
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
            super::props::on_mouse_up(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "mouseup",
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
                    "mouseup",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::TouchEvent>,
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
            super::props::on_touch_cancel(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "touchcancel",
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
                    "touchcancel",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::TouchEvent>,
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
            super::props::on_touch_end(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "touchend",
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
                    "touchend",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::TouchEvent>,
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
            super::props::on_touch_move(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "touchmove",
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
                    "touchmove",
                    callable.clone(),
                )
            })
        }
    }
    impl<
            V: crate::imports::frender_events::MaybeHandleEvent<events::TouchEvent>,
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
            super::props::on_touch_start(V::initialize_handle_event_state(this.0, |callable| {
                crate::imports::frender_events::EventListener::new(
                    dom_element,
                    "touchstart",
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
                    "touchstart",
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
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_cancel<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_error<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_scroll<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<
            'a,
            V: crate::imports::frender_events::MaybeHandleEvent<events::SecurityPolicyViolationEvent>,
        > crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_security_policy_violation<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_select<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::WheelEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_wheel<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_copy<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_cut<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_paste<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::CompositionEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_composition_end<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::CompositionEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_composition_start<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::CompositionEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_composition_update<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::FocusEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_blur<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::FocusEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_focus<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::FocusEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_focus_in<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::FocusEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_focus_out<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_fullscreen_change<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::Event>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_fullscreen_error<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::KeyboardEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_key_down<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::KeyboardEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_key_up<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_aux_click<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_click<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_context_menu<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_double_click<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_mouse_down<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_mouse_enter<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_mouse_leave<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_mouse_move<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_mouse_out<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_mouse_over<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a> for super::props::on_mouse_up<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::TouchEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_touch_cancel<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::TouchEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_touch_end<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::TouchEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
        for super::props::on_touch_move<V>
    {
        type IntoIterAttrs =
            ::core::iter::Empty<crate::imports::frender_ssr::element::html::HtmlAttrPair<'a>>;
        fn into_iter_attrs(this: Self) -> Self::IntoIterAttrs {
            ::core::iter::empty()
        }
    }
    impl<'a, V: crate::imports::frender_events::MaybeHandleEvent<events::TouchEvent>>
        crate::imports::frender_ssr::attrs::IntoIteratorAttrs<'a>
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
