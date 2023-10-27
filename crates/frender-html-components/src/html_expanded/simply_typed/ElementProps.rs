def_props_type!(
    #[derive(Debug, Clone, Copy, Default)]
    ElementProps(
        children,
        css: bounds![Css::Bounds],
        class: bounds![DomTokens::Bounds],
        id: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        part: bounds![crate::imports::impl_bounds::MaybeValue::Bounds<str>],
        on_cancel:
            bounds![
                #[event(crate::imports::frender_html::html::event_type_helpers::on_cancel)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_error:
            bounds![
                #[event(crate::imports::frender_html::html::event_type_helpers::on_error)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_scroll:
            bounds![
                #[event(crate::imports::frender_html::html::event_type_helpers::on_scroll)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_security_policy_violation:
            bounds![
                #[event(crate
    ::imports::frender_html::html::event_type_helpers::on_security_policy_violation)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_select:
            bounds![
                #[event(crate::imports::frender_html::html::event_type_helpers::on_select)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_wheel:
            bounds![
                #[event(crate::imports::frender_html::html::event_type_helpers::on_wheel)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_copy:
            bounds![
                #[event(crate::imports::frender_html::html::event_type_helpers::on_copy)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_cut:
            bounds![
                #[event(crate::imports::frender_html::html::event_type_helpers::on_cut)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_paste:
            bounds![
                #[event(crate::imports::frender_html::html::event_type_helpers::on_paste)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_composition_end:
            bounds![
                #[event(
                    crate::imports::frender_html::html::event_type_helpers::on_composition_end
                )]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_composition_start:
            bounds![
                #[event(
                    crate::imports::frender_html::html::event_type_helpers::on_composition_start
                )]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_composition_update:
            bounds![
                #[event(
                    crate::imports::frender_html::html::event_type_helpers::on_composition_update
                )]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_blur:
            bounds![
                #[event(crate::imports::frender_html::html::event_type_helpers::on_blur)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_focus:
            bounds![
                #[event(crate::imports::frender_html::html::event_type_helpers::on_focus)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_focus_in:
            bounds![
                #[event(crate::imports::frender_html::html::event_type_helpers::on_focus_in)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_focus_out:
            bounds![
                #[event(crate::imports::frender_html::html::event_type_helpers::on_focus_out)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_fullscreen_change:
            bounds![
                #[event(
                    crate::imports::frender_html::html::event_type_helpers::on_fullscreen_change
                )]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_fullscreen_error:
            bounds![
                #[event(
                    crate::imports::frender_html::html::event_type_helpers::on_fullscreen_error
                )]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_key_down:
            bounds![
                #[event(crate::imports::frender_html::html::event_type_helpers::on_key_down)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_key_up:
            bounds![
                #[event(crate::imports::frender_html::html::event_type_helpers::on_key_up)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_aux_click:
            bounds![
                #[event(crate::imports::frender_html::html::event_type_helpers::on_aux_click)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_click:
            bounds![
                #[event(crate::imports::frender_html::html::event_type_helpers::on_click)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_context_menu:
            bounds![
                #[event(crate::imports::frender_html::html::event_type_helpers::on_context_menu)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_double_click:
            bounds![
                #[event(crate::imports::frender_html::html::event_type_helpers::on_double_click)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_mouse_down:
            bounds![
                #[event(crate::imports::frender_html::html::event_type_helpers::on_mouse_down)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_mouse_enter:
            bounds![
                #[event(crate::imports::frender_html::html::event_type_helpers::on_mouse_enter)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_mouse_leave:
            bounds![
                #[event(crate::imports::frender_html::html::event_type_helpers::on_mouse_leave)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_mouse_move:
            bounds![
                #[event(crate::imports::frender_html::html::event_type_helpers::on_mouse_move)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_mouse_out:
            bounds![
                #[event(crate::imports::frender_html::html::event_type_helpers::on_mouse_out)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_mouse_over:
            bounds![
                #[event(crate::imports::frender_html::html::event_type_helpers::on_mouse_over)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_mouse_up:
            bounds![
                #[event(crate::imports::frender_html::html::event_type_helpers::on_mouse_up)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_touch_cancel:
            bounds![
                #[event(crate::imports::frender_html::html::event_type_helpers::on_touch_cancel)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_touch_end:
            bounds![
                #[event(crate::imports::frender_html::html::event_type_helpers::on_touch_end)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_touch_move:
            bounds![
                #[event(crate::imports::frender_html::html::event_type_helpers::on_touch_move)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
        on_touch_start:
            bounds![
                #[event(crate::imports::frender_html::html::event_type_helpers::on_touch_start)]
                crate::imports::impl_bounds::MaybeHandleEvent::Bounds
            ],
    )
);
#[cfg(feature = "csr")]
mod imp {
    #![allow(unused_variables)]
    #[allow(unused_imports)]
    use super::super::*;
    crate::imports::impl_bounds!(super::props::css(
        bounds as Css,
        element as Element,
        attr_name = "css"
    ));
    crate::imports::impl_bounds!(super::props::class(
        bounds as DomTokens,
        element as Element,
        attr_name = "class",
        csr {
            get_mut_dom_token_list: frender_html::renderer::node_behaviors::Element::class_list,
        }
    ));
    crate::imports::impl_bounds!(super::props::id(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as Element,
        attr_name = "id",
        csr {
            update: |el: &mut ET::Element<Renderer>, renderer: &mut _, _, v: &_| el
                .set_id(renderer, v),
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::part(
        bounds as crate::imports::impl_bounds::MaybeValue<str>,
        element as Element,
        attr_name = "part",
        csr {
            update: crate::imports::impl_bounds::MaybeValue::csr::default_update,
            remove: crate::imports::impl_bounds::MaybeValue::csr::default_remove,
        },
    ));
    crate::imports::impl_bounds!(super::props::on_cancel(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_cancel)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as Element,
        attr_name = "on_cancel",
    ));
    crate::imports::impl_bounds!(super::props::on_error(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_error)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as Element,
        attr_name = "on_error",
    ));
    crate::imports::impl_bounds!(super::props::on_scroll(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_scroll)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as Element,
        attr_name = "on_scroll",
    ));
    crate::imports::impl_bounds!(super::props::on_security_policy_violation(
        #[event(
            crate::imports::frender_html::html::event_type_helpers::on_security_policy_violation
        )]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as Element,
        attr_name = "on_security_policy_violation",
    ));
    crate::imports::impl_bounds!(super::props::on_select(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_select)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as Element,
        attr_name = "on_select",
    ));
    crate::imports::impl_bounds!(super::props::on_wheel(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_wheel)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as Element,
        attr_name = "on_wheel",
    ));
    crate::imports::impl_bounds!(super::props::on_copy(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_copy)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as Element,
        attr_name = "on_copy",
    ));
    crate::imports::impl_bounds!(super::props::on_cut(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_cut)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as Element,
        attr_name = "on_cut",
    ));
    crate::imports::impl_bounds!(super::props::on_paste(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_paste)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as Element,
        attr_name = "on_paste",
    ));
    crate::imports::impl_bounds!(super::props::on_composition_end(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_composition_end)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as Element,
        attr_name = "on_composition_end",
    ));
    crate::imports::impl_bounds!(super::props::on_composition_start(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_composition_start)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as Element,
        attr_name = "on_composition_start",
    ));
    crate::imports::impl_bounds!(super::props::on_composition_update(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_composition_update)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as Element,
        attr_name = "on_composition_update",
    ));
    crate::imports::impl_bounds!(super::props::on_blur(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_blur)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as Element,
        attr_name = "on_blur",
    ));
    crate::imports::impl_bounds!(super::props::on_focus(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_focus)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as Element,
        attr_name = "on_focus",
    ));
    crate::imports::impl_bounds!(super::props::on_focus_in(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_focus_in)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as Element,
        attr_name = "on_focus_in",
    ));
    crate::imports::impl_bounds!(super::props::on_focus_out(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_focus_out)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as Element,
        attr_name = "on_focus_out",
    ));
    crate::imports::impl_bounds!(super::props::on_fullscreen_change(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_fullscreen_change)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as Element,
        attr_name = "on_fullscreen_change",
    ));
    crate::imports::impl_bounds!(super::props::on_fullscreen_error(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_fullscreen_error)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as Element,
        attr_name = "on_fullscreen_error",
    ));
    crate::imports::impl_bounds!(super::props::on_key_down(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_key_down)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as Element,
        attr_name = "on_key_down",
    ));
    crate::imports::impl_bounds!(super::props::on_key_up(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_key_up)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as Element,
        attr_name = "on_key_up",
    ));
    crate::imports::impl_bounds!(super::props::on_aux_click(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_aux_click)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as Element,
        attr_name = "on_aux_click",
    ));
    crate::imports::impl_bounds!(super::props::on_click(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_click)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as Element,
        attr_name = "on_click",
    ));
    crate::imports::impl_bounds!(super::props::on_context_menu(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_context_menu)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as Element,
        attr_name = "on_context_menu",
    ));
    crate::imports::impl_bounds!(super::props::on_double_click(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_double_click)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as Element,
        attr_name = "on_double_click",
    ));
    crate::imports::impl_bounds!(super::props::on_mouse_down(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_mouse_down)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as Element,
        attr_name = "on_mouse_down",
    ));
    crate::imports::impl_bounds!(super::props::on_mouse_enter(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_mouse_enter)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as Element,
        attr_name = "on_mouse_enter",
    ));
    crate::imports::impl_bounds!(super::props::on_mouse_leave(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_mouse_leave)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as Element,
        attr_name = "on_mouse_leave",
    ));
    crate::imports::impl_bounds!(super::props::on_mouse_move(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_mouse_move)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as Element,
        attr_name = "on_mouse_move",
    ));
    crate::imports::impl_bounds!(super::props::on_mouse_out(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_mouse_out)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as Element,
        attr_name = "on_mouse_out",
    ));
    crate::imports::impl_bounds!(super::props::on_mouse_over(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_mouse_over)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as Element,
        attr_name = "on_mouse_over",
    ));
    crate::imports::impl_bounds!(super::props::on_mouse_up(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_mouse_up)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as Element,
        attr_name = "on_mouse_up",
    ));
    crate::imports::impl_bounds!(super::props::on_touch_cancel(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_touch_cancel)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as Element,
        attr_name = "on_touch_cancel",
    ));
    crate::imports::impl_bounds!(super::props::on_touch_end(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_touch_end)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as Element,
        attr_name = "on_touch_end",
    ));
    crate::imports::impl_bounds!(super::props::on_touch_move(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_touch_move)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as Element,
        attr_name = "on_touch_move",
    ));
    crate::imports::impl_bounds!(super::props::on_touch_start(
        #[event(crate::imports::frender_html::html::event_type_helpers::on_touch_start)]
        bounds as crate::imports::impl_bounds::MaybeHandleEvent,
        element as Element,
        attr_name = "on_touch_start",
    ));
}
mod imports {
    #[allow(unused_imports)]
    use super::super::*;
    pub(super) use crate::imports::frender_html_simple::def_props_type;
}
use imports::def_props_type;
