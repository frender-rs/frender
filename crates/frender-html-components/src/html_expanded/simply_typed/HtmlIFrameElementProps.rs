def_props_type!(
    #[derive(Debug, Clone, Copy, Default)]
    HtmlIFrameElementProps(
        ..HtmlElementProps(
            ..ElementProps(
                children,
                css: bounds![Css::Bounds],
                class: bounds![DomTokens::Bounds],
                id: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
                part: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
                on_cancel:
                    bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
                on_error:
                    bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
                on_scroll:
                    bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
                on_security_policy_violation:
                    bounds![
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds<
                            events::SecurityPolicyViolationEvent,
                        >
                    ],
                on_select:
                    bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
                on_wheel:
                    bounds![
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::WheelEvent>
                    ],
                on_copy:
                    bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
                on_cut:
                    bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
                on_paste:
                    bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
                on_composition_end:
                    bounds![
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds<
                            events::CompositionEvent,
                        >
                    ],
                on_composition_start:
                    bounds![
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds<
                            events::CompositionEvent,
                        >
                    ],
                on_composition_update:
                    bounds![
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds<
                            events::CompositionEvent,
                        >
                    ],
                on_blur:
                    bounds![
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::FocusEvent>
                    ],
                on_focus:
                    bounds![
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::FocusEvent>
                    ],
                on_focus_in:
                    bounds![
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::FocusEvent>
                    ],
                on_focus_out:
                    bounds![
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::FocusEvent>
                    ],
                on_fullscreen_change:
                    bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
                on_fullscreen_error:
                    bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
                on_key_down:
                    bounds![
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds<
                            events::KeyboardEvent,
                        >
                    ],
                on_key_up:
                    bounds![
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds<
                            events::KeyboardEvent,
                        >
                    ],
                on_aux_click:
                    bounds![
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::MouseEvent>
                    ],
                on_click:
                    bounds![
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::MouseEvent>
                    ],
                on_context_menu:
                    bounds![
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::MouseEvent>
                    ],
                on_double_click:
                    bounds![
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::MouseEvent>
                    ],
                on_mouse_down:
                    bounds![
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::MouseEvent>
                    ],
                on_mouse_enter:
                    bounds![
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::MouseEvent>
                    ],
                on_mouse_leave:
                    bounds![
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::MouseEvent>
                    ],
                on_mouse_move:
                    bounds![
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::MouseEvent>
                    ],
                on_mouse_out:
                    bounds![
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::MouseEvent>
                    ],
                on_mouse_over:
                    bounds![
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::MouseEvent>
                    ],
                on_mouse_up:
                    bounds![
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::MouseEvent>
                    ],
                on_touch_cancel:
                    bounds![
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::TouchEvent>
                    ],
                on_touch_end:
                    bounds![
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::TouchEvent>
                    ],
                on_touch_move:
                    bounds![
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::TouchEvent>
                    ],
                on_touch_start:
                    bounds![
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::TouchEvent>
                    ],
            ),
            access_key: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
            auto_capitalize: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
            auto_focus: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<bool>],
            content_editable: bounds![MaybeContentEditable::Bounds],
            context_menu: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
            dir: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
            draggable: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<bool>],
            enter_key_hint: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
            hidden: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<bool>],
            inert: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<bool>],
            input_mode: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
            is: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
            item_id: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
            item_prop: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
            item_ref: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
            item_scope: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
            item_type: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
            lang: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
            nonce: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
            role: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
            slot: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
            spellcheck: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<bool>],
            style: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
            tab_index: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<i32>],
            title: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
            translate: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
            virtual_keyboard_policy: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
            on_invalid:
                bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
            on_animation_cancel:
                bounds![
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::AnimationEvent>
                ],
            on_animation_end:
                bounds![
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::AnimationEvent>
                ],
            on_animation_iteration:
                bounds![
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::AnimationEvent>
                ],
            on_animation_start:
                bounds![
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::AnimationEvent>
                ],
            on_before_input:
                bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::InputEvent>],
            on_input: bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
            on_change:
                bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
            on_got_pointer_capture:
                bounds![
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::PointerEvent>
                ],
            on_lost_pointer_capture:
                bounds![
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::PointerEvent>
                ],
            on_pointer_cancel:
                bounds![
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::PointerEvent>
                ],
            on_pointer_down:
                bounds![
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::PointerEvent>
                ],
            on_pointer_enter:
                bounds![
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::PointerEvent>
                ],
            on_pointer_leave:
                bounds![
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::PointerEvent>
                ],
            on_pointer_move:
                bounds![
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::PointerEvent>
                ],
            on_pointer_out:
                bounds![
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::PointerEvent>
                ],
            on_pointer_over:
                bounds![
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::PointerEvent>
                ],
            on_pointer_up:
                bounds![
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::PointerEvent>
                ],
            on_transition_cancel:
                bounds![
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::TransitionEvent>
                ],
            on_transition_end:
                bounds![
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::TransitionEvent>
                ],
            on_transition_run:
                bounds![
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::TransitionEvent>
                ],
            on_transition_start:
                bounds![
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::TransitionEvent>
                ],
            on_drag: bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
            on_drag_end:
                bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
            on_drag_enter:
                bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
            on_drag_leave:
                bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
            on_drag_over:
                bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
            on_drag_start:
                bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
            on_drop: bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        ),
        allow: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        allow_fullscreen: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<bool>],
        allow_payment_request: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<bool>],
        csp: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        fetch_priority: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        height: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        loading: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        name: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        referrer_policy: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        sandbox: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        src: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        src_doc: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        width: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
    )
);
#[cfg(feature = "csr")]
mod imp {
    #![allow(unused_variables)]
    #[allow(unused_imports)]
    use super::super::*;
    crate::imports::impl_bounds!(super::props::allow(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlIFrameElement,
        attr_name = "allow",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::allow_fullscreen(
        bounds as crate::imports::impl_bounds::MaybeValue<bool>,
        element as web_sys::HtmlIFrameElement,
        attr_name = "allowfullscreen",
        csr {
            update: |el: &web_sys::HtmlIFrameElement, _, &v: &_| el.set_allow_fullscreen(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::allow_payment_request(
        bounds as crate::imports::impl_bounds::MaybeValue<bool>,
        element as web_sys::HtmlIFrameElement,
        attr_name = "allowpaymentrequest",
        csr {
            update: |el: &web_sys::HtmlIFrameElement, _, &v: &_| el.set_allow_payment_request(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::csp(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlIFrameElement,
        attr_name = "csp",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::fetch_priority(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlIFrameElement,
        attr_name = "fetchpriority",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::height(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlIFrameElement,
        attr_name = "height",
        csr {
            update: |el: &web_sys::HtmlIFrameElement, _, v: &_| el.set_height(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::loading(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlIFrameElement,
        attr_name = "loading",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::name(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlIFrameElement,
        attr_name = "name",
        csr {
            update: |el: &web_sys::HtmlIFrameElement, _, v: &_| el.set_name(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::referrer_policy(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlIFrameElement,
        attr_name = "referrerpolicy",
        csr {
            update: |el: &web_sys::HtmlIFrameElement, _, v: &_| el.set_referrer_policy(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::sandbox(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlIFrameElement,
        attr_name = "sandbox",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::src(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlIFrameElement,
        attr_name = "src",
        csr {
            update: |el: &web_sys::HtmlIFrameElement, _, v: &_| el.set_src(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::src_doc(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlIFrameElement,
        attr_name = "srcdoc",
        csr {
            update: |el: &web_sys::HtmlIFrameElement, _, v: &_| el.set_srcdoc(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::width(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlIFrameElement,
        attr_name = "width",
        csr {
            update: |el: &web_sys::HtmlIFrameElement, _, v: &_| el.set_width(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
}
mod imports {
    #[allow(unused_imports)]
    use super::super::*;
    pub(super) use crate::imports::frender_html_simple::def_props_type;
}
use imports::def_props_type;
