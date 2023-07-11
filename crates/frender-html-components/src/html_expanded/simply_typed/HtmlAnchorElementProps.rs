def_props_type!(
    #[derive(Debug, Clone, Copy, Default)]
    HtmlAnchorElementProps(
        ..HtmlElementProps(
            ..ElementProps(
                children,
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
        download: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        href: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        ping: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        referrer_policy: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        rel: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        target: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        href_lang: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        r#type: alias![type_] + bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
    )
);
#[cfg(feature = "csr")]
mod imp {
    #![allow(unused_variables)]
    #[allow(unused_imports)]
    use super::super::*;
    crate::imports::impl_bounds!(super::props::download(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlAnchorElement,
        attr_name = "download",
        csr {
            update: |el: &web_sys::HtmlAnchorElement, _, v: &_| el.set_download(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::href(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlAnchorElement,
        attr_name = "href",
        csr {
            update: |el: &web_sys::HtmlAnchorElement, _, v: &_| el.set_href(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::ping(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlAnchorElement,
        attr_name = "ping",
        csr {
            update: |el: &web_sys::HtmlAnchorElement, _, v: &_| el.set_ping(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::referrer_policy(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlAnchorElement,
        attr_name = "referrerpolicy",
        csr {
            update: |el: &web_sys::HtmlAnchorElement, _, v: &_| el.set_referrer_policy(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::rel(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlAnchorElement,
        attr_name = "rel",
        csr {
            update: |el: &web_sys::HtmlAnchorElement, _, v: &_| el.set_rel(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::target(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlAnchorElement,
        attr_name = "target",
        csr {
            update: |el: &web_sys::HtmlAnchorElement, _, v: &_| el.set_target(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::href_lang(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlAnchorElement,
        attr_name = "hreflang",
        csr {
            update: |el: &web_sys::HtmlAnchorElement, _, v: &_| el.set_hreflang(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::r#type(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlAnchorElement,
        attr_name = "type",
        csr {
            update: |el: &web_sys::HtmlAnchorElement, _, v: &_| el.set_type(v),
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
