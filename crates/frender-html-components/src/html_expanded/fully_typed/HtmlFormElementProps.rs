#[allow(non_snake_case)]
#[inline(always)]
pub fn HtmlFormElementProps() -> Building<TypesInitial> {
    #[allow(unused_imports)]
    use super::*;
    self::Building {
        HtmlElementProps: HtmlElementProps::build(HtmlElementProps()),
        accept: (),
        accept_charset: (),
        auto_complete: (),
        name: (),
        rel: (),
        action: (),
        enc_type: (),
        method: (),
        no_validate: (),
        target: (),
        on_form_data: (),
        on_reset: (),
        on_submit: (),
    }
}
pub mod prelude {}
pub mod overwrite {
    #![allow(non_camel_case_types)]
    pub type HtmlElementProps<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = Value,
        accept = <TypeDefs as super::Types>::accept,
        accept_charset = <TypeDefs as super::Types>::accept_charset,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        name = <TypeDefs as super::Types>::name,
        rel = <TypeDefs as super::Types>::rel,
        action = <TypeDefs as super::Types>::action,
        enc_type = <TypeDefs as super::Types>::enc_type,
        method = <TypeDefs as super::Types>::method,
        no_validate = <TypeDefs as super::Types>::no_validate,
        target = <TypeDefs as super::Types>::target,
        on_form_data = <TypeDefs as super::Types>::on_form_data,
        on_reset = <TypeDefs as super::Types>::on_reset,
        on_submit = <TypeDefs as super::Types>::on_submit,
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
    pub type accept<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = Value,
        accept_charset = <TypeDefs as super::Types>::accept_charset,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        name = <TypeDefs as super::Types>::name,
        rel = <TypeDefs as super::Types>::rel,
        action = <TypeDefs as super::Types>::action,
        enc_type = <TypeDefs as super::Types>::enc_type,
        method = <TypeDefs as super::Types>::method,
        no_validate = <TypeDefs as super::Types>::no_validate,
        target = <TypeDefs as super::Types>::target,
        on_form_data = <TypeDefs as super::Types>::on_form_data,
        on_reset = <TypeDefs as super::Types>::on_reset,
        on_submit = <TypeDefs as super::Types>::on_submit,
    >;
    pub type accept_charset<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        accept_charset = Value,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        name = <TypeDefs as super::Types>::name,
        rel = <TypeDefs as super::Types>::rel,
        action = <TypeDefs as super::Types>::action,
        enc_type = <TypeDefs as super::Types>::enc_type,
        method = <TypeDefs as super::Types>::method,
        no_validate = <TypeDefs as super::Types>::no_validate,
        target = <TypeDefs as super::Types>::target,
        on_form_data = <TypeDefs as super::Types>::on_form_data,
        on_reset = <TypeDefs as super::Types>::on_reset,
        on_submit = <TypeDefs as super::Types>::on_submit,
    >;
    pub type auto_complete<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        accept_charset = <TypeDefs as super::Types>::accept_charset,
        auto_complete = Value,
        name = <TypeDefs as super::Types>::name,
        rel = <TypeDefs as super::Types>::rel,
        action = <TypeDefs as super::Types>::action,
        enc_type = <TypeDefs as super::Types>::enc_type,
        method = <TypeDefs as super::Types>::method,
        no_validate = <TypeDefs as super::Types>::no_validate,
        target = <TypeDefs as super::Types>::target,
        on_form_data = <TypeDefs as super::Types>::on_form_data,
        on_reset = <TypeDefs as super::Types>::on_reset,
        on_submit = <TypeDefs as super::Types>::on_submit,
    >;
    pub type name<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        accept_charset = <TypeDefs as super::Types>::accept_charset,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        name = Value,
        rel = <TypeDefs as super::Types>::rel,
        action = <TypeDefs as super::Types>::action,
        enc_type = <TypeDefs as super::Types>::enc_type,
        method = <TypeDefs as super::Types>::method,
        no_validate = <TypeDefs as super::Types>::no_validate,
        target = <TypeDefs as super::Types>::target,
        on_form_data = <TypeDefs as super::Types>::on_form_data,
        on_reset = <TypeDefs as super::Types>::on_reset,
        on_submit = <TypeDefs as super::Types>::on_submit,
    >;
    pub type rel<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        accept_charset = <TypeDefs as super::Types>::accept_charset,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        name = <TypeDefs as super::Types>::name,
        rel = Value,
        action = <TypeDefs as super::Types>::action,
        enc_type = <TypeDefs as super::Types>::enc_type,
        method = <TypeDefs as super::Types>::method,
        no_validate = <TypeDefs as super::Types>::no_validate,
        target = <TypeDefs as super::Types>::target,
        on_form_data = <TypeDefs as super::Types>::on_form_data,
        on_reset = <TypeDefs as super::Types>::on_reset,
        on_submit = <TypeDefs as super::Types>::on_submit,
    >;
    pub type action<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        accept_charset = <TypeDefs as super::Types>::accept_charset,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        name = <TypeDefs as super::Types>::name,
        rel = <TypeDefs as super::Types>::rel,
        action = Value,
        enc_type = <TypeDefs as super::Types>::enc_type,
        method = <TypeDefs as super::Types>::method,
        no_validate = <TypeDefs as super::Types>::no_validate,
        target = <TypeDefs as super::Types>::target,
        on_form_data = <TypeDefs as super::Types>::on_form_data,
        on_reset = <TypeDefs as super::Types>::on_reset,
        on_submit = <TypeDefs as super::Types>::on_submit,
    >;
    pub type enc_type<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        accept_charset = <TypeDefs as super::Types>::accept_charset,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        name = <TypeDefs as super::Types>::name,
        rel = <TypeDefs as super::Types>::rel,
        action = <TypeDefs as super::Types>::action,
        enc_type = Value,
        method = <TypeDefs as super::Types>::method,
        no_validate = <TypeDefs as super::Types>::no_validate,
        target = <TypeDefs as super::Types>::target,
        on_form_data = <TypeDefs as super::Types>::on_form_data,
        on_reset = <TypeDefs as super::Types>::on_reset,
        on_submit = <TypeDefs as super::Types>::on_submit,
    >;
    pub type method<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        accept_charset = <TypeDefs as super::Types>::accept_charset,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        name = <TypeDefs as super::Types>::name,
        rel = <TypeDefs as super::Types>::rel,
        action = <TypeDefs as super::Types>::action,
        enc_type = <TypeDefs as super::Types>::enc_type,
        method = Value,
        no_validate = <TypeDefs as super::Types>::no_validate,
        target = <TypeDefs as super::Types>::target,
        on_form_data = <TypeDefs as super::Types>::on_form_data,
        on_reset = <TypeDefs as super::Types>::on_reset,
        on_submit = <TypeDefs as super::Types>::on_submit,
    >;
    pub type no_validate<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        accept_charset = <TypeDefs as super::Types>::accept_charset,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        name = <TypeDefs as super::Types>::name,
        rel = <TypeDefs as super::Types>::rel,
        action = <TypeDefs as super::Types>::action,
        enc_type = <TypeDefs as super::Types>::enc_type,
        method = <TypeDefs as super::Types>::method,
        no_validate = Value,
        target = <TypeDefs as super::Types>::target,
        on_form_data = <TypeDefs as super::Types>::on_form_data,
        on_reset = <TypeDefs as super::Types>::on_reset,
        on_submit = <TypeDefs as super::Types>::on_submit,
    >;
    pub type target<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        accept_charset = <TypeDefs as super::Types>::accept_charset,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        name = <TypeDefs as super::Types>::name,
        rel = <TypeDefs as super::Types>::rel,
        action = <TypeDefs as super::Types>::action,
        enc_type = <TypeDefs as super::Types>::enc_type,
        method = <TypeDefs as super::Types>::method,
        no_validate = <TypeDefs as super::Types>::no_validate,
        target = Value,
        on_form_data = <TypeDefs as super::Types>::on_form_data,
        on_reset = <TypeDefs as super::Types>::on_reset,
        on_submit = <TypeDefs as super::Types>::on_submit,
    >;
    pub type on_form_data<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        accept_charset = <TypeDefs as super::Types>::accept_charset,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        name = <TypeDefs as super::Types>::name,
        rel = <TypeDefs as super::Types>::rel,
        action = <TypeDefs as super::Types>::action,
        enc_type = <TypeDefs as super::Types>::enc_type,
        method = <TypeDefs as super::Types>::method,
        no_validate = <TypeDefs as super::Types>::no_validate,
        target = <TypeDefs as super::Types>::target,
        on_form_data = Value,
        on_reset = <TypeDefs as super::Types>::on_reset,
        on_submit = <TypeDefs as super::Types>::on_submit,
    >;
    pub type on_reset<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        accept_charset = <TypeDefs as super::Types>::accept_charset,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        name = <TypeDefs as super::Types>::name,
        rel = <TypeDefs as super::Types>::rel,
        action = <TypeDefs as super::Types>::action,
        enc_type = <TypeDefs as super::Types>::enc_type,
        method = <TypeDefs as super::Types>::method,
        no_validate = <TypeDefs as super::Types>::no_validate,
        target = <TypeDefs as super::Types>::target,
        on_form_data = <TypeDefs as super::Types>::on_form_data,
        on_reset = Value,
        on_submit = <TypeDefs as super::Types>::on_submit,
    >;
    pub type on_submit<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        accept_charset = <TypeDefs as super::Types>::accept_charset,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        name = <TypeDefs as super::Types>::name,
        rel = <TypeDefs as super::Types>::rel,
        action = <TypeDefs as super::Types>::action,
        enc_type = <TypeDefs as super::Types>::enc_type,
        method = <TypeDefs as super::Types>::method,
        no_validate = <TypeDefs as super::Types>::no_validate,
        target = <TypeDefs as super::Types>::target,
        on_form_data = <TypeDefs as super::Types>::on_form_data,
        on_reset = <TypeDefs as super::Types>::on_reset,
        on_submit = Value,
    >;
}
mod trait_types {
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub trait Types {
        type HtmlElementProps: ?::core::marker::Sized + HtmlElementProps::Types;
        type accept: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>;
        type accept_charset: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>;
        type auto_complete: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>;
        type name: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>;
        type rel: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>;
        type action: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>;
        type enc_type: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>;
        type method: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>;
        type no_validate: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>;
        type target: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>;
        type on_form_data;
        type on_reset;
        type on_submit;
    }
}
pub use trait_types::Types;
pub use trait_types::Types as ValidTypes;
pub mod data_struct {
    #[non_exhaustive]
    pub struct HtmlFormElementProps<TypeDefs: super::Types + ?::core::marker::Sized> {
        pub HtmlElementProps: super::super::HtmlElementProps::Data<TypeDefs::HtmlElementProps>,
        pub accept: TypeDefs::accept,
        pub accept_charset: TypeDefs::accept_charset,
        pub auto_complete: TypeDefs::auto_complete,
        pub name: TypeDefs::name,
        pub rel: TypeDefs::rel,
        pub action: TypeDefs::action,
        pub enc_type: TypeDefs::enc_type,
        pub method: TypeDefs::method,
        pub no_validate: TypeDefs::no_validate,
        pub target: TypeDefs::target,
        pub on_form_data: TypeDefs::on_form_data,
        pub on_reset: TypeDefs::on_reset,
        pub on_submit: TypeDefs::on_submit,
    }
}
pub use ::core::convert::identity as Building;
pub use ::core::convert::identity as build;
pub use data_struct::HtmlFormElementProps as Data;
pub use data_struct::HtmlFormElementProps as Building;
pub struct Replacing<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
mod types_initial {
    #[allow(unused_imports)]
    use super::super::*;
    pub type TypesInitial = dyn super::Types<
        HtmlElementProps = HtmlElementProps::TypesInitial,
        accept = (),
        accept_charset = (),
        auto_complete = (),
        name = (),
        rel = (),
        action = (),
        enc_type = (),
        method = (),
        no_validate = (),
        target = (),
        on_form_data = (),
        on_reset = (),
        on_submit = (),
    >;
}
pub use types_initial::TypesInitial;
pub type DataInitial = Data<TypesInitial>;
#[cfg(feature = "csr")]
pub mod render_state {
    #[allow(non_camel_case_types)]
    pub trait RenderStateTypes {
        type HtmlElementProps: crate::imports::frender_csr::props::IntrinsicComponentPollReactive;
        type accept;
        type accept_charset;
        type auto_complete;
        type name;
        type rel;
        type action;
        type enc_type;
        type method;
        type no_validate;
        type target;
        type on_form_data;
        type on_reset;
        type on_submit;
    }
    crate::imports::pin_project! {
        #[project = RenderStateProj] pub struct RenderState < TypeDefs : RenderStateTypes
        > where TypeDefs : ? ::core::marker::Sized { #[pin] pub HtmlElementProps :
        TypeDefs::HtmlElementProps, pub accept : TypeDefs::accept, pub accept_charset :
        TypeDefs::accept_charset, pub auto_complete : TypeDefs::auto_complete, pub name :
        TypeDefs::name, pub rel : TypeDefs::rel, pub action : TypeDefs::action, pub
        enc_type : TypeDefs::enc_type, pub method : TypeDefs::method, pub no_validate :
        TypeDefs::no_validate, pub target : TypeDefs::target, pub on_form_data :
        TypeDefs::on_form_data, pub on_reset : TypeDefs::on_reset, pub on_submit :
        TypeDefs::on_submit, }
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
            }
        }
        ///See [`HtmlElementProps::class`]
        #[inline(always)]
        pub fn class<V: Todo<unimplemented![]>>(
            self,
            class: V,
        ) -> super::Building<super::overwrite::class<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.class(class),
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
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
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
            }
        }
        #[deprecated = "This attribute has been deprecated and should not be used. Instead, use the accept attribute on <input type=file> elements."]
        #[inline(always)]
        pub fn accept<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            accept: V,
        ) -> super::Building<super::overwrite::accept<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
            }
        }
        #[inline(always)]
        pub fn accept_charset<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
        >(
            self,
            accept_charset: V,
        ) -> super::Building<super::overwrite::accept_charset<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                accept: self.accept,
                accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
            }
        }
        #[inline(always)]
        pub fn auto_complete<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
        >(
            self,
            auto_complete: V,
        ) -> super::Building<super::overwrite::auto_complete<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
            }
        }
        #[inline(always)]
        pub fn name<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            name: V,
        ) -> super::Building<super::overwrite::name<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
            }
        }
        #[inline(always)]
        pub fn rel<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            rel: V,
        ) -> super::Building<super::overwrite::rel<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
            }
        }
        #[inline(always)]
        pub fn action<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            action: V,
        ) -> super::Building<super::overwrite::action<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
            }
        }
        #[inline(always)]
        pub fn enc_type<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            enc_type: V,
        ) -> super::Building<super::overwrite::enc_type<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
            }
        }
        #[inline(always)]
        pub fn method<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            method: V,
        ) -> super::Building<super::overwrite::method<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
            }
        }
        #[inline(always)]
        pub fn no_validate<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
        >(
            self,
            no_validate: V,
        ) -> super::Building<super::overwrite::no_validate<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
            }
        }
        #[inline(always)]
        pub fn target<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            target: V,
        ) -> super::Building<super::overwrite::target<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
            }
        }
        /// Event [`formdata`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/formdata_event)
        ///
        /// The `formdata` event fires after the entry list representing the form's data is constructed.
        #[inline(always)]
        pub fn on_form_data<V>(
            self,
            on_form_data: V,
        ) -> super::Building<super::overwrite::on_form_data<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data,
                on_reset: self.on_reset,
                on_submit: self.on_submit,
            }
        }
        /// Event [`reset`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/reset_event)
        ///
        /// The `reset` event fires when a form is reset.
        #[inline(always)]
        pub fn on_reset<V>(
            self,
            on_reset: V,
        ) -> super::Building<super::overwrite::on_reset<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset,
                on_submit: self.on_submit,
            }
        }
        /// Event [`submit`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/submit_event)
        ///
        /// The `submit` event fires when a form is submitted.
        #[inline(always)]
        pub fn on_submit<V>(
            self,
            on_submit: V,
        ) -> super::Building<super::overwrite::on_submit<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                accept: self.accept,
                accept_charset: self.accept_charset,
                auto_complete: self.auto_complete,
                name: self.name,
                rel: self.rel,
                action: self.action,
                enc_type: self.enc_type,
                method: self.method,
                no_validate: self.no_validate,
                target: self.target,
                on_form_data: self.on_form_data,
                on_reset: self.on_reset,
                on_submit,
            }
        }
    }
}
#[cfg(feature = "csr")]
mod impl_update_element {
    #[allow(unused_imports)]
    use super::super::*;
    impl<TypeDefs: ?::core::marker::Sized + super::Types>
        crate::imports::frender_csr::props::UpdateElement<web_sys::HtmlFormElement>
        for super::Data<TypeDefs>
    where
        HtmlElementProps::Data<TypeDefs::HtmlElementProps>:
            crate::imports::frender_csr::props::UpdateElement<web_sys::HtmlElement>,
        TypeDefs::on_form_data:
            crate::imports::frender_html::props::UpdateDomEventListener<events::Event>,
        TypeDefs::on_reset:
            crate::imports::frender_html::props::UpdateDomEventListener<events::Event>,
        TypeDefs::on_submit:
            crate::imports::frender_html::props::UpdateDomEventListener<events::Event>,
    {
        type State = super::render_state::RenderState<
            dyn super::render_state::RenderStateTypes<
                HtmlElementProps = <HtmlElementProps::Data<
                    TypeDefs::HtmlElementProps,
                > as crate::imports::frender_csr::props::UpdateElement<
                    web_sys::HtmlElement,
                >>::State,
                accept = <TypeDefs::accept as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                accept_charset = <TypeDefs::accept_charset as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                auto_complete = <TypeDefs::auto_complete as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                name = <TypeDefs::name as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                rel = <TypeDefs::rel as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                action = <TypeDefs::action as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                enc_type = <TypeDefs::enc_type as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                method = <TypeDefs::method as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                no_validate = <TypeDefs::no_validate as ::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::State,
                target = <TypeDefs::target as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                on_form_data = <TypeDefs::on_form_data as crate::imports::frender_html::props::UpdateDomEventListener<
                    events::Event,
                >>::State,
                on_reset = <TypeDefs::on_reset as crate::imports::frender_html::props::UpdateDomEventListener<
                    events::Event,
                >>::State,
                on_submit = <TypeDefs::on_submit as crate::imports::frender_html::props::UpdateDomEventListener<
                    events::Event,
                >>::State,
            >,
        >;
        fn initialize_state(
            this: Self,
            element: &web_sys::HtmlFormElement,
            children_ctx: &mut ::frender_csr::Dom,
        ) -> Self::State {
            let dom_element: &::web_sys::Element = element.as_ref();
            super::render_state::RenderState {
                HtmlElementProps: <HtmlElementProps::Data<
                    TypeDefs::HtmlElementProps,
                > as crate::imports::frender_csr::props::UpdateElement<
                    web_sys::HtmlElement,
                >>::initialize_state(this.HtmlElementProps, element, children_ctx),
                accept: <TypeDefs::accept as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.accept,
                    |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "accept",
                    ),
                    || dom_element.remove_attribute("accept").unwrap(),
                ),
                accept_charset: <TypeDefs::accept_charset as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.accept_charset,
                    |v| dom_element.set_accept_charset(v),
                    || dom_element.remove_attribute("accept-charset").unwrap(),
                ),
                auto_complete: <TypeDefs::auto_complete as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.auto_complete,
                    |v| dom_element.set_autocomplete(v),
                    || dom_element.remove_attribute("autocomplete").unwrap(),
                ),
                name: <TypeDefs::name as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.name,
                    |v| dom_element.set_name(v),
                    || dom_element.remove_attribute("name").unwrap(),
                ),
                rel: <TypeDefs::rel as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.rel,
                    |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "rel",
                    ),
                    || dom_element.remove_attribute("rel").unwrap(),
                ),
                action: <TypeDefs::action as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.action,
                    |v| dom_element.set_action(v),
                    || dom_element.remove_attribute("action").unwrap(),
                ),
                enc_type: <TypeDefs::enc_type as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.enc_type,
                    |v| dom_element.set_enctype(v),
                    || dom_element.remove_attribute("enctype").unwrap(),
                ),
                method: <TypeDefs::method as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.method,
                    |v| dom_element.set_method(v),
                    || dom_element.remove_attribute("method").unwrap(),
                ),
                no_validate: <TypeDefs::no_validate as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.no_validate,
                    |v| dom_element.set_no_validate(*v),
                    || dom_element.remove_attribute("novalidate").unwrap(),
                ),
                target: <TypeDefs::target as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.target,
                    |v| dom_element.set_target(v),
                    || dom_element.remove_attribute("target").unwrap(),
                ),
                on_form_data: crate::imports::frender_html::props::UpdateDomEventListener::<
                    events::Event,
                >::initialize_dom_event_listener_state(this.on_form_data, element),
                on_reset: crate::imports::frender_html::props::UpdateDomEventListener::<
                    events::Event,
                >::initialize_dom_event_listener_state(this.on_reset, element),
                on_submit: crate::imports::frender_html::props::UpdateDomEventListener::<
                    events::Event,
                >::initialize_dom_event_listener_state(this.on_submit, element),
            }
        }
        fn update_element(
            this: Self,
            element: &web_sys::HtmlFormElement,
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
            <TypeDefs::accept as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.accept,
                state.accept,
                |v| {
                    crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "accept",
                )
                },
                || dom_element.remove_attribute("accept").unwrap(),
            );
            <TypeDefs::accept_charset as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.accept_charset,
                state.accept_charset,
                |v| dom_element.set_accept_charset(v),
                || dom_element.remove_attribute("accept-charset").unwrap(),
            );
            <TypeDefs::auto_complete as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.auto_complete,
                state.auto_complete,
                |v| dom_element.set_autocomplete(v),
                || dom_element.remove_attribute("autocomplete").unwrap(),
            );
            <TypeDefs::name as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.name,
                state.name,
                |v| dom_element.set_name(v),
                || dom_element.remove_attribute("name").unwrap(),
            );
            <TypeDefs::rel as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.rel,
                state.rel,
                |v| {
                    crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "rel",
                )
                },
                || dom_element.remove_attribute("rel").unwrap(),
            );
            <TypeDefs::action as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.action,
                state.action,
                |v| dom_element.set_action(v),
                || dom_element.remove_attribute("action").unwrap(),
            );
            <TypeDefs::enc_type as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.enc_type,
                state.enc_type,
                |v| dom_element.set_enctype(v),
                || dom_element.remove_attribute("enctype").unwrap(),
            );
            <TypeDefs::method as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.method,
                state.method,
                |v| dom_element.set_method(v),
                || dom_element.remove_attribute("method").unwrap(),
            );
            <TypeDefs::no_validate as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                bool,
            >>::maybe_update_value_with_state(
                this.no_validate,
                state.no_validate,
                |v| dom_element.set_no_validate(*v),
                || dom_element.remove_attribute("novalidate").unwrap(),
            );
            <TypeDefs::target as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.target,
                state.target,
                |v| dom_element.set_target(v),
                || dom_element.remove_attribute("target").unwrap(),
            );
            crate::imports::frender_html::props::UpdateDomEventListener::<
                events::Event,
            >::update_dom_event_listener(this.on_form_data, element, state.on_form_data);
            crate::imports::frender_html::props::UpdateDomEventListener::<
                events::Event,
            >::update_dom_event_listener(this.on_reset, element, state.on_reset);
            crate::imports::frender_html::props::UpdateDomEventListener::<
                events::Event,
            >::update_dom_event_listener(this.on_submit, element, state.on_submit);
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
                10usize,
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
                                <TypeDefs::accept as ::frender_html::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.accept)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("accept"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::accept_charset as ::frender_html::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.accept_charset)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("accept-charset"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::auto_complete as ::frender_html::props::MaybeUpdateValueWithState<
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
                                <TypeDefs::name as ::frender_html::props::MaybeUpdateValueWithState<
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
                                <TypeDefs::rel as ::frender_html::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.rel)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("rel"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::action as ::frender_html::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.action)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("action"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::enc_type as ::frender_html::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.enc_type)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("enctype"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::method as ::frender_html::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.method)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("method"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::no_validate as ::frender_html::props::MaybeUpdateValueWithState<
                                    bool,
                                >>::maybe_into_html_attribute_value(this.no_validate)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("novalidate"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::target as ::frender_html::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.target)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("target"),
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
