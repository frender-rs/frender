#[allow(non_snake_case)]
#[inline(always)]
pub fn HtmlIFrameElementProps() -> Building<TypesInitial> {
    #[allow(unused_imports)]
    use super::*;
    self::Building {
        HtmlElementProps: HtmlElementProps::build(HtmlElementProps()),
        allow: (),
        allow_fullscreen: (),
        allow_payment_request: (),
        csp: (),
        fetch_priority: (),
        height: (),
        loading: (),
        name: (),
        referrer_policy: (),
        sandbox: (),
        src: (),
        src_doc: (),
        width: (),
    }
}
pub mod prelude {}
pub mod overwrite {
    #![allow(non_camel_case_types)]
    pub type HtmlElementProps<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = Value,
        allow = <TypeDefs as super::Types>::allow,
        allow_fullscreen = <TypeDefs as super::Types>::allow_fullscreen,
        allow_payment_request = <TypeDefs as super::Types>::allow_payment_request,
        csp = <TypeDefs as super::Types>::csp,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        height = <TypeDefs as super::Types>::height,
        loading = <TypeDefs as super::Types>::loading,
        name = <TypeDefs as super::Types>::name,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        sandbox = <TypeDefs as super::Types>::sandbox,
        src = <TypeDefs as super::Types>::src,
        src_doc = <TypeDefs as super::Types>::src_doc,
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
    pub type allow<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        allow = Value,
        allow_fullscreen = <TypeDefs as super::Types>::allow_fullscreen,
        allow_payment_request = <TypeDefs as super::Types>::allow_payment_request,
        csp = <TypeDefs as super::Types>::csp,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        height = <TypeDefs as super::Types>::height,
        loading = <TypeDefs as super::Types>::loading,
        name = <TypeDefs as super::Types>::name,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        sandbox = <TypeDefs as super::Types>::sandbox,
        src = <TypeDefs as super::Types>::src,
        src_doc = <TypeDefs as super::Types>::src_doc,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type allow_fullscreen<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        allow = <TypeDefs as super::Types>::allow,
        allow_fullscreen = Value,
        allow_payment_request = <TypeDefs as super::Types>::allow_payment_request,
        csp = <TypeDefs as super::Types>::csp,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        height = <TypeDefs as super::Types>::height,
        loading = <TypeDefs as super::Types>::loading,
        name = <TypeDefs as super::Types>::name,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        sandbox = <TypeDefs as super::Types>::sandbox,
        src = <TypeDefs as super::Types>::src,
        src_doc = <TypeDefs as super::Types>::src_doc,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type allow_payment_request<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        allow = <TypeDefs as super::Types>::allow,
        allow_fullscreen = <TypeDefs as super::Types>::allow_fullscreen,
        allow_payment_request = Value,
        csp = <TypeDefs as super::Types>::csp,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        height = <TypeDefs as super::Types>::height,
        loading = <TypeDefs as super::Types>::loading,
        name = <TypeDefs as super::Types>::name,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        sandbox = <TypeDefs as super::Types>::sandbox,
        src = <TypeDefs as super::Types>::src,
        src_doc = <TypeDefs as super::Types>::src_doc,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type csp<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        allow = <TypeDefs as super::Types>::allow,
        allow_fullscreen = <TypeDefs as super::Types>::allow_fullscreen,
        allow_payment_request = <TypeDefs as super::Types>::allow_payment_request,
        csp = Value,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        height = <TypeDefs as super::Types>::height,
        loading = <TypeDefs as super::Types>::loading,
        name = <TypeDefs as super::Types>::name,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        sandbox = <TypeDefs as super::Types>::sandbox,
        src = <TypeDefs as super::Types>::src,
        src_doc = <TypeDefs as super::Types>::src_doc,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type fetch_priority<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        allow = <TypeDefs as super::Types>::allow,
        allow_fullscreen = <TypeDefs as super::Types>::allow_fullscreen,
        allow_payment_request = <TypeDefs as super::Types>::allow_payment_request,
        csp = <TypeDefs as super::Types>::csp,
        fetch_priority = Value,
        height = <TypeDefs as super::Types>::height,
        loading = <TypeDefs as super::Types>::loading,
        name = <TypeDefs as super::Types>::name,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        sandbox = <TypeDefs as super::Types>::sandbox,
        src = <TypeDefs as super::Types>::src,
        src_doc = <TypeDefs as super::Types>::src_doc,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type height<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        allow = <TypeDefs as super::Types>::allow,
        allow_fullscreen = <TypeDefs as super::Types>::allow_fullscreen,
        allow_payment_request = <TypeDefs as super::Types>::allow_payment_request,
        csp = <TypeDefs as super::Types>::csp,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        height = Value,
        loading = <TypeDefs as super::Types>::loading,
        name = <TypeDefs as super::Types>::name,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        sandbox = <TypeDefs as super::Types>::sandbox,
        src = <TypeDefs as super::Types>::src,
        src_doc = <TypeDefs as super::Types>::src_doc,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type loading<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        allow = <TypeDefs as super::Types>::allow,
        allow_fullscreen = <TypeDefs as super::Types>::allow_fullscreen,
        allow_payment_request = <TypeDefs as super::Types>::allow_payment_request,
        csp = <TypeDefs as super::Types>::csp,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        height = <TypeDefs as super::Types>::height,
        loading = Value,
        name = <TypeDefs as super::Types>::name,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        sandbox = <TypeDefs as super::Types>::sandbox,
        src = <TypeDefs as super::Types>::src,
        src_doc = <TypeDefs as super::Types>::src_doc,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type name<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        allow = <TypeDefs as super::Types>::allow,
        allow_fullscreen = <TypeDefs as super::Types>::allow_fullscreen,
        allow_payment_request = <TypeDefs as super::Types>::allow_payment_request,
        csp = <TypeDefs as super::Types>::csp,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        height = <TypeDefs as super::Types>::height,
        loading = <TypeDefs as super::Types>::loading,
        name = Value,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        sandbox = <TypeDefs as super::Types>::sandbox,
        src = <TypeDefs as super::Types>::src,
        src_doc = <TypeDefs as super::Types>::src_doc,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type referrer_policy<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        allow = <TypeDefs as super::Types>::allow,
        allow_fullscreen = <TypeDefs as super::Types>::allow_fullscreen,
        allow_payment_request = <TypeDefs as super::Types>::allow_payment_request,
        csp = <TypeDefs as super::Types>::csp,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        height = <TypeDefs as super::Types>::height,
        loading = <TypeDefs as super::Types>::loading,
        name = <TypeDefs as super::Types>::name,
        referrer_policy = Value,
        sandbox = <TypeDefs as super::Types>::sandbox,
        src = <TypeDefs as super::Types>::src,
        src_doc = <TypeDefs as super::Types>::src_doc,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type sandbox<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        allow = <TypeDefs as super::Types>::allow,
        allow_fullscreen = <TypeDefs as super::Types>::allow_fullscreen,
        allow_payment_request = <TypeDefs as super::Types>::allow_payment_request,
        csp = <TypeDefs as super::Types>::csp,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        height = <TypeDefs as super::Types>::height,
        loading = <TypeDefs as super::Types>::loading,
        name = <TypeDefs as super::Types>::name,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        sandbox = Value,
        src = <TypeDefs as super::Types>::src,
        src_doc = <TypeDefs as super::Types>::src_doc,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type src<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        allow = <TypeDefs as super::Types>::allow,
        allow_fullscreen = <TypeDefs as super::Types>::allow_fullscreen,
        allow_payment_request = <TypeDefs as super::Types>::allow_payment_request,
        csp = <TypeDefs as super::Types>::csp,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        height = <TypeDefs as super::Types>::height,
        loading = <TypeDefs as super::Types>::loading,
        name = <TypeDefs as super::Types>::name,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        sandbox = <TypeDefs as super::Types>::sandbox,
        src = Value,
        src_doc = <TypeDefs as super::Types>::src_doc,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type src_doc<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        allow = <TypeDefs as super::Types>::allow,
        allow_fullscreen = <TypeDefs as super::Types>::allow_fullscreen,
        allow_payment_request = <TypeDefs as super::Types>::allow_payment_request,
        csp = <TypeDefs as super::Types>::csp,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        height = <TypeDefs as super::Types>::height,
        loading = <TypeDefs as super::Types>::loading,
        name = <TypeDefs as super::Types>::name,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        sandbox = <TypeDefs as super::Types>::sandbox,
        src = <TypeDefs as super::Types>::src,
        src_doc = Value,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type width<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        allow = <TypeDefs as super::Types>::allow,
        allow_fullscreen = <TypeDefs as super::Types>::allow_fullscreen,
        allow_payment_request = <TypeDefs as super::Types>::allow_payment_request,
        csp = <TypeDefs as super::Types>::csp,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        height = <TypeDefs as super::Types>::height,
        loading = <TypeDefs as super::Types>::loading,
        name = <TypeDefs as super::Types>::name,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        sandbox = <TypeDefs as super::Types>::sandbox,
        src = <TypeDefs as super::Types>::src,
        src_doc = <TypeDefs as super::Types>::src_doc,
        width = Value,
    >;
}
mod trait_types {
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub trait Types {
        type HtmlElementProps: ?::core::marker::Sized + HtmlElementProps::Types;
        type allow: crate::MaybeUpdateValueWithState<str>;
        type allow_fullscreen: crate::MaybeUpdateValueWithState<bool>;
        type allow_payment_request: crate::MaybeUpdateValueWithState<bool>;
        type csp: crate::MaybeUpdateValueWithState<str>;
        type fetch_priority: crate::MaybeUpdateValueWithState<str>;
        type height: crate::MaybeUpdateValueWithState<str>;
        type loading: crate::MaybeUpdateValueWithState<str>;
        type name: crate::MaybeUpdateValueWithState<str>;
        type referrer_policy: crate::MaybeUpdateValueWithState<str>;
        type sandbox: crate::MaybeUpdateValueWithState<str>;
        type src: crate::MaybeUpdateValueWithState<str>;
        type src_doc: crate::MaybeUpdateValueWithState<str>;
        type width: crate::MaybeUpdateValueWithState<str>;
    }
}
pub use trait_types::Types;
pub use trait_types::Types as ValidTypes;
pub mod data_struct {
    #[non_exhaustive]
    pub struct HtmlIFrameElementProps<TypeDefs: super::Types + ?::core::marker::Sized> {
        pub HtmlElementProps: super::super::HtmlElementProps::Data<TypeDefs::HtmlElementProps>,
        pub allow: TypeDefs::allow,
        pub allow_fullscreen: TypeDefs::allow_fullscreen,
        pub allow_payment_request: TypeDefs::allow_payment_request,
        pub csp: TypeDefs::csp,
        pub fetch_priority: TypeDefs::fetch_priority,
        pub height: TypeDefs::height,
        pub loading: TypeDefs::loading,
        pub name: TypeDefs::name,
        pub referrer_policy: TypeDefs::referrer_policy,
        pub sandbox: TypeDefs::sandbox,
        pub src: TypeDefs::src,
        pub src_doc: TypeDefs::src_doc,
        pub width: TypeDefs::width,
    }
}
pub use ::core::convert::identity as Building;
pub use ::core::convert::identity as build;
pub use data_struct::HtmlIFrameElementProps as Data;
pub use data_struct::HtmlIFrameElementProps as Building;
pub struct Replacing<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
mod types_initial {
    #[allow(unused_imports)]
    use super::super::*;
    pub type TypesInitial = dyn super::Types<
        HtmlElementProps = HtmlElementProps::TypesInitial,
        allow = (),
        allow_fullscreen = (),
        allow_payment_request = (),
        csp = (),
        fetch_priority = (),
        height = (),
        loading = (),
        name = (),
        referrer_policy = (),
        sandbox = (),
        src = (),
        src_doc = (),
        width = (),
    >;
}
pub use types_initial::TypesInitial;
pub type DataInitial = Data<TypesInitial>;
#[cfg(feature = "dom")]
pub mod render_state {
    #[allow(non_camel_case_types)]
    pub trait RenderStateTypes {
        type HtmlElementProps: crate::props::IntrinsicComponentPollReactive;
        type allow;
        type allow_fullscreen;
        type allow_payment_request;
        type csp;
        type fetch_priority;
        type height;
        type loading;
        type name;
        type referrer_policy;
        type sandbox;
        type src;
        type src_doc;
        type width;
    }
    pub struct RenderState<TypeDefs: RenderStateTypes>
    where
        TypeDefs: ?::core::marker::Sized,
    {
        pub HtmlElementProps: TypeDefs::HtmlElementProps,
        pub allow: TypeDefs::allow,
        pub allow_fullscreen: TypeDefs::allow_fullscreen,
        pub allow_payment_request: TypeDefs::allow_payment_request,
        pub csp: TypeDefs::csp,
        pub fetch_priority: TypeDefs::fetch_priority,
        pub height: TypeDefs::height,
        pub loading: TypeDefs::loading,
        pub name: TypeDefs::name,
        pub referrer_policy: TypeDefs::referrer_policy,
        pub sandbox: TypeDefs::sandbox,
        pub src: TypeDefs::src,
        pub src_doc: TypeDefs::src_doc,
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
        pub allow: &'__pin mut (TypeDefs::allow),
        pub allow_fullscreen: &'__pin mut (TypeDefs::allow_fullscreen),
        pub allow_payment_request: &'__pin mut (TypeDefs::allow_payment_request),
        pub csp: &'__pin mut (TypeDefs::csp),
        pub fetch_priority: &'__pin mut (TypeDefs::fetch_priority),
        pub height: &'__pin mut (TypeDefs::height),
        pub loading: &'__pin mut (TypeDefs::loading),
        pub name: &'__pin mut (TypeDefs::name),
        pub referrer_policy: &'__pin mut (TypeDefs::referrer_policy),
        pub sandbox: &'__pin mut (TypeDefs::sandbox),
        pub src: &'__pin mut (TypeDefs::src),
        pub src_doc: &'__pin mut (TypeDefs::src_doc),
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
            pub allow: &'__pin (TypeDefs::allow),
            pub allow_fullscreen: &'__pin (TypeDefs::allow_fullscreen),
            pub allow_payment_request: &'__pin (TypeDefs::allow_payment_request),
            pub csp: &'__pin (TypeDefs::csp),
            pub fetch_priority: &'__pin (TypeDefs::fetch_priority),
            pub height: &'__pin (TypeDefs::height),
            pub loading: &'__pin (TypeDefs::loading),
            pub name: &'__pin (TypeDefs::name),
            pub referrer_policy: &'__pin (TypeDefs::referrer_policy),
            pub sandbox: &'__pin (TypeDefs::sandbox),
            pub src: &'__pin (TypeDefs::src),
            pub src_doc: &'__pin (TypeDefs::src_doc),
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
                        allow,
                        allow_fullscreen,
                        allow_payment_request,
                        csp,
                        fetch_priority,
                        height,
                        loading,
                        name,
                        referrer_policy,
                        sandbox,
                        src,
                        src_doc,
                        width,
                    } = self.get_unchecked_mut();
                    RenderStateProj {
                        HtmlElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            HtmlElementProps,
                        ),
                        allow: allow,
                        allow_fullscreen: allow_fullscreen,
                        allow_payment_request: allow_payment_request,
                        csp: csp,
                        fetch_priority: fetch_priority,
                        height: height,
                        loading: loading,
                        name: name,
                        referrer_policy: referrer_policy,
                        sandbox: sandbox,
                        src: src,
                        src_doc: src_doc,
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
                        allow,
                        allow_fullscreen,
                        allow_payment_request,
                        csp,
                        fetch_priority,
                        height,
                        loading,
                        name,
                        referrer_policy,
                        sandbox,
                        src,
                        src_doc,
                        width,
                    } = self.get_ref();
                    ProjectionRef {
                        HtmlElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            HtmlElementProps,
                        ),
                        allow: allow,
                        allow_fullscreen: allow_fullscreen,
                        allow_payment_request: allow_payment_request,
                        csp: csp,
                        fetch_priority: fetch_priority,
                        height: height,
                        loading: loading,
                        name: name,
                        referrer_policy: referrer_policy,
                        sandbox: sandbox,
                        src: src,
                        src_doc: src_doc,
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
            allow: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::allow>,
            allow_fullscreen:
                ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::allow_fullscreen>,
            allow_payment_request:
                ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::allow_payment_request>,
            csp: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::csp>,
            fetch_priority: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::fetch_priority>,
            height: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::height>,
            loading: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::loading>,
            name: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::name>,
            referrer_policy: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::referrer_policy>,
            sandbox: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::sandbox>,
            src: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::src>,
            src_doc: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::src_doc>,
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
            let _ = &this.allow;
            let _ = &this.allow_fullscreen;
            let _ = &this.allow_payment_request;
            let _ = &this.csp;
            let _ = &this.fetch_priority;
            let _ = &this.height;
            let _ = &this.loading;
            let _ = &this.name;
            let _ = &this.referrer_policy;
            let _ = &this.sandbox;
            let _ = &this.src;
            let _ = &this.src_doc;
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
            }
        }
        #[inline(always)]
        pub fn allow<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            allow: V,
        ) -> super::Building<super::overwrite::allow<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
            }
        }
        #[inline(always)]
        pub fn allow_fullscreen<V: crate::MaybeUpdateValueWithState<bool>>(
            self,
            allow_fullscreen: V,
        ) -> super::Building<super::overwrite::allow_fullscreen<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                allow: self.allow,
                allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
            }
        }
        #[inline(always)]
        pub fn allow_payment_request<V: crate::MaybeUpdateValueWithState<bool>>(
            self,
            allow_payment_request: V,
        ) -> super::Building<super::overwrite::allow_payment_request<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
            }
        }
        #[inline(always)]
        pub fn csp<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            csp: V,
        ) -> super::Building<super::overwrite::csp<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
            }
        }
        #[inline(always)]
        pub fn fetch_priority<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            fetch_priority: V,
        ) -> super::Building<super::overwrite::fetch_priority<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
            }
        }
        #[inline(always)]
        pub fn height<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            height: V,
        ) -> super::Building<super::overwrite::height<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
            }
        }
        #[inline(always)]
        pub fn loading<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            loading: V,
        ) -> super::Building<super::overwrite::loading<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
            }
        }
        #[inline(always)]
        pub fn name<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            name: V,
        ) -> super::Building<super::overwrite::name<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
            }
        }
        #[inline(always)]
        pub fn referrer_policy<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            referrer_policy: V,
        ) -> super::Building<super::overwrite::referrer_policy<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
            }
        }
        #[inline(always)]
        pub fn sandbox<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            sandbox: V,
        ) -> super::Building<super::overwrite::sandbox<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox,
                src: self.src,
                src_doc: self.src_doc,
                width: self.width,
            }
        }
        #[inline(always)]
        pub fn src<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            src: V,
        ) -> super::Building<super::overwrite::src<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src,
                src_doc: self.src_doc,
                width: self.width,
            }
        }
        #[inline(always)]
        pub fn src_doc<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            src_doc: V,
        ) -> super::Building<super::overwrite::src_doc<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc,
                width: self.width,
            }
        }
        #[inline(always)]
        pub fn width<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            width: V,
        ) -> super::Building<super::overwrite::width<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                allow: self.allow,
                allow_fullscreen: self.allow_fullscreen,
                allow_payment_request: self.allow_payment_request,
                csp: self.csp,
                fetch_priority: self.fetch_priority,
                height: self.height,
                loading: self.loading,
                name: self.name,
                referrer_policy: self.referrer_policy,
                sandbox: self.sandbox,
                src: self.src,
                src_doc: self.src_doc,
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
        crate::props::UpdateElement<web_sys::HtmlIFrameElement> for super::Data<TypeDefs>
    where
        HtmlElementProps::Data<TypeDefs::HtmlElementProps>:
            crate::props::UpdateElement<web_sys::HtmlElement>,
    {
        type State = super::render_state::RenderState<
            dyn super::render_state::RenderStateTypes<
                HtmlElementProps = <HtmlElementProps::Data<
                    TypeDefs::HtmlElementProps,
                > as crate::props::UpdateElement<web_sys::HtmlElement>>::State,
                allow = <TypeDefs::allow as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                allow_fullscreen = <TypeDefs::allow_fullscreen as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::State,
                allow_payment_request = <TypeDefs::allow_payment_request as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::State,
                csp = <TypeDefs::csp as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                fetch_priority = <TypeDefs::fetch_priority as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                height = <TypeDefs::height as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                loading = <TypeDefs::loading as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                name = <TypeDefs::name as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                referrer_policy = <TypeDefs::referrer_policy as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                sandbox = <TypeDefs::sandbox as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                src = <TypeDefs::src as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                src_doc = <TypeDefs::src_doc as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                width = <TypeDefs::width as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
            >,
        >;
        fn initialize_state(
            this: Self,
            element: &web_sys::HtmlIFrameElement,
            children_ctx: &mut ::frender_dom::Dom,
        ) -> Self::State {
            let dom_element: &::web_sys::Element = element.as_ref();
            super::render_state::RenderState {
                HtmlElementProps: <HtmlElementProps::Data<
                    TypeDefs::HtmlElementProps,
                > as crate::props::UpdateElement<
                    web_sys::HtmlElement,
                >>::initialize_state(this.HtmlElementProps, element, children_ctx),
                allow: <TypeDefs::allow as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.allow,
                    |v| crate::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "allow",
                    ),
                    || dom_element.remove_attribute("allow").unwrap(),
                ),
                allow_fullscreen: <TypeDefs::allow_fullscreen as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.allow_fullscreen,
                    |v| element.set_allow_fullscreen(*v),
                    || dom_element.remove_attribute("allowfullscreen").unwrap(),
                ),
                allow_payment_request: <TypeDefs::allow_payment_request as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.allow_payment_request,
                    |v| element.set_allow_payment_request(*v),
                    || { dom_element.remove_attribute("allowpaymentrequest").unwrap() },
                ),
                csp: <TypeDefs::csp as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.csp,
                    |v| crate::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "csp",
                    ),
                    || dom_element.remove_attribute("csp").unwrap(),
                ),
                fetch_priority: <TypeDefs::fetch_priority as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.fetch_priority,
                    |v| crate::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "fetchpriority",
                    ),
                    || dom_element.remove_attribute("fetchpriority").unwrap(),
                ),
                height: <TypeDefs::height as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.height,
                    |v| element.set_height(v),
                    || dom_element.remove_attribute("height").unwrap(),
                ),
                loading: <TypeDefs::loading as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.loading,
                    |v| crate::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "loading",
                    ),
                    || dom_element.remove_attribute("loading").unwrap(),
                ),
                name: <TypeDefs::name as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.name,
                    |v| element.set_name(v),
                    || dom_element.remove_attribute("name").unwrap(),
                ),
                referrer_policy: <TypeDefs::referrer_policy as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.referrer_policy,
                    |v| element.set_referrer_policy(v),
                    || dom_element.remove_attribute("referrerpolicy").unwrap(),
                ),
                sandbox: <TypeDefs::sandbox as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.sandbox,
                    |v| crate::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "sandbox",
                    ),
                    || dom_element.remove_attribute("sandbox").unwrap(),
                ),
                src: <TypeDefs::src as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.src,
                    |v| element.set_src(v),
                    || dom_element.remove_attribute("src").unwrap(),
                ),
                src_doc: <TypeDefs::src_doc as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.src_doc,
                    |v| element.set_srcdoc(v),
                    || dom_element.remove_attribute("srcdoc").unwrap(),
                ),
                width: <TypeDefs::width as ::frender_dom::props::MaybeUpdateValueWithState<
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
            element: &web_sys::HtmlIFrameElement,
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
            <TypeDefs::allow as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.allow,
                state.allow,
                |v| crate::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "allow",
                ),
                || dom_element.remove_attribute("allow").unwrap(),
            );
            <TypeDefs::allow_fullscreen as ::frender_dom::props::MaybeUpdateValueWithState<
                bool,
            >>::maybe_update_value_with_state(
                this.allow_fullscreen,
                state.allow_fullscreen,
                |v| element.set_allow_fullscreen(*v),
                || dom_element.remove_attribute("allowfullscreen").unwrap(),
            );
            <TypeDefs::allow_payment_request as ::frender_dom::props::MaybeUpdateValueWithState<
                bool,
            >>::maybe_update_value_with_state(
                this.allow_payment_request,
                state.allow_payment_request,
                |v| element.set_allow_payment_request(*v),
                || dom_element.remove_attribute("allowpaymentrequest").unwrap(),
            );
            <TypeDefs::csp as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.csp,
                state.csp,
                |v| crate::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "csp",
                ),
                || dom_element.remove_attribute("csp").unwrap(),
            );
            <TypeDefs::fetch_priority as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.fetch_priority,
                state.fetch_priority,
                |v| crate::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "fetchpriority",
                ),
                || dom_element.remove_attribute("fetchpriority").unwrap(),
            );
            <TypeDefs::height as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.height,
                state.height,
                |v| element.set_height(v),
                || dom_element.remove_attribute("height").unwrap(),
            );
            <TypeDefs::loading as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.loading,
                state.loading,
                |v| crate::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "loading",
                ),
                || dom_element.remove_attribute("loading").unwrap(),
            );
            <TypeDefs::name as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.name,
                state.name,
                |v| element.set_name(v),
                || dom_element.remove_attribute("name").unwrap(),
            );
            <TypeDefs::referrer_policy as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.referrer_policy,
                state.referrer_policy,
                |v| element.set_referrer_policy(v),
                || dom_element.remove_attribute("referrerpolicy").unwrap(),
            );
            <TypeDefs::sandbox as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.sandbox,
                state.sandbox,
                |v| crate::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "sandbox",
                ),
                || dom_element.remove_attribute("sandbox").unwrap(),
            );
            <TypeDefs::src as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.src,
                state.src,
                |v| element.set_src(v),
                || dom_element.remove_attribute("src").unwrap(),
            );
            <TypeDefs::src_doc as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.src_doc,
                state.src_doc,
                |v| element.set_srcdoc(v),
                || dom_element.remove_attribute("srcdoc").unwrap(),
            );
            <TypeDefs::width as ::frender_dom::props::MaybeUpdateValueWithState<
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
                                <TypeDefs::allow as ::frender_dom::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.allow)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("allow"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::allow_fullscreen as ::frender_dom::props::MaybeUpdateValueWithState<
                                    bool,
                                >>::maybe_into_html_attribute_value(this.allow_fullscreen)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("allowfullscreen"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::allow_payment_request as ::frender_dom::props::MaybeUpdateValueWithState<
                                    bool,
                                >>::maybe_into_html_attribute_value(
                                        this.allow_payment_request,
                                    )
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("allowpaymentrequest"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::csp as ::frender_dom::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.csp)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("csp"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::fetch_priority as ::frender_dom::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.fetch_priority)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("fetchpriority"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::height as ::frender_dom::props::MaybeUpdateValueWithState<
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
                                <TypeDefs::loading as ::frender_dom::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.loading)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("loading"),
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
                                <TypeDefs::referrer_policy as ::frender_dom::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.referrer_policy)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("referrerpolicy"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::sandbox as ::frender_dom::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.sandbox)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("sandbox"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::src as ::frender_dom::props::MaybeUpdateValueWithState<
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
                                <TypeDefs::src_doc as ::frender_dom::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.src_doc)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("srcdoc"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::width as ::frender_dom::props::MaybeUpdateValueWithState<
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
