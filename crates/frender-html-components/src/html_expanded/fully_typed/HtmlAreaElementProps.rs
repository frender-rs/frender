#[allow(non_snake_case)]
#[inline(always)]
pub fn HtmlAreaElementProps() -> Building<TypesInitial> {
    #[allow(unused_imports)]
    use super::*;
    self::Building {
        HtmlElementWithHrefProps: HtmlElementWithHrefProps::build(HtmlElementWithHrefProps()),
        alt: (),
        coords: (),
        shape: (),
    }
}
pub mod prelude {}
pub mod overwrite {
    #![allow(non_camel_case_types)]
    pub type HtmlElementWithHrefProps<TypeDefs, Value> = dyn super::Types<
        HtmlElementWithHrefProps = Value,
        alt = <TypeDefs as super::Types>::alt,
        coords = <TypeDefs as super::Types>::coords,
        shape = <TypeDefs as super::Types>::shape,
    >;
    pub type HtmlElementProps<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::HtmlElementProps<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type ElementProps<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::ElementProps<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type children<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::children<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type css<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::css<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type class<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::class<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type id<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::id<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type part<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::part<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_cancel<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_cancel<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_error<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_error<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_scroll<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_scroll<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_security_policy_violation<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_security_policy_violation<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_select<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_select<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_wheel<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_wheel<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_copy<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_copy<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_cut<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_cut<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_paste<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_paste<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_composition_end<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_composition_end<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_composition_start<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_composition_start<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_composition_update<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_composition_update<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_blur<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_blur<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_focus<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_focus<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_focus_in<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_focus_in<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_focus_out<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_focus_out<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_fullscreen_change<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_fullscreen_change<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_fullscreen_error<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_fullscreen_error<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_key_down<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_key_down<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_key_up<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_key_up<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_aux_click<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_aux_click<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_click<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_click<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_context_menu<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_context_menu<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_double_click<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_double_click<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_mouse_down<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_mouse_down<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_mouse_enter<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_mouse_enter<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_mouse_leave<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_mouse_leave<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_mouse_move<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_mouse_move<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_mouse_out<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_mouse_out<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_mouse_over<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_mouse_over<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_mouse_up<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_mouse_up<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_touch_cancel<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_touch_cancel<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_touch_end<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_touch_end<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_touch_move<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_touch_move<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_touch_start<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_touch_start<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type access_key<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::access_key<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type auto_capitalize<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::auto_capitalize<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type auto_focus<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::auto_focus<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type content_editable<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::content_editable<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type context_menu<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::context_menu<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type dir<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::dir<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type draggable<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::draggable<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type enter_key_hint<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::enter_key_hint<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type hidden<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::hidden<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type inert<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::inert<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type input_mode<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::input_mode<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type is<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::is<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type item_id<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::item_id<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type item_prop<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::item_prop<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type item_ref<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::item_ref<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type item_scope<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::item_scope<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type item_type<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::item_type<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type lang<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::lang<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type nonce<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::nonce<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type role<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::role<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type slot<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::slot<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type spellcheck<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::spellcheck<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type style<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::style<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type tab_index<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::tab_index<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type title<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::title<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type translate<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::translate<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type virtual_keyboard_policy<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::virtual_keyboard_policy<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_invalid<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_invalid<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_animation_cancel<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_animation_cancel<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_animation_end<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_animation_end<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_animation_iteration<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_animation_iteration<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_animation_start<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_animation_start<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_before_input<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_before_input<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_input<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_input<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_change<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_change<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_got_pointer_capture<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_got_pointer_capture<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_lost_pointer_capture<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_lost_pointer_capture<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_pointer_cancel<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_pointer_cancel<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_pointer_down<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_pointer_down<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_pointer_enter<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_pointer_enter<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_pointer_leave<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_pointer_leave<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_pointer_move<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_pointer_move<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_pointer_out<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_pointer_out<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_pointer_over<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_pointer_over<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_pointer_up<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_pointer_up<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_transition_cancel<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_transition_cancel<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_transition_end<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_transition_end<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_transition_run<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_transition_run<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_transition_start<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_transition_start<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_drag<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_drag<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_drag_end<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_drag_end<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_drag_enter<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_drag_enter<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_drag_leave<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_drag_leave<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_drag_over<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_drag_over<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_drag_start<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_drag_start<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type on_drop<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::on_drop<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type download<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::download<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type href<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::href<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type ping<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::ping<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type referrer_policy<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::referrer_policy<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type rel<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::rel<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type target<TypeDefs, Value> = self::HtmlElementWithHrefProps<
        TypeDefs,
        super::super::HtmlElementWithHrefProps::overwrite::target<
            <TypeDefs as super::Types>::HtmlElementWithHrefProps,
            Value,
        >,
    >;
    pub type alt<TypeDefs, Value> = dyn super::Types<
        HtmlElementWithHrefProps = <TypeDefs as super::Types>::HtmlElementWithHrefProps,
        alt = Value,
        coords = <TypeDefs as super::Types>::coords,
        shape = <TypeDefs as super::Types>::shape,
    >;
    pub type coords<TypeDefs, Value> = dyn super::Types<
        HtmlElementWithHrefProps = <TypeDefs as super::Types>::HtmlElementWithHrefProps,
        alt = <TypeDefs as super::Types>::alt,
        coords = Value,
        shape = <TypeDefs as super::Types>::shape,
    >;
    pub type shape<TypeDefs, Value> = dyn super::Types<
        HtmlElementWithHrefProps = <TypeDefs as super::Types>::HtmlElementWithHrefProps,
        alt = <TypeDefs as super::Types>::alt,
        coords = <TypeDefs as super::Types>::coords,
        shape = Value,
    >;
}
mod trait_types {
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub trait Types {
        type HtmlElementWithHrefProps: ?::core::marker::Sized + HtmlElementWithHrefProps::Types;
        type alt: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>;
        type coords: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>;
        type shape: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>;
    }
}
pub use trait_types::Types;
pub use trait_types::Types as ValidTypes;
pub mod data_struct {
    #[non_exhaustive]
    pub struct HtmlAreaElementProps<TypeDefs: super::Types + ?::core::marker::Sized> {
        pub HtmlElementWithHrefProps:
            super::super::HtmlElementWithHrefProps::Data<TypeDefs::HtmlElementWithHrefProps>,
        pub alt: TypeDefs::alt,
        pub coords: TypeDefs::coords,
        pub shape: TypeDefs::shape,
    }
}
pub use ::core::convert::identity as Building;
pub use ::core::convert::identity as build;
pub use data_struct::HtmlAreaElementProps as Data;
pub use data_struct::HtmlAreaElementProps as Building;
pub struct Replacing<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
mod types_initial {
    #[allow(unused_imports)]
    use super::super::*;
    pub type TypesInitial = dyn super::Types<
        HtmlElementWithHrefProps = HtmlElementWithHrefProps::TypesInitial,
        alt = (),
        coords = (),
        shape = (),
    >;
}
pub use types_initial::TypesInitial;
pub type DataInitial = Data<TypesInitial>;
#[cfg(feature = "csr")]
pub mod render_state {
    #[allow(non_camel_case_types)]
    pub trait RenderStateTypes {
        type HtmlElementWithHrefProps: crate::imports::frender_csr::props::IntrinsicComponentPollReactive;
        type alt;
        type coords;
        type shape;
    }
    crate::imports::pin_project! {
        #[project = RenderStateProj] pub struct RenderState < TypeDefs : RenderStateTypes
        > where TypeDefs : ? ::core::marker::Sized { #[pin] pub HtmlElementWithHrefProps
        : TypeDefs::HtmlElementWithHrefProps, pub alt : TypeDefs::alt, pub coords :
        TypeDefs::coords, pub shape : TypeDefs::shape, }
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
                self.project().HtmlElementWithHrefProps,
                cx,
            )
        }
    }
}
mod builder_and_replacer {
    #[allow(unused_imports)]
    use super::super::*;
    impl<TypeDefs: super::Types + ?::core::marker::Sized> super::Building<TypeDefs> {
        ///See [`HtmlElementWithHrefProps::children`]
        #[inline(always)]
        pub fn children<V>(
            self,
            children: V,
        ) -> super::Building<super::overwrite::children<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.children(children),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::css`]
        #[inline(always)]
        pub fn css<V: Todo<unimplemented![]>>(
            self,
            css: V,
        ) -> super::Building<super::overwrite::css<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.css(css),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::class`]
        #[inline(always)]
        pub fn class<V: Todo<unimplemented![]>>(
            self,
            class: V,
        ) -> super::Building<super::overwrite::class<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.class(class),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::id`]
        #[inline(always)]
        pub fn id<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            id: V,
        ) -> super::Building<super::overwrite::id<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.id(id),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::part`]
        #[inline(always)]
        pub fn part<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            part: V,
        ) -> super::Building<super::overwrite::part<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.part(part),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_cancel`]
        #[inline(always)]
        pub fn on_cancel<V>(
            self,
            on_cancel: V,
        ) -> super::Building<super::overwrite::on_cancel<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.on_cancel(on_cancel),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_error`]
        #[inline(always)]
        pub fn on_error<V>(
            self,
            on_error: V,
        ) -> super::Building<super::overwrite::on_error<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.on_error(on_error),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_scroll`]
        #[inline(always)]
        pub fn on_scroll<V>(
            self,
            on_scroll: V,
        ) -> super::Building<super::overwrite::on_scroll<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.on_scroll(on_scroll),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_security_policy_violation`]
        #[inline(always)]
        pub fn on_security_policy_violation<V>(
            self,
            on_security_policy_violation: V,
        ) -> super::Building<super::overwrite::on_security_policy_violation<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .on_security_policy_violation(on_security_policy_violation),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_select`]
        #[inline(always)]
        pub fn on_select<V>(
            self,
            on_select: V,
        ) -> super::Building<super::overwrite::on_select<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.on_select(on_select),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_wheel`]
        #[inline(always)]
        pub fn on_wheel<V>(
            self,
            on_wheel: V,
        ) -> super::Building<super::overwrite::on_wheel<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.on_wheel(on_wheel),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_copy`]
        #[inline(always)]
        pub fn on_copy<V>(
            self,
            on_copy: V,
        ) -> super::Building<super::overwrite::on_copy<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.on_copy(on_copy),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_cut`]
        #[inline(always)]
        pub fn on_cut<V>(
            self,
            on_cut: V,
        ) -> super::Building<super::overwrite::on_cut<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.on_cut(on_cut),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_paste`]
        #[inline(always)]
        pub fn on_paste<V>(
            self,
            on_paste: V,
        ) -> super::Building<super::overwrite::on_paste<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.on_paste(on_paste),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_composition_end`]
        #[inline(always)]
        pub fn on_composition_end<V>(
            self,
            on_composition_end: V,
        ) -> super::Building<super::overwrite::on_composition_end<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .on_composition_end(on_composition_end),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_composition_start`]
        #[inline(always)]
        pub fn on_composition_start<V>(
            self,
            on_composition_start: V,
        ) -> super::Building<super::overwrite::on_composition_start<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .on_composition_start(on_composition_start),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_composition_update`]
        #[inline(always)]
        pub fn on_composition_update<V>(
            self,
            on_composition_update: V,
        ) -> super::Building<super::overwrite::on_composition_update<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .on_composition_update(on_composition_update),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_blur`]
        #[inline(always)]
        pub fn on_blur<V>(
            self,
            on_blur: V,
        ) -> super::Building<super::overwrite::on_blur<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.on_blur(on_blur),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_focus`]
        #[inline(always)]
        pub fn on_focus<V>(
            self,
            on_focus: V,
        ) -> super::Building<super::overwrite::on_focus<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.on_focus(on_focus),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_focus_in`]
        #[inline(always)]
        pub fn on_focus_in<V>(
            self,
            on_focus_in: V,
        ) -> super::Building<super::overwrite::on_focus_in<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.on_focus_in(on_focus_in),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_focus_out`]
        #[inline(always)]
        pub fn on_focus_out<V>(
            self,
            on_focus_out: V,
        ) -> super::Building<super::overwrite::on_focus_out<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.on_focus_out(on_focus_out),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_fullscreen_change`]
        #[inline(always)]
        pub fn on_fullscreen_change<V>(
            self,
            on_fullscreen_change: V,
        ) -> super::Building<super::overwrite::on_fullscreen_change<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .on_fullscreen_change(on_fullscreen_change),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_fullscreen_error`]
        #[inline(always)]
        pub fn on_fullscreen_error<V>(
            self,
            on_fullscreen_error: V,
        ) -> super::Building<super::overwrite::on_fullscreen_error<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .on_fullscreen_error(on_fullscreen_error),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_key_down`]
        #[inline(always)]
        pub fn on_key_down<V>(
            self,
            on_key_down: V,
        ) -> super::Building<super::overwrite::on_key_down<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.on_key_down(on_key_down),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_key_up`]
        #[inline(always)]
        pub fn on_key_up<V>(
            self,
            on_key_up: V,
        ) -> super::Building<super::overwrite::on_key_up<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.on_key_up(on_key_up),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_aux_click`]
        #[inline(always)]
        pub fn on_aux_click<V>(
            self,
            on_aux_click: V,
        ) -> super::Building<super::overwrite::on_aux_click<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.on_aux_click(on_aux_click),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_click`]
        #[inline(always)]
        pub fn on_click<V>(
            self,
            on_click: V,
        ) -> super::Building<super::overwrite::on_click<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.on_click(on_click),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_context_menu`]
        #[inline(always)]
        pub fn on_context_menu<V>(
            self,
            on_context_menu: V,
        ) -> super::Building<super::overwrite::on_context_menu<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .on_context_menu(on_context_menu),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_double_click`]
        #[inline(always)]
        pub fn on_double_click<V>(
            self,
            on_double_click: V,
        ) -> super::Building<super::overwrite::on_double_click<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .on_double_click(on_double_click),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_mouse_down`]
        #[inline(always)]
        pub fn on_mouse_down<V>(
            self,
            on_mouse_down: V,
        ) -> super::Building<super::overwrite::on_mouse_down<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .on_mouse_down(on_mouse_down),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_mouse_enter`]
        #[inline(always)]
        pub fn on_mouse_enter<V>(
            self,
            on_mouse_enter: V,
        ) -> super::Building<super::overwrite::on_mouse_enter<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .on_mouse_enter(on_mouse_enter),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_mouse_leave`]
        #[inline(always)]
        pub fn on_mouse_leave<V>(
            self,
            on_mouse_leave: V,
        ) -> super::Building<super::overwrite::on_mouse_leave<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .on_mouse_leave(on_mouse_leave),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_mouse_move`]
        #[inline(always)]
        pub fn on_mouse_move<V>(
            self,
            on_mouse_move: V,
        ) -> super::Building<super::overwrite::on_mouse_move<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .on_mouse_move(on_mouse_move),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_mouse_out`]
        #[inline(always)]
        pub fn on_mouse_out<V>(
            self,
            on_mouse_out: V,
        ) -> super::Building<super::overwrite::on_mouse_out<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.on_mouse_out(on_mouse_out),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_mouse_over`]
        #[inline(always)]
        pub fn on_mouse_over<V>(
            self,
            on_mouse_over: V,
        ) -> super::Building<super::overwrite::on_mouse_over<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .on_mouse_over(on_mouse_over),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_mouse_up`]
        #[inline(always)]
        pub fn on_mouse_up<V>(
            self,
            on_mouse_up: V,
        ) -> super::Building<super::overwrite::on_mouse_up<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.on_mouse_up(on_mouse_up),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_touch_cancel`]
        #[inline(always)]
        pub fn on_touch_cancel<V>(
            self,
            on_touch_cancel: V,
        ) -> super::Building<super::overwrite::on_touch_cancel<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .on_touch_cancel(on_touch_cancel),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_touch_end`]
        #[inline(always)]
        pub fn on_touch_end<V>(
            self,
            on_touch_end: V,
        ) -> super::Building<super::overwrite::on_touch_end<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.on_touch_end(on_touch_end),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_touch_move`]
        #[inline(always)]
        pub fn on_touch_move<V>(
            self,
            on_touch_move: V,
        ) -> super::Building<super::overwrite::on_touch_move<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .on_touch_move(on_touch_move),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_touch_start`]
        #[inline(always)]
        pub fn on_touch_start<V>(
            self,
            on_touch_start: V,
        ) -> super::Building<super::overwrite::on_touch_start<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .on_touch_start(on_touch_start),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::access_key`]
        #[inline(always)]
        pub fn access_key<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
        >(
            self,
            access_key: V,
        ) -> super::Building<super::overwrite::access_key<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.access_key(access_key),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::auto_capitalize`]
        #[inline(always)]
        pub fn auto_capitalize<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
        >(
            self,
            auto_capitalize: V,
        ) -> super::Building<super::overwrite::auto_capitalize<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .auto_capitalize(auto_capitalize),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::auto_focus`]
        #[inline(always)]
        pub fn auto_focus<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
        >(
            self,
            auto_focus: V,
        ) -> super::Building<super::overwrite::auto_focus<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.auto_focus(auto_focus),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::content_editable`]
        #[inline(always)]
        pub fn content_editable<V: Todo<unimplemented![]>>(
            self,
            content_editable: V,
        ) -> super::Building<super::overwrite::content_editable<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .content_editable(content_editable),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::context_menu`]
        #[inline(always)]
        pub fn context_menu<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
        >(
            self,
            context_menu: V,
        ) -> super::Building<super::overwrite::context_menu<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.context_menu(context_menu),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::dir`]
        #[inline(always)]
        pub fn dir<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            dir: V,
        ) -> super::Building<super::overwrite::dir<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.dir(dir),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::draggable`]
        #[inline(always)]
        pub fn draggable<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
        >(
            self,
            draggable: V,
        ) -> super::Building<super::overwrite::draggable<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.draggable(draggable),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::enter_key_hint`]
        #[inline(always)]
        pub fn enter_key_hint<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
        >(
            self,
            enter_key_hint: V,
        ) -> super::Building<super::overwrite::enter_key_hint<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .enter_key_hint(enter_key_hint),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::hidden`]
        #[inline(always)]
        pub fn hidden<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>>(
            self,
            hidden: V,
        ) -> super::Building<super::overwrite::hidden<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.hidden(hidden),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::inert`]
        #[inline(always)]
        pub fn inert<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>>(
            self,
            inert: V,
        ) -> super::Building<super::overwrite::inert<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.inert(inert),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::input_mode`]
        #[inline(always)]
        pub fn input_mode<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
        >(
            self,
            input_mode: V,
        ) -> super::Building<super::overwrite::input_mode<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.input_mode(input_mode),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::is`]
        #[inline(always)]
        pub fn is<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            is: V,
        ) -> super::Building<super::overwrite::is<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.is(is),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::item_id`]
        #[inline(always)]
        pub fn item_id<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            item_id: V,
        ) -> super::Building<super::overwrite::item_id<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.item_id(item_id),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::item_prop`]
        #[inline(always)]
        pub fn item_prop<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            item_prop: V,
        ) -> super::Building<super::overwrite::item_prop<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.item_prop(item_prop),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::item_ref`]
        #[inline(always)]
        pub fn item_ref<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            item_ref: V,
        ) -> super::Building<super::overwrite::item_ref<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.item_ref(item_ref),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::item_scope`]
        #[inline(always)]
        pub fn item_scope<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
        >(
            self,
            item_scope: V,
        ) -> super::Building<super::overwrite::item_scope<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.item_scope(item_scope),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::item_type`]
        #[inline(always)]
        pub fn item_type<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            item_type: V,
        ) -> super::Building<super::overwrite::item_type<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.item_type(item_type),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::lang`]
        #[inline(always)]
        pub fn lang<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            lang: V,
        ) -> super::Building<super::overwrite::lang<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.lang(lang),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::nonce`]
        #[inline(always)]
        pub fn nonce<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            nonce: V,
        ) -> super::Building<super::overwrite::nonce<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.nonce(nonce),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::role`]
        #[inline(always)]
        pub fn role<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            role: V,
        ) -> super::Building<super::overwrite::role<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.role(role),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::slot`]
        #[inline(always)]
        pub fn slot<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            slot: V,
        ) -> super::Building<super::overwrite::slot<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.slot(slot),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::spellcheck`]
        #[inline(always)]
        pub fn spellcheck<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
        >(
            self,
            spellcheck: V,
        ) -> super::Building<super::overwrite::spellcheck<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.spellcheck(spellcheck),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::style`]
        #[inline(always)]
        pub fn style<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            style: V,
        ) -> super::Building<super::overwrite::style<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.style(style),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::tab_index`]
        #[inline(always)]
        pub fn tab_index<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<i32>>(
            self,
            tab_index: V,
        ) -> super::Building<super::overwrite::tab_index<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.tab_index(tab_index),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::title`]
        #[inline(always)]
        pub fn title<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            title: V,
        ) -> super::Building<super::overwrite::title<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.title(title),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::translate`]
        #[inline(always)]
        pub fn translate<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            translate: V,
        ) -> super::Building<super::overwrite::translate<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.translate(translate),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::virtual_keyboard_policy`]
        #[inline(always)]
        pub fn virtual_keyboard_policy<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
        >(
            self,
            virtual_keyboard_policy: V,
        ) -> super::Building<super::overwrite::virtual_keyboard_policy<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .virtual_keyboard_policy(virtual_keyboard_policy),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_invalid`]
        #[inline(always)]
        pub fn on_invalid<V>(
            self,
            on_invalid: V,
        ) -> super::Building<super::overwrite::on_invalid<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.on_invalid(on_invalid),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_animation_cancel`]
        #[inline(always)]
        pub fn on_animation_cancel<V>(
            self,
            on_animation_cancel: V,
        ) -> super::Building<super::overwrite::on_animation_cancel<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .on_animation_cancel(on_animation_cancel),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_animation_end`]
        #[inline(always)]
        pub fn on_animation_end<V>(
            self,
            on_animation_end: V,
        ) -> super::Building<super::overwrite::on_animation_end<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .on_animation_end(on_animation_end),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_animation_iteration`]
        #[inline(always)]
        pub fn on_animation_iteration<V>(
            self,
            on_animation_iteration: V,
        ) -> super::Building<super::overwrite::on_animation_iteration<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .on_animation_iteration(on_animation_iteration),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_animation_start`]
        #[inline(always)]
        pub fn on_animation_start<V>(
            self,
            on_animation_start: V,
        ) -> super::Building<super::overwrite::on_animation_start<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .on_animation_start(on_animation_start),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_before_input`]
        #[inline(always)]
        pub fn on_before_input<V>(
            self,
            on_before_input: V,
        ) -> super::Building<super::overwrite::on_before_input<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .on_before_input(on_before_input),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_input`]
        #[inline(always)]
        pub fn on_input<V>(
            self,
            on_input: V,
        ) -> super::Building<super::overwrite::on_input<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.on_input(on_input),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_change`]
        #[inline(always)]
        pub fn on_change<V>(
            self,
            on_change: V,
        ) -> super::Building<super::overwrite::on_change<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.on_change(on_change),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_got_pointer_capture`]
        #[inline(always)]
        pub fn on_got_pointer_capture<V>(
            self,
            on_got_pointer_capture: V,
        ) -> super::Building<super::overwrite::on_got_pointer_capture<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .on_got_pointer_capture(on_got_pointer_capture),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_lost_pointer_capture`]
        #[inline(always)]
        pub fn on_lost_pointer_capture<V>(
            self,
            on_lost_pointer_capture: V,
        ) -> super::Building<super::overwrite::on_lost_pointer_capture<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .on_lost_pointer_capture(on_lost_pointer_capture),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_pointer_cancel`]
        #[inline(always)]
        pub fn on_pointer_cancel<V>(
            self,
            on_pointer_cancel: V,
        ) -> super::Building<super::overwrite::on_pointer_cancel<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .on_pointer_cancel(on_pointer_cancel),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_pointer_down`]
        #[inline(always)]
        pub fn on_pointer_down<V>(
            self,
            on_pointer_down: V,
        ) -> super::Building<super::overwrite::on_pointer_down<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .on_pointer_down(on_pointer_down),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_pointer_enter`]
        #[inline(always)]
        pub fn on_pointer_enter<V>(
            self,
            on_pointer_enter: V,
        ) -> super::Building<super::overwrite::on_pointer_enter<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .on_pointer_enter(on_pointer_enter),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_pointer_leave`]
        #[inline(always)]
        pub fn on_pointer_leave<V>(
            self,
            on_pointer_leave: V,
        ) -> super::Building<super::overwrite::on_pointer_leave<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .on_pointer_leave(on_pointer_leave),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_pointer_move`]
        #[inline(always)]
        pub fn on_pointer_move<V>(
            self,
            on_pointer_move: V,
        ) -> super::Building<super::overwrite::on_pointer_move<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .on_pointer_move(on_pointer_move),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_pointer_out`]
        #[inline(always)]
        pub fn on_pointer_out<V>(
            self,
            on_pointer_out: V,
        ) -> super::Building<super::overwrite::on_pointer_out<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .on_pointer_out(on_pointer_out),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_pointer_over`]
        #[inline(always)]
        pub fn on_pointer_over<V>(
            self,
            on_pointer_over: V,
        ) -> super::Building<super::overwrite::on_pointer_over<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .on_pointer_over(on_pointer_over),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_pointer_up`]
        #[inline(always)]
        pub fn on_pointer_up<V>(
            self,
            on_pointer_up: V,
        ) -> super::Building<super::overwrite::on_pointer_up<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .on_pointer_up(on_pointer_up),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_transition_cancel`]
        #[inline(always)]
        pub fn on_transition_cancel<V>(
            self,
            on_transition_cancel: V,
        ) -> super::Building<super::overwrite::on_transition_cancel<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .on_transition_cancel(on_transition_cancel),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_transition_end`]
        #[inline(always)]
        pub fn on_transition_end<V>(
            self,
            on_transition_end: V,
        ) -> super::Building<super::overwrite::on_transition_end<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .on_transition_end(on_transition_end),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_transition_run`]
        #[inline(always)]
        pub fn on_transition_run<V>(
            self,
            on_transition_run: V,
        ) -> super::Building<super::overwrite::on_transition_run<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .on_transition_run(on_transition_run),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_transition_start`]
        #[inline(always)]
        pub fn on_transition_start<V>(
            self,
            on_transition_start: V,
        ) -> super::Building<super::overwrite::on_transition_start<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .on_transition_start(on_transition_start),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_drag`]
        #[inline(always)]
        pub fn on_drag<V>(
            self,
            on_drag: V,
        ) -> super::Building<super::overwrite::on_drag<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.on_drag(on_drag),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_drag_end`]
        #[inline(always)]
        pub fn on_drag_end<V>(
            self,
            on_drag_end: V,
        ) -> super::Building<super::overwrite::on_drag_end<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.on_drag_end(on_drag_end),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_drag_enter`]
        #[inline(always)]
        pub fn on_drag_enter<V>(
            self,
            on_drag_enter: V,
        ) -> super::Building<super::overwrite::on_drag_enter<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .on_drag_enter(on_drag_enter),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_drag_leave`]
        #[inline(always)]
        pub fn on_drag_leave<V>(
            self,
            on_drag_leave: V,
        ) -> super::Building<super::overwrite::on_drag_leave<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .on_drag_leave(on_drag_leave),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_drag_over`]
        #[inline(always)]
        pub fn on_drag_over<V>(
            self,
            on_drag_over: V,
        ) -> super::Building<super::overwrite::on_drag_over<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.on_drag_over(on_drag_over),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_drag_start`]
        #[inline(always)]
        pub fn on_drag_start<V>(
            self,
            on_drag_start: V,
        ) -> super::Building<super::overwrite::on_drag_start<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .on_drag_start(on_drag_start),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::on_drop`]
        #[inline(always)]
        pub fn on_drop<V>(
            self,
            on_drop: V,
        ) -> super::Building<super::overwrite::on_drop<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.on_drop(on_drop),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::download`]
        #[inline(always)]
        pub fn download<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            download: V,
        ) -> super::Building<super::overwrite::download<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.download(download),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::href`]
        #[inline(always)]
        pub fn href<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            href: V,
        ) -> super::Building<super::overwrite::href<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.href(href),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::ping`]
        #[inline(always)]
        pub fn ping<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            ping: V,
        ) -> super::Building<super::overwrite::ping<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.ping(ping),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::referrer_policy`]
        #[inline(always)]
        pub fn referrer_policy<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
        >(
            self,
            referrer_policy: V,
        ) -> super::Building<super::overwrite::referrer_policy<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self
                    .HtmlElementWithHrefProps
                    .referrer_policy(referrer_policy),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::rel`]
        #[inline(always)]
        pub fn rel<V: Todo<unimplemented![]>>(
            self,
            rel: V,
        ) -> super::Building<super::overwrite::rel<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.rel(rel),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementWithHrefProps::target`]
        #[inline(always)]
        pub fn target<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            target: V,
        ) -> super::Building<super::overwrite::target<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps.target(target),
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        #[inline(always)]
        pub fn alt<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            alt: V,
        ) -> super::Building<super::overwrite::alt<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps,
                alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        #[inline(always)]
        pub fn coords<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            coords: V,
        ) -> super::Building<super::overwrite::coords<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps,
                alt: self.alt,
                coords,
                shape: self.shape,
            }
        }
        #[inline(always)]
        pub fn shape<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            shape: V,
        ) -> super::Building<super::overwrite::shape<TypeDefs, V>> {
            super::Data {
                HtmlElementWithHrefProps: self.HtmlElementWithHrefProps,
                alt: self.alt,
                coords: self.coords,
                shape,
            }
        }
    }
}
#[cfg(feature = "csr")]
mod impl_update_element {
    #[allow(unused_imports)]
    use super::super::*;
    impl<TypeDefs: ?::core::marker::Sized + super::Types>
        crate::imports::frender_csr::props::UpdateElement<HtmlAreaElement> for super::Data<TypeDefs>
    where
        HtmlElementWithHrefProps::Data<TypeDefs::HtmlElementWithHrefProps>:
            crate::imports::frender_csr::props::UpdateElement<HtmlElementWithHref>,
    {
        type State = super::render_state::RenderState<
            dyn super::render_state::RenderStateTypes<
                HtmlElementWithHrefProps = <HtmlElementWithHrefProps::Data<
                    TypeDefs::HtmlElementWithHrefProps,
                > as crate::imports::frender_csr::props::UpdateElement<
                    HtmlElementWithHref,
                >>::State,
                alt = <TypeDefs::alt as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                coords = <TypeDefs::coords as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                shape = <TypeDefs::shape as ::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
            >,
        >;
        fn initialize_state(
            this: Self,
            element: &HtmlAreaElement,
            children_ctx: &mut ::frender_csr::Dom,
        ) -> Self::State {
            let dom_element: &::web_sys::Element = element.as_ref();
            super::render_state::RenderState {
                HtmlElementWithHrefProps: <HtmlElementWithHrefProps::Data<
                    TypeDefs::HtmlElementWithHrefProps,
                > as crate::imports::frender_csr::props::UpdateElement<
                    HtmlElementWithHref,
                >>::initialize_state(
                    this.HtmlElementWithHrefProps,
                    element,
                    children_ctx,
                ),
                alt: <TypeDefs::alt as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.alt,
                    |v| dom_element.set_alt(v),
                    || dom_element.remove_attribute("alt").unwrap(),
                ),
                coords: <TypeDefs::coords as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.coords,
                    |v| dom_element.set_coords(v),
                    || dom_element.remove_attribute("coords").unwrap(),
                ),
                shape: <TypeDefs::shape as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.shape,
                    |v| dom_element.set_shape(v),
                    || dom_element.remove_attribute("shape").unwrap(),
                ),
            }
        }
        fn update_element(
            this: Self,
            element: &HtmlAreaElement,
            children_ctx: &mut ::frender_csr::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let state = state.pin_project();
            let dom_element: &::web_sys::Element = element.as_ref();
            crate::imports::frender_csr::props::UpdateElement::update_element(
                this.HtmlElementWithHrefProps,
                element.as_ref(),
                children_ctx,
                state.HtmlElementWithHrefProps,
            );
            <TypeDefs::alt as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.alt,
                state.alt,
                |v| dom_element.set_alt(v),
                || dom_element.remove_attribute("alt").unwrap(),
            );
            <TypeDefs::coords as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.coords,
                state.coords,
                |v| dom_element.set_coords(v),
                || dom_element.remove_attribute("coords").unwrap(),
            );
            <TypeDefs::shape as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.shape,
                state.shape,
                |v| dom_element.set_shape(v),
                || dom_element.remove_attribute("shape").unwrap(),
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
        HtmlElementWithHrefProps::Data<TypeDefs::HtmlElementWithHrefProps>:
            ::frender_ssr::IntoSsrData<W>,
    {
        type Children = <HtmlElementWithHrefProps::Data<
            TypeDefs::HtmlElementWithHrefProps,
        > as ::frender_ssr::IntoSsrData<W>>::Children;
        type ChildrenRenderState = <HtmlElementWithHrefProps::Data<
            TypeDefs::HtmlElementWithHrefProps,
        > as ::frender_ssr::IntoSsrData<W>>::ChildrenRenderState;
        type Attrs = ::core::iter::Chain<
            <HtmlElementWithHrefProps::Data<
                TypeDefs::HtmlElementWithHrefProps,
            > as ::frender_ssr::IntoSsrData<W>>::Attrs,
            ::frender_ssr::utils::filter::FilterArray<
                ::frender_ssr::element::html::HtmlAttrPair<'static>,
                3usize,
            >,
        >;
        fn into_ssr_data(this: Self) -> (Self::Children, Self::Attrs) {
            let (children, attrs) =
                ::frender_ssr::IntoSsrData::into_ssr_data(this.HtmlElementWithHrefProps);
            (
                children,
                attrs
                    .chain(
                        ::frender_ssr::utils::filter::FilterIdentity(
                            [
                                <TypeDefs::alt as ::frender_html::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.alt)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("alt"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::coords as ::frender_html::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.coords)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("coords"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::shape as ::frender_html::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.shape)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("shape"),
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
