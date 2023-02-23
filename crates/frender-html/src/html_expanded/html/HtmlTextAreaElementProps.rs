#[allow(non_snake_case)]
#[inline(always)]
pub fn HtmlTextAreaElementProps() -> Building<TypesInitial> {
    #[allow(unused_imports)]
    use super::*;
    self::Building {
        HtmlElementProps: HtmlElementProps::build(HtmlElementProps()),
        auto_complete: (),
        auto_correct: (),
        cols: (),
        disabled: (),
        form: (),
        max_length: (),
        min_length: (),
        name: (),
        placeholder: (),
        read_only: (),
        required: (),
        rows: (),
        wrap: (),
    }
}
pub mod prelude {}
pub mod overwrite {
    #![allow(non_camel_case_types)]
    pub type HtmlElementProps<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = Value,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        auto_correct = <TypeDefs as super::Types>::auto_correct,
        cols = <TypeDefs as super::Types>::cols,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        max_length = <TypeDefs as super::Types>::max_length,
        min_length = <TypeDefs as super::Types>::min_length,
        name = <TypeDefs as super::Types>::name,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        rows = <TypeDefs as super::Types>::rows,
        wrap = <TypeDefs as super::Types>::wrap,
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
    pub type auto_complete<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_complete = Value,
        auto_correct = <TypeDefs as super::Types>::auto_correct,
        cols = <TypeDefs as super::Types>::cols,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        max_length = <TypeDefs as super::Types>::max_length,
        min_length = <TypeDefs as super::Types>::min_length,
        name = <TypeDefs as super::Types>::name,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        rows = <TypeDefs as super::Types>::rows,
        wrap = <TypeDefs as super::Types>::wrap,
    >;
    pub type auto_correct<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        auto_correct = Value,
        cols = <TypeDefs as super::Types>::cols,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        max_length = <TypeDefs as super::Types>::max_length,
        min_length = <TypeDefs as super::Types>::min_length,
        name = <TypeDefs as super::Types>::name,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        rows = <TypeDefs as super::Types>::rows,
        wrap = <TypeDefs as super::Types>::wrap,
    >;
    pub type cols<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        auto_correct = <TypeDefs as super::Types>::auto_correct,
        cols = Value,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        max_length = <TypeDefs as super::Types>::max_length,
        min_length = <TypeDefs as super::Types>::min_length,
        name = <TypeDefs as super::Types>::name,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        rows = <TypeDefs as super::Types>::rows,
        wrap = <TypeDefs as super::Types>::wrap,
    >;
    pub type disabled<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        auto_correct = <TypeDefs as super::Types>::auto_correct,
        cols = <TypeDefs as super::Types>::cols,
        disabled = Value,
        form = <TypeDefs as super::Types>::form,
        max_length = <TypeDefs as super::Types>::max_length,
        min_length = <TypeDefs as super::Types>::min_length,
        name = <TypeDefs as super::Types>::name,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        rows = <TypeDefs as super::Types>::rows,
        wrap = <TypeDefs as super::Types>::wrap,
    >;
    pub type form<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        auto_correct = <TypeDefs as super::Types>::auto_correct,
        cols = <TypeDefs as super::Types>::cols,
        disabled = <TypeDefs as super::Types>::disabled,
        form = Value,
        max_length = <TypeDefs as super::Types>::max_length,
        min_length = <TypeDefs as super::Types>::min_length,
        name = <TypeDefs as super::Types>::name,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        rows = <TypeDefs as super::Types>::rows,
        wrap = <TypeDefs as super::Types>::wrap,
    >;
    pub type max_length<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        auto_correct = <TypeDefs as super::Types>::auto_correct,
        cols = <TypeDefs as super::Types>::cols,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        max_length = Value,
        min_length = <TypeDefs as super::Types>::min_length,
        name = <TypeDefs as super::Types>::name,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        rows = <TypeDefs as super::Types>::rows,
        wrap = <TypeDefs as super::Types>::wrap,
    >;
    pub type min_length<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        auto_correct = <TypeDefs as super::Types>::auto_correct,
        cols = <TypeDefs as super::Types>::cols,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        max_length = <TypeDefs as super::Types>::max_length,
        min_length = Value,
        name = <TypeDefs as super::Types>::name,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        rows = <TypeDefs as super::Types>::rows,
        wrap = <TypeDefs as super::Types>::wrap,
    >;
    pub type name<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        auto_correct = <TypeDefs as super::Types>::auto_correct,
        cols = <TypeDefs as super::Types>::cols,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        max_length = <TypeDefs as super::Types>::max_length,
        min_length = <TypeDefs as super::Types>::min_length,
        name = Value,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        rows = <TypeDefs as super::Types>::rows,
        wrap = <TypeDefs as super::Types>::wrap,
    >;
    pub type placeholder<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        auto_correct = <TypeDefs as super::Types>::auto_correct,
        cols = <TypeDefs as super::Types>::cols,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        max_length = <TypeDefs as super::Types>::max_length,
        min_length = <TypeDefs as super::Types>::min_length,
        name = <TypeDefs as super::Types>::name,
        placeholder = Value,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        rows = <TypeDefs as super::Types>::rows,
        wrap = <TypeDefs as super::Types>::wrap,
    >;
    pub type read_only<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        auto_correct = <TypeDefs as super::Types>::auto_correct,
        cols = <TypeDefs as super::Types>::cols,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        max_length = <TypeDefs as super::Types>::max_length,
        min_length = <TypeDefs as super::Types>::min_length,
        name = <TypeDefs as super::Types>::name,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = Value,
        required = <TypeDefs as super::Types>::required,
        rows = <TypeDefs as super::Types>::rows,
        wrap = <TypeDefs as super::Types>::wrap,
    >;
    pub type required<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        auto_correct = <TypeDefs as super::Types>::auto_correct,
        cols = <TypeDefs as super::Types>::cols,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        max_length = <TypeDefs as super::Types>::max_length,
        min_length = <TypeDefs as super::Types>::min_length,
        name = <TypeDefs as super::Types>::name,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = Value,
        rows = <TypeDefs as super::Types>::rows,
        wrap = <TypeDefs as super::Types>::wrap,
    >;
    pub type rows<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        auto_correct = <TypeDefs as super::Types>::auto_correct,
        cols = <TypeDefs as super::Types>::cols,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        max_length = <TypeDefs as super::Types>::max_length,
        min_length = <TypeDefs as super::Types>::min_length,
        name = <TypeDefs as super::Types>::name,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        rows = Value,
        wrap = <TypeDefs as super::Types>::wrap,
    >;
    pub type wrap<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        auto_correct = <TypeDefs as super::Types>::auto_correct,
        cols = <TypeDefs as super::Types>::cols,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        max_length = <TypeDefs as super::Types>::max_length,
        min_length = <TypeDefs as super::Types>::min_length,
        name = <TypeDefs as super::Types>::name,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        rows = <TypeDefs as super::Types>::rows,
        wrap = Value,
    >;
}
mod trait_types {
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub trait Types {
        type HtmlElementProps: ?::core::marker::Sized + HtmlElementProps::Types;
        type auto_complete: crate::MaybeUpdateValueWithState<str>;
        type auto_correct: crate::MaybeUpdateValueWithState<str>;
        type cols: crate::MaybeUpdateValueWithState<u32>;
        type disabled: crate::MaybeUpdateValueWithState<bool>;
        type form: crate::MaybeUpdateValueWithState<str>;
        type max_length: crate::MaybeUpdateValueWithState<i32>;
        type min_length: crate::MaybeUpdateValueWithState<i32>;
        type name: crate::MaybeUpdateValueWithState<str>;
        type placeholder: crate::MaybeUpdateValueWithState<str>;
        type read_only: crate::MaybeUpdateValueWithState<bool>;
        type required: crate::MaybeUpdateValueWithState<bool>;
        type rows: crate::MaybeUpdateValueWithState<u32>;
        type wrap: crate::MaybeUpdateValueWithState<str>;
    }
}
pub use trait_types::Types;
pub use trait_types::Types as ValidTypes;
pub mod data_struct {
    #[non_exhaustive]
    pub struct HtmlTextAreaElementProps<TypeDefs: super::Types + ?::core::marker::Sized> {
        pub HtmlElementProps: super::super::HtmlElementProps::Data<TypeDefs::HtmlElementProps>,
        pub auto_complete: TypeDefs::auto_complete,
        pub auto_correct: TypeDefs::auto_correct,
        pub cols: TypeDefs::cols,
        pub disabled: TypeDefs::disabled,
        pub form: TypeDefs::form,
        pub max_length: TypeDefs::max_length,
        pub min_length: TypeDefs::min_length,
        pub name: TypeDefs::name,
        pub placeholder: TypeDefs::placeholder,
        pub read_only: TypeDefs::read_only,
        pub required: TypeDefs::required,
        pub rows: TypeDefs::rows,
        pub wrap: TypeDefs::wrap,
    }
}
pub use ::core::convert::identity as Building;
pub use ::core::convert::identity as build;
pub use data_struct::HtmlTextAreaElementProps as Data;
pub use data_struct::HtmlTextAreaElementProps as Building;
pub struct Replacing<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
mod types_initial {
    #[allow(unused_imports)]
    use super::super::*;
    pub type TypesInitial = dyn super::Types<
        HtmlElementProps = HtmlElementProps::TypesInitial,
        auto_complete = (),
        auto_correct = (),
        cols = (),
        disabled = (),
        form = (),
        max_length = (),
        min_length = (),
        name = (),
        placeholder = (),
        read_only = (),
        required = (),
        rows = (),
        wrap = (),
    >;
}
pub use types_initial::TypesInitial;
pub type DataInitial = Data<TypesInitial>;
#[cfg(feature = "dom")]
pub mod render_state {
    #[allow(non_camel_case_types)]
    pub trait RenderStateTypes {
        type HtmlElementProps: crate::props::IntrinsicComponentPollReactive;
        type auto_complete;
        type auto_correct;
        type cols;
        type disabled;
        type form;
        type max_length;
        type min_length;
        type name;
        type placeholder;
        type read_only;
        type required;
        type rows;
        type wrap;
    }
    pub struct RenderState<TypeDefs: RenderStateTypes>
    where
        TypeDefs: ?::core::marker::Sized,
    {
        pub HtmlElementProps: TypeDefs::HtmlElementProps,
        pub auto_complete: TypeDefs::auto_complete,
        pub auto_correct: TypeDefs::auto_correct,
        pub cols: TypeDefs::cols,
        pub disabled: TypeDefs::disabled,
        pub form: TypeDefs::form,
        pub max_length: TypeDefs::max_length,
        pub min_length: TypeDefs::min_length,
        pub name: TypeDefs::name,
        pub placeholder: TypeDefs::placeholder,
        pub read_only: TypeDefs::read_only,
        pub required: TypeDefs::required,
        pub rows: TypeDefs::rows,
        pub wrap: TypeDefs::wrap,
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
        pub auto_complete: &'__pin mut (TypeDefs::auto_complete),
        pub auto_correct: &'__pin mut (TypeDefs::auto_correct),
        pub cols: &'__pin mut (TypeDefs::cols),
        pub disabled: &'__pin mut (TypeDefs::disabled),
        pub form: &'__pin mut (TypeDefs::form),
        pub max_length: &'__pin mut (TypeDefs::max_length),
        pub min_length: &'__pin mut (TypeDefs::min_length),
        pub name: &'__pin mut (TypeDefs::name),
        pub placeholder: &'__pin mut (TypeDefs::placeholder),
        pub read_only: &'__pin mut (TypeDefs::read_only),
        pub required: &'__pin mut (TypeDefs::required),
        pub rows: &'__pin mut (TypeDefs::rows),
        pub wrap: &'__pin mut (TypeDefs::wrap),
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
            pub auto_complete: &'__pin (TypeDefs::auto_complete),
            pub auto_correct: &'__pin (TypeDefs::auto_correct),
            pub cols: &'__pin (TypeDefs::cols),
            pub disabled: &'__pin (TypeDefs::disabled),
            pub form: &'__pin (TypeDefs::form),
            pub max_length: &'__pin (TypeDefs::max_length),
            pub min_length: &'__pin (TypeDefs::min_length),
            pub name: &'__pin (TypeDefs::name),
            pub placeholder: &'__pin (TypeDefs::placeholder),
            pub read_only: &'__pin (TypeDefs::read_only),
            pub required: &'__pin (TypeDefs::required),
            pub rows: &'__pin (TypeDefs::rows),
            pub wrap: &'__pin (TypeDefs::wrap),
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
                        auto_complete,
                        auto_correct,
                        cols,
                        disabled,
                        form,
                        max_length,
                        min_length,
                        name,
                        placeholder,
                        read_only,
                        required,
                        rows,
                        wrap,
                    } = self.get_unchecked_mut();
                    RenderStateProj {
                        HtmlElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            HtmlElementProps,
                        ),
                        auto_complete: auto_complete,
                        auto_correct: auto_correct,
                        cols: cols,
                        disabled: disabled,
                        form: form,
                        max_length: max_length,
                        min_length: min_length,
                        name: name,
                        placeholder: placeholder,
                        read_only: read_only,
                        required: required,
                        rows: rows,
                        wrap: wrap,
                    }
                }
            }
            pub(crate) fn project_ref<'__pin>(
                self: ::pin_project_lite::__private::Pin<&'__pin Self>,
            ) -> ProjectionRef<'__pin, TypeDefs> {
                unsafe {
                    let Self {
                        HtmlElementProps,
                        auto_complete,
                        auto_correct,
                        cols,
                        disabled,
                        form,
                        max_length,
                        min_length,
                        name,
                        placeholder,
                        read_only,
                        required,
                        rows,
                        wrap,
                    } = self.get_ref();
                    ProjectionRef {
                        HtmlElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            HtmlElementProps,
                        ),
                        auto_complete: auto_complete,
                        auto_correct: auto_correct,
                        cols: cols,
                        disabled: disabled,
                        form: form,
                        max_length: max_length,
                        min_length: min_length,
                        name: name,
                        placeholder: placeholder,
                        read_only: read_only,
                        required: required,
                        rows: rows,
                        wrap: wrap,
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
            auto_complete: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::auto_complete>,
            auto_correct: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::auto_correct>,
            cols: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::cols>,
            disabled: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::disabled>,
            form: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::form>,
            max_length: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::max_length>,
            min_length: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::min_length>,
            name: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::name>,
            placeholder: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::placeholder>,
            read_only: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::read_only>,
            required: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::required>,
            rows: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::rows>,
            wrap: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::wrap>,
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
            let _ = &this.auto_complete;
            let _ = &this.auto_correct;
            let _ = &this.cols;
            let _ = &this.disabled;
            let _ = &this.form;
            let _ = &this.max_length;
            let _ = &this.min_length;
            let _ = &this.name;
            let _ = &this.placeholder;
            let _ = &this.read_only;
            let _ = &this.required;
            let _ = &this.rows;
            let _ = &this.wrap;
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
        crate::props::IntrinsicComponentPollReactive for RenderState<TypeDefs>
    {
        #[inline]
        fn intrinsic_component_poll_reactive(
            self: ::core::pin::Pin<&mut Self>,
            cx: &mut ::core::task::Context<'_>,
        ) -> ::core::task::Poll<bool> {
            crate::props::IntrinsicComponentPollReactive::intrinsic_component_poll_reactive(
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        ///See [`HtmlElementProps::class`]
        #[inline(always)]
        pub fn class<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            class: V,
        ) -> super::Building<super::overwrite::class<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.class(class),
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        ///See [`HtmlElementProps::id`]
        #[inline(always)]
        pub fn id<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            id: V,
        ) -> super::Building<super::overwrite::id<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.id(id),
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        ///See [`HtmlElementProps::part`]
        #[inline(always)]
        pub fn part<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            part: V,
        ) -> super::Building<super::overwrite::part<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.part(part),
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        ///See [`HtmlElementProps::access_key`]
        #[inline(always)]
        pub fn access_key<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            access_key: V,
        ) -> super::Building<super::overwrite::access_key<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.access_key(access_key),
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        ///See [`HtmlElementProps::auto_capitalize`]
        #[inline(always)]
        pub fn auto_capitalize<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            auto_capitalize: V,
        ) -> super::Building<super::overwrite::auto_capitalize<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.auto_capitalize(auto_capitalize),
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        ///See [`HtmlElementProps::auto_focus`]
        #[inline(always)]
        pub fn auto_focus<V: crate::MaybeUpdateValueWithState<bool>>(
            self,
            auto_focus: V,
        ) -> super::Building<super::overwrite::auto_focus<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.auto_focus(auto_focus),
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        ///See [`HtmlElementProps::content_editable`]
        #[inline(always)]
        pub fn content_editable<V: crate::props::MaybeInherit<bool>>(
            self,
            content_editable: V,
        ) -> super::Building<super::overwrite::content_editable<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.content_editable(content_editable),
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        ///See [`HtmlElementProps::context_menu`]
        #[inline(always)]
        pub fn context_menu<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            context_menu: V,
        ) -> super::Building<super::overwrite::context_menu<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.context_menu(context_menu),
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        ///See [`HtmlElementProps::dir`]
        #[inline(always)]
        pub fn dir<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            dir: V,
        ) -> super::Building<super::overwrite::dir<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.dir(dir),
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        ///See [`HtmlElementProps::draggable`]
        #[inline(always)]
        pub fn draggable<V: crate::MaybeUpdateValueWithState<bool>>(
            self,
            draggable: V,
        ) -> super::Building<super::overwrite::draggable<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.draggable(draggable),
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        ///See [`HtmlElementProps::enter_key_hint`]
        #[inline(always)]
        pub fn enter_key_hint<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            enter_key_hint: V,
        ) -> super::Building<super::overwrite::enter_key_hint<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.enter_key_hint(enter_key_hint),
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        ///See [`HtmlElementProps::hidden`]
        #[inline(always)]
        pub fn hidden<V: crate::MaybeUpdateValueWithState<bool>>(
            self,
            hidden: V,
        ) -> super::Building<super::overwrite::hidden<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.hidden(hidden),
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        ///See [`HtmlElementProps::inert`]
        #[inline(always)]
        pub fn inert<V: crate::MaybeUpdateValueWithState<bool>>(
            self,
            inert: V,
        ) -> super::Building<super::overwrite::inert<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.inert(inert),
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        ///See [`HtmlElementProps::input_mode`]
        #[inline(always)]
        pub fn input_mode<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            input_mode: V,
        ) -> super::Building<super::overwrite::input_mode<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.input_mode(input_mode),
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        ///See [`HtmlElementProps::is`]
        #[inline(always)]
        pub fn is<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            is: V,
        ) -> super::Building<super::overwrite::is<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.is(is),
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        ///See [`HtmlElementProps::item_id`]
        #[inline(always)]
        pub fn item_id<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            item_id: V,
        ) -> super::Building<super::overwrite::item_id<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.item_id(item_id),
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        ///See [`HtmlElementProps::item_prop`]
        #[inline(always)]
        pub fn item_prop<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            item_prop: V,
        ) -> super::Building<super::overwrite::item_prop<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.item_prop(item_prop),
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        ///See [`HtmlElementProps::item_ref`]
        #[inline(always)]
        pub fn item_ref<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            item_ref: V,
        ) -> super::Building<super::overwrite::item_ref<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.item_ref(item_ref),
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        ///See [`HtmlElementProps::item_scope`]
        #[inline(always)]
        pub fn item_scope<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            item_scope: V,
        ) -> super::Building<super::overwrite::item_scope<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.item_scope(item_scope),
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        ///See [`HtmlElementProps::item_type`]
        #[inline(always)]
        pub fn item_type<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            item_type: V,
        ) -> super::Building<super::overwrite::item_type<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.item_type(item_type),
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        ///See [`HtmlElementProps::lang`]
        #[inline(always)]
        pub fn lang<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            lang: V,
        ) -> super::Building<super::overwrite::lang<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.lang(lang),
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        ///See [`HtmlElementProps::nonce`]
        #[inline(always)]
        pub fn nonce<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            nonce: V,
        ) -> super::Building<super::overwrite::nonce<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.nonce(nonce),
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        ///See [`HtmlElementProps::role`]
        #[inline(always)]
        pub fn role<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            role: V,
        ) -> super::Building<super::overwrite::role<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.role(role),
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        ///See [`HtmlElementProps::slot`]
        #[inline(always)]
        pub fn slot<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            slot: V,
        ) -> super::Building<super::overwrite::slot<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.slot(slot),
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        ///See [`HtmlElementProps::spellcheck`]
        #[inline(always)]
        pub fn spellcheck<V: crate::MaybeUpdateValueWithState<bool>>(
            self,
            spellcheck: V,
        ) -> super::Building<super::overwrite::spellcheck<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.spellcheck(spellcheck),
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        ///See [`HtmlElementProps::style`]
        #[inline(always)]
        pub fn style<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            style: V,
        ) -> super::Building<super::overwrite::style<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.style(style),
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        ///See [`HtmlElementProps::tab_index`]
        #[inline(always)]
        pub fn tab_index<V: crate::MaybeUpdateValueWithState<i32>>(
            self,
            tab_index: V,
        ) -> super::Building<super::overwrite::tab_index<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.tab_index(tab_index),
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        ///See [`HtmlElementProps::title`]
        #[inline(always)]
        pub fn title<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            title: V,
        ) -> super::Building<super::overwrite::title<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.title(title),
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        ///See [`HtmlElementProps::translate`]
        #[inline(always)]
        pub fn translate<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            translate: V,
        ) -> super::Building<super::overwrite::translate<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.translate(translate),
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        ///See [`HtmlElementProps::virtual_keyboard_policy`]
        #[inline(always)]
        pub fn virtual_keyboard_policy<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            virtual_keyboard_policy: V,
        ) -> super::Building<super::overwrite::virtual_keyboard_policy<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self
                    .HtmlElementProps
                    .virtual_keyboard_policy(virtual_keyboard_policy),
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
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
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        #[inline(always)]
        pub fn auto_complete<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            auto_complete: V,
        ) -> super::Building<super::overwrite::auto_complete<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        #[inline(always)]
        pub fn auto_correct<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            auto_correct: V,
        ) -> super::Building<super::overwrite::auto_correct<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_complete: self.auto_complete,
                auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        #[inline(always)]
        pub fn cols<V: crate::MaybeUpdateValueWithState<u32>>(
            self,
            cols: V,
        ) -> super::Building<super::overwrite::cols<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        #[inline(always)]
        pub fn disabled<V: crate::MaybeUpdateValueWithState<bool>>(
            self,
            disabled: V,
        ) -> super::Building<super::overwrite::disabled<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        #[inline(always)]
        pub fn form<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            form: V,
        ) -> super::Building<super::overwrite::form<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        #[inline(always)]
        pub fn max_length<V: crate::MaybeUpdateValueWithState<i32>>(
            self,
            max_length: V,
        ) -> super::Building<super::overwrite::max_length<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        #[inline(always)]
        pub fn min_length<V: crate::MaybeUpdateValueWithState<i32>>(
            self,
            min_length: V,
        ) -> super::Building<super::overwrite::min_length<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        #[inline(always)]
        pub fn name<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            name: V,
        ) -> super::Building<super::overwrite::name<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        #[inline(always)]
        pub fn placeholder<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            placeholder: V,
        ) -> super::Building<super::overwrite::placeholder<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        #[inline(always)]
        pub fn read_only<V: crate::MaybeUpdateValueWithState<bool>>(
            self,
            read_only: V,
        ) -> super::Building<super::overwrite::read_only<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only,
                required: self.required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        #[inline(always)]
        pub fn required<V: crate::MaybeUpdateValueWithState<bool>>(
            self,
            required: V,
        ) -> super::Building<super::overwrite::required<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required,
                rows: self.rows,
                wrap: self.wrap,
            }
        }
        #[inline(always)]
        pub fn rows<V: crate::MaybeUpdateValueWithState<u32>>(
            self,
            rows: V,
        ) -> super::Building<super::overwrite::rows<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows,
                wrap: self.wrap,
            }
        }
        #[inline(always)]
        pub fn wrap<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            wrap: V,
        ) -> super::Building<super::overwrite::wrap<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                auto_complete: self.auto_complete,
                auto_correct: self.auto_correct,
                cols: self.cols,
                disabled: self.disabled,
                form: self.form,
                max_length: self.max_length,
                min_length: self.min_length,
                name: self.name,
                placeholder: self.placeholder,
                read_only: self.read_only,
                required: self.required,
                rows: self.rows,
                wrap,
            }
        }
    }
}
#[cfg(feature = "dom")]
mod impl_update_element {
    #[allow(unused_imports)]
    use super::super::*;
    impl<TypeDefs: ?::core::marker::Sized + super::Types>
        crate::props::UpdateElement<web_sys::HtmlTextAreaElement> for super::Data<TypeDefs>
    where
        HtmlElementProps::Data<TypeDefs::HtmlElementProps>:
            crate::props::UpdateElement<web_sys::HtmlElement>,
    {
        type State = super::render_state::RenderState<
            dyn super::render_state::RenderStateTypes<
                HtmlElementProps = <HtmlElementProps::Data<
                    TypeDefs::HtmlElementProps,
                > as crate::props::UpdateElement<web_sys::HtmlElement>>::State,
                auto_complete = <TypeDefs::auto_complete as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                auto_correct = <TypeDefs::auto_correct as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                cols = <TypeDefs::cols as ::frender_dom::props::MaybeUpdateValueWithState<
                    u32,
                >>::State,
                disabled = <TypeDefs::disabled as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::State,
                form = <TypeDefs::form as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                max_length = <TypeDefs::max_length as ::frender_dom::props::MaybeUpdateValueWithState<
                    i32,
                >>::State,
                min_length = <TypeDefs::min_length as ::frender_dom::props::MaybeUpdateValueWithState<
                    i32,
                >>::State,
                name = <TypeDefs::name as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                placeholder = <TypeDefs::placeholder as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                read_only = <TypeDefs::read_only as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::State,
                required = <TypeDefs::required as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::State,
                rows = <TypeDefs::rows as ::frender_dom::props::MaybeUpdateValueWithState<
                    u32,
                >>::State,
                wrap = <TypeDefs::wrap as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
            >,
        >;
        fn initialize_state(
            this: Self,
            element: &web_sys::HtmlTextAreaElement,
            children_ctx: &mut ::frender_dom::Dom,
        ) -> Self::State {
            let dom_element: &::web_sys::Element = element.as_ref();
            super::render_state::RenderState {
                HtmlElementProps: <HtmlElementProps::Data<
                    TypeDefs::HtmlElementProps,
                > as crate::props::UpdateElement<
                    web_sys::HtmlElement,
                >>::initialize_state(this.HtmlElementProps, element, children_ctx),
                auto_complete: <TypeDefs::auto_complete as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.auto_complete,
                    |v| element.set_autocomplete(v),
                    || dom_element.remove_attribute("autocomplete").unwrap(),
                ),
                auto_correct: <TypeDefs::auto_correct as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.auto_correct,
                    |v| crate::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "auto_correct",
                    ),
                    || dom_element.remove_attribute("auto_correct").unwrap(),
                ),
                cols: <TypeDefs::cols as ::frender_dom::props::MaybeUpdateValueWithState<
                    u32,
                >>::initialize_state_and_update(
                    this.cols,
                    |v| element.set_cols(*v),
                    || dom_element.remove_attribute("cols").unwrap(),
                ),
                disabled: <TypeDefs::disabled as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.disabled,
                    |v| element.set_disabled(*v),
                    || dom_element.remove_attribute("disabled").unwrap(),
                ),
                form: <TypeDefs::form as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.form,
                    |v| crate::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "form",
                    ),
                    || dom_element.remove_attribute("form").unwrap(),
                ),
                max_length: <TypeDefs::max_length as ::frender_dom::props::MaybeUpdateValueWithState<
                    i32,
                >>::initialize_state_and_update(
                    this.max_length,
                    |v| element.set_max_length(*v),
                    || dom_element.remove_attribute("maxlength").unwrap(),
                ),
                min_length: <TypeDefs::min_length as ::frender_dom::props::MaybeUpdateValueWithState<
                    i32,
                >>::initialize_state_and_update(
                    this.min_length,
                    |v| element.set_min_length(*v),
                    || dom_element.remove_attribute("minlength").unwrap(),
                ),
                name: <TypeDefs::name as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.name,
                    |v| element.set_name(v),
                    || dom_element.remove_attribute("name").unwrap(),
                ),
                placeholder: <TypeDefs::placeholder as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.placeholder,
                    |v| element.set_placeholder(v),
                    || dom_element.remove_attribute("placeholder").unwrap(),
                ),
                read_only: <TypeDefs::read_only as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.read_only,
                    |v| element.set_read_only(*v),
                    || dom_element.remove_attribute("readonly").unwrap(),
                ),
                required: <TypeDefs::required as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.required,
                    |v| element.set_required(*v),
                    || dom_element.remove_attribute("required").unwrap(),
                ),
                rows: <TypeDefs::rows as ::frender_dom::props::MaybeUpdateValueWithState<
                    u32,
                >>::initialize_state_and_update(
                    this.rows,
                    |v| element.set_rows(*v),
                    || dom_element.remove_attribute("rows").unwrap(),
                ),
                wrap: <TypeDefs::wrap as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.wrap,
                    |v| element.set_wrap(v),
                    || dom_element.remove_attribute("wrap").unwrap(),
                ),
            }
        }
        fn update_element(
            this: Self,
            element: &web_sys::HtmlTextAreaElement,
            children_ctx: &mut ::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let state = state.pin_project();
            let dom_element: &::web_sys::Element = element.as_ref();
            crate::props::UpdateElement::update_element(
                this.HtmlElementProps,
                element.as_ref(),
                children_ctx,
                state.HtmlElementProps,
            );
            <TypeDefs::auto_complete as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.auto_complete,
                state.auto_complete,
                |v| element.set_autocomplete(v),
                || dom_element.remove_attribute("autocomplete").unwrap(),
            );
            <TypeDefs::auto_correct as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.auto_correct,
                state.auto_correct,
                |v| crate::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "auto_correct",
                ),
                || dom_element.remove_attribute("auto_correct").unwrap(),
            );
            <TypeDefs::cols as ::frender_dom::props::MaybeUpdateValueWithState<
                u32,
            >>::maybe_update_value_with_state(
                this.cols,
                state.cols,
                |v| element.set_cols(*v),
                || dom_element.remove_attribute("cols").unwrap(),
            );
            <TypeDefs::disabled as ::frender_dom::props::MaybeUpdateValueWithState<
                bool,
            >>::maybe_update_value_with_state(
                this.disabled,
                state.disabled,
                |v| element.set_disabled(*v),
                || dom_element.remove_attribute("disabled").unwrap(),
            );
            <TypeDefs::form as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.form,
                state.form,
                |v| crate::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "form",
                ),
                || dom_element.remove_attribute("form").unwrap(),
            );
            <TypeDefs::max_length as ::frender_dom::props::MaybeUpdateValueWithState<
                i32,
            >>::maybe_update_value_with_state(
                this.max_length,
                state.max_length,
                |v| element.set_max_length(*v),
                || dom_element.remove_attribute("maxlength").unwrap(),
            );
            <TypeDefs::min_length as ::frender_dom::props::MaybeUpdateValueWithState<
                i32,
            >>::maybe_update_value_with_state(
                this.min_length,
                state.min_length,
                |v| element.set_min_length(*v),
                || dom_element.remove_attribute("minlength").unwrap(),
            );
            <TypeDefs::name as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.name,
                state.name,
                |v| element.set_name(v),
                || dom_element.remove_attribute("name").unwrap(),
            );
            <TypeDefs::placeholder as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.placeholder,
                state.placeholder,
                |v| element.set_placeholder(v),
                || dom_element.remove_attribute("placeholder").unwrap(),
            );
            <TypeDefs::read_only as ::frender_dom::props::MaybeUpdateValueWithState<
                bool,
            >>::maybe_update_value_with_state(
                this.read_only,
                state.read_only,
                |v| element.set_read_only(*v),
                || dom_element.remove_attribute("readonly").unwrap(),
            );
            <TypeDefs::required as ::frender_dom::props::MaybeUpdateValueWithState<
                bool,
            >>::maybe_update_value_with_state(
                this.required,
                state.required,
                |v| element.set_required(*v),
                || dom_element.remove_attribute("required").unwrap(),
            );
            <TypeDefs::rows as ::frender_dom::props::MaybeUpdateValueWithState<
                u32,
            >>::maybe_update_value_with_state(
                this.rows,
                state.rows,
                |v| element.set_rows(*v),
                || dom_element.remove_attribute("rows").unwrap(),
            );
            <TypeDefs::wrap as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.wrap,
                state.wrap,
                |v| element.set_wrap(v),
                || dom_element.remove_attribute("wrap").unwrap(),
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
                                <TypeDefs::auto_complete as ::frender_dom::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.auto_complete)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("autocomplete"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::auto_correct as ::frender_dom::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.auto_correct)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("auto_correct"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::cols as ::frender_dom::props::MaybeUpdateValueWithState<
                                    u32,
                                >>::maybe_into_html_attribute_value(this.cols)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("cols"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::disabled as ::frender_dom::props::MaybeUpdateValueWithState<
                                    bool,
                                >>::maybe_into_html_attribute_value(this.disabled)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("disabled"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::form as ::frender_dom::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.form)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("form"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::max_length as ::frender_dom::props::MaybeUpdateValueWithState<
                                    i32,
                                >>::maybe_into_html_attribute_value(this.max_length)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("maxlength"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::min_length as ::frender_dom::props::MaybeUpdateValueWithState<
                                    i32,
                                >>::maybe_into_html_attribute_value(this.min_length)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("minlength"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::name as ::frender_dom::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.name)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("name"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::placeholder as ::frender_dom::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.placeholder)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("placeholder"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::read_only as ::frender_dom::props::MaybeUpdateValueWithState<
                                    bool,
                                >>::maybe_into_html_attribute_value(this.read_only)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("readonly"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::required as ::frender_dom::props::MaybeUpdateValueWithState<
                                    bool,
                                >>::maybe_into_html_attribute_value(this.required)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("required"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::rows as ::frender_dom::props::MaybeUpdateValueWithState<
                                    u32,
                                >>::maybe_into_html_attribute_value(this.rows)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("rows"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::wrap as ::frender_dom::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.wrap)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("wrap"),
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
