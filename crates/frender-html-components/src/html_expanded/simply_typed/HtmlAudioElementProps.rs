def_props_type!(
    #[derive(Debug, Clone, Copy, Default)]
    HtmlAudioElementProps(
        ..HtmlMediaElementProps(
            ..HtmlElementProps(
                ..ElementProps(
                    children,
                    class:
                        bounds![
                            crate::imports::frender_html::props::MaybeUpdateValueWithState<str>
                        ],
                    id: bounds![
                        crate::imports::frender_html::props::MaybeUpdateValueWithState<str>
                    ],
                    part: bounds![
                        crate::imports::frender_html::props::MaybeUpdateValueWithState<str>
                    ],
                    on_cancel:
                        bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
                    on_error:
                        bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
                    on_scroll:
                        bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
                    on_security_policy_violation:
                        bounds![
                            crate::imports::frender_events::MaybeHandleEvent<
                                events::SecurityPolicyViolationEvent,
                            >
                        ],
                    on_select:
                        bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
                    on_wheel:
                        bounds![
                            crate::imports::frender_events::MaybeHandleEvent<events::WheelEvent>
                        ],
                    on_copy:
                        bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
                    on_cut:
                        bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
                    on_paste:
                        bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
                    on_composition_end:
                        bounds![
                            crate::imports::frender_events::MaybeHandleEvent<
                                events::CompositionEvent,
                            >
                        ],
                    on_composition_start:
                        bounds![
                            crate::imports::frender_events::MaybeHandleEvent<
                                events::CompositionEvent,
                            >
                        ],
                    on_composition_update:
                        bounds![
                            crate::imports::frender_events::MaybeHandleEvent<
                                events::CompositionEvent,
                            >
                        ],
                    on_blur:
                        bounds![
                            crate::imports::frender_events::MaybeHandleEvent<events::FocusEvent>
                        ],
                    on_focus:
                        bounds![
                            crate::imports::frender_events::MaybeHandleEvent<events::FocusEvent>
                        ],
                    on_focus_in:
                        bounds![
                            crate::imports::frender_events::MaybeHandleEvent<events::FocusEvent>
                        ],
                    on_focus_out:
                        bounds![
                            crate::imports::frender_events::MaybeHandleEvent<events::FocusEvent>
                        ],
                    on_fullscreen_change:
                        bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
                    on_fullscreen_error:
                        bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
                    on_key_down:
                        bounds![
                            crate::imports::frender_events::MaybeHandleEvent<events::KeyboardEvent>
                        ],
                    on_key_up:
                        bounds![
                            crate::imports::frender_events::MaybeHandleEvent<events::KeyboardEvent>
                        ],
                    on_aux_click:
                        bounds![
                            crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>
                        ],
                    on_click:
                        bounds![
                            crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>
                        ],
                    on_context_menu:
                        bounds![
                            crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>
                        ],
                    on_double_click:
                        bounds![
                            crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>
                        ],
                    on_mouse_down:
                        bounds![
                            crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>
                        ],
                    on_mouse_enter:
                        bounds![
                            crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>
                        ],
                    on_mouse_leave:
                        bounds![
                            crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>
                        ],
                    on_mouse_move:
                        bounds![
                            crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>
                        ],
                    on_mouse_out:
                        bounds![
                            crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>
                        ],
                    on_mouse_over:
                        bounds![
                            crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>
                        ],
                    on_mouse_up:
                        bounds![
                            crate::imports::frender_events::MaybeHandleEvent<events::MouseEvent>
                        ],
                    on_touch_cancel:
                        bounds![
                            crate::imports::frender_events::MaybeHandleEvent<events::TouchEvent>
                        ],
                    on_touch_end:
                        bounds![
                            crate::imports::frender_events::MaybeHandleEvent<events::TouchEvent>
                        ],
                    on_touch_move:
                        bounds![
                            crate::imports::frender_events::MaybeHandleEvent<events::TouchEvent>
                        ],
                    on_touch_start:
                        bounds![
                            crate::imports::frender_events::MaybeHandleEvent<events::TouchEvent>
                        ],
                ),
                access_key:
                    bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
                auto_capitalize:
                    bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
                auto_focus:
                    bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>],
                context_menu:
                    bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
                dir: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
                draggable:
                    bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>],
                enter_key_hint:
                    bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
                hidden:
                    bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>],
                inert:
                    bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>],
                input_mode:
                    bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
                is: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
                item_id:
                    bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
                item_prop:
                    bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
                item_ref:
                    bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
                item_scope:
                    bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
                item_type:
                    bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
                lang: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
                nonce: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
                role: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
                slot: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
                spellcheck:
                    bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>],
                style: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
                tab_index:
                    bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<i32>],
                title: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
                translate:
                    bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
                virtual_keyboard_policy:
                    bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
                on_invalid:
                    bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
                on_animation_cancel:
                    bounds![
                        crate::imports::frender_events::MaybeHandleEvent<events::AnimationEvent>
                    ],
                on_animation_end:
                    bounds![
                        crate::imports::frender_events::MaybeHandleEvent<events::AnimationEvent>
                    ],
                on_animation_iteration:
                    bounds![
                        crate::imports::frender_events::MaybeHandleEvent<events::AnimationEvent>
                    ],
                on_animation_start:
                    bounds![
                        crate::imports::frender_events::MaybeHandleEvent<events::AnimationEvent>
                    ],
                on_before_input:
                    bounds![crate::imports::frender_events::MaybeHandleEvent<events::InputEvent>],
                on_input: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
                on_change: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
                on_got_pointer_capture:
                    bounds![crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>],
                on_lost_pointer_capture:
                    bounds![crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>],
                on_pointer_cancel:
                    bounds![crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>],
                on_pointer_down:
                    bounds![crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>],
                on_pointer_enter:
                    bounds![crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>],
                on_pointer_leave:
                    bounds![crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>],
                on_pointer_move:
                    bounds![crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>],
                on_pointer_out:
                    bounds![crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>],
                on_pointer_over:
                    bounds![crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>],
                on_pointer_up:
                    bounds![crate::imports::frender_events::MaybeHandleEvent<events::PointerEvent>],
                on_transition_cancel:
                    bounds![
                        crate::imports::frender_events::MaybeHandleEvent<events::TransitionEvent>
                    ],
                on_transition_end:
                    bounds![
                        crate::imports::frender_events::MaybeHandleEvent<events::TransitionEvent>
                    ],
                on_transition_run:
                    bounds![
                        crate::imports::frender_events::MaybeHandleEvent<events::TransitionEvent>
                    ],
                on_transition_start:
                    bounds![
                        crate::imports::frender_events::MaybeHandleEvent<events::TransitionEvent>
                    ],
                on_drag: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
                on_drag_end:
                    bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
                on_drag_enter:
                    bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
                on_drag_leave:
                    bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
                on_drag_over:
                    bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
                on_drag_start:
                    bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
                on_drop: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
            ),
            auto_play:
                bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>],
            controls: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>],
            cross_origin:
                bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
            r#loop: alias![loop_]
                + bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>],
            muted: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>],
            preload: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
            src: bounds![crate::imports::frender_html::props::MaybeUpdateValueWithState<str>],
            on_abort: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
            on_can_play: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
            on_can_play_through:
                bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
            on_duration_change:
                bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
            on_emptied: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
            on_ended: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
            on_loaded_data:
                bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
            on_loaded_metadata:
                bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
            on_load_start: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
            on_pause: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
            on_play: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
            on_playing: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
            on_progress: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
            on_rate_change:
                bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
            on_resize: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
            on_seeked: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
            on_seeking: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
            on_stalled: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
            on_suspend: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
            on_time_update:
                bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
            on_volume_change:
                bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
            on_waiting: bounds![crate::imports::frender_events::MaybeHandleEvent<events::Event>],
        ),
    )
);
#[cfg(feature = "csr")]
mod impl_dom_for_props {
    #![allow(unused_variables)]
    #[allow(unused_imports)]
    use super::super::*;
}
#[cfg(feature = "ssr")]
mod impl_ssr_for_props {
    #![allow(unused_variables)]
    #[allow(unused_imports)]
    use super::super::*;
}
mod imports {
    #[allow(unused_imports)]
    use super::super::*;
    pub(super) use crate::imports::frender_html_simple::def_props_type;
}
use imports::def_props_type;
