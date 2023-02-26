#[allow(non_snake_case)]
#[inline(always)]
pub fn HtmlAreaElementProps() -> Building<TypesInitial> {
    #[allow(unused_imports)]
    use super::*;
    self::Building {
        HtmlElementProps: HtmlElementProps::build(HtmlElementProps()),
        download: (),
        href: (),
        ping: (),
        referrer_policy: (),
        rel: (),
        target: (),
        alt: (),
        coords: (),
        shape: (),
    }
}
pub mod prelude {}
pub mod overwrite {
    #![allow(non_camel_case_types)]
    pub type HtmlElementProps<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = Value,
        download = <TypeDefs as super::Types>::download,
        href = <TypeDefs as super::Types>::href,
        ping = <TypeDefs as super::Types>::ping,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        rel = <TypeDefs as super::Types>::rel,
        target = <TypeDefs as super::Types>::target,
        alt = <TypeDefs as super::Types>::alt,
        coords = <TypeDefs as super::Types>::coords,
        shape = <TypeDefs as super::Types>::shape,
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
    pub type download<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        download = Value,
        href = <TypeDefs as super::Types>::href,
        ping = <TypeDefs as super::Types>::ping,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        rel = <TypeDefs as super::Types>::rel,
        target = <TypeDefs as super::Types>::target,
        alt = <TypeDefs as super::Types>::alt,
        coords = <TypeDefs as super::Types>::coords,
        shape = <TypeDefs as super::Types>::shape,
    >;
    pub type href<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        download = <TypeDefs as super::Types>::download,
        href = Value,
        ping = <TypeDefs as super::Types>::ping,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        rel = <TypeDefs as super::Types>::rel,
        target = <TypeDefs as super::Types>::target,
        alt = <TypeDefs as super::Types>::alt,
        coords = <TypeDefs as super::Types>::coords,
        shape = <TypeDefs as super::Types>::shape,
    >;
    pub type ping<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        download = <TypeDefs as super::Types>::download,
        href = <TypeDefs as super::Types>::href,
        ping = Value,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        rel = <TypeDefs as super::Types>::rel,
        target = <TypeDefs as super::Types>::target,
        alt = <TypeDefs as super::Types>::alt,
        coords = <TypeDefs as super::Types>::coords,
        shape = <TypeDefs as super::Types>::shape,
    >;
    pub type referrer_policy<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        download = <TypeDefs as super::Types>::download,
        href = <TypeDefs as super::Types>::href,
        ping = <TypeDefs as super::Types>::ping,
        referrer_policy = Value,
        rel = <TypeDefs as super::Types>::rel,
        target = <TypeDefs as super::Types>::target,
        alt = <TypeDefs as super::Types>::alt,
        coords = <TypeDefs as super::Types>::coords,
        shape = <TypeDefs as super::Types>::shape,
    >;
    pub type rel<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        download = <TypeDefs as super::Types>::download,
        href = <TypeDefs as super::Types>::href,
        ping = <TypeDefs as super::Types>::ping,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        rel = Value,
        target = <TypeDefs as super::Types>::target,
        alt = <TypeDefs as super::Types>::alt,
        coords = <TypeDefs as super::Types>::coords,
        shape = <TypeDefs as super::Types>::shape,
    >;
    pub type target<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        download = <TypeDefs as super::Types>::download,
        href = <TypeDefs as super::Types>::href,
        ping = <TypeDefs as super::Types>::ping,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        rel = <TypeDefs as super::Types>::rel,
        target = Value,
        alt = <TypeDefs as super::Types>::alt,
        coords = <TypeDefs as super::Types>::coords,
        shape = <TypeDefs as super::Types>::shape,
    >;
    pub type alt<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        download = <TypeDefs as super::Types>::download,
        href = <TypeDefs as super::Types>::href,
        ping = <TypeDefs as super::Types>::ping,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        rel = <TypeDefs as super::Types>::rel,
        target = <TypeDefs as super::Types>::target,
        alt = Value,
        coords = <TypeDefs as super::Types>::coords,
        shape = <TypeDefs as super::Types>::shape,
    >;
    pub type coords<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        download = <TypeDefs as super::Types>::download,
        href = <TypeDefs as super::Types>::href,
        ping = <TypeDefs as super::Types>::ping,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        rel = <TypeDefs as super::Types>::rel,
        target = <TypeDefs as super::Types>::target,
        alt = <TypeDefs as super::Types>::alt,
        coords = Value,
        shape = <TypeDefs as super::Types>::shape,
    >;
    pub type shape<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        download = <TypeDefs as super::Types>::download,
        href = <TypeDefs as super::Types>::href,
        ping = <TypeDefs as super::Types>::ping,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        rel = <TypeDefs as super::Types>::rel,
        target = <TypeDefs as super::Types>::target,
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
        type HtmlElementProps: ?::core::marker::Sized + HtmlElementProps::Types;
        type download: crate::imports::MaybeUpdateValueWithState<str>;
        type href: crate::imports::MaybeUpdateValueWithState<str>;
        type ping: crate::imports::MaybeUpdateValueWithState<str>;
        type referrer_policy: crate::imports::MaybeUpdateValueWithState<str>;
        type rel: crate::imports::MaybeUpdateValueWithState<str>;
        type target: crate::imports::MaybeUpdateValueWithState<str>;
        type alt: crate::imports::MaybeUpdateValueWithState<str>;
        type coords: crate::imports::MaybeUpdateValueWithState<str>;
        type shape: crate::imports::MaybeUpdateValueWithState<str>;
    }
}
pub use trait_types::Types;
pub use trait_types::Types as ValidTypes;
pub mod data_struct {
    #[non_exhaustive]
    pub struct HtmlAreaElementProps<TypeDefs: super::Types + ?::core::marker::Sized> {
        pub HtmlElementProps: super::super::HtmlElementProps::Data<TypeDefs::HtmlElementProps>,
        pub download: TypeDefs::download,
        pub href: TypeDefs::href,
        pub ping: TypeDefs::ping,
        pub referrer_policy: TypeDefs::referrer_policy,
        pub rel: TypeDefs::rel,
        pub target: TypeDefs::target,
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
        HtmlElementProps = HtmlElementProps::TypesInitial,
        download = (),
        href = (),
        ping = (),
        referrer_policy = (),
        rel = (),
        target = (),
        alt = (),
        coords = (),
        shape = (),
    >;
}
pub use types_initial::TypesInitial;
pub type DataInitial = Data<TypesInitial>;
#[cfg(feature = "dom")]
pub mod render_state {
    #[allow(non_camel_case_types)]
    pub trait RenderStateTypes {
        type HtmlElementProps: crate::imports::props::IntrinsicComponentPollReactive;
        type download;
        type href;
        type ping;
        type referrer_policy;
        type rel;
        type target;
        type alt;
        type coords;
        type shape;
    }
    pub struct RenderState<TypeDefs: RenderStateTypes>
    where
        TypeDefs: ?::core::marker::Sized,
    {
        pub HtmlElementProps: TypeDefs::HtmlElementProps,
        pub download: TypeDefs::download,
        pub href: TypeDefs::href,
        pub ping: TypeDefs::ping,
        pub referrer_policy: TypeDefs::referrer_policy,
        pub rel: TypeDefs::rel,
        pub target: TypeDefs::target,
        pub alt: TypeDefs::alt,
        pub coords: TypeDefs::coords,
        pub shape: TypeDefs::shape,
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
        pub download: &'__pin mut (TypeDefs::download),
        pub href: &'__pin mut (TypeDefs::href),
        pub ping: &'__pin mut (TypeDefs::ping),
        pub referrer_policy: &'__pin mut (TypeDefs::referrer_policy),
        pub rel: &'__pin mut (TypeDefs::rel),
        pub target: &'__pin mut (TypeDefs::target),
        pub alt: &'__pin mut (TypeDefs::alt),
        pub coords: &'__pin mut (TypeDefs::coords),
        pub shape: &'__pin mut (TypeDefs::shape),
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
            pub download: &'__pin (TypeDefs::download),
            pub href: &'__pin (TypeDefs::href),
            pub ping: &'__pin (TypeDefs::ping),
            pub referrer_policy: &'__pin (TypeDefs::referrer_policy),
            pub rel: &'__pin (TypeDefs::rel),
            pub target: &'__pin (TypeDefs::target),
            pub alt: &'__pin (TypeDefs::alt),
            pub coords: &'__pin (TypeDefs::coords),
            pub shape: &'__pin (TypeDefs::shape),
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
                        download,
                        href,
                        ping,
                        referrer_policy,
                        rel,
                        target,
                        alt,
                        coords,
                        shape,
                    } = self.get_unchecked_mut();
                    RenderStateProj {
                        HtmlElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            HtmlElementProps,
                        ),
                        download: download,
                        href: href,
                        ping: ping,
                        referrer_policy: referrer_policy,
                        rel: rel,
                        target: target,
                        alt: alt,
                        coords: coords,
                        shape: shape,
                    }
                }
            }
            pub(crate) fn project_ref<'__pin>(
                self: ::pin_project_lite::__private::Pin<&'__pin Self>,
            ) -> ProjectionRef<'__pin, TypeDefs> {
                unsafe {
                    let Self {
                        HtmlElementProps,
                        download,
                        href,
                        ping,
                        referrer_policy,
                        rel,
                        target,
                        alt,
                        coords,
                        shape,
                    } = self.get_ref();
                    ProjectionRef {
                        HtmlElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            HtmlElementProps,
                        ),
                        download: download,
                        href: href,
                        ping: ping,
                        referrer_policy: referrer_policy,
                        rel: rel,
                        target: target,
                        alt: alt,
                        coords: coords,
                        shape: shape,
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
            download: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::download>,
            href: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::href>,
            ping: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::ping>,
            referrer_policy: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::referrer_policy>,
            rel: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::rel>,
            target: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::target>,
            alt: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::alt>,
            coords: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::coords>,
            shape: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::shape>,
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
            let _ = &this.download;
            let _ = &this.href;
            let _ = &this.ping;
            let _ = &this.referrer_policy;
            let _ = &this.rel;
            let _ = &this.target;
            let _ = &this.alt;
            let _ = &this.coords;
            let _ = &this.shape;
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
        crate::imports::props::IntrinsicComponentPollReactive for RenderState<TypeDefs>
    {
        #[inline]
        fn intrinsic_component_poll_reactive(
            self: ::core::pin::Pin<&mut Self>,
            cx: &mut ::core::task::Context<'_>,
        ) -> ::core::task::Poll<bool> {
            crate::imports::props::IntrinsicComponentPollReactive::intrinsic_component_poll_reactive(
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementProps::class`]
        #[inline(always)]
        pub fn class<V: crate::imports::MaybeUpdateValueWithState<str>>(
            self,
            class: V,
        ) -> super::Building<super::overwrite::class<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.class(class),
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementProps::id`]
        #[inline(always)]
        pub fn id<V: crate::imports::MaybeUpdateValueWithState<str>>(
            self,
            id: V,
        ) -> super::Building<super::overwrite::id<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.id(id),
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementProps::part`]
        #[inline(always)]
        pub fn part<V: crate::imports::MaybeUpdateValueWithState<str>>(
            self,
            part: V,
        ) -> super::Building<super::overwrite::part<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.part(part),
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementProps::access_key`]
        #[inline(always)]
        pub fn access_key<V: crate::imports::MaybeUpdateValueWithState<str>>(
            self,
            access_key: V,
        ) -> super::Building<super::overwrite::access_key<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.access_key(access_key),
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementProps::auto_capitalize`]
        #[inline(always)]
        pub fn auto_capitalize<V: crate::imports::MaybeUpdateValueWithState<str>>(
            self,
            auto_capitalize: V,
        ) -> super::Building<super::overwrite::auto_capitalize<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.auto_capitalize(auto_capitalize),
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementProps::auto_focus`]
        #[inline(always)]
        pub fn auto_focus<V: crate::imports::MaybeUpdateValueWithState<bool>>(
            self,
            auto_focus: V,
        ) -> super::Building<super::overwrite::auto_focus<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.auto_focus(auto_focus),
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementProps::context_menu`]
        #[inline(always)]
        pub fn context_menu<V: crate::imports::MaybeUpdateValueWithState<str>>(
            self,
            context_menu: V,
        ) -> super::Building<super::overwrite::context_menu<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.context_menu(context_menu),
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementProps::dir`]
        #[inline(always)]
        pub fn dir<V: crate::imports::MaybeUpdateValueWithState<str>>(
            self,
            dir: V,
        ) -> super::Building<super::overwrite::dir<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.dir(dir),
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementProps::draggable`]
        #[inline(always)]
        pub fn draggable<V: crate::imports::MaybeUpdateValueWithState<bool>>(
            self,
            draggable: V,
        ) -> super::Building<super::overwrite::draggable<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.draggable(draggable),
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementProps::enter_key_hint`]
        #[inline(always)]
        pub fn enter_key_hint<V: crate::imports::MaybeUpdateValueWithState<str>>(
            self,
            enter_key_hint: V,
        ) -> super::Building<super::overwrite::enter_key_hint<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.enter_key_hint(enter_key_hint),
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementProps::hidden`]
        #[inline(always)]
        pub fn hidden<V: crate::imports::MaybeUpdateValueWithState<bool>>(
            self,
            hidden: V,
        ) -> super::Building<super::overwrite::hidden<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.hidden(hidden),
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementProps::inert`]
        #[inline(always)]
        pub fn inert<V: crate::imports::MaybeUpdateValueWithState<bool>>(
            self,
            inert: V,
        ) -> super::Building<super::overwrite::inert<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.inert(inert),
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementProps::input_mode`]
        #[inline(always)]
        pub fn input_mode<V: crate::imports::MaybeUpdateValueWithState<str>>(
            self,
            input_mode: V,
        ) -> super::Building<super::overwrite::input_mode<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.input_mode(input_mode),
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementProps::is`]
        #[inline(always)]
        pub fn is<V: crate::imports::MaybeUpdateValueWithState<str>>(
            self,
            is: V,
        ) -> super::Building<super::overwrite::is<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.is(is),
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementProps::item_id`]
        #[inline(always)]
        pub fn item_id<V: crate::imports::MaybeUpdateValueWithState<str>>(
            self,
            item_id: V,
        ) -> super::Building<super::overwrite::item_id<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.item_id(item_id),
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementProps::item_prop`]
        #[inline(always)]
        pub fn item_prop<V: crate::imports::MaybeUpdateValueWithState<str>>(
            self,
            item_prop: V,
        ) -> super::Building<super::overwrite::item_prop<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.item_prop(item_prop),
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementProps::item_ref`]
        #[inline(always)]
        pub fn item_ref<V: crate::imports::MaybeUpdateValueWithState<str>>(
            self,
            item_ref: V,
        ) -> super::Building<super::overwrite::item_ref<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.item_ref(item_ref),
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementProps::item_scope`]
        #[inline(always)]
        pub fn item_scope<V: crate::imports::MaybeUpdateValueWithState<str>>(
            self,
            item_scope: V,
        ) -> super::Building<super::overwrite::item_scope<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.item_scope(item_scope),
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementProps::item_type`]
        #[inline(always)]
        pub fn item_type<V: crate::imports::MaybeUpdateValueWithState<str>>(
            self,
            item_type: V,
        ) -> super::Building<super::overwrite::item_type<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.item_type(item_type),
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementProps::lang`]
        #[inline(always)]
        pub fn lang<V: crate::imports::MaybeUpdateValueWithState<str>>(
            self,
            lang: V,
        ) -> super::Building<super::overwrite::lang<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.lang(lang),
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementProps::nonce`]
        #[inline(always)]
        pub fn nonce<V: crate::imports::MaybeUpdateValueWithState<str>>(
            self,
            nonce: V,
        ) -> super::Building<super::overwrite::nonce<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.nonce(nonce),
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementProps::role`]
        #[inline(always)]
        pub fn role<V: crate::imports::MaybeUpdateValueWithState<str>>(
            self,
            role: V,
        ) -> super::Building<super::overwrite::role<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.role(role),
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementProps::slot`]
        #[inline(always)]
        pub fn slot<V: crate::imports::MaybeUpdateValueWithState<str>>(
            self,
            slot: V,
        ) -> super::Building<super::overwrite::slot<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.slot(slot),
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementProps::spellcheck`]
        #[inline(always)]
        pub fn spellcheck<V: crate::imports::MaybeUpdateValueWithState<bool>>(
            self,
            spellcheck: V,
        ) -> super::Building<super::overwrite::spellcheck<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.spellcheck(spellcheck),
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementProps::style`]
        #[inline(always)]
        pub fn style<V: crate::imports::MaybeUpdateValueWithState<str>>(
            self,
            style: V,
        ) -> super::Building<super::overwrite::style<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.style(style),
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementProps::tab_index`]
        #[inline(always)]
        pub fn tab_index<V: crate::imports::MaybeUpdateValueWithState<i32>>(
            self,
            tab_index: V,
        ) -> super::Building<super::overwrite::tab_index<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.tab_index(tab_index),
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementProps::title`]
        #[inline(always)]
        pub fn title<V: crate::imports::MaybeUpdateValueWithState<str>>(
            self,
            title: V,
        ) -> super::Building<super::overwrite::title<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.title(title),
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementProps::translate`]
        #[inline(always)]
        pub fn translate<V: crate::imports::MaybeUpdateValueWithState<str>>(
            self,
            translate: V,
        ) -> super::Building<super::overwrite::translate<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps.translate(translate),
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        ///See [`HtmlElementProps::virtual_keyboard_policy`]
        #[inline(always)]
        pub fn virtual_keyboard_policy<V: crate::imports::MaybeUpdateValueWithState<str>>(
            self,
            virtual_keyboard_policy: V,
        ) -> super::Building<super::overwrite::virtual_keyboard_policy<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self
                    .HtmlElementProps
                    .virtual_keyboard_policy(virtual_keyboard_policy),
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
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
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        #[inline(always)]
        pub fn download<V: crate::imports::MaybeUpdateValueWithState<str>>(
            self,
            download: V,
        ) -> super::Building<super::overwrite::download<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        #[inline(always)]
        pub fn href<V: crate::imports::MaybeUpdateValueWithState<str>>(
            self,
            href: V,
        ) -> super::Building<super::overwrite::href<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                download: self.download,
                href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        #[inline(always)]
        pub fn ping<V: crate::imports::MaybeUpdateValueWithState<str>>(
            self,
            ping: V,
        ) -> super::Building<super::overwrite::ping<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                download: self.download,
                href: self.href,
                ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        #[inline(always)]
        pub fn referrer_policy<V: crate::imports::MaybeUpdateValueWithState<str>>(
            self,
            referrer_policy: V,
        ) -> super::Building<super::overwrite::referrer_policy<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        #[inline(always)]
        pub fn rel<V: crate::imports::MaybeUpdateValueWithState<str>>(
            self,
            rel: V,
        ) -> super::Building<super::overwrite::rel<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        #[inline(always)]
        pub fn target<V: crate::imports::MaybeUpdateValueWithState<str>>(
            self,
            target: V,
        ) -> super::Building<super::overwrite::target<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target,
                alt: self.alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        #[inline(always)]
        pub fn alt<V: crate::imports::MaybeUpdateValueWithState<str>>(
            self,
            alt: V,
        ) -> super::Building<super::overwrite::alt<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt,
                coords: self.coords,
                shape: self.shape,
            }
        }
        #[inline(always)]
        pub fn coords<V: crate::imports::MaybeUpdateValueWithState<str>>(
            self,
            coords: V,
        ) -> super::Building<super::overwrite::coords<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords,
                shape: self.shape,
            }
        }
        #[inline(always)]
        pub fn shape<V: crate::imports::MaybeUpdateValueWithState<str>>(
            self,
            shape: V,
        ) -> super::Building<super::overwrite::shape<TypeDefs, V>> {
            super::Data {
                HtmlElementProps: self.HtmlElementProps,
                download: self.download,
                href: self.href,
                ping: self.ping,
                referrer_policy: self.referrer_policy,
                rel: self.rel,
                target: self.target,
                alt: self.alt,
                coords: self.coords,
                shape,
            }
        }
    }
}
#[cfg(feature = "dom")]
mod impl_update_element {
    #[allow(unused_imports)]
    use super::super::*;
    impl<TypeDefs: ?::core::marker::Sized + super::Types>
        crate::imports::props::UpdateElement<web_sys::HtmlAreaElement> for super::Data<TypeDefs>
    where
        HtmlElementProps::Data<TypeDefs::HtmlElementProps>:
            crate::imports::props::UpdateElement<web_sys::HtmlElement>,
    {
        type State = super::render_state::RenderState<
            dyn super::render_state::RenderStateTypes<
                HtmlElementProps = <HtmlElementProps::Data<
                    TypeDefs::HtmlElementProps,
                > as crate::imports::props::UpdateElement<web_sys::HtmlElement>>::State,
                download = <TypeDefs::download as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                href = <TypeDefs::href as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                ping = <TypeDefs::ping as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                referrer_policy = <TypeDefs::referrer_policy as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                rel = <TypeDefs::rel as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                target = <TypeDefs::target as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                alt = <TypeDefs::alt as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                coords = <TypeDefs::coords as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                shape = <TypeDefs::shape as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
            >,
        >;
        fn initialize_state(
            this: Self,
            element: &web_sys::HtmlAreaElement,
            children_ctx: &mut ::frender_dom::Dom,
        ) -> Self::State {
            let dom_element: &::web_sys::Element = element.as_ref();
            super::render_state::RenderState {
                HtmlElementProps: <HtmlElementProps::Data<
                    TypeDefs::HtmlElementProps,
                > as crate::imports::props::UpdateElement<
                    web_sys::HtmlElement,
                >>::initialize_state(this.HtmlElementProps, element, children_ctx),
                download: <TypeDefs::download as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.download,
                    |v| element.set_download(v),
                    || dom_element.remove_attribute("download").unwrap(),
                ),
                href: <TypeDefs::href as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.href,
                    |v| element.set_href(v),
                    || dom_element.remove_attribute("href").unwrap(),
                ),
                ping: <TypeDefs::ping as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.ping,
                    |v| element.set_ping(v),
                    || dom_element.remove_attribute("ping").unwrap(),
                ),
                referrer_policy: <TypeDefs::referrer_policy as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.referrer_policy,
                    |v| element.set_referrer_policy(v),
                    || dom_element.remove_attribute("referrerpolicy").unwrap(),
                ),
                rel: <TypeDefs::rel as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.rel,
                    |v| element.set_rel(v),
                    || dom_element.remove_attribute("rel").unwrap(),
                ),
                target: <TypeDefs::target as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.target,
                    |v| element.set_target(v),
                    || dom_element.remove_attribute("target").unwrap(),
                ),
                alt: <TypeDefs::alt as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.alt,
                    |v| element.set_alt(v),
                    || dom_element.remove_attribute("alt").unwrap(),
                ),
                coords: <TypeDefs::coords as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.coords,
                    |v| element.set_coords(v),
                    || dom_element.remove_attribute("coords").unwrap(),
                ),
                shape: <TypeDefs::shape as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.shape,
                    |v| element.set_shape(v),
                    || dom_element.remove_attribute("shape").unwrap(),
                ),
            }
        }
        fn update_element(
            this: Self,
            element: &web_sys::HtmlAreaElement,
            children_ctx: &mut ::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let state = state.pin_project();
            let dom_element: &::web_sys::Element = element.as_ref();
            crate::imports::props::UpdateElement::update_element(
                this.HtmlElementProps,
                element.as_ref(),
                children_ctx,
                state.HtmlElementProps,
            );
            <TypeDefs::download as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.download,
                state.download,
                |v| element.set_download(v),
                || dom_element.remove_attribute("download").unwrap(),
            );
            <TypeDefs::href as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.href,
                state.href,
                |v| element.set_href(v),
                || dom_element.remove_attribute("href").unwrap(),
            );
            <TypeDefs::ping as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.ping,
                state.ping,
                |v| element.set_ping(v),
                || dom_element.remove_attribute("ping").unwrap(),
            );
            <TypeDefs::referrer_policy as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.referrer_policy,
                state.referrer_policy,
                |v| element.set_referrer_policy(v),
                || dom_element.remove_attribute("referrerpolicy").unwrap(),
            );
            <TypeDefs::rel as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.rel,
                state.rel,
                |v| element.set_rel(v),
                || dom_element.remove_attribute("rel").unwrap(),
            );
            <TypeDefs::target as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.target,
                state.target,
                |v| element.set_target(v),
                || dom_element.remove_attribute("target").unwrap(),
            );
            <TypeDefs::alt as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.alt,
                state.alt,
                |v| element.set_alt(v),
                || dom_element.remove_attribute("alt").unwrap(),
            );
            <TypeDefs::coords as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.coords,
                state.coords,
                |v| element.set_coords(v),
                || dom_element.remove_attribute("coords").unwrap(),
            );
            <TypeDefs::shape as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.shape,
                state.shape,
                |v| element.set_shape(v),
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
                9usize,
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
                                <TypeDefs::download as ::frender_dom::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.download)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("download"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::href as ::frender_dom::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.href)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("href"),
                                        if let Some(value) = value {
                                            ::frender_ssr::element::html::HtmlAttributeValue::String(
                                                value,
                                            )
                                        } else {
                                            ::frender_ssr::element::html::HtmlAttributeValue::BooleanTrue
                                        },
                                    )),
                                <TypeDefs::ping as ::frender_dom::props::MaybeUpdateValueWithState<
                                    str,
                                >>::maybe_into_html_attribute_value(this.ping)
                                    .map(|value| (
                                        ::std::borrow::Cow::Borrowed("ping"),
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
                                <TypeDefs::rel as ::frender_dom::props::MaybeUpdateValueWithState<
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
                                <TypeDefs::target as ::frender_dom::props::MaybeUpdateValueWithState<
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
                                <TypeDefs::alt as ::frender_dom::props::MaybeUpdateValueWithState<
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
                                <TypeDefs::coords as ::frender_dom::props::MaybeUpdateValueWithState<
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
                                <TypeDefs::shape as ::frender_dom::props::MaybeUpdateValueWithState<
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
