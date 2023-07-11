def_props_type!(
    #[derive(Debug, Clone, Copy, Default)]
    HtmlMediaElementProps(
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
        auto_play: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<bool>],
        controls: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<bool>],
        cross_origin: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        r#loop: alias![loop_] + bounds![crate::imports::impl_bounds::MaybeValue::Bounds<bool>],
        muted: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<bool>],
        preload: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        src: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        on_abort: bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        on_can_play: bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        on_can_play_through:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        on_duration_change:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        on_emptied: bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        on_ended: bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        on_loaded_data:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        on_loaded_metadata:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        on_load_start:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        on_pause: bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        on_play: bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        on_playing: bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        on_progress: bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        on_rate_change:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        on_resize: bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        on_seeked: bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        on_seeking: bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        on_stalled: bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        on_suspend: bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        on_time_update:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        on_volume_change:
            bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
        on_waiting: bounds![crate::imports::impl_bounds::MaybeHandleEvent::Bounds<events::Event>],
    )
);
#[cfg(feature = "csr")]
mod imp {
    #![allow(unused_variables)]
    #[allow(unused_imports)]
    use super::super::*;
    crate::imports::impl_bounds!(super::props::auto_play(
        bounds as crate::imports::impl_bounds::MaybeValue<bool>,
        element as web_sys::HtmlMediaElement,
        attr_name = "autoplay",
        csr {
            update: |el: &web_sys::HtmlMediaElement, _, &v: &_| el.set_autoplay(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::controls(
        bounds as crate::imports::impl_bounds::MaybeValue<bool>,
        element as web_sys::HtmlMediaElement,
        attr_name = "controls",
        csr {
            update: |el: &web_sys::HtmlMediaElement, _, &v: &_| el.set_controls(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::cross_origin(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlMediaElement,
        attr_name = "crossorigin",
        csr {
            update: |el: &web_sys::HtmlMediaElement, _, v: &_| (|v: &_| el
                .set_cross_origin(Some(v)))(
                v
            ),
            remove: |el: &web_sys::HtmlMediaElement, _| (|| el.set_cross_origin(None))(),
        },
    ));
    crate::imports::impl_bounds!(super::props::r#loop(
        bounds as crate::imports::impl_bounds::MaybeValue<bool>,
        element as web_sys::HtmlMediaElement,
        attr_name = "loop",
        csr {
            update: |el: &web_sys::HtmlMediaElement, _, &v: &_| el.set_loop(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::muted(
        bounds as crate::imports::impl_bounds::MaybeValue<bool>,
        element as web_sys::HtmlMediaElement,
        attr_name = "muted",
        csr {
            update: |el: &web_sys::HtmlMediaElement, _, &v: &_| el.set_muted(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::preload(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlMediaElement,
        attr_name = "preload",
        csr {
            update: |el: &web_sys::HtmlMediaElement, _, v: &_| el.set_preload(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::src(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as web_sys::HtmlMediaElement,
        attr_name = "src",
        csr {
            update: |el: &web_sys::HtmlMediaElement, _, v: &_| el.set_src(v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::on_abort(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::HtmlMediaElement,
        attr_name = "abort",
    ));
    crate::imports::impl_bounds!(super::props::on_can_play(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::HtmlMediaElement,
        attr_name = "canplay",
    ));
    crate::imports::impl_bounds!(super::props::on_can_play_through(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::HtmlMediaElement,
        attr_name = "canplaythrough",
    ));
    crate::imports::impl_bounds!(super::props::on_duration_change(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::HtmlMediaElement,
        attr_name = "durationchange",
    ));
    crate::imports::impl_bounds!(super::props::on_emptied(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::HtmlMediaElement,
        attr_name = "emptied",
    ));
    crate::imports::impl_bounds!(super::props::on_ended(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::HtmlMediaElement,
        attr_name = "ended",
    ));
    crate::imports::impl_bounds!(super::props::on_loaded_data(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::HtmlMediaElement,
        attr_name = "loadeddata",
    ));
    crate::imports::impl_bounds!(super::props::on_loaded_metadata(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::HtmlMediaElement,
        attr_name = "loadedmetadata",
    ));
    crate::imports::impl_bounds!(super::props::on_load_start(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::HtmlMediaElement,
        attr_name = "loadstart",
    ));
    crate::imports::impl_bounds!(super::props::on_pause(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::HtmlMediaElement,
        attr_name = "pause",
    ));
    crate::imports::impl_bounds!(super::props::on_play(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::HtmlMediaElement,
        attr_name = "play",
    ));
    crate::imports::impl_bounds!(super::props::on_playing(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::HtmlMediaElement,
        attr_name = "playing",
    ));
    crate::imports::impl_bounds!(super::props::on_progress(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::HtmlMediaElement,
        attr_name = "progress",
    ));
    crate::imports::impl_bounds!(super::props::on_rate_change(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::HtmlMediaElement,
        attr_name = "ratechange",
    ));
    crate::imports::impl_bounds!(super::props::on_resize(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::HtmlMediaElement,
        attr_name = "resize",
    ));
    crate::imports::impl_bounds!(super::props::on_seeked(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::HtmlMediaElement,
        attr_name = "seeked",
    ));
    crate::imports::impl_bounds!(super::props::on_seeking(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::HtmlMediaElement,
        attr_name = "seeking",
    ));
    crate::imports::impl_bounds!(super::props::on_stalled(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::HtmlMediaElement,
        attr_name = "stalled",
    ));
    crate::imports::impl_bounds!(super::props::on_suspend(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::HtmlMediaElement,
        attr_name = "suspend",
    ));
    crate::imports::impl_bounds!(super::props::on_time_update(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::HtmlMediaElement,
        attr_name = "timeupdate",
    ));
    crate::imports::impl_bounds!(super::props::on_volume_change(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::HtmlMediaElement,
        attr_name = "volumechange",
    ));
    crate::imports::impl_bounds!(super::props::on_waiting(
        bounds as crate::imports::impl_bounds::MaybeHandleEvent<events::Event>,
        element as web_sys::HtmlMediaElement,
        attr_name = "waiting",
    ));
}
mod imports {
    #[allow(unused_imports)]
    use super::super::*;
    pub(super) use crate::imports::frender_html_simple::def_props_type;
}
use imports::def_props_type;
