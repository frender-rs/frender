def_props_type!(
    #[derive(Debug, Clone, Copy, Default)]
    HtmlElementProps(
        ..ElementProps(
            children,
            css: bounds![Css::Bounds],
            class: bounds![DomTokens::Bounds],
            id: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
            part: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
            on_cancel:
                bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
            on_error: bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
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
            on_blur:
                bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::FocusEvent>],
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
                bounds![
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::KeyboardEvent>
                ],
            on_key_up:
                bounds![
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::KeyboardEvent>
                ],
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
        on_invalid: bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        on_animation_cancel:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::AnimationEvent>],
        on_animation_end:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::AnimationEvent>],
        on_animation_iteration:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::AnimationEvent>],
        on_animation_start:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::AnimationEvent>],
        on_before_input:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::InputEvent>],
        on_input: bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        on_change: bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        on_got_pointer_capture:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::PointerEvent>],
        on_lost_pointer_capture:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::PointerEvent>],
        on_pointer_cancel:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::PointerEvent>],
        on_pointer_down:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::PointerEvent>],
        on_pointer_enter:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::PointerEvent>],
        on_pointer_leave:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::PointerEvent>],
        on_pointer_move:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::PointerEvent>],
        on_pointer_out:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::PointerEvent>],
        on_pointer_over:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::PointerEvent>],
        on_pointer_up:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::PointerEvent>],
        on_transition_cancel:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::TransitionEvent>],
        on_transition_end:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::TransitionEvent>],
        on_transition_run:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::TransitionEvent>],
        on_transition_start:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::TransitionEvent>],
        on_drag: bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        on_drag_end: bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        on_drag_enter:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        on_drag_leave:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        on_drag_over: bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        on_drag_start:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        on_drop: bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
    )
);
#[cfg(feature = "csr")]
mod imp {
    #![allow(unused_variables)]
    #[allow(unused_imports)]
    use super::super::*;
    crate::imports::impl_bounds!(super::props::access_key(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlElement,
        attr_name = "accesskey",
        csr {
            update: |el: &web_sys::HtmlElement, _, v: &_| el.set_access_key(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::auto_capitalize(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlElement,
        attr_name = "autocapitalize",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::auto_focus(
        bounds as crate::imports::impl_bounds::MaybeValue<bool>,
        element as web_sys::HtmlElement,
        attr_name = "autofocus",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::content_editable(
        bounds as MaybeContentEditable,
        element as web_sys::HtmlElement,
        attr_name = "contenteditable",
        csr {
            update: |v: &_, el: &web_sys::HtmlElement, _attr_name: &_| el.set_content_editable(v),
            remove: MaybeContentEditable::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::context_menu(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlElement,
        attr_name = "contextmenu",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::dir(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlElement,
        attr_name = "dir",
        csr {
            update: |el: &web_sys::HtmlElement, _, v: &_| el.set_dir(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::draggable(
        bounds as crate::imports::impl_bounds::MaybeValue<bool>,
        element as web_sys::HtmlElement,
        attr_name = "draggable",
        csr {
            update: |el: &web_sys::HtmlElement, _, &v: &_| el.set_draggable(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::enter_key_hint(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlElement,
        attr_name = "enterkeyhint",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::hidden(
        bounds as crate::imports::impl_bounds::MaybeValue<bool>,
        element as web_sys::HtmlElement,
        attr_name = "hidden",
        csr {
            update: |el: &web_sys::HtmlElement, _, &v: &_| el.set_hidden(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::inert(
        bounds as crate::imports::impl_bounds::MaybeValue<bool>,
        element as web_sys::HtmlElement,
        attr_name = "inert",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::input_mode(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlElement,
        attr_name = "inputmode",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::is(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlElement,
        attr_name = "is",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::item_id(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlElement,
        attr_name = "itemid",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::item_prop(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlElement,
        attr_name = "itemprop",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::item_ref(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlElement,
        attr_name = "itemref",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::item_scope(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlElement,
        attr_name = "itemscope",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::item_type(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlElement,
        attr_name = "itemtype",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::lang(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlElement,
        attr_name = "lang",
        csr {
            update: |el: &web_sys::HtmlElement, _, v: &_| el.set_lang(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::nonce(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlElement,
        attr_name = "nonce",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::role(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlElement,
        attr_name = "role",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::slot(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlElement,
        attr_name = "slot",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::spellcheck(
        bounds as crate::imports::impl_bounds::MaybeValue<bool>,
        element as web_sys::HtmlElement,
        attr_name = "spellcheck",
        csr {
            update: |el: &web_sys::HtmlElement, _, &v: &_| el.set_spellcheck(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::style(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlElement,
        attr_name = "style",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::tab_index(
        bounds as crate::imports::impl_bounds::MaybeValue<i32>,
        element as web_sys::HtmlElement,
        attr_name = "tabindex",
        csr {
            update: |el: &web_sys::HtmlElement, _, &v: &_| el.set_tab_index(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::title(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlElement,
        attr_name = "title",
        csr {
            update: |el: &web_sys::HtmlElement, _, v: &_| el.set_title(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::translate(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlElement,
        attr_name = "translate",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::virtual_keyboard_policy(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlElement,
        attr_name = "virtualkeyboardpolicy",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::on_invalid(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::HtmlElement,
        attr_name = "invalid",
    ));
    crate::imports::impl_bounds!(super::props::on_animation_cancel(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::AnimationEvent>,
        element as web_sys::HtmlElement,
        attr_name = "animationcancel",
    ));
    crate::imports::impl_bounds!(super::props::on_animation_end(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::AnimationEvent>,
        element as web_sys::HtmlElement,
        attr_name = "animationend",
    ));
    crate::imports::impl_bounds!(super::props::on_animation_iteration(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::AnimationEvent>,
        element as web_sys::HtmlElement,
        attr_name = "animationiteration",
    ));
    crate::imports::impl_bounds!(super::props::on_animation_start(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::AnimationEvent>,
        element as web_sys::HtmlElement,
        attr_name = "animationstart",
    ));
    crate::imports::impl_bounds!(super::props::on_before_input(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::InputEvent>,
        element as web_sys::HtmlElement,
        attr_name = "beforeinput",
    ));
    crate::imports::impl_bounds!(super::props::on_input(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::HtmlElement,
        attr_name = "input",
    ));
    crate::imports::impl_bounds!(super::props::on_change(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::HtmlElement,
        attr_name = "change",
    ));
    crate::imports::impl_bounds!(super::props::on_got_pointer_capture(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::PointerEvent>,
        element as web_sys::HtmlElement,
        attr_name = "gotpointercapture",
    ));
    crate::imports::impl_bounds!(super::props::on_lost_pointer_capture(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::PointerEvent>,
        element as web_sys::HtmlElement,
        attr_name = "lostpointercapture",
    ));
    crate::imports::impl_bounds!(super::props::on_pointer_cancel(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::PointerEvent>,
        element as web_sys::HtmlElement,
        attr_name = "pointercancel",
    ));
    crate::imports::impl_bounds!(super::props::on_pointer_down(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::PointerEvent>,
        element as web_sys::HtmlElement,
        attr_name = "pointerdown",
    ));
    crate::imports::impl_bounds!(super::props::on_pointer_enter(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::PointerEvent>,
        element as web_sys::HtmlElement,
        attr_name = "pointerenter",
    ));
    crate::imports::impl_bounds!(super::props::on_pointer_leave(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::PointerEvent>,
        element as web_sys::HtmlElement,
        attr_name = "pointerleave",
    ));
    crate::imports::impl_bounds!(super::props::on_pointer_move(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::PointerEvent>,
        element as web_sys::HtmlElement,
        attr_name = "pointermove",
    ));
    crate::imports::impl_bounds!(super::props::on_pointer_out(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::PointerEvent>,
        element as web_sys::HtmlElement,
        attr_name = "pointerout",
    ));
    crate::imports::impl_bounds!(super::props::on_pointer_over(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::PointerEvent>,
        element as web_sys::HtmlElement,
        attr_name = "pointerover",
    ));
    crate::imports::impl_bounds!(super::props::on_pointer_up(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::PointerEvent>,
        element as web_sys::HtmlElement,
        attr_name = "pointerup",
    ));
    crate::imports::impl_bounds!(super::props::on_transition_cancel(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::TransitionEvent>,
        element as web_sys::HtmlElement,
        attr_name = "transitioncancel",
    ));
    crate::imports::impl_bounds!(super::props::on_transition_end(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::TransitionEvent>,
        element as web_sys::HtmlElement,
        attr_name = "transitionend",
    ));
    crate::imports::impl_bounds!(super::props::on_transition_run(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::TransitionEvent>,
        element as web_sys::HtmlElement,
        attr_name = "transitionrun",
    ));
    crate::imports::impl_bounds!(super::props::on_transition_start(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::TransitionEvent>,
        element as web_sys::HtmlElement,
        attr_name = "transitionstart",
    ));
    crate::imports::impl_bounds!(super::props::on_drag(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::HtmlElement,
        attr_name = "drag",
    ));
    crate::imports::impl_bounds!(super::props::on_drag_end(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::HtmlElement,
        attr_name = "dragend",
    ));
    crate::imports::impl_bounds!(super::props::on_drag_enter(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::HtmlElement,
        attr_name = "dragenter",
    ));
    crate::imports::impl_bounds!(super::props::on_drag_leave(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::HtmlElement,
        attr_name = "dragleave",
    ));
    crate::imports::impl_bounds!(super::props::on_drag_over(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::HtmlElement,
        attr_name = "dragover",
    ));
    crate::imports::impl_bounds!(super::props::on_drag_start(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::HtmlElement,
        attr_name = "dragstart",
    ));
    crate::imports::impl_bounds!(super::props::on_drop(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::HtmlElement,
        attr_name = "drop",
    ));
}
mod imports {
    #[allow(unused_imports)]
    use super::super::*;
    pub(super) use crate::imports::frender_html_simple::def_props_type;
}
use imports::def_props_type;
