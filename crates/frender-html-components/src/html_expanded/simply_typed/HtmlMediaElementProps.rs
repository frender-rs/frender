def_props_type!(
    #[derive(Debug, Clone, Copy, Default)]
    HtmlMediaElementProps(
        ..HtmlElementProps(
            ..ElementProps(
                children,
                css: bounds![Css::Bounds],
                class: bounds![DomTokens::Bounds],
                id: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
                part: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
                on_cancel:
                    bounds![
                        #[event(
                            crate::imports::frender_html::event_types::type_of_event::on_cancel
                        )]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_error:
                    bounds![
                        #[event(
                            crate::imports::frender_html::event_types::type_of_event::on_error
                        )]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_scroll:
                    bounds![
                        #[event(
                            crate::imports::frender_html::event_types::type_of_event::on_scroll
                        )]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_security_policy_violation:
                    bounds![
                        #[event(crate
    ::imports::frender_html::event_types::type_of_event::on_security_policy_violation)]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_select:
                    bounds![
                        #[event(
                            crate::imports::frender_html::event_types::type_of_event::on_select
                        )]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_wheel:
                    bounds![
                        #[event(
                            crate::imports::frender_html::event_types::type_of_event::on_wheel
                        )]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_copy:
                    bounds![
                        #[event(crate::imports::frender_html::event_types::type_of_event::on_copy)]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_cut:
                    bounds![
                        #[event(crate::imports::frender_html::event_types::type_of_event::on_cut)]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_paste:
                    bounds![
                        #[event(
                            crate::imports::frender_html::event_types::type_of_event::on_paste
                        )]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_composition_end:
                    bounds![
                        #[event(crate
    ::imports::frender_html::event_types::type_of_event::on_composition_end)]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_composition_start:
                    bounds![
                        #[event(crate
    ::imports::frender_html::event_types::type_of_event::on_composition_start)]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_composition_update:
                    bounds![
                        #[event(crate
    ::imports::frender_html::event_types::type_of_event::on_composition_update)]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_blur:
                    bounds![
                        #[event(crate::imports::frender_html::event_types::type_of_event::on_blur)]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_focus:
                    bounds![
                        #[event(
                            crate::imports::frender_html::event_types::type_of_event::on_focus
                        )]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_focus_in:
                    bounds![
                        #[event(
                            crate::imports::frender_html::event_types::type_of_event::on_focus_in
                        )]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_focus_out:
                    bounds![
                        #[event(
                            crate::imports::frender_html::event_types::type_of_event::on_focus_out
                        )]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_fullscreen_change:
                    bounds![
                        #[event(crate
    ::imports::frender_html::event_types::type_of_event::on_fullscreen_change)]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_fullscreen_error:
                    bounds![
                        #[event(crate
    ::imports::frender_html::event_types::type_of_event::on_fullscreen_error)]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_key_down:
                    bounds![
                        #[event(
                            crate::imports::frender_html::event_types::type_of_event::on_key_down
                        )]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_key_up:
                    bounds![
                        #[event(
                            crate::imports::frender_html::event_types::type_of_event::on_key_up
                        )]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_aux_click:
                    bounds![
                        #[event(
                            crate::imports::frender_html::event_types::type_of_event::on_aux_click
                        )]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_click:
                    bounds![
                        #[event(
                            crate::imports::frender_html::event_types::type_of_event::on_click
                        )]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_context_menu:
                    bounds![
                        #[event(crate
    ::imports::frender_html::event_types::type_of_event::on_context_menu)]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_double_click:
                    bounds![
                        #[event(crate
    ::imports::frender_html::event_types::type_of_event::on_double_click)]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_mouse_down:
                    bounds![
                        #[event(
                            crate::imports::frender_html::event_types::type_of_event::on_mouse_down
                        )]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_mouse_enter:
                    bounds![
                        #[event(crate
    ::imports::frender_html::event_types::type_of_event::on_mouse_enter)]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_mouse_leave:
                    bounds![
                        #[event(crate
    ::imports::frender_html::event_types::type_of_event::on_mouse_leave)]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_mouse_move:
                    bounds![
                        #[event(
                            crate::imports::frender_html::event_types::type_of_event::on_mouse_move
                        )]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_mouse_out:
                    bounds![
                        #[event(
                            crate::imports::frender_html::event_types::type_of_event::on_mouse_out
                        )]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_mouse_over:
                    bounds![
                        #[event(
                            crate::imports::frender_html::event_types::type_of_event::on_mouse_over
                        )]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_mouse_up:
                    bounds![
                        #[event(
                            crate::imports::frender_html::event_types::type_of_event::on_mouse_up
                        )]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_touch_cancel:
                    bounds![
                        #[event(crate
    ::imports::frender_html::event_types::type_of_event::on_touch_cancel)]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_touch_end:
                    bounds![
                        #[event(
                            crate::imports::frender_html::event_types::type_of_event::on_touch_end
                        )]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_touch_move:
                    bounds![
                        #[event(
                            crate::imports::frender_html::event_types::type_of_event::on_touch_move
                        )]
                        crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                    ],
                on_touch_start:
                    bounds![
                        #[event(crate
    ::imports::frender_html::event_types::type_of_event::on_touch_start)]
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
                    #[event(crate::imports::frender_html::event_types::type_of_event::on_invalid)]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_animation_cancel:
                bounds![
                    #[event(crate
    ::imports::frender_html::event_types::type_of_event::on_animation_cancel)]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_animation_end:
                bounds![
                    #[event(
                        crate::imports::frender_html::event_types::type_of_event::on_animation_end
                    )]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_animation_iteration:
                bounds![
                    #[event(crate
    ::imports::frender_html::event_types::type_of_event::on_animation_iteration)]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_animation_start:
                bounds![
                    #[event(crate
    ::imports::frender_html::event_types::type_of_event::on_animation_start)]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_before_input:
                bounds![
                    #[event(
                        crate::imports::frender_html::event_types::type_of_event::on_before_input
                    )]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_input:
                bounds![
                    #[event(crate::imports::frender_html::event_types::type_of_event::on_input)]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_change:
                bounds![
                    #[event(crate::imports::frender_html::event_types::type_of_event::on_change)]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_got_pointer_capture:
                bounds![
                    #[event(crate
    ::imports::frender_html::event_types::type_of_event::on_got_pointer_capture)]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_lost_pointer_capture:
                bounds![
                    #[event(crate
    ::imports::frender_html::event_types::type_of_event::on_lost_pointer_capture)]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_pointer_cancel:
                bounds![
                    #[event(
                        crate::imports::frender_html::event_types::type_of_event::on_pointer_cancel
                    )]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_pointer_down:
                bounds![
                    #[event(
                        crate::imports::frender_html::event_types::type_of_event::on_pointer_down
                    )]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_pointer_enter:
                bounds![
                    #[event(
                        crate::imports::frender_html::event_types::type_of_event::on_pointer_enter
                    )]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_pointer_leave:
                bounds![
                    #[event(
                        crate::imports::frender_html::event_types::type_of_event::on_pointer_leave
                    )]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_pointer_move:
                bounds![
                    #[event(
                        crate::imports::frender_html::event_types::type_of_event::on_pointer_move
                    )]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_pointer_out:
                bounds![
                    #[event(
                        crate::imports::frender_html::event_types::type_of_event::on_pointer_out
                    )]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_pointer_over:
                bounds![
                    #[event(
                        crate::imports::frender_html::event_types::type_of_event::on_pointer_over
                    )]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_pointer_up:
                bounds![
                    #[event(
                        crate::imports::frender_html::event_types::type_of_event::on_pointer_up
                    )]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_transition_cancel:
                bounds![
                    #[event(crate
    ::imports::frender_html::event_types::type_of_event::on_transition_cancel)]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_transition_end:
                bounds![
                    #[event(
                        crate::imports::frender_html::event_types::type_of_event::on_transition_end
                    )]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_transition_run:
                bounds![
                    #[event(
                        crate::imports::frender_html::event_types::type_of_event::on_transition_run
                    )]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_transition_start:
                bounds![
                    #[event(crate
    ::imports::frender_html::event_types::type_of_event::on_transition_start)]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_drag:
                bounds![
                    #[event(crate::imports::frender_html::event_types::type_of_event::on_drag)]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_drag_end:
                bounds![
                    #[event(crate::imports::frender_html::event_types::type_of_event::on_drag_end)]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_drag_enter:
                bounds![
                    #[event(
                        crate::imports::frender_html::event_types::type_of_event::on_drag_enter
                    )]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_drag_leave:
                bounds![
                    #[event(
                        crate::imports::frender_html::event_types::type_of_event::on_drag_leave
                    )]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_drag_over:
                bounds![
                    #[event(
                        crate::imports::frender_html::event_types::type_of_event::on_drag_over
                    )]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_drag_start:
                bounds![
                    #[event(
                        crate::imports::frender_html::event_types::type_of_event::on_drag_start
                    )]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
            on_drop:
                bounds![
                    #[event(crate::imports::frender_html::event_types::type_of_event::on_drop)]
                    crate::imports::impl_bounds::MaybeHandleEvent::Bounds
                ],
        ),
        auto_play: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<bool>],
        controls: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<bool>],
        cross_origin: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        r#loop: alias![loop_] + bounds![crate::imports::impl_bounds::MaybeValue::Bounds<bool>],
        muted: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<bool>],
        preload: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        src: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        on_abort:
            bounds![
                #[event(crate::imports::frender_html::event_types::type_of_event::on_abort)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_can_play:
            bounds![
                #[event(crate::imports::frender_html::event_types::type_of_event::on_can_play)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_can_play_through:
            bounds![
                #[event(
                    crate::imports::frender_html::event_types::type_of_event::on_can_play_through
                )]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_duration_change:
            bounds![
                #[event(
                    crate::imports::frender_html::event_types::type_of_event::on_duration_change
                )]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_emptied:
            bounds![
                #[event(crate::imports::frender_html::event_types::type_of_event::on_emptied)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_ended:
            bounds![
                #[event(crate::imports::frender_html::event_types::type_of_event::on_ended)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_loaded_data:
            bounds![
                #[event(crate::imports::frender_html::event_types::type_of_event::on_loaded_data)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_loaded_metadata:
            bounds![
                #[event(
                    crate::imports::frender_html::event_types::type_of_event::on_loaded_metadata
                )]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_load_start:
            bounds![
                #[event(crate::imports::frender_html::event_types::type_of_event::on_load_start)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_pause:
            bounds![
                #[event(crate::imports::frender_html::event_types::type_of_event::on_pause)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_play:
            bounds![
                #[event(crate::imports::frender_html::event_types::type_of_event::on_play)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_playing:
            bounds![
                #[event(crate::imports::frender_html::event_types::type_of_event::on_playing)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_progress:
            bounds![
                #[event(crate::imports::frender_html::event_types::type_of_event::on_progress)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_rate_change:
            bounds![
                #[event(crate::imports::frender_html::event_types::type_of_event::on_rate_change)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_resize:
            bounds![
                #[event(crate::imports::frender_html::event_types::type_of_event::on_resize)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_seeked:
            bounds![
                #[event(crate::imports::frender_html::event_types::type_of_event::on_seeked)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_seeking:
            bounds![
                #[event(crate::imports::frender_html::event_types::type_of_event::on_seeking)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_stalled:
            bounds![
                #[event(crate::imports::frender_html::event_types::type_of_event::on_stalled)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_suspend:
            bounds![
                #[event(crate::imports::frender_html::event_types::type_of_event::on_suspend)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_time_update:
            bounds![
                #[event(crate::imports::frender_html::event_types::type_of_event::on_time_update)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_volume_change:
            bounds![
                #[event(
                    crate::imports::frender_html::event_types::type_of_event::on_volume_change
                )]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_waiting:
            bounds![
                #[event(crate::imports::frender_html::event_types::type_of_event::on_waiting)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
    )
);
#[cfg(feature = "csr")]
mod imp {
    #![allow(unused_variables)]
    #[allow(unused_imports)]
    use super::super::*;
    crate::imports::impl_bounds!(super::props::auto_play(
        bounds as crate::imports::impl_bounds::MaybeValue<bool>,
        element as HtmlMediaElement,
        attr_name = "autoplay",
        csr {
            update: |el: &mut ET::HtmlMediaElement<Renderer>, renderer: &mut _, _, &v: &_| el
                .set_auto_play(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::controls(
        bounds as crate::imports::impl_bounds::MaybeValue<bool>,
        element as HtmlMediaElement,
        attr_name = "controls",
        csr {
            update: |el: &mut ET::HtmlMediaElement<Renderer>, renderer: &mut _, _, &v: &_| el
                .set_controls(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::cross_origin(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlMediaElement,
        attr_name = "crossorigin",
        csr {
            update: |element: &mut ET::HtmlMediaElement<Renderer>,
                     renderer: &mut Renderer,
                     _: &_,
                     value: &_| element.set_cross_origin(renderer, Some(value)),
            remove: |element: &mut ET::HtmlMediaElement<Renderer>,
                     renderer: &mut Renderer,
                     _: &_| element.set_cross_origin(renderer, None),
        },
    ));
    crate::imports::impl_bounds!(super::props::r#loop(
        bounds as crate::imports::impl_bounds::MaybeValue<bool>,
        element as HtmlMediaElement,
        attr_name = "loop",
        csr {
            update: |el: &mut ET::HtmlMediaElement<Renderer>, renderer: &mut _, _, &v: &_| el
                .set_loop(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::muted(
        bounds as crate::imports::impl_bounds::MaybeValue<bool>,
        element as HtmlMediaElement,
        attr_name = "muted",
        csr {
            update: |el: &mut ET::HtmlMediaElement<Renderer>, renderer: &mut _, _, &v: &_| el
                .set_muted(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::preload(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlMediaElement,
        attr_name = "preload",
        csr {
            update: |el: &mut ET::HtmlMediaElement<Renderer>, renderer: &mut _, _, v: &_| el
                .set_preload(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::src(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as HtmlMediaElement,
        attr_name = "src",
        csr {
            update: |el: &mut ET::HtmlMediaElement<Renderer>, renderer: &mut _, _, v: &_| el
                .set_src(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::on_abort(
        #[event(crate::imports::frender_html::event_types::type_of_event::on_abort)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlMediaElement,
        attr_name = "on_abort",
    ));
    crate::imports::impl_bounds!(super::props::on_can_play(
        #[event(crate::imports::frender_html::event_types::type_of_event::on_can_play)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlMediaElement,
        attr_name = "on_can_play",
    ));
    crate::imports::impl_bounds!(super::props::on_can_play_through(
        #[event(crate::imports::frender_html::event_types::type_of_event::on_can_play_through)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlMediaElement,
        attr_name = "on_can_play_through",
    ));
    crate::imports::impl_bounds!(super::props::on_duration_change(
        #[event(crate::imports::frender_html::event_types::type_of_event::on_duration_change)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlMediaElement,
        attr_name = "on_duration_change",
    ));
    crate::imports::impl_bounds!(super::props::on_emptied(
        #[event(crate::imports::frender_html::event_types::type_of_event::on_emptied)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlMediaElement,
        attr_name = "on_emptied",
    ));
    crate::imports::impl_bounds!(super::props::on_ended(
        #[event(crate::imports::frender_html::event_types::type_of_event::on_ended)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlMediaElement,
        attr_name = "on_ended",
    ));
    crate::imports::impl_bounds!(super::props::on_loaded_data(
        #[event(crate::imports::frender_html::event_types::type_of_event::on_loaded_data)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlMediaElement,
        attr_name = "on_loaded_data",
    ));
    crate::imports::impl_bounds!(super::props::on_loaded_metadata(
        #[event(crate::imports::frender_html::event_types::type_of_event::on_loaded_metadata)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlMediaElement,
        attr_name = "on_loaded_metadata",
    ));
    crate::imports::impl_bounds!(super::props::on_load_start(
        #[event(crate::imports::frender_html::event_types::type_of_event::on_load_start)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlMediaElement,
        attr_name = "on_load_start",
    ));
    crate::imports::impl_bounds!(super::props::on_pause(
        #[event(crate::imports::frender_html::event_types::type_of_event::on_pause)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlMediaElement,
        attr_name = "on_pause",
    ));
    crate::imports::impl_bounds!(super::props::on_play(
        #[event(crate::imports::frender_html::event_types::type_of_event::on_play)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlMediaElement,
        attr_name = "on_play",
    ));
    crate::imports::impl_bounds!(super::props::on_playing(
        #[event(crate::imports::frender_html::event_types::type_of_event::on_playing)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlMediaElement,
        attr_name = "on_playing",
    ));
    crate::imports::impl_bounds!(super::props::on_progress(
        #[event(crate::imports::frender_html::event_types::type_of_event::on_progress)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlMediaElement,
        attr_name = "on_progress",
    ));
    crate::imports::impl_bounds!(super::props::on_rate_change(
        #[event(crate::imports::frender_html::event_types::type_of_event::on_rate_change)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlMediaElement,
        attr_name = "on_rate_change",
    ));
    crate::imports::impl_bounds!(super::props::on_resize(
        #[event(crate::imports::frender_html::event_types::type_of_event::on_resize)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlMediaElement,
        attr_name = "on_resize",
    ));
    crate::imports::impl_bounds!(super::props::on_seeked(
        #[event(crate::imports::frender_html::event_types::type_of_event::on_seeked)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlMediaElement,
        attr_name = "on_seeked",
    ));
    crate::imports::impl_bounds!(super::props::on_seeking(
        #[event(crate::imports::frender_html::event_types::type_of_event::on_seeking)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlMediaElement,
        attr_name = "on_seeking",
    ));
    crate::imports::impl_bounds!(super::props::on_stalled(
        #[event(crate::imports::frender_html::event_types::type_of_event::on_stalled)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlMediaElement,
        attr_name = "on_stalled",
    ));
    crate::imports::impl_bounds!(super::props::on_suspend(
        #[event(crate::imports::frender_html::event_types::type_of_event::on_suspend)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlMediaElement,
        attr_name = "on_suspend",
    ));
    crate::imports::impl_bounds!(super::props::on_time_update(
        #[event(crate::imports::frender_html::event_types::type_of_event::on_time_update)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlMediaElement,
        attr_name = "on_time_update",
    ));
    crate::imports::impl_bounds!(super::props::on_volume_change(
        #[event(crate::imports::frender_html::event_types::type_of_event::on_volume_change)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlMediaElement,
        attr_name = "on_volume_change",
    ));
    crate::imports::impl_bounds!(super::props::on_waiting(
        #[event(crate::imports::frender_html::event_types::type_of_event::on_waiting)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as HtmlMediaElement,
        attr_name = "on_waiting",
    ));
}
mod imports {
    #[allow(unused_imports)]
    use super::super::*;
    pub(super) use crate::imports::frender_html_simple::def_props_type;
}
use imports::def_props_type;
