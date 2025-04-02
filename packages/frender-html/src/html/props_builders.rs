use super::prop_markers::conflicted_names;
use super::*;
use crate::intrinsic::{AllowAttribute, AllowAttributeName, AllowAttributeWithPinnedState, AllowChildren, Intrinsic};
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::Node as prop_markers, props::Node as props};
};
impl<C> AllowChildren<C> for super::markers::Element {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::Element where super::markers::Node: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::Element where super::markers::Node: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::Element
where
    super::markers::Node: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Node as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::Element as prop_markers, props::Element as props};
    impl AllowAttribute<prop_markers::ref_element> for super::markers::Element {}
    impl AllowAttribute<prop_markers::class> for super::markers::Element {}
    impl AllowAttribute<prop_markers::id> for super::markers::Element {}
    impl AllowAttribute<prop_markers::part> for super::markers::Element {}
    impl AllowAttributeWithPinnedState<prop_markers::on_cancel> for super::markers::Element {}
    impl AllowAttributeWithPinnedState<prop_markers::on_error> for super::markers::Element {}
    impl AllowAttributeWithPinnedState<prop_markers::on_scroll> for super::markers::Element {}
    impl AllowAttributeWithPinnedState<prop_markers::on_security_policy_violation> for super::markers::Element {}
    impl AllowAttributeWithPinnedState<prop_markers::on_select> for super::markers::Element {}
    impl AllowAttributeWithPinnedState<prop_markers::on_wheel> for super::markers::Element {}
    impl AllowAttributeWithPinnedState<prop_markers::on_copy> for super::markers::Element {}
    impl AllowAttributeWithPinnedState<prop_markers::on_cut> for super::markers::Element {}
    impl AllowAttributeWithPinnedState<prop_markers::on_paste> for super::markers::Element {}
    impl AllowAttributeWithPinnedState<prop_markers::on_composition_end> for super::markers::Element {}
    impl AllowAttributeWithPinnedState<prop_markers::on_composition_start> for super::markers::Element {}
    impl AllowAttributeWithPinnedState<prop_markers::on_composition_update> for super::markers::Element {}
    impl AllowAttributeWithPinnedState<prop_markers::on_blur> for super::markers::Element {}
    impl AllowAttributeWithPinnedState<prop_markers::on_focus> for super::markers::Element {}
    impl AllowAttributeWithPinnedState<prop_markers::on_focus_in> for super::markers::Element {}
    impl AllowAttributeWithPinnedState<prop_markers::on_focus_out> for super::markers::Element {}
    impl AllowAttributeWithPinnedState<prop_markers::on_fullscreen_change> for super::markers::Element {}
    impl AllowAttributeWithPinnedState<prop_markers::on_fullscreen_error> for super::markers::Element {}
    impl AllowAttributeWithPinnedState<prop_markers::on_key_down> for super::markers::Element {}
    impl AllowAttributeWithPinnedState<prop_markers::on_key_up> for super::markers::Element {}
    impl AllowAttributeWithPinnedState<prop_markers::on_aux_click> for super::markers::Element {}
    impl AllowAttributeWithPinnedState<prop_markers::on_click> for super::markers::Element {}
    impl AllowAttributeWithPinnedState<prop_markers::on_context_menu> for super::markers::Element {}
    impl AllowAttributeWithPinnedState<prop_markers::on_double_click> for super::markers::Element {}
    impl AllowAttributeWithPinnedState<prop_markers::on_mouse_down> for super::markers::Element {}
    impl AllowAttributeWithPinnedState<prop_markers::on_mouse_enter> for super::markers::Element {}
    impl AllowAttributeWithPinnedState<prop_markers::on_mouse_leave> for super::markers::Element {}
    impl AllowAttributeWithPinnedState<prop_markers::on_mouse_move> for super::markers::Element {}
    impl AllowAttributeWithPinnedState<prop_markers::on_mouse_out> for super::markers::Element {}
    impl AllowAttributeWithPinnedState<prop_markers::on_mouse_over> for super::markers::Element {}
    impl AllowAttributeWithPinnedState<prop_markers::on_mouse_up> for super::markers::Element {}
    impl AllowAttributeWithPinnedState<prop_markers::on_touch_cancel> for super::markers::Element {}
    impl AllowAttributeWithPinnedState<prop_markers::on_touch_end> for super::markers::Element {}
    impl AllowAttributeWithPinnedState<prop_markers::on_touch_move> for super::markers::Element {}
    impl AllowAttributeWithPinnedState<prop_markers::on_touch_start> for super::markers::Element {}
    impl<M: AllowAttribute<prop_markers::children>, C, A, P> Intrinsic<M, C, A, P> {}
    impl<M: AllowAttribute<prop_markers::ref_element>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn ref_element<V: FnOnce(&frender_dom::node_ref::Element)>(self, value: V) -> Intrinsic<M, C, (A, props::ref_element<V>), P> {
            Self::with_attribute_appended(self, props::ref_element(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::class>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn class<V: DomTokens::Bounds>(self, value: V) -> Intrinsic<M, C, (A, props::class<V>), P> {
            Self::with_attribute_appended(self, props::class(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::id>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn id<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::id<V>), P> {
            Self::with_attribute_appended(self, props::id(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::part>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn part<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::part<V>), P> {
            Self::with_attribute_appended(self, props::part(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_cancel>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`cancel`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/cancel_event)
        ///
        /// Fires on a [`<dialog>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dialog) when the user instructs the browser that they wish to dismiss the currently open modal dialog. The browser fires this event when the user presses the <kbd>Esc</kbd> key to close the modal dialog.
        pub fn on_cancel<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_cancel<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_cancel(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_error>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`error`](https://developer.mozilla.org/en-US/docs/Web/API/Element/error_event)
        ///
        /// Fired when a resource failed to load, or can't be used. For example, if a script has an execution error or an image can't be found or is invalid.
        pub fn on_error<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_error<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_error(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_scroll>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`scroll`](https://developer.mozilla.org/en-US/docs/Web/API/Element/scroll_event)
        ///
        /// Fired when the document view or an element has been scrolled.
        pub fn on_scroll<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_scroll<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_scroll(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_security_policy_violation>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`securitypolicyviolation`](https://developer.mozilla.org/en-US/docs/Web/API/Element/securitypolicyviolation_event)
        ///
        /// Fired when a [Content Security Policy](https://developer.mozilla.org/en-US/docs/Web/HTTP/CSP) is violated.
        pub fn on_security_policy_violation<V: frender_common::MaybeHandleEvent<dyn crate::values::event::SecurityPolicyViolationEvent> + 'static>(
            self,
            value: V,
        ) -> Intrinsic<M, C, A, (P, props::on_security_policy_violation<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_security_policy_violation(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_select>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`select`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/select_event)
        ///
        /// Fired when some text has been selected.
        pub fn on_select<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_select<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_select(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_wheel>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`wheel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/wheel_event)
        ///
        /// Fired when the user rotates a wheel button on a pointing device (typically a mouse).
        pub fn on_wheel<V: frender_common::MaybeHandleEvent<dyn crate::values::event::WheelEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_wheel<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_wheel(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_copy>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`copy`](https://developer.mozilla.org/en-US/docs/Web/API/Element/copy_event)
        ///
        /// Fired when the user initiates a copy action through the browser's user interface.
        pub fn on_copy<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_copy<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_copy(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_cut>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`cut`](https://developer.mozilla.org/en-US/docs/Web/API/Element/cut_event)
        ///
        /// Fired when the user initiates a cut action through the browser's user interface.
        pub fn on_cut<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_cut<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_cut(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_paste>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`paste`](https://developer.mozilla.org/en-US/docs/Web/API/Element/paste_event)
        ///
        /// Fired when the user initiates a paste action through the browser's user interface.
        pub fn on_paste<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_paste<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_paste(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_composition_end>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`compositionend`](https://developer.mozilla.org/en-US/docs/Web/API/Element/compositionend_event)
        ///
        /// Fired when a text composition system such as an [input method editor](https://developer.mozilla.org/en-US/docs/Glossary/Input_method_editor) completes or cancels the current composition session.
        pub fn on_composition_end<V: frender_common::MaybeHandleEvent<dyn crate::values::event::CompositionEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_composition_end<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_composition_end(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_composition_start>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`compositionstart`](https://developer.mozilla.org/en-US/docs/Web/API/Element/compositionstart_event)
        ///
        /// Fired when a text composition system such as an [input method editor](https://developer.mozilla.org/en-US/docs/Glossary/Input_method_editor) starts a new composition session.
        pub fn on_composition_start<V: frender_common::MaybeHandleEvent<dyn crate::values::event::CompositionEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_composition_start<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_composition_start(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_composition_update>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`compositionupdate`](https://developer.mozilla.org/en-US/docs/Web/API/Element/compositionupdate_event)
        ///
        /// Fired when a new character is received in the context of a text composition session controlled by a text composition system such as an [input method editor](https://developer.mozilla.org/en-US/docs/Glossary/Input_method_editor).
        pub fn on_composition_update<V: frender_common::MaybeHandleEvent<dyn crate::values::event::CompositionEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_composition_update<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_composition_update(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_blur>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`blur`](https://developer.mozilla.org/en-US/docs/Web/API/Element/blur_event)
        ///
        /// Fired when an element has lost focus.
        pub fn on_blur<V: frender_common::MaybeHandleEvent<dyn crate::values::event::FocusEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_blur<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_blur(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_focus>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`focus`](https://developer.mozilla.org/en-US/docs/Web/API/Element/focus_event)
        ///
        /// Fired when an element has gained focus.
        pub fn on_focus<V: frender_common::MaybeHandleEvent<dyn crate::values::event::FocusEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_focus<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_focus(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_focus_in>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`focusin`](https://developer.mozilla.org/en-US/docs/Web/API/Element/focusin_event)
        ///
        /// Fired when an element has gained focus, after [`focus`](https://developer.mozilla.org/en-US/docs/Web/API/Element/focus_event).
        pub fn on_focus_in<V: frender_common::MaybeHandleEvent<dyn crate::values::event::FocusEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_focus_in<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_focus_in(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_focus_out>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`focusout`](https://developer.mozilla.org/en-US/docs/Web/API/Element/focusout_event)
        ///
        /// Fired when an element has lost focus, after [`blur`](https://developer.mozilla.org/en-US/docs/Web/API/Element/blur_event).
        pub fn on_focus_out<V: frender_common::MaybeHandleEvent<dyn crate::values::event::FocusEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_focus_out<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_focus_out(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_fullscreen_change>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`fullscreenchange`](https://developer.mozilla.org/en-US/docs/Web/API/Element/fullscreenchange_event)
        ///
        /// Sent to an [`Element`](https://developer.mozilla.org/en-US/docs/Web/API/Element) when it transitions into or out of [fullscreen](https://developer.mozilla.org/en-US/docs/Web/API/Fullscreen_API/Guide) mode.
        pub fn on_fullscreen_change<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_fullscreen_change<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_fullscreen_change(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_fullscreen_error>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`fullscreenerror`](https://developer.mozilla.org/en-US/docs/Web/API/Element/fullscreenerror_event)
        ///
        /// Sent to an `Element` if an error occurs while attempting to switch it into or out of [fullscreen](https://developer.mozilla.org/en-US/docs/Web/API/Fullscreen_API/Guide) mode.
        pub fn on_fullscreen_error<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_fullscreen_error<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_fullscreen_error(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_key_down>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`keydown`](https://developer.mozilla.org/en-US/docs/Web/API/Element/keydown_event)
        ///
        /// Fired when a key is pressed.
        pub fn on_key_down<V: frender_common::MaybeHandleEvent<dyn crate::values::event::KeyboardEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_key_down<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_key_down(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_key_up>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`keyup`](https://developer.mozilla.org/en-US/docs/Web/API/Element/keyup_event)
        ///
        /// Fired when a key is released.
        pub fn on_key_up<V: frender_common::MaybeHandleEvent<dyn crate::values::event::KeyboardEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_key_up<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_key_up(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_aux_click>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`auxclick`](https://developer.mozilla.org/en-US/docs/Web/API/Element/auxclick_event)
        ///
        /// Fired when a non-primary pointing device button (e.g., any mouse button other than the left button) has been pressed and released on an element.
        pub fn on_aux_click<V: frender_common::MaybeHandleEvent<dyn crate::values::event::MouseEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_aux_click<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_aux_click(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_click>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`click`](https://developer.mozilla.org/en-US/docs/Web/API/Element/click_event)
        ///
        /// Fired when a pointing device button (e.g., a mouse's primary button) is pressed and released on a single element.
        pub fn on_click<V: frender_common::MaybeHandleEvent<dyn crate::values::event::MouseEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_click<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_click(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_context_menu>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`contextmenu`](https://developer.mozilla.org/en-US/docs/Web/API/Element/contextmenu_event)
        ///
        /// Fired when the user attempts to open a context menu.
        pub fn on_context_menu<V: frender_common::MaybeHandleEvent<dyn crate::values::event::MouseEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_context_menu<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_context_menu(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_double_click>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`dblclick`](https://developer.mozilla.org/en-US/docs/Web/API/Element/dblclick_event)
        ///
        /// Fired when a pointing device button (e.g., a mouse's primary button) is clicked twice on a single element.
        pub fn on_double_click<V: frender_common::MaybeHandleEvent<dyn crate::values::event::MouseEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_double_click<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_double_click(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_mouse_down>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`mousedown`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mousedown_event)
        ///
        /// Fired when a pointing device button is pressed on an element.
        pub fn on_mouse_down<V: frender_common::MaybeHandleEvent<dyn crate::values::event::MouseEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_mouse_down<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_mouse_down(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_mouse_enter>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`mouseenter`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseenter_event)
        ///
        /// Fired when a pointing device (usually a mouse) is moved over the element that has the listener attached.
        pub fn on_mouse_enter<V: frender_common::MaybeHandleEvent<dyn crate::values::event::MouseEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_mouse_enter<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_mouse_enter(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_mouse_leave>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`mouseleave`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseleave_event)
        ///
        /// Fired when the pointer of a pointing device (usually a mouse) is moved out of an element that has the listener attached to it.
        pub fn on_mouse_leave<V: frender_common::MaybeHandleEvent<dyn crate::values::event::MouseEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_mouse_leave<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_mouse_leave(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_mouse_move>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`mousemove`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mousemove_event)
        ///
        /// Fired when a pointing device (usually a mouse) is moved while over an element.
        pub fn on_mouse_move<V: frender_common::MaybeHandleEvent<dyn crate::values::event::MouseEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_mouse_move<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_mouse_move(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_mouse_out>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`mouseout`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseout_event)
        ///
        /// Fired when a pointing device (usually a mouse) is moved off the element to which the listener is attached or off one of its children.
        pub fn on_mouse_out<V: frender_common::MaybeHandleEvent<dyn crate::values::event::MouseEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_mouse_out<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_mouse_out(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_mouse_over>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`mouseover`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseover_event)
        ///
        /// Fired when a pointing device is moved onto the element to which the listener is attached or onto one of its children.
        pub fn on_mouse_over<V: frender_common::MaybeHandleEvent<dyn crate::values::event::MouseEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_mouse_over<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_mouse_over(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_mouse_up>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`mouseup`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseup_event)
        ///
        /// Fired when a pointing device button is released on an element.
        pub fn on_mouse_up<V: frender_common::MaybeHandleEvent<dyn crate::values::event::MouseEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_mouse_up<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_mouse_up(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_touch_cancel>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`touchcancel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/touchcancel_event)
        ///
        /// Fired when one or more touch points have been disrupted in an implementation-specific manner (for example, too many touch points are created).
        pub fn on_touch_cancel<V: frender_common::MaybeHandleEvent<dyn crate::values::event::TouchEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_touch_cancel<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_touch_cancel(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_touch_end>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`touchend`](https://developer.mozilla.org/en-US/docs/Web/API/Element/touchend_event)
        ///
        /// Fired when one or more touch points are removed from the touch surface.
        pub fn on_touch_end<V: frender_common::MaybeHandleEvent<dyn crate::values::event::TouchEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_touch_end<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_touch_end(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_touch_move>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`touchmove`](https://developer.mozilla.org/en-US/docs/Web/API/Element/touchmove_event)
        ///
        /// Fired when one or more touch points are moved along the touch surface.
        pub fn on_touch_move<V: frender_common::MaybeHandleEvent<dyn crate::values::event::TouchEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_touch_move<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_touch_move(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_touch_start>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`touchstart`](https://developer.mozilla.org/en-US/docs/Web/API/Element/touchstart_event)
        ///
        /// Fired when one or more touch points are placed on the touch surface.
        pub fn on_touch_start<V: frender_common::MaybeHandleEvent<dyn crate::values::event::TouchEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_touch_start<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_touch_start(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::ElementWithHrefAttribute where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithHrefAttribute where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithHrefAttribute where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithHrefAttribute
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithHrefAttribute as prop_markers, props::ElementWithHrefAttribute as props};
    impl AllowAttribute<prop_markers::href> for super::markers::ElementWithHrefAttribute {}
    impl<M: AllowAttribute<prop_markers::href>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn href<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::href<V>), P> {
            Self::with_attribute_appended(self, props::href(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::ElementWithTargetAttribute where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithTargetAttribute where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithTargetAttribute where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithTargetAttribute
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithTargetAttribute as prop_markers, props::ElementWithTargetAttribute as props};
    impl AllowAttribute<prop_markers::target> for super::markers::ElementWithTargetAttribute {}
    impl<M: AllowAttribute<prop_markers::target>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn target<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::target<V>), P> {
            Self::with_attribute_appended(self, props::target(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::ElementWithTypeAttribute where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithTypeAttribute where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithTypeAttribute where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithTypeAttribute
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithTypeAttribute as prop_markers, props::ElementWithTypeAttribute as props};
    impl AllowAttribute<prop_markers::r#type> for super::markers::ElementWithTypeAttribute {}
    impl<M: AllowAttribute<prop_markers::r#type>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn r#type<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::r#type<V>), P> {
            Self::with_attribute_appended(self, props::r#type(value))
        }
        pub fn type_<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::r#type<V>), P> {
            Self::with_attribute_appended(self, props::r#type(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::ElementWithCiteAttribute where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithCiteAttribute where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithCiteAttribute where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithCiteAttribute
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithCiteAttribute as prop_markers, props::ElementWithCiteAttribute as props};
    impl AllowAttribute<prop_markers::cite> for super::markers::ElementWithCiteAttribute {}
    impl<M: AllowAttribute<prop_markers::cite>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn cite<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::cite<V>), P> {
            Self::with_attribute_appended(self, props::cite(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::ElementWithPlaceHolderAttribute where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithPlaceHolderAttribute where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithPlaceHolderAttribute where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithPlaceHolderAttribute
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithPlaceHolderAttribute as prop_markers, props::ElementWithPlaceHolderAttribute as props};
    impl AllowAttribute<prop_markers::placeholder> for super::markers::ElementWithPlaceHolderAttribute {}
    impl<M: AllowAttribute<prop_markers::placeholder>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn placeholder<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::placeholder<V>), P> {
            Self::with_attribute_appended(self, props::placeholder(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::ElementWithMaxMinLengthAttributes where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithMaxMinLengthAttributes where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithMaxMinLengthAttributes where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithMaxMinLengthAttributes
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithMaxMinLengthAttributes as prop_markers, props::ElementWithMaxMinLengthAttributes as props};
    impl AllowAttribute<prop_markers::max_length> for super::markers::ElementWithMaxMinLengthAttributes {}
    impl AllowAttribute<prop_markers::min_length> for super::markers::ElementWithMaxMinLengthAttributes {}
    impl<M: AllowAttribute<prop_markers::max_length>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn max_length<V: frender_attr_value::IntoAttrValue<i32>>(self, value: V) -> Intrinsic<M, C, (A, props::max_length<V>), P> {
            Self::with_attribute_appended(self, props::max_length(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::min_length>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn min_length<V: frender_attr_value::IntoAttrValue<i32>>(self, value: V) -> Intrinsic<M, C, (A, props::min_length<V>), P> {
            Self::with_attribute_appended(self, props::min_length(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::ElementWithHeightWidthStrAttributes where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithHeightWidthStrAttributes where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithHeightWidthStrAttributes where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithHeightWidthStrAttributes
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithHeightWidthStrAttributes as prop_markers, props::ElementWithHeightWidthStrAttributes as props};
    impl AllowAttributeName<conflicted_names::height> for super::markers::ElementWithHeightWidthStrAttributes {
        type AttributeMarker = prop_markers::height;
    }
    impl AllowAttribute<prop_markers::height> for super::markers::ElementWithHeightWidthStrAttributes {}
    impl AllowAttributeName<conflicted_names::width> for super::markers::ElementWithHeightWidthStrAttributes {
        type AttributeMarker = prop_markers::width;
    }
    impl AllowAttribute<prop_markers::width> for super::markers::ElementWithHeightWidthStrAttributes {}
};
impl<C> AllowChildren<C> for super::markers::ElementWithHeightWidthU32Attributes where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithHeightWidthU32Attributes where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithHeightWidthU32Attributes where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithHeightWidthU32Attributes
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithHeightWidthU32Attributes as prop_markers, props::ElementWithHeightWidthU32Attributes as props};
    impl AllowAttributeName<conflicted_names::height> for super::markers::ElementWithHeightWidthU32Attributes {
        type AttributeMarker = prop_markers::height;
    }
    impl AllowAttribute<prop_markers::height> for super::markers::ElementWithHeightWidthU32Attributes {}
    impl AllowAttributeName<conflicted_names::width> for super::markers::ElementWithHeightWidthU32Attributes {
        type AttributeMarker = prop_markers::width;
    }
    impl AllowAttribute<prop_markers::width> for super::markers::ElementWithHeightWidthU32Attributes {}
};
impl<C> AllowChildren<C> for super::markers::ElementWithMaxF64Attribute where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithMaxF64Attribute where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithMaxF64Attribute where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithMaxF64Attribute
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithMaxF64Attribute as prop_markers, props::ElementWithMaxF64Attribute as props};
    impl AllowAttributeName<conflicted_names::max> for super::markers::ElementWithMaxF64Attribute {
        type AttributeMarker = prop_markers::max;
    }
    impl AllowAttribute<prop_markers::max> for super::markers::ElementWithMaxF64Attribute {}
};
impl<C> AllowChildren<C> for super::markers::ElementWithValueF64Attribute where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithValueF64Attribute where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithValueF64Attribute where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithValueF64Attribute
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithValueF64Attribute as prop_markers, props::ElementWithValueF64Attribute as props};
    impl AllowAttributeName<conflicted_names::value> for super::markers::ElementWithValueF64Attribute {
        type AttributeMarker = prop_markers::value;
    }
    impl AllowAttribute<prop_markers::value> for super::markers::ElementWithValueF64Attribute {}
};
impl<C> AllowChildren<C> for super::markers::ElementWithValueStrAttribute where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithValueStrAttribute where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithValueStrAttribute where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithValueStrAttribute
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithValueStrAttribute as prop_markers, props::ElementWithValueStrAttribute as props};
    impl AllowAttributeName<conflicted_names::value> for super::markers::ElementWithValueStrAttribute {
        type AttributeMarker = prop_markers::value;
    }
    impl AllowAttribute<prop_markers::value> for super::markers::ElementWithValueStrAttribute {}
};
impl<C> AllowChildren<C> for super::markers::ElementWithOpenAttribute where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithOpenAttribute where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithOpenAttribute where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithOpenAttribute
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithOpenAttribute as prop_markers, props::ElementWithOpenAttribute as props};
    impl AllowAttribute<prop_markers::open> for super::markers::ElementWithOpenAttribute {}
    impl<M: AllowAttribute<prop_markers::open>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn open<V: frender_attr_value::IntoAttrValue<bool>>(self, value: V) -> Intrinsic<M, C, (A, props::open<V>), P> {
            Self::with_attribute_appended(self, props::open(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::ElementWithNameAttribute where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithNameAttribute where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithNameAttribute where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithNameAttribute
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithNameAttribute as prop_markers, props::ElementWithNameAttribute as props};
    impl AllowAttribute<prop_markers::name> for super::markers::ElementWithNameAttribute {}
    impl<M: AllowAttribute<prop_markers::name>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn name<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::name<V>), P> {
            Self::with_attribute_appended(self, props::name(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::ElementWithDisabledAttribute where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithDisabledAttribute where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithDisabledAttribute where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithDisabledAttribute
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithDisabledAttribute as prop_markers, props::ElementWithDisabledAttribute as props};
    impl AllowAttribute<prop_markers::disabled> for super::markers::ElementWithDisabledAttribute {}
    impl<M: AllowAttribute<prop_markers::disabled>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn disabled<V: frender_attr_value::IntoAttrValue<bool>>(self, value: V) -> Intrinsic<M, C, (A, props::disabled<V>), P> {
            Self::with_attribute_appended(self, props::disabled(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::ElementWithCrossOriginAttribute where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithCrossOriginAttribute where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithCrossOriginAttribute where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithCrossOriginAttribute
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithCrossOriginAttribute as prop_markers, props::ElementWithCrossOriginAttribute as props};
    impl AllowAttribute<prop_markers::cross_origin> for super::markers::ElementWithCrossOriginAttribute {}
    impl<M: AllowAttribute<prop_markers::cross_origin>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn cross_origin<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::cross_origin<V>), P> {
            Self::with_attribute_appended(self, props::cross_origin(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::ElementWithRelAttribute where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithRelAttribute where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithRelAttribute where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithRelAttribute
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithRelAttribute as prop_markers, props::ElementWithRelAttribute as props};
    impl AllowAttribute<prop_markers::rel> for super::markers::ElementWithRelAttribute {}
    impl<M: AllowAttribute<prop_markers::rel>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn rel<V: DomTokens::Bounds>(self, value: V) -> Intrinsic<M, C, (A, props::rel<V>), P> {
            Self::with_attribute_appended(self, props::rel(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::ElementWithReferrerPolicyAttribute where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithReferrerPolicyAttribute where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithReferrerPolicyAttribute where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithReferrerPolicyAttribute
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithReferrerPolicyAttribute as prop_markers, props::ElementWithReferrerPolicyAttribute as props};
    impl AllowAttribute<prop_markers::referrer_policy> for super::markers::ElementWithReferrerPolicyAttribute {}
    impl<M: AllowAttribute<prop_markers::referrer_policy>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn referrer_policy<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::referrer_policy<V>), P> {
            Self::with_attribute_appended(self, props::referrer_policy(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::ElementWithAltAttribute where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithAltAttribute where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithAltAttribute where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithAltAttribute
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithAltAttribute as prop_markers, props::ElementWithAltAttribute as props};
    impl AllowAttribute<prop_markers::alt> for super::markers::ElementWithAltAttribute {}
    impl<M: AllowAttribute<prop_markers::alt>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn alt<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::alt<V>), P> {
            Self::with_attribute_appended(self, props::alt(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::ElementWithLoadingAttribute where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithLoadingAttribute where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithLoadingAttribute where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithLoadingAttribute
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithLoadingAttribute as prop_markers, props::ElementWithLoadingAttribute as props};
    impl AllowAttribute<prop_markers::loading> for super::markers::ElementWithLoadingAttribute {}
    impl<M: AllowAttribute<prop_markers::loading>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn loading<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::loading<V>), P> {
            Self::with_attribute_appended(self, props::loading(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::ElementWithAcceptAttribute where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithAcceptAttribute where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithAcceptAttribute where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithAcceptAttribute
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithAcceptAttribute as prop_markers, props::ElementWithAcceptAttribute as props};
    impl AllowAttribute<prop_markers::accept> for super::markers::ElementWithAcceptAttribute {}
    impl<M: AllowAttribute<prop_markers::accept>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn accept<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::accept<V>), P> {
            Self::with_attribute_appended(self, props::accept(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::ElementWithAutoCompleteAttribute where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithAutoCompleteAttribute where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithAutoCompleteAttribute where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithAutoCompleteAttribute
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithAutoCompleteAttribute as prop_markers, props::ElementWithAutoCompleteAttribute as props};
    impl AllowAttribute<prop_markers::auto_complete> for super::markers::ElementWithAutoCompleteAttribute {}
    impl<M: AllowAttribute<prop_markers::auto_complete>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn auto_complete<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::auto_complete<V>), P> {
            Self::with_attribute_appended(self, props::auto_complete(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::ElementWithAutoCorrectAttribute where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithAutoCorrectAttribute where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithAutoCorrectAttribute where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithAutoCorrectAttribute
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithAutoCorrectAttribute as prop_markers, props::ElementWithAutoCorrectAttribute as props};
    impl AllowAttribute<prop_markers::auto_correct> for super::markers::ElementWithAutoCorrectAttribute {}
    impl<M: AllowAttribute<prop_markers::auto_correct>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn auto_correct<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::auto_correct<V>), P> {
            Self::with_attribute_appended(self, props::auto_correct(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::ElementWithFormAttribute where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithFormAttribute where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithFormAttribute where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithFormAttribute
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithFormAttribute as prop_markers, props::ElementWithFormAttribute as props};
    impl AllowAttribute<prop_markers::form> for super::markers::ElementWithFormAttribute {}
    impl<M: AllowAttribute<prop_markers::form>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn form<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::form<V>), P> {
            Self::with_attribute_appended(self, props::form(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::ElementWithFormAttributes where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithFormAttributes where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithFormAttributes where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithFormAttributes
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithFormAttributes as prop_markers, props::ElementWithFormAttributes as props};
    impl AllowAttribute<prop_markers::form_action> for super::markers::ElementWithFormAttributes {}
    impl AllowAttribute<prop_markers::form_enc_type> for super::markers::ElementWithFormAttributes {}
    impl AllowAttribute<prop_markers::form_method> for super::markers::ElementWithFormAttributes {}
    impl AllowAttribute<prop_markers::form_no_validate> for super::markers::ElementWithFormAttributes {}
    impl AllowAttribute<prop_markers::form_target> for super::markers::ElementWithFormAttributes {}
    impl AllowAttribute<prop_markers::form> for super::markers::ElementWithFormAttributes {}
    impl<M: AllowAttribute<prop_markers::form_action>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn form_action<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::form_action<V>), P> {
            Self::with_attribute_appended(self, props::form_action(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::form_enc_type>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn form_enc_type<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::form_enc_type<V>), P> {
            Self::with_attribute_appended(self, props::form_enc_type(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::form_method>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn form_method<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::form_method<V>), P> {
            Self::with_attribute_appended(self, props::form_method(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::form_no_validate>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn form_no_validate<V: frender_attr_value::IntoAttrValue<bool>>(self, value: V) -> Intrinsic<M, C, (A, props::form_no_validate<V>), P> {
            Self::with_attribute_appended(self, props::form_no_validate(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::form_target>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn form_target<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::form_target<V>), P> {
            Self::with_attribute_appended(self, props::form_target(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::ElementWithFetchPriorityAttribute where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithFetchPriorityAttribute where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithFetchPriorityAttribute where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithFetchPriorityAttribute
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithFetchPriorityAttribute as prop_markers, props::ElementWithFetchPriorityAttribute as props};
    impl AllowAttribute<prop_markers::fetch_priority> for super::markers::ElementWithFetchPriorityAttribute {}
    impl<M: AllowAttribute<prop_markers::fetch_priority>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn fetch_priority<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::fetch_priority<V>), P> {
            Self::with_attribute_appended(self, props::fetch_priority(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::ElementWithHrefLangAttribute where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithHrefLangAttribute where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithHrefLangAttribute where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithHrefLangAttribute
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithHrefLangAttribute as prop_markers, props::ElementWithHrefLangAttribute as props};
    impl AllowAttribute<prop_markers::href_lang> for super::markers::ElementWithHrefLangAttribute {}
    impl AllowAttribute<prop_markers::href> for super::markers::ElementWithHrefLangAttribute {}
    impl<M: AllowAttribute<prop_markers::href_lang>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn href_lang<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::href_lang<V>), P> {
            Self::with_attribute_appended(self, props::href_lang(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::ElementWithSizesAttribute where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithSizesAttribute where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithSizesAttribute where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithSizesAttribute
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithSizesAttribute as prop_markers, props::ElementWithSizesAttribute as props};
    impl AllowAttribute<prop_markers::sizes> for super::markers::ElementWithSizesAttribute {}
    impl<M: AllowAttribute<prop_markers::sizes>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn sizes<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::sizes<V>), P> {
            Self::with_attribute_appended(self, props::sizes(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::ElementWithUseMapAttribute where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithUseMapAttribute where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithUseMapAttribute where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithUseMapAttribute
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithUseMapAttribute as prop_markers, props::ElementWithUseMapAttribute as props};
    impl AllowAttribute<prop_markers::use_map> for super::markers::ElementWithUseMapAttribute {}
    impl<M: AllowAttribute<prop_markers::use_map>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn use_map<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::use_map<V>), P> {
            Self::with_attribute_appended(self, props::use_map(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::ElementWithLabelAttribute where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithLabelAttribute where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithLabelAttribute where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithLabelAttribute
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithLabelAttribute as prop_markers, props::ElementWithLabelAttribute as props};
    impl AllowAttribute<prop_markers::label> for super::markers::ElementWithLabelAttribute {}
    impl<M: AllowAttribute<prop_markers::label>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn label<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::label<V>), P> {
            Self::with_attribute_appended(self, props::label(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::ElementWithForAttribute where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithForAttribute where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithForAttribute where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithForAttribute
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithForAttribute as prop_markers, props::ElementWithForAttribute as props};
    impl AllowAttribute<prop_markers::r#for> for super::markers::ElementWithForAttribute {}
    impl<M: AllowAttribute<prop_markers::r#for>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn r#for<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::r#for<V>), P> {
            Self::with_attribute_appended(self, props::r#for(value))
        }
        pub fn html_for<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::r#for<V>), P> {
            Self::with_attribute_appended(self, props::r#for(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::ElementWithIntegrityAttribute where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithIntegrityAttribute where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithIntegrityAttribute where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithIntegrityAttribute
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithIntegrityAttribute as prop_markers, props::ElementWithIntegrityAttribute as props};
    impl AllowAttribute<prop_markers::integrity> for super::markers::ElementWithIntegrityAttribute {}
    impl<M: AllowAttribute<prop_markers::integrity>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn integrity<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::integrity<V>), P> {
            Self::with_attribute_appended(self, props::integrity(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::ElementWithBlockingAttribute where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithBlockingAttribute where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithBlockingAttribute where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithBlockingAttribute
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithBlockingAttribute as prop_markers, props::ElementWithBlockingAttribute as props};
    impl AllowAttribute<prop_markers::blocking> for super::markers::ElementWithBlockingAttribute {}
    impl<M: AllowAttribute<prop_markers::blocking>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn blocking<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::blocking<V>), P> {
            Self::with_attribute_appended(self, props::blocking(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::ElementWithMultipleAttribute where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithMultipleAttribute where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithMultipleAttribute where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithMultipleAttribute
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithMultipleAttribute as prop_markers, props::ElementWithMultipleAttribute as props};
    impl AllowAttribute<prop_markers::multiple> for super::markers::ElementWithMultipleAttribute {}
    impl<M: AllowAttribute<prop_markers::multiple>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn multiple<V: frender_attr_value::IntoAttrValue<bool>>(self, value: V) -> Intrinsic<M, C, (A, props::multiple<V>), P> {
            Self::with_attribute_appended(self, props::multiple(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::ElementWithRequiredAttribute where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithRequiredAttribute where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithRequiredAttribute where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithRequiredAttribute
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithRequiredAttribute as prop_markers, props::ElementWithRequiredAttribute as props};
    impl AllowAttribute<prop_markers::required> for super::markers::ElementWithRequiredAttribute {}
    impl<M: AllowAttribute<prop_markers::required>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn required<V: frender_attr_value::IntoAttrValue<bool>>(self, value: V) -> Intrinsic<M, C, (A, props::required<V>), P> {
            Self::with_attribute_appended(self, props::required(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::ElementWithSizeU32Attribute where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithSizeU32Attribute where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithSizeU32Attribute where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithSizeU32Attribute
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithSizeU32Attribute as prop_markers, props::ElementWithSizeU32Attribute as props};
    impl AllowAttribute<prop_markers::size> for super::markers::ElementWithSizeU32Attribute {}
    impl<M: AllowAttribute<prop_markers::size>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn size<V: frender_attr_value::IntoAttrValue<u32>>(self, value: V) -> Intrinsic<M, C, (A, props::size<V>), P> {
            Self::with_attribute_appended(self, props::size(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::ElementWithSrcAttribute where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithSrcAttribute where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithSrcAttribute where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithSrcAttribute
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithSrcAttribute as prop_markers, props::ElementWithSrcAttribute as props};
    impl AllowAttribute<prop_markers::src> for super::markers::ElementWithSrcAttribute {}
    impl<M: AllowAttribute<prop_markers::src>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn src<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::src<V>), P> {
            Self::with_attribute_appended(self, props::src(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::ElementWithSrcsetAttribute where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithSrcsetAttribute where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithSrcsetAttribute where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithSrcsetAttribute
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithSrcsetAttribute as prop_markers, props::ElementWithSrcsetAttribute as props};
    impl AllowAttribute<prop_markers::srcset> for super::markers::ElementWithSrcsetAttribute {}
    impl AllowAttribute<prop_markers::src> for super::markers::ElementWithSrcsetAttribute {}
    impl<M: AllowAttribute<prop_markers::srcset>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn srcset<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::srcset<V>), P> {
            Self::with_attribute_appended(self, props::srcset(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::ElementWithBgColorAttribute where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithBgColorAttribute where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithBgColorAttribute where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithBgColorAttribute
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithBgColorAttribute as prop_markers, props::ElementWithBgColorAttribute as props};
    impl AllowAttribute<prop_markers::bg_color> for super::markers::ElementWithBgColorAttribute {}
    impl<M: AllowAttribute<prop_markers::bg_color>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn bg_color<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::bg_color<V>), P> {
            Self::with_attribute_appended(self, props::bg_color(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::ElementWithAlignAttribute where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithAlignAttribute where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithAlignAttribute where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithAlignAttribute
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithAlignAttribute as prop_markers, props::ElementWithAlignAttribute as props};
    impl AllowAttribute<prop_markers::align> for super::markers::ElementWithAlignAttribute {}
    impl<M: AllowAttribute<prop_markers::align>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn align<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::align<V>), P> {
            Self::with_attribute_appended(self, props::align(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::ElementWithMediaAttribute where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithMediaAttribute where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithMediaAttribute where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithMediaAttribute
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithMediaAttribute as prop_markers, props::ElementWithMediaAttribute as props};
    impl AllowAttribute<prop_markers::media> for super::markers::ElementWithMediaAttribute {}
    impl<M: AllowAttribute<prop_markers::media>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn media<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::media<V>), P> {
            Self::with_attribute_appended(self, props::media(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::ElementWithReadOnlyAttribute where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithReadOnlyAttribute where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithReadOnlyAttribute where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithReadOnlyAttribute
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithReadOnlyAttribute as prop_markers, props::ElementWithReadOnlyAttribute as props};
    impl AllowAttribute<prop_markers::read_only> for super::markers::ElementWithReadOnlyAttribute {}
    impl<M: AllowAttribute<prop_markers::read_only>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn read_only<V: frender_attr_value::IntoAttrValue<bool>>(self, value: V) -> Intrinsic<M, C, (A, props::read_only<V>), P> {
            Self::with_attribute_appended(self, props::read_only(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::ElementWithDateTimeAttribute where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::ElementWithDateTimeAttribute where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::ElementWithDateTimeAttribute where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::ElementWithDateTimeAttribute
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::ElementWithDateTimeAttribute as prop_markers, props::ElementWithDateTimeAttribute as props};
    impl AllowAttribute<prop_markers::date_time> for super::markers::ElementWithDateTimeAttribute {}
    impl<M: AllowAttribute<prop_markers::date_time>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn date_time<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::date_time<V>), P> {
            Self::with_attribute_appended(self, props::date_time(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::HtmlElement where super::markers::Element: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlElement where super::markers::Element: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlElement where super::markers::Element: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlElement
where
    super::markers::Element: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::Element as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlElement as prop_markers, props::HtmlElement as props};
    impl AllowAttribute<prop_markers::ref_html_element> for super::markers::HtmlElement {}
    impl AllowAttribute<prop_markers::access_key> for super::markers::HtmlElement {}
    impl AllowAttribute<prop_markers::auto_capitalize> for super::markers::HtmlElement {}
    impl AllowAttribute<prop_markers::auto_focus> for super::markers::HtmlElement {}
    impl AllowAttribute<prop_markers::content_editable> for super::markers::HtmlElement {}
    impl AllowAttribute<prop_markers::context_menu> for super::markers::HtmlElement {}
    impl AllowAttribute<prop_markers::dir> for super::markers::HtmlElement {}
    impl AllowAttribute<prop_markers::draggable> for super::markers::HtmlElement {}
    impl AllowAttribute<prop_markers::enter_key_hint> for super::markers::HtmlElement {}
    impl AllowAttribute<prop_markers::hidden> for super::markers::HtmlElement {}
    impl AllowAttribute<prop_markers::inert> for super::markers::HtmlElement {}
    impl AllowAttribute<prop_markers::input_mode> for super::markers::HtmlElement {}
    impl AllowAttribute<prop_markers::is> for super::markers::HtmlElement {}
    impl AllowAttribute<prop_markers::item_id> for super::markers::HtmlElement {}
    impl AllowAttribute<prop_markers::item_prop> for super::markers::HtmlElement {}
    impl AllowAttribute<prop_markers::item_ref> for super::markers::HtmlElement {}
    impl AllowAttribute<prop_markers::item_scope> for super::markers::HtmlElement {}
    impl AllowAttribute<prop_markers::item_type> for super::markers::HtmlElement {}
    impl AllowAttribute<prop_markers::lang> for super::markers::HtmlElement {}
    impl AllowAttribute<prop_markers::nonce> for super::markers::HtmlElement {}
    impl AllowAttribute<prop_markers::role> for super::markers::HtmlElement {}
    impl AllowAttribute<prop_markers::slot> for super::markers::HtmlElement {}
    impl AllowAttribute<prop_markers::spellcheck> for super::markers::HtmlElement {}
    impl AllowAttribute<prop_markers::style> for super::markers::HtmlElement {}
    impl AllowAttribute<prop_markers::tab_index> for super::markers::HtmlElement {}
    impl AllowAttribute<prop_markers::title> for super::markers::HtmlElement {}
    impl AllowAttribute<prop_markers::translate> for super::markers::HtmlElement {}
    impl AllowAttribute<prop_markers::virtual_keyboard_policy> for super::markers::HtmlElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_invalid> for super::markers::HtmlElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_animation_cancel> for super::markers::HtmlElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_animation_end> for super::markers::HtmlElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_animation_iteration> for super::markers::HtmlElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_animation_start> for super::markers::HtmlElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_before_input> for super::markers::HtmlElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_input> for super::markers::HtmlElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_change> for super::markers::HtmlElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_got_pointer_capture> for super::markers::HtmlElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_lost_pointer_capture> for super::markers::HtmlElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_pointer_cancel> for super::markers::HtmlElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_pointer_down> for super::markers::HtmlElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_pointer_enter> for super::markers::HtmlElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_pointer_leave> for super::markers::HtmlElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_pointer_move> for super::markers::HtmlElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_pointer_out> for super::markers::HtmlElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_pointer_over> for super::markers::HtmlElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_pointer_up> for super::markers::HtmlElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_transition_cancel> for super::markers::HtmlElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_transition_end> for super::markers::HtmlElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_transition_run> for super::markers::HtmlElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_transition_start> for super::markers::HtmlElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_drag> for super::markers::HtmlElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_drag_end> for super::markers::HtmlElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_drag_enter> for super::markers::HtmlElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_drag_leave> for super::markers::HtmlElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_drag_over> for super::markers::HtmlElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_drag_start> for super::markers::HtmlElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_drop> for super::markers::HtmlElement {}
    impl<M: AllowAttribute<prop_markers::ref_html_element>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn ref_html_element<V: FnOnce(&frender_dom::node_ref::HtmlElement)>(self, value: V) -> Intrinsic<M, C, (A, props::ref_html_element<V>), P> {
            Self::with_attribute_appended(self, props::ref_html_element(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::access_key>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn access_key<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::access_key<V>), P> {
            Self::with_attribute_appended(self, props::access_key(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::auto_capitalize>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn auto_capitalize<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::auto_capitalize<V>), P> {
            Self::with_attribute_appended(self, props::auto_capitalize(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::auto_focus>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn auto_focus<V: frender_attr_value::IntoAttrValue<bool>>(self, value: V) -> Intrinsic<M, C, (A, props::auto_focus<V>), P> {
            Self::with_attribute_appended(self, props::auto_focus(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::content_editable>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn content_editable<V: frender_attr_value::IntoAttrValue<AttrKindOfContentEditable>>(self, value: V) -> Intrinsic<M, C, (A, props::content_editable<V>), P> {
            Self::with_attribute_appended(self, props::content_editable(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::context_menu>, C, A, P> Intrinsic<M, C, A, P> {
        #[deprecated = "See https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/contextMenu"]
        pub fn context_menu<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::context_menu<V>), P> {
            Self::with_attribute_appended(self, props::context_menu(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::dir>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn dir<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::dir<V>), P> {
            Self::with_attribute_appended(self, props::dir(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::draggable>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn draggable<V: frender_attr_value::IntoAttrValue<bool>>(self, value: V) -> Intrinsic<M, C, (A, props::draggable<V>), P> {
            Self::with_attribute_appended(self, props::draggable(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::enter_key_hint>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn enter_key_hint<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::enter_key_hint<V>), P> {
            Self::with_attribute_appended(self, props::enter_key_hint(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::hidden>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn hidden<V: frender_attr_value::IntoAttrValue<bool>>(self, value: V) -> Intrinsic<M, C, (A, props::hidden<V>), P> {
            Self::with_attribute_appended(self, props::hidden(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::inert>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn inert<V: frender_attr_value::IntoAttrValue<bool>>(self, value: V) -> Intrinsic<M, C, (A, props::inert<V>), P> {
            Self::with_attribute_appended(self, props::inert(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::input_mode>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn input_mode<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::input_mode<V>), P> {
            Self::with_attribute_appended(self, props::input_mode(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::is>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn is<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::is<V>), P> {
            Self::with_attribute_appended(self, props::is(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::item_id>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn item_id<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::item_id<V>), P> {
            Self::with_attribute_appended(self, props::item_id(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::item_prop>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn item_prop<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::item_prop<V>), P> {
            Self::with_attribute_appended(self, props::item_prop(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::item_ref>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn item_ref<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::item_ref<V>), P> {
            Self::with_attribute_appended(self, props::item_ref(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::item_scope>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn item_scope<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::item_scope<V>), P> {
            Self::with_attribute_appended(self, props::item_scope(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::item_type>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn item_type<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::item_type<V>), P> {
            Self::with_attribute_appended(self, props::item_type(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::lang>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn lang<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::lang<V>), P> {
            Self::with_attribute_appended(self, props::lang(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::nonce>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn nonce<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::nonce<V>), P> {
            Self::with_attribute_appended(self, props::nonce(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::role>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn role<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::role<V>), P> {
            Self::with_attribute_appended(self, props::role(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::slot>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn slot<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::slot<V>), P> {
            Self::with_attribute_appended(self, props::slot(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::spellcheck>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn spellcheck<V: frender_attr_value::IntoAttrValue<AttrKindOfSpellcheck>>(self, value: V) -> Intrinsic<M, C, (A, props::spellcheck<V>), P> {
            Self::with_attribute_appended(self, props::spellcheck(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::style>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn style<V: Style::Bounds>(self, value: V) -> Intrinsic<M, C, (A, props::style<V>), P> {
            Self::with_attribute_appended(self, props::style(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::tab_index>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn tab_index<V: frender_attr_value::IntoAttrValue<i32>>(self, value: V) -> Intrinsic<M, C, (A, props::tab_index<V>), P> {
            Self::with_attribute_appended(self, props::tab_index(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::title>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn title<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::title<V>), P> {
            Self::with_attribute_appended(self, props::title(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::translate>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn translate<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::translate<V>), P> {
            Self::with_attribute_appended(self, props::translate(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::virtual_keyboard_policy>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn virtual_keyboard_policy<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::virtual_keyboard_policy<V>), P> {
            Self::with_attribute_appended(self, props::virtual_keyboard_policy(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_invalid>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`invalid`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/invalid_event)
        ///
        /// Fired when an element does not satisfy its constraints during constraint validation.
        pub fn on_invalid<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_invalid<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_invalid(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_animation_cancel>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`animationcancel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/animationcancel_event)
        ///
        /// Fired when an animation unexpectedly aborts.
        pub fn on_animation_cancel<V: frender_common::MaybeHandleEvent<dyn crate::values::event::AnimationEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_animation_cancel<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_animation_cancel(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_animation_end>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`animationend`](https://developer.mozilla.org/en-US/docs/Web/API/Element/animationend_event)
        ///
        /// Fired when an animation has completed normally.
        pub fn on_animation_end<V: frender_common::MaybeHandleEvent<dyn crate::values::event::AnimationEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_animation_end<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_animation_end(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_animation_iteration>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`animationiteration`](https://developer.mozilla.org/en-US/docs/Web/API/Element/animationiteration_event)
        ///
        /// Fired when an animation iteration has completed.
        pub fn on_animation_iteration<V: frender_common::MaybeHandleEvent<dyn crate::values::event::AnimationEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_animation_iteration<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_animation_iteration(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_animation_start>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`animationstart`](https://developer.mozilla.org/en-US/docs/Web/API/Element/animationstart_event)
        ///
        /// Fired when an animation starts.
        pub fn on_animation_start<V: frender_common::MaybeHandleEvent<dyn crate::values::event::AnimationEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_animation_start<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_animation_start(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_before_input>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`beforeinput`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/beforeinput_event)
        ///
        /// Fired when the value of an [`<input>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input), [`<select>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select), or [`<textarea>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea) element is about to be modified.
        pub fn on_before_input<V: frender_common::MaybeHandleEvent<dyn crate::values::event::InputEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_before_input<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_before_input(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_input>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`input`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/input_event)
        ///
        /// Fired when the `value` of an [`<input>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input), [`<select>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select), or [`<textarea>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea) element has been changed.
        pub fn on_input<V: frender_common::MaybeHandleEvent<dyn crate::values::event::InputEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_input<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_input(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_change>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`change`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/change_event)
        ///
        /// Fired when the `value` of an [`<input>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input), [`<select>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select), or [`<textarea>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea) element has been changed and committed by the user. Unlike the [`input`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/input_event) event, the `change` event is not necessarily fired for each alteration to an element's `value`.
        pub fn on_change<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_change<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_change(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_got_pointer_capture>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`gotpointercapture`](https://developer.mozilla.org/en-US/docs/Web/API/Element/gotpointercapture_event)
        ///
        /// Fired when an element captures a pointer using [`setPointerCapture()`](https://developer.mozilla.org/en-US/docs/Web/API/Element/setPointerCapture).
        pub fn on_got_pointer_capture<V: frender_common::MaybeHandleEvent<dyn crate::values::event::PointerEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_got_pointer_capture<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_got_pointer_capture(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_lost_pointer_capture>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`lostpointercapture`](https://developer.mozilla.org/en-US/docs/Web/API/Element/lostpointercapture_event)
        ///
        /// Fired when a [captured pointer](https://developer.mozilla.org/en-US/docs/Web/API/Pointer_events#pointer_capture) is released.
        pub fn on_lost_pointer_capture<V: frender_common::MaybeHandleEvent<dyn crate::values::event::PointerEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_lost_pointer_capture<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_lost_pointer_capture(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_pointer_cancel>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`pointercancel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointercancel_event)
        ///
        /// Fired when a pointer event is canceled.
        pub fn on_pointer_cancel<V: frender_common::MaybeHandleEvent<dyn crate::values::event::PointerEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_pointer_cancel<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_pointer_cancel(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_pointer_down>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`pointerdown`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerdown_event)
        ///
        /// Fired when a pointer becomes active.
        pub fn on_pointer_down<V: frender_common::MaybeHandleEvent<dyn crate::values::event::PointerEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_pointer_down<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_pointer_down(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_pointer_enter>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`pointerenter`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerenter_event)
        ///
        /// Fired when a pointer is moved into the hit test boundaries of an element or one of its descendants.
        pub fn on_pointer_enter<V: frender_common::MaybeHandleEvent<dyn crate::values::event::PointerEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_pointer_enter<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_pointer_enter(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_pointer_leave>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`pointerleave`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerleave_event)
        ///
        /// Fired when a pointer is moved out of the hit test boundaries of an element.
        pub fn on_pointer_leave<V: frender_common::MaybeHandleEvent<dyn crate::values::event::PointerEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_pointer_leave<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_pointer_leave(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_pointer_move>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`pointermove`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointermove_event)
        ///
        /// Fired when a pointer changes coordinates.
        pub fn on_pointer_move<V: frender_common::MaybeHandleEvent<dyn crate::values::event::PointerEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_pointer_move<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_pointer_move(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_pointer_out>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`pointerout`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerout_event)
        ///
        /// Fired when a pointer is moved out of the *hit test* boundaries of an element (among other reasons).
        pub fn on_pointer_out<V: frender_common::MaybeHandleEvent<dyn crate::values::event::PointerEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_pointer_out<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_pointer_out(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_pointer_over>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`pointerover`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerover_event)
        ///
        /// Fired when a pointer is moved into an element's hit test boundaries.
        pub fn on_pointer_over<V: frender_common::MaybeHandleEvent<dyn crate::values::event::PointerEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_pointer_over<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_pointer_over(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_pointer_up>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`pointerup`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerup_event)
        ///
        /// Fired when a pointer is no longer active.
        pub fn on_pointer_up<V: frender_common::MaybeHandleEvent<dyn crate::values::event::PointerEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_pointer_up<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_pointer_up(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_transition_cancel>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`transitioncancel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/transitioncancel_event)
        ///
        /// Fired when a [CSS transition](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Transitions/Using_CSS_transitions) is canceled.
        pub fn on_transition_cancel<V: frender_common::MaybeHandleEvent<dyn crate::values::event::TransitionEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_transition_cancel<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_transition_cancel(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_transition_end>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`transitionend`](https://developer.mozilla.org/en-US/docs/Web/API/Element/transitionend_event)
        ///
        /// Fired when a [CSS transition](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Transitions/Using_CSS_transitions) has completed.
        pub fn on_transition_end<V: frender_common::MaybeHandleEvent<dyn crate::values::event::TransitionEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_transition_end<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_transition_end(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_transition_run>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`transitionrun`](https://developer.mozilla.org/en-US/docs/Web/API/Element/transitionrun_event)
        ///
        /// Fired when a [CSS transition](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Transitions/Using_CSS_transitions) is first created.
        pub fn on_transition_run<V: frender_common::MaybeHandleEvent<dyn crate::values::event::TransitionEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_transition_run<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_transition_run(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_transition_start>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`transitionstart`](https://developer.mozilla.org/en-US/docs/Web/API/Element/transitionstart_event)
        ///
        /// Fired when a [CSS transition](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Transitions/Using_CSS_transitions) has actually started.
        pub fn on_transition_start<V: frender_common::MaybeHandleEvent<dyn crate::values::event::TransitionEvent> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_transition_start<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_transition_start(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_drag>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`drag`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/drag_event)
        ///
        /// This event is fired when an element or text selection is being dragged.
        pub fn on_drag<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_drag<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_drag(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_drag_end>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`dragend`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragend_event)
        ///
        /// This event is fired when a drag operation is being ended (by releasing a mouse button or hitting the escape key).
        pub fn on_drag_end<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_drag_end<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_drag_end(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_drag_enter>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`dragenter`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragenter_event)
        ///
        /// This event is fired when a dragged element or text selection enters a valid drop target.
        pub fn on_drag_enter<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_drag_enter<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_drag_enter(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_drag_leave>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`dragleave`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragleave_event)
        ///
        /// This event is fired when a dragged element or text selection leaves a valid drop target.
        pub fn on_drag_leave<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_drag_leave<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_drag_leave(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_drag_over>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`dragover`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragover_event)
        ///
        /// This event is fired continuously when an element or text selection is being dragged and the mouse pointer is over a valid drop target (every 50 ms WHEN mouse is not moving ELSE much faster between 5 ms (slow movement) and 1ms (fast movement) approximately. This firing pattern is different than [`mouseover`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseover_event) ).
        pub fn on_drag_over<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_drag_over<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_drag_over(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_drag_start>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`dragstart`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragstart_event)
        ///
        /// This event is fired when the user starts dragging an element or text selection.
        pub fn on_drag_start<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_drag_start<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_drag_start(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_drop>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`drop`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/drop_event)
        ///
        /// This event is fired when an element or text selection is dropped on a valid drop target.
        pub fn on_drop<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_drop<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_drop(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::HtmlDataListElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlDataListElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlDataListElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlDataListElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlDataListElement as prop_markers, props::HtmlDataListElement as props};
};
impl<C> AllowChildren<C> for super::markers::HtmlDivElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlDivElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlDivElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlDivElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlDivElement as prop_markers, props::HtmlDivElement as props};
};
impl<C> AllowChildren<C> for super::markers::HtmlDListElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlDListElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlDListElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlDListElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlDListElement as prop_markers, props::HtmlDListElement as props};
};
impl<C> AllowChildren<C> for super::markers::HtmlHeadingElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlHeadingElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlHeadingElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlHeadingElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlHeadingElement as prop_markers, props::HtmlHeadingElement as props};
};
impl<C> AllowChildren<C> for super::markers::HtmlHeadElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlHeadElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlHeadElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlHeadElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlHeadElement as prop_markers, props::HtmlHeadElement as props};
};
impl<C> AllowChildren<C> for super::markers::HtmlHrElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlHrElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlHrElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlHrElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlHrElement as prop_markers, props::HtmlHrElement as props};
};
impl<C> AllowChildren<C> for super::markers::HtmlLegendElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlLegendElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlLegendElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlLegendElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlLegendElement as prop_markers, props::HtmlLegendElement as props};
};
impl<C> AllowChildren<C> for super::markers::HtmlMenuElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlMenuElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlMenuElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlMenuElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlMenuElement as prop_markers, props::HtmlMenuElement as props};
};
impl<C> AllowChildren<C> for super::markers::HtmlParagraphElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlParagraphElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlParagraphElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlParagraphElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlParagraphElement as prop_markers, props::HtmlParagraphElement as props};
};
impl<C> AllowChildren<C> for super::markers::HtmlPictureElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlPictureElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlPictureElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlPictureElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlPictureElement as prop_markers, props::HtmlPictureElement as props};
};
impl<C> AllowChildren<C> for super::markers::HtmlPreElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlPreElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlPreElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlPreElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlPreElement as prop_markers, props::HtmlPreElement as props};
};
impl<C> AllowChildren<C> for super::markers::HtmlSpanElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlSpanElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlSpanElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlSpanElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlSpanElement as prop_markers, props::HtmlSpanElement as props};
};
impl<C> AllowChildren<C> for super::markers::HtmlTemplateElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlTemplateElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlTemplateElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlTemplateElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlTemplateElement as prop_markers, props::HtmlTemplateElement as props};
};
impl<C> AllowChildren<C> for super::markers::HtmlTitleElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlTitleElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlTitleElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlTitleElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlTitleElement as prop_markers, props::HtmlTitleElement as props};
};
impl<C> AllowChildren<C> for super::markers::HtmlElementWithHref where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlElementWithHref where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlElementWithHref where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlElementWithHref
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlElementWithHref as prop_markers, props::HtmlElementWithHref as props};
    impl AllowAttribute<prop_markers::download> for super::markers::HtmlElementWithHref {}
    impl AllowAttribute<prop_markers::ping> for super::markers::HtmlElementWithHref {}
    impl AllowAttribute<prop_markers::href> for super::markers::HtmlElementWithHref {}
    impl AllowAttribute<prop_markers::target> for super::markers::HtmlElementWithHref {}
    impl AllowAttribute<prop_markers::referrer_policy> for super::markers::HtmlElementWithHref {}
    impl AllowAttribute<prop_markers::rel> for super::markers::HtmlElementWithHref {}
    impl<M: AllowAttribute<prop_markers::download>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn download<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::download<V>), P> {
            Self::with_attribute_appended(self, props::download(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::ping>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn ping<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::ping<V>), P> {
            Self::with_attribute_appended(self, props::ping(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::HtmlAnchorElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlAnchorElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlAnchorElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlAnchorElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlAnchorElement as prop_markers, props::HtmlAnchorElement as props};
    impl AllowAttribute<prop_markers::download> for super::markers::HtmlAnchorElement {}
    impl AllowAttribute<prop_markers::ping> for super::markers::HtmlAnchorElement {}
    impl AllowAttribute<prop_markers::r#type> for super::markers::HtmlAnchorElement {}
    impl AllowAttribute<prop_markers::href_lang> for super::markers::HtmlAnchorElement {}
    impl AllowAttribute<prop_markers::href> for super::markers::HtmlAnchorElement {}
    impl AllowAttribute<prop_markers::target> for super::markers::HtmlAnchorElement {}
    impl AllowAttribute<prop_markers::referrer_policy> for super::markers::HtmlAnchorElement {}
    impl AllowAttribute<prop_markers::rel> for super::markers::HtmlAnchorElement {}
};
impl<C> AllowChildren<C> for super::markers::HtmlAreaElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlAreaElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlAreaElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlAreaElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlAreaElement as prop_markers, props::HtmlAreaElement as props};
    impl AllowAttribute<prop_markers::coords> for super::markers::HtmlAreaElement {}
    impl AllowAttribute<prop_markers::shape> for super::markers::HtmlAreaElement {}
    impl AllowAttribute<prop_markers::download> for super::markers::HtmlAreaElement {}
    impl AllowAttribute<prop_markers::ping> for super::markers::HtmlAreaElement {}
    impl AllowAttribute<prop_markers::alt> for super::markers::HtmlAreaElement {}
    impl AllowAttribute<prop_markers::href> for super::markers::HtmlAreaElement {}
    impl AllowAttribute<prop_markers::target> for super::markers::HtmlAreaElement {}
    impl AllowAttribute<prop_markers::referrer_policy> for super::markers::HtmlAreaElement {}
    impl AllowAttribute<prop_markers::rel> for super::markers::HtmlAreaElement {}
    impl<M: AllowAttribute<prop_markers::coords>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn coords<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::coords<V>), P> {
            Self::with_attribute_appended(self, props::coords(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::shape>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn shape<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::shape<V>), P> {
            Self::with_attribute_appended(self, props::shape(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::HtmlMediaElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlMediaElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlMediaElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlMediaElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlMediaElement as prop_markers, props::HtmlMediaElement as props};
    impl AllowAttribute<prop_markers::auto_play> for super::markers::HtmlMediaElement {}
    impl AllowAttribute<prop_markers::controls> for super::markers::HtmlMediaElement {}
    impl AllowAttribute<prop_markers::r#loop> for super::markers::HtmlMediaElement {}
    impl AllowAttribute<prop_markers::muted> for super::markers::HtmlMediaElement {}
    impl AllowAttribute<prop_markers::preload> for super::markers::HtmlMediaElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_abort> for super::markers::HtmlMediaElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_can_play> for super::markers::HtmlMediaElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_can_play_through> for super::markers::HtmlMediaElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_duration_change> for super::markers::HtmlMediaElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_emptied> for super::markers::HtmlMediaElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_ended> for super::markers::HtmlMediaElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_loaded_data> for super::markers::HtmlMediaElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_loaded_metadata> for super::markers::HtmlMediaElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_load_start> for super::markers::HtmlMediaElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_pause> for super::markers::HtmlMediaElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_play> for super::markers::HtmlMediaElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_playing> for super::markers::HtmlMediaElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_progress> for super::markers::HtmlMediaElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_rate_change> for super::markers::HtmlMediaElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_resize> for super::markers::HtmlMediaElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_seeked> for super::markers::HtmlMediaElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_seeking> for super::markers::HtmlMediaElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_stalled> for super::markers::HtmlMediaElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_suspend> for super::markers::HtmlMediaElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_time_update> for super::markers::HtmlMediaElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_volume_change> for super::markers::HtmlMediaElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_waiting> for super::markers::HtmlMediaElement {}
    impl AllowAttribute<prop_markers::src> for super::markers::HtmlMediaElement {}
    impl AllowAttribute<prop_markers::cross_origin> for super::markers::HtmlMediaElement {}
    impl<M: AllowAttribute<prop_markers::auto_play>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn auto_play<V: frender_attr_value::IntoAttrValue<bool>>(self, value: V) -> Intrinsic<M, C, (A, props::auto_play<V>), P> {
            Self::with_attribute_appended(self, props::auto_play(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::controls>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn controls<V: frender_attr_value::IntoAttrValue<bool>>(self, value: V) -> Intrinsic<M, C, (A, props::controls<V>), P> {
            Self::with_attribute_appended(self, props::controls(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::r#loop>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn r#loop<V: frender_attr_value::IntoAttrValue<bool>>(self, value: V) -> Intrinsic<M, C, (A, props::r#loop<V>), P> {
            Self::with_attribute_appended(self, props::r#loop(value))
        }
        pub fn loop_<V: frender_attr_value::IntoAttrValue<bool>>(self, value: V) -> Intrinsic<M, C, (A, props::r#loop<V>), P> {
            Self::with_attribute_appended(self, props::r#loop(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::muted>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn muted<V: frender_attr_value::IntoAttrValue<bool>>(self, value: V) -> Intrinsic<M, C, (A, props::muted<V>), P> {
            Self::with_attribute_appended(self, props::muted(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::preload>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn preload<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::preload<V>), P> {
            Self::with_attribute_appended(self, props::preload(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_abort>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`abort`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/abort_event)
        ///
        /// Fired when the resource was not fully loaded, but not as the result of an error.
        pub fn on_abort<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_abort<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_abort(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_can_play>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`canplay`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/canplay_event)
        ///
        /// Fired when the user agent can play the media, but estimates that **not** enough data has been loaded to play the media up to its end without having to stop for further buffering of content.
        pub fn on_can_play<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_can_play<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_can_play(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_can_play_through>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`canplaythrough`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/canplaythrough_event)
        ///
        /// Fired when the user agent can play the media, and estimates that enough data has been loaded to play the media up to its end without having to stop for further buffering of content.
        pub fn on_can_play_through<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_can_play_through<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_can_play_through(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_duration_change>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`durationchange`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/durationchange_event)
        ///
        /// Fired when the duration property has been updated.
        pub fn on_duration_change<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_duration_change<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_duration_change(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_emptied>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`emptied`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/emptied_event)
        ///
        /// Fired when the media has become empty; for example, when the media has already been loaded (or partially loaded), and the [`HTMLMediaElement.load()`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/load) method is called to reload it.
        pub fn on_emptied<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_emptied<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_emptied(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_ended>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`ended`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/ended_event)
        ///
        /// Fired when playback stops when end of the media (<audio> or <video>) is reached or because no further data is available.
        pub fn on_ended<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_ended<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_ended(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_loaded_data>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`loadeddata`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/loadeddata_event)
        ///
        /// Fired when the first frame of the media has finished loading.
        pub fn on_loaded_data<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_loaded_data<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_loaded_data(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_loaded_metadata>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`loadedmetadata`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/loadedmetadata_event)
        ///
        /// Fired when the metadata has been loaded.
        pub fn on_loaded_metadata<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_loaded_metadata<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_loaded_metadata(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_load_start>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`loadstart`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/loadstart_event)
        ///
        /// Fired when the browser has started to load a resource.
        pub fn on_load_start<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_load_start<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_load_start(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_pause>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`pause`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/pause_event)
        ///
        /// Fired when a request to pause play is handled and the activity has entered its paused state, most commonly occurring when the media's [`HTMLMediaElement.pause()`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/pause) method is called.
        pub fn on_pause<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_pause<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_pause(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_play>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`play`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/play_event)
        ///
        /// Fired when the `paused` property is changed from `true` to `false`, as a result of the [`HTMLMediaElement.play()`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/play) method, or the `autoplay` attribute.
        pub fn on_play<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_play<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_play(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_playing>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`playing`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/playing_event)
        ///
        /// Fired when playback is ready to start after having been paused or delayed due to lack of data.
        pub fn on_playing<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_playing<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_playing(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_progress>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`progress`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/progress_event)
        ///
        /// Fired periodically as the browser loads a resource.
        pub fn on_progress<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_progress<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_progress(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_rate_change>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`ratechange`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/ratechange_event)
        ///
        /// Fired when the playback rate has changed.
        pub fn on_rate_change<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_rate_change<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_rate_change(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_resize>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`resize`]()
        ///
        /// Fired when one or both of the `videoWidth` and `videoHeight` properties have just been updated.
        pub fn on_resize<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_resize<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_resize(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_seeked>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`seeked`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/seeked_event)
        ///
        /// Fired when a seek operation completes.
        pub fn on_seeked<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_seeked<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_seeked(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_seeking>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`seeking`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/seeking_event)
        ///
        /// Fired when a seek operation begins.
        pub fn on_seeking<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_seeking<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_seeking(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_stalled>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`stalled`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/stalled_event)
        ///
        /// Fired when the user agent is trying to fetch media data, but data is unexpectedly not forthcoming.
        pub fn on_stalled<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_stalled<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_stalled(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_suspend>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`suspend`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/suspend_event)
        ///
        /// Fired when the media data loading has been suspended.
        pub fn on_suspend<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_suspend<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_suspend(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_time_update>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`timeupdate`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/timeupdate_event)
        ///
        /// Fired when the time indicated by the [`currentTime`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/currentTime) property has been updated.
        pub fn on_time_update<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_time_update<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_time_update(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_volume_change>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`volumechange`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/volumechange_event)
        ///
        /// Fired when the volume has changed.
        pub fn on_volume_change<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_volume_change<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_volume_change(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_waiting>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`waiting`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/waiting_event)
        ///
        /// Fired when playback has stopped because of a temporary lack of data.
        pub fn on_waiting<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_waiting<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_waiting(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::HtmlBaseElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlBaseElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlBaseElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlBaseElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlBaseElement as prop_markers, props::HtmlBaseElement as props};
    impl AllowAttribute<prop_markers::href> for super::markers::HtmlBaseElement {}
    impl AllowAttribute<prop_markers::target> for super::markers::HtmlBaseElement {}
};
impl<C> AllowChildren<C> for super::markers::HtmlQuoteElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlQuoteElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlQuoteElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlQuoteElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlQuoteElement as prop_markers, props::HtmlQuoteElement as props};
    impl AllowAttribute<prop_markers::cite> for super::markers::HtmlQuoteElement {}
};
impl<C> AllowChildren<C> for super::markers::HtmlBodyElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlBodyElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlBodyElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlBodyElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlBodyElement as prop_markers, props::HtmlBodyElement as props};
    impl AllowAttribute<prop_markers::alink> for super::markers::HtmlBodyElement {}
    impl<M: AllowAttribute<prop_markers::alink>, C, A, P> Intrinsic<M, C, A, P> {
        #[deprecated = "Use the CSS color property in conjunction with the :active pseudo-class instead."]
        pub fn alink<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::alink<V>), P> {
            Self::with_attribute_appended(self, props::alink(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::HtmlBrElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlBrElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlBrElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlBrElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlBrElement as prop_markers, props::HtmlBrElement as props};
    impl AllowAttribute<prop_markers::clear> for super::markers::HtmlBrElement {}
    impl<M: AllowAttribute<prop_markers::clear>, C, A, P> Intrinsic<M, C, A, P> {
        #[deprecated]
        pub fn clear<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::clear<V>), P> {
            Self::with_attribute_appended(self, props::clear(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::HtmlButtonElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlButtonElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlButtonElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlButtonElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlButtonElement as prop_markers, props::HtmlButtonElement as props};
    impl AllowAttribute<prop_markers::r#type> for super::markers::HtmlButtonElement {}
    impl AllowAttribute<prop_markers::form_action> for super::markers::HtmlButtonElement {}
    impl AllowAttribute<prop_markers::form_enc_type> for super::markers::HtmlButtonElement {}
    impl AllowAttribute<prop_markers::form_method> for super::markers::HtmlButtonElement {}
    impl AllowAttribute<prop_markers::form_no_validate> for super::markers::HtmlButtonElement {}
    impl AllowAttribute<prop_markers::form_target> for super::markers::HtmlButtonElement {}
    impl AllowAttribute<prop_markers::disabled> for super::markers::HtmlButtonElement {}
    impl AllowAttribute<prop_markers::name> for super::markers::HtmlButtonElement {}
    impl AllowAttributeName<conflicted_names::value> for super::markers::HtmlButtonElement {
        type AttributeMarker = prop_markers::value;
    }
    impl AllowAttribute<prop_markers::value> for super::markers::HtmlButtonElement {}
    impl AllowAttribute<prop_markers::form> for super::markers::HtmlButtonElement {}
};
impl<C> AllowChildren<C> for super::markers::HtmlCanvasElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlCanvasElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlCanvasElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlCanvasElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlCanvasElement as prop_markers, props::HtmlCanvasElement as props};
    impl AllowAttributeName<conflicted_names::height> for super::markers::HtmlCanvasElement {
        type AttributeMarker = prop_markers::height;
    }
    impl AllowAttribute<prop_markers::height> for super::markers::HtmlCanvasElement {}
    impl AllowAttributeName<conflicted_names::width> for super::markers::HtmlCanvasElement {
        type AttributeMarker = prop_markers::width;
    }
    impl AllowAttribute<prop_markers::width> for super::markers::HtmlCanvasElement {}
};
impl<C> AllowChildren<C> for super::markers::HtmlTableCaptionElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlTableCaptionElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlTableCaptionElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlTableCaptionElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlTableCaptionElement as prop_markers, props::HtmlTableCaptionElement as props};
    impl AllowAttribute<prop_markers::align> for super::markers::HtmlTableCaptionElement {}
};
impl<C> AllowChildren<C> for super::markers::HtmlDataElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlDataElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlDataElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlDataElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlDataElement as prop_markers, props::HtmlDataElement as props};
    impl AllowAttributeName<conflicted_names::value> for super::markers::HtmlDataElement {
        type AttributeMarker = prop_markers::value;
    }
    impl AllowAttribute<prop_markers::value> for super::markers::HtmlDataElement {}
};
impl<C> AllowChildren<C> for super::markers::HtmlModElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlModElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlModElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlModElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlModElement as prop_markers, props::HtmlModElement as props};
    impl AllowAttribute<prop_markers::cite> for super::markers::HtmlModElement {}
    impl AllowAttribute<prop_markers::date_time> for super::markers::HtmlModElement {}
};
impl<C> AllowChildren<C> for super::markers::HtmlDetailsElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlDetailsElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlDetailsElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlDetailsElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlDetailsElement as prop_markers, props::HtmlDetailsElement as props};
    impl AllowAttribute<prop_markers::open> for super::markers::HtmlDetailsElement {}
};
impl<C> AllowChildren<C> for super::markers::HtmlDialogElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlDialogElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlDialogElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlDialogElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlDialogElement as prop_markers, props::HtmlDialogElement as props};
    impl AllowAttribute<prop_markers::open> for super::markers::HtmlDialogElement {}
};
impl<C> AllowChildren<C> for super::markers::HtmlEmbedElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlEmbedElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlEmbedElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlEmbedElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlEmbedElement as prop_markers, props::HtmlEmbedElement as props};
    impl AllowAttribute<prop_markers::r#type> for super::markers::HtmlEmbedElement {}
    impl AllowAttribute<prop_markers::src> for super::markers::HtmlEmbedElement {}
    impl AllowAttributeName<conflicted_names::height> for super::markers::HtmlEmbedElement {
        type AttributeMarker = prop_markers::height;
    }
    impl AllowAttribute<prop_markers::height> for super::markers::HtmlEmbedElement {}
    impl AllowAttributeName<conflicted_names::width> for super::markers::HtmlEmbedElement {
        type AttributeMarker = prop_markers::width;
    }
    impl AllowAttribute<prop_markers::width> for super::markers::HtmlEmbedElement {}
};
impl<C> AllowChildren<C> for super::markers::HtmlFieldSetElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlFieldSetElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlFieldSetElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlFieldSetElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlFieldSetElement as prop_markers, props::HtmlFieldSetElement as props};
    impl AllowAttribute<prop_markers::form> for super::markers::HtmlFieldSetElement {}
    impl AllowAttribute<prop_markers::disabled> for super::markers::HtmlFieldSetElement {}
    impl AllowAttribute<prop_markers::name> for super::markers::HtmlFieldSetElement {}
};
impl<C> AllowChildren<C> for super::markers::HtmlFormElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlFormElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlFormElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlFormElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlFormElement as prop_markers, props::HtmlFormElement as props};
    impl AllowAttribute<prop_markers::accept_charset> for super::markers::HtmlFormElement {}
    impl AllowAttribute<prop_markers::action> for super::markers::HtmlFormElement {}
    impl AllowAttribute<prop_markers::enc_type> for super::markers::HtmlFormElement {}
    impl AllowAttribute<prop_markers::method> for super::markers::HtmlFormElement {}
    impl AllowAttribute<prop_markers::no_validate> for super::markers::HtmlFormElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_form_data> for super::markers::HtmlFormElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_reset> for super::markers::HtmlFormElement {}
    impl AllowAttributeWithPinnedState<prop_markers::on_submit> for super::markers::HtmlFormElement {}
    impl AllowAttribute<prop_markers::target> for super::markers::HtmlFormElement {}
    impl AllowAttribute<prop_markers::auto_complete> for super::markers::HtmlFormElement {}
    impl AllowAttribute<prop_markers::accept> for super::markers::HtmlFormElement {}
    impl AllowAttribute<prop_markers::rel> for super::markers::HtmlFormElement {}
    impl AllowAttribute<prop_markers::name> for super::markers::HtmlFormElement {}
    impl<M: AllowAttribute<prop_markers::accept_charset>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn accept_charset<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::accept_charset<V>), P> {
            Self::with_attribute_appended(self, props::accept_charset(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::action>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn action<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::action<V>), P> {
            Self::with_attribute_appended(self, props::action(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::enc_type>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn enc_type<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::enc_type<V>), P> {
            Self::with_attribute_appended(self, props::enc_type(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::method>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn method<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::method<V>), P> {
            Self::with_attribute_appended(self, props::method(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::no_validate>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn no_validate<V: frender_attr_value::IntoAttrValue<bool>>(self, value: V) -> Intrinsic<M, C, (A, props::no_validate<V>), P> {
            Self::with_attribute_appended(self, props::no_validate(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_form_data>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`formdata`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/formdata_event)
        ///
        /// The `formdata` event fires after the entry list representing the form's data is constructed.
        pub fn on_form_data<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_form_data<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_form_data(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_reset>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`reset`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/reset_event)
        ///
        /// The `reset` event fires when a form is reset.
        pub fn on_reset<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_reset<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_reset(value))
        }
    }
    impl<M: AllowAttributeWithPinnedState<prop_markers::on_submit>, C, A, P> Intrinsic<M, C, A, P> {
        /// Event [`submit`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/submit_event)
        ///
        /// The `submit` event fires when a form is submitted.
        pub fn on_submit<V: frender_common::MaybeHandleEvent<dyn crate::values::event::Event> + 'static>(self, value: V) -> Intrinsic<M, C, A, (P, props::on_submit<V>)> {
            Self::with_attribute_with_pinned_state_appended(self, props::on_submit(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::HtmlHtmlElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlHtmlElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlHtmlElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlHtmlElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlHtmlElement as prop_markers, props::HtmlHtmlElement as props};
    impl AllowAttribute<prop_markers::xmlns> for super::markers::HtmlHtmlElement {}
    impl<M: AllowAttribute<prop_markers::xmlns>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn xmlns<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::xmlns<V>), P> {
            Self::with_attribute_appended(self, props::xmlns(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::HtmlIFrameElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlIFrameElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlIFrameElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlIFrameElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlIFrameElement as prop_markers, props::HtmlIFrameElement as props};
    impl AllowAttribute<prop_markers::allow> for super::markers::HtmlIFrameElement {}
    impl AllowAttribute<prop_markers::allow_fullscreen> for super::markers::HtmlIFrameElement {}
    impl AllowAttribute<prop_markers::allow_payment_request> for super::markers::HtmlIFrameElement {}
    impl AllowAttribute<prop_markers::csp> for super::markers::HtmlIFrameElement {}
    impl AllowAttribute<prop_markers::sandbox> for super::markers::HtmlIFrameElement {}
    impl AllowAttribute<prop_markers::src_doc> for super::markers::HtmlIFrameElement {}
    impl AllowAttribute<prop_markers::src> for super::markers::HtmlIFrameElement {}
    impl AllowAttribute<prop_markers::fetch_priority> for super::markers::HtmlIFrameElement {}
    impl AllowAttribute<prop_markers::loading> for super::markers::HtmlIFrameElement {}
    impl AllowAttribute<prop_markers::referrer_policy> for super::markers::HtmlIFrameElement {}
    impl AllowAttribute<prop_markers::name> for super::markers::HtmlIFrameElement {}
    impl AllowAttributeName<conflicted_names::height> for super::markers::HtmlIFrameElement {
        type AttributeMarker = prop_markers::height;
    }
    impl AllowAttribute<prop_markers::height> for super::markers::HtmlIFrameElement {}
    impl AllowAttributeName<conflicted_names::width> for super::markers::HtmlIFrameElement {
        type AttributeMarker = prop_markers::width;
    }
    impl AllowAttribute<prop_markers::width> for super::markers::HtmlIFrameElement {}
    impl<M: AllowAttribute<prop_markers::allow>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn allow<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::allow<V>), P> {
            Self::with_attribute_appended(self, props::allow(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::allow_fullscreen>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn allow_fullscreen<V: frender_attr_value::IntoAttrValue<bool>>(self, value: V) -> Intrinsic<M, C, (A, props::allow_fullscreen<V>), P> {
            Self::with_attribute_appended(self, props::allow_fullscreen(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::allow_payment_request>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn allow_payment_request<V: frender_attr_value::IntoAttrValue<bool>>(self, value: V) -> Intrinsic<M, C, (A, props::allow_payment_request<V>), P> {
            Self::with_attribute_appended(self, props::allow_payment_request(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::csp>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn csp<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::csp<V>), P> {
            Self::with_attribute_appended(self, props::csp(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::sandbox>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn sandbox<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::sandbox<V>), P> {
            Self::with_attribute_appended(self, props::sandbox(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::src_doc>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn src_doc<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::src_doc<V>), P> {
            Self::with_attribute_appended(self, props::src_doc(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::HtmlImageElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlImageElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlImageElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlImageElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlImageElement as prop_markers, props::HtmlImageElement as props};
    impl AllowAttribute<prop_markers::decoding> for super::markers::HtmlImageElement {}
    impl AllowAttribute<prop_markers::element_timing> for super::markers::HtmlImageElement {}
    impl AllowAttribute<prop_markers::is_map> for super::markers::HtmlImageElement {}
    impl AllowAttribute<prop_markers::srcset> for super::markers::HtmlImageElement {}
    impl AllowAttribute<prop_markers::use_map> for super::markers::HtmlImageElement {}
    impl AllowAttribute<prop_markers::sizes> for super::markers::HtmlImageElement {}
    impl AllowAttribute<prop_markers::loading> for super::markers::HtmlImageElement {}
    impl AllowAttribute<prop_markers::alt> for super::markers::HtmlImageElement {}
    impl AllowAttribute<prop_markers::referrer_policy> for super::markers::HtmlImageElement {}
    impl AllowAttribute<prop_markers::cross_origin> for super::markers::HtmlImageElement {}
    impl AllowAttributeName<conflicted_names::height> for super::markers::HtmlImageElement {
        type AttributeMarker = prop_markers::height;
    }
    impl AllowAttribute<prop_markers::height> for super::markers::HtmlImageElement {}
    impl AllowAttributeName<conflicted_names::width> for super::markers::HtmlImageElement {
        type AttributeMarker = prop_markers::width;
    }
    impl AllowAttribute<prop_markers::width> for super::markers::HtmlImageElement {}
    impl AllowAttribute<prop_markers::src> for super::markers::HtmlImageElement {}
    impl<M: AllowAttribute<prop_markers::decoding>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn decoding<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::decoding<V>), P> {
            Self::with_attribute_appended(self, props::decoding(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::element_timing>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn element_timing<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::element_timing<V>), P> {
            Self::with_attribute_appended(self, props::element_timing(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::is_map>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn is_map<V: frender_attr_value::IntoAttrValue<bool>>(self, value: V) -> Intrinsic<M, C, (A, props::is_map<V>), P> {
            Self::with_attribute_appended(self, props::is_map(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::HtmlInputElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlInputElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlInputElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlInputElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlInputElement as prop_markers, props::HtmlInputElement as props};
    impl AllowAttribute<prop_markers::capture> for super::markers::HtmlInputElement {}
    impl AllowAttribute<prop_markers::dirname> for super::markers::HtmlInputElement {}
    impl AllowAttribute<prop_markers::list> for super::markers::HtmlInputElement {}
    impl AllowAttributeName<conflicted_names::max> for super::markers::HtmlInputElement {
        type AttributeMarker = prop_markers::max;
    }
    impl AllowAttribute<prop_markers::max> for super::markers::HtmlInputElement {}
    impl AllowAttributeName<conflicted_names::min> for super::markers::HtmlInputElement {
        type AttributeMarker = prop_markers::min;
    }
    impl AllowAttribute<prop_markers::min> for super::markers::HtmlInputElement {}
    impl AllowAttribute<prop_markers::pattern> for super::markers::HtmlInputElement {}
    impl AllowAttribute<prop_markers::step> for super::markers::HtmlInputElement {}
    impl AllowAttribute<prop_markers::read_only> for super::markers::HtmlInputElement {}
    impl AllowAttribute<prop_markers::placeholder> for super::markers::HtmlInputElement {}
    impl AllowAttribute<prop_markers::max_length> for super::markers::HtmlInputElement {}
    impl AllowAttribute<prop_markers::min_length> for super::markers::HtmlInputElement {}
    impl AllowAttribute<prop_markers::src> for super::markers::HtmlInputElement {}
    impl AllowAttribute<prop_markers::size> for super::markers::HtmlInputElement {}
    impl AllowAttribute<prop_markers::required> for super::markers::HtmlInputElement {}
    impl AllowAttribute<prop_markers::multiple> for super::markers::HtmlInputElement {}
    impl AllowAttribute<prop_markers::form_action> for super::markers::HtmlInputElement {}
    impl AllowAttribute<prop_markers::form_enc_type> for super::markers::HtmlInputElement {}
    impl AllowAttribute<prop_markers::form_method> for super::markers::HtmlInputElement {}
    impl AllowAttribute<prop_markers::form_no_validate> for super::markers::HtmlInputElement {}
    impl AllowAttribute<prop_markers::form_target> for super::markers::HtmlInputElement {}
    impl AllowAttribute<prop_markers::auto_complete> for super::markers::HtmlInputElement {}
    impl AllowAttribute<prop_markers::auto_correct> for super::markers::HtmlInputElement {}
    impl AllowAttribute<prop_markers::accept> for super::markers::HtmlInputElement {}
    impl AllowAttribute<prop_markers::alt> for super::markers::HtmlInputElement {}
    impl AllowAttribute<prop_markers::disabled> for super::markers::HtmlInputElement {}
    impl AllowAttribute<prop_markers::name> for super::markers::HtmlInputElement {}
    impl AllowAttributeName<conflicted_names::height> for super::markers::HtmlInputElement {
        type AttributeMarker = prop_markers::height;
    }
    impl AllowAttribute<prop_markers::height> for super::markers::HtmlInputElement {}
    impl AllowAttributeName<conflicted_names::width> for super::markers::HtmlInputElement {
        type AttributeMarker = prop_markers::width;
    }
    impl AllowAttribute<prop_markers::width> for super::markers::HtmlInputElement {}
    impl AllowAttribute<prop_markers::form> for super::markers::HtmlInputElement {}
    impl<M: AllowAttribute<prop_markers::capture>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn capture<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::capture<V>), P> {
            Self::with_attribute_appended(self, props::capture(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::dirname>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn dirname<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::dirname<V>), P> {
            Self::with_attribute_appended(self, props::dirname(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::list>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn list<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::list<V>), P> {
            Self::with_attribute_appended(self, props::list(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::pattern>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn pattern<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::pattern<V>), P> {
            Self::with_attribute_appended(self, props::pattern(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::step>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn step<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::step<V>), P> {
            Self::with_attribute_appended(self, props::step(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::HtmlLabelElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlLabelElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlLabelElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlLabelElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlLabelElement as prop_markers, props::HtmlLabelElement as props};
    impl AllowAttribute<prop_markers::r#for> for super::markers::HtmlLabelElement {}
};
impl<C> AllowChildren<C> for super::markers::HtmlLiElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlLiElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlLiElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlLiElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlLiElement as prop_markers, props::HtmlLiElement as props};
    impl AllowAttributeName<conflicted_names::value> for super::markers::HtmlLiElement {
        type AttributeMarker = prop_markers::value;
    }
    impl AllowAttribute<prop_markers::value> for super::markers::HtmlLiElement {}
};
impl<C> AllowChildren<C> for super::markers::HtmlLinkElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlLinkElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlLinkElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlLinkElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlLinkElement as prop_markers, props::HtmlLinkElement as props};
    impl AllowAttribute<prop_markers::r#as> for super::markers::HtmlLinkElement {}
    impl AllowAttribute<prop_markers::image_sizes> for super::markers::HtmlLinkElement {}
    impl AllowAttribute<prop_markers::image_src_set> for super::markers::HtmlLinkElement {}
    impl AllowAttribute<prop_markers::prefetch> for super::markers::HtmlLinkElement {}
    impl AllowAttribute<prop_markers::href> for super::markers::HtmlLinkElement {}
    impl AllowAttribute<prop_markers::r#type> for super::markers::HtmlLinkElement {}
    impl AllowAttribute<prop_markers::media> for super::markers::HtmlLinkElement {}
    impl AllowAttribute<prop_markers::blocking> for super::markers::HtmlLinkElement {}
    impl AllowAttribute<prop_markers::integrity> for super::markers::HtmlLinkElement {}
    impl AllowAttribute<prop_markers::sizes> for super::markers::HtmlLinkElement {}
    impl AllowAttribute<prop_markers::href_lang> for super::markers::HtmlLinkElement {}
    impl AllowAttribute<prop_markers::fetch_priority> for super::markers::HtmlLinkElement {}
    impl AllowAttribute<prop_markers::referrer_policy> for super::markers::HtmlLinkElement {}
    impl AllowAttribute<prop_markers::rel> for super::markers::HtmlLinkElement {}
    impl AllowAttribute<prop_markers::cross_origin> for super::markers::HtmlLinkElement {}
    impl<M: AllowAttribute<prop_markers::r#as>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn r#as<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::r#as<V>), P> {
            Self::with_attribute_appended(self, props::r#as(value))
        }
        pub fn as_<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::r#as<V>), P> {
            Self::with_attribute_appended(self, props::r#as(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::image_sizes>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn image_sizes<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::image_sizes<V>), P> {
            Self::with_attribute_appended(self, props::image_sizes(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::image_src_set>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn image_src_set<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::image_src_set<V>), P> {
            Self::with_attribute_appended(self, props::image_src_set(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::prefetch>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn prefetch<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::prefetch<V>), P> {
            Self::with_attribute_appended(self, props::prefetch(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::HtmlMapElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlMapElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlMapElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlMapElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlMapElement as prop_markers, props::HtmlMapElement as props};
    impl AllowAttribute<prop_markers::name> for super::markers::HtmlMapElement {}
};
impl<C> AllowChildren<C> for super::markers::HtmlMetaElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlMetaElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlMetaElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlMetaElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlMetaElement as prop_markers, props::HtmlMetaElement as props};
    impl AllowAttribute<prop_markers::charset> for super::markers::HtmlMetaElement {}
    impl AllowAttribute<prop_markers::content> for super::markers::HtmlMetaElement {}
    impl AllowAttribute<prop_markers::http_equiv> for super::markers::HtmlMetaElement {}
    impl AllowAttribute<prop_markers::name> for super::markers::HtmlMetaElement {}
    impl<M: AllowAttribute<prop_markers::charset>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn charset<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::charset<V>), P> {
            Self::with_attribute_appended(self, props::charset(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::content>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn content<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::content<V>), P> {
            Self::with_attribute_appended(self, props::content(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::http_equiv>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn http_equiv<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::http_equiv<V>), P> {
            Self::with_attribute_appended(self, props::http_equiv(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::HtmlMeterElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlMeterElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlMeterElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlMeterElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlMeterElement as prop_markers, props::HtmlMeterElement as props};
    impl AllowAttributeName<conflicted_names::min> for super::markers::HtmlMeterElement {
        type AttributeMarker = prop_markers::min;
    }
    impl AllowAttribute<prop_markers::min> for super::markers::HtmlMeterElement {}
    impl AllowAttribute<prop_markers::low> for super::markers::HtmlMeterElement {}
    impl AllowAttribute<prop_markers::high> for super::markers::HtmlMeterElement {}
    impl AllowAttribute<prop_markers::optimum> for super::markers::HtmlMeterElement {}
    impl AllowAttributeName<conflicted_names::max> for super::markers::HtmlMeterElement {
        type AttributeMarker = prop_markers::max;
    }
    impl AllowAttribute<prop_markers::max> for super::markers::HtmlMeterElement {}
    impl AllowAttributeName<conflicted_names::value> for super::markers::HtmlMeterElement {
        type AttributeMarker = prop_markers::value;
    }
    impl AllowAttribute<prop_markers::value> for super::markers::HtmlMeterElement {}
    impl<M: AllowAttribute<prop_markers::low>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn low<V: frender_attr_value::IntoAttrValue<f64>>(self, value: V) -> Intrinsic<M, C, (A, props::low<V>), P> {
            Self::with_attribute_appended(self, props::low(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::high>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn high<V: frender_attr_value::IntoAttrValue<f64>>(self, value: V) -> Intrinsic<M, C, (A, props::high<V>), P> {
            Self::with_attribute_appended(self, props::high(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::optimum>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn optimum<V: frender_attr_value::IntoAttrValue<f64>>(self, value: V) -> Intrinsic<M, C, (A, props::optimum<V>), P> {
            Self::with_attribute_appended(self, props::optimum(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::HtmlObjectElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlObjectElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlObjectElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlObjectElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlObjectElement as prop_markers, props::HtmlObjectElement as props};
    impl AllowAttribute<prop_markers::data> for super::markers::HtmlObjectElement {}
    impl AllowAttribute<prop_markers::r#type> for super::markers::HtmlObjectElement {}
    impl AllowAttribute<prop_markers::use_map> for super::markers::HtmlObjectElement {}
    impl AllowAttribute<prop_markers::form> for super::markers::HtmlObjectElement {}
    impl AllowAttribute<prop_markers::name> for super::markers::HtmlObjectElement {}
    impl AllowAttributeName<conflicted_names::height> for super::markers::HtmlObjectElement {
        type AttributeMarker = prop_markers::height;
    }
    impl AllowAttribute<prop_markers::height> for super::markers::HtmlObjectElement {}
    impl AllowAttributeName<conflicted_names::width> for super::markers::HtmlObjectElement {
        type AttributeMarker = prop_markers::width;
    }
    impl AllowAttribute<prop_markers::width> for super::markers::HtmlObjectElement {}
    impl<M: AllowAttribute<prop_markers::data>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn data<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::data<V>), P> {
            Self::with_attribute_appended(self, props::data(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::HtmlOListElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlOListElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlOListElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlOListElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlOListElement as prop_markers, props::HtmlOListElement as props};
    impl AllowAttribute<prop_markers::reversed> for super::markers::HtmlOListElement {}
    impl AllowAttribute<prop_markers::start> for super::markers::HtmlOListElement {}
    impl AllowAttribute<prop_markers::r#type> for super::markers::HtmlOListElement {}
    impl<M: AllowAttribute<prop_markers::reversed>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn reversed<V: frender_attr_value::IntoAttrValue<bool>>(self, value: V) -> Intrinsic<M, C, (A, props::reversed<V>), P> {
            Self::with_attribute_appended(self, props::reversed(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::start>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn start<V: frender_attr_value::IntoAttrValue<i32>>(self, value: V) -> Intrinsic<M, C, (A, props::start<V>), P> {
            Self::with_attribute_appended(self, props::start(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::HtmlOptGroupElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlOptGroupElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlOptGroupElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlOptGroupElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlOptGroupElement as prop_markers, props::HtmlOptGroupElement as props};
    impl AllowAttribute<prop_markers::label> for super::markers::HtmlOptGroupElement {}
    impl AllowAttribute<prop_markers::disabled> for super::markers::HtmlOptGroupElement {}
};
impl<C> AllowChildren<C> for super::markers::HtmlOptionElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlOptionElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlOptionElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlOptionElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlOptionElement as prop_markers, props::HtmlOptionElement as props};
    impl AllowAttribute<prop_markers::selected> for super::markers::HtmlOptionElement {}
    impl AllowAttribute<prop_markers::label> for super::markers::HtmlOptionElement {}
    impl AllowAttribute<prop_markers::disabled> for super::markers::HtmlOptionElement {}
    impl AllowAttributeName<conflicted_names::value> for super::markers::HtmlOptionElement {
        type AttributeMarker = prop_markers::value;
    }
    impl AllowAttribute<prop_markers::value> for super::markers::HtmlOptionElement {}
    impl<M: AllowAttribute<prop_markers::selected>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn selected<V: frender_attr_value::IntoAttrValue<bool>>(self, value: V) -> Intrinsic<M, C, (A, props::selected<V>), P> {
            Self::with_attribute_appended(self, props::selected(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::HtmlOutputElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlOutputElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlOutputElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlOutputElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlOutputElement as prop_markers, props::HtmlOutputElement as props};
    impl AllowAttribute<prop_markers::r#for> for super::markers::HtmlOutputElement {}
    impl AllowAttribute<prop_markers::form> for super::markers::HtmlOutputElement {}
    impl AllowAttribute<prop_markers::name> for super::markers::HtmlOutputElement {}
};
impl<C> AllowChildren<C> for super::markers::HtmlProgressElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlProgressElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlProgressElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlProgressElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlProgressElement as prop_markers, props::HtmlProgressElement as props};
    impl AllowAttributeName<conflicted_names::max> for super::markers::HtmlProgressElement {
        type AttributeMarker = prop_markers::max;
    }
    impl AllowAttribute<prop_markers::max> for super::markers::HtmlProgressElement {}
    impl AllowAttributeName<conflicted_names::value> for super::markers::HtmlProgressElement {
        type AttributeMarker = prop_markers::value;
    }
    impl AllowAttribute<prop_markers::value> for super::markers::HtmlProgressElement {}
};
impl<C: frender_dom::script::ScriptContent> AllowChildren<C> for super::markers::HtmlScriptElement {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlScriptElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlScriptElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlScriptElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlScriptElement as prop_markers, props::HtmlScriptElement as props};
    impl AllowAttribute<prop_markers::r#async> for super::markers::HtmlScriptElement {}
    impl AllowAttribute<prop_markers::defer> for super::markers::HtmlScriptElement {}
    impl AllowAttribute<prop_markers::no_module> for super::markers::HtmlScriptElement {}
    impl AllowAttribute<prop_markers::r#type> for super::markers::HtmlScriptElement {}
    impl AllowAttribute<prop_markers::src> for super::markers::HtmlScriptElement {}
    impl AllowAttribute<prop_markers::blocking> for super::markers::HtmlScriptElement {}
    impl AllowAttribute<prop_markers::integrity> for super::markers::HtmlScriptElement {}
    impl AllowAttribute<prop_markers::fetch_priority> for super::markers::HtmlScriptElement {}
    impl AllowAttribute<prop_markers::referrer_policy> for super::markers::HtmlScriptElement {}
    impl AllowAttribute<prop_markers::cross_origin> for super::markers::HtmlScriptElement {}
    impl<M: AllowAttribute<prop_markers::children>, C, A, P> Intrinsic<M, C, A, P> {}
    impl<M: AllowAttribute<prop_markers::r#async>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn r#async<V: frender_attr_value::IntoAttrValue<bool>>(self, value: V) -> Intrinsic<M, C, (A, props::r#async<V>), P> {
            Self::with_attribute_appended(self, props::r#async(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::defer>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn defer<V: frender_attr_value::IntoAttrValue<bool>>(self, value: V) -> Intrinsic<M, C, (A, props::defer<V>), P> {
            Self::with_attribute_appended(self, props::defer(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::no_module>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn no_module<V: frender_attr_value::IntoAttrValue<bool>>(self, value: V) -> Intrinsic<M, C, (A, props::no_module<V>), P> {
            Self::with_attribute_appended(self, props::no_module(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::HtmlSelectElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlSelectElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlSelectElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlSelectElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlSelectElement as prop_markers, props::HtmlSelectElement as props};
    impl AllowAttribute<prop_markers::size> for super::markers::HtmlSelectElement {}
    impl AllowAttribute<prop_markers::required> for super::markers::HtmlSelectElement {}
    impl AllowAttribute<prop_markers::multiple> for super::markers::HtmlSelectElement {}
    impl AllowAttribute<prop_markers::form> for super::markers::HtmlSelectElement {}
    impl AllowAttribute<prop_markers::auto_complete> for super::markers::HtmlSelectElement {}
    impl AllowAttribute<prop_markers::disabled> for super::markers::HtmlSelectElement {}
    impl AllowAttribute<prop_markers::name> for super::markers::HtmlSelectElement {}
};
impl<C> AllowChildren<C> for super::markers::HtmlSlotElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlSlotElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlSlotElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlSlotElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlSlotElement as prop_markers, props::HtmlSlotElement as props};
    impl AllowAttribute<prop_markers::name> for super::markers::HtmlSlotElement {}
};
impl<C> AllowChildren<C> for super::markers::HtmlSourceElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlSourceElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlSourceElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlSourceElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlSourceElement as prop_markers, props::HtmlSourceElement as props};
    impl AllowAttribute<prop_markers::r#type> for super::markers::HtmlSourceElement {}
    impl AllowAttribute<prop_markers::media> for super::markers::HtmlSourceElement {}
    impl AllowAttribute<prop_markers::srcset> for super::markers::HtmlSourceElement {}
    impl AllowAttribute<prop_markers::sizes> for super::markers::HtmlSourceElement {}
    impl AllowAttributeName<conflicted_names::height> for super::markers::HtmlSourceElement {
        type AttributeMarker = prop_markers::height;
    }
    impl AllowAttribute<prop_markers::height> for super::markers::HtmlSourceElement {}
    impl AllowAttributeName<conflicted_names::width> for super::markers::HtmlSourceElement {
        type AttributeMarker = prop_markers::width;
    }
    impl AllowAttribute<prop_markers::width> for super::markers::HtmlSourceElement {}
    impl AllowAttribute<prop_markers::src> for super::markers::HtmlSourceElement {}
};
impl<C> AllowChildren<C> for super::markers::HtmlStyleElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlStyleElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlStyleElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlStyleElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlStyleElement as prop_markers, props::HtmlStyleElement as props};
    impl AllowAttribute<prop_markers::r#type> for super::markers::HtmlStyleElement {}
    impl AllowAttribute<prop_markers::media> for super::markers::HtmlStyleElement {}
    impl AllowAttribute<prop_markers::blocking> for super::markers::HtmlStyleElement {}
};
impl<C> AllowChildren<C> for super::markers::HtmlTableElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlTableElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlTableElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlTableElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlTableElement as prop_markers, props::HtmlTableElement as props};
    impl AllowAttribute<prop_markers::border> for super::markers::HtmlTableElement {}
    impl AllowAttribute<prop_markers::cell_padding> for super::markers::HtmlTableElement {}
    impl AllowAttribute<prop_markers::cell_spacing> for super::markers::HtmlTableElement {}
    impl AllowAttribute<prop_markers::frame> for super::markers::HtmlTableElement {}
    impl AllowAttribute<prop_markers::rules> for super::markers::HtmlTableElement {}
    impl AllowAttribute<prop_markers::summary> for super::markers::HtmlTableElement {}
    impl AllowAttributeName<conflicted_names::width> for super::markers::HtmlTableElement {
        type AttributeMarker = prop_markers::width;
    }
    impl AllowAttribute<prop_markers::width> for super::markers::HtmlTableElement {}
    impl AllowAttribute<prop_markers::align> for super::markers::HtmlTableElement {}
    impl AllowAttribute<prop_markers::bg_color> for super::markers::HtmlTableElement {}
    impl<M: AllowAttribute<prop_markers::border>, C, A, P> Intrinsic<M, C, A, P> {
        #[deprecated]
        pub fn border<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::border<V>), P> {
            Self::with_attribute_appended(self, props::border(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::cell_padding>, C, A, P> Intrinsic<M, C, A, P> {
        #[deprecated]
        pub fn cell_padding<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::cell_padding<V>), P> {
            Self::with_attribute_appended(self, props::cell_padding(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::cell_spacing>, C, A, P> Intrinsic<M, C, A, P> {
        #[deprecated]
        pub fn cell_spacing<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::cell_spacing<V>), P> {
            Self::with_attribute_appended(self, props::cell_spacing(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::frame>, C, A, P> Intrinsic<M, C, A, P> {
        #[deprecated]
        pub fn frame<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::frame<V>), P> {
            Self::with_attribute_appended(self, props::frame(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::rules>, C, A, P> Intrinsic<M, C, A, P> {
        #[deprecated]
        pub fn rules<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::rules<V>), P> {
            Self::with_attribute_appended(self, props::rules(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::summary>, C, A, P> Intrinsic<M, C, A, P> {
        #[deprecated]
        pub fn summary<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::summary<V>), P> {
            Self::with_attribute_appended(self, props::summary(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::HtmlTableChildElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlTableChildElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlTableChildElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlTableChildElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlTableChildElement as prop_markers, props::HtmlTableChildElement as props};
    impl AllowAttribute<prop_markers::char> for super::markers::HtmlTableChildElement {}
    impl AllowAttribute<prop_markers::char_off> for super::markers::HtmlTableChildElement {}
    impl AllowAttribute<prop_markers::v_align> for super::markers::HtmlTableChildElement {}
    impl AllowAttribute<prop_markers::align> for super::markers::HtmlTableChildElement {}
    impl AllowAttribute<prop_markers::bg_color> for super::markers::HtmlTableChildElement {}
    impl<M: AllowAttribute<prop_markers::char>, C, A, P> Intrinsic<M, C, A, P> {
        #[deprecated]
        pub fn char<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::char<V>), P> {
            Self::with_attribute_appended(self, props::char(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::char_off>, C, A, P> Intrinsic<M, C, A, P> {
        #[deprecated]
        pub fn char_off<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::char_off<V>), P> {
            Self::with_attribute_appended(self, props::char_off(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::v_align>, C, A, P> Intrinsic<M, C, A, P> {
        #[deprecated]
        pub fn v_align<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::v_align<V>), P> {
            Self::with_attribute_appended(self, props::v_align(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::HtmlTableSectionElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlTableSectionElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlTableSectionElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlTableSectionElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlTableSectionElement as prop_markers, props::HtmlTableSectionElement as props};
    impl AllowAttribute<prop_markers::char> for super::markers::HtmlTableSectionElement {}
    impl AllowAttribute<prop_markers::char_off> for super::markers::HtmlTableSectionElement {}
    impl AllowAttribute<prop_markers::v_align> for super::markers::HtmlTableSectionElement {}
    impl AllowAttribute<prop_markers::align> for super::markers::HtmlTableSectionElement {}
    impl AllowAttribute<prop_markers::bg_color> for super::markers::HtmlTableSectionElement {}
};
impl<C> AllowChildren<C> for super::markers::HtmlTableRowElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlTableRowElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlTableRowElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlTableRowElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlTableRowElement as prop_markers, props::HtmlTableRowElement as props};
    impl AllowAttribute<prop_markers::char> for super::markers::HtmlTableRowElement {}
    impl AllowAttribute<prop_markers::char_off> for super::markers::HtmlTableRowElement {}
    impl AllowAttribute<prop_markers::v_align> for super::markers::HtmlTableRowElement {}
    impl AllowAttribute<prop_markers::align> for super::markers::HtmlTableRowElement {}
    impl AllowAttribute<prop_markers::bg_color> for super::markers::HtmlTableRowElement {}
};
impl<C> AllowChildren<C> for super::markers::HtmlTableColElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlTableColElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlTableColElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlTableColElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlTableColElement as prop_markers, props::HtmlTableColElement as props};
    impl AllowAttribute<prop_markers::span> for super::markers::HtmlTableColElement {}
    impl AllowAttributeName<conflicted_names::width> for super::markers::HtmlTableColElement {
        type AttributeMarker = prop_markers::width;
    }
    impl AllowAttribute<prop_markers::width> for super::markers::HtmlTableColElement {}
    impl AllowAttribute<prop_markers::char> for super::markers::HtmlTableColElement {}
    impl AllowAttribute<prop_markers::char_off> for super::markers::HtmlTableColElement {}
    impl AllowAttribute<prop_markers::v_align> for super::markers::HtmlTableColElement {}
    impl AllowAttribute<prop_markers::align> for super::markers::HtmlTableColElement {}
    impl AllowAttribute<prop_markers::bg_color> for super::markers::HtmlTableColElement {}
    impl<M: AllowAttribute<prop_markers::span>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn span<V: frender_attr_value::IntoAttrValue<u32>>(self, value: V) -> Intrinsic<M, C, (A, props::span<V>), P> {
            Self::with_attribute_appended(self, props::span(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::HtmlTableCellElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlTableCellElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlTableCellElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlTableCellElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlTableCellElement as prop_markers, props::HtmlTableCellElement as props};
    impl AllowAttribute<prop_markers::col_span> for super::markers::HtmlTableCellElement {}
    impl AllowAttribute<prop_markers::headers> for super::markers::HtmlTableCellElement {}
    impl AllowAttribute<prop_markers::row_span> for super::markers::HtmlTableCellElement {}
    impl AllowAttribute<prop_markers::abbr> for super::markers::HtmlTableCellElement {}
    impl AllowAttribute<prop_markers::axis> for super::markers::HtmlTableCellElement {}
    impl AllowAttribute<prop_markers::scope> for super::markers::HtmlTableCellElement {}
    impl AllowAttributeName<conflicted_names::height> for super::markers::HtmlTableCellElement {
        type AttributeMarker = prop_markers::height;
    }
    impl AllowAttribute<prop_markers::height> for super::markers::HtmlTableCellElement {}
    impl AllowAttributeName<conflicted_names::width> for super::markers::HtmlTableCellElement {
        type AttributeMarker = prop_markers::width;
    }
    impl AllowAttribute<prop_markers::width> for super::markers::HtmlTableCellElement {}
    impl AllowAttribute<prop_markers::char> for super::markers::HtmlTableCellElement {}
    impl AllowAttribute<prop_markers::char_off> for super::markers::HtmlTableCellElement {}
    impl AllowAttribute<prop_markers::v_align> for super::markers::HtmlTableCellElement {}
    impl AllowAttribute<prop_markers::align> for super::markers::HtmlTableCellElement {}
    impl AllowAttribute<prop_markers::bg_color> for super::markers::HtmlTableCellElement {}
    impl<M: AllowAttribute<prop_markers::col_span>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn col_span<V: frender_attr_value::IntoAttrValue<u32>>(self, value: V) -> Intrinsic<M, C, (A, props::col_span<V>), P> {
            Self::with_attribute_appended(self, props::col_span(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::headers>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn headers<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::headers<V>), P> {
            Self::with_attribute_appended(self, props::headers(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::row_span>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn row_span<V: frender_attr_value::IntoAttrValue<u32>>(self, value: V) -> Intrinsic<M, C, (A, props::row_span<V>), P> {
            Self::with_attribute_appended(self, props::row_span(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::abbr>, C, A, P> Intrinsic<M, C, A, P> {
        #[deprecated = "Do not use this attribute as it is obsolete in the latest standard. Alternatively, you can put the abbreviated description inside the cell and place the long content in the title attribute."]
        pub fn abbr<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::abbr<V>), P> {
            Self::with_attribute_appended(self, props::abbr(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::axis>, C, A, P> Intrinsic<M, C, A, P> {
        #[deprecated]
        pub fn axis<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::axis<V>), P> {
            Self::with_attribute_appended(self, props::axis(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::scope>, C, A, P> Intrinsic<M, C, A, P> {
        #[deprecated]
        pub fn scope<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::scope<V>), P> {
            Self::with_attribute_appended(self, props::scope(value))
        }
    }
};
impl<C: TextAreaValue> AllowChildren<C> for super::markers::HtmlTextAreaElement {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlTextAreaElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlTextAreaElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlTextAreaElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlTextAreaElement as prop_markers, props::HtmlTextAreaElement as props};
    impl AllowAttribute<prop_markers::cols> for super::markers::HtmlTextAreaElement {}
    impl AllowAttribute<prop_markers::rows> for super::markers::HtmlTextAreaElement {}
    impl AllowAttribute<prop_markers::wrap> for super::markers::HtmlTextAreaElement {}
    impl AllowAttribute<prop_markers::read_only> for super::markers::HtmlTextAreaElement {}
    impl AllowAttribute<prop_markers::placeholder> for super::markers::HtmlTextAreaElement {}
    impl AllowAttribute<prop_markers::max_length> for super::markers::HtmlTextAreaElement {}
    impl AllowAttribute<prop_markers::min_length> for super::markers::HtmlTextAreaElement {}
    impl AllowAttribute<prop_markers::required> for super::markers::HtmlTextAreaElement {}
    impl AllowAttribute<prop_markers::form> for super::markers::HtmlTextAreaElement {}
    impl AllowAttribute<prop_markers::auto_complete> for super::markers::HtmlTextAreaElement {}
    impl AllowAttribute<prop_markers::auto_correct> for super::markers::HtmlTextAreaElement {}
    impl AllowAttribute<prop_markers::disabled> for super::markers::HtmlTextAreaElement {}
    impl AllowAttribute<prop_markers::name> for super::markers::HtmlTextAreaElement {}
    impl<M: AllowAttribute<prop_markers::children>, C, A, P> Intrinsic<M, C, A, P> {}
    impl<M: AllowAttribute<prop_markers::cols>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn cols<V: frender_attr_value::IntoAttrValue<u32>>(self, value: V) -> Intrinsic<M, C, (A, props::cols<V>), P> {
            Self::with_attribute_appended(self, props::cols(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::rows>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn rows<V: frender_attr_value::IntoAttrValue<u32>>(self, value: V) -> Intrinsic<M, C, (A, props::rows<V>), P> {
            Self::with_attribute_appended(self, props::rows(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::wrap>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn wrap<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::wrap<V>), P> {
            Self::with_attribute_appended(self, props::wrap(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::HtmlTimeElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlTimeElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlTimeElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlTimeElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlTimeElement as prop_markers, props::HtmlTimeElement as props};
    impl AllowAttribute<prop_markers::date_time> for super::markers::HtmlTimeElement {}
};
impl<C> AllowChildren<C> for super::markers::HtmlTrackElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlTrackElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlTrackElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlTrackElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlTrackElement as prop_markers, props::HtmlTrackElement as props};
    impl AllowAttribute<prop_markers::default> for super::markers::HtmlTrackElement {}
    impl AllowAttribute<prop_markers::kind> for super::markers::HtmlTrackElement {}
    impl AllowAttribute<prop_markers::src_lang> for super::markers::HtmlTrackElement {}
    impl AllowAttribute<prop_markers::src> for super::markers::HtmlTrackElement {}
    impl AllowAttribute<prop_markers::label> for super::markers::HtmlTrackElement {}
    impl<M: AllowAttribute<prop_markers::default>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn default<V: frender_attr_value::IntoAttrValue<bool>>(self, value: V) -> Intrinsic<M, C, (A, props::default<V>), P> {
            Self::with_attribute_appended(self, props::default(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::kind>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn kind<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::kind<V>), P> {
            Self::with_attribute_appended(self, props::kind(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::src_lang>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn src_lang<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::src_lang<V>), P> {
            Self::with_attribute_appended(self, props::src_lang(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::HtmlUListElement where super::markers::HtmlElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlUListElement where super::markers::HtmlElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlUListElement where super::markers::HtmlElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlUListElement
where
    super::markers::HtmlElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlUListElement as prop_markers, props::HtmlUListElement as props};
    impl AllowAttribute<prop_markers::compact> for super::markers::HtmlUListElement {}
    impl AllowAttribute<prop_markers::r#type> for super::markers::HtmlUListElement {}
    impl<M: AllowAttribute<prop_markers::compact>, C, A, P> Intrinsic<M, C, A, P> {
        #[deprecated = "Do not use this attribute, as it has been deprecated: use CSS instead. To give a similar effect as the compact attribute, the CSS property line-height can be used with a value of 80%."]
        pub fn compact<V: frender_attr_value::IntoAttrValue<bool>>(self, value: V) -> Intrinsic<M, C, (A, props::compact<V>), P> {
            Self::with_attribute_appended(self, props::compact(value))
        }
    }
};
impl<C> AllowChildren<C> for super::markers::HtmlAudioElement where super::markers::HtmlMediaElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlAudioElement where super::markers::HtmlMediaElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlAudioElement where super::markers::HtmlMediaElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlAudioElement
where
    super::markers::HtmlMediaElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlMediaElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlAudioElement as prop_markers, props::HtmlAudioElement as props};
};
impl<C> AllowChildren<C> for super::markers::HtmlVideoElement where super::markers::HtmlMediaElement: AllowChildren<C> {}
impl<AttrMarker> AllowAttribute<AttrMarker> for super::markers::HtmlVideoElement where super::markers::HtmlMediaElement: AllowAttribute<AttrMarker> {}
impl<AttrMarker> AllowAttributeWithPinnedState<AttrMarker> for super::markers::HtmlVideoElement where super::markers::HtmlMediaElement: AllowAttributeWithPinnedState<AttrMarker> {}
impl<AttrName> AllowAttributeName<AttrName> for super::markers::HtmlVideoElement
where
    super::markers::HtmlMediaElement: AllowAttributeName<AttrName>,
{
    type AttributeMarker = <super::markers::HtmlMediaElement as AllowAttributeName<AttrName>>::AttributeMarker;
}
const _: () = {
    #[allow(unused_imports)]
    use super::{prop_markers::HtmlVideoElement as prop_markers, props::HtmlVideoElement as props};
    impl AllowAttribute<prop_markers::plays_inline> for super::markers::HtmlVideoElement {}
    impl AllowAttribute<prop_markers::poster> for super::markers::HtmlVideoElement {}
    impl AllowAttributeName<conflicted_names::height> for super::markers::HtmlVideoElement {
        type AttributeMarker = prop_markers::height;
    }
    impl AllowAttribute<prop_markers::height> for super::markers::HtmlVideoElement {}
    impl AllowAttributeName<conflicted_names::width> for super::markers::HtmlVideoElement {
        type AttributeMarker = prop_markers::width;
    }
    impl AllowAttribute<prop_markers::width> for super::markers::HtmlVideoElement {}
    impl<M: AllowAttribute<prop_markers::plays_inline>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn plays_inline<V: frender_attr_value::IntoAttrValue<bool>>(self, value: V) -> Intrinsic<M, C, (A, props::plays_inline<V>), P> {
            Self::with_attribute_appended(self, props::plays_inline(value))
        }
    }
    impl<M: AllowAttribute<prop_markers::poster>, C, A, P> Intrinsic<M, C, A, P> {
        pub fn poster<V: frender_attr_value::IntoAttrValue<AttrKindOfStr>>(self, value: V) -> Intrinsic<M, C, (A, props::poster<V>), P> {
            Self::with_attribute_appended(self, props::poster(value))
        }
    }
};
