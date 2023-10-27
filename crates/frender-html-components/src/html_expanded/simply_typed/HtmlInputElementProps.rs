def_props_type!(
    #[derive(Debug, Clone, Copy, Default)]
    HtmlInputElementProps(
        ..HtmlElementProps(
            ..ElementProps(
                children,
                css: bounds![Css::Bounds],
                class: bounds![DomTokens::Bounds],
                id: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
                part: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
                on_cancel:
                    bounds![
                        #[event(crate::imports::frender_html::html::event_type_helpers::on_cancel)]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_error:
                    bounds![
                        #[event(crate::imports::frender_html::html::event_type_helpers::on_error)]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_scroll:
                    bounds![
                        #[event(crate::imports::frender_html::html::event_type_helpers::on_scroll)]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_security_policy_violation:
                    bounds![
                        #[event(crate
    ::imports::frender_html::html::event_type_helpers::on_security_policy_violation)]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_select:
                    bounds![
                        #[event(crate::imports::frender_html::html::event_type_helpers::on_select)]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_wheel:
                    bounds![
                        #[event(crate::imports::frender_html::html::event_type_helpers::on_wheel)]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_copy:
                    bounds![
                        #[event(crate::imports::frender_html::html::event_type_helpers::on_copy)]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_cut:
                    bounds![
                        #[event(crate::imports::frender_html::html::event_type_helpers::on_cut)]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_paste:
                    bounds![
                        #[event(crate::imports::frender_html::html::event_type_helpers::on_paste)]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_composition_end:
                    bounds![
                        #[event(crate
    ::imports::frender_html::html::event_type_helpers::on_composition_end)]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_composition_start:
                    bounds![
                        #[event(crate
    ::imports::frender_html::html::event_type_helpers::on_composition_start)]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_composition_update:
                    bounds![
                        #[event(crate
    ::imports::frender_html::html::event_type_helpers::on_composition_update)]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_blur:
                    bounds![
                        #[event(crate::imports::frender_html::html::event_type_helpers::on_blur)]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_focus:
                    bounds![
                        #[event(crate::imports::frender_html::html::event_type_helpers::on_focus)]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_focus_in:
                    bounds![
                        #[event(
                            crate::imports::frender_html::html::event_type_helpers::on_focus_in
                        )]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_focus_out:
                    bounds![
                        #[event(
                            crate::imports::frender_html::html::event_type_helpers::on_focus_out
                        )]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_fullscreen_change:
                    bounds![
                        #[event(crate
    ::imports::frender_html::html::event_type_helpers::on_fullscreen_change)]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_fullscreen_error:
                    bounds![
                        #[event(crate
    ::imports::frender_html::html::event_type_helpers::on_fullscreen_error)]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_key_down:
                    bounds![
                        #[event(
                            crate::imports::frender_html::html::event_type_helpers::on_key_down
                        )]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_key_up:
                    bounds![
                        #[event(crate::imports::frender_html::html::event_type_helpers::on_key_up)]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_aux_click:
                    bounds![
                        #[event(
                            crate::imports::frender_html::html::event_type_helpers::on_aux_click
                        )]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_click:
                    bounds![
                        #[event(crate::imports::frender_html::html::event_type_helpers::on_click)]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_context_menu:
                    bounds![
                        #[event(
                            crate::imports::frender_html::html::event_type_helpers::on_context_menu
                        )]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_double_click:
                    bounds![
                        #[event(
                            crate::imports::frender_html::html::event_type_helpers::on_double_click
                        )]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_mouse_down:
                    bounds![
                        #[event(
                            crate::imports::frender_html::html::event_type_helpers::on_mouse_down
                        )]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_mouse_enter:
                    bounds![
                        #[event(
                            crate::imports::frender_html::html::event_type_helpers::on_mouse_enter
                        )]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_mouse_leave:
                    bounds![
                        #[event(
                            crate::imports::frender_html::html::event_type_helpers::on_mouse_leave
                        )]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_mouse_move:
                    bounds![
                        #[event(
                            crate::imports::frender_html::html::event_type_helpers::on_mouse_move
                        )]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_mouse_out:
                    bounds![
                        #[event(
                            crate::imports::frender_html::html::event_type_helpers::on_mouse_out
                        )]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_mouse_over:
                    bounds![
                        #[event(
                            crate::imports::frender_html::html::event_type_helpers::on_mouse_over
                        )]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_mouse_up:
                    bounds![
                        #[event(
                            crate::imports::frender_html::html::event_type_helpers::on_mouse_up
                        )]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_touch_cancel:
                    bounds![
                        #[event(
                            crate::imports::frender_html::html::event_type_helpers::on_touch_cancel
                        )]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_touch_end:
                    bounds![
                        #[event(
                            crate::imports::frender_html::html::event_type_helpers::on_touch_end
                        )]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_touch_move:
                    bounds![
                        #[event(
                            crate::imports::frender_html::html::event_type_helpers::on_touch_move
                        )]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_touch_start:
                    bounds![
                        #[event(
                            crate::imports::frender_html::html::event_type_helpers::on_touch_start
                        )]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
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
                bounds![
                    #[event(crate::imports::frender_html::html::event_type_helpers::on_invalid)]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_animation_cancel:
                bounds![
                    #[event(
                        crate::imports::frender_html::html::event_type_helpers::on_animation_cancel
                    )]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_animation_end:
                bounds![
                    #[event(
                        crate::imports::frender_html::html::event_type_helpers::on_animation_end
                    )]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_animation_iteration:
                bounds![
                    #[event(crate
    ::imports::frender_html::html::event_type_helpers::on_animation_iteration)]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_animation_start:
                bounds![
                    #[event(
                        crate::imports::frender_html::html::event_type_helpers::on_animation_start
                    )]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_before_input:
                bounds![
                    #[event(
                        crate::imports::frender_html::html::event_type_helpers::on_before_input
                    )]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_input:
                bounds![
                    #[event(crate::imports::frender_html::html::event_type_helpers::on_input)]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_change:
                bounds![
                    #[event(crate::imports::frender_html::html::event_type_helpers::on_change)]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_got_pointer_capture:
                bounds![
                    #[event(crate
    ::imports::frender_html::html::event_type_helpers::on_got_pointer_capture)]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_lost_pointer_capture:
                bounds![
                    #[event(crate
    ::imports::frender_html::html::event_type_helpers::on_lost_pointer_capture)]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_pointer_cancel:
                bounds![
                    #[event(
                        crate::imports::frender_html::html::event_type_helpers::on_pointer_cancel
                    )]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_pointer_down:
                bounds![
                    #[event(
                        crate::imports::frender_html::html::event_type_helpers::on_pointer_down
                    )]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_pointer_enter:
                bounds![
                    #[event(
                        crate::imports::frender_html::html::event_type_helpers::on_pointer_enter
                    )]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_pointer_leave:
                bounds![
                    #[event(
                        crate::imports::frender_html::html::event_type_helpers::on_pointer_leave
                    )]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_pointer_move:
                bounds![
                    #[event(
                        crate::imports::frender_html::html::event_type_helpers::on_pointer_move
                    )]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_pointer_out:
                bounds![
                    #[event(
                        crate::imports::frender_html::html::event_type_helpers::on_pointer_out
                    )]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_pointer_over:
                bounds![
                    #[event(
                        crate::imports::frender_html::html::event_type_helpers::on_pointer_over
                    )]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_pointer_up:
                bounds![
                    #[event(crate::imports::frender_html::html::event_type_helpers::on_pointer_up)]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_transition_cancel:
                bounds![
                    #[event(crate
    ::imports::frender_html::html::event_type_helpers::on_transition_cancel)]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_transition_end:
                bounds![
                    #[event(
                        crate::imports::frender_html::html::event_type_helpers::on_transition_end
                    )]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_transition_run:
                bounds![
                    #[event(
                        crate::imports::frender_html::html::event_type_helpers::on_transition_run
                    )]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_transition_start:
                bounds![
                    #[event(
                        crate::imports::frender_html::html::event_type_helpers::on_transition_start
                    )]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_drag:
                bounds![
                    #[event(crate::imports::frender_html::html::event_type_helpers::on_drag)]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_drag_end:
                bounds![
                    #[event(crate::imports::frender_html::html::event_type_helpers::on_drag_end)]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_drag_enter:
                bounds![
                    #[event(crate::imports::frender_html::html::event_type_helpers::on_drag_enter)]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_drag_leave:
                bounds![
                    #[event(crate::imports::frender_html::html::event_type_helpers::on_drag_leave)]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_drag_over:
                bounds![
                    #[event(crate::imports::frender_html::html::event_type_helpers::on_drag_over)]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_drag_start:
                bounds![
                    #[event(crate::imports::frender_html::html::event_type_helpers::on_drag_start)]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_drop:
                bounds![
                    #[event(crate::imports::frender_html::html::event_type_helpers::on_drop)]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
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
        element as HtmlInputElement,
        attr_name = "accept",
        csr {
            update: |el: &mut ET::HtmlInputElement<Renderer>, renderer: &mut _, _, v: &_| el
                .set_accept(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::alt(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlInputElement,
        attr_name = "alt",
        csr {
            update: |el: &mut ET::HtmlInputElement<Renderer>, renderer: &mut _, _, v: &_| el
                .set_alt(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::auto_complete(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlInputElement,
        attr_name = "autocomplete",
        csr {
            update: |el: &mut ET::HtmlInputElement<Renderer>, renderer: &mut _, _, v: &_| el
                .set_auto_complete(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::capture(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlInputElement,
        attr_name = "capture",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::checked(
        bounds as crate::imports::impl_bounds::MaybeValue<bool>,
        element as HtmlInputElement,
        attr_name = "checked",
        csr {
            update: |el: &mut ET::HtmlInputElement<Renderer>, renderer: &mut _, _, &v: &_| el
                .set_checked(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::dirname(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlInputElement,
        attr_name = "dirname",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::disabled(
        bounds as crate::imports::impl_bounds::MaybeValue<bool>,
        element as HtmlInputElement,
        attr_name = "disabled",
        csr {
            update: |el: &mut ET::HtmlInputElement<Renderer>, renderer: &mut _, _, &v: &_| el
                .set_disabled(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::form(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlInputElement,
        attr_name = "form",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::form_action(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlInputElement,
        attr_name = "formaction",
        csr {
            update: |el: &mut ET::HtmlInputElement<Renderer>, renderer: &mut _, _, v: &_| el
                .set_form_action(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::form_enc_type(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlInputElement,
        attr_name = "formenctype",
        csr {
            update: |el: &mut ET::HtmlInputElement<Renderer>, renderer: &mut _, _, v: &_| el
                .set_form_enctype(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::form_method(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlInputElement,
        attr_name = "formmethod",
        csr {
            update: |el: &mut ET::HtmlInputElement<Renderer>, renderer: &mut _, _, v: &_| el
                .set_form_method(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::form_no_validate(
        bounds as crate::imports::impl_bounds::MaybeValue<bool>,
        element as HtmlInputElement,
        attr_name = "formnovalidate",
        csr {
            update: |el: &mut ET::HtmlInputElement<Renderer>, renderer: &mut _, _, &v: &_| el
                .set_form_no_validate(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::form_target(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlInputElement,
        attr_name = "formtarget",
        csr {
            update: |el: &mut ET::HtmlInputElement<Renderer>, renderer: &mut _, _, v: &_| el
                .set_form_target(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::height(
        bounds as crate::imports::impl_bounds::MaybeValue<u32>,
        element as HtmlInputElement,
        attr_name = "height",
        csr {
            update: |el: &mut ET::HtmlInputElement<Renderer>, renderer: &mut _, _, &v: &_| el
                .set_height(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::list(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlInputElement,
        attr_name = "list",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::max(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlInputElement,
        attr_name = "max",
        csr {
            update: |el: &mut ET::HtmlInputElement<Renderer>, renderer: &mut _, _, v: &_| el
                .set_max(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::max_length(
        bounds as crate::imports::impl_bounds::MaybeValue<i32>,
        element as HtmlInputElement,
        attr_name = "maxlength",
        csr {
            update: |el: &mut ET::HtmlInputElement<Renderer>, renderer: &mut _, _, &v: &_| el
                .set_max_length(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::min(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlInputElement,
        attr_name = "min",
        csr {
            update: |el: &mut ET::HtmlInputElement<Renderer>, renderer: &mut _, _, v: &_| el
                .set_min(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::min_length(
        bounds as crate::imports::impl_bounds::MaybeValue<i32>,
        element as HtmlInputElement,
        attr_name = "minlength",
        csr {
            update: |el: &mut ET::HtmlInputElement<Renderer>, renderer: &mut _, _, &v: &_| el
                .set_min_length(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::multiple(
        bounds as crate::imports::impl_bounds::MaybeValue<bool>,
        element as HtmlInputElement,
        attr_name = "multiple",
        csr {
            update: |el: &mut ET::HtmlInputElement<Renderer>, renderer: &mut _, _, &v: &_| el
                .set_multiple(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::name(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlInputElement,
        attr_name = "name",
        csr {
            update: |el: &mut ET::HtmlInputElement<Renderer>, renderer: &mut _, _, v: &_| el
                .set_name(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::pattern(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlInputElement,
        attr_name = "pattern",
        csr {
            update: |el: &mut ET::HtmlInputElement<Renderer>, renderer: &mut _, _, v: &_| el
                .set_pattern(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::placeholder(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlInputElement,
        attr_name = "placeholder",
        csr {
            update: |el: &mut ET::HtmlInputElement<Renderer>, renderer: &mut _, _, v: &_| el
                .set_placeholder(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::read_only(
        bounds as crate::imports::impl_bounds::MaybeValue<bool>,
        element as HtmlInputElement,
        attr_name = "readonly",
        csr {
            update: |el: &mut ET::HtmlInputElement<Renderer>, renderer: &mut _, _, &v: &_| el
                .set_read_only(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::required(
        bounds as crate::imports::impl_bounds::MaybeValue<bool>,
        element as HtmlInputElement,
        attr_name = "required",
        csr {
            update: |el: &mut ET::HtmlInputElement<Renderer>, renderer: &mut _, _, &v: &_| el
                .set_required(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::size(
        bounds as crate::imports::impl_bounds::MaybeValue<u32>,
        element as HtmlInputElement,
        attr_name = "size",
        csr {
            update: |el: &mut ET::HtmlInputElement<Renderer>, renderer: &mut _, _, &v: &_| el
                .set_size(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::src(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlInputElement,
        attr_name = "src",
        csr {
            update: |el: &mut ET::HtmlInputElement<Renderer>, renderer: &mut _, _, v: &_| el
                .set_src(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::step(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlInputElement,
        attr_name = "step",
        csr {
            update: |el: &mut ET::HtmlInputElement<Renderer>, renderer: &mut _, _, v: &_| el
                .set_step(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::r#type(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlInputElement,
        attr_name = "type",
        csr {
            update: |el: &mut ET::HtmlInputElement<Renderer>, renderer: &mut _, _, v: &_| el
                .set_type(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::value(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlInputElement,
        attr_name = "value",
        csr {
            update: |el: &mut ET::HtmlInputElement<Renderer>, renderer: &mut _, _, v: &_| el
                .set_value(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::width(
        bounds as crate::imports::impl_bounds::MaybeValue<u32>,
        element as HtmlInputElement,
        attr_name = "width",
        csr {
            update: |el: &mut ET::HtmlInputElement<Renderer>, renderer: &mut _, _, &v: &_| el
                .set_width(renderer, v),
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
