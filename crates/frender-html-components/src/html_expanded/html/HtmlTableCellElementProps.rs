#[allow(non_snake_case)]
#[inline(always)]
pub fn HtmlTableCellElementProps() -> Building<TypesInitial> {
    #[allow(unused_imports)]
    use super::*;
    self::Building {
        HtmlElementProps: HtmlElementProps::build(HtmlElementProps()),
        align: (),
        bg_color: (),
        char: (),
        char_off: (),
        v_align: (),
        col_span: (),
        headers: (),
        row_span: (),
        abbr: (),
        axis: (),
        height: (),
        scope: (),
        width: (),
    }
}
pub mod prelude {}
pub mod overwrite {
    #![allow(non_camel_case_types)]
    pub type HtmlElementProps<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = Value,
        align = <TypeDefs as super::Types>::align,
        bg_color = <TypeDefs as super::Types>::bg_color,
        char = <TypeDefs as super::Types>::char,
        char_off = <TypeDefs as super::Types>::char_off,
        v_align = <TypeDefs as super::Types>::v_align,
        col_span = <TypeDefs as super::Types>::col_span,
        headers = <TypeDefs as super::Types>::headers,
        row_span = <TypeDefs as super::Types>::row_span,
        abbr = <TypeDefs as super::Types>::abbr,
        axis = <TypeDefs as super::Types>::axis,
        height = <TypeDefs as super::Types>::height,
        scope = <TypeDefs as super::Types>::scope,
        width = <TypeDefs as super::Types>::width,
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
    pub type content_editable<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::content_editable<
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
    pub type align<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        align = Value,
        bg_color = <TypeDefs as super::Types>::bg_color,
        char = <TypeDefs as super::Types>::char,
        char_off = <TypeDefs as super::Types>::char_off,
        v_align = <TypeDefs as super::Types>::v_align,
        col_span = <TypeDefs as super::Types>::col_span,
        headers = <TypeDefs as super::Types>::headers,
        row_span = <TypeDefs as super::Types>::row_span,
        abbr = <TypeDefs as super::Types>::abbr,
        axis = <TypeDefs as super::Types>::axis,
        height = <TypeDefs as super::Types>::height,
        scope = <TypeDefs as super::Types>::scope,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type bg_color<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        align = <TypeDefs as super::Types>::align,
        bg_color = Value,
        char = <TypeDefs as super::Types>::char,
        char_off = <TypeDefs as super::Types>::char_off,
        v_align = <TypeDefs as super::Types>::v_align,
        col_span = <TypeDefs as super::Types>::col_span,
        headers = <TypeDefs as super::Types>::headers,
        row_span = <TypeDefs as super::Types>::row_span,
        abbr = <TypeDefs as super::Types>::abbr,
        axis = <TypeDefs as super::Types>::axis,
        height = <TypeDefs as super::Types>::height,
        scope = <TypeDefs as super::Types>::scope,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type char<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        align = <TypeDefs as super::Types>::align,
        bg_color = <TypeDefs as super::Types>::bg_color,
        char = Value,
        char_off = <TypeDefs as super::Types>::char_off,
        v_align = <TypeDefs as super::Types>::v_align,
        col_span = <TypeDefs as super::Types>::col_span,
        headers = <TypeDefs as super::Types>::headers,
        row_span = <TypeDefs as super::Types>::row_span,
        abbr = <TypeDefs as super::Types>::abbr,
        axis = <TypeDefs as super::Types>::axis,
        height = <TypeDefs as super::Types>::height,
        scope = <TypeDefs as super::Types>::scope,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type char_off<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        align = <TypeDefs as super::Types>::align,
        bg_color = <TypeDefs as super::Types>::bg_color,
        char = <TypeDefs as super::Types>::char,
        char_off = Value,
        v_align = <TypeDefs as super::Types>::v_align,
        col_span = <TypeDefs as super::Types>::col_span,
        headers = <TypeDefs as super::Types>::headers,
        row_span = <TypeDefs as super::Types>::row_span,
        abbr = <TypeDefs as super::Types>::abbr,
        axis = <TypeDefs as super::Types>::axis,
        height = <TypeDefs as super::Types>::height,
        scope = <TypeDefs as super::Types>::scope,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type v_align<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        align = <TypeDefs as super::Types>::align,
        bg_color = <TypeDefs as super::Types>::bg_color,
        char = <TypeDefs as super::Types>::char,
        char_off = <TypeDefs as super::Types>::char_off,
        v_align = Value,
        col_span = <TypeDefs as super::Types>::col_span,
        headers = <TypeDefs as super::Types>::headers,
        row_span = <TypeDefs as super::Types>::row_span,
        abbr = <TypeDefs as super::Types>::abbr,
        axis = <TypeDefs as super::Types>::axis,
        height = <TypeDefs as super::Types>::height,
        scope = <TypeDefs as super::Types>::scope,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type col_span<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        align = <TypeDefs as super::Types>::align,
        bg_color = <TypeDefs as super::Types>::bg_color,
        char = <TypeDefs as super::Types>::char,
        char_off = <TypeDefs as super::Types>::char_off,
        v_align = <TypeDefs as super::Types>::v_align,
        col_span = Value,
        headers = <TypeDefs as super::Types>::headers,
        row_span = <TypeDefs as super::Types>::row_span,
        abbr = <TypeDefs as super::Types>::abbr,
        axis = <TypeDefs as super::Types>::axis,
        height = <TypeDefs as super::Types>::height,
        scope = <TypeDefs as super::Types>::scope,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type headers<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        align = <TypeDefs as super::Types>::align,
        bg_color = <TypeDefs as super::Types>::bg_color,
        char = <TypeDefs as super::Types>::char,
        char_off = <TypeDefs as super::Types>::char_off,
        v_align = <TypeDefs as super::Types>::v_align,
        col_span = <TypeDefs as super::Types>::col_span,
        headers = Value,
        row_span = <TypeDefs as super::Types>::row_span,
        abbr = <TypeDefs as super::Types>::abbr,
        axis = <TypeDefs as super::Types>::axis,
        height = <TypeDefs as super::Types>::height,
        scope = <TypeDefs as super::Types>::scope,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type row_span<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        align = <TypeDefs as super::Types>::align,
        bg_color = <TypeDefs as super::Types>::bg_color,
        char = <TypeDefs as super::Types>::char,
        char_off = <TypeDefs as super::Types>::char_off,
        v_align = <TypeDefs as super::Types>::v_align,
        col_span = <TypeDefs as super::Types>::col_span,
        headers = <TypeDefs as super::Types>::headers,
        row_span = Value,
        abbr = <TypeDefs as super::Types>::abbr,
        axis = <TypeDefs as super::Types>::axis,
        height = <TypeDefs as super::Types>::height,
        scope = <TypeDefs as super::Types>::scope,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type abbr<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        align = <TypeDefs as super::Types>::align,
        bg_color = <TypeDefs as super::Types>::bg_color,
        char = <TypeDefs as super::Types>::char,
        char_off = <TypeDefs as super::Types>::char_off,
        v_align = <TypeDefs as super::Types>::v_align,
        col_span = <TypeDefs as super::Types>::col_span,
        headers = <TypeDefs as super::Types>::headers,
        row_span = <TypeDefs as super::Types>::row_span,
        abbr = Value,
        axis = <TypeDefs as super::Types>::axis,
        height = <TypeDefs as super::Types>::height,
        scope = <TypeDefs as super::Types>::scope,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type axis<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        align = <TypeDefs as super::Types>::align,
        bg_color = <TypeDefs as super::Types>::bg_color,
        char = <TypeDefs as super::Types>::char,
        char_off = <TypeDefs as super::Types>::char_off,
        v_align = <TypeDefs as super::Types>::v_align,
        col_span = <TypeDefs as super::Types>::col_span,
        headers = <TypeDefs as super::Types>::headers,
        row_span = <TypeDefs as super::Types>::row_span,
        abbr = <TypeDefs as super::Types>::abbr,
        axis = Value,
        height = <TypeDefs as super::Types>::height,
        scope = <TypeDefs as super::Types>::scope,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type height<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        align = <TypeDefs as super::Types>::align,
        bg_color = <TypeDefs as super::Types>::bg_color,
        char = <TypeDefs as super::Types>::char,
        char_off = <TypeDefs as super::Types>::char_off,
        v_align = <TypeDefs as super::Types>::v_align,
        col_span = <TypeDefs as super::Types>::col_span,
        headers = <TypeDefs as super::Types>::headers,
        row_span = <TypeDefs as super::Types>::row_span,
        abbr = <TypeDefs as super::Types>::abbr,
        axis = <TypeDefs as super::Types>::axis,
        height = Value,
        scope = <TypeDefs as super::Types>::scope,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type scope<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        align = <TypeDefs as super::Types>::align,
        bg_color = <TypeDefs as super::Types>::bg_color,
        char = <TypeDefs as super::Types>::char,
        char_off = <TypeDefs as super::Types>::char_off,
        v_align = <TypeDefs as super::Types>::v_align,
        col_span = <TypeDefs as super::Types>::col_span,
        headers = <TypeDefs as super::Types>::headers,
        row_span = <TypeDefs as super::Types>::row_span,
        abbr = <TypeDefs as super::Types>::abbr,
        axis = <TypeDefs as super::Types>::axis,
        height = <TypeDefs as super::Types>::height,
        scope = Value,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type width<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        align = <TypeDefs as super::Types>::align,
        bg_color = <TypeDefs as super::Types>::bg_color,
        char = <TypeDefs as super::Types>::char,
        char_off = <TypeDefs as super::Types>::char_off,
        v_align = <TypeDefs as super::Types>::v_align,
        col_span = <TypeDefs as super::Types>::col_span,
        headers = <TypeDefs as super::Types>::headers,
        row_span = <TypeDefs as super::Types>::row_span,
        abbr = <TypeDefs as super::Types>::abbr,
        axis = <TypeDefs as super::Types>::axis,
        height = <TypeDefs as super::Types>::height,
        scope = <TypeDefs as super::Types>::scope,
        width = Value,
    >;
}
mod trait_types {
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub trait Types {
        type HtmlElementProps: ?::core::marker::Sized + HtmlElementProps::Types;
        type align: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>;
        type bg_color: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>;
        type char: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>;
        type char_off: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>;
        type v_align: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>;
        type col_span: crate::imports::frender_html::props::MaybeUpdateValueWithState<u32>;
        type headers: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>;
        type row_span: crate::imports::frender_html::props::MaybeUpdateValueWithState<u32>;
        type abbr: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>;
        type axis: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>;
        type height: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>;
        type scope: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>;
        type width: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>;
    }
}
pub use trait_types::Types;
pub use trait_types::Types as ValidTypes;
pub mod data_struct {
    #[non_exhaustive]
    pub struct HtmlTableCellElementProps<TypeDefs: super::Types + ?::core::marker::Sized> {
        pub HtmlElementProps: super::super::HtmlElementProps::Data<TypeDefs::HtmlElementProps>,
        pub align: TypeDefs::align,
        pub bg_color: TypeDefs::bg_color,
        pub char: TypeDefs::char,
        pub char_off: TypeDefs::char_off,
        pub v_align: TypeDefs::v_align,
        pub col_span: TypeDefs::col_span,
        pub headers: TypeDefs::headers,
        pub row_span: TypeDefs::row_span,
        pub abbr: TypeDefs::abbr,
        pub axis: TypeDefs::axis,
        pub height: TypeDefs::height,
        pub scope: TypeDefs::scope,
        pub width: TypeDefs::width,
    }
}
pub use ::core::convert::identity as Building;
pub use ::core::convert::identity as build;
pub use data_struct::HtmlTableCellElementProps as Data;
pub use data_struct::HtmlTableCellElementProps as Building;
pub struct Replacing<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
mod types_initial {
    #[allow(unused_imports)]
    use super::super::*;
    pub type TypesInitial = dyn super::Types<
        HtmlElementProps = HtmlElementProps::TypesInitial,
        align = (),
        bg_color = (),
        char = (),
        char_off = (),
        v_align = (),
        col_span = (),
        headers = (),
        row_span = (),
        abbr = (),
        axis = (),
        height = (),
        scope = (),
        width = (),
    >;
}
pub use types_initial::TypesInitial;
pub type DataInitial = Data<TypesInitial>;
#[cfg(feature = "dom")]
pub mod render_state {
    #[allow(non_camel_case_types)]
    pub trait RenderStateTypes {
        type HtmlElementProps: crate::imports::frender_dom::props::IntrinsicComponentPollReactive;
        type align;
        type bg_color;
        type char;
        type char_off;
        type v_align;
        type col_span;
        type headers;
        type row_span;
        type abbr;
        type axis;
        type height;
        type scope;
        type width;
    }
    pub struct RenderState<TypeDefs: RenderStateTypes>
    where
        TypeDefs: ?::core::marker::Sized,
    {
        pub HtmlElementProps: TypeDefs::HtmlElementProps,
        pub align: TypeDefs::align,
        pub bg_color: TypeDefs::bg_color,
        pub char: TypeDefs::char,
        pub char_off: TypeDefs::char_off,
        pub v_align: TypeDefs::v_align,
        pub col_span: TypeDefs::col_span,
        pub headers: TypeDefs::headers,
        pub row_span: TypeDefs::row_span,
        pub abbr: TypeDefs::abbr,
        pub axis: TypeDefs::axis,
        pub height: TypeDefs::height,
        pub scope: TypeDefs::scope,
        pub width: TypeDefs::width,
    }
    #[allow(dead_code)]
    #[allow(single_use_lifetimes)]
    #[allow(clippy::unknown_clippy_lints)]
    #[allow(clippy::mut_mut)]
    #[allow(clippy::redundant_pub_crate)]
    #[allow(clippy::ref_option_ref)]
    #[allow(clippy::type_repetition_in_bounds)]
    pub(crate) struct RenderStateProj<'__pin, TypeDefs: RenderStateTypes>
    where
        RenderState<TypeDefs>: '__pin,
        TypeDefs: ?::core::marker::Sized,
    {
        pub HtmlElementProps:
            ::pin_project_lite::__private::Pin<&'__pin mut (TypeDefs::HtmlElementProps)>,
        pub align: &'__pin mut (TypeDefs::align),
        pub bg_color: &'__pin mut (TypeDefs::bg_color),
        pub char: &'__pin mut (TypeDefs::char),
        pub char_off: &'__pin mut (TypeDefs::char_off),
        pub v_align: &'__pin mut (TypeDefs::v_align),
        pub col_span: &'__pin mut (TypeDefs::col_span),
        pub headers: &'__pin mut (TypeDefs::headers),
        pub row_span: &'__pin mut (TypeDefs::row_span),
        pub abbr: &'__pin mut (TypeDefs::abbr),
        pub axis: &'__pin mut (TypeDefs::axis),
        pub height: &'__pin mut (TypeDefs::height),
        pub scope: &'__pin mut (TypeDefs::scope),
        pub width: &'__pin mut (TypeDefs::width),
    }
    #[allow(explicit_outlives_requirements)]
    #[allow(single_use_lifetimes)]
    #[allow(clippy::unknown_clippy_lints)]
    #[allow(clippy::redundant_pub_crate)]
    #[allow(clippy::used_underscore_binding)]
    const _: () = {
        #[allow(dead_code)]
        #[allow(single_use_lifetimes)]
        #[allow(clippy::unknown_clippy_lints)]
        #[allow(clippy::mut_mut)]
        #[allow(clippy::redundant_pub_crate)]
        #[allow(clippy::ref_option_ref)]
        #[allow(clippy::type_repetition_in_bounds)]
        pub(crate) struct ProjectionRef<'__pin, TypeDefs: RenderStateTypes>
        where
            RenderState<TypeDefs>: '__pin,
            TypeDefs: ?::core::marker::Sized,
        {
            pub HtmlElementProps:
                ::pin_project_lite::__private::Pin<&'__pin (TypeDefs::HtmlElementProps)>,
            pub align: &'__pin (TypeDefs::align),
            pub bg_color: &'__pin (TypeDefs::bg_color),
            pub char: &'__pin (TypeDefs::char),
            pub char_off: &'__pin (TypeDefs::char_off),
            pub v_align: &'__pin (TypeDefs::v_align),
            pub col_span: &'__pin (TypeDefs::col_span),
            pub headers: &'__pin (TypeDefs::headers),
            pub row_span: &'__pin (TypeDefs::row_span),
            pub abbr: &'__pin (TypeDefs::abbr),
            pub axis: &'__pin (TypeDefs::axis),
            pub height: &'__pin (TypeDefs::height),
            pub scope: &'__pin (TypeDefs::scope),
            pub width: &'__pin (TypeDefs::width),
        }
        impl<TypeDefs: RenderStateTypes> RenderState<TypeDefs>
        where
            TypeDefs: ?::core::marker::Sized,
        {
            pub(crate) fn project<'__pin>(
                self: ::pin_project_lite::__private::Pin<&'__pin mut Self>,
            ) -> RenderStateProj<'__pin, TypeDefs> {
                unsafe {
                    let Self {
                        HtmlElementProps,
                        align,
                        bg_color,
                        char,
                        char_off,
                        v_align,
                        col_span,
                        headers,
                        row_span,
                        abbr,
                        axis,
                        height,
                        scope,
                        width,
                    } = self.get_unchecked_mut();
                    RenderStateProj {
                        HtmlElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            HtmlElementProps,
                        ),
                        align: align,
                        bg_color: bg_color,
                        char: char,
                        char_off: char_off,
                        v_align: v_align,
                        col_span: col_span,
                        headers: headers,
                        row_span: row_span,
                        abbr: abbr,
                        axis: axis,
                        height: height,
                        scope: scope,
                        width: width,
                    }
                }
            }
            pub(crate) fn project_ref<'__pin>(
                self: ::pin_project_lite::__private::Pin<&'__pin Self>,
            ) -> ProjectionRef<'__pin, TypeDefs> {
                unsafe {
                    let Self {
                        HtmlElementProps,
                        align,
                        bg_color,
                        char,
                        char_off,
                        v_align,
                        col_span,
                        headers,
                        row_span,
                        abbr,
                        axis,
                        height,
                        scope,
                        width,
                    } = self.get_ref();
                    ProjectionRef {
                        HtmlElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            HtmlElementProps,
                        ),
                        align: align,
                        bg_color: bg_color,
                        char: char,
                        char_off: char_off,
                        v_align: v_align,
                        col_span: col_span,
                        headers: headers,
                        row_span: row_span,
                        abbr: abbr,
                        axis: axis,
                        height: height,
                        scope: scope,
                        width: width,
                    }
                }
            }
        }
        #[allow(non_snake_case)]
        pub struct __Origin<'__pin, TypeDefs: RenderStateTypes>
        where
            TypeDefs: ?::core::marker::Sized,
        {
            __dummy_lifetime: ::pin_project_lite::__private::PhantomData<&'__pin ()>,
            HtmlElementProps: TypeDefs::HtmlElementProps,
            align: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::align>,
            bg_color: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::bg_color>,
            char: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::char>,
            char_off: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::char_off>,
            v_align: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::v_align>,
            col_span: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::col_span>,
            headers: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::headers>,
            row_span: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::row_span>,
            abbr: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::abbr>,
            axis: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::axis>,
            height: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::height>,
            scope: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::scope>,
            width: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::width>,
        }
        impl<'__pin, TypeDefs: RenderStateTypes> ::pin_project_lite::__private::Unpin
            for RenderState<TypeDefs>
        where
            __Origin<'__pin, TypeDefs>: ::pin_project_lite::__private::Unpin,
            TypeDefs: ?::core::marker::Sized,
        {
        }
        trait MustNotImplDrop {}
        #[allow(clippy::drop_bounds, drop_bounds)]
        impl<T: ::pin_project_lite::__private::Drop> MustNotImplDrop for T {}
        impl<TypeDefs: RenderStateTypes> MustNotImplDrop for RenderState<TypeDefs> where
            TypeDefs: ?::core::marker::Sized
        {
        }
        #[forbid(unaligned_references, safe_packed_borrows)]
        fn __assert_not_repr_packed<TypeDefs: RenderStateTypes>(this: &RenderState<TypeDefs>)
        where
            TypeDefs: ?::core::marker::Sized,
        {
            let _ = &this.HtmlElementProps;
            let _ = &this.align;
            let _ = &this.bg_color;
            let _ = &this.char;
            let _ = &this.char_off;
            let _ = &this.v_align;
            let _ = &this.col_span;
            let _ = &this.headers;
            let _ = &this.row_span;
            let _ = &this.abbr;
            let _ = &this.axis;
            let _ = &this.height;
            let _ = &this.scope;
            let _ = &this.width;
        }
    };
    impl<TypeDefs: ?::core::marker::Sized + RenderStateTypes> RenderState<TypeDefs> {
        #[inline(always)]
        pub(crate) fn pin_project(
            self: ::core::pin::Pin<&mut Self>,
        ) -> RenderStateProj<'_, TypeDefs> {
            self.project()
        }
    }
    impl<TypeDefs: ?::core::marker::Sized + RenderStateTypes>
        crate::imports::frender_dom::props::IntrinsicComponentPollReactive
        for RenderState<TypeDefs>
    {
        #[inline]
        fn intrinsic_component_poll_reactive(
            self: ::core::pin::Pin<&mut Self>,
            cx: &mut ::core::task::Context<'_>,
        ) -> ::core::task::Poll<bool> {
            crate::imports::frender_dom::props::IntrinsicComponentPollReactive::intrinsic_component_poll_reactive(
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
            }
        }
        ///See [`HtmlElementProps::content_editable`]
        #[inline(always)]
        pub fn content_editable<V: frender_html::props::MaybeInherit<bool>>(
            self,
            content_editable: V,
        ) -> super::Building<super::overwrite::content_editable<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.content_editable(content_editable),
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
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
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
            }
        }
        #[deprecated]
        #[inline(always)]
        pub fn align<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            align: V,
        ) -> super::Building<super::overwrite::align<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
            }
        }
        #[deprecated]
        #[inline(always)]
        pub fn bg_color<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            bg_color: V,
        ) -> super::Building<super::overwrite::bg_color<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                align: self.align,
                bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
            }
        }
        #[deprecated]
        #[inline(always)]
        pub fn char<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            char: V,
        ) -> super::Building<super::overwrite::char<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                align: self.align,
                bg_color: self.bg_color,
                char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
            }
        }
        #[deprecated]
        #[inline(always)]
        pub fn char_off<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            char_off: V,
        ) -> super::Building<super::overwrite::char_off<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
            }
        }
        #[deprecated]
        #[inline(always)]
        pub fn v_align<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            v_align: V,
        ) -> super::Building<super::overwrite::v_align<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
            }
        }
        #[inline(always)]
        pub fn col_span<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<u32>>(
            self,
            col_span: V,
        ) -> super::Building<super::overwrite::col_span<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
            }
        }
        #[inline(always)]
        pub fn headers<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            headers: V,
        ) -> super::Building<super::overwrite::headers<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
            }
        }
        #[inline(always)]
        pub fn row_span<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<u32>>(
            self,
            row_span: V,
        ) -> super::Building<super::overwrite::row_span<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
            }
        }
        #[deprecated = "Do not use this attribute as it is obsolete in the latest standard. Alternatively, you can put the abbreviated description inside the cell and place the long content in the title attribute."]
        #[inline(always)]
        pub fn abbr<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            abbr: V,
        ) -> super::Building<super::overwrite::abbr<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
            }
        }
        #[deprecated]
        #[inline(always)]
        pub fn axis<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            axis: V,
        ) -> super::Building<super::overwrite::axis<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis,
                height: self.height,
                scope: self.scope,
                width: self.width,
            }
        }
        #[deprecated = "Use the CSS height property instead."]
        #[inline(always)]
        pub fn height<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            height: V,
        ) -> super::Building<super::overwrite::height<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height,
                scope: self.scope,
                width: self.width,
            }
        }
        #[deprecated]
        #[inline(always)]
        pub fn scope<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            scope: V,
        ) -> super::Building<super::overwrite::scope<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope,
                width: self.width,
            }
        }
        #[deprecated]
        #[inline(always)]
        pub fn width<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            width: V,
        ) -> super::Building<super::overwrite::width<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                align: self.align,
                bg_color: self.bg_color,
                char: self.char,
                char_off: self.char_off,
                v_align: self.v_align,
                col_span: self.col_span,
                headers: self.headers,
                row_span: self.row_span,
                abbr: self.abbr,
                axis: self.axis,
                height: self.height,
                scope: self.scope,
                width,
            }
        }
    }
}
#[cfg(feature = "dom")]
mod impl_update_element {
    #[allow(unused_imports)]
    use super::super::*;
    impl<TypeDefs: ?::core::marker::Sized + super::Types>
        crate::imports::frender_dom::props::UpdateElement<web_sys::HtmlTableCellElement>
        for super::Data<TypeDefs>
    where
        HtmlElementProps::Data<TypeDefs::HtmlElementProps>:
            crate::imports::frender_dom::props::UpdateElement<web_sys::HtmlElement>,
    {
        type State = super::render_state::RenderState<
            dyn super::render_state::RenderStateTypes<
                HtmlElementProps = <HtmlElementProps::Data<
                    TypeDefs::HtmlElementProps,
                > as crate::imports::frender_dom::props::UpdateElement<
                    web_sys::HtmlElement,
                >>::State,
                align = <TypeDefs::align as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                bg_color = <TypeDefs::bg_color as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                char = <TypeDefs::char as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                char_off = <TypeDefs::char_off as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                v_align = <TypeDefs::v_align as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                col_span = <TypeDefs::col_span as ::frender_html::props::MaybeUpdateValueWithState<
                    u32,
                >>::State,
                headers = <TypeDefs::headers as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                row_span = <TypeDefs::row_span as ::frender_html::props::MaybeUpdateValueWithState<
                    u32,
                >>::State,
                abbr = <TypeDefs::abbr as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                axis = <TypeDefs::axis as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                height = <TypeDefs::height as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                scope = <TypeDefs::scope as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                width = <TypeDefs::width as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
            >,
        >;
        fn initialize_state(
            this: Self,
            element: &web_sys::HtmlTableCellElement,
            children_ctx: &mut ::frender_dom::Dom,
        ) -> Self::State {
            let dom_element: &::web_sys::Element = element.as_ref();
            super::render_state::RenderState {
                HtmlElementProps: <HtmlElementProps::Data<
                    TypeDefs::HtmlElementProps,
                > as crate::imports::frender_dom::props::UpdateElement<
                    web_sys::HtmlElement,
                >>::initialize_state(this.HtmlElementProps, element, children_ctx),
                align: <TypeDefs::align as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.align,
                    |v| element.set_align(v),
                    || dom_element.remove_attribute("align").unwrap(),
                ),
                bg_color: <TypeDefs::bg_color as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.bg_color,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "bgcolor",
                    ),
                    || dom_element.remove_attribute("bgcolor").unwrap(),
                ),
                char: <TypeDefs::char as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.char,
                    |v| element.set_ch(v),
                    || dom_element.remove_attribute("char").unwrap(),
                ),
                char_off: <TypeDefs::char_off as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.char_off,
                    |v| element.set_ch_off(v),
                    || dom_element.remove_attribute("charoff").unwrap(),
                ),
                v_align: <TypeDefs::v_align as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.v_align,
                    |v| element.set_v_align(v),
                    || dom_element.remove_attribute("valign").unwrap(),
                ),
                col_span: <TypeDefs::col_span as ::frender_html::props::MaybeUpdateValueWithState<
                    u32,
                >>::initialize_state_and_update(
                    this.col_span,
                    |v| element.set_col_span(*v),
                    || dom_element.remove_attribute("colspan").unwrap(),
                ),
                headers: <TypeDefs::headers as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.headers,
                    |v| element.set_headers(v),
                    || dom_element.remove_attribute("headers").unwrap(),
                ),
                row_span: <TypeDefs::row_span as ::frender_html::props::MaybeUpdateValueWithState<
                    u32,
                >>::initialize_state_and_update(
                    this.row_span,
                    |v| element.set_row_span(*v),
                    || dom_element.remove_attribute("rowspan").unwrap(),
                ),
                abbr: <TypeDefs::abbr as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.abbr,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "abbr",
                    ),
                    || dom_element.remove_attribute("abbr").unwrap(),
                ),
                axis: <TypeDefs::axis as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.axis,
                    |v| element.set_axis(v),
                    || dom_element.remove_attribute("axis").unwrap(),
                ),
                height: <TypeDefs::height as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.height,
                    |v| element.set_height(v),
                    || dom_element.remove_attribute("height").unwrap(),
                ),
                scope: <TypeDefs::scope as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.scope,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "scope",
                    ),
                    || dom_element.remove_attribute("scope").unwrap(),
                ),
                width: <TypeDefs::width as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.width,
                    |v| element.set_width(v),
                    || dom_element.remove_attribute("width").unwrap(),
                ),
            }
        }
        fn update_element(
            this: Self,
            element: &web_sys::HtmlTableCellElement,
            children_ctx: &mut ::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let state = state.pin_project();
            let dom_element: &::web_sys::Element = element.as_ref();
            crate::imports::frender_dom::props::UpdateElement::update_element(
                this.HtmlElementProps,
                element.as_ref(),
                children_ctx,
                state.HtmlElementProps,
            );
            <TypeDefs::align as ::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.align,
                state.align,
                |v| element.set_align(v),
                || dom_element.remove_attribute("align").unwrap(),
            );
            <TypeDefs::bg_color as ::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.bg_color,
                state.bg_color,
                |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "bgcolor",
                ),
                || dom_element.remove_attribute("bgcolor").unwrap(),
            );
            <TypeDefs::char as ::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.char,
                state.char,
                |v| element.set_ch(v),
                || dom_element.remove_attribute("char").unwrap(),
            );
            <TypeDefs::char_off as ::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.char_off,
                state.char_off,
                |v| element.set_ch_off(v),
                || dom_element.remove_attribute("charoff").unwrap(),
            );
            <TypeDefs::v_align as ::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.v_align,
                state.v_align,
                |v| element.set_v_align(v),
                || dom_element.remove_attribute("valign").unwrap(),
            );
            <TypeDefs::col_span as ::frender_html::props::MaybeUpdateValueWithState<
                u32,
            >>::maybe_update_value_with_state(
                this.col_span,
                state.col_span,
                |v| element.set_col_span(*v),
                || dom_element.remove_attribute("colspan").unwrap(),
            );
            <TypeDefs::headers as ::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.headers,
                state.headers,
                |v| element.set_headers(v),
                || dom_element.remove_attribute("headers").unwrap(),
            );
            <TypeDefs::row_span as ::frender_html::props::MaybeUpdateValueWithState<
                u32,
            >>::maybe_update_value_with_state(
                this.row_span,
                state.row_span,
                |v| element.set_row_span(*v),
                || dom_element.remove_attribute("rowspan").unwrap(),
            );
            <TypeDefs::abbr as ::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.abbr,
                state.abbr,
                |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "abbr",
                ),
                || dom_element.remove_attribute("abbr").unwrap(),
            );
            <TypeDefs::axis as ::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.axis,
                state.axis,
                |v| element.set_axis(v),
                || dom_element.remove_attribute("axis").unwrap(),
            );
            <TypeDefs::height as ::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.height,
                state.height,
                |v| element.set_height(v),
                || dom_element.remove_attribute("height").unwrap(),
            );
            <TypeDefs::scope as ::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.scope,
                state.scope,
                |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "scope",
                ),
                || dom_element.remove_attribute("scope").unwrap(),
            );
            <TypeDefs::width as ::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.width,
                state.width,
                |v| element.set_width(v),
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
                13usize,
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
                                <TypeDefs::align as ::frender_html::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.align)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("align"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::bg_color as ::frender_html::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.bg_color)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("bgcolor"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::char as ::frender_html::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.char)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("char"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::char_off as ::frender_html::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.char_off)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("charoff"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::v_align as ::frender_html::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.v_align)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("valign"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::col_span as ::frender_html::props::MaybeUpdateValueWithState<
                                    u32,
                                >>::maybe_into_html_attribute_value(this.col_span)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("colspan"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::headers as ::frender_html::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.headers)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("headers"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::row_span as ::frender_html::props::MaybeUpdateValueWithState<
                                    u32,
                                >>::maybe_into_html_attribute_value(this.row_span)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("rowspan"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::abbr as ::frender_html::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.abbr)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("abbr"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::axis as ::frender_html::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.axis)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("axis"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::height as ::frender_html::props::MaybeUpdateValueWithState<
                                    str,
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
                                <TypeDefs::scope as ::frender_html::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.scope)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("scope"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::width as ::frender_html::props::MaybeUpdateValueWithState<
                                    str,
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