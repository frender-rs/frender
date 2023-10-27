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
                    #[event(
                        crate::imports::frender_html::html::event_type_helpers::on_composition_end
                    )]
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
                    #[event(crate::imports::frender_html::html::event_type_helpers::on_focus_in)]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_focus_out:
                bounds![
                    #[event(crate::imports::frender_html::html::event_type_helpers::on_focus_out)]
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
                    #[event(
                        crate::imports::frender_html::html::event_type_helpers::on_fullscreen_error
                    )]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_key_down:
                bounds![
                    #[event(crate::imports::frender_html::html::event_type_helpers::on_key_down)]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_key_up:
                bounds![
                    #[event(crate::imports::frender_html::html::event_type_helpers::on_key_up)]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_aux_click:
                bounds![
                    #[event(crate::imports::frender_html::html::event_type_helpers::on_aux_click)]
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
                    #[event(crate::imports::frender_html::html::event_type_helpers::on_mouse_down)]
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
                    #[event(crate::imports::frender_html::html::event_type_helpers::on_mouse_move)]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_mouse_out:
                bounds![
                    #[event(crate::imports::frender_html::html::event_type_helpers::on_mouse_out)]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_mouse_over:
                bounds![
                    #[event(crate::imports::frender_html::html::event_type_helpers::on_mouse_over)]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_mouse_up:
                bounds![
                    #[event(crate::imports::frender_html::html::event_type_helpers::on_mouse_up)]
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
                    #[event(crate::imports::frender_html::html::event_type_helpers::on_touch_end)]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_touch_move:
                bounds![
                    #[event(crate::imports::frender_html::html::event_type_helpers::on_touch_move)]
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
                #[event(crate::imports::frender_html::html::event_type_helpers::on_animation_end)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_animation_iteration:
            bounds![
                #[event(
                    crate::imports::frender_html::html::event_type_helpers::on_animation_iteration
                )]
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
                #[event(crate::imports::frender_html::html::event_type_helpers::on_before_input)]
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
                #[event(
                    crate::imports::frender_html::html::event_type_helpers::on_got_pointer_capture
                )]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_lost_pointer_capture:
            bounds![
                #[event(
                    crate::imports::frender_html::html::event_type_helpers::on_lost_pointer_capture
                )]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_pointer_cancel:
            bounds![
                #[event(crate::imports::frender_html::html::event_type_helpers::on_pointer_cancel)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_pointer_down:
            bounds![
                #[event(crate::imports::frender_html::html::event_type_helpers::on_pointer_down)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_pointer_enter:
            bounds![
                #[event(crate::imports::frender_html::html::event_type_helpers::on_pointer_enter)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_pointer_leave:
            bounds![
                #[event(crate::imports::frender_html::html::event_type_helpers::on_pointer_leave)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_pointer_move:
            bounds![
                #[event(crate::imports::frender_html::html::event_type_helpers::on_pointer_move)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_pointer_out:
            bounds![
                #[event(crate::imports::frender_html::html::event_type_helpers::on_pointer_out)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_pointer_over:
            bounds![
                #[event(crate::imports::frender_html::html::event_type_helpers::on_pointer_over)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_pointer_up:
            bounds![
                #[event(crate::imports::frender_html::html::event_type_helpers::on_pointer_up)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_transition_cancel:
            bounds![
                #[event(
                    crate::imports::frender_html::html::event_type_helpers::on_transition_cancel
                )]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_transition_end:
            bounds![
                #[event(crate::imports::frender_html::html::event_type_helpers::on_transition_end)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_transition_run:
            bounds![
                #[event(crate::imports::frender_html::html::event_type_helpers::on_transition_run)]
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
    )
);
#[cfg(feature = "csr")]
mod imp {
    #![allow(unused_variables)]
    #[allow(unused_imports)]
    use super::super::*;
    crate::imports::impl_bounds!(super::props::access_key(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlElement,
        attr_name = "accesskey",
        csr {
            update: |el: &mut ET::HtmlElement<Renderer>, renderer: &mut _, _, v: &_| el
                .set_access_key(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::auto_capitalize(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlElement,
        attr_name = "autocapitalize",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::auto_focus(
        bounds as crate::imports::impl_bounds::MaybeValue<bool>,
        element as HtmlElement,
        attr_name = "autofocus",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::content_editable(
        bounds as MaybeContentEditable,
        element as HtmlElement,
        attr_name = "contenteditable",
        csr {
            update: |el: &mut _, renderer: &mut _, _, v: &_| {
                frender_html::renderer::node_behaviors::HtmlElement::set_content_editable(
                    el, renderer, v,
                )
            },
            remove: MaybeContentEditable::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::context_menu(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlElement,
        attr_name = "contextmenu",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::dir(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlElement,
        attr_name = "dir",
        csr {
            update: |el: &mut ET::HtmlElement<Renderer>, renderer: &mut _, _, v: &_| el
                .set_dir(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::draggable(
        bounds as crate::imports::impl_bounds::MaybeValue<bool>,
        element as HtmlElement,
        attr_name = "draggable",
        csr {
            update: |el: &mut ET::HtmlElement<Renderer>, renderer: &mut _, _, &v: &_| el
                .set_draggable(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::enter_key_hint(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlElement,
        attr_name = "enterkeyhint",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::hidden(
        bounds as crate::imports::impl_bounds::MaybeValue<bool>,
        element as HtmlElement,
        attr_name = "hidden",
        csr {
            update: |el: &mut ET::HtmlElement<Renderer>, renderer: &mut _, _, &v: &_| el
                .set_hidden(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::inert(
        bounds as crate::imports::impl_bounds::MaybeValue<bool>,
        element as HtmlElement,
        attr_name = "inert",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::input_mode(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlElement,
        attr_name = "inputmode",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::is(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlElement,
        attr_name = "is",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::item_id(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlElement,
        attr_name = "itemid",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::item_prop(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlElement,
        attr_name = "itemprop",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::item_ref(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlElement,
        attr_name = "itemref",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::item_scope(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlElement,
        attr_name = "itemscope",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::item_type(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlElement,
        attr_name = "itemtype",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::lang(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlElement,
        attr_name = "lang",
        csr {
            update: |el: &mut ET::HtmlElement<Renderer>, renderer: &mut _, _, v: &_| el
                .set_lang(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::nonce(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlElement,
        attr_name = "nonce",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::role(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlElement,
        attr_name = "role",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::slot(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlElement,
        attr_name = "slot",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::spellcheck(
        bounds as crate::imports::impl_bounds::MaybeValue<bool>,
        element as HtmlElement,
        attr_name = "spellcheck",
        csr {
            update: |el: &mut ET::HtmlElement<Renderer>, renderer: &mut _, _, &v: &_| el
                .set_spellcheck(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::style(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlElement,
        attr_name = "style",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::tab_index(
        bounds as crate::imports::impl_bounds::MaybeValue<i32>,
        element as HtmlElement,
        attr_name = "tabindex",
        csr {
            update: |el: &mut ET::HtmlElement<Renderer>, renderer: &mut _, _, &v: &_| el
                .set_tab_index(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::title(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlElement,
        attr_name = "title",
        csr {
            update: |el: &mut ET::HtmlElement<Renderer>, renderer: &mut _, _, v: &_| el
                .set_title(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::translate(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlElement,
        attr_name = "translate",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::virtual_keyboard_policy(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlElement,
        attr_name = "virtualkeyboardpolicy",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::on_invalid(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_invalid)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlElement,
        attr_name = "on_invalid",
    ));
    crate::imports::impl_bounds!(super::props::on_animation_cancel(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_animation_cancel)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlElement,
        attr_name = "on_animation_cancel",
    ));
    crate::imports::impl_bounds!(super::props::on_animation_end(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_animation_end)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlElement,
        attr_name = "on_animation_end",
    ));
    crate::imports::impl_bounds!(super::props::on_animation_iteration(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_animation_iteration)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlElement,
        attr_name = "on_animation_iteration",
    ));
    crate::imports::impl_bounds!(super::props::on_animation_start(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_animation_start)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlElement,
        attr_name = "on_animation_start",
    ));
    crate::imports::impl_bounds!(super::props::on_before_input(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_before_input)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlElement,
        attr_name = "on_before_input",
    ));
    crate::imports::impl_bounds!(super::props::on_input(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_input)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlElement,
        attr_name = "on_input",
    ));
    crate::imports::impl_bounds!(super::props::on_change(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_change)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlElement,
        attr_name = "on_change",
    ));
    crate::imports::impl_bounds!(super::props::on_got_pointer_capture(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_got_pointer_capture)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlElement,
        attr_name = "on_got_pointer_capture",
    ));
    crate::imports::impl_bounds!(super::props::on_lost_pointer_capture(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_lost_pointer_capture)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlElement,
        attr_name = "on_lost_pointer_capture",
    ));
    crate::imports::impl_bounds!(super::props::on_pointer_cancel(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_pointer_cancel)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlElement,
        attr_name = "on_pointer_cancel",
    ));
    crate::imports::impl_bounds!(super::props::on_pointer_down(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_pointer_down)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlElement,
        attr_name = "on_pointer_down",
    ));
    crate::imports::impl_bounds!(super::props::on_pointer_enter(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_pointer_enter)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlElement,
        attr_name = "on_pointer_enter",
    ));
    crate::imports::impl_bounds!(super::props::on_pointer_leave(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_pointer_leave)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlElement,
        attr_name = "on_pointer_leave",
    ));
    crate::imports::impl_bounds!(super::props::on_pointer_move(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_pointer_move)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlElement,
        attr_name = "on_pointer_move",
    ));
    crate::imports::impl_bounds!(super::props::on_pointer_out(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_pointer_out)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlElement,
        attr_name = "on_pointer_out",
    ));
    crate::imports::impl_bounds!(super::props::on_pointer_over(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_pointer_over)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlElement,
        attr_name = "on_pointer_over",
    ));
    crate::imports::impl_bounds!(super::props::on_pointer_up(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_pointer_up)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlElement,
        attr_name = "on_pointer_up",
    ));
    crate::imports::impl_bounds!(super::props::on_transition_cancel(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_transition_cancel)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlElement,
        attr_name = "on_transition_cancel",
    ));
    crate::imports::impl_bounds!(super::props::on_transition_end(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_transition_end)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlElement,
        attr_name = "on_transition_end",
    ));
    crate::imports::impl_bounds!(super::props::on_transition_run(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_transition_run)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlElement,
        attr_name = "on_transition_run",
    ));
    crate::imports::impl_bounds!(super::props::on_transition_start(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_transition_start)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlElement,
        attr_name = "on_transition_start",
    ));
    crate::imports::impl_bounds!(super::props::on_drag(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_drag)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlElement,
        attr_name = "on_drag",
    ));
    crate::imports::impl_bounds!(super::props::on_drag_end(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_drag_end)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlElement,
        attr_name = "on_drag_end",
    ));
    crate::imports::impl_bounds!(super::props::on_drag_enter(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_drag_enter)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlElement,
        attr_name = "on_drag_enter",
    ));
    crate::imports::impl_bounds!(super::props::on_drag_leave(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_drag_leave)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlElement,
        attr_name = "on_drag_leave",
    ));
    crate::imports::impl_bounds!(super::props::on_drag_over(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_drag_over)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlElement,
        attr_name = "on_drag_over",
    ));
    crate::imports::impl_bounds!(super::props::on_drag_start(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_drag_start)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlElement,
        attr_name = "on_drag_start",
    ));
    crate::imports::impl_bounds!(super::props::on_drop(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_drop)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlElement,
        attr_name = "on_drop",
    ));
}
mod imports {
    #[allow(unused_imports)]
    use super::super::*;
    pub(super) use crate::imports::frender_html_simple::def_props_type;
}
use imports::def_props_type;
