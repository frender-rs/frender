#[allow(non_snake_case)]
#[inline(always)]
pub fn HtmlMediaElementProps() -> Building<TypesInitial> {
    #[allow(unused_imports)]
    use super::*;
    self::Building {
        HtmlElementProps: HtmlElementProps::build(HtmlElementProps()),
        auto_play: (),
        controls: (),
        cross_origin: (),
        r#loop: (),
        muted: (),
        preload: (),
        src: (),
        on_abort: (),
        on_can_play: (),
        on_can_play_through: (),
        on_duration_change: (),
        on_emptied: (),
        on_ended: (),
        on_loaded_data: (),
        on_loaded_metadata: (),
        on_load_start: (),
        on_pause: (),
        on_play: (),
        on_playing: (),
        on_progress: (),
        on_rate_change: (),
        on_resize: (),
        on_seeked: (),
        on_seeking: (),
        on_stalled: (),
        on_suspend: (),
        on_time_update: (),
        on_volume_change: (),
        on_waiting: (),
    }
}
pub mod prelude {}
pub mod overwrite {
    #![allow(non_camel_case_types)]
    pub type HtmlElementProps<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = Value,
        auto_play = <TypeDefs as super::Types>::auto_play,
        controls = <TypeDefs as super::Types>::controls,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        r#loop = <TypeDefs as super::Types>::r#loop,
        muted = <TypeDefs as super::Types>::muted,
        preload = <TypeDefs as super::Types>::preload,
        src = <TypeDefs as super::Types>::src,
        on_abort = <TypeDefs as super::Types>::on_abort,
        on_can_play = <TypeDefs as super::Types>::on_can_play,
        on_can_play_through = <TypeDefs as super::Types>::on_can_play_through,
        on_duration_change = <TypeDefs as super::Types>::on_duration_change,
        on_emptied = <TypeDefs as super::Types>::on_emptied,
        on_ended = <TypeDefs as super::Types>::on_ended,
        on_loaded_data = <TypeDefs as super::Types>::on_loaded_data,
        on_loaded_metadata = <TypeDefs as super::Types>::on_loaded_metadata,
        on_load_start = <TypeDefs as super::Types>::on_load_start,
        on_pause = <TypeDefs as super::Types>::on_pause,
        on_play = <TypeDefs as super::Types>::on_play,
        on_playing = <TypeDefs as super::Types>::on_playing,
        on_progress = <TypeDefs as super::Types>::on_progress,
        on_rate_change = <TypeDefs as super::Types>::on_rate_change,
        on_resize = <TypeDefs as super::Types>::on_resize,
        on_seeked = <TypeDefs as super::Types>::on_seeked,
        on_seeking = <TypeDefs as super::Types>::on_seeking,
        on_stalled = <TypeDefs as super::Types>::on_stalled,
        on_suspend = <TypeDefs as super::Types>::on_suspend,
        on_time_update = <TypeDefs as super::Types>::on_time_update,
        on_volume_change = <TypeDefs as super::Types>::on_volume_change,
        on_waiting = <TypeDefs as super::Types>::on_waiting,
    >;
    pub type ElementProps<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::ElementProps<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type children<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::children<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type class<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::class<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type id<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::id<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type part<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::part<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_cancel<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_cancel<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_error<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_error<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_scroll<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_scroll<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_security_policy_violation<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_security_policy_violation<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_select<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_select<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_wheel<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_wheel<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_copy<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_copy<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_cut<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_cut<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_paste<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_paste<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_composition_end<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_composition_end<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_composition_start<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_composition_start<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_composition_update<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_composition_update<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_blur<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_blur<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_focus<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_focus<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_focus_in<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_focus_in<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_focus_out<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_focus_out<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_fullscreen_change<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_fullscreen_change<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_fullscreen_error<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_fullscreen_error<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_key_down<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_key_down<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_key_up<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_key_up<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_aux_click<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_aux_click<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_click<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_click<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_context_menu<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_context_menu<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_double_click<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_double_click<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_mouse_down<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_mouse_down<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_mouse_enter<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_mouse_enter<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_mouse_leave<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_mouse_leave<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_mouse_move<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_mouse_move<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_mouse_out<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_mouse_out<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_mouse_over<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_mouse_over<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_mouse_up<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_mouse_up<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_touch_cancel<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_touch_cancel<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_touch_end<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_touch_end<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_touch_move<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_touch_move<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_touch_start<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_touch_start<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type access_key<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::access_key<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type auto_capitalize<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::auto_capitalize<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type auto_focus<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::auto_focus<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type context_menu<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::context_menu<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type dir<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::dir<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type draggable<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::draggable<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type enter_key_hint<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::enter_key_hint<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type hidden<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::hidden<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type inert<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::inert<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type input_mode<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::input_mode<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type is<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::is<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type item_id<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::item_id<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type item_prop<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::item_prop<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type item_ref<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::item_ref<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type item_scope<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::item_scope<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type item_type<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::item_type<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type lang<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::lang<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type nonce<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::nonce<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type role<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::role<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type slot<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::slot<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type spellcheck<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::spellcheck<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type style<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::style<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type tab_index<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::tab_index<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type title<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::title<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type translate<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::translate<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type virtual_keyboard_policy<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::virtual_keyboard_policy<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_invalid<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_invalid<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_animation_cancel<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_animation_cancel<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_animation_end<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_animation_end<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_animation_iteration<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_animation_iteration<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_animation_start<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_animation_start<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_before_input<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_before_input<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_input<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_input<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_change<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_change<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_got_pointer_capture<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_got_pointer_capture<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_lost_pointer_capture<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_lost_pointer_capture<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_pointer_cancel<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_pointer_cancel<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_pointer_down<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_pointer_down<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_pointer_enter<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_pointer_enter<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_pointer_leave<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_pointer_leave<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_pointer_move<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_pointer_move<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_pointer_out<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_pointer_out<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_pointer_over<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_pointer_over<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_pointer_up<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_pointer_up<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_transition_cancel<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_transition_cancel<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_transition_end<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_transition_end<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_transition_run<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_transition_run<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_transition_start<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_transition_start<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_drag<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_drag<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_drag_end<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_drag_end<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_drag_enter<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_drag_enter<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_drag_leave<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_drag_leave<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_drag_over<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_drag_over<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_drag_start<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_drag_start<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_drop<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_drop<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type auto_play<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_play = Value,
        controls = <TypeDefs as super::Types>::controls,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        r#loop = <TypeDefs as super::Types>::r#loop,
        muted = <TypeDefs as super::Types>::muted,
        preload = <TypeDefs as super::Types>::preload,
        src = <TypeDefs as super::Types>::src,
        on_abort = <TypeDefs as super::Types>::on_abort,
        on_can_play = <TypeDefs as super::Types>::on_can_play,
        on_can_play_through = <TypeDefs as super::Types>::on_can_play_through,
        on_duration_change = <TypeDefs as super::Types>::on_duration_change,
        on_emptied = <TypeDefs as super::Types>::on_emptied,
        on_ended = <TypeDefs as super::Types>::on_ended,
        on_loaded_data = <TypeDefs as super::Types>::on_loaded_data,
        on_loaded_metadata = <TypeDefs as super::Types>::on_loaded_metadata,
        on_load_start = <TypeDefs as super::Types>::on_load_start,
        on_pause = <TypeDefs as super::Types>::on_pause,
        on_play = <TypeDefs as super::Types>::on_play,
        on_playing = <TypeDefs as super::Types>::on_playing,
        on_progress = <TypeDefs as super::Types>::on_progress,
        on_rate_change = <TypeDefs as super::Types>::on_rate_change,
        on_resize = <TypeDefs as super::Types>::on_resize,
        on_seeked = <TypeDefs as super::Types>::on_seeked,
        on_seeking = <TypeDefs as super::Types>::on_seeking,
        on_stalled = <TypeDefs as super::Types>::on_stalled,
        on_suspend = <TypeDefs as super::Types>::on_suspend,
        on_time_update = <TypeDefs as super::Types>::on_time_update,
        on_volume_change = <TypeDefs as super::Types>::on_volume_change,
        on_waiting = <TypeDefs as super::Types>::on_waiting,
    >;
    pub type controls<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_play = <TypeDefs as super::Types>::auto_play,
        controls = Value,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        r#loop = <TypeDefs as super::Types>::r#loop,
        muted = <TypeDefs as super::Types>::muted,
        preload = <TypeDefs as super::Types>::preload,
        src = <TypeDefs as super::Types>::src,
        on_abort = <TypeDefs as super::Types>::on_abort,
        on_can_play = <TypeDefs as super::Types>::on_can_play,
        on_can_play_through = <TypeDefs as super::Types>::on_can_play_through,
        on_duration_change = <TypeDefs as super::Types>::on_duration_change,
        on_emptied = <TypeDefs as super::Types>::on_emptied,
        on_ended = <TypeDefs as super::Types>::on_ended,
        on_loaded_data = <TypeDefs as super::Types>::on_loaded_data,
        on_loaded_metadata = <TypeDefs as super::Types>::on_loaded_metadata,
        on_load_start = <TypeDefs as super::Types>::on_load_start,
        on_pause = <TypeDefs as super::Types>::on_pause,
        on_play = <TypeDefs as super::Types>::on_play,
        on_playing = <TypeDefs as super::Types>::on_playing,
        on_progress = <TypeDefs as super::Types>::on_progress,
        on_rate_change = <TypeDefs as super::Types>::on_rate_change,
        on_resize = <TypeDefs as super::Types>::on_resize,
        on_seeked = <TypeDefs as super::Types>::on_seeked,
        on_seeking = <TypeDefs as super::Types>::on_seeking,
        on_stalled = <TypeDefs as super::Types>::on_stalled,
        on_suspend = <TypeDefs as super::Types>::on_suspend,
        on_time_update = <TypeDefs as super::Types>::on_time_update,
        on_volume_change = <TypeDefs as super::Types>::on_volume_change,
        on_waiting = <TypeDefs as super::Types>::on_waiting,
    >;
    pub type cross_origin<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_play = <TypeDefs as super::Types>::auto_play,
        controls = <TypeDefs as super::Types>::controls,
        cross_origin = Value,
        r#loop = <TypeDefs as super::Types>::r#loop,
        muted = <TypeDefs as super::Types>::muted,
        preload = <TypeDefs as super::Types>::preload,
        src = <TypeDefs as super::Types>::src,
        on_abort = <TypeDefs as super::Types>::on_abort,
        on_can_play = <TypeDefs as super::Types>::on_can_play,
        on_can_play_through = <TypeDefs as super::Types>::on_can_play_through,
        on_duration_change = <TypeDefs as super::Types>::on_duration_change,
        on_emptied = <TypeDefs as super::Types>::on_emptied,
        on_ended = <TypeDefs as super::Types>::on_ended,
        on_loaded_data = <TypeDefs as super::Types>::on_loaded_data,
        on_loaded_metadata = <TypeDefs as super::Types>::on_loaded_metadata,
        on_load_start = <TypeDefs as super::Types>::on_load_start,
        on_pause = <TypeDefs as super::Types>::on_pause,
        on_play = <TypeDefs as super::Types>::on_play,
        on_playing = <TypeDefs as super::Types>::on_playing,
        on_progress = <TypeDefs as super::Types>::on_progress,
        on_rate_change = <TypeDefs as super::Types>::on_rate_change,
        on_resize = <TypeDefs as super::Types>::on_resize,
        on_seeked = <TypeDefs as super::Types>::on_seeked,
        on_seeking = <TypeDefs as super::Types>::on_seeking,
        on_stalled = <TypeDefs as super::Types>::on_stalled,
        on_suspend = <TypeDefs as super::Types>::on_suspend,
        on_time_update = <TypeDefs as super::Types>::on_time_update,
        on_volume_change = <TypeDefs as super::Types>::on_volume_change,
        on_waiting = <TypeDefs as super::Types>::on_waiting,
    >;
    pub type r#loop<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_play = <TypeDefs as super::Types>::auto_play,
        controls = <TypeDefs as super::Types>::controls,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        r#loop = Value,
        muted = <TypeDefs as super::Types>::muted,
        preload = <TypeDefs as super::Types>::preload,
        src = <TypeDefs as super::Types>::src,
        on_abort = <TypeDefs as super::Types>::on_abort,
        on_can_play = <TypeDefs as super::Types>::on_can_play,
        on_can_play_through = <TypeDefs as super::Types>::on_can_play_through,
        on_duration_change = <TypeDefs as super::Types>::on_duration_change,
        on_emptied = <TypeDefs as super::Types>::on_emptied,
        on_ended = <TypeDefs as super::Types>::on_ended,
        on_loaded_data = <TypeDefs as super::Types>::on_loaded_data,
        on_loaded_metadata = <TypeDefs as super::Types>::on_loaded_metadata,
        on_load_start = <TypeDefs as super::Types>::on_load_start,
        on_pause = <TypeDefs as super::Types>::on_pause,
        on_play = <TypeDefs as super::Types>::on_play,
        on_playing = <TypeDefs as super::Types>::on_playing,
        on_progress = <TypeDefs as super::Types>::on_progress,
        on_rate_change = <TypeDefs as super::Types>::on_rate_change,
        on_resize = <TypeDefs as super::Types>::on_resize,
        on_seeked = <TypeDefs as super::Types>::on_seeked,
        on_seeking = <TypeDefs as super::Types>::on_seeking,
        on_stalled = <TypeDefs as super::Types>::on_stalled,
        on_suspend = <TypeDefs as super::Types>::on_suspend,
        on_time_update = <TypeDefs as super::Types>::on_time_update,
        on_volume_change = <TypeDefs as super::Types>::on_volume_change,
        on_waiting = <TypeDefs as super::Types>::on_waiting,
    >;
    pub type muted<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_play = <TypeDefs as super::Types>::auto_play,
        controls = <TypeDefs as super::Types>::controls,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        r#loop = <TypeDefs as super::Types>::r#loop,
        muted = Value,
        preload = <TypeDefs as super::Types>::preload,
        src = <TypeDefs as super::Types>::src,
        on_abort = <TypeDefs as super::Types>::on_abort,
        on_can_play = <TypeDefs as super::Types>::on_can_play,
        on_can_play_through = <TypeDefs as super::Types>::on_can_play_through,
        on_duration_change = <TypeDefs as super::Types>::on_duration_change,
        on_emptied = <TypeDefs as super::Types>::on_emptied,
        on_ended = <TypeDefs as super::Types>::on_ended,
        on_loaded_data = <TypeDefs as super::Types>::on_loaded_data,
        on_loaded_metadata = <TypeDefs as super::Types>::on_loaded_metadata,
        on_load_start = <TypeDefs as super::Types>::on_load_start,
        on_pause = <TypeDefs as super::Types>::on_pause,
        on_play = <TypeDefs as super::Types>::on_play,
        on_playing = <TypeDefs as super::Types>::on_playing,
        on_progress = <TypeDefs as super::Types>::on_progress,
        on_rate_change = <TypeDefs as super::Types>::on_rate_change,
        on_resize = <TypeDefs as super::Types>::on_resize,
        on_seeked = <TypeDefs as super::Types>::on_seeked,
        on_seeking = <TypeDefs as super::Types>::on_seeking,
        on_stalled = <TypeDefs as super::Types>::on_stalled,
        on_suspend = <TypeDefs as super::Types>::on_suspend,
        on_time_update = <TypeDefs as super::Types>::on_time_update,
        on_volume_change = <TypeDefs as super::Types>::on_volume_change,
        on_waiting = <TypeDefs as super::Types>::on_waiting,
    >;
    pub type preload<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_play = <TypeDefs as super::Types>::auto_play,
        controls = <TypeDefs as super::Types>::controls,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        r#loop = <TypeDefs as super::Types>::r#loop,
        muted = <TypeDefs as super::Types>::muted,
        preload = Value,
        src = <TypeDefs as super::Types>::src,
        on_abort = <TypeDefs as super::Types>::on_abort,
        on_can_play = <TypeDefs as super::Types>::on_can_play,
        on_can_play_through = <TypeDefs as super::Types>::on_can_play_through,
        on_duration_change = <TypeDefs as super::Types>::on_duration_change,
        on_emptied = <TypeDefs as super::Types>::on_emptied,
        on_ended = <TypeDefs as super::Types>::on_ended,
        on_loaded_data = <TypeDefs as super::Types>::on_loaded_data,
        on_loaded_metadata = <TypeDefs as super::Types>::on_loaded_metadata,
        on_load_start = <TypeDefs as super::Types>::on_load_start,
        on_pause = <TypeDefs as super::Types>::on_pause,
        on_play = <TypeDefs as super::Types>::on_play,
        on_playing = <TypeDefs as super::Types>::on_playing,
        on_progress = <TypeDefs as super::Types>::on_progress,
        on_rate_change = <TypeDefs as super::Types>::on_rate_change,
        on_resize = <TypeDefs as super::Types>::on_resize,
        on_seeked = <TypeDefs as super::Types>::on_seeked,
        on_seeking = <TypeDefs as super::Types>::on_seeking,
        on_stalled = <TypeDefs as super::Types>::on_stalled,
        on_suspend = <TypeDefs as super::Types>::on_suspend,
        on_time_update = <TypeDefs as super::Types>::on_time_update,
        on_volume_change = <TypeDefs as super::Types>::on_volume_change,
        on_waiting = <TypeDefs as super::Types>::on_waiting,
    >;
    pub type src<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_play = <TypeDefs as super::Types>::auto_play,
        controls = <TypeDefs as super::Types>::controls,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        r#loop = <TypeDefs as super::Types>::r#loop,
        muted = <TypeDefs as super::Types>::muted,
        preload = <TypeDefs as super::Types>::preload,
        src = Value,
        on_abort = <TypeDefs as super::Types>::on_abort,
        on_can_play = <TypeDefs as super::Types>::on_can_play,
        on_can_play_through = <TypeDefs as super::Types>::on_can_play_through,
        on_duration_change = <TypeDefs as super::Types>::on_duration_change,
        on_emptied = <TypeDefs as super::Types>::on_emptied,
        on_ended = <TypeDefs as super::Types>::on_ended,
        on_loaded_data = <TypeDefs as super::Types>::on_loaded_data,
        on_loaded_metadata = <TypeDefs as super::Types>::on_loaded_metadata,
        on_load_start = <TypeDefs as super::Types>::on_load_start,
        on_pause = <TypeDefs as super::Types>::on_pause,
        on_play = <TypeDefs as super::Types>::on_play,
        on_playing = <TypeDefs as super::Types>::on_playing,
        on_progress = <TypeDefs as super::Types>::on_progress,
        on_rate_change = <TypeDefs as super::Types>::on_rate_change,
        on_resize = <TypeDefs as super::Types>::on_resize,
        on_seeked = <TypeDefs as super::Types>::on_seeked,
        on_seeking = <TypeDefs as super::Types>::on_seeking,
        on_stalled = <TypeDefs as super::Types>::on_stalled,
        on_suspend = <TypeDefs as super::Types>::on_suspend,
        on_time_update = <TypeDefs as super::Types>::on_time_update,
        on_volume_change = <TypeDefs as super::Types>::on_volume_change,
        on_waiting = <TypeDefs as super::Types>::on_waiting,
    >;
    pub type on_abort<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_play = <TypeDefs as super::Types>::auto_play,
        controls = <TypeDefs as super::Types>::controls,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        r#loop = <TypeDefs as super::Types>::r#loop,
        muted = <TypeDefs as super::Types>::muted,
        preload = <TypeDefs as super::Types>::preload,
        src = <TypeDefs as super::Types>::src,
        on_abort = Value,
        on_can_play = <TypeDefs as super::Types>::on_can_play,
        on_can_play_through = <TypeDefs as super::Types>::on_can_play_through,
        on_duration_change = <TypeDefs as super::Types>::on_duration_change,
        on_emptied = <TypeDefs as super::Types>::on_emptied,
        on_ended = <TypeDefs as super::Types>::on_ended,
        on_loaded_data = <TypeDefs as super::Types>::on_loaded_data,
        on_loaded_metadata = <TypeDefs as super::Types>::on_loaded_metadata,
        on_load_start = <TypeDefs as super::Types>::on_load_start,
        on_pause = <TypeDefs as super::Types>::on_pause,
        on_play = <TypeDefs as super::Types>::on_play,
        on_playing = <TypeDefs as super::Types>::on_playing,
        on_progress = <TypeDefs as super::Types>::on_progress,
        on_rate_change = <TypeDefs as super::Types>::on_rate_change,
        on_resize = <TypeDefs as super::Types>::on_resize,
        on_seeked = <TypeDefs as super::Types>::on_seeked,
        on_seeking = <TypeDefs as super::Types>::on_seeking,
        on_stalled = <TypeDefs as super::Types>::on_stalled,
        on_suspend = <TypeDefs as super::Types>::on_suspend,
        on_time_update = <TypeDefs as super::Types>::on_time_update,
        on_volume_change = <TypeDefs as super::Types>::on_volume_change,
        on_waiting = <TypeDefs as super::Types>::on_waiting,
    >;
    pub type on_can_play<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_play = <TypeDefs as super::Types>::auto_play,
        controls = <TypeDefs as super::Types>::controls,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        r#loop = <TypeDefs as super::Types>::r#loop,
        muted = <TypeDefs as super::Types>::muted,
        preload = <TypeDefs as super::Types>::preload,
        src = <TypeDefs as super::Types>::src,
        on_abort = <TypeDefs as super::Types>::on_abort,
        on_can_play = Value,
        on_can_play_through = <TypeDefs as super::Types>::on_can_play_through,
        on_duration_change = <TypeDefs as super::Types>::on_duration_change,
        on_emptied = <TypeDefs as super::Types>::on_emptied,
        on_ended = <TypeDefs as super::Types>::on_ended,
        on_loaded_data = <TypeDefs as super::Types>::on_loaded_data,
        on_loaded_metadata = <TypeDefs as super::Types>::on_loaded_metadata,
        on_load_start = <TypeDefs as super::Types>::on_load_start,
        on_pause = <TypeDefs as super::Types>::on_pause,
        on_play = <TypeDefs as super::Types>::on_play,
        on_playing = <TypeDefs as super::Types>::on_playing,
        on_progress = <TypeDefs as super::Types>::on_progress,
        on_rate_change = <TypeDefs as super::Types>::on_rate_change,
        on_resize = <TypeDefs as super::Types>::on_resize,
        on_seeked = <TypeDefs as super::Types>::on_seeked,
        on_seeking = <TypeDefs as super::Types>::on_seeking,
        on_stalled = <TypeDefs as super::Types>::on_stalled,
        on_suspend = <TypeDefs as super::Types>::on_suspend,
        on_time_update = <TypeDefs as super::Types>::on_time_update,
        on_volume_change = <TypeDefs as super::Types>::on_volume_change,
        on_waiting = <TypeDefs as super::Types>::on_waiting,
    >;
    pub type on_can_play_through<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_play = <TypeDefs as super::Types>::auto_play,
        controls = <TypeDefs as super::Types>::controls,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        r#loop = <TypeDefs as super::Types>::r#loop,
        muted = <TypeDefs as super::Types>::muted,
        preload = <TypeDefs as super::Types>::preload,
        src = <TypeDefs as super::Types>::src,
        on_abort = <TypeDefs as super::Types>::on_abort,
        on_can_play = <TypeDefs as super::Types>::on_can_play,
        on_can_play_through = Value,
        on_duration_change = <TypeDefs as super::Types>::on_duration_change,
        on_emptied = <TypeDefs as super::Types>::on_emptied,
        on_ended = <TypeDefs as super::Types>::on_ended,
        on_loaded_data = <TypeDefs as super::Types>::on_loaded_data,
        on_loaded_metadata = <TypeDefs as super::Types>::on_loaded_metadata,
        on_load_start = <TypeDefs as super::Types>::on_load_start,
        on_pause = <TypeDefs as super::Types>::on_pause,
        on_play = <TypeDefs as super::Types>::on_play,
        on_playing = <TypeDefs as super::Types>::on_playing,
        on_progress = <TypeDefs as super::Types>::on_progress,
        on_rate_change = <TypeDefs as super::Types>::on_rate_change,
        on_resize = <TypeDefs as super::Types>::on_resize,
        on_seeked = <TypeDefs as super::Types>::on_seeked,
        on_seeking = <TypeDefs as super::Types>::on_seeking,
        on_stalled = <TypeDefs as super::Types>::on_stalled,
        on_suspend = <TypeDefs as super::Types>::on_suspend,
        on_time_update = <TypeDefs as super::Types>::on_time_update,
        on_volume_change = <TypeDefs as super::Types>::on_volume_change,
        on_waiting = <TypeDefs as super::Types>::on_waiting,
    >;
    pub type on_duration_change<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_play = <TypeDefs as super::Types>::auto_play,
        controls = <TypeDefs as super::Types>::controls,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        r#loop = <TypeDefs as super::Types>::r#loop,
        muted = <TypeDefs as super::Types>::muted,
        preload = <TypeDefs as super::Types>::preload,
        src = <TypeDefs as super::Types>::src,
        on_abort = <TypeDefs as super::Types>::on_abort,
        on_can_play = <TypeDefs as super::Types>::on_can_play,
        on_can_play_through = <TypeDefs as super::Types>::on_can_play_through,
        on_duration_change = Value,
        on_emptied = <TypeDefs as super::Types>::on_emptied,
        on_ended = <TypeDefs as super::Types>::on_ended,
        on_loaded_data = <TypeDefs as super::Types>::on_loaded_data,
        on_loaded_metadata = <TypeDefs as super::Types>::on_loaded_metadata,
        on_load_start = <TypeDefs as super::Types>::on_load_start,
        on_pause = <TypeDefs as super::Types>::on_pause,
        on_play = <TypeDefs as super::Types>::on_play,
        on_playing = <TypeDefs as super::Types>::on_playing,
        on_progress = <TypeDefs as super::Types>::on_progress,
        on_rate_change = <TypeDefs as super::Types>::on_rate_change,
        on_resize = <TypeDefs as super::Types>::on_resize,
        on_seeked = <TypeDefs as super::Types>::on_seeked,
        on_seeking = <TypeDefs as super::Types>::on_seeking,
        on_stalled = <TypeDefs as super::Types>::on_stalled,
        on_suspend = <TypeDefs as super::Types>::on_suspend,
        on_time_update = <TypeDefs as super::Types>::on_time_update,
        on_volume_change = <TypeDefs as super::Types>::on_volume_change,
        on_waiting = <TypeDefs as super::Types>::on_waiting,
    >;
    pub type on_emptied<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_play = <TypeDefs as super::Types>::auto_play,
        controls = <TypeDefs as super::Types>::controls,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        r#loop = <TypeDefs as super::Types>::r#loop,
        muted = <TypeDefs as super::Types>::muted,
        preload = <TypeDefs as super::Types>::preload,
        src = <TypeDefs as super::Types>::src,
        on_abort = <TypeDefs as super::Types>::on_abort,
        on_can_play = <TypeDefs as super::Types>::on_can_play,
        on_can_play_through = <TypeDefs as super::Types>::on_can_play_through,
        on_duration_change = <TypeDefs as super::Types>::on_duration_change,
        on_emptied = Value,
        on_ended = <TypeDefs as super::Types>::on_ended,
        on_loaded_data = <TypeDefs as super::Types>::on_loaded_data,
        on_loaded_metadata = <TypeDefs as super::Types>::on_loaded_metadata,
        on_load_start = <TypeDefs as super::Types>::on_load_start,
        on_pause = <TypeDefs as super::Types>::on_pause,
        on_play = <TypeDefs as super::Types>::on_play,
        on_playing = <TypeDefs as super::Types>::on_playing,
        on_progress = <TypeDefs as super::Types>::on_progress,
        on_rate_change = <TypeDefs as super::Types>::on_rate_change,
        on_resize = <TypeDefs as super::Types>::on_resize,
        on_seeked = <TypeDefs as super::Types>::on_seeked,
        on_seeking = <TypeDefs as super::Types>::on_seeking,
        on_stalled = <TypeDefs as super::Types>::on_stalled,
        on_suspend = <TypeDefs as super::Types>::on_suspend,
        on_time_update = <TypeDefs as super::Types>::on_time_update,
        on_volume_change = <TypeDefs as super::Types>::on_volume_change,
        on_waiting = <TypeDefs as super::Types>::on_waiting,
    >;
    pub type on_ended<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_play = <TypeDefs as super::Types>::auto_play,
        controls = <TypeDefs as super::Types>::controls,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        r#loop = <TypeDefs as super::Types>::r#loop,
        muted = <TypeDefs as super::Types>::muted,
        preload = <TypeDefs as super::Types>::preload,
        src = <TypeDefs as super::Types>::src,
        on_abort = <TypeDefs as super::Types>::on_abort,
        on_can_play = <TypeDefs as super::Types>::on_can_play,
        on_can_play_through = <TypeDefs as super::Types>::on_can_play_through,
        on_duration_change = <TypeDefs as super::Types>::on_duration_change,
        on_emptied = <TypeDefs as super::Types>::on_emptied,
        on_ended = Value,
        on_loaded_data = <TypeDefs as super::Types>::on_loaded_data,
        on_loaded_metadata = <TypeDefs as super::Types>::on_loaded_metadata,
        on_load_start = <TypeDefs as super::Types>::on_load_start,
        on_pause = <TypeDefs as super::Types>::on_pause,
        on_play = <TypeDefs as super::Types>::on_play,
        on_playing = <TypeDefs as super::Types>::on_playing,
        on_progress = <TypeDefs as super::Types>::on_progress,
        on_rate_change = <TypeDefs as super::Types>::on_rate_change,
        on_resize = <TypeDefs as super::Types>::on_resize,
        on_seeked = <TypeDefs as super::Types>::on_seeked,
        on_seeking = <TypeDefs as super::Types>::on_seeking,
        on_stalled = <TypeDefs as super::Types>::on_stalled,
        on_suspend = <TypeDefs as super::Types>::on_suspend,
        on_time_update = <TypeDefs as super::Types>::on_time_update,
        on_volume_change = <TypeDefs as super::Types>::on_volume_change,
        on_waiting = <TypeDefs as super::Types>::on_waiting,
    >;
    pub type on_loaded_data<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_play = <TypeDefs as super::Types>::auto_play,
        controls = <TypeDefs as super::Types>::controls,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        r#loop = <TypeDefs as super::Types>::r#loop,
        muted = <TypeDefs as super::Types>::muted,
        preload = <TypeDefs as super::Types>::preload,
        src = <TypeDefs as super::Types>::src,
        on_abort = <TypeDefs as super::Types>::on_abort,
        on_can_play = <TypeDefs as super::Types>::on_can_play,
        on_can_play_through = <TypeDefs as super::Types>::on_can_play_through,
        on_duration_change = <TypeDefs as super::Types>::on_duration_change,
        on_emptied = <TypeDefs as super::Types>::on_emptied,
        on_ended = <TypeDefs as super::Types>::on_ended,
        on_loaded_data = Value,
        on_loaded_metadata = <TypeDefs as super::Types>::on_loaded_metadata,
        on_load_start = <TypeDefs as super::Types>::on_load_start,
        on_pause = <TypeDefs as super::Types>::on_pause,
        on_play = <TypeDefs as super::Types>::on_play,
        on_playing = <TypeDefs as super::Types>::on_playing,
        on_progress = <TypeDefs as super::Types>::on_progress,
        on_rate_change = <TypeDefs as super::Types>::on_rate_change,
        on_resize = <TypeDefs as super::Types>::on_resize,
        on_seeked = <TypeDefs as super::Types>::on_seeked,
        on_seeking = <TypeDefs as super::Types>::on_seeking,
        on_stalled = <TypeDefs as super::Types>::on_stalled,
        on_suspend = <TypeDefs as super::Types>::on_suspend,
        on_time_update = <TypeDefs as super::Types>::on_time_update,
        on_volume_change = <TypeDefs as super::Types>::on_volume_change,
        on_waiting = <TypeDefs as super::Types>::on_waiting,
    >;
    pub type on_loaded_metadata<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_play = <TypeDefs as super::Types>::auto_play,
        controls = <TypeDefs as super::Types>::controls,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        r#loop = <TypeDefs as super::Types>::r#loop,
        muted = <TypeDefs as super::Types>::muted,
        preload = <TypeDefs as super::Types>::preload,
        src = <TypeDefs as super::Types>::src,
        on_abort = <TypeDefs as super::Types>::on_abort,
        on_can_play = <TypeDefs as super::Types>::on_can_play,
        on_can_play_through = <TypeDefs as super::Types>::on_can_play_through,
        on_duration_change = <TypeDefs as super::Types>::on_duration_change,
        on_emptied = <TypeDefs as super::Types>::on_emptied,
        on_ended = <TypeDefs as super::Types>::on_ended,
        on_loaded_data = <TypeDefs as super::Types>::on_loaded_data,
        on_loaded_metadata = Value,
        on_load_start = <TypeDefs as super::Types>::on_load_start,
        on_pause = <TypeDefs as super::Types>::on_pause,
        on_play = <TypeDefs as super::Types>::on_play,
        on_playing = <TypeDefs as super::Types>::on_playing,
        on_progress = <TypeDefs as super::Types>::on_progress,
        on_rate_change = <TypeDefs as super::Types>::on_rate_change,
        on_resize = <TypeDefs as super::Types>::on_resize,
        on_seeked = <TypeDefs as super::Types>::on_seeked,
        on_seeking = <TypeDefs as super::Types>::on_seeking,
        on_stalled = <TypeDefs as super::Types>::on_stalled,
        on_suspend = <TypeDefs as super::Types>::on_suspend,
        on_time_update = <TypeDefs as super::Types>::on_time_update,
        on_volume_change = <TypeDefs as super::Types>::on_volume_change,
        on_waiting = <TypeDefs as super::Types>::on_waiting,
    >;
    pub type on_load_start<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_play = <TypeDefs as super::Types>::auto_play,
        controls = <TypeDefs as super::Types>::controls,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        r#loop = <TypeDefs as super::Types>::r#loop,
        muted = <TypeDefs as super::Types>::muted,
        preload = <TypeDefs as super::Types>::preload,
        src = <TypeDefs as super::Types>::src,
        on_abort = <TypeDefs as super::Types>::on_abort,
        on_can_play = <TypeDefs as super::Types>::on_can_play,
        on_can_play_through = <TypeDefs as super::Types>::on_can_play_through,
        on_duration_change = <TypeDefs as super::Types>::on_duration_change,
        on_emptied = <TypeDefs as super::Types>::on_emptied,
        on_ended = <TypeDefs as super::Types>::on_ended,
        on_loaded_data = <TypeDefs as super::Types>::on_loaded_data,
        on_loaded_metadata = <TypeDefs as super::Types>::on_loaded_metadata,
        on_load_start = Value,
        on_pause = <TypeDefs as super::Types>::on_pause,
        on_play = <TypeDefs as super::Types>::on_play,
        on_playing = <TypeDefs as super::Types>::on_playing,
        on_progress = <TypeDefs as super::Types>::on_progress,
        on_rate_change = <TypeDefs as super::Types>::on_rate_change,
        on_resize = <TypeDefs as super::Types>::on_resize,
        on_seeked = <TypeDefs as super::Types>::on_seeked,
        on_seeking = <TypeDefs as super::Types>::on_seeking,
        on_stalled = <TypeDefs as super::Types>::on_stalled,
        on_suspend = <TypeDefs as super::Types>::on_suspend,
        on_time_update = <TypeDefs as super::Types>::on_time_update,
        on_volume_change = <TypeDefs as super::Types>::on_volume_change,
        on_waiting = <TypeDefs as super::Types>::on_waiting,
    >;
    pub type on_pause<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_play = <TypeDefs as super::Types>::auto_play,
        controls = <TypeDefs as super::Types>::controls,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        r#loop = <TypeDefs as super::Types>::r#loop,
        muted = <TypeDefs as super::Types>::muted,
        preload = <TypeDefs as super::Types>::preload,
        src = <TypeDefs as super::Types>::src,
        on_abort = <TypeDefs as super::Types>::on_abort,
        on_can_play = <TypeDefs as super::Types>::on_can_play,
        on_can_play_through = <TypeDefs as super::Types>::on_can_play_through,
        on_duration_change = <TypeDefs as super::Types>::on_duration_change,
        on_emptied = <TypeDefs as super::Types>::on_emptied,
        on_ended = <TypeDefs as super::Types>::on_ended,
        on_loaded_data = <TypeDefs as super::Types>::on_loaded_data,
        on_loaded_metadata = <TypeDefs as super::Types>::on_loaded_metadata,
        on_load_start = <TypeDefs as super::Types>::on_load_start,
        on_pause = Value,
        on_play = <TypeDefs as super::Types>::on_play,
        on_playing = <TypeDefs as super::Types>::on_playing,
        on_progress = <TypeDefs as super::Types>::on_progress,
        on_rate_change = <TypeDefs as super::Types>::on_rate_change,
        on_resize = <TypeDefs as super::Types>::on_resize,
        on_seeked = <TypeDefs as super::Types>::on_seeked,
        on_seeking = <TypeDefs as super::Types>::on_seeking,
        on_stalled = <TypeDefs as super::Types>::on_stalled,
        on_suspend = <TypeDefs as super::Types>::on_suspend,
        on_time_update = <TypeDefs as super::Types>::on_time_update,
        on_volume_change = <TypeDefs as super::Types>::on_volume_change,
        on_waiting = <TypeDefs as super::Types>::on_waiting,
    >;
    pub type on_play<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_play = <TypeDefs as super::Types>::auto_play,
        controls = <TypeDefs as super::Types>::controls,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        r#loop = <TypeDefs as super::Types>::r#loop,
        muted = <TypeDefs as super::Types>::muted,
        preload = <TypeDefs as super::Types>::preload,
        src = <TypeDefs as super::Types>::src,
        on_abort = <TypeDefs as super::Types>::on_abort,
        on_can_play = <TypeDefs as super::Types>::on_can_play,
        on_can_play_through = <TypeDefs as super::Types>::on_can_play_through,
        on_duration_change = <TypeDefs as super::Types>::on_duration_change,
        on_emptied = <TypeDefs as super::Types>::on_emptied,
        on_ended = <TypeDefs as super::Types>::on_ended,
        on_loaded_data = <TypeDefs as super::Types>::on_loaded_data,
        on_loaded_metadata = <TypeDefs as super::Types>::on_loaded_metadata,
        on_load_start = <TypeDefs as super::Types>::on_load_start,
        on_pause = <TypeDefs as super::Types>::on_pause,
        on_play = Value,
        on_playing = <TypeDefs as super::Types>::on_playing,
        on_progress = <TypeDefs as super::Types>::on_progress,
        on_rate_change = <TypeDefs as super::Types>::on_rate_change,
        on_resize = <TypeDefs as super::Types>::on_resize,
        on_seeked = <TypeDefs as super::Types>::on_seeked,
        on_seeking = <TypeDefs as super::Types>::on_seeking,
        on_stalled = <TypeDefs as super::Types>::on_stalled,
        on_suspend = <TypeDefs as super::Types>::on_suspend,
        on_time_update = <TypeDefs as super::Types>::on_time_update,
        on_volume_change = <TypeDefs as super::Types>::on_volume_change,
        on_waiting = <TypeDefs as super::Types>::on_waiting,
    >;
    pub type on_playing<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_play = <TypeDefs as super::Types>::auto_play,
        controls = <TypeDefs as super::Types>::controls,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        r#loop = <TypeDefs as super::Types>::r#loop,
        muted = <TypeDefs as super::Types>::muted,
        preload = <TypeDefs as super::Types>::preload,
        src = <TypeDefs as super::Types>::src,
        on_abort = <TypeDefs as super::Types>::on_abort,
        on_can_play = <TypeDefs as super::Types>::on_can_play,
        on_can_play_through = <TypeDefs as super::Types>::on_can_play_through,
        on_duration_change = <TypeDefs as super::Types>::on_duration_change,
        on_emptied = <TypeDefs as super::Types>::on_emptied,
        on_ended = <TypeDefs as super::Types>::on_ended,
        on_loaded_data = <TypeDefs as super::Types>::on_loaded_data,
        on_loaded_metadata = <TypeDefs as super::Types>::on_loaded_metadata,
        on_load_start = <TypeDefs as super::Types>::on_load_start,
        on_pause = <TypeDefs as super::Types>::on_pause,
        on_play = <TypeDefs as super::Types>::on_play,
        on_playing = Value,
        on_progress = <TypeDefs as super::Types>::on_progress,
        on_rate_change = <TypeDefs as super::Types>::on_rate_change,
        on_resize = <TypeDefs as super::Types>::on_resize,
        on_seeked = <TypeDefs as super::Types>::on_seeked,
        on_seeking = <TypeDefs as super::Types>::on_seeking,
        on_stalled = <TypeDefs as super::Types>::on_stalled,
        on_suspend = <TypeDefs as super::Types>::on_suspend,
        on_time_update = <TypeDefs as super::Types>::on_time_update,
        on_volume_change = <TypeDefs as super::Types>::on_volume_change,
        on_waiting = <TypeDefs as super::Types>::on_waiting,
    >;
    pub type on_progress<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_play = <TypeDefs as super::Types>::auto_play,
        controls = <TypeDefs as super::Types>::controls,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        r#loop = <TypeDefs as super::Types>::r#loop,
        muted = <TypeDefs as super::Types>::muted,
        preload = <TypeDefs as super::Types>::preload,
        src = <TypeDefs as super::Types>::src,
        on_abort = <TypeDefs as super::Types>::on_abort,
        on_can_play = <TypeDefs as super::Types>::on_can_play,
        on_can_play_through = <TypeDefs as super::Types>::on_can_play_through,
        on_duration_change = <TypeDefs as super::Types>::on_duration_change,
        on_emptied = <TypeDefs as super::Types>::on_emptied,
        on_ended = <TypeDefs as super::Types>::on_ended,
        on_loaded_data = <TypeDefs as super::Types>::on_loaded_data,
        on_loaded_metadata = <TypeDefs as super::Types>::on_loaded_metadata,
        on_load_start = <TypeDefs as super::Types>::on_load_start,
        on_pause = <TypeDefs as super::Types>::on_pause,
        on_play = <TypeDefs as super::Types>::on_play,
        on_playing = <TypeDefs as super::Types>::on_playing,
        on_progress = Value,
        on_rate_change = <TypeDefs as super::Types>::on_rate_change,
        on_resize = <TypeDefs as super::Types>::on_resize,
        on_seeked = <TypeDefs as super::Types>::on_seeked,
        on_seeking = <TypeDefs as super::Types>::on_seeking,
        on_stalled = <TypeDefs as super::Types>::on_stalled,
        on_suspend = <TypeDefs as super::Types>::on_suspend,
        on_time_update = <TypeDefs as super::Types>::on_time_update,
        on_volume_change = <TypeDefs as super::Types>::on_volume_change,
        on_waiting = <TypeDefs as super::Types>::on_waiting,
    >;
    pub type on_rate_change<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_play = <TypeDefs as super::Types>::auto_play,
        controls = <TypeDefs as super::Types>::controls,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        r#loop = <TypeDefs as super::Types>::r#loop,
        muted = <TypeDefs as super::Types>::muted,
        preload = <TypeDefs as super::Types>::preload,
        src = <TypeDefs as super::Types>::src,
        on_abort = <TypeDefs as super::Types>::on_abort,
        on_can_play = <TypeDefs as super::Types>::on_can_play,
        on_can_play_through = <TypeDefs as super::Types>::on_can_play_through,
        on_duration_change = <TypeDefs as super::Types>::on_duration_change,
        on_emptied = <TypeDefs as super::Types>::on_emptied,
        on_ended = <TypeDefs as super::Types>::on_ended,
        on_loaded_data = <TypeDefs as super::Types>::on_loaded_data,
        on_loaded_metadata = <TypeDefs as super::Types>::on_loaded_metadata,
        on_load_start = <TypeDefs as super::Types>::on_load_start,
        on_pause = <TypeDefs as super::Types>::on_pause,
        on_play = <TypeDefs as super::Types>::on_play,
        on_playing = <TypeDefs as super::Types>::on_playing,
        on_progress = <TypeDefs as super::Types>::on_progress,
        on_rate_change = Value,
        on_resize = <TypeDefs as super::Types>::on_resize,
        on_seeked = <TypeDefs as super::Types>::on_seeked,
        on_seeking = <TypeDefs as super::Types>::on_seeking,
        on_stalled = <TypeDefs as super::Types>::on_stalled,
        on_suspend = <TypeDefs as super::Types>::on_suspend,
        on_time_update = <TypeDefs as super::Types>::on_time_update,
        on_volume_change = <TypeDefs as super::Types>::on_volume_change,
        on_waiting = <TypeDefs as super::Types>::on_waiting,
    >;
    pub type on_resize<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_play = <TypeDefs as super::Types>::auto_play,
        controls = <TypeDefs as super::Types>::controls,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        r#loop = <TypeDefs as super::Types>::r#loop,
        muted = <TypeDefs as super::Types>::muted,
        preload = <TypeDefs as super::Types>::preload,
        src = <TypeDefs as super::Types>::src,
        on_abort = <TypeDefs as super::Types>::on_abort,
        on_can_play = <TypeDefs as super::Types>::on_can_play,
        on_can_play_through = <TypeDefs as super::Types>::on_can_play_through,
        on_duration_change = <TypeDefs as super::Types>::on_duration_change,
        on_emptied = <TypeDefs as super::Types>::on_emptied,
        on_ended = <TypeDefs as super::Types>::on_ended,
        on_loaded_data = <TypeDefs as super::Types>::on_loaded_data,
        on_loaded_metadata = <TypeDefs as super::Types>::on_loaded_metadata,
        on_load_start = <TypeDefs as super::Types>::on_load_start,
        on_pause = <TypeDefs as super::Types>::on_pause,
        on_play = <TypeDefs as super::Types>::on_play,
        on_playing = <TypeDefs as super::Types>::on_playing,
        on_progress = <TypeDefs as super::Types>::on_progress,
        on_rate_change = <TypeDefs as super::Types>::on_rate_change,
        on_resize = Value,
        on_seeked = <TypeDefs as super::Types>::on_seeked,
        on_seeking = <TypeDefs as super::Types>::on_seeking,
        on_stalled = <TypeDefs as super::Types>::on_stalled,
        on_suspend = <TypeDefs as super::Types>::on_suspend,
        on_time_update = <TypeDefs as super::Types>::on_time_update,
        on_volume_change = <TypeDefs as super::Types>::on_volume_change,
        on_waiting = <TypeDefs as super::Types>::on_waiting,
    >;
    pub type on_seeked<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_play = <TypeDefs as super::Types>::auto_play,
        controls = <TypeDefs as super::Types>::controls,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        r#loop = <TypeDefs as super::Types>::r#loop,
        muted = <TypeDefs as super::Types>::muted,
        preload = <TypeDefs as super::Types>::preload,
        src = <TypeDefs as super::Types>::src,
        on_abort = <TypeDefs as super::Types>::on_abort,
        on_can_play = <TypeDefs as super::Types>::on_can_play,
        on_can_play_through = <TypeDefs as super::Types>::on_can_play_through,
        on_duration_change = <TypeDefs as super::Types>::on_duration_change,
        on_emptied = <TypeDefs as super::Types>::on_emptied,
        on_ended = <TypeDefs as super::Types>::on_ended,
        on_loaded_data = <TypeDefs as super::Types>::on_loaded_data,
        on_loaded_metadata = <TypeDefs as super::Types>::on_loaded_metadata,
        on_load_start = <TypeDefs as super::Types>::on_load_start,
        on_pause = <TypeDefs as super::Types>::on_pause,
        on_play = <TypeDefs as super::Types>::on_play,
        on_playing = <TypeDefs as super::Types>::on_playing,
        on_progress = <TypeDefs as super::Types>::on_progress,
        on_rate_change = <TypeDefs as super::Types>::on_rate_change,
        on_resize = <TypeDefs as super::Types>::on_resize,
        on_seeked = Value,
        on_seeking = <TypeDefs as super::Types>::on_seeking,
        on_stalled = <TypeDefs as super::Types>::on_stalled,
        on_suspend = <TypeDefs as super::Types>::on_suspend,
        on_time_update = <TypeDefs as super::Types>::on_time_update,
        on_volume_change = <TypeDefs as super::Types>::on_volume_change,
        on_waiting = <TypeDefs as super::Types>::on_waiting,
    >;
    pub type on_seeking<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_play = <TypeDefs as super::Types>::auto_play,
        controls = <TypeDefs as super::Types>::controls,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        r#loop = <TypeDefs as super::Types>::r#loop,
        muted = <TypeDefs as super::Types>::muted,
        preload = <TypeDefs as super::Types>::preload,
        src = <TypeDefs as super::Types>::src,
        on_abort = <TypeDefs as super::Types>::on_abort,
        on_can_play = <TypeDefs as super::Types>::on_can_play,
        on_can_play_through = <TypeDefs as super::Types>::on_can_play_through,
        on_duration_change = <TypeDefs as super::Types>::on_duration_change,
        on_emptied = <TypeDefs as super::Types>::on_emptied,
        on_ended = <TypeDefs as super::Types>::on_ended,
        on_loaded_data = <TypeDefs as super::Types>::on_loaded_data,
        on_loaded_metadata = <TypeDefs as super::Types>::on_loaded_metadata,
        on_load_start = <TypeDefs as super::Types>::on_load_start,
        on_pause = <TypeDefs as super::Types>::on_pause,
        on_play = <TypeDefs as super::Types>::on_play,
        on_playing = <TypeDefs as super::Types>::on_playing,
        on_progress = <TypeDefs as super::Types>::on_progress,
        on_rate_change = <TypeDefs as super::Types>::on_rate_change,
        on_resize = <TypeDefs as super::Types>::on_resize,
        on_seeked = <TypeDefs as super::Types>::on_seeked,
        on_seeking = Value,
        on_stalled = <TypeDefs as super::Types>::on_stalled,
        on_suspend = <TypeDefs as super::Types>::on_suspend,
        on_time_update = <TypeDefs as super::Types>::on_time_update,
        on_volume_change = <TypeDefs as super::Types>::on_volume_change,
        on_waiting = <TypeDefs as super::Types>::on_waiting,
    >;
    pub type on_stalled<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_play = <TypeDefs as super::Types>::auto_play,
        controls = <TypeDefs as super::Types>::controls,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        r#loop = <TypeDefs as super::Types>::r#loop,
        muted = <TypeDefs as super::Types>::muted,
        preload = <TypeDefs as super::Types>::preload,
        src = <TypeDefs as super::Types>::src,
        on_abort = <TypeDefs as super::Types>::on_abort,
        on_can_play = <TypeDefs as super::Types>::on_can_play,
        on_can_play_through = <TypeDefs as super::Types>::on_can_play_through,
        on_duration_change = <TypeDefs as super::Types>::on_duration_change,
        on_emptied = <TypeDefs as super::Types>::on_emptied,
        on_ended = <TypeDefs as super::Types>::on_ended,
        on_loaded_data = <TypeDefs as super::Types>::on_loaded_data,
        on_loaded_metadata = <TypeDefs as super::Types>::on_loaded_metadata,
        on_load_start = <TypeDefs as super::Types>::on_load_start,
        on_pause = <TypeDefs as super::Types>::on_pause,
        on_play = <TypeDefs as super::Types>::on_play,
        on_playing = <TypeDefs as super::Types>::on_playing,
        on_progress = <TypeDefs as super::Types>::on_progress,
        on_rate_change = <TypeDefs as super::Types>::on_rate_change,
        on_resize = <TypeDefs as super::Types>::on_resize,
        on_seeked = <TypeDefs as super::Types>::on_seeked,
        on_seeking = <TypeDefs as super::Types>::on_seeking,
        on_stalled = Value,
        on_suspend = <TypeDefs as super::Types>::on_suspend,
        on_time_update = <TypeDefs as super::Types>::on_time_update,
        on_volume_change = <TypeDefs as super::Types>::on_volume_change,
        on_waiting = <TypeDefs as super::Types>::on_waiting,
    >;
    pub type on_suspend<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_play = <TypeDefs as super::Types>::auto_play,
        controls = <TypeDefs as super::Types>::controls,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        r#loop = <TypeDefs as super::Types>::r#loop,
        muted = <TypeDefs as super::Types>::muted,
        preload = <TypeDefs as super::Types>::preload,
        src = <TypeDefs as super::Types>::src,
        on_abort = <TypeDefs as super::Types>::on_abort,
        on_can_play = <TypeDefs as super::Types>::on_can_play,
        on_can_play_through = <TypeDefs as super::Types>::on_can_play_through,
        on_duration_change = <TypeDefs as super::Types>::on_duration_change,
        on_emptied = <TypeDefs as super::Types>::on_emptied,
        on_ended = <TypeDefs as super::Types>::on_ended,
        on_loaded_data = <TypeDefs as super::Types>::on_loaded_data,
        on_loaded_metadata = <TypeDefs as super::Types>::on_loaded_metadata,
        on_load_start = <TypeDefs as super::Types>::on_load_start,
        on_pause = <TypeDefs as super::Types>::on_pause,
        on_play = <TypeDefs as super::Types>::on_play,
        on_playing = <TypeDefs as super::Types>::on_playing,
        on_progress = <TypeDefs as super::Types>::on_progress,
        on_rate_change = <TypeDefs as super::Types>::on_rate_change,
        on_resize = <TypeDefs as super::Types>::on_resize,
        on_seeked = <TypeDefs as super::Types>::on_seeked,
        on_seeking = <TypeDefs as super::Types>::on_seeking,
        on_stalled = <TypeDefs as super::Types>::on_stalled,
        on_suspend = Value,
        on_time_update = <TypeDefs as super::Types>::on_time_update,
        on_volume_change = <TypeDefs as super::Types>::on_volume_change,
        on_waiting = <TypeDefs as super::Types>::on_waiting,
    >;
    pub type on_time_update<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_play = <TypeDefs as super::Types>::auto_play,
        controls = <TypeDefs as super::Types>::controls,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        r#loop = <TypeDefs as super::Types>::r#loop,
        muted = <TypeDefs as super::Types>::muted,
        preload = <TypeDefs as super::Types>::preload,
        src = <TypeDefs as super::Types>::src,
        on_abort = <TypeDefs as super::Types>::on_abort,
        on_can_play = <TypeDefs as super::Types>::on_can_play,
        on_can_play_through = <TypeDefs as super::Types>::on_can_play_through,
        on_duration_change = <TypeDefs as super::Types>::on_duration_change,
        on_emptied = <TypeDefs as super::Types>::on_emptied,
        on_ended = <TypeDefs as super::Types>::on_ended,
        on_loaded_data = <TypeDefs as super::Types>::on_loaded_data,
        on_loaded_metadata = <TypeDefs as super::Types>::on_loaded_metadata,
        on_load_start = <TypeDefs as super::Types>::on_load_start,
        on_pause = <TypeDefs as super::Types>::on_pause,
        on_play = <TypeDefs as super::Types>::on_play,
        on_playing = <TypeDefs as super::Types>::on_playing,
        on_progress = <TypeDefs as super::Types>::on_progress,
        on_rate_change = <TypeDefs as super::Types>::on_rate_change,
        on_resize = <TypeDefs as super::Types>::on_resize,
        on_seeked = <TypeDefs as super::Types>::on_seeked,
        on_seeking = <TypeDefs as super::Types>::on_seeking,
        on_stalled = <TypeDefs as super::Types>::on_stalled,
        on_suspend = <TypeDefs as super::Types>::on_suspend,
        on_time_update = Value,
        on_volume_change = <TypeDefs as super::Types>::on_volume_change,
        on_waiting = <TypeDefs as super::Types>::on_waiting,
    >;
    pub type on_volume_change<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_play = <TypeDefs as super::Types>::auto_play,
        controls = <TypeDefs as super::Types>::controls,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        r#loop = <TypeDefs as super::Types>::r#loop,
        muted = <TypeDefs as super::Types>::muted,
        preload = <TypeDefs as super::Types>::preload,
        src = <TypeDefs as super::Types>::src,
        on_abort = <TypeDefs as super::Types>::on_abort,
        on_can_play = <TypeDefs as super::Types>::on_can_play,
        on_can_play_through = <TypeDefs as super::Types>::on_can_play_through,
        on_duration_change = <TypeDefs as super::Types>::on_duration_change,
        on_emptied = <TypeDefs as super::Types>::on_emptied,
        on_ended = <TypeDefs as super::Types>::on_ended,
        on_loaded_data = <TypeDefs as super::Types>::on_loaded_data,
        on_loaded_metadata = <TypeDefs as super::Types>::on_loaded_metadata,
        on_load_start = <TypeDefs as super::Types>::on_load_start,
        on_pause = <TypeDefs as super::Types>::on_pause,
        on_play = <TypeDefs as super::Types>::on_play,
        on_playing = <TypeDefs as super::Types>::on_playing,
        on_progress = <TypeDefs as super::Types>::on_progress,
        on_rate_change = <TypeDefs as super::Types>::on_rate_change,
        on_resize = <TypeDefs as super::Types>::on_resize,
        on_seeked = <TypeDefs as super::Types>::on_seeked,
        on_seeking = <TypeDefs as super::Types>::on_seeking,
        on_stalled = <TypeDefs as super::Types>::on_stalled,
        on_suspend = <TypeDefs as super::Types>::on_suspend,
        on_time_update = <TypeDefs as super::Types>::on_time_update,
        on_volume_change = Value,
        on_waiting = <TypeDefs as super::Types>::on_waiting,
    >;
    pub type on_waiting<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_play = <TypeDefs as super::Types>::auto_play,
        controls = <TypeDefs as super::Types>::controls,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        r#loop = <TypeDefs as super::Types>::r#loop,
        muted = <TypeDefs as super::Types>::muted,
        preload = <TypeDefs as super::Types>::preload,
        src = <TypeDefs as super::Types>::src,
        on_abort = <TypeDefs as super::Types>::on_abort,
        on_can_play = <TypeDefs as super::Types>::on_can_play,
        on_can_play_through = <TypeDefs as super::Types>::on_can_play_through,
        on_duration_change = <TypeDefs as super::Types>::on_duration_change,
        on_emptied = <TypeDefs as super::Types>::on_emptied,
        on_ended = <TypeDefs as super::Types>::on_ended,
        on_loaded_data = <TypeDefs as super::Types>::on_loaded_data,
        on_loaded_metadata = <TypeDefs as super::Types>::on_loaded_metadata,
        on_load_start = <TypeDefs as super::Types>::on_load_start,
        on_pause = <TypeDefs as super::Types>::on_pause,
        on_play = <TypeDefs as super::Types>::on_play,
        on_playing = <TypeDefs as super::Types>::on_playing,
        on_progress = <TypeDefs as super::Types>::on_progress,
        on_rate_change = <TypeDefs as super::Types>::on_rate_change,
        on_resize = <TypeDefs as super::Types>::on_resize,
        on_seeked = <TypeDefs as super::Types>::on_seeked,
        on_seeking = <TypeDefs as super::Types>::on_seeking,
        on_stalled = <TypeDefs as super::Types>::on_stalled,
        on_suspend = <TypeDefs as super::Types>::on_suspend,
        on_time_update = <TypeDefs as super::Types>::on_time_update,
        on_volume_change = <TypeDefs as super::Types>::on_volume_change,
        on_waiting = Value,
    >;
}
mod trait_types {
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub trait Types {
        type HtmlElementProps: ?::core::marker::Sized + HtmlElementProps::Types;
        type auto_play: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>;
        type controls: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>;
        type cross_origin: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>;
        type r#loop: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>;
        type muted: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>;
        type preload: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>;
        type src: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>;
        type on_abort;
        type on_can_play;
        type on_can_play_through;
        type on_duration_change;
        type on_emptied;
        type on_ended;
        type on_loaded_data;
        type on_loaded_metadata;
        type on_load_start;
        type on_pause;
        type on_play;
        type on_playing;
        type on_progress;
        type on_rate_change;
        type on_resize;
        type on_seeked;
        type on_seeking;
        type on_stalled;
        type on_suspend;
        type on_time_update;
        type on_volume_change;
        type on_waiting;
    }
}
pub use trait_types::Types;
pub use trait_types::Types as ValidTypes;
pub mod data_struct {
    #[non_exhaustive]
    pub struct HtmlMediaElementProps<TypeDefs: super::Types + ?::core::marker::Sized> {
        pub HtmlElementProps: super::super::HtmlElementProps::Data<TypeDefs::HtmlElementProps>,
        pub auto_play: TypeDefs::auto_play,
        pub controls: TypeDefs::controls,
        pub cross_origin: TypeDefs::cross_origin,
        pub r#loop: TypeDefs::r#loop,
        pub muted: TypeDefs::muted,
        pub preload: TypeDefs::preload,
        pub src: TypeDefs::src,
        pub on_abort: TypeDefs::on_abort,
        pub on_can_play: TypeDefs::on_can_play,
        pub on_can_play_through: TypeDefs::on_can_play_through,
        pub on_duration_change: TypeDefs::on_duration_change,
        pub on_emptied: TypeDefs::on_emptied,
        pub on_ended: TypeDefs::on_ended,
        pub on_loaded_data: TypeDefs::on_loaded_data,
        pub on_loaded_metadata: TypeDefs::on_loaded_metadata,
        pub on_load_start: TypeDefs::on_load_start,
        pub on_pause: TypeDefs::on_pause,
        pub on_play: TypeDefs::on_play,
        pub on_playing: TypeDefs::on_playing,
        pub on_progress: TypeDefs::on_progress,
        pub on_rate_change: TypeDefs::on_rate_change,
        pub on_resize: TypeDefs::on_resize,
        pub on_seeked: TypeDefs::on_seeked,
        pub on_seeking: TypeDefs::on_seeking,
        pub on_stalled: TypeDefs::on_stalled,
        pub on_suspend: TypeDefs::on_suspend,
        pub on_time_update: TypeDefs::on_time_update,
        pub on_volume_change: TypeDefs::on_volume_change,
        pub on_waiting: TypeDefs::on_waiting,
    }
}
pub use ::core::convert::identity as Building;
pub use ::core::convert::identity as build;
pub use data_struct::HtmlMediaElementProps as Data;
pub use data_struct::HtmlMediaElementProps as Building;
pub struct Replacing<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
mod types_initial {
    #[allow(unused_imports)]
    use super::super::*;
    pub type TypesInitial = dyn super::Types<
        HtmlElementProps = HtmlElementProps::TypesInitial,
        auto_play = (),
        controls = (),
        cross_origin = (),
        r#loop = (),
        muted = (),
        preload = (),
        src = (),
        on_abort = (),
        on_can_play = (),
        on_can_play_through = (),
        on_duration_change = (),
        on_emptied = (),
        on_ended = (),
        on_loaded_data = (),
        on_loaded_metadata = (),
        on_load_start = (),
        on_pause = (),
        on_play = (),
        on_playing = (),
        on_progress = (),
        on_rate_change = (),
        on_resize = (),
        on_seeked = (),
        on_seeking = (),
        on_stalled = (),
        on_suspend = (),
        on_time_update = (),
        on_volume_change = (),
        on_waiting = (),
    >;
}
pub use types_initial::TypesInitial;
pub type DataInitial = Data<TypesInitial>;
#[cfg(feature = "csr")]
pub mod render_state {
    #[allow(non_camel_case_types)]
    pub trait RenderStateTypes {
        type HtmlElementProps: crate::imports::frender_csr::props::IntrinsicComponentPollReactive;
        type auto_play;
        type controls;
        type cross_origin;
        type r#loop;
        type muted;
        type preload;
        type src;
        type on_abort;
        type on_can_play;
        type on_can_play_through;
        type on_duration_change;
        type on_emptied;
        type on_ended;
        type on_loaded_data;
        type on_loaded_metadata;
        type on_load_start;
        type on_pause;
        type on_play;
        type on_playing;
        type on_progress;
        type on_rate_change;
        type on_resize;
        type on_seeked;
        type on_seeking;
        type on_stalled;
        type on_suspend;
        type on_time_update;
        type on_volume_change;
        type on_waiting;
    }
    crate::imports::pin_project! {
        #[project = RenderStateProj] pub struct RenderState < TypeDefs : RenderStateTypes
        > where TypeDefs : ? ::core::marker::Sized { #[pin] pub HtmlElementProps :
        TypeDefs::HtmlElementProps, pub auto_play : TypeDefs::auto_play, pub controls :
        TypeDefs::controls, pub cross_origin : TypeDefs::cross_origin, pub r#loop :
        TypeDefs::r#loop, pub muted : TypeDefs::muted, pub preload : TypeDefs::preload,
        pub src : TypeDefs::src, pub on_abort : TypeDefs::on_abort, pub on_can_play :
        TypeDefs::on_can_play, pub on_can_play_through : TypeDefs::on_can_play_through,
        pub on_duration_change : TypeDefs::on_duration_change, pub on_emptied :
        TypeDefs::on_emptied, pub on_ended : TypeDefs::on_ended, pub on_loaded_data :
        TypeDefs::on_loaded_data, pub on_loaded_metadata : TypeDefs::on_loaded_metadata,
        pub on_load_start : TypeDefs::on_load_start, pub on_pause : TypeDefs::on_pause,
        pub on_play : TypeDefs::on_play, pub on_playing : TypeDefs::on_playing, pub
        on_progress : TypeDefs::on_progress, pub on_rate_change :
        TypeDefs::on_rate_change, pub on_resize : TypeDefs::on_resize, pub on_seeked :
        TypeDefs::on_seeked, pub on_seeking : TypeDefs::on_seeking, pub on_stalled :
        TypeDefs::on_stalled, pub on_suspend : TypeDefs::on_suspend, pub on_time_update :
        TypeDefs::on_time_update, pub on_volume_change : TypeDefs::on_volume_change, pub
        on_waiting : TypeDefs::on_waiting, }
    }
    impl<TypeDefs: ?::core::marker::Sized + RenderStateTypes> RenderState<TypeDefs> {
        #[inline(always)]
        pub(crate) fn pin_project(
            self: ::core::pin::Pin<&mut Self>,
        ) -> RenderStateProj<'_, TypeDefs> {
            self.project()
        }
    }
    impl<TypeDefs: ?::core::marker::Sized + RenderStateTypes>
        crate::imports::frender_csr::props::IntrinsicComponentPollReactive
        for RenderState<TypeDefs>
    {
        #[inline]
        fn intrinsic_component_poll_reactive(
            self: ::core::pin::Pin<&mut Self>,
            cx: &mut ::core::task::Context<'_>,
        ) -> ::core::task::Poll<bool> {
            crate::imports::frender_csr::props::IntrinsicComponentPollReactive::intrinsic_component_poll_reactive(
                self.project().HtmlElementProps,
                cx,
            )
        }
    }
}
mod builder_and_replacer {
    #[allow(unused_imports)]
    use super::super::*;
    impl<TypeDefs: super::Types + ?::core::marker::Sized> super::Building<TypeDefs> {
        ///See [`HtmlElementProps::children`]
        #[inline(always)]
        pub fn children<V>(
            self,
            children: V,
        ) -> super::Building<super::overwrite::children<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.children(children),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::class`]
        #[inline(always)]
        pub fn class<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            class: V,
        ) -> super::Building<super::overwrite::class<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.class(class),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::id`]
        #[inline(always)]
        pub fn id<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            id: V,
        ) -> super::Building<super::overwrite::id<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.id(id),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::part`]
        #[inline(always)]
        pub fn part<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            part: V,
        ) -> super::Building<super::overwrite::part<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.part(part),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_cancel`]
        #[inline(always)]
        pub fn on_cancel<V>(
            self,
            on_cancel: V,
        ) -> super::Building<super::overwrite::on_cancel<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_cancel(on_cancel),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_error`]
        #[inline(always)]
        pub fn on_error<V>(
            self,
            on_error: V,
        ) -> super::Building<super::overwrite::on_error<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_error(on_error),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_scroll`]
        #[inline(always)]
        pub fn on_scroll<V>(
            self,
            on_scroll: V,
        ) -> super::Building<super::overwrite::on_scroll<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_scroll(on_scroll),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_security_policy_violation`]
        #[inline(always)]
        pub fn on_security_policy_violation<V>(
            self,
            on_security_policy_violation: V,
        ) -> super::Building<super::overwrite::on_security_policy_violation<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self
                    .HtmlElementProps
                    .on_security_policy_violation(on_security_policy_violation),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_select`]
        #[inline(always)]
        pub fn on_select<V>(
            self,
            on_select: V,
        ) -> super::Building<super::overwrite::on_select<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_select(on_select),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_wheel`]
        #[inline(always)]
        pub fn on_wheel<V>(
            self,
            on_wheel: V,
        ) -> super::Building<super::overwrite::on_wheel<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_wheel(on_wheel),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_copy`]
        #[inline(always)]
        pub fn on_copy<V>(
            self,
            on_copy: V,
        ) -> super::Building<super::overwrite::on_copy<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_copy(on_copy),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_cut`]
        #[inline(always)]
        pub fn on_cut<V>(
            self,
            on_cut: V,
        ) -> super::Building<super::overwrite::on_cut<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_cut(on_cut),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_paste`]
        #[inline(always)]
        pub fn on_paste<V>(
            self,
            on_paste: V,
        ) -> super::Building<super::overwrite::on_paste<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_paste(on_paste),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_composition_end`]
        #[inline(always)]
        pub fn on_composition_end<V>(
            self,
            on_composition_end: V,
        ) -> super::Building<super::overwrite::on_composition_end<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_composition_end(on_composition_end),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_composition_start`]
        #[inline(always)]
        pub fn on_composition_start<V>(
            self,
            on_composition_start: V,
        ) -> super::Building<super::overwrite::on_composition_start<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self
                    .HtmlElementProps
                    .on_composition_start(on_composition_start),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_composition_update`]
        #[inline(always)]
        pub fn on_composition_update<V>(
            self,
            on_composition_update: V,
        ) -> super::Building<super::overwrite::on_composition_update<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self
                    .HtmlElementProps
                    .on_composition_update(on_composition_update),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_blur`]
        #[inline(always)]
        pub fn on_blur<V>(
            self,
            on_blur: V,
        ) -> super::Building<super::overwrite::on_blur<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_blur(on_blur),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_focus`]
        #[inline(always)]
        pub fn on_focus<V>(
            self,
            on_focus: V,
        ) -> super::Building<super::overwrite::on_focus<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_focus(on_focus),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_focus_in`]
        #[inline(always)]
        pub fn on_focus_in<V>(
            self,
            on_focus_in: V,
        ) -> super::Building<super::overwrite::on_focus_in<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_focus_in(on_focus_in),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_focus_out`]
        #[inline(always)]
        pub fn on_focus_out<V>(
            self,
            on_focus_out: V,
        ) -> super::Building<super::overwrite::on_focus_out<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_focus_out(on_focus_out),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_fullscreen_change`]
        #[inline(always)]
        pub fn on_fullscreen_change<V>(
            self,
            on_fullscreen_change: V,
        ) -> super::Building<super::overwrite::on_fullscreen_change<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self
                    .HtmlElementProps
                    .on_fullscreen_change(on_fullscreen_change),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_fullscreen_error`]
        #[inline(always)]
        pub fn on_fullscreen_error<V>(
            self,
            on_fullscreen_error: V,
        ) -> super::Building<super::overwrite::on_fullscreen_error<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self
                    .HtmlElementProps
                    .on_fullscreen_error(on_fullscreen_error),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_key_down`]
        #[inline(always)]
        pub fn on_key_down<V>(
            self,
            on_key_down: V,
        ) -> super::Building<super::overwrite::on_key_down<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_key_down(on_key_down),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_key_up`]
        #[inline(always)]
        pub fn on_key_up<V>(
            self,
            on_key_up: V,
        ) -> super::Building<super::overwrite::on_key_up<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_key_up(on_key_up),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_aux_click`]
        #[inline(always)]
        pub fn on_aux_click<V>(
            self,
            on_aux_click: V,
        ) -> super::Building<super::overwrite::on_aux_click<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_aux_click(on_aux_click),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_click`]
        #[inline(always)]
        pub fn on_click<V>(
            self,
            on_click: V,
        ) -> super::Building<super::overwrite::on_click<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_click(on_click),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_context_menu`]
        #[inline(always)]
        pub fn on_context_menu<V>(
            self,
            on_context_menu: V,
        ) -> super::Building<super::overwrite::on_context_menu<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_context_menu(on_context_menu),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_double_click`]
        #[inline(always)]
        pub fn on_double_click<V>(
            self,
            on_double_click: V,
        ) -> super::Building<super::overwrite::on_double_click<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_double_click(on_double_click),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_mouse_down`]
        #[inline(always)]
        pub fn on_mouse_down<V>(
            self,
            on_mouse_down: V,
        ) -> super::Building<super::overwrite::on_mouse_down<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_mouse_down(on_mouse_down),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_mouse_enter`]
        #[inline(always)]
        pub fn on_mouse_enter<V>(
            self,
            on_mouse_enter: V,
        ) -> super::Building<super::overwrite::on_mouse_enter<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_mouse_enter(on_mouse_enter),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_mouse_leave`]
        #[inline(always)]
        pub fn on_mouse_leave<V>(
            self,
            on_mouse_leave: V,
        ) -> super::Building<super::overwrite::on_mouse_leave<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_mouse_leave(on_mouse_leave),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_mouse_move`]
        #[inline(always)]
        pub fn on_mouse_move<V>(
            self,
            on_mouse_move: V,
        ) -> super::Building<super::overwrite::on_mouse_move<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_mouse_move(on_mouse_move),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_mouse_out`]
        #[inline(always)]
        pub fn on_mouse_out<V>(
            self,
            on_mouse_out: V,
        ) -> super::Building<super::overwrite::on_mouse_out<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_mouse_out(on_mouse_out),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_mouse_over`]
        #[inline(always)]
        pub fn on_mouse_over<V>(
            self,
            on_mouse_over: V,
        ) -> super::Building<super::overwrite::on_mouse_over<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_mouse_over(on_mouse_over),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_mouse_up`]
        #[inline(always)]
        pub fn on_mouse_up<V>(
            self,
            on_mouse_up: V,
        ) -> super::Building<super::overwrite::on_mouse_up<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_mouse_up(on_mouse_up),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_touch_cancel`]
        #[inline(always)]
        pub fn on_touch_cancel<V>(
            self,
            on_touch_cancel: V,
        ) -> super::Building<super::overwrite::on_touch_cancel<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_touch_cancel(on_touch_cancel),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_touch_end`]
        #[inline(always)]
        pub fn on_touch_end<V>(
            self,
            on_touch_end: V,
        ) -> super::Building<super::overwrite::on_touch_end<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_touch_end(on_touch_end),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_touch_move`]
        #[inline(always)]
        pub fn on_touch_move<V>(
            self,
            on_touch_move: V,
        ) -> super::Building<super::overwrite::on_touch_move<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_touch_move(on_touch_move),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_touch_start`]
        #[inline(always)]
        pub fn on_touch_start<V>(
            self,
            on_touch_start: V,
        ) -> super::Building<super::overwrite::on_touch_start<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_touch_start(on_touch_start),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::access_key`]
        #[inline(always)]
        pub fn access_key<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
        >(
            self,
            access_key: V,
        ) -> super::Building<super::overwrite::access_key<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.access_key(access_key),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::auto_capitalize`]
        #[inline(always)]
        pub fn auto_capitalize<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
        >(
            self,
            auto_capitalize: V,
        ) -> super::Building<super::overwrite::auto_capitalize<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.auto_capitalize(auto_capitalize),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::auto_focus`]
        #[inline(always)]
        pub fn auto_focus<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
        >(
            self,
            auto_focus: V,
        ) -> super::Building<super::overwrite::auto_focus<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.auto_focus(auto_focus),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::context_menu`]
        #[inline(always)]
        pub fn context_menu<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
        >(
            self,
            context_menu: V,
        ) -> super::Building<super::overwrite::context_menu<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.context_menu(context_menu),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::dir`]
        #[inline(always)]
        pub fn dir<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            dir: V,
        ) -> super::Building<super::overwrite::dir<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.dir(dir),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::draggable`]
        #[inline(always)]
        pub fn draggable<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
        >(
            self,
            draggable: V,
        ) -> super::Building<super::overwrite::draggable<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.draggable(draggable),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::enter_key_hint`]
        #[inline(always)]
        pub fn enter_key_hint<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
        >(
            self,
            enter_key_hint: V,
        ) -> super::Building<super::overwrite::enter_key_hint<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.enter_key_hint(enter_key_hint),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::hidden`]
        #[inline(always)]
        pub fn hidden<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>>(
            self,
            hidden: V,
        ) -> super::Building<super::overwrite::hidden<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.hidden(hidden),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::inert`]
        #[inline(always)]
        pub fn inert<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>>(
            self,
            inert: V,
        ) -> super::Building<super::overwrite::inert<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.inert(inert),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::input_mode`]
        #[inline(always)]
        pub fn input_mode<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
        >(
            self,
            input_mode: V,
        ) -> super::Building<super::overwrite::input_mode<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.input_mode(input_mode),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::is`]
        #[inline(always)]
        pub fn is<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            is: V,
        ) -> super::Building<super::overwrite::is<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.is(is),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::item_id`]
        #[inline(always)]
        pub fn item_id<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            item_id: V,
        ) -> super::Building<super::overwrite::item_id<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.item_id(item_id),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::item_prop`]
        #[inline(always)]
        pub fn item_prop<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            item_prop: V,
        ) -> super::Building<super::overwrite::item_prop<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.item_prop(item_prop),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::item_ref`]
        #[inline(always)]
        pub fn item_ref<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            item_ref: V,
        ) -> super::Building<super::overwrite::item_ref<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.item_ref(item_ref),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::item_scope`]
        #[inline(always)]
        pub fn item_scope<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
        >(
            self,
            item_scope: V,
        ) -> super::Building<super::overwrite::item_scope<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.item_scope(item_scope),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::item_type`]
        #[inline(always)]
        pub fn item_type<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            item_type: V,
        ) -> super::Building<super::overwrite::item_type<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.item_type(item_type),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::lang`]
        #[inline(always)]
        pub fn lang<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            lang: V,
        ) -> super::Building<super::overwrite::lang<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.lang(lang),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::nonce`]
        #[inline(always)]
        pub fn nonce<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            nonce: V,
        ) -> super::Building<super::overwrite::nonce<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.nonce(nonce),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::role`]
        #[inline(always)]
        pub fn role<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            role: V,
        ) -> super::Building<super::overwrite::role<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.role(role),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::slot`]
        #[inline(always)]
        pub fn slot<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            slot: V,
        ) -> super::Building<super::overwrite::slot<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.slot(slot),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::spellcheck`]
        #[inline(always)]
        pub fn spellcheck<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
        >(
            self,
            spellcheck: V,
        ) -> super::Building<super::overwrite::spellcheck<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.spellcheck(spellcheck),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::style`]
        #[inline(always)]
        pub fn style<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            style: V,
        ) -> super::Building<super::overwrite::style<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.style(style),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::tab_index`]
        #[inline(always)]
        pub fn tab_index<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<i32>>(
            self,
            tab_index: V,
        ) -> super::Building<super::overwrite::tab_index<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.tab_index(tab_index),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::title`]
        #[inline(always)]
        pub fn title<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            title: V,
        ) -> super::Building<super::overwrite::title<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.title(title),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::translate`]
        #[inline(always)]
        pub fn translate<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            translate: V,
        ) -> super::Building<super::overwrite::translate<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.translate(translate),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::virtual_keyboard_policy`]
        #[inline(always)]
        pub fn virtual_keyboard_policy<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
        >(
            self,
            virtual_keyboard_policy: V,
        ) -> super::Building<super::overwrite::virtual_keyboard_policy<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self
                    .HtmlElementProps
                    .virtual_keyboard_policy(virtual_keyboard_policy),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_invalid`]
        #[inline(always)]
        pub fn on_invalid<V>(
            self,
            on_invalid: V,
        ) -> super::Building<super::overwrite::on_invalid<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_invalid(on_invalid),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_animation_cancel`]
        #[inline(always)]
        pub fn on_animation_cancel<V>(
            self,
            on_animation_cancel: V,
        ) -> super::Building<super::overwrite::on_animation_cancel<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self
                    .HtmlElementProps
                    .on_animation_cancel(on_animation_cancel),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_animation_end`]
        #[inline(always)]
        pub fn on_animation_end<V>(
            self,
            on_animation_end: V,
        ) -> super::Building<super::overwrite::on_animation_end<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_animation_end(on_animation_end),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_animation_iteration`]
        #[inline(always)]
        pub fn on_animation_iteration<V>(
            self,
            on_animation_iteration: V,
        ) -> super::Building<super::overwrite::on_animation_iteration<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self
                    .HtmlElementProps
                    .on_animation_iteration(on_animation_iteration),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_animation_start`]
        #[inline(always)]
        pub fn on_animation_start<V>(
            self,
            on_animation_start: V,
        ) -> super::Building<super::overwrite::on_animation_start<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_animation_start(on_animation_start),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_before_input`]
        #[inline(always)]
        pub fn on_before_input<V>(
            self,
            on_before_input: V,
        ) -> super::Building<super::overwrite::on_before_input<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_before_input(on_before_input),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_input`]
        #[inline(always)]
        pub fn on_input<V>(
            self,
            on_input: V,
        ) -> super::Building<super::overwrite::on_input<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_input(on_input),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_change`]
        #[inline(always)]
        pub fn on_change<V>(
            self,
            on_change: V,
        ) -> super::Building<super::overwrite::on_change<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_change(on_change),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_got_pointer_capture`]
        #[inline(always)]
        pub fn on_got_pointer_capture<V>(
            self,
            on_got_pointer_capture: V,
        ) -> super::Building<super::overwrite::on_got_pointer_capture<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self
                    .HtmlElementProps
                    .on_got_pointer_capture(on_got_pointer_capture),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_lost_pointer_capture`]
        #[inline(always)]
        pub fn on_lost_pointer_capture<V>(
            self,
            on_lost_pointer_capture: V,
        ) -> super::Building<super::overwrite::on_lost_pointer_capture<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self
                    .HtmlElementProps
                    .on_lost_pointer_capture(on_lost_pointer_capture),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_pointer_cancel`]
        #[inline(always)]
        pub fn on_pointer_cancel<V>(
            self,
            on_pointer_cancel: V,
        ) -> super::Building<super::overwrite::on_pointer_cancel<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_pointer_cancel(on_pointer_cancel),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_pointer_down`]
        #[inline(always)]
        pub fn on_pointer_down<V>(
            self,
            on_pointer_down: V,
        ) -> super::Building<super::overwrite::on_pointer_down<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_pointer_down(on_pointer_down),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_pointer_enter`]
        #[inline(always)]
        pub fn on_pointer_enter<V>(
            self,
            on_pointer_enter: V,
        ) -> super::Building<super::overwrite::on_pointer_enter<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_pointer_enter(on_pointer_enter),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_pointer_leave`]
        #[inline(always)]
        pub fn on_pointer_leave<V>(
            self,
            on_pointer_leave: V,
        ) -> super::Building<super::overwrite::on_pointer_leave<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_pointer_leave(on_pointer_leave),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_pointer_move`]
        #[inline(always)]
        pub fn on_pointer_move<V>(
            self,
            on_pointer_move: V,
        ) -> super::Building<super::overwrite::on_pointer_move<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_pointer_move(on_pointer_move),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_pointer_out`]
        #[inline(always)]
        pub fn on_pointer_out<V>(
            self,
            on_pointer_out: V,
        ) -> super::Building<super::overwrite::on_pointer_out<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_pointer_out(on_pointer_out),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_pointer_over`]
        #[inline(always)]
        pub fn on_pointer_over<V>(
            self,
            on_pointer_over: V,
        ) -> super::Building<super::overwrite::on_pointer_over<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_pointer_over(on_pointer_over),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_pointer_up`]
        #[inline(always)]
        pub fn on_pointer_up<V>(
            self,
            on_pointer_up: V,
        ) -> super::Building<super::overwrite::on_pointer_up<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_pointer_up(on_pointer_up),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_transition_cancel`]
        #[inline(always)]
        pub fn on_transition_cancel<V>(
            self,
            on_transition_cancel: V,
        ) -> super::Building<super::overwrite::on_transition_cancel<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self
                    .HtmlElementProps
                    .on_transition_cancel(on_transition_cancel),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_transition_end`]
        #[inline(always)]
        pub fn on_transition_end<V>(
            self,
            on_transition_end: V,
        ) -> super::Building<super::overwrite::on_transition_end<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_transition_end(on_transition_end),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_transition_run`]
        #[inline(always)]
        pub fn on_transition_run<V>(
            self,
            on_transition_run: V,
        ) -> super::Building<super::overwrite::on_transition_run<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_transition_run(on_transition_run),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_transition_start`]
        #[inline(always)]
        pub fn on_transition_start<V>(
            self,
            on_transition_start: V,
        ) -> super::Building<super::overwrite::on_transition_start<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self
                    .HtmlElementProps
                    .on_transition_start(on_transition_start),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_drag`]
        #[inline(always)]
        pub fn on_drag<V>(
            self,
            on_drag: V,
        ) -> super::Building<super::overwrite::on_drag<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_drag(on_drag),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_drag_end`]
        #[inline(always)]
        pub fn on_drag_end<V>(
            self,
            on_drag_end: V,
        ) -> super::Building<super::overwrite::on_drag_end<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_drag_end(on_drag_end),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_drag_enter`]
        #[inline(always)]
        pub fn on_drag_enter<V>(
            self,
            on_drag_enter: V,
        ) -> super::Building<super::overwrite::on_drag_enter<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_drag_enter(on_drag_enter),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_drag_leave`]
        #[inline(always)]
        pub fn on_drag_leave<V>(
            self,
            on_drag_leave: V,
        ) -> super::Building<super::overwrite::on_drag_leave<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_drag_leave(on_drag_leave),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_drag_over`]
        #[inline(always)]
        pub fn on_drag_over<V>(
            self,
            on_drag_over: V,
        ) -> super::Building<super::overwrite::on_drag_over<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_drag_over(on_drag_over),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_drag_start`]
        #[inline(always)]
        pub fn on_drag_start<V>(
            self,
            on_drag_start: V,
        ) -> super::Building<super::overwrite::on_drag_start<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_drag_start(on_drag_start),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        ///See [`HtmlElementProps::on_drop`]
        #[inline(always)]
        pub fn on_drop<V>(
            self,
            on_drop: V,
        ) -> super::Building<super::overwrite::on_drop<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.on_drop(on_drop),
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        #[inline(always)]
        pub fn auto_play<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
        >(
            self,
            auto_play: V,
        ) -> super::Building<super::overwrite::auto_play<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        #[inline(always)]
        pub fn controls<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>>(
            self,
            controls: V,
        ) -> super::Building<super::overwrite::controls<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_play: self.auto_play,
                controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        #[inline(always)]
        pub fn cross_origin<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
        >(
            self,
            cross_origin: V,
        ) -> super::Building<super::overwrite::cross_origin<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        #[inline(always)]
        pub fn r#loop<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>>(
            self,
            r#loop: V,
        ) -> super::Building<super::overwrite::r#loop<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        #[inline(always)]
        pub fn muted<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>>(
            self,
            muted: V,
        ) -> super::Building<super::overwrite::muted<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        #[inline(always)]
        pub fn preload<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            preload: V,
        ) -> super::Building<super::overwrite::preload<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        #[inline(always)]
        pub fn src<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            src: V,
        ) -> super::Building<super::overwrite::src<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        #[inline(always)]
        pub fn on_abort<V>(
            self,
            on_abort: V,
        ) -> super::Building<super::overwrite::on_abort<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        #[inline(always)]
        pub fn on_can_play<V>(
            self,
            on_can_play: V,
        ) -> super::Building<super::overwrite::on_can_play<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        #[inline(always)]
        pub fn on_can_play_through<V>(
            self,
            on_can_play_through: V,
        ) -> super::Building<super::overwrite::on_can_play_through<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        #[inline(always)]
        pub fn on_duration_change<V>(
            self,
            on_duration_change: V,
        ) -> super::Building<super::overwrite::on_duration_change<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        #[inline(always)]
        pub fn on_emptied<V>(
            self,
            on_emptied: V,
        ) -> super::Building<super::overwrite::on_emptied<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        #[inline(always)]
        pub fn on_ended<V>(
            self,
            on_ended: V,
        ) -> super::Building<super::overwrite::on_ended<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        #[inline(always)]
        pub fn on_loaded_data<V>(
            self,
            on_loaded_data: V,
        ) -> super::Building<super::overwrite::on_loaded_data<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        #[inline(always)]
        pub fn on_loaded_metadata<V>(
            self,
            on_loaded_metadata: V,
        ) -> super::Building<super::overwrite::on_loaded_metadata<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        #[inline(always)]
        pub fn on_load_start<V>(
            self,
            on_load_start: V,
        ) -> super::Building<super::overwrite::on_load_start<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        #[inline(always)]
        pub fn on_pause<V>(
            self,
            on_pause: V,
        ) -> super::Building<super::overwrite::on_pause<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        #[inline(always)]
        pub fn on_play<V>(
            self,
            on_play: V,
        ) -> super::Building<super::overwrite::on_play<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        #[inline(always)]
        pub fn on_playing<V>(
            self,
            on_playing: V,
        ) -> super::Building<super::overwrite::on_playing<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        #[inline(always)]
        pub fn on_progress<V>(
            self,
            on_progress: V,
        ) -> super::Building<super::overwrite::on_progress<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        #[inline(always)]
        pub fn on_rate_change<V>(
            self,
            on_rate_change: V,
        ) -> super::Building<super::overwrite::on_rate_change<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        #[inline(always)]
        pub fn on_resize<V>(
            self,
            on_resize: V,
        ) -> super::Building<super::overwrite::on_resize<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        #[inline(always)]
        pub fn on_seeked<V>(
            self,
            on_seeked: V,
        ) -> super::Building<super::overwrite::on_seeked<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        #[inline(always)]
        pub fn on_seeking<V>(
            self,
            on_seeking: V,
        ) -> super::Building<super::overwrite::on_seeking<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        #[inline(always)]
        pub fn on_stalled<V>(
            self,
            on_stalled: V,
        ) -> super::Building<super::overwrite::on_stalled<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        #[inline(always)]
        pub fn on_suspend<V>(
            self,
            on_suspend: V,
        ) -> super::Building<super::overwrite::on_suspend<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        #[inline(always)]
        pub fn on_time_update<V>(
            self,
            on_time_update: V,
        ) -> super::Building<super::overwrite::on_time_update<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        #[inline(always)]
        pub fn on_volume_change<V>(
            self,
            on_volume_change: V,
        ) -> super::Building<super::overwrite::on_volume_change<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change,
                on_waiting: self.on_waiting,
            }
        }
        #[inline(always)]
        pub fn on_waiting<V>(
            self,
            on_waiting: V,
        ) -> super::Building<super::overwrite::on_waiting<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_play: self.auto_play,
                controls: self.controls,
                cross_origin: self.cross_origin,
                r#loop: self.r#loop,
                muted: self.muted,
                preload: self.preload,
                src: self.src,
                on_abort: self.on_abort,
                on_can_play: self.on_can_play,
                on_can_play_through: self.on_can_play_through,
                on_duration_change: self.on_duration_change,
                on_emptied: self.on_emptied,
                on_ended: self.on_ended,
                on_loaded_data: self.on_loaded_data,
                on_loaded_metadata: self.on_loaded_metadata,
                on_load_start: self.on_load_start,
                on_pause: self.on_pause,
                on_play: self.on_play,
                on_playing: self.on_playing,
                on_progress: self.on_progress,
                on_rate_change: self.on_rate_change,
                on_resize: self.on_resize,
                on_seeked: self.on_seeked,
                on_seeking: self.on_seeking,
                on_stalled: self.on_stalled,
                on_suspend: self.on_suspend,
                on_time_update: self.on_time_update,
                on_volume_change: self.on_volume_change,
                on_waiting,
            }
        }
    }
}
#[cfg(feature = "csr")]
mod impl_update_element {
    #[allow(unused_imports)]
    use super::super::*;
    impl<TypeDefs: ?::core::marker::Sized + super::Types>
        crate::imports::frender_csr::props::UpdateElement<web_sys::HtmlMediaElement>
        for super::Data<TypeDefs>
    where
        HtmlElementProps::Data<TypeDefs::HtmlElementProps>:
            crate::imports::frender_csr::props::UpdateElement<web_sys::HtmlElement>,
        TypeDefs::on_abort:
            crate::imports::frender_html::props::UpdateDomEventListener<events::Abort>,
        TypeDefs::on_can_play:
            crate::imports::frender_html::props::UpdateDomEventListener<events::CanPlay>,
        TypeDefs::on_can_play_through:
            crate::imports::frender_html::props::UpdateDomEventListener<events::CanPlayThrough>,
        TypeDefs::on_duration_change:
            crate::imports::frender_html::props::UpdateDomEventListener<events::DurationChange>,
        TypeDefs::on_emptied:
            crate::imports::frender_html::props::UpdateDomEventListener<events::Emptied>,
        TypeDefs::on_ended:
            crate::imports::frender_html::props::UpdateDomEventListener<events::Ended>,
        TypeDefs::on_loaded_data:
            crate::imports::frender_html::props::UpdateDomEventListener<events::LoadedData>,
        TypeDefs::on_loaded_metadata:
            crate::imports::frender_html::props::UpdateDomEventListener<events::LoadedMetadata>,
        TypeDefs::on_load_start:
            crate::imports::frender_html::props::UpdateDomEventListener<events::LoadStart>,
        TypeDefs::on_pause:
            crate::imports::frender_html::props::UpdateDomEventListener<events::Pause>,
        TypeDefs::on_play:
            crate::imports::frender_html::props::UpdateDomEventListener<events::Play>,
        TypeDefs::on_playing:
            crate::imports::frender_html::props::UpdateDomEventListener<events::Playing>,
        TypeDefs::on_progress:
            crate::imports::frender_html::props::UpdateDomEventListener<events::Progress>,
        TypeDefs::on_rate_change:
            crate::imports::frender_html::props::UpdateDomEventListener<events::RateChange>,
        TypeDefs::on_resize:
            crate::imports::frender_html::props::UpdateDomEventListener<events::Resize>,
        TypeDefs::on_seeked:
            crate::imports::frender_html::props::UpdateDomEventListener<events::Seeked>,
        TypeDefs::on_seeking:
            crate::imports::frender_html::props::UpdateDomEventListener<events::Seeking>,
        TypeDefs::on_stalled:
            crate::imports::frender_html::props::UpdateDomEventListener<events::Stalled>,
        TypeDefs::on_suspend:
            crate::imports::frender_html::props::UpdateDomEventListener<events::Suspend>,
        TypeDefs::on_time_update:
            crate::imports::frender_html::props::UpdateDomEventListener<events::TimeUpdate>,
        TypeDefs::on_volume_change:
            crate::imports::frender_html::props::UpdateDomEventListener<events::VolumeChange>,
        TypeDefs::on_waiting:
            crate::imports::frender_html::props::UpdateDomEventListener<events::Waiting>,
    {
        type State = super::render_state::RenderState<
            dyn super::render_state::RenderStateTypes<
                HtmlElementProps = <HtmlElementProps::Data<
                    TypeDefs::HtmlElementProps,
                > as crate::imports::frender_csr::props::UpdateElement<
                    web_sys::HtmlElement,
                >>::State,
                auto_play = <TypeDefs::auto_play as ::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::State,
                controls = <TypeDefs::controls as ::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::State,
                cross_origin = <TypeDefs::cross_origin as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                r#loop = <TypeDefs::r#loop as ::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::State,
                muted = <TypeDefs::muted as ::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::State,
                preload = <TypeDefs::preload as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                src = <TypeDefs::src as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                on_abort = <TypeDefs::on_abort as crate::imports::frender_html::props::UpdateDomEventListener<
                    events::Abort,
                >>::State,
                on_can_play = <TypeDefs::on_can_play as crate::imports::frender_html::props::UpdateDomEventListener<
                    events::CanPlay,
                >>::State,
                on_can_play_through = <TypeDefs::on_can_play_through as crate::imports::frender_html::props::UpdateDomEventListener<
                    events::CanPlayThrough,
                >>::State,
                on_duration_change = <TypeDefs::on_duration_change as crate::imports::frender_html::props::UpdateDomEventListener<
                    events::DurationChange,
                >>::State,
                on_emptied = <TypeDefs::on_emptied as crate::imports::frender_html::props::UpdateDomEventListener<
                    events::Emptied,
                >>::State,
                on_ended = <TypeDefs::on_ended as crate::imports::frender_html::props::UpdateDomEventListener<
                    events::Ended,
                >>::State,
                on_loaded_data = <TypeDefs::on_loaded_data as crate::imports::frender_html::props::UpdateDomEventListener<
                    events::LoadedData,
                >>::State,
                on_loaded_metadata = <TypeDefs::on_loaded_metadata as crate::imports::frender_html::props::UpdateDomEventListener<
                    events::LoadedMetadata,
                >>::State,
                on_load_start = <TypeDefs::on_load_start as crate::imports::frender_html::props::UpdateDomEventListener<
                    events::LoadStart,
                >>::State,
                on_pause = <TypeDefs::on_pause as crate::imports::frender_html::props::UpdateDomEventListener<
                    events::Pause,
                >>::State,
                on_play = <TypeDefs::on_play as crate::imports::frender_html::props::UpdateDomEventListener<
                    events::Play,
                >>::State,
                on_playing = <TypeDefs::on_playing as crate::imports::frender_html::props::UpdateDomEventListener<
                    events::Playing,
                >>::State,
                on_progress = <TypeDefs::on_progress as crate::imports::frender_html::props::UpdateDomEventListener<
                    events::Progress,
                >>::State,
                on_rate_change = <TypeDefs::on_rate_change as crate::imports::frender_html::props::UpdateDomEventListener<
                    events::RateChange,
                >>::State,
                on_resize = <TypeDefs::on_resize as crate::imports::frender_html::props::UpdateDomEventListener<
                    events::Resize,
                >>::State,
                on_seeked = <TypeDefs::on_seeked as crate::imports::frender_html::props::UpdateDomEventListener<
                    events::Seeked,
                >>::State,
                on_seeking = <TypeDefs::on_seeking as crate::imports::frender_html::props::UpdateDomEventListener<
                    events::Seeking,
                >>::State,
                on_stalled = <TypeDefs::on_stalled as crate::imports::frender_html::props::UpdateDomEventListener<
                    events::Stalled,
                >>::State,
                on_suspend = <TypeDefs::on_suspend as crate::imports::frender_html::props::UpdateDomEventListener<
                    events::Suspend,
                >>::State,
                on_time_update = <TypeDefs::on_time_update as crate::imports::frender_html::props::UpdateDomEventListener<
                    events::TimeUpdate,
                >>::State,
                on_volume_change = <TypeDefs::on_volume_change as crate::imports::frender_html::props::UpdateDomEventListener<
                    events::VolumeChange,
                >>::State,
                on_waiting = <TypeDefs::on_waiting as crate::imports::frender_html::props::UpdateDomEventListener<
                    events::Waiting,
                >>::State,
            >,
        >;
        fn initialize_state(
            this: Self,
            element: &web_sys::HtmlMediaElement,
            children_ctx: &mut ::frender_csr::Dom,
        ) -> Self::State {
            let dom_element: &::web_sys::Element = element.as_ref();
            super::render_state::RenderState {
                HtmlElementProps: <HtmlElementProps::Data<
                    TypeDefs::HtmlElementProps,
                > as crate::imports::frender_csr::props::UpdateElement<
                    web_sys::HtmlElement,
                >>::initialize_state(this.HtmlElementProps, element, children_ctx),
                auto_play: <TypeDefs::auto_play as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.auto_play,
                    |v| element.set_autoplay(*v),
                    || dom_element.remove_attribute("autoplay").unwrap(),
                ),
                controls: <TypeDefs::controls as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.controls,
                    |v| element.set_controls(*v),
                    || dom_element.remove_attribute("controls").unwrap(),
                ),
                cross_origin: <TypeDefs::cross_origin as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.cross_origin,
                    match element {
                        el => |v: &_| el.set_cross_origin(Some(v)),
                    },
                    match element {
                        el => || el.set_cross_origin(None),
                    },
                ),
                r#loop: <TypeDefs::r#loop as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.r#loop,
                    |v| element.set_loop(*v),
                    || dom_element.remove_attribute("loop").unwrap(),
                ),
                muted: <TypeDefs::muted as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.muted,
                    |v| element.set_muted(*v),
                    || dom_element.remove_attribute("muted").unwrap(),
                ),
                preload: <TypeDefs::preload as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.preload,
                    |v| element.set_preload(v),
                    || dom_element.remove_attribute("preload").unwrap(),
                ),
                src: <TypeDefs::src as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.src,
                    |v| element.set_src(v),
                    || dom_element.remove_attribute("src").unwrap(),
                ),
                on_abort: crate::imports::frender_html::props::UpdateDomEventListener::<
                    events::Abort,
                >::initialize_dom_event_listener_state(this.on_abort, element),
                on_can_play: crate::imports::frender_html::props::UpdateDomEventListener::<
                    events::CanPlay,
                >::initialize_dom_event_listener_state(this.on_can_play, element),
                on_can_play_through: crate::imports::frender_html::props::UpdateDomEventListener::<
                    events::CanPlayThrough,
                >::initialize_dom_event_listener_state(
                    this.on_can_play_through,
                    element,
                ),
                on_duration_change: crate::imports::frender_html::props::UpdateDomEventListener::<
                    events::DurationChange,
                >::initialize_dom_event_listener_state(this.on_duration_change, element),
                on_emptied: crate::imports::frender_html::props::UpdateDomEventListener::<
                    events::Emptied,
                >::initialize_dom_event_listener_state(this.on_emptied, element),
                on_ended: crate::imports::frender_html::props::UpdateDomEventListener::<
                    events::Ended,
                >::initialize_dom_event_listener_state(this.on_ended, element),
                on_loaded_data: crate::imports::frender_html::props::UpdateDomEventListener::<
                    events::LoadedData,
                >::initialize_dom_event_listener_state(this.on_loaded_data, element),
                on_loaded_metadata: crate::imports::frender_html::props::UpdateDomEventListener::<
                    events::LoadedMetadata,
                >::initialize_dom_event_listener_state(this.on_loaded_metadata, element),
                on_load_start: crate::imports::frender_html::props::UpdateDomEventListener::<
                    events::LoadStart,
                >::initialize_dom_event_listener_state(this.on_load_start, element),
                on_pause: crate::imports::frender_html::props::UpdateDomEventListener::<
                    events::Pause,
                >::initialize_dom_event_listener_state(this.on_pause, element),
                on_play: crate::imports::frender_html::props::UpdateDomEventListener::<
                    events::Play,
                >::initialize_dom_event_listener_state(this.on_play, element),
                on_playing: crate::imports::frender_html::props::UpdateDomEventListener::<
                    events::Playing,
                >::initialize_dom_event_listener_state(this.on_playing, element),
                on_progress: crate::imports::frender_html::props::UpdateDomEventListener::<
                    events::Progress,
                >::initialize_dom_event_listener_state(this.on_progress, element),
                on_rate_change: crate::imports::frender_html::props::UpdateDomEventListener::<
                    events::RateChange,
                >::initialize_dom_event_listener_state(this.on_rate_change, element),
                on_resize: crate::imports::frender_html::props::UpdateDomEventListener::<
                    events::Resize,
                >::initialize_dom_event_listener_state(this.on_resize, element),
                on_seeked: crate::imports::frender_html::props::UpdateDomEventListener::<
                    events::Seeked,
                >::initialize_dom_event_listener_state(this.on_seeked, element),
                on_seeking: crate::imports::frender_html::props::UpdateDomEventListener::<
                    events::Seeking,
                >::initialize_dom_event_listener_state(this.on_seeking, element),
                on_stalled: crate::imports::frender_html::props::UpdateDomEventListener::<
                    events::Stalled,
                >::initialize_dom_event_listener_state(this.on_stalled, element),
                on_suspend: crate::imports::frender_html::props::UpdateDomEventListener::<
                    events::Suspend,
                >::initialize_dom_event_listener_state(this.on_suspend, element),
                on_time_update: crate::imports::frender_html::props::UpdateDomEventListener::<
                    events::TimeUpdate,
                >::initialize_dom_event_listener_state(this.on_time_update, element),
                on_volume_change: crate::imports::frender_html::props::UpdateDomEventListener::<
                    events::VolumeChange,
                >::initialize_dom_event_listener_state(this.on_volume_change, element),
                on_waiting: crate::imports::frender_html::props::UpdateDomEventListener::<
                    events::Waiting,
                >::initialize_dom_event_listener_state(this.on_waiting, element),
            }
        }
        fn update_element(
            this: Self,
            element: &web_sys::HtmlMediaElement,
            children_ctx: &mut ::frender_csr::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let state = state.pin_project();
            let dom_element: &::web_sys::Element = element.as_ref();
            crate::imports::frender_csr::props::UpdateElement::update_element(
                this.HtmlElementProps,
                element.as_ref(),
                children_ctx,
                state.HtmlElementProps,
            );
            <TypeDefs::auto_play as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                bool,
            >>::maybe_update_value_with_state(
                this.auto_play,
                state.auto_play,
                |v| element.set_autoplay(*v),
                || dom_element.remove_attribute("autoplay").unwrap(),
            );
            <TypeDefs::controls as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                bool,
            >>::maybe_update_value_with_state(
                this.controls,
                state.controls,
                |v| element.set_controls(*v),
                || dom_element.remove_attribute("controls").unwrap(),
            );
            <TypeDefs::cross_origin as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.cross_origin,
                state.cross_origin,
                match element {
                    el => |v: &_| el.set_cross_origin(Some(v)),
                },
                match element {
                    el => || el.set_cross_origin(None),
                },
            );
            <TypeDefs::r#loop as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                bool,
            >>::maybe_update_value_with_state(
                this.r#loop,
                state.r#loop,
                |v| element.set_loop(*v),
                || dom_element.remove_attribute("loop").unwrap(),
            );
            <TypeDefs::muted as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                bool,
            >>::maybe_update_value_with_state(
                this.muted,
                state.muted,
                |v| element.set_muted(*v),
                || dom_element.remove_attribute("muted").unwrap(),
            );
            <TypeDefs::preload as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.preload,
                state.preload,
                |v| element.set_preload(v),
                || dom_element.remove_attribute("preload").unwrap(),
            );
            <TypeDefs::src as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.src,
                state.src,
                |v| element.set_src(v),
                || dom_element.remove_attribute("src").unwrap(),
            );
            crate::imports::frender_html::props::UpdateDomEventListener::<
                events::Abort,
            >::update_dom_event_listener(this.on_abort, element, state.on_abort);
            crate::imports::frender_html::props::UpdateDomEventListener::<
                events::CanPlay,
            >::update_dom_event_listener(this.on_can_play, element, state.on_can_play);
            crate::imports::frender_html::props::UpdateDomEventListener::<
                events::CanPlayThrough,
            >::update_dom_event_listener(
                this.on_can_play_through,
                element,
                state.on_can_play_through,
            );
            crate::imports::frender_html::props::UpdateDomEventListener::<
                events::DurationChange,
            >::update_dom_event_listener(
                this.on_duration_change,
                element,
                state.on_duration_change,
            );
            crate::imports::frender_html::props::UpdateDomEventListener::<
                events::Emptied,
            >::update_dom_event_listener(this.on_emptied, element, state.on_emptied);
            crate::imports::frender_html::props::UpdateDomEventListener::<
                events::Ended,
            >::update_dom_event_listener(this.on_ended, element, state.on_ended);
            crate::imports::frender_html::props::UpdateDomEventListener::<
                events::LoadedData,
            >::update_dom_event_listener(
                this.on_loaded_data,
                element,
                state.on_loaded_data,
            );
            crate::imports::frender_html::props::UpdateDomEventListener::<
                events::LoadedMetadata,
            >::update_dom_event_listener(
                this.on_loaded_metadata,
                element,
                state.on_loaded_metadata,
            );
            crate::imports::frender_html::props::UpdateDomEventListener::<
                events::LoadStart,
            >::update_dom_event_listener(
                this.on_load_start,
                element,
                state.on_load_start,
            );
            crate::imports::frender_html::props::UpdateDomEventListener::<
                events::Pause,
            >::update_dom_event_listener(this.on_pause, element, state.on_pause);
            crate::imports::frender_html::props::UpdateDomEventListener::<
                events::Play,
            >::update_dom_event_listener(this.on_play, element, state.on_play);
            crate::imports::frender_html::props::UpdateDomEventListener::<
                events::Playing,
            >::update_dom_event_listener(this.on_playing, element, state.on_playing);
            crate::imports::frender_html::props::UpdateDomEventListener::<
                events::Progress,
            >::update_dom_event_listener(this.on_progress, element, state.on_progress);
            crate::imports::frender_html::props::UpdateDomEventListener::<
                events::RateChange,
            >::update_dom_event_listener(
                this.on_rate_change,
                element,
                state.on_rate_change,
            );
            crate::imports::frender_html::props::UpdateDomEventListener::<
                events::Resize,
            >::update_dom_event_listener(this.on_resize, element, state.on_resize);
            crate::imports::frender_html::props::UpdateDomEventListener::<
                events::Seeked,
            >::update_dom_event_listener(this.on_seeked, element, state.on_seeked);
            crate::imports::frender_html::props::UpdateDomEventListener::<
                events::Seeking,
            >::update_dom_event_listener(this.on_seeking, element, state.on_seeking);
            crate::imports::frender_html::props::UpdateDomEventListener::<
                events::Stalled,
            >::update_dom_event_listener(this.on_stalled, element, state.on_stalled);
            crate::imports::frender_html::props::UpdateDomEventListener::<
                events::Suspend,
            >::update_dom_event_listener(this.on_suspend, element, state.on_suspend);
            crate::imports::frender_html::props::UpdateDomEventListener::<
                events::TimeUpdate,
            >::update_dom_event_listener(
                this.on_time_update,
                element,
                state.on_time_update,
            );
            crate::imports::frender_html::props::UpdateDomEventListener::<
                events::VolumeChange,
            >::update_dom_event_listener(
                this.on_volume_change,
                element,
                state.on_volume_change,
            );
            crate::imports::frender_html::props::UpdateDomEventListener::<
                events::Waiting,
            >::update_dom_event_listener(this.on_waiting, element, state.on_waiting);
        }
    }
}
#[cfg(feature = "ssr")]
mod impl_into_ssr_data {
    #[allow(unused_imports)]
    use super::super::*;
    impl<
            TypeDefs: ?::core::marker::Sized + super::Types,
            W: ::frender_ssr::AsyncWrite + ::core::marker::Unpin,
        > ::frender_ssr::IntoSsrData<W> for super::Data<TypeDefs>
    where
        HtmlElementProps::Data<TypeDefs::HtmlElementProps>: ::frender_ssr::IntoSsrData<W>,
    {
        type Children = <HtmlElementProps::Data<
            TypeDefs::HtmlElementProps,
        > as ::frender_ssr::IntoSsrData<W>>::Children;
        type ChildrenRenderState = <HtmlElementProps::Data<
            TypeDefs::HtmlElementProps,
        > as ::frender_ssr::IntoSsrData<W>>::ChildrenRenderState;
        type Attrs = ::core::iter::Chain<
            <HtmlElementProps::Data<
                TypeDefs::HtmlElementProps,
            > as ::frender_ssr::IntoSsrData<W>>::Attrs,
            ::frender_ssr::utils::filter::FilterArray<
                ::frender_ssr::element::html::HtmlAttrPair<'static>,
                7usize,
            >,
        >;
        fn into_ssr_data(this: Self) -> (Self::Children, Self::Attrs) {
            let (children, attrs) =
                ::frender_ssr::IntoSsrData::into_ssr_data(this.HtmlElementProps);
            (
                children,
                attrs
                    .chain(
                        ::frender_ssr::utils::filter::FilterIdentity(
                            [
                                <TypeDefs::auto_play as ::frender_html::props::MaybeUpdateValueWithState<
                                    bool,
                                >>::maybe_into_html_attribute_value(this.auto_play)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("autoplay"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::controls as ::frender_html::props::MaybeUpdateValueWithState<
                                    bool,
                                >>::maybe_into_html_attribute_value(this.controls)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("controls"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::cross_origin as ::frender_html::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.cross_origin)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("crossorigin"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::r#loop as ::frender_html::props::MaybeUpdateValueWithState<
                                    bool,
                                >>::maybe_into_html_attribute_value(this.r#loop)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("loop"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::muted as ::frender_html::props::MaybeUpdateValueWithState<
                                    bool,
                                >>::maybe_into_html_attribute_value(this.muted)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("muted"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::preload as ::frender_html::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.preload)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("preload"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::src as ::frender_html::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.src)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("src"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                            ]
                                .into_iter(),
                        ),
                    ),
            )
        }
    }
}
