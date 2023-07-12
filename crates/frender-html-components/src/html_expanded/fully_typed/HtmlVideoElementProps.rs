#[allow(non_snake_case)]
#[inline(always)]
pub fn HtmlVideoElementProps() -> Building<TypesInitial> {
    #[allow(unused_imports)]
    use super::*;
    self::Building {
        HtmlMediaElementProps: HtmlMediaElementProps::build(HtmlMediaElementProps()),
        height: (),
        plays_inline: (),
        poster: (),
        width: (),
    }
}
pub mod prelude {}
pub mod overwrite {
    #![allow(non_camel_case_types)]
    pub type HtmlMediaElementProps<TypeDefs, Value> = dyn super::Types<
        HtmlMediaElementProps = Value,
        height = <TypeDefs as super::Types>::height,
        plays_inline = <TypeDefs as super::Types>::plays_inline,
        poster = <TypeDefs as super::Types>::poster,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type HtmlElementProps<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::HtmlElementProps<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type ElementProps<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::ElementProps<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type children<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::children<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type class<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::class<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type id<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::id<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type part<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::part<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_cancel<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_cancel<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_error<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_error<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_scroll<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_scroll<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_security_policy_violation<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_security_policy_violation<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_select<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_select<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_wheel<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_wheel<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_copy<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_copy<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_cut<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_cut<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_paste<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_paste<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_composition_end<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_composition_end<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_composition_start<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_composition_start<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_composition_update<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_composition_update<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_blur<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_blur<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_focus<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_focus<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_focus_in<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_focus_in<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_focus_out<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_focus_out<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_fullscreen_change<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_fullscreen_change<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_fullscreen_error<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_fullscreen_error<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_key_down<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_key_down<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_key_up<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_key_up<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_aux_click<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_aux_click<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_click<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_click<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_context_menu<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_context_menu<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_double_click<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_double_click<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_mouse_down<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_mouse_down<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_mouse_enter<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_mouse_enter<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_mouse_leave<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_mouse_leave<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_mouse_move<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_mouse_move<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_mouse_out<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_mouse_out<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_mouse_over<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_mouse_over<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_mouse_up<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_mouse_up<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_touch_cancel<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_touch_cancel<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_touch_end<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_touch_end<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_touch_move<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_touch_move<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_touch_start<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_touch_start<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type access_key<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::access_key<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type auto_capitalize<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::auto_capitalize<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type auto_focus<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::auto_focus<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type content_editable<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::content_editable<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type context_menu<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::context_menu<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type dir<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::dir<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type draggable<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::draggable<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type enter_key_hint<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::enter_key_hint<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type hidden<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::hidden<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type inert<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::inert<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type input_mode<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::input_mode<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type is<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::is<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type item_id<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::item_id<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type item_prop<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::item_prop<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type item_ref<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::item_ref<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type item_scope<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::item_scope<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type item_type<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::item_type<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type lang<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::lang<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type nonce<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::nonce<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type role<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::role<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type slot<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::slot<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type spellcheck<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::spellcheck<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type style<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::style<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type tab_index<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::tab_index<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type title<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::title<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type translate<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::translate<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type virtual_keyboard_policy<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::virtual_keyboard_policy<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_invalid<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_invalid<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_animation_cancel<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_animation_cancel<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_animation_end<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_animation_end<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_animation_iteration<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_animation_iteration<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_animation_start<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_animation_start<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_before_input<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_before_input<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_input<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_input<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_change<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_change<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_got_pointer_capture<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_got_pointer_capture<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_lost_pointer_capture<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_lost_pointer_capture<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_pointer_cancel<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_pointer_cancel<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_pointer_down<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_pointer_down<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_pointer_enter<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_pointer_enter<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_pointer_leave<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_pointer_leave<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_pointer_move<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_pointer_move<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_pointer_out<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_pointer_out<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_pointer_over<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_pointer_over<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_pointer_up<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_pointer_up<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_transition_cancel<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_transition_cancel<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_transition_end<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_transition_end<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_transition_run<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_transition_run<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_transition_start<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_transition_start<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_drag<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_drag<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_drag_end<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_drag_end<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_drag_enter<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_drag_enter<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_drag_leave<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_drag_leave<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_drag_over<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_drag_over<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_drag_start<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_drag_start<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_drop<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_drop<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type auto_play<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::auto_play<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type controls<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::controls<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type cross_origin<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::cross_origin<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type r#loop<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::r#loop<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type muted<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::muted<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type preload<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::preload<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type src<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::src<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_abort<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_abort<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_can_play<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_can_play<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_can_play_through<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_can_play_through<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_duration_change<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_duration_change<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_emptied<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_emptied<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_ended<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_ended<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_loaded_data<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_loaded_data<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_loaded_metadata<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_loaded_metadata<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_load_start<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_load_start<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_pause<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_pause<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_play<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_play<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_playing<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_playing<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_progress<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_progress<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_rate_change<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_rate_change<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_resize<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_resize<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_seeked<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_seeked<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_seeking<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_seeking<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_stalled<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_stalled<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_suspend<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_suspend<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_time_update<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_time_update<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_volume_change<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_volume_change<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_waiting<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_waiting<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type height<TypeDefs, Value> = dyn super::Types<
        HtmlMediaElementProps = <TypeDefs as super::Types>::HtmlMediaElementProps,
        height = Value,
        plays_inline = <TypeDefs as super::Types>::plays_inline,
        poster = <TypeDefs as super::Types>::poster,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type plays_inline<TypeDefs, Value> = dyn super::Types<
        HtmlMediaElementProps = <TypeDefs as super::Types>::HtmlMediaElementProps,
        height = <TypeDefs as super::Types>::height,
        plays_inline = Value,
        poster = <TypeDefs as super::Types>::poster,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type poster<TypeDefs, Value> = dyn super::Types<
        HtmlMediaElementProps = <TypeDefs as super::Types>::HtmlMediaElementProps,
        height = <TypeDefs as super::Types>::height,
        plays_inline = <TypeDefs as super::Types>::plays_inline,
        poster = Value,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type width<TypeDefs, Value> = dyn super::Types<
        HtmlMediaElementProps = <TypeDefs as super::Types>::HtmlMediaElementProps,
        height = <TypeDefs as super::Types>::height,
        plays_inline = <TypeDefs as super::Types>::plays_inline,
        poster = <TypeDefs as super::Types>::poster,
        width = Value,
    >;
}
mod trait_types {
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub trait Types {
        type HtmlMediaElementProps: ?::core::marker::Sized + HtmlMediaElementProps::Types;
        type height: crate::imports::frender_html::props::MaybeUpdateValueWithState<u32>;
        type plays_inline: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>;
        type poster: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>;
        type width: crate::imports::frender_html::props::MaybeUpdateValueWithState<u32>;
    }
}
pub use trait_types::Types;
pub use trait_types::Types as ValidTypes;
pub mod data_struct {
    #[non_exhaustive]
    pub struct HtmlVideoElementProps<TypeDefs: super::Types + ?::core::marker::Sized> {
        pub HtmlMediaElementProps:
            super::super::HtmlMediaElementProps::Data<TypeDefs::HtmlMediaElementProps>,
        pub height: TypeDefs::height,
        pub plays_inline: TypeDefs::plays_inline,
        pub poster: TypeDefs::poster,
        pub width: TypeDefs::width,
    }
}
pub use ::core::convert::identity as Building;
pub use ::core::convert::identity as build;
pub use data_struct::HtmlVideoElementProps as Data;
pub use data_struct::HtmlVideoElementProps as Building;
pub struct Replacing<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
mod types_initial {
    #[allow(unused_imports)]
    use super::super::*;
    pub type TypesInitial = dyn super::Types<
        HtmlMediaElementProps = HtmlMediaElementProps::TypesInitial,
        height = (),
        plays_inline = (),
        poster = (),
        width = (),
    >;
}
pub use types_initial::TypesInitial;
pub type DataInitial = Data<TypesInitial>;
#[cfg(feature = "csr")]
pub mod render_state {
    #[allow(non_camel_case_types)]
    pub trait RenderStateTypes {
        type HtmlMediaElementProps: crate::imports::frender_csr::props::IntrinsicComponentPollReactive;
        type height;
        type plays_inline;
        type poster;
        type width;
    }
    crate::imports::pin_project! {
        #[project = RenderStateProj] pub struct RenderState < TypeDefs : RenderStateTypes
        > where TypeDefs : ? ::core::marker::Sized { #[pin] pub HtmlMediaElementProps :
        TypeDefs::HtmlMediaElementProps, pub height : TypeDefs::height, pub plays_inline
        : TypeDefs::plays_inline, pub poster : TypeDefs::poster, pub width :
        TypeDefs::width, }
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
                self.project().HtmlMediaElementProps,
                cx,
            )
        }
    }
}
mod builder_and_replacer {
    #[allow(unused_imports)]
    use super::super::*;
    impl<TypeDefs: super::Types + ?::core::marker::Sized> super::Building<TypeDefs> {
        ///See [`HtmlMediaElementProps::children`]
        #[inline(always)]
        pub fn children<V>(
            self,
            children: V,
        ) -> super::Building<super::overwrite::children<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.children(children),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::class`]
        #[inline(always)]
        pub fn class<V: Todo<unimplemented![]>>(
            self,
            class: V,
        ) -> super::Building<super::overwrite::class<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.class(class),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::id`]
        #[inline(always)]
        pub fn id<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            id: V,
        ) -> super::Building<super::overwrite::id<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.id(id),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::part`]
        #[inline(always)]
        pub fn part<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            part: V,
        ) -> super::Building<super::overwrite::part<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.part(part),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_cancel`]
        #[inline(always)]
        pub fn on_cancel<V>(
            self,
            on_cancel: V,
        ) -> super::Building<super::overwrite::on_cancel<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_cancel(on_cancel),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_error`]
        #[inline(always)]
        pub fn on_error<V>(
            self,
            on_error: V,
        ) -> super::Building<super::overwrite::on_error<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_error(on_error),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_scroll`]
        #[inline(always)]
        pub fn on_scroll<V>(
            self,
            on_scroll: V,
        ) -> super::Building<super::overwrite::on_scroll<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_scroll(on_scroll),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_security_policy_violation`]
        #[inline(always)]
        pub fn on_security_policy_violation<V>(
            self,
            on_security_policy_violation: V,
        ) -> super::Building<super::overwrite::on_security_policy_violation<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self
                    .HtmlMediaElementProps
                    .on_security_policy_violation(on_security_policy_violation),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_select`]
        #[inline(always)]
        pub fn on_select<V>(
            self,
            on_select: V,
        ) -> super::Building<super::overwrite::on_select<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_select(on_select),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_wheel`]
        #[inline(always)]
        pub fn on_wheel<V>(
            self,
            on_wheel: V,
        ) -> super::Building<super::overwrite::on_wheel<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_wheel(on_wheel),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_copy`]
        #[inline(always)]
        pub fn on_copy<V>(
            self,
            on_copy: V,
        ) -> super::Building<super::overwrite::on_copy<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_copy(on_copy),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_cut`]
        #[inline(always)]
        pub fn on_cut<V>(
            self,
            on_cut: V,
        ) -> super::Building<super::overwrite::on_cut<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_cut(on_cut),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_paste`]
        #[inline(always)]
        pub fn on_paste<V>(
            self,
            on_paste: V,
        ) -> super::Building<super::overwrite::on_paste<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_paste(on_paste),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_composition_end`]
        #[inline(always)]
        pub fn on_composition_end<V>(
            self,
            on_composition_end: V,
        ) -> super::Building<super::overwrite::on_composition_end<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self
                    .HtmlMediaElementProps
                    .on_composition_end(on_composition_end),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_composition_start`]
        #[inline(always)]
        pub fn on_composition_start<V>(
            self,
            on_composition_start: V,
        ) -> super::Building<super::overwrite::on_composition_start<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self
                    .HtmlMediaElementProps
                    .on_composition_start(on_composition_start),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_composition_update`]
        #[inline(always)]
        pub fn on_composition_update<V>(
            self,
            on_composition_update: V,
        ) -> super::Building<super::overwrite::on_composition_update<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self
                    .HtmlMediaElementProps
                    .on_composition_update(on_composition_update),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_blur`]
        #[inline(always)]
        pub fn on_blur<V>(
            self,
            on_blur: V,
        ) -> super::Building<super::overwrite::on_blur<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_blur(on_blur),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_focus`]
        #[inline(always)]
        pub fn on_focus<V>(
            self,
            on_focus: V,
        ) -> super::Building<super::overwrite::on_focus<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_focus(on_focus),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_focus_in`]
        #[inline(always)]
        pub fn on_focus_in<V>(
            self,
            on_focus_in: V,
        ) -> super::Building<super::overwrite::on_focus_in<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_focus_in(on_focus_in),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_focus_out`]
        #[inline(always)]
        pub fn on_focus_out<V>(
            self,
            on_focus_out: V,
        ) -> super::Building<super::overwrite::on_focus_out<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_focus_out(on_focus_out),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_fullscreen_change`]
        #[inline(always)]
        pub fn on_fullscreen_change<V>(
            self,
            on_fullscreen_change: V,
        ) -> super::Building<super::overwrite::on_fullscreen_change<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self
                    .HtmlMediaElementProps
                    .on_fullscreen_change(on_fullscreen_change),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_fullscreen_error`]
        #[inline(always)]
        pub fn on_fullscreen_error<V>(
            self,
            on_fullscreen_error: V,
        ) -> super::Building<super::overwrite::on_fullscreen_error<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self
                    .HtmlMediaElementProps
                    .on_fullscreen_error(on_fullscreen_error),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_key_down`]
        #[inline(always)]
        pub fn on_key_down<V>(
            self,
            on_key_down: V,
        ) -> super::Building<super::overwrite::on_key_down<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_key_down(on_key_down),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_key_up`]
        #[inline(always)]
        pub fn on_key_up<V>(
            self,
            on_key_up: V,
        ) -> super::Building<super::overwrite::on_key_up<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_key_up(on_key_up),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_aux_click`]
        #[inline(always)]
        pub fn on_aux_click<V>(
            self,
            on_aux_click: V,
        ) -> super::Building<super::overwrite::on_aux_click<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_aux_click(on_aux_click),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_click`]
        #[inline(always)]
        pub fn on_click<V>(
            self,
            on_click: V,
        ) -> super::Building<super::overwrite::on_click<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_click(on_click),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_context_menu`]
        #[inline(always)]
        pub fn on_context_menu<V>(
            self,
            on_context_menu: V,
        ) -> super::Building<super::overwrite::on_context_menu<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_context_menu(on_context_menu),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_double_click`]
        #[inline(always)]
        pub fn on_double_click<V>(
            self,
            on_double_click: V,
        ) -> super::Building<super::overwrite::on_double_click<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_double_click(on_double_click),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_mouse_down`]
        #[inline(always)]
        pub fn on_mouse_down<V>(
            self,
            on_mouse_down: V,
        ) -> super::Building<super::overwrite::on_mouse_down<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_mouse_down(on_mouse_down),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_mouse_enter`]
        #[inline(always)]
        pub fn on_mouse_enter<V>(
            self,
            on_mouse_enter: V,
        ) -> super::Building<super::overwrite::on_mouse_enter<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_mouse_enter(on_mouse_enter),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_mouse_leave`]
        #[inline(always)]
        pub fn on_mouse_leave<V>(
            self,
            on_mouse_leave: V,
        ) -> super::Building<super::overwrite::on_mouse_leave<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_mouse_leave(on_mouse_leave),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_mouse_move`]
        #[inline(always)]
        pub fn on_mouse_move<V>(
            self,
            on_mouse_move: V,
        ) -> super::Building<super::overwrite::on_mouse_move<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_mouse_move(on_mouse_move),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_mouse_out`]
        #[inline(always)]
        pub fn on_mouse_out<V>(
            self,
            on_mouse_out: V,
        ) -> super::Building<super::overwrite::on_mouse_out<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_mouse_out(on_mouse_out),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_mouse_over`]
        #[inline(always)]
        pub fn on_mouse_over<V>(
            self,
            on_mouse_over: V,
        ) -> super::Building<super::overwrite::on_mouse_over<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_mouse_over(on_mouse_over),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_mouse_up`]
        #[inline(always)]
        pub fn on_mouse_up<V>(
            self,
            on_mouse_up: V,
        ) -> super::Building<super::overwrite::on_mouse_up<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_mouse_up(on_mouse_up),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_touch_cancel`]
        #[inline(always)]
        pub fn on_touch_cancel<V>(
            self,
            on_touch_cancel: V,
        ) -> super::Building<super::overwrite::on_touch_cancel<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_touch_cancel(on_touch_cancel),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_touch_end`]
        #[inline(always)]
        pub fn on_touch_end<V>(
            self,
            on_touch_end: V,
        ) -> super::Building<super::overwrite::on_touch_end<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_touch_end(on_touch_end),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_touch_move`]
        #[inline(always)]
        pub fn on_touch_move<V>(
            self,
            on_touch_move: V,
        ) -> super::Building<super::overwrite::on_touch_move<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_touch_move(on_touch_move),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_touch_start`]
        #[inline(always)]
        pub fn on_touch_start<V>(
            self,
            on_touch_start: V,
        ) -> super::Building<super::overwrite::on_touch_start<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_touch_start(on_touch_start),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::access_key`]
        #[inline(always)]
        pub fn access_key<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
        >(
            self,
            access_key: V,
        ) -> super::Building<super::overwrite::access_key<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.access_key(access_key),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::auto_capitalize`]
        #[inline(always)]
        pub fn auto_capitalize<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
        >(
            self,
            auto_capitalize: V,
        ) -> super::Building<super::overwrite::auto_capitalize<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.auto_capitalize(auto_capitalize),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::auto_focus`]
        #[inline(always)]
        pub fn auto_focus<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
        >(
            self,
            auto_focus: V,
        ) -> super::Building<super::overwrite::auto_focus<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.auto_focus(auto_focus),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::content_editable`]
        #[inline(always)]
        pub fn content_editable<V: Todo<unimplemented![]>>(
            self,
            content_editable: V,
        ) -> super::Building<super::overwrite::content_editable<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self
                    .HtmlMediaElementProps
                    .content_editable(content_editable),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::context_menu`]
        #[inline(always)]
        pub fn context_menu<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
        >(
            self,
            context_menu: V,
        ) -> super::Building<super::overwrite::context_menu<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.context_menu(context_menu),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::dir`]
        #[inline(always)]
        pub fn dir<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            dir: V,
        ) -> super::Building<super::overwrite::dir<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.dir(dir),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::draggable`]
        #[inline(always)]
        pub fn draggable<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
        >(
            self,
            draggable: V,
        ) -> super::Building<super::overwrite::draggable<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.draggable(draggable),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::enter_key_hint`]
        #[inline(always)]
        pub fn enter_key_hint<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
        >(
            self,
            enter_key_hint: V,
        ) -> super::Building<super::overwrite::enter_key_hint<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.enter_key_hint(enter_key_hint),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::hidden`]
        #[inline(always)]
        pub fn hidden<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>>(
            self,
            hidden: V,
        ) -> super::Building<super::overwrite::hidden<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.hidden(hidden),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::inert`]
        #[inline(always)]
        pub fn inert<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>>(
            self,
            inert: V,
        ) -> super::Building<super::overwrite::inert<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.inert(inert),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::input_mode`]
        #[inline(always)]
        pub fn input_mode<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
        >(
            self,
            input_mode: V,
        ) -> super::Building<super::overwrite::input_mode<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.input_mode(input_mode),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::is`]
        #[inline(always)]
        pub fn is<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            is: V,
        ) -> super::Building<super::overwrite::is<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.is(is),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::item_id`]
        #[inline(always)]
        pub fn item_id<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            item_id: V,
        ) -> super::Building<super::overwrite::item_id<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.item_id(item_id),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::item_prop`]
        #[inline(always)]
        pub fn item_prop<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            item_prop: V,
        ) -> super::Building<super::overwrite::item_prop<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.item_prop(item_prop),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::item_ref`]
        #[inline(always)]
        pub fn item_ref<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            item_ref: V,
        ) -> super::Building<super::overwrite::item_ref<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.item_ref(item_ref),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::item_scope`]
        #[inline(always)]
        pub fn item_scope<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
        >(
            self,
            item_scope: V,
        ) -> super::Building<super::overwrite::item_scope<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.item_scope(item_scope),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::item_type`]
        #[inline(always)]
        pub fn item_type<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            item_type: V,
        ) -> super::Building<super::overwrite::item_type<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.item_type(item_type),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::lang`]
        #[inline(always)]
        pub fn lang<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            lang: V,
        ) -> super::Building<super::overwrite::lang<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.lang(lang),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::nonce`]
        #[inline(always)]
        pub fn nonce<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            nonce: V,
        ) -> super::Building<super::overwrite::nonce<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.nonce(nonce),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::role`]
        #[inline(always)]
        pub fn role<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            role: V,
        ) -> super::Building<super::overwrite::role<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.role(role),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::slot`]
        #[inline(always)]
        pub fn slot<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            slot: V,
        ) -> super::Building<super::overwrite::slot<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.slot(slot),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::spellcheck`]
        #[inline(always)]
        pub fn spellcheck<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
        >(
            self,
            spellcheck: V,
        ) -> super::Building<super::overwrite::spellcheck<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.spellcheck(spellcheck),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::style`]
        #[inline(always)]
        pub fn style<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            style: V,
        ) -> super::Building<super::overwrite::style<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.style(style),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::tab_index`]
        #[inline(always)]
        pub fn tab_index<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<i32>>(
            self,
            tab_index: V,
        ) -> super::Building<super::overwrite::tab_index<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.tab_index(tab_index),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::title`]
        #[inline(always)]
        pub fn title<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            title: V,
        ) -> super::Building<super::overwrite::title<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.title(title),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::translate`]
        #[inline(always)]
        pub fn translate<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            translate: V,
        ) -> super::Building<super::overwrite::translate<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.translate(translate),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::virtual_keyboard_policy`]
        #[inline(always)]
        pub fn virtual_keyboard_policy<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
        >(
            self,
            virtual_keyboard_policy: V,
        ) -> super::Building<super::overwrite::virtual_keyboard_policy<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self
                    .HtmlMediaElementProps
                    .virtual_keyboard_policy(virtual_keyboard_policy),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_invalid`]
        #[inline(always)]
        pub fn on_invalid<V>(
            self,
            on_invalid: V,
        ) -> super::Building<super::overwrite::on_invalid<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_invalid(on_invalid),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_animation_cancel`]
        #[inline(always)]
        pub fn on_animation_cancel<V>(
            self,
            on_animation_cancel: V,
        ) -> super::Building<super::overwrite::on_animation_cancel<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self
                    .HtmlMediaElementProps
                    .on_animation_cancel(on_animation_cancel),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_animation_end`]
        #[inline(always)]
        pub fn on_animation_end<V>(
            self,
            on_animation_end: V,
        ) -> super::Building<super::overwrite::on_animation_end<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self
                    .HtmlMediaElementProps
                    .on_animation_end(on_animation_end),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_animation_iteration`]
        #[inline(always)]
        pub fn on_animation_iteration<V>(
            self,
            on_animation_iteration: V,
        ) -> super::Building<super::overwrite::on_animation_iteration<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self
                    .HtmlMediaElementProps
                    .on_animation_iteration(on_animation_iteration),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_animation_start`]
        #[inline(always)]
        pub fn on_animation_start<V>(
            self,
            on_animation_start: V,
        ) -> super::Building<super::overwrite::on_animation_start<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self
                    .HtmlMediaElementProps
                    .on_animation_start(on_animation_start),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_before_input`]
        #[inline(always)]
        pub fn on_before_input<V>(
            self,
            on_before_input: V,
        ) -> super::Building<super::overwrite::on_before_input<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_before_input(on_before_input),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_input`]
        #[inline(always)]
        pub fn on_input<V>(
            self,
            on_input: V,
        ) -> super::Building<super::overwrite::on_input<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_input(on_input),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_change`]
        #[inline(always)]
        pub fn on_change<V>(
            self,
            on_change: V,
        ) -> super::Building<super::overwrite::on_change<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_change(on_change),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_got_pointer_capture`]
        #[inline(always)]
        pub fn on_got_pointer_capture<V>(
            self,
            on_got_pointer_capture: V,
        ) -> super::Building<super::overwrite::on_got_pointer_capture<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self
                    .HtmlMediaElementProps
                    .on_got_pointer_capture(on_got_pointer_capture),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_lost_pointer_capture`]
        #[inline(always)]
        pub fn on_lost_pointer_capture<V>(
            self,
            on_lost_pointer_capture: V,
        ) -> super::Building<super::overwrite::on_lost_pointer_capture<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self
                    .HtmlMediaElementProps
                    .on_lost_pointer_capture(on_lost_pointer_capture),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_pointer_cancel`]
        #[inline(always)]
        pub fn on_pointer_cancel<V>(
            self,
            on_pointer_cancel: V,
        ) -> super::Building<super::overwrite::on_pointer_cancel<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self
                    .HtmlMediaElementProps
                    .on_pointer_cancel(on_pointer_cancel),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_pointer_down`]
        #[inline(always)]
        pub fn on_pointer_down<V>(
            self,
            on_pointer_down: V,
        ) -> super::Building<super::overwrite::on_pointer_down<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_pointer_down(on_pointer_down),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_pointer_enter`]
        #[inline(always)]
        pub fn on_pointer_enter<V>(
            self,
            on_pointer_enter: V,
        ) -> super::Building<super::overwrite::on_pointer_enter<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self
                    .HtmlMediaElementProps
                    .on_pointer_enter(on_pointer_enter),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_pointer_leave`]
        #[inline(always)]
        pub fn on_pointer_leave<V>(
            self,
            on_pointer_leave: V,
        ) -> super::Building<super::overwrite::on_pointer_leave<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self
                    .HtmlMediaElementProps
                    .on_pointer_leave(on_pointer_leave),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_pointer_move`]
        #[inline(always)]
        pub fn on_pointer_move<V>(
            self,
            on_pointer_move: V,
        ) -> super::Building<super::overwrite::on_pointer_move<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_pointer_move(on_pointer_move),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_pointer_out`]
        #[inline(always)]
        pub fn on_pointer_out<V>(
            self,
            on_pointer_out: V,
        ) -> super::Building<super::overwrite::on_pointer_out<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_pointer_out(on_pointer_out),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_pointer_over`]
        #[inline(always)]
        pub fn on_pointer_over<V>(
            self,
            on_pointer_over: V,
        ) -> super::Building<super::overwrite::on_pointer_over<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_pointer_over(on_pointer_over),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_pointer_up`]
        #[inline(always)]
        pub fn on_pointer_up<V>(
            self,
            on_pointer_up: V,
        ) -> super::Building<super::overwrite::on_pointer_up<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_pointer_up(on_pointer_up),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_transition_cancel`]
        #[inline(always)]
        pub fn on_transition_cancel<V>(
            self,
            on_transition_cancel: V,
        ) -> super::Building<super::overwrite::on_transition_cancel<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self
                    .HtmlMediaElementProps
                    .on_transition_cancel(on_transition_cancel),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_transition_end`]
        #[inline(always)]
        pub fn on_transition_end<V>(
            self,
            on_transition_end: V,
        ) -> super::Building<super::overwrite::on_transition_end<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self
                    .HtmlMediaElementProps
                    .on_transition_end(on_transition_end),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_transition_run`]
        #[inline(always)]
        pub fn on_transition_run<V>(
            self,
            on_transition_run: V,
        ) -> super::Building<super::overwrite::on_transition_run<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self
                    .HtmlMediaElementProps
                    .on_transition_run(on_transition_run),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_transition_start`]
        #[inline(always)]
        pub fn on_transition_start<V>(
            self,
            on_transition_start: V,
        ) -> super::Building<super::overwrite::on_transition_start<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self
                    .HtmlMediaElementProps
                    .on_transition_start(on_transition_start),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_drag`]
        #[inline(always)]
        pub fn on_drag<V>(
            self,
            on_drag: V,
        ) -> super::Building<super::overwrite::on_drag<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_drag(on_drag),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_drag_end`]
        #[inline(always)]
        pub fn on_drag_end<V>(
            self,
            on_drag_end: V,
        ) -> super::Building<super::overwrite::on_drag_end<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_drag_end(on_drag_end),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_drag_enter`]
        #[inline(always)]
        pub fn on_drag_enter<V>(
            self,
            on_drag_enter: V,
        ) -> super::Building<super::overwrite::on_drag_enter<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_drag_enter(on_drag_enter),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_drag_leave`]
        #[inline(always)]
        pub fn on_drag_leave<V>(
            self,
            on_drag_leave: V,
        ) -> super::Building<super::overwrite::on_drag_leave<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_drag_leave(on_drag_leave),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_drag_over`]
        #[inline(always)]
        pub fn on_drag_over<V>(
            self,
            on_drag_over: V,
        ) -> super::Building<super::overwrite::on_drag_over<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_drag_over(on_drag_over),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_drag_start`]
        #[inline(always)]
        pub fn on_drag_start<V>(
            self,
            on_drag_start: V,
        ) -> super::Building<super::overwrite::on_drag_start<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_drag_start(on_drag_start),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_drop`]
        #[inline(always)]
        pub fn on_drop<V>(
            self,
            on_drop: V,
        ) -> super::Building<super::overwrite::on_drop<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_drop(on_drop),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::auto_play`]
        #[inline(always)]
        pub fn auto_play<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
        >(
            self,
            auto_play: V,
        ) -> super::Building<super::overwrite::auto_play<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.auto_play(auto_play),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::controls`]
        #[inline(always)]
        pub fn controls<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>>(
            self,
            controls: V,
        ) -> super::Building<super::overwrite::controls<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.controls(controls),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::cross_origin`]
        #[inline(always)]
        pub fn cross_origin<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
        >(
            self,
            cross_origin: V,
        ) -> super::Building<super::overwrite::cross_origin<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.cross_origin(cross_origin),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::r#loop`]
        #[inline(always)]
        pub fn r#loop<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>>(
            self,
            r#loop: V,
        ) -> super::Building<super::overwrite::r#loop<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.r#loop(r#loop),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::muted`]
        #[inline(always)]
        pub fn muted<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>>(
            self,
            muted: V,
        ) -> super::Building<super::overwrite::muted<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.muted(muted),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::preload`]
        #[inline(always)]
        pub fn preload<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            preload: V,
        ) -> super::Building<super::overwrite::preload<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.preload(preload),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::src`]
        #[inline(always)]
        pub fn src<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            src: V,
        ) -> super::Building<super::overwrite::src<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.src(src),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_abort`]
        #[inline(always)]
        pub fn on_abort<V>(
            self,
            on_abort: V,
        ) -> super::Building<super::overwrite::on_abort<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_abort(on_abort),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_can_play`]
        #[inline(always)]
        pub fn on_can_play<V>(
            self,
            on_can_play: V,
        ) -> super::Building<super::overwrite::on_can_play<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_can_play(on_can_play),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_can_play_through`]
        #[inline(always)]
        pub fn on_can_play_through<V>(
            self,
            on_can_play_through: V,
        ) -> super::Building<super::overwrite::on_can_play_through<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self
                    .HtmlMediaElementProps
                    .on_can_play_through(on_can_play_through),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_duration_change`]
        #[inline(always)]
        pub fn on_duration_change<V>(
            self,
            on_duration_change: V,
        ) -> super::Building<super::overwrite::on_duration_change<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self
                    .HtmlMediaElementProps
                    .on_duration_change(on_duration_change),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_emptied`]
        #[inline(always)]
        pub fn on_emptied<V>(
            self,
            on_emptied: V,
        ) -> super::Building<super::overwrite::on_emptied<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_emptied(on_emptied),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_ended`]
        #[inline(always)]
        pub fn on_ended<V>(
            self,
            on_ended: V,
        ) -> super::Building<super::overwrite::on_ended<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_ended(on_ended),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_loaded_data`]
        #[inline(always)]
        pub fn on_loaded_data<V>(
            self,
            on_loaded_data: V,
        ) -> super::Building<super::overwrite::on_loaded_data<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_loaded_data(on_loaded_data),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_loaded_metadata`]
        #[inline(always)]
        pub fn on_loaded_metadata<V>(
            self,
            on_loaded_metadata: V,
        ) -> super::Building<super::overwrite::on_loaded_metadata<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self
                    .HtmlMediaElementProps
                    .on_loaded_metadata(on_loaded_metadata),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_load_start`]
        #[inline(always)]
        pub fn on_load_start<V>(
            self,
            on_load_start: V,
        ) -> super::Building<super::overwrite::on_load_start<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_load_start(on_load_start),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_pause`]
        #[inline(always)]
        pub fn on_pause<V>(
            self,
            on_pause: V,
        ) -> super::Building<super::overwrite::on_pause<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_pause(on_pause),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_play`]
        #[inline(always)]
        pub fn on_play<V>(
            self,
            on_play: V,
        ) -> super::Building<super::overwrite::on_play<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_play(on_play),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_playing`]
        #[inline(always)]
        pub fn on_playing<V>(
            self,
            on_playing: V,
        ) -> super::Building<super::overwrite::on_playing<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_playing(on_playing),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_progress`]
        #[inline(always)]
        pub fn on_progress<V>(
            self,
            on_progress: V,
        ) -> super::Building<super::overwrite::on_progress<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_progress(on_progress),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_rate_change`]
        #[inline(always)]
        pub fn on_rate_change<V>(
            self,
            on_rate_change: V,
        ) -> super::Building<super::overwrite::on_rate_change<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_rate_change(on_rate_change),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_resize`]
        #[inline(always)]
        pub fn on_resize<V>(
            self,
            on_resize: V,
        ) -> super::Building<super::overwrite::on_resize<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_resize(on_resize),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_seeked`]
        #[inline(always)]
        pub fn on_seeked<V>(
            self,
            on_seeked: V,
        ) -> super::Building<super::overwrite::on_seeked<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_seeked(on_seeked),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_seeking`]
        #[inline(always)]
        pub fn on_seeking<V>(
            self,
            on_seeking: V,
        ) -> super::Building<super::overwrite::on_seeking<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_seeking(on_seeking),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_stalled`]
        #[inline(always)]
        pub fn on_stalled<V>(
            self,
            on_stalled: V,
        ) -> super::Building<super::overwrite::on_stalled<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_stalled(on_stalled),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_suspend`]
        #[inline(always)]
        pub fn on_suspend<V>(
            self,
            on_suspend: V,
        ) -> super::Building<super::overwrite::on_suspend<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_suspend(on_suspend),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_time_update`]
        #[inline(always)]
        pub fn on_time_update<V>(
            self,
            on_time_update: V,
        ) -> super::Building<super::overwrite::on_time_update<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_time_update(on_time_update),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_volume_change`]
        #[inline(always)]
        pub fn on_volume_change<V>(
            self,
            on_volume_change: V,
        ) -> super::Building<super::overwrite::on_volume_change<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self
                    .HtmlMediaElementProps
                    .on_volume_change(on_volume_change),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        ///See [`HtmlMediaElementProps::on_waiting`]
        #[inline(always)]
        pub fn on_waiting<V>(
            self,
            on_waiting: V,
        ) -> super::Building<super::overwrite::on_waiting<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps.on_waiting(on_waiting),
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        #[inline(always)]
        pub fn height<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<u32>>(
            self,
            height: V,
        ) -> super::Building<super::overwrite::height<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps,
                height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        #[inline(always)]
        pub fn plays_inline<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
        >(
            self,
            plays_inline: V,
        ) -> super::Building<super::overwrite::plays_inline<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps,
                height: self.height,
                plays_inline,
                poster: self.poster,
                width: self.width,
            }
        }
        #[inline(always)]
        pub fn poster<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            poster: V,
        ) -> super::Building<super::overwrite::poster<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps,
                height: self.height,
                plays_inline: self.plays_inline,
                poster,
                width: self.width,
            }
        }
        #[inline(always)]
        pub fn width<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<u32>>(
            self,
            width: V,
        ) -> super::Building<super::overwrite::width<TypeDefs, V>> {
            super::Data {
                HtmlMediaElementProps: self.HtmlMediaElementProps,
                height: self.height,
                plays_inline: self.plays_inline,
                poster: self.poster,
                width,
            }
        }
    }
}
#[cfg(feature = "csr")]
mod impl_update_element {
    #[allow(unused_imports)]
    use super::super::*;
    impl<TypeDefs: ?::core::marker::Sized + super::Types>
        crate::imports::frender_csr::props::UpdateElement<web_sys::HtmlVideoElement>
        for super::Data<TypeDefs>
    where
        HtmlMediaElementProps::Data<TypeDefs::HtmlMediaElementProps>:
            crate::imports::frender_csr::props::UpdateElement<web_sys::HtmlMediaElement>,
    {
        type State = super::render_state::RenderState<
            dyn super::render_state::RenderStateTypes<
                HtmlMediaElementProps = <HtmlMediaElementProps::Data<
                    TypeDefs::HtmlMediaElementProps,
                > as crate::imports::frender_csr::props::UpdateElement<
                    web_sys::HtmlMediaElement,
                >>::State,
                height = <TypeDefs::height as ::frender_html::props::MaybeUpdateValueWithState<
                    u32,
                >>::State,
                plays_inline = <TypeDefs::plays_inline as ::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::State,
                poster = <TypeDefs::poster as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                width = <TypeDefs::width as ::frender_html::props::MaybeUpdateValueWithState<
                    u32,
                >>::State,
            >,
        >;
        fn initialize_state(
            this: Self,
            element: &web_sys::HtmlVideoElement,
            children_ctx: &mut ::frender_csr::Dom,
        ) -> Self::State {
            let dom_element: &::web_sys::Element = element.as_ref();
            super::render_state::RenderState {
                HtmlMediaElementProps: <HtmlMediaElementProps::Data<
                    TypeDefs::HtmlMediaElementProps,
                > as crate::imports::frender_csr::props::UpdateElement<
                    web_sys::HtmlMediaElement,
                >>::initialize_state(this.HtmlMediaElementProps, element, children_ctx),
                height: <TypeDefs::height as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    u32,
                >>::initialize_state_and_update(
                    this.height,
                    |v| dom_element.set_height(*v),
                    || dom_element.remove_attribute("height").unwrap(),
                ),
                plays_inline: <TypeDefs::plays_inline as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.plays_inline,
                    |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
                        *v,
                        dom_element,
                        "playsinline",
                    ),
                    || dom_element.remove_attribute("playsinline").unwrap(),
                ),
                poster: <TypeDefs::poster as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.poster,
                    |v| dom_element.set_poster(v),
                    || dom_element.remove_attribute("poster").unwrap(),
                ),
                width: <TypeDefs::width as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    u32,
                >>::initialize_state_and_update(
                    this.width,
                    |v| dom_element.set_width(*v),
                    || dom_element.remove_attribute("width").unwrap(),
                ),
            }
        }
        fn update_element(
            this: Self,
            element: &web_sys::HtmlVideoElement,
            children_ctx: &mut ::frender_csr::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let state = state.pin_project();
            let dom_element: &::web_sys::Element = element.as_ref();
            crate::imports::frender_csr::props::UpdateElement::update_element(
                this.HtmlMediaElementProps,
                element.as_ref(),
                children_ctx,
                state.HtmlMediaElementProps,
            );
            <TypeDefs::height as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                u32,
            >>::maybe_update_value_with_state(
                this.height,
                state.height,
                |v| dom_element.set_height(*v),
                || dom_element.remove_attribute("height").unwrap(),
            );
            <TypeDefs::plays_inline as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                bool,
            >>::maybe_update_value_with_state(
                this.plays_inline,
                state.plays_inline,
                |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
                    *v,
                    dom_element,
                    "playsinline",
                ),
                || dom_element.remove_attribute("playsinline").unwrap(),
            );
            <TypeDefs::poster as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.poster,
                state.poster,
                |v| dom_element.set_poster(v),
                || dom_element.remove_attribute("poster").unwrap(),
            );
            <TypeDefs::width as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                u32,
            >>::maybe_update_value_with_state(
                this.width,
                state.width,
                |v| dom_element.set_width(*v),
                || dom_element.remove_attribute("width").unwrap(),
            );
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
        HtmlMediaElementProps::Data<TypeDefs::HtmlMediaElementProps>: ::frender_ssr::IntoSsrData<W>,
    {
        type Children = <HtmlMediaElementProps::Data<
            TypeDefs::HtmlMediaElementProps,
        > as ::frender_ssr::IntoSsrData<W>>::Children;
        type ChildrenRenderState = <HtmlMediaElementProps::Data<
            TypeDefs::HtmlMediaElementProps,
        > as ::frender_ssr::IntoSsrData<W>>::ChildrenRenderState;
        type Attrs = ::core::iter::Chain<
            <HtmlMediaElementProps::Data<
                TypeDefs::HtmlMediaElementProps,
            > as ::frender_ssr::IntoSsrData<W>>::Attrs,
            ::frender_ssr::utils::filter::FilterArray<
                ::frender_ssr::element::html::HtmlAttrPair<'static>,
                4usize,
            >,
        >;
        fn into_ssr_data(this: Self) -> (Self::Children, Self::Attrs) {
            let (children, attrs) =
                ::frender_ssr::IntoSsrData::into_ssr_data(this.HtmlMediaElementProps);
            (
                children,
                attrs
                    .chain(
                        ::frender_ssr::utils::filter::FilterIdentity(
                            [
                                <TypeDefs::height as ::frender_html::props::MaybeUpdateValueWithState<
                                    u32,
                                >>::maybe_into_html_attribute_value(this.height)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("height"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::plays_inline as ::frender_html::props::MaybeUpdateValueWithState<
                                    bool,
                                >>::maybe_into_html_attribute_value(this.plays_inline)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("playsinline"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::poster as ::frender_html::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.poster)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("poster"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::width as ::frender_html::props::MaybeUpdateValueWithState<
                                    u32,
                                >>::maybe_into_html_attribute_value(this.width)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("width"),
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
