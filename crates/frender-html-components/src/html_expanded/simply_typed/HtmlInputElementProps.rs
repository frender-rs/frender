def_props_type!(
    #[derive(Debug, Clone, Copy, Default)]
    HtmlInputElementProps(
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
        accept: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        alt: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        auto_complete: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        capture: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        checked: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<bool>],
        dirname: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        disabled: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<bool>],
        form: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        form_action: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        form_enc_type: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        form_method: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        form_no_validate: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<bool>],
        form_target: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        height: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<u32>],
        list: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        max: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        max_length: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<i32>],
        min: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        min_length: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<i32>],
        multiple: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<bool>],
        name: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        pattern: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        placeholder: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        read_only: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<bool>],
        required: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<bool>],
        size: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<u32>],
        src: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        step: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        r#type: alias![type_] + bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        value: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        width: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<u32>],
    )
);
#[cfg(feature = "csr")]
mod imp {
    #![allow(unused_variables)]
    #[allow(unused_imports)]
    use super::super::*;
    crate::imports::impl_bounds!(super::props::accept(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlInputElement,
        attr_name = "accept",
        csr {
            update: |el: &web_sys::HtmlInputElement, _, v: &_| el.set_accept(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::alt(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlInputElement,
        attr_name = "alt",
        csr {
            update: |el: &web_sys::HtmlInputElement, _, v: &_| el.set_alt(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::auto_complete(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlInputElement,
        attr_name = "autocomplete",
        csr {
            update: |el: &web_sys::HtmlInputElement, _, v: &_| el.set_autocomplete(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::capture(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlInputElement,
        attr_name = "capture",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::checked(
        bounds as crate::imports::impl_bounds::MaybeValue<bool>,
        element as web_sys::HtmlInputElement,
        attr_name = "checked",
        csr {
            update: |el: &web_sys::HtmlInputElement, _, &v: &_| el.set_checked(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::dirname(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlInputElement,
        attr_name = "dirname",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::disabled(
        bounds as crate::imports::impl_bounds::MaybeValue<bool>,
        element as web_sys::HtmlInputElement,
        attr_name = "disabled",
        csr {
            update: |el: &web_sys::HtmlInputElement, _, &v: &_| el.set_disabled(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::form(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlInputElement,
        attr_name = "form",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::form_action(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlInputElement,
        attr_name = "formaction",
        csr {
            update: |el: &web_sys::HtmlInputElement, _, v: &_| el.set_form_action(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::form_enc_type(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlInputElement,
        attr_name = "formenctype",
        csr {
            update: |el: &web_sys::HtmlInputElement, _, v: &_| el.set_form_enctype(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::form_method(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlInputElement,
        attr_name = "formmethod",
        csr {
            update: |el: &web_sys::HtmlInputElement, _, v: &_| el.set_form_method(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::form_no_validate(
        bounds as crate::imports::impl_bounds::MaybeValue<bool>,
        element as web_sys::HtmlInputElement,
        attr_name = "formnovalidate",
        csr {
            update: |el: &web_sys::HtmlInputElement, _, &v: &_| el.set_form_no_validate(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::form_target(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlInputElement,
        attr_name = "formtarget",
        csr {
            update: |el: &web_sys::HtmlInputElement, _, v: &_| el.set_form_target(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::height(
        bounds as crate::imports::impl_bounds::MaybeValue<u32>,
        element as web_sys::HtmlInputElement,
        attr_name = "height",
        csr {
            update: |el: &web_sys::HtmlInputElement, _, &v: &_| el.set_height(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::list(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlInputElement,
        attr_name = "list",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::max(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlInputElement,
        attr_name = "max",
        csr {
            update: |el: &web_sys::HtmlInputElement, _, v: &_| el.set_max(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::max_length(
        bounds as crate::imports::impl_bounds::MaybeValue<i32>,
        element as web_sys::HtmlInputElement,
        attr_name = "maxlength",
        csr {
            update: |el: &web_sys::HtmlInputElement, _, &v: &_| el.set_max_length(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::min(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlInputElement,
        attr_name = "min",
        csr {
            update: |el: &web_sys::HtmlInputElement, _, v: &_| el.set_min(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::min_length(
        bounds as crate::imports::impl_bounds::MaybeValue<i32>,
        element as web_sys::HtmlInputElement,
        attr_name = "minlength",
        csr {
            update: |el: &web_sys::HtmlInputElement, _, &v: &_| el.set_min_length(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::multiple(
        bounds as crate::imports::impl_bounds::MaybeValue<bool>,
        element as web_sys::HtmlInputElement,
        attr_name = "multiple",
        csr {
            update: |el: &web_sys::HtmlInputElement, _, &v: &_| el.set_multiple(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::name(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlInputElement,
        attr_name = "name",
        csr {
            update: |el: &web_sys::HtmlInputElement, _, v: &_| el.set_name(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::pattern(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlInputElement,
        attr_name = "pattern",
        csr {
            update: |el: &web_sys::HtmlInputElement, _, v: &_| el.set_pattern(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::placeholder(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlInputElement,
        attr_name = "placeholder",
        csr {
            update: |el: &web_sys::HtmlInputElement, _, v: &_| el.set_placeholder(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::read_only(
        bounds as crate::imports::impl_bounds::MaybeValue<bool>,
        element as web_sys::HtmlInputElement,
        attr_name = "readonly",
        csr {
            update: |el: &web_sys::HtmlInputElement, _, &v: &_| el.set_read_only(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::required(
        bounds as crate::imports::impl_bounds::MaybeValue<bool>,
        element as web_sys::HtmlInputElement,
        attr_name = "required",
        csr {
            update: |el: &web_sys::HtmlInputElement, _, &v: &_| el.set_required(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::size(
        bounds as crate::imports::impl_bounds::MaybeValue<u32>,
        element as web_sys::HtmlInputElement,
        attr_name = "size",
        csr {
            update: |el: &web_sys::HtmlInputElement, _, &v: &_| el.set_size(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::src(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlInputElement,
        attr_name = "src",
        csr {
            update: |el: &web_sys::HtmlInputElement, _, v: &_| el.set_src(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::step(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlInputElement,
        attr_name = "step",
        csr {
            update: |el: &web_sys::HtmlInputElement, _, v: &_| el.set_step(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::r#type(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlInputElement,
        attr_name = "type",
        csr {
            update: |el: &web_sys::HtmlInputElement, _, v: &_| el.set_type(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::value(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlInputElement,
        attr_name = "value",
        csr {
            update: |el: &web_sys::HtmlInputElement, _, v: &_| el.set_value(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::width(
        bounds as crate::imports::impl_bounds::MaybeValue<u32>,
        element as web_sys::HtmlInputElement,
        attr_name = "width",
        csr {
            update: |el: &web_sys::HtmlInputElement, _, &v: &_| el.set_width(v),
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
