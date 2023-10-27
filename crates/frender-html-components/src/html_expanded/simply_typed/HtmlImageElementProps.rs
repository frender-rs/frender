def_props_type!(
    #[derive(Debug, Clone, Copy, Default)]
    HtmlImageElementProps(
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
        alt: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        cross_origin: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        decoding: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        element_timing: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        height: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<u32>],
        is_map: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<bool>],
        loading: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        referrer_policy: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        sizes: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        src: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        srcset: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        width: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<u32>],
        use_map: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
    )
);
#[cfg(feature = "csr")]
mod imp {
    #![allow(unused_variables)]
    #[allow(unused_imports)]
    use super::super::*;
    crate::imports::impl_bounds!(super::props::alt(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlImageElement,
        attr_name = "alt",
        csr {
            update: |el: &mut ET::HtmlImageElement<Renderer>, renderer: &mut _, _, v: &_| el
                .set_alt(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::cross_origin(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlImageElement,
        attr_name = "crossorigin",
        csr {
            update: |element: &mut ET::HtmlImageElement<Renderer>,
                     renderer: &mut Renderer,
                     _: &_,
                     value: &_| element.set_cross_origin(renderer, Some(value)),
            remove: |element: &mut ET::HtmlImageElement<Renderer>,
                     renderer: &mut Renderer,
                     _: &_| element.set_cross_origin(renderer, None),
        },
    ));
    crate::imports::impl_bounds!(super::props::decoding(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlImageElement,
        attr_name = "decoding",
        csr {
            update: |el: &mut ET::HtmlImageElement<Renderer>, renderer: &mut _, _, v: &_| el
                .set_decoding(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::element_timing(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlImageElement,
        attr_name = "elementtiming",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::height(
        bounds as crate::imports::impl_bounds::MaybeValue<u32>,
        element as HtmlImageElement,
        attr_name = "height",
        csr {
            update: |el: &mut ET::HtmlImageElement<Renderer>, renderer: &mut _, _, &v: &_| el
                .set_height(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::is_map(
        bounds as crate::imports::impl_bounds::MaybeValue<bool>,
        element as HtmlImageElement,
        attr_name = "ismap",
        csr {
            update: |el: &mut ET::HtmlImageElement<Renderer>, renderer: &mut _, _, &v: &_| el
                .set_is_map(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::loading(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlImageElement,
        attr_name = "loading",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::referrer_policy(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlImageElement,
        attr_name = "referrerpolicy",
        csr {
            update: |el: &mut ET::HtmlImageElement<Renderer>, renderer: &mut _, _, v: &_| el
                .set_referrer_policy(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::sizes(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlImageElement,
        attr_name = "sizes",
        csr {
            update: |el: &mut ET::HtmlImageElement<Renderer>, renderer: &mut _, _, v: &_| el
                .set_sizes(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::src(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlImageElement,
        attr_name = "src",
        csr {
            update: |el: &mut ET::HtmlImageElement<Renderer>, renderer: &mut _, _, v: &_| el
                .set_src(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::srcset(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlImageElement,
        attr_name = "srcset",
        csr {
            update: |el: &mut ET::HtmlImageElement<Renderer>, renderer: &mut _, _, v: &_| el
                .set_srcset(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::width(
        bounds as crate::imports::impl_bounds::MaybeValue<u32>,
        element as HtmlImageElement,
        attr_name = "width",
        csr {
            update: |el: &mut ET::HtmlImageElement<Renderer>, renderer: &mut _, _, &v: &_| el
                .set_width(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::use_map(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlImageElement,
        attr_name = "usemap",
        csr {
            update: |el: &mut ET::HtmlImageElement<Renderer>, renderer: &mut _, _, v: &_| el
                .set_use_map(renderer, v),
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
