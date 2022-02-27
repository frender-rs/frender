pub trait IntrinsicComponent {
    const INTRINSIC_TAG: &'static str;
}

crate::macros::def_props_trait! {
    [TElement] HtmlCommonSharedProps [TElement]
    : HtmlCommonSharedPropsBuilder
    {
        children[TNode: react::Node]: Option<TNode> {
            impl |this, v| {
                use react::any_js_props::AnyJsPropsBuilder;
                use react::Node;
                this.as_mut().set_children(v.into_children());
                this
            }
        },
        ref_el[TWriteRef: 'static + react::WriteRef<TElement> + react::SafeIntoJsRuntime]: Option<TWriteRef> { safe_into_js_runtime? },
        default_checked: Option<bool>,
        class@"className": Option<&str>,
        draggable: Option<bool>,
        hidden: Option<bool>,
        id: Option<&str>,
        lang: Option<&str>,
        placeholder: Option<&str>,
        style: Option<crate::css::CssProperties>,
        tab_index: Option<i32>,
        title: Option<&str>,

        // React-specific Attributes
        suppress_content_editable_warning: Option<bool>,
        suppress_hydration_warning: Option<bool>,

        // Standard HTML Attributes
        access_key: Option<&str>,
        content_editable: Option<crate::Inheritable<bool>>,
        context_menu: Option<&str>,
        dir: Option<&str>,
        slot: Option<&str>,
        spell_check: Option<bool>,
        translate: Option<&str>, // TODO: ser: yes | no

        // Unknown
        radio_group: Option<&str>, // <command>, <menuitem>

        // WAI-ARIA
        role: Option<crate::aria::Role>,

        // RDFa Attributes
        about: Option<&str>,
        datatype: Option<&str>,
        inlist: Option<&wasm_bindgen::JsValue>,
        prefix: Option<&str>,
        property: Option<&str>,
        resource: Option<&str>,
        type_of@"typeof": Option<&str>,
        vocab: Option<&str>,

        // Non-standard Attributes
        auto_capitalize: Option<&str>,
        auto_correct: Option<&str>,
        auto_save: Option<&str>,
        color: Option<&str>,
        item_prop: Option<&str>,
        item_scope: Option<bool>,
        item_type: Option<&str>,
        item_id: Option<&str>,
        item_ref: Option<&str>,
        results: Option<i32>,
        security: Option<&str>,
        unselectable: Option<&str>, // TODO: ser: 'on' | 'off' | undefined;

        // Living Standard
        input_mode: Option<crate::HtmlInputMode>,
        is: Option<&str>,

        // events
        // Clipboard Events
        on_copy: react::event::ClipboardEvent<TElement> { event_handler },
        on_copy_capture: react::event::ClipboardEvent<TElement> { event_handler },
        on_cut: react::event::ClipboardEvent<TElement> { event_handler },
        on_cut_capture: react::event::ClipboardEvent<TElement> { event_handler },
        on_paste: react::event::ClipboardEvent<TElement> { event_handler },
        on_paste_capture: react::event::ClipboardEvent<TElement> { event_handler },

        // Composition Events
        on_composition_end: react::event::CompositionEvent<TElement> { event_handler },
        on_composition_end_capture: react::event::CompositionEvent<TElement> { event_handler },
        on_composition_start: react::event::CompositionEvent<TElement> { event_handler },
        on_composition_start_capture: react::event::CompositionEvent<TElement> { event_handler },
        on_composition_update: react::event::CompositionEvent<TElement> { event_handler },
        on_composition_update_capture: react::event::CompositionEvent<TElement> { event_handler },

        // Focus Events
        on_focus: react::event::FocusEvent<TElement> { event_handler },
        on_focus_capture: react::event::FocusEvent<TElement> { event_handler },
        on_blur: react::event::FocusEvent<TElement> { event_handler },
        on_blur_capture: react::event::FocusEvent<TElement> { event_handler },

        // Form Events
        on_change: react::event::FormEvent<TElement> { event_handler },
        on_change_capture: react::event::FormEvent<TElement> { event_handler },
        on_before_input: react::event::FormEvent<TElement> { event_handler },
        on_before_input_capture: react::event::FormEvent<TElement> { event_handler },
        on_input: react::event::FormEvent<TElement> { event_handler },
        on_input_capture: react::event::FormEvent<TElement> { event_handler },
        on_reset: react::event::FormEvent<TElement> { event_handler },
        on_reset_capture: react::event::FormEvent<TElement> { event_handler },
        on_submit: react::event::FormEvent<TElement> { event_handler },
        on_submit_capture: react::event::FormEvent<TElement> { event_handler },
        on_invalid: react::event::FormEvent<TElement> { event_handler },
        on_invalid_capture: react::event::FormEvent<TElement> { event_handler },

        // Image Events
        on_load: react::event::SyntheticEvent<TElement> { event_handler },
        on_load_capture: react::event::SyntheticEvent<TElement> { event_handler },
        on_error: react::event::SyntheticEvent<TElement> { event_handler },
        on_error_capture: react::event::SyntheticEvent<TElement> { event_handler },

        // Keyboard Events
        on_key_down: react::event::KeyboardEvent<TElement> { event_handler },
        on_key_down_capture: react::event::KeyboardEvent<TElement> { event_handler },
        on_key_press: react::event::KeyboardEvent<TElement> { event_handler },
        on_key_press_capture: react::event::KeyboardEvent<TElement> { event_handler },
        on_key_up: react::event::KeyboardEvent<TElement> { event_handler },
        on_key_up_capture: react::event::KeyboardEvent<TElement> { event_handler },

        // Media Events
        on_abort: react::event::SyntheticEvent<TElement> { event_handler },
        on_abort_capture: react::event::SyntheticEvent<TElement> { event_handler },
        on_can_play: react::event::SyntheticEvent<TElement> { event_handler },
        on_can_play_capture: react::event::SyntheticEvent<TElement> { event_handler },
        on_can_play_through: react::event::SyntheticEvent<TElement> { event_handler },
        on_can_play_through_capture: react::event::SyntheticEvent<TElement> { event_handler },
        on_duration_change: react::event::SyntheticEvent<TElement> { event_handler },
        on_duration_change_capture: react::event::SyntheticEvent<TElement> { event_handler },
        on_emptied: react::event::SyntheticEvent<TElement> { event_handler },
        on_emptied_capture: react::event::SyntheticEvent<TElement> { event_handler },
        on_encrypted: react::event::SyntheticEvent<TElement> { event_handler },
        on_encrypted_capture: react::event::SyntheticEvent<TElement> { event_handler },
        on_ended: react::event::SyntheticEvent<TElement> { event_handler },
        on_ended_capture: react::event::SyntheticEvent<TElement> { event_handler },
        on_loaded_data: react::event::SyntheticEvent<TElement> { event_handler },
        on_loaded_data_capture: react::event::SyntheticEvent<TElement> { event_handler },
        on_loaded_metadata: react::event::SyntheticEvent<TElement> { event_handler },
        on_loaded_metadata_capture: react::event::SyntheticEvent<TElement> { event_handler },
        on_load_start: react::event::SyntheticEvent<TElement> { event_handler },
        on_load_start_capture: react::event::SyntheticEvent<TElement> { event_handler },
        on_pause: react::event::SyntheticEvent<TElement> { event_handler },
        on_pause_capture: react::event::SyntheticEvent<TElement> { event_handler },
        on_play: react::event::SyntheticEvent<TElement> { event_handler },
        on_play_capture: react::event::SyntheticEvent<TElement> { event_handler },
        on_playing: react::event::SyntheticEvent<TElement> { event_handler },
        on_playing_capture: react::event::SyntheticEvent<TElement> { event_handler },
        on_progress: react::event::SyntheticEvent<TElement> { event_handler },
        on_progress_capture: react::event::SyntheticEvent<TElement> { event_handler },
        on_rate_change: react::event::SyntheticEvent<TElement> { event_handler },
        on_rate_change_capture: react::event::SyntheticEvent<TElement> { event_handler },
        on_seeked: react::event::SyntheticEvent<TElement> { event_handler },
        on_seeked_capture: react::event::SyntheticEvent<TElement> { event_handler },
        on_seeking: react::event::SyntheticEvent<TElement> { event_handler },
        on_seeking_capture: react::event::SyntheticEvent<TElement> { event_handler },
        on_stalled: react::event::SyntheticEvent<TElement> { event_handler },
        on_stalled_capture: react::event::SyntheticEvent<TElement> { event_handler },
        on_suspend: react::event::SyntheticEvent<TElement> { event_handler },
        on_suspend_capture: react::event::SyntheticEvent<TElement> { event_handler },
        on_time_update: react::event::SyntheticEvent<TElement> { event_handler },
        on_time_update_capture: react::event::SyntheticEvent<TElement> { event_handler },
        on_volume_change: react::event::SyntheticEvent<TElement> { event_handler },
        on_volume_change_capture: react::event::SyntheticEvent<TElement> { event_handler },
        on_waiting: react::event::SyntheticEvent<TElement> { event_handler },
        on_waiting_capture: react::event::SyntheticEvent<TElement> { event_handler },

        // MouseEvents
        on_aux_click: react::event::MouseEvent<TElement> { event_handler },
        on_aux_click_capture: react::event::MouseEvent<TElement> { event_handler },
        on_click: react::event::MouseEvent<TElement> { event_handler },
        on_click_capture: react::event::MouseEvent<TElement> { event_handler },
        on_context_menu: react::event::MouseEvent<TElement> { event_handler },
        on_context_menu_capture: react::event::MouseEvent<TElement> { event_handler },
        on_double_click: react::event::MouseEvent<TElement> { event_handler },
        on_double_click_capture: react::event::MouseEvent<TElement> { event_handler },
        on_drag: react::event::DragEvent<TElement> { event_handler },
        on_drag_capture: react::event::DragEvent<TElement> { event_handler },
        on_drag_end: react::event::DragEvent<TElement> { event_handler },
        on_drag_end_capture: react::event::DragEvent<TElement> { event_handler },
        on_drag_enter: react::event::DragEvent<TElement> { event_handler },
        on_drag_enter_capture: react::event::DragEvent<TElement> { event_handler },
        on_drag_exit: react::event::DragEvent<TElement> { event_handler },
        on_drag_exit_capture: react::event::DragEvent<TElement> { event_handler },
        on_drag_leave: react::event::DragEvent<TElement> { event_handler },
        on_drag_leave_capture: react::event::DragEvent<TElement> { event_handler },
        on_drag_over: react::event::DragEvent<TElement> { event_handler },
        on_drag_over_capture: react::event::DragEvent<TElement> { event_handler },
        on_drag_start: react::event::DragEvent<TElement> { event_handler },
        on_drag_start_capture: react::event::DragEvent<TElement> { event_handler },
        on_drop: react::event::DragEvent<TElement> { event_handler },
        on_drop_capture: react::event::DragEvent<TElement> { event_handler },
        on_mouse_down: react::event::MouseEvent<TElement> { event_handler },
        on_mouse_down_capture: react::event::MouseEvent<TElement> { event_handler },
        on_mouse_enter: react::event::MouseEvent<TElement> { event_handler },
        on_mouse_leave: react::event::MouseEvent<TElement> { event_handler },
        on_mouse_move: react::event::MouseEvent<TElement> { event_handler },
        on_mouse_move_capture: react::event::MouseEvent<TElement> { event_handler },
        on_mouse_out: react::event::MouseEvent<TElement> { event_handler },
        on_mouse_out_capture: react::event::MouseEvent<TElement> { event_handler },
        on_mouse_over: react::event::MouseEvent<TElement> { event_handler },
        on_mouse_over_capture: react::event::MouseEvent<TElement> { event_handler },
        on_mouse_up: react::event::MouseEvent<TElement> { event_handler },
        on_mouse_up_capture: react::event::MouseEvent<TElement> { event_handler },

        // Selection Events
        on_select: react::event::SyntheticEvent<TElement> { event_handler },
        on_select_capture: react::event::SyntheticEvent<TElement> { event_handler },

        // Touch Events
        on_touch_cancel: react::event::TouchEvent<TElement> { event_handler },
        on_touch_cancel_capture: react::event::TouchEvent<TElement> { event_handler },
        on_touch_end: react::event::TouchEvent<TElement> { event_handler },
        on_touch_end_capture: react::event::TouchEvent<TElement> { event_handler },
        on_touch_move: react::event::TouchEvent<TElement> { event_handler },
        on_touch_move_capture: react::event::TouchEvent<TElement> { event_handler },
        on_touch_start: react::event::TouchEvent<TElement> { event_handler },
        on_touch_start_capture: react::event::TouchEvent<TElement> { event_handler },

        // Pointer Events
        on_pointer_down: react::event::PointerEvent<TElement> { event_handler },
        on_pointer_down_capture: react::event::PointerEvent<TElement> { event_handler },
        on_pointer_move: react::event::PointerEvent<TElement> { event_handler },
        on_pointer_move_capture: react::event::PointerEvent<TElement> { event_handler },
        on_pointer_up: react::event::PointerEvent<TElement> { event_handler },
        on_pointer_up_capture: react::event::PointerEvent<TElement> { event_handler },
        on_pointer_cancel: react::event::PointerEvent<TElement> { event_handler },
        on_pointer_cancel_capture: react::event::PointerEvent<TElement> { event_handler },
        on_pointer_enter: react::event::PointerEvent<TElement> { event_handler },
        on_pointer_enter_capture: react::event::PointerEvent<TElement> { event_handler },
        on_pointer_leave: react::event::PointerEvent<TElement> { event_handler },
        on_pointer_leave_capture: react::event::PointerEvent<TElement> { event_handler },
        on_pointer_over: react::event::PointerEvent<TElement> { event_handler },
        on_pointer_over_capture: react::event::PointerEvent<TElement> { event_handler },
        on_pointer_out: react::event::PointerEvent<TElement> { event_handler },
        on_pointer_out_capture: react::event::PointerEvent<TElement> { event_handler },
        on_got_pointer_capture: react::event::PointerEvent<TElement> { event_handler },
        on_got_pointer_capture_capture: react::event::PointerEvent<TElement> { event_handler },
        on_lost_pointer_capture: react::event::PointerEvent<TElement> { event_handler },
        on_lost_pointer_capture_capture: react::event::PointerEvent<TElement> { event_handler },

        // UI Events
        on_scroll: react::event::UiEvent<TElement> { event_handler },
        on_scroll_capture: react::event::UiEvent<TElement> { event_handler },

        // Wheel Events
        on_wheel: react::event::WheelEvent<TElement> { event_handler },
        on_wheel_capture: react::event::WheelEvent<TElement> { event_handler },

        // Animation Events
        on_animation_start: react::event::AnimationEvent<TElement> { event_handler },
        on_animation_start_capture: react::event::AnimationEvent<TElement> { event_handler },
        on_animation_end: react::event::AnimationEvent<TElement> { event_handler },
        on_animation_end_capture: react::event::AnimationEvent<TElement> { event_handler },
        on_animation_iteration: react::event::AnimationEvent<TElement> { event_handler },
        on_animation_iteration_capture: react::event::AnimationEvent<TElement> { event_handler },

        // Transition Events
        on_transition_end: react::event::TransitionEvent<TElement> { event_handler },
        on_transition_end_capture: react::event::TransitionEvent<TElement> { event_handler },
    }
}
