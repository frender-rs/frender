def_props_type!(
    #[derive(Debug, Clone, Copy, Default)]
    ElementProps(
        children,
        css: bounds![Css::Bounds],
        class: bounds![DomTokens::Bounds],
        id: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        part: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        on_cancel: bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        on_error: bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        on_scroll: bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        on_security_policy_violation:
            bounds![
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds<
                    events::SecurityPolicyViolationEvent,
                >
            ],
        on_select: bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        on_wheel:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::WheelEvent>],
        on_copy: bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        on_cut: bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        on_paste: bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        on_composition_end:
            bounds![
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::CompositionEvent>
            ],
        on_composition_start:
            bounds![
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::CompositionEvent>
            ],
        on_composition_update:
            bounds![
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::CompositionEvent>
            ],
        on_blur: bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::FocusEvent>],
        on_focus:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::FocusEvent>],
        on_focus_in:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::FocusEvent>],
        on_focus_out:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::FocusEvent>],
        on_fullscreen_change:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        on_fullscreen_error:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        on_key_down:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::KeyboardEvent>],
        on_key_up:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::KeyboardEvent>],
        on_aux_click:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::MouseEvent>],
        on_click:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::MouseEvent>],
        on_context_menu:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::MouseEvent>],
        on_double_click:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::MouseEvent>],
        on_mouse_down:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::MouseEvent>],
        on_mouse_enter:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::MouseEvent>],
        on_mouse_leave:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::MouseEvent>],
        on_mouse_move:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::MouseEvent>],
        on_mouse_out:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::MouseEvent>],
        on_mouse_over:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::MouseEvent>],
        on_mouse_up:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::MouseEvent>],
        on_touch_cancel:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::TouchEvent>],
        on_touch_end:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::TouchEvent>],
        on_touch_move:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::TouchEvent>],
        on_touch_start:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::TouchEvent>],
    )
);
#[cfg(feature = "csr")]
mod imp {
    #![allow(unused_variables)]
    #[allow(unused_imports)]
    use super::super::*;
    crate::imports::impl_bounds!(super::props::css(
        bounds as Css,
        element as web_sys::Element,
        attr_name = "css"
    ));
    crate::imports::impl_bounds!(super::props::class(
        bounds as DomTokens,
        element as web_sys::Element,
        attr_name = "class",
        csr {
            get_dom_token: web_sys::Element::class_list
        }
    ));
    crate::imports::impl_bounds!(super::props::id(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::Element,
        attr_name = "id",
        csr {
            update: |el: &web_sys::Element, _, v: &_| el.set_id(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::part(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::Element,
        attr_name = "part",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::on_cancel(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::Element,
        attr_name = "cancel",
    ));
    crate::imports::impl_bounds!(super::props::on_error(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::Element,
        attr_name = "error",
    ));
    crate::imports::impl_bounds!(super::props::on_scroll(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::Element,
        attr_name = "scroll",
    ));
    crate::imports::impl_bounds!(super::props::on_security_policy_violation(
        bounds
            as crate::imports::impl_bounds::MaybeHandleEvent<events::SecurityPolicyViolationEvent>,
        element as web_sys::Element,
        attr_name = "securitypolicyviolation",
    ));
    crate::imports::impl_bounds!(super::props::on_select(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::Element,
        attr_name = "select",
    ));
    crate::imports::impl_bounds!(super::props::on_wheel(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::WheelEvent>,
        element as web_sys::Element,
        attr_name = "wheel",
    ));
    crate::imports::impl_bounds!(super::props::on_copy(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::Element,
        attr_name = "copy",
    ));
    crate::imports::impl_bounds!(super::props::on_cut(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::Element,
        attr_name = "cut",
    ));
    crate::imports::impl_bounds!(super::props::on_paste(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::Element,
        attr_name = "paste",
    ));
    crate::imports::impl_bounds!(super::props::on_composition_end(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::CompositionEvent>,
        element as web_sys::Element,
        attr_name = "compositionend",
    ));
    crate::imports::impl_bounds!(super::props::on_composition_start(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::CompositionEvent>,
        element as web_sys::Element,
        attr_name = "compositionstart",
    ));
    crate::imports::impl_bounds!(super::props::on_composition_update(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::CompositionEvent>,
        element as web_sys::Element,
        attr_name = "compositionupdate",
    ));
    crate::imports::impl_bounds!(super::props::on_blur(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::FocusEvent>,
        element as web_sys::Element,
        attr_name = "blur",
    ));
    crate::imports::impl_bounds!(super::props::on_focus(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::FocusEvent>,
        element as web_sys::Element,
        attr_name = "focus",
    ));
    crate::imports::impl_bounds!(super::props::on_focus_in(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::FocusEvent>,
        element as web_sys::Element,
        attr_name = "focusin",
    ));
    crate::imports::impl_bounds!(super::props::on_focus_out(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::FocusEvent>,
        element as web_sys::Element,
        attr_name = "focusout",
    ));
    crate::imports::impl_bounds!(super::props::on_fullscreen_change(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::Element,
        attr_name = "fullscreenchange",
    ));
    crate::imports::impl_bounds!(super::props::on_fullscreen_error(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::Element,
        attr_name = "fullscreenerror",
    ));
    crate::imports::impl_bounds!(super::props::on_key_down(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::KeyboardEvent>,
        element as web_sys::Element,
        attr_name = "keydown",
    ));
    crate::imports::impl_bounds!(super::props::on_key_up(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::KeyboardEvent>,
        element as web_sys::Element,
        attr_name = "keyup",
    ));
    crate::imports::impl_bounds!(super::props::on_aux_click(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::MouseEvent>,
        element as web_sys::Element,
        attr_name = "auxclick",
    ));
    crate::imports::impl_bounds!(super::props::on_click(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::MouseEvent>,
        element as web_sys::Element,
        attr_name = "click",
    ));
    crate::imports::impl_bounds!(super::props::on_context_menu(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::MouseEvent>,
        element as web_sys::Element,
        attr_name = "contextmenu",
    ));
    crate::imports::impl_bounds!(super::props::on_double_click(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::MouseEvent>,
        element as web_sys::Element,
        attr_name = "dblclick",
    ));
    crate::imports::impl_bounds!(super::props::on_mouse_down(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::MouseEvent>,
        element as web_sys::Element,
        attr_name = "mousedown",
    ));
    crate::imports::impl_bounds!(super::props::on_mouse_enter(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::MouseEvent>,
        element as web_sys::Element,
        attr_name = "mouseenter",
    ));
    crate::imports::impl_bounds!(super::props::on_mouse_leave(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::MouseEvent>,
        element as web_sys::Element,
        attr_name = "mouseleave",
    ));
    crate::imports::impl_bounds!(super::props::on_mouse_move(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::MouseEvent>,
        element as web_sys::Element,
        attr_name = "mousemove",
    ));
    crate::imports::impl_bounds!(super::props::on_mouse_out(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::MouseEvent>,
        element as web_sys::Element,
        attr_name = "mouseout",
    ));
    crate::imports::impl_bounds!(super::props::on_mouse_over(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::MouseEvent>,
        element as web_sys::Element,
        attr_name = "mouseover",
    ));
    crate::imports::impl_bounds!(super::props::on_mouse_up(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::MouseEvent>,
        element as web_sys::Element,
        attr_name = "mouseup",
    ));
    crate::imports::impl_bounds!(super::props::on_touch_cancel(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::TouchEvent>,
        element as web_sys::Element,
        attr_name = "touchcancel",
    ));
    crate::imports::impl_bounds!(super::props::on_touch_end(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::TouchEvent>,
        element as web_sys::Element,
        attr_name = "touchend",
    ));
    crate::imports::impl_bounds!(super::props::on_touch_move(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::TouchEvent>,
        element as web_sys::Element,
        attr_name = "touchmove",
    ));
    crate::imports::impl_bounds!(super::props::on_touch_start(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::TouchEvent>,
        element as web_sys::Element,
        attr_name = "touchstart",
    ));
}
mod imports {
    #[allow(unused_imports)]
    use super::super::*;
    pub(super) use crate::imports::frender_html_simple::def_props_type;
}
use imports::def_props_type;
