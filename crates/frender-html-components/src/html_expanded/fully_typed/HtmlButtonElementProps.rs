#[allow(non_snake_case)]
#[inline(always)]
pub fn HtmlButtonElementProps() -> Building<TypesInitial> {
    #[allow(unused_imports)]
    use super::*;
    self::Building {
        HtmlElementProps: HtmlElementProps::build(HtmlElementProps()),
        disabled: (),
        form: (),
        form_action: (),
        form_enc_type: (),
        form_method: (),
        form_no_validate: (),
        form_target: (),
        name: (),
        r#type: (),
        value: (),
    }
}
pub mod prelude {}
pub mod overwrite {
    #![allow(non_camel_case_types)]
    pub type HtmlElementProps<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = Value,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        name = <TypeDefs as super::Types>::name,
        r#type = <TypeDefs as super::Types>::r#type,
        value = <TypeDefs as super::Types>::value,
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
    pub type disabled<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        disabled = Value,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        name = <TypeDefs as super::Types>::name,
        r#type = <TypeDefs as super::Types>::r#type,
        value = <TypeDefs as super::Types>::value,
    >;
    pub type form<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        disabled = <TypeDefs as super::Types>::disabled,
        form = Value,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        name = <TypeDefs as super::Types>::name,
        r#type = <TypeDefs as super::Types>::r#type,
        value = <TypeDefs as super::Types>::value,
    >;
    pub type form_action<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = Value,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        name = <TypeDefs as super::Types>::name,
        r#type = <TypeDefs as super::Types>::r#type,
        value = <TypeDefs as super::Types>::value,
    >;
    pub type form_enc_type<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = Value,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        name = <TypeDefs as super::Types>::name,
        r#type = <TypeDefs as super::Types>::r#type,
        value = <TypeDefs as super::Types>::value,
    >;
    pub type form_method<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = Value,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        name = <TypeDefs as super::Types>::name,
        r#type = <TypeDefs as super::Types>::r#type,
        value = <TypeDefs as super::Types>::value,
    >;
    pub type form_no_validate<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = Value,
        form_target = <TypeDefs as super::Types>::form_target,
        name = <TypeDefs as super::Types>::name,
        r#type = <TypeDefs as super::Types>::r#type,
        value = <TypeDefs as super::Types>::value,
    >;
    pub type form_target<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = Value,
        name = <TypeDefs as super::Types>::name,
        r#type = <TypeDefs as super::Types>::r#type,
        value = <TypeDefs as super::Types>::value,
    >;
    pub type name<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        name = Value,
        r#type = <TypeDefs as super::Types>::r#type,
        value = <TypeDefs as super::Types>::value,
    >;
    pub type r#type<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        name = <TypeDefs as super::Types>::name,
        r#type = Value,
        value = <TypeDefs as super::Types>::value,
    >;
    pub type value<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        name = <TypeDefs as super::Types>::name,
        r#type = <TypeDefs as super::Types>::r#type,
        value = Value,
    >;
}
mod trait_types {
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub trait Types {
        type HtmlElementProps: ?::core::marker::Sized + HtmlElementProps::Types;
        type disabled: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>;
        type form: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>;
        type form_action: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>;
        type form_enc_type: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>;
        type form_method: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>;
        type form_no_validate: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>;
        type form_target: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>;
        type name: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>;
        type r#type: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>;
        type value: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>;
    }
}
pub use trait_types::Types;
pub use trait_types::Types as ValidTypes;
pub mod data_struct {
    #[non_exhaustive]
    pub struct HtmlButtonElementProps<TypeDefs: super::Types + ?::core::marker::Sized> {
        pub HtmlElementProps: super::super::HtmlElementProps::Data<TypeDefs::HtmlElementProps>,
        pub disabled: TypeDefs::disabled,
        pub form: TypeDefs::form,
        pub form_action: TypeDefs::form_action,
        pub form_enc_type: TypeDefs::form_enc_type,
        pub form_method: TypeDefs::form_method,
        pub form_no_validate: TypeDefs::form_no_validate,
        pub form_target: TypeDefs::form_target,
        pub name: TypeDefs::name,
        pub r#type: TypeDefs::r#type,
        pub value: TypeDefs::value,
    }
}
pub use ::core::convert::identity as Building;
pub use ::core::convert::identity as build;
pub use data_struct::HtmlButtonElementProps as Data;
pub use data_struct::HtmlButtonElementProps as Building;
pub struct Replacing<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
mod types_initial {
    #[allow(unused_imports)]
    use super::super::*;
    pub type TypesInitial = dyn super::Types<
        HtmlElementProps = HtmlElementProps::TypesInitial,
        disabled = (),
        form = (),
        form_action = (),
        form_enc_type = (),
        form_method = (),
        form_no_validate = (),
        form_target = (),
        name = (),
        r#type = (),
        value = (),
    >;
}
pub use types_initial::TypesInitial;
pub type DataInitial = Data<TypesInitial>;
#[cfg(feature = "csr")]
pub mod render_state {
    #[allow(non_camel_case_types)]
    pub trait RenderStateTypes {
        type HtmlElementProps: crate::imports::frender_csr::props::IntrinsicComponentPollReactive;
        type disabled;
        type form;
        type form_action;
        type form_enc_type;
        type form_method;
        type form_no_validate;
        type form_target;
        type name;
        type r#type;
        type value;
    }
    crate::imports::pin_project! {
        #[project = RenderStateProj] pub struct RenderState < TypeDefs : RenderStateTypes
        > where TypeDefs : ? ::core::marker::Sized { #[pin] pub HtmlElementProps :
        TypeDefs::HtmlElementProps, pub disabled : TypeDefs::disabled, pub form :
        TypeDefs::form, pub form_action : TypeDefs::form_action, pub form_enc_type :
        TypeDefs::form_enc_type, pub form_method : TypeDefs::form_method, pub
        form_no_validate : TypeDefs::form_no_validate, pub form_target :
        TypeDefs::form_target, pub name : TypeDefs::name, pub r#type : TypeDefs::r#type,
        pub value : TypeDefs::value, }
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
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
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
            }
        }
        #[inline(always)]
        pub fn disabled<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>>(
            self,
            disabled: V,
        ) -> super::Building<super::overwrite::disabled<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
            }
        }
        #[inline(always)]
        pub fn form<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            form: V,
        ) -> super::Building<super::overwrite::form<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                disabled: self.disabled,
                form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
            }
        }
        #[inline(always)]
        pub fn form_action<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
        >(
            self,
            form_action: V,
        ) -> super::Building<super::overwrite::form_action<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                disabled: self.disabled,
                form: self.form,
                form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
            }
        }
        #[inline(always)]
        pub fn form_enc_type<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
        >(
            self,
            form_enc_type: V,
        ) -> super::Building<super::overwrite::form_enc_type<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
            }
        }
        #[inline(always)]
        pub fn form_method<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
        >(
            self,
            form_method: V,
        ) -> super::Building<super::overwrite::form_method<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
            }
        }
        #[inline(always)]
        pub fn form_no_validate<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
        >(
            self,
            form_no_validate: V,
        ) -> super::Building<super::overwrite::form_no_validate<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
            }
        }
        #[inline(always)]
        pub fn form_target<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
        >(
            self,
            form_target: V,
        ) -> super::Building<super::overwrite::form_target<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target,
                name: self.name,
                r#type: self.r#type,
                value: self.value,
            }
        }
        #[inline(always)]
        pub fn name<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            name: V,
        ) -> super::Building<super::overwrite::name<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name,
                r#type: self.r#type,
                value: self.value,
            }
        }
        #[inline(always)]
        pub fn r#type<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            r#type: V,
        ) -> super::Building<super::overwrite::r#type<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type,
                value: self.value,
            }
        }
        #[inline(always)]
        pub fn value<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            value: V,
        ) -> super::Building<super::overwrite::value<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                disabled: self.disabled,
                form: self.form,
                form_action: self.form_action,
                form_enc_type: self.form_enc_type,
                form_method: self.form_method,
                form_no_validate: self.form_no_validate,
                form_target: self.form_target,
                name: self.name,
                r#type: self.r#type,
                value,
            }
        }
    }
}
#[cfg(feature = "csr")]
mod impl_update_element {
    #[allow(unused_imports)]
    use super::super::*;
    impl<TypeDefs: ?::core::marker::Sized + super::Types>
        crate::imports::frender_csr::props::UpdateElement<web_sys::HtmlButtonElement>
        for super::Data<TypeDefs>
    where
        HtmlElementProps::Data<TypeDefs::HtmlElementProps>:
            crate::imports::frender_csr::props::UpdateElement<web_sys::HtmlElement>,
    {
        type State = super::render_state::RenderState<
            dyn super::render_state::RenderStateTypes<
                HtmlElementProps = <HtmlElementProps::Data<
                    TypeDefs::HtmlElementProps,
                > as crate::imports::frender_csr::props::UpdateElement<
                    web_sys::HtmlElement,
                >>::State,
                disabled = <TypeDefs::disabled as ::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::State,
                form = <TypeDefs::form as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                form_action = <TypeDefs::form_action as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                form_enc_type = <TypeDefs::form_enc_type as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                form_method = <TypeDefs::form_method as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                form_no_validate = <TypeDefs::form_no_validate as ::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::State,
                form_target = <TypeDefs::form_target as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                name = <TypeDefs::name as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                r#type = <TypeDefs::r#type as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                value = <TypeDefs::value as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
            >,
        >;
        fn initialize_state(
            this: Self,
            element: &web_sys::HtmlButtonElement,
            children_ctx: &mut ::frender_csr::Dom,
        ) -> Self::State {
            let dom_element: &::web_sys::Element = element.as_ref();
            super::render_state::RenderState {
                HtmlElementProps: <HtmlElementProps::Data<
                    TypeDefs::HtmlElementProps,
                > as crate::imports::frender_csr::props::UpdateElement<
                    web_sys::HtmlElement,
                >>::initialize_state(this.HtmlElementProps, element, children_ctx),
                disabled: <TypeDefs::disabled as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.disabled,
                    |v| element.set_disabled(*v),
                    || dom_element.remove_attribute("disabled").unwrap(),
                ),
                form: <TypeDefs::form as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.form,
                    |v| crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "form",
                    ),
                    || dom_element.remove_attribute("form").unwrap(),
                ),
                form_action: <TypeDefs::form_action as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.form_action,
                    |v| element.set_form_action(v),
                    || dom_element.remove_attribute("formaction").unwrap(),
                ),
                form_enc_type: <TypeDefs::form_enc_type as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.form_enc_type,
                    |v| element.set_form_enctype(v),
                    || dom_element.remove_attribute("formenctype").unwrap(),
                ),
                form_method: <TypeDefs::form_method as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.form_method,
                    |v| element.set_form_method(v),
                    || dom_element.remove_attribute("formmethod").unwrap(),
                ),
                form_no_validate: <TypeDefs::form_no_validate as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.form_no_validate,
                    |v| element.set_form_no_validate(*v),
                    || dom_element.remove_attribute("formnovalidate").unwrap(),
                ),
                form_target: <TypeDefs::form_target as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.form_target,
                    |v| element.set_form_target(v),
                    || dom_element.remove_attribute("formtarget").unwrap(),
                ),
                name: <TypeDefs::name as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.name,
                    |v| element.set_name(v),
                    || dom_element.remove_attribute("name").unwrap(),
                ),
                r#type: <TypeDefs::r#type as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.r#type,
                    |v| element.set_type(v),
                    || dom_element.remove_attribute("type").unwrap(),
                ),
                value: <TypeDefs::value as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.value,
                    |v| element.set_value(v),
                    || dom_element.remove_attribute("value").unwrap(),
                ),
            }
        }
        fn update_element(
            this: Self,
            element: &web_sys::HtmlButtonElement,
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
            <TypeDefs::disabled as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                bool,
            >>::maybe_update_value_with_state(
                this.disabled,
                state.disabled,
                |v| element.set_disabled(*v),
                || dom_element.remove_attribute("disabled").unwrap(),
            );
            <TypeDefs::form as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.form,
                state.form,
                |v| {
                    crate::imports::frender_csr::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "form",
                )
                },
                || dom_element.remove_attribute("form").unwrap(),
            );
            <TypeDefs::form_action as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.form_action,
                state.form_action,
                |v| element.set_form_action(v),
                || dom_element.remove_attribute("formaction").unwrap(),
            );
            <TypeDefs::form_enc_type as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.form_enc_type,
                state.form_enc_type,
                |v| element.set_form_enctype(v),
                || dom_element.remove_attribute("formenctype").unwrap(),
            );
            <TypeDefs::form_method as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.form_method,
                state.form_method,
                |v| element.set_form_method(v),
                || dom_element.remove_attribute("formmethod").unwrap(),
            );
            <TypeDefs::form_no_validate as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                bool,
            >>::maybe_update_value_with_state(
                this.form_no_validate,
                state.form_no_validate,
                |v| element.set_form_no_validate(*v),
                || dom_element.remove_attribute("formnovalidate").unwrap(),
            );
            <TypeDefs::form_target as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.form_target,
                state.form_target,
                |v| element.set_form_target(v),
                || dom_element.remove_attribute("formtarget").unwrap(),
            );
            <TypeDefs::name as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.name,
                state.name,
                |v| element.set_name(v),
                || dom_element.remove_attribute("name").unwrap(),
            );
            <TypeDefs::r#type as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.r#type,
                state.r#type,
                |v| element.set_type(v),
                || dom_element.remove_attribute("type").unwrap(),
            );
            <TypeDefs::value as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.value,
                state.value,
                |v| element.set_value(v),
                || dom_element.remove_attribute("value").unwrap(),
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
                                <TypeDefs::disabled as ::frender_html::props::MaybeUpdateValueWithState<
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
                                <TypeDefs::form as ::frender_html::props::MaybeUpdateValueWithState<
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
                                <TypeDefs::form_action as ::frender_html::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.form_action)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("formaction"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::form_enc_type as ::frender_html::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.form_enc_type)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("formenctype"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::form_method as ::frender_html::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.form_method)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("formmethod"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::form_no_validate as ::frender_html::props::MaybeUpdateValueWithState<
                                    bool,
                                >>::maybe_into_html_attribute_value(this.form_no_validate)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("formnovalidate"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::form_target as ::frender_html::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.form_target)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("formtarget"),
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
                                <TypeDefs::r#type as ::frender_html::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.r#type)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("type"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::value as ::frender_html::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.value)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("value"),
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
