use super::*;
pub trait Node: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes + crate::props_builder::PropsBuilderAppendEventListeners {}
impl<C, A, ELS> Node for super::props::Node<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::Node, Props: Node> Node for crate::dom::component::IntrinsicElement<Tag, Props> {}
pub trait Element: Node {
    fn ref_element<V: SetRef::Bounds<frender_dom::node_ref::Element>>(self, value: V) -> Self::AppendAttributes<super::attributes::Element::attributes::ref_element<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::ref_element(value))
    }
    fn class<V: DomTokens::Bounds>(self, value: V) -> Self::AppendAttributes<super::attributes::Element::attributes::class<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::class(value))
    }
    fn id<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::Element::attributes::id<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::id(value))
    }
    fn part<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::Element::attributes::part<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::part(value))
    }
    /// Event [`cancel`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/cancel_event)
    ///
    /// Fires on a [`<dialog>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dialog) when the user instructs the browser that they wish to dismiss the currently open modal dialog. The browser fires this event when the user presses the <kbd>Esc</kbd> key to close the modal dialog.
    fn on_cancel<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::Element::attributes::on_cancel<V>> {
        Self::append_event_listeners(self, super::attributes::Element::attributes::on_cancel(value))
    }
    /// Event [`error`](https://developer.mozilla.org/en-US/docs/Web/API/Element/error_event)
    ///
    /// Fired when a resource failed to load, or can't be used. For example, if a script has an execution error or an image can't be found or is invalid.
    fn on_error<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::Element::attributes::on_error<V>> {
        Self::append_event_listeners(self, super::attributes::Element::attributes::on_error(value))
    }
    /// Event [`scroll`](https://developer.mozilla.org/en-US/docs/Web/API/Element/scroll_event)
    ///
    /// Fired when the document view or an element has been scrolled.
    fn on_scroll<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::Element::attributes::on_scroll<V>> {
        Self::append_event_listeners(self, super::attributes::Element::attributes::on_scroll(value))
    }
    /// Event [`securitypolicyviolation`](https://developer.mozilla.org/en-US/docs/Web/API/Element/securitypolicyviolation_event)
    ///
    /// Fired when a [Content Security Policy](https://developer.mozilla.org/en-US/docs/Web/HTTP/CSP) is violated.
    fn on_security_policy_violation<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::SecurityPolicyViolationEvent> + 'static>(
        self,
        value: V,
    ) -> Self::AppendEventListeners<super::attributes::Element::attributes::on_security_policy_violation<V>> {
        Self::append_event_listeners(self, super::attributes::Element::attributes::on_security_policy_violation(value))
    }
    /// Event [`select`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/select_event)
    ///
    /// Fired when some text has been selected.
    fn on_select<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::Element::attributes::on_select<V>> {
        Self::append_event_listeners(self, super::attributes::Element::attributes::on_select(value))
    }
    /// Event [`wheel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/wheel_event)
    ///
    /// Fired when the user rotates a wheel button on a pointing device (typically a mouse).
    fn on_wheel<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::WheelEvent> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::Element::attributes::on_wheel<V>> {
        Self::append_event_listeners(self, super::attributes::Element::attributes::on_wheel(value))
    }
    /// Event [`copy`](https://developer.mozilla.org/en-US/docs/Web/API/Element/copy_event)
    ///
    /// Fired when the user initiates a copy action through the browser's user interface.
    fn on_copy<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::Element::attributes::on_copy<V>> {
        Self::append_event_listeners(self, super::attributes::Element::attributes::on_copy(value))
    }
    /// Event [`cut`](https://developer.mozilla.org/en-US/docs/Web/API/Element/cut_event)
    ///
    /// Fired when the user initiates a cut action through the browser's user interface.
    fn on_cut<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::Element::attributes::on_cut<V>> {
        Self::append_event_listeners(self, super::attributes::Element::attributes::on_cut(value))
    }
    /// Event [`paste`](https://developer.mozilla.org/en-US/docs/Web/API/Element/paste_event)
    ///
    /// Fired when the user initiates a paste action through the browser's user interface.
    fn on_paste<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::Element::attributes::on_paste<V>> {
        Self::append_event_listeners(self, super::attributes::Element::attributes::on_paste(value))
    }
    /// Event [`compositionend`](https://developer.mozilla.org/en-US/docs/Web/API/Element/compositionend_event)
    ///
    /// Fired when a text composition system such as an [input method editor](https://developer.mozilla.org/en-US/docs/Glossary/Input_method_editor) completes or cancels the current composition session.
    fn on_composition_end<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::CompositionEvent> + 'static>(
        self,
        value: V,
    ) -> Self::AppendEventListeners<super::attributes::Element::attributes::on_composition_end<V>> {
        Self::append_event_listeners(self, super::attributes::Element::attributes::on_composition_end(value))
    }
    /// Event [`compositionstart`](https://developer.mozilla.org/en-US/docs/Web/API/Element/compositionstart_event)
    ///
    /// Fired when a text composition system such as an [input method editor](https://developer.mozilla.org/en-US/docs/Glossary/Input_method_editor) starts a new composition session.
    fn on_composition_start<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::CompositionEvent> + 'static>(
        self,
        value: V,
    ) -> Self::AppendEventListeners<super::attributes::Element::attributes::on_composition_start<V>> {
        Self::append_event_listeners(self, super::attributes::Element::attributes::on_composition_start(value))
    }
    /// Event [`compositionupdate`](https://developer.mozilla.org/en-US/docs/Web/API/Element/compositionupdate_event)
    ///
    /// Fired when a new character is received in the context of a text composition session controlled by a text composition system such as an [input method editor](https://developer.mozilla.org/en-US/docs/Glossary/Input_method_editor).
    fn on_composition_update<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::CompositionEvent> + 'static>(
        self,
        value: V,
    ) -> Self::AppendEventListeners<super::attributes::Element::attributes::on_composition_update<V>> {
        Self::append_event_listeners(self, super::attributes::Element::attributes::on_composition_update(value))
    }
    /// Event [`blur`](https://developer.mozilla.org/en-US/docs/Web/API/Element/blur_event)
    ///
    /// Fired when an element has lost focus.
    fn on_blur<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::FocusEvent> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::Element::attributes::on_blur<V>> {
        Self::append_event_listeners(self, super::attributes::Element::attributes::on_blur(value))
    }
    /// Event [`focus`](https://developer.mozilla.org/en-US/docs/Web/API/Element/focus_event)
    ///
    /// Fired when an element has gained focus.
    fn on_focus<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::FocusEvent> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::Element::attributes::on_focus<V>> {
        Self::append_event_listeners(self, super::attributes::Element::attributes::on_focus(value))
    }
    /// Event [`focusin`](https://developer.mozilla.org/en-US/docs/Web/API/Element/focusin_event)
    ///
    /// Fired when an element has gained focus, after [`focus`](https://developer.mozilla.org/en-US/docs/Web/API/Element/focus_event).
    fn on_focus_in<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::FocusEvent> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::Element::attributes::on_focus_in<V>> {
        Self::append_event_listeners(self, super::attributes::Element::attributes::on_focus_in(value))
    }
    /// Event [`focusout`](https://developer.mozilla.org/en-US/docs/Web/API/Element/focusout_event)
    ///
    /// Fired when an element has lost focus, after [`blur`](https://developer.mozilla.org/en-US/docs/Web/API/Element/blur_event).
    fn on_focus_out<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::FocusEvent> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::Element::attributes::on_focus_out<V>> {
        Self::append_event_listeners(self, super::attributes::Element::attributes::on_focus_out(value))
    }
    /// Event [`fullscreenchange`](https://developer.mozilla.org/en-US/docs/Web/API/Element/fullscreenchange_event)
    ///
    /// Sent to an [`Element`](https://developer.mozilla.org/en-US/docs/Web/API/Element) when it transitions into or out of [fullscreen](https://developer.mozilla.org/en-US/docs/Web/API/Fullscreen_API/Guide) mode.
    fn on_fullscreen_change<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::Element::attributes::on_fullscreen_change<V>> {
        Self::append_event_listeners(self, super::attributes::Element::attributes::on_fullscreen_change(value))
    }
    /// Event [`fullscreenerror`](https://developer.mozilla.org/en-US/docs/Web/API/Element/fullscreenerror_event)
    ///
    /// Sent to an `Element` if an error occurs while attempting to switch it into or out of [fullscreen](https://developer.mozilla.org/en-US/docs/Web/API/Fullscreen_API/Guide) mode.
    fn on_fullscreen_error<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::Element::attributes::on_fullscreen_error<V>> {
        Self::append_event_listeners(self, super::attributes::Element::attributes::on_fullscreen_error(value))
    }
    /// Event [`keydown`](https://developer.mozilla.org/en-US/docs/Web/API/Element/keydown_event)
    ///
    /// Fired when a key is pressed.
    fn on_key_down<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::KeyboardEvent> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::Element::attributes::on_key_down<V>> {
        Self::append_event_listeners(self, super::attributes::Element::attributes::on_key_down(value))
    }
    /// Event [`keyup`](https://developer.mozilla.org/en-US/docs/Web/API/Element/keyup_event)
    ///
    /// Fired when a key is released.
    fn on_key_up<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::KeyboardEvent> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::Element::attributes::on_key_up<V>> {
        Self::append_event_listeners(self, super::attributes::Element::attributes::on_key_up(value))
    }
    /// Event [`auxclick`](https://developer.mozilla.org/en-US/docs/Web/API/Element/auxclick_event)
    ///
    /// Fired when a non-primary pointing device button (e.g., any mouse button other than the left button) has been pressed and released on an element.
    fn on_aux_click<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::MouseEvent> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::Element::attributes::on_aux_click<V>> {
        Self::append_event_listeners(self, super::attributes::Element::attributes::on_aux_click(value))
    }
    /// Event [`click`](https://developer.mozilla.org/en-US/docs/Web/API/Element/click_event)
    ///
    /// Fired when a pointing device button (e.g., a mouse's primary button) is pressed and released on a single element.
    fn on_click<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::MouseEvent> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::Element::attributes::on_click<V>> {
        Self::append_event_listeners(self, super::attributes::Element::attributes::on_click(value))
    }
    /// Event [`contextmenu`](https://developer.mozilla.org/en-US/docs/Web/API/Element/contextmenu_event)
    ///
    /// Fired when the user attempts to open a context menu.
    fn on_context_menu<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::MouseEvent> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::Element::attributes::on_context_menu<V>> {
        Self::append_event_listeners(self, super::attributes::Element::attributes::on_context_menu(value))
    }
    /// Event [`dblclick`](https://developer.mozilla.org/en-US/docs/Web/API/Element/dblclick_event)
    ///
    /// Fired when a pointing device button (e.g., a mouse's primary button) is clicked twice on a single element.
    fn on_double_click<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::MouseEvent> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::Element::attributes::on_double_click<V>> {
        Self::append_event_listeners(self, super::attributes::Element::attributes::on_double_click(value))
    }
    /// Event [`mousedown`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mousedown_event)
    ///
    /// Fired when a pointing device button is pressed on an element.
    fn on_mouse_down<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::MouseEvent> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::Element::attributes::on_mouse_down<V>> {
        Self::append_event_listeners(self, super::attributes::Element::attributes::on_mouse_down(value))
    }
    /// Event [`mouseenter`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseenter_event)
    ///
    /// Fired when a pointing device (usually a mouse) is moved over the element that has the listener attached.
    fn on_mouse_enter<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::MouseEvent> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::Element::attributes::on_mouse_enter<V>> {
        Self::append_event_listeners(self, super::attributes::Element::attributes::on_mouse_enter(value))
    }
    /// Event [`mouseleave`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseleave_event)
    ///
    /// Fired when the pointer of a pointing device (usually a mouse) is moved out of an element that has the listener attached to it.
    fn on_mouse_leave<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::MouseEvent> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::Element::attributes::on_mouse_leave<V>> {
        Self::append_event_listeners(self, super::attributes::Element::attributes::on_mouse_leave(value))
    }
    /// Event [`mousemove`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mousemove_event)
    ///
    /// Fired when a pointing device (usually a mouse) is moved while over an element.
    fn on_mouse_move<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::MouseEvent> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::Element::attributes::on_mouse_move<V>> {
        Self::append_event_listeners(self, super::attributes::Element::attributes::on_mouse_move(value))
    }
    /// Event [`mouseout`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseout_event)
    ///
    /// Fired when a pointing device (usually a mouse) is moved off the element to which the listener is attached or off one of its children.
    fn on_mouse_out<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::MouseEvent> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::Element::attributes::on_mouse_out<V>> {
        Self::append_event_listeners(self, super::attributes::Element::attributes::on_mouse_out(value))
    }
    /// Event [`mouseover`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseover_event)
    ///
    /// Fired when a pointing device is moved onto the element to which the listener is attached or onto one of its children.
    fn on_mouse_over<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::MouseEvent> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::Element::attributes::on_mouse_over<V>> {
        Self::append_event_listeners(self, super::attributes::Element::attributes::on_mouse_over(value))
    }
    /// Event [`mouseup`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseup_event)
    ///
    /// Fired when a pointing device button is released on an element.
    fn on_mouse_up<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::MouseEvent> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::Element::attributes::on_mouse_up<V>> {
        Self::append_event_listeners(self, super::attributes::Element::attributes::on_mouse_up(value))
    }
    /// Event [`touchcancel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/touchcancel_event)
    ///
    /// Fired when one or more touch points have been disrupted in an implementation-specific manner (for example, too many touch points are created).
    fn on_touch_cancel<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::TouchEvent> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::Element::attributes::on_touch_cancel<V>> {
        Self::append_event_listeners(self, super::attributes::Element::attributes::on_touch_cancel(value))
    }
    /// Event [`touchend`](https://developer.mozilla.org/en-US/docs/Web/API/Element/touchend_event)
    ///
    /// Fired when one or more touch points are removed from the touch surface.
    fn on_touch_end<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::TouchEvent> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::Element::attributes::on_touch_end<V>> {
        Self::append_event_listeners(self, super::attributes::Element::attributes::on_touch_end(value))
    }
    /// Event [`touchmove`](https://developer.mozilla.org/en-US/docs/Web/API/Element/touchmove_event)
    ///
    /// Fired when one or more touch points are moved along the touch surface.
    fn on_touch_move<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::TouchEvent> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::Element::attributes::on_touch_move<V>> {
        Self::append_event_listeners(self, super::attributes::Element::attributes::on_touch_move(value))
    }
    /// Event [`touchstart`](https://developer.mozilla.org/en-US/docs/Web/API/Element/touchstart_event)
    ///
    /// Fired when one or more touch points are placed on the touch surface.
    fn on_touch_start<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::TouchEvent> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::Element::attributes::on_touch_start<V>> {
        Self::append_event_listeners(self, super::attributes::Element::attributes::on_touch_start(value))
    }
}
impl<C, A, ELS> Element for super::props::Element<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::Element<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::Element, Props: Element> Element for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::Element<crate::Empty, A, ELS> {
    type WithChildren = super::props::Element<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::Element { props: self.props.children(children) }
    }
}
pub trait ElementWithHrefAttribute: Element {
    fn href<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithHrefAttribute::attributes::href<V>> {
        Self::append_attributes(self, super::attributes::ElementWithHrefAttribute::attributes::href(value))
    }
}
impl<C, A, ELS> ElementWithHrefAttribute for super::props::ElementWithHrefAttribute<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithHrefAttribute<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithHrefAttribute<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithHrefAttribute, Props: ElementWithHrefAttribute> ElementWithHrefAttribute for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithHrefAttribute<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithHrefAttribute<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithHrefAttribute { props: self.props.children(children) }
    }
}
pub trait ElementWithTargetAttribute: Element {
    fn target<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithTargetAttribute::attributes::target<V>> {
        Self::append_attributes(self, super::attributes::ElementWithTargetAttribute::attributes::target(value))
    }
}
impl<C, A, ELS> ElementWithTargetAttribute for super::props::ElementWithTargetAttribute<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithTargetAttribute<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithTargetAttribute<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithTargetAttribute, Props: ElementWithTargetAttribute> ElementWithTargetAttribute for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithTargetAttribute<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithTargetAttribute<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithTargetAttribute { props: self.props.children(children) }
    }
}
pub trait ElementWithTypeAttribute: Element {
    fn r#type<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithTypeAttribute::attributes::r#type<V>> {
        Self::append_attributes(self, super::attributes::ElementWithTypeAttribute::attributes::r#type(value))
    }
    fn type_<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithTypeAttribute::attributes::r#type<V>> {
        Self::append_attributes(self, super::attributes::ElementWithTypeAttribute::attributes::r#type(value))
    }
}
impl<C, A, ELS> ElementWithTypeAttribute for super::props::ElementWithTypeAttribute<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithTypeAttribute<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithTypeAttribute<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithTypeAttribute, Props: ElementWithTypeAttribute> ElementWithTypeAttribute for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithTypeAttribute<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithTypeAttribute<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithTypeAttribute { props: self.props.children(children) }
    }
}
pub trait ElementWithCiteAttribute: Element {
    fn cite<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithCiteAttribute::attributes::cite<V>> {
        Self::append_attributes(self, super::attributes::ElementWithCiteAttribute::attributes::cite(value))
    }
}
impl<C, A, ELS> ElementWithCiteAttribute for super::props::ElementWithCiteAttribute<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithCiteAttribute<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithCiteAttribute<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithCiteAttribute, Props: ElementWithCiteAttribute> ElementWithCiteAttribute for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithCiteAttribute<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithCiteAttribute<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithCiteAttribute { props: self.props.children(children) }
    }
}
pub trait ElementWithPlaceHolderAttribute: Element {
    fn placeholder<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithPlaceHolderAttribute::attributes::placeholder<V>> {
        Self::append_attributes(self, super::attributes::ElementWithPlaceHolderAttribute::attributes::placeholder(value))
    }
}
impl<C, A, ELS> ElementWithPlaceHolderAttribute for super::props::ElementWithPlaceHolderAttribute<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithPlaceHolderAttribute<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithPlaceHolderAttribute<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithPlaceHolderAttribute, Props: ElementWithPlaceHolderAttribute> ElementWithPlaceHolderAttribute for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithPlaceHolderAttribute<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithPlaceHolderAttribute<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithPlaceHolderAttribute { props: self.props.children(children) }
    }
}
pub trait ElementWithMaxMinLengthAttributes: Element {
    fn max_length<V: crate::impl_bounds::MaybeValue::Bounds<i32>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithMaxMinLengthAttributes::attributes::max_length<V>> {
        Self::append_attributes(self, super::attributes::ElementWithMaxMinLengthAttributes::attributes::max_length(value))
    }
    fn min_length<V: crate::impl_bounds::MaybeValue::Bounds<i32>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithMaxMinLengthAttributes::attributes::min_length<V>> {
        Self::append_attributes(self, super::attributes::ElementWithMaxMinLengthAttributes::attributes::min_length(value))
    }
}
impl<C, A, ELS> ElementWithMaxMinLengthAttributes for super::props::ElementWithMaxMinLengthAttributes<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithMaxMinLengthAttributes<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithMaxMinLengthAttributes<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithMaxMinLengthAttributes, Props: ElementWithMaxMinLengthAttributes> ElementWithMaxMinLengthAttributes for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithMaxMinLengthAttributes<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithMaxMinLengthAttributes<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithMaxMinLengthAttributes { props: self.props.children(children) }
    }
}
pub trait ElementWithHeightWidthStrAttributes: Element {
    fn height<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithHeightWidthStrAttributes::attributes::height<V>> {
        Self::append_attributes(self, super::attributes::ElementWithHeightWidthStrAttributes::attributes::height(value))
    }
    fn width<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithHeightWidthStrAttributes::attributes::width<V>> {
        Self::append_attributes(self, super::attributes::ElementWithHeightWidthStrAttributes::attributes::width(value))
    }
}
impl<C, A, ELS> ElementWithHeightWidthStrAttributes for super::props::ElementWithHeightWidthStrAttributes<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithHeightWidthStrAttributes<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithHeightWidthStrAttributes<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithHeightWidthStrAttributes, Props: ElementWithHeightWidthStrAttributes> ElementWithHeightWidthStrAttributes for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithHeightWidthStrAttributes<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithHeightWidthStrAttributes<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithHeightWidthStrAttributes { props: self.props.children(children) }
    }
}
pub trait ElementWithHeightWidthU32Attributes: Element {
    fn height<V: crate::impl_bounds::MaybeValue::Bounds<u32>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithHeightWidthU32Attributes::attributes::height<V>> {
        Self::append_attributes(self, super::attributes::ElementWithHeightWidthU32Attributes::attributes::height(value))
    }
    fn width<V: crate::impl_bounds::MaybeValue::Bounds<u32>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithHeightWidthU32Attributes::attributes::width<V>> {
        Self::append_attributes(self, super::attributes::ElementWithHeightWidthU32Attributes::attributes::width(value))
    }
}
impl<C, A, ELS> ElementWithHeightWidthU32Attributes for super::props::ElementWithHeightWidthU32Attributes<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithHeightWidthU32Attributes<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithHeightWidthU32Attributes<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithHeightWidthU32Attributes, Props: ElementWithHeightWidthU32Attributes> ElementWithHeightWidthU32Attributes for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithHeightWidthU32Attributes<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithHeightWidthU32Attributes<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithHeightWidthU32Attributes { props: self.props.children(children) }
    }
}
pub trait ElementWithMaxF64Attribute: Element {
    fn max<V: crate::impl_bounds::MaybeValue::Bounds<f64>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithMaxF64Attribute::attributes::max<V>> {
        Self::append_attributes(self, super::attributes::ElementWithMaxF64Attribute::attributes::max(value))
    }
}
impl<C, A, ELS> ElementWithMaxF64Attribute for super::props::ElementWithMaxF64Attribute<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithMaxF64Attribute<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithMaxF64Attribute<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithMaxF64Attribute, Props: ElementWithMaxF64Attribute> ElementWithMaxF64Attribute for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithMaxF64Attribute<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithMaxF64Attribute<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithMaxF64Attribute { props: self.props.children(children) }
    }
}
pub trait ElementWithValueF64Attribute: Element {
    fn value<V: crate::impl_bounds::MaybeValue::Bounds<f64>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithValueF64Attribute::attributes::value<V>> {
        Self::append_attributes(self, super::attributes::ElementWithValueF64Attribute::attributes::value(value))
    }
}
impl<C, A, ELS> ElementWithValueF64Attribute for super::props::ElementWithValueF64Attribute<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithValueF64Attribute<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithValueF64Attribute<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithValueF64Attribute, Props: ElementWithValueF64Attribute> ElementWithValueF64Attribute for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithValueF64Attribute<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithValueF64Attribute<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithValueF64Attribute { props: self.props.children(children) }
    }
}
pub trait ElementWithValueStrAttribute: Element {
    fn value<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithValueStrAttribute::attributes::value<V>> {
        Self::append_attributes(self, super::attributes::ElementWithValueStrAttribute::attributes::value(value))
    }
}
impl<C, A, ELS> ElementWithValueStrAttribute for super::props::ElementWithValueStrAttribute<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithValueStrAttribute<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithValueStrAttribute<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithValueStrAttribute, Props: ElementWithValueStrAttribute> ElementWithValueStrAttribute for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithValueStrAttribute<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithValueStrAttribute<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithValueStrAttribute { props: self.props.children(children) }
    }
}
pub trait ElementWithOpenAttribute: Element {
    fn open<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithOpenAttribute::attributes::open<V>> {
        Self::append_attributes(self, super::attributes::ElementWithOpenAttribute::attributes::open(value))
    }
}
impl<C, A, ELS> ElementWithOpenAttribute for super::props::ElementWithOpenAttribute<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithOpenAttribute<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithOpenAttribute<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithOpenAttribute, Props: ElementWithOpenAttribute> ElementWithOpenAttribute for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithOpenAttribute<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithOpenAttribute<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithOpenAttribute { props: self.props.children(children) }
    }
}
pub trait ElementWithNameAttribute: Element {
    fn name<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithNameAttribute::attributes::name<V>> {
        Self::append_attributes(self, super::attributes::ElementWithNameAttribute::attributes::name(value))
    }
}
impl<C, A, ELS> ElementWithNameAttribute for super::props::ElementWithNameAttribute<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithNameAttribute<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithNameAttribute<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithNameAttribute, Props: ElementWithNameAttribute> ElementWithNameAttribute for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithNameAttribute<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithNameAttribute<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithNameAttribute { props: self.props.children(children) }
    }
}
pub trait ElementWithDisabledAttribute: Element {
    fn disabled<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithDisabledAttribute::attributes::disabled<V>> {
        Self::append_attributes(self, super::attributes::ElementWithDisabledAttribute::attributes::disabled(value))
    }
}
impl<C, A, ELS> ElementWithDisabledAttribute for super::props::ElementWithDisabledAttribute<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithDisabledAttribute<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithDisabledAttribute<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithDisabledAttribute, Props: ElementWithDisabledAttribute> ElementWithDisabledAttribute for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithDisabledAttribute<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithDisabledAttribute<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithDisabledAttribute { props: self.props.children(children) }
    }
}
pub trait ElementWithCrossOriginAttribute: Element {
    fn cross_origin<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithCrossOriginAttribute::attributes::cross_origin<V>> {
        Self::append_attributes(self, super::attributes::ElementWithCrossOriginAttribute::attributes::cross_origin(value))
    }
}
impl<C, A, ELS> ElementWithCrossOriginAttribute for super::props::ElementWithCrossOriginAttribute<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithCrossOriginAttribute<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithCrossOriginAttribute<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithCrossOriginAttribute, Props: ElementWithCrossOriginAttribute> ElementWithCrossOriginAttribute for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithCrossOriginAttribute<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithCrossOriginAttribute<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithCrossOriginAttribute { props: self.props.children(children) }
    }
}
pub trait ElementWithRelAttribute: Element {
    fn rel<V: DomTokens::Bounds>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithRelAttribute::attributes::rel<V>> {
        Self::append_attributes(self, super::attributes::ElementWithRelAttribute::attributes::rel(value))
    }
}
impl<C, A, ELS> ElementWithRelAttribute for super::props::ElementWithRelAttribute<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithRelAttribute<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithRelAttribute<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithRelAttribute, Props: ElementWithRelAttribute> ElementWithRelAttribute for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithRelAttribute<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithRelAttribute<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithRelAttribute { props: self.props.children(children) }
    }
}
pub trait ElementWithReferrerPolicyAttribute: Element {
    fn referrer_policy<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithReferrerPolicyAttribute::attributes::referrer_policy<V>> {
        Self::append_attributes(self, super::attributes::ElementWithReferrerPolicyAttribute::attributes::referrer_policy(value))
    }
}
impl<C, A, ELS> ElementWithReferrerPolicyAttribute for super::props::ElementWithReferrerPolicyAttribute<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithReferrerPolicyAttribute<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithReferrerPolicyAttribute<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithReferrerPolicyAttribute, Props: ElementWithReferrerPolicyAttribute> ElementWithReferrerPolicyAttribute for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithReferrerPolicyAttribute<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithReferrerPolicyAttribute<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithReferrerPolicyAttribute { props: self.props.children(children) }
    }
}
pub trait ElementWithAltAttribute: Element {
    fn alt<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithAltAttribute::attributes::alt<V>> {
        Self::append_attributes(self, super::attributes::ElementWithAltAttribute::attributes::alt(value))
    }
}
impl<C, A, ELS> ElementWithAltAttribute for super::props::ElementWithAltAttribute<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithAltAttribute<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithAltAttribute<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithAltAttribute, Props: ElementWithAltAttribute> ElementWithAltAttribute for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithAltAttribute<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithAltAttribute<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithAltAttribute { props: self.props.children(children) }
    }
}
pub trait ElementWithLoadingAttribute: Element {
    fn loading<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithLoadingAttribute::attributes::loading<V>> {
        Self::append_attributes(self, super::attributes::ElementWithLoadingAttribute::attributes::loading(value))
    }
}
impl<C, A, ELS> ElementWithLoadingAttribute for super::props::ElementWithLoadingAttribute<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithLoadingAttribute<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithLoadingAttribute<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithLoadingAttribute, Props: ElementWithLoadingAttribute> ElementWithLoadingAttribute for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithLoadingAttribute<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithLoadingAttribute<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithLoadingAttribute { props: self.props.children(children) }
    }
}
pub trait ElementWithAcceptAttribute: Element {
    fn accept<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithAcceptAttribute::attributes::accept<V>> {
        Self::append_attributes(self, super::attributes::ElementWithAcceptAttribute::attributes::accept(value))
    }
}
impl<C, A, ELS> ElementWithAcceptAttribute for super::props::ElementWithAcceptAttribute<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithAcceptAttribute<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithAcceptAttribute<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithAcceptAttribute, Props: ElementWithAcceptAttribute> ElementWithAcceptAttribute for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithAcceptAttribute<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithAcceptAttribute<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithAcceptAttribute { props: self.props.children(children) }
    }
}
pub trait ElementWithAutoCompleteAttribute: Element {
    fn auto_complete<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithAutoCompleteAttribute::attributes::auto_complete<V>> {
        Self::append_attributes(self, super::attributes::ElementWithAutoCompleteAttribute::attributes::auto_complete(value))
    }
}
impl<C, A, ELS> ElementWithAutoCompleteAttribute for super::props::ElementWithAutoCompleteAttribute<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithAutoCompleteAttribute<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithAutoCompleteAttribute<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithAutoCompleteAttribute, Props: ElementWithAutoCompleteAttribute> ElementWithAutoCompleteAttribute for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithAutoCompleteAttribute<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithAutoCompleteAttribute<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithAutoCompleteAttribute { props: self.props.children(children) }
    }
}
pub trait ElementWithAutoCorrectAttribute: Element {
    fn auto_correct<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithAutoCorrectAttribute::attributes::auto_correct<V>> {
        Self::append_attributes(self, super::attributes::ElementWithAutoCorrectAttribute::attributes::auto_correct(value))
    }
}
impl<C, A, ELS> ElementWithAutoCorrectAttribute for super::props::ElementWithAutoCorrectAttribute<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithAutoCorrectAttribute<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithAutoCorrectAttribute<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithAutoCorrectAttribute, Props: ElementWithAutoCorrectAttribute> ElementWithAutoCorrectAttribute for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithAutoCorrectAttribute<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithAutoCorrectAttribute<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithAutoCorrectAttribute { props: self.props.children(children) }
    }
}
pub trait ElementWithFormAttribute: Element {
    fn form<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithFormAttribute::attributes::form<V>> {
        Self::append_attributes(self, super::attributes::ElementWithFormAttribute::attributes::form(value))
    }
}
impl<C, A, ELS> ElementWithFormAttribute for super::props::ElementWithFormAttribute<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithFormAttribute<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithFormAttribute<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithFormAttribute, Props: ElementWithFormAttribute> ElementWithFormAttribute for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithFormAttribute<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithFormAttribute<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithFormAttribute { props: self.props.children(children) }
    }
}
pub trait ElementWithFormAttributes: Element + ElementWithFormAttribute {
    fn form_action<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithFormAttributes::attributes::form_action<V>> {
        Self::append_attributes(self, super::attributes::ElementWithFormAttributes::attributes::form_action(value))
    }
    fn form_enc_type<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithFormAttributes::attributes::form_enc_type<V>> {
        Self::append_attributes(self, super::attributes::ElementWithFormAttributes::attributes::form_enc_type(value))
    }
    fn form_method<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithFormAttributes::attributes::form_method<V>> {
        Self::append_attributes(self, super::attributes::ElementWithFormAttributes::attributes::form_method(value))
    }
    fn form_no_validate<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithFormAttributes::attributes::form_no_validate<V>> {
        Self::append_attributes(self, super::attributes::ElementWithFormAttributes::attributes::form_no_validate(value))
    }
    fn form_target<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithFormAttributes::attributes::form_target<V>> {
        Self::append_attributes(self, super::attributes::ElementWithFormAttributes::attributes::form_target(value))
    }
}
impl<C, A, ELS> ElementWithFormAttributes for super::props::ElementWithFormAttributes<C, A, ELS> {}
impl<C, A, ELS> ElementWithFormAttribute for super::props::ElementWithFormAttributes<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithFormAttributes<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithFormAttributes<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithFormAttributes, Props: ElementWithFormAttributes> ElementWithFormAttributes for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithFormAttributes<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithFormAttributes<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithFormAttributes { props: self.props.children(children) }
    }
}
pub trait ElementWithFetchPriorityAttribute: Element {
    fn fetch_priority<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithFetchPriorityAttribute::attributes::fetch_priority<V>> {
        Self::append_attributes(self, super::attributes::ElementWithFetchPriorityAttribute::attributes::fetch_priority(value))
    }
}
impl<C, A, ELS> ElementWithFetchPriorityAttribute for super::props::ElementWithFetchPriorityAttribute<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithFetchPriorityAttribute<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithFetchPriorityAttribute<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithFetchPriorityAttribute, Props: ElementWithFetchPriorityAttribute> ElementWithFetchPriorityAttribute for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithFetchPriorityAttribute<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithFetchPriorityAttribute<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithFetchPriorityAttribute { props: self.props.children(children) }
    }
}
pub trait ElementWithHrefLangAttribute: Element + ElementWithHrefAttribute {
    fn href_lang<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithHrefLangAttribute::attributes::href_lang<V>> {
        Self::append_attributes(self, super::attributes::ElementWithHrefLangAttribute::attributes::href_lang(value))
    }
}
impl<C, A, ELS> ElementWithHrefLangAttribute for super::props::ElementWithHrefLangAttribute<C, A, ELS> {}
impl<C, A, ELS> ElementWithHrefAttribute for super::props::ElementWithHrefLangAttribute<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithHrefLangAttribute<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithHrefLangAttribute<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithHrefLangAttribute, Props: ElementWithHrefLangAttribute> ElementWithHrefLangAttribute for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithHrefLangAttribute<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithHrefLangAttribute<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithHrefLangAttribute { props: self.props.children(children) }
    }
}
pub trait ElementWithSizesAttribute: Element {
    fn sizes<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithSizesAttribute::attributes::sizes<V>> {
        Self::append_attributes(self, super::attributes::ElementWithSizesAttribute::attributes::sizes(value))
    }
}
impl<C, A, ELS> ElementWithSizesAttribute for super::props::ElementWithSizesAttribute<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithSizesAttribute<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithSizesAttribute<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithSizesAttribute, Props: ElementWithSizesAttribute> ElementWithSizesAttribute for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithSizesAttribute<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithSizesAttribute<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithSizesAttribute { props: self.props.children(children) }
    }
}
pub trait ElementWithUseMapAttribute: Element {
    fn use_map<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithUseMapAttribute::attributes::use_map<V>> {
        Self::append_attributes(self, super::attributes::ElementWithUseMapAttribute::attributes::use_map(value))
    }
}
impl<C, A, ELS> ElementWithUseMapAttribute for super::props::ElementWithUseMapAttribute<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithUseMapAttribute<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithUseMapAttribute<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithUseMapAttribute, Props: ElementWithUseMapAttribute> ElementWithUseMapAttribute for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithUseMapAttribute<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithUseMapAttribute<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithUseMapAttribute { props: self.props.children(children) }
    }
}
pub trait ElementWithLabelAttribute: Element {
    fn label<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithLabelAttribute::attributes::label<V>> {
        Self::append_attributes(self, super::attributes::ElementWithLabelAttribute::attributes::label(value))
    }
}
impl<C, A, ELS> ElementWithLabelAttribute for super::props::ElementWithLabelAttribute<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithLabelAttribute<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithLabelAttribute<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithLabelAttribute, Props: ElementWithLabelAttribute> ElementWithLabelAttribute for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithLabelAttribute<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithLabelAttribute<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithLabelAttribute { props: self.props.children(children) }
    }
}
pub trait ElementWithForAttribute: Element {
    fn r#for<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithForAttribute::attributes::r#for<V>> {
        Self::append_attributes(self, super::attributes::ElementWithForAttribute::attributes::r#for(value))
    }
    fn html_for<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithForAttribute::attributes::r#for<V>> {
        Self::append_attributes(self, super::attributes::ElementWithForAttribute::attributes::r#for(value))
    }
}
impl<C, A, ELS> ElementWithForAttribute for super::props::ElementWithForAttribute<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithForAttribute<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithForAttribute<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithForAttribute, Props: ElementWithForAttribute> ElementWithForAttribute for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithForAttribute<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithForAttribute<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithForAttribute { props: self.props.children(children) }
    }
}
pub trait ElementWithIntegrityAttribute: Element {
    fn integrity<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithIntegrityAttribute::attributes::integrity<V>> {
        Self::append_attributes(self, super::attributes::ElementWithIntegrityAttribute::attributes::integrity(value))
    }
}
impl<C, A, ELS> ElementWithIntegrityAttribute for super::props::ElementWithIntegrityAttribute<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithIntegrityAttribute<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithIntegrityAttribute<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithIntegrityAttribute, Props: ElementWithIntegrityAttribute> ElementWithIntegrityAttribute for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithIntegrityAttribute<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithIntegrityAttribute<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithIntegrityAttribute { props: self.props.children(children) }
    }
}
pub trait ElementWithBlockingAttribute: Element {
    fn blocking<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithBlockingAttribute::attributes::blocking<V>> {
        Self::append_attributes(self, super::attributes::ElementWithBlockingAttribute::attributes::blocking(value))
    }
}
impl<C, A, ELS> ElementWithBlockingAttribute for super::props::ElementWithBlockingAttribute<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithBlockingAttribute<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithBlockingAttribute<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithBlockingAttribute, Props: ElementWithBlockingAttribute> ElementWithBlockingAttribute for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithBlockingAttribute<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithBlockingAttribute<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithBlockingAttribute { props: self.props.children(children) }
    }
}
pub trait ElementWithMultipleAttribute: Element {
    fn multiple<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithMultipleAttribute::attributes::multiple<V>> {
        Self::append_attributes(self, super::attributes::ElementWithMultipleAttribute::attributes::multiple(value))
    }
}
impl<C, A, ELS> ElementWithMultipleAttribute for super::props::ElementWithMultipleAttribute<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithMultipleAttribute<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithMultipleAttribute<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithMultipleAttribute, Props: ElementWithMultipleAttribute> ElementWithMultipleAttribute for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithMultipleAttribute<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithMultipleAttribute<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithMultipleAttribute { props: self.props.children(children) }
    }
}
pub trait ElementWithRequiredAttribute: Element {
    fn required<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithRequiredAttribute::attributes::required<V>> {
        Self::append_attributes(self, super::attributes::ElementWithRequiredAttribute::attributes::required(value))
    }
}
impl<C, A, ELS> ElementWithRequiredAttribute for super::props::ElementWithRequiredAttribute<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithRequiredAttribute<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithRequiredAttribute<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithRequiredAttribute, Props: ElementWithRequiredAttribute> ElementWithRequiredAttribute for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithRequiredAttribute<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithRequiredAttribute<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithRequiredAttribute { props: self.props.children(children) }
    }
}
pub trait ElementWithSizeU32Attribute: Element {
    fn size<V: crate::impl_bounds::MaybeValue::Bounds<u32>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithSizeU32Attribute::attributes::size<V>> {
        Self::append_attributes(self, super::attributes::ElementWithSizeU32Attribute::attributes::size(value))
    }
}
impl<C, A, ELS> ElementWithSizeU32Attribute for super::props::ElementWithSizeU32Attribute<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithSizeU32Attribute<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithSizeU32Attribute<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithSizeU32Attribute, Props: ElementWithSizeU32Attribute> ElementWithSizeU32Attribute for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithSizeU32Attribute<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithSizeU32Attribute<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithSizeU32Attribute { props: self.props.children(children) }
    }
}
pub trait ElementWithSrcAttribute: Element {
    fn src<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithSrcAttribute::attributes::src<V>> {
        Self::append_attributes(self, super::attributes::ElementWithSrcAttribute::attributes::src(value))
    }
}
impl<C, A, ELS> ElementWithSrcAttribute for super::props::ElementWithSrcAttribute<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithSrcAttribute<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithSrcAttribute<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithSrcAttribute, Props: ElementWithSrcAttribute> ElementWithSrcAttribute for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithSrcAttribute<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithSrcAttribute<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithSrcAttribute { props: self.props.children(children) }
    }
}
pub trait ElementWithSrcsetAttribute: Element + ElementWithSrcAttribute {
    fn srcset<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithSrcsetAttribute::attributes::srcset<V>> {
        Self::append_attributes(self, super::attributes::ElementWithSrcsetAttribute::attributes::srcset(value))
    }
}
impl<C, A, ELS> ElementWithSrcsetAttribute for super::props::ElementWithSrcsetAttribute<C, A, ELS> {}
impl<C, A, ELS> ElementWithSrcAttribute for super::props::ElementWithSrcsetAttribute<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithSrcsetAttribute<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithSrcsetAttribute<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithSrcsetAttribute, Props: ElementWithSrcsetAttribute> ElementWithSrcsetAttribute for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithSrcsetAttribute<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithSrcsetAttribute<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithSrcsetAttribute { props: self.props.children(children) }
    }
}
pub trait ElementWithBgColorAttribute: Element {
    fn bg_color<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithBgColorAttribute::attributes::bg_color<V>> {
        Self::append_attributes(self, super::attributes::ElementWithBgColorAttribute::attributes::bg_color(value))
    }
}
impl<C, A, ELS> ElementWithBgColorAttribute for super::props::ElementWithBgColorAttribute<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithBgColorAttribute<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithBgColorAttribute<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithBgColorAttribute, Props: ElementWithBgColorAttribute> ElementWithBgColorAttribute for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithBgColorAttribute<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithBgColorAttribute<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithBgColorAttribute { props: self.props.children(children) }
    }
}
pub trait ElementWithAlignAttribute: Element {
    fn align<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithAlignAttribute::attributes::align<V>> {
        Self::append_attributes(self, super::attributes::ElementWithAlignAttribute::attributes::align(value))
    }
}
impl<C, A, ELS> ElementWithAlignAttribute for super::props::ElementWithAlignAttribute<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithAlignAttribute<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithAlignAttribute<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithAlignAttribute, Props: ElementWithAlignAttribute> ElementWithAlignAttribute for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithAlignAttribute<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithAlignAttribute<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithAlignAttribute { props: self.props.children(children) }
    }
}
pub trait ElementWithMediaAttribute: Element {
    fn media<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithMediaAttribute::attributes::media<V>> {
        Self::append_attributes(self, super::attributes::ElementWithMediaAttribute::attributes::media(value))
    }
}
impl<C, A, ELS> ElementWithMediaAttribute for super::props::ElementWithMediaAttribute<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithMediaAttribute<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithMediaAttribute<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithMediaAttribute, Props: ElementWithMediaAttribute> ElementWithMediaAttribute for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithMediaAttribute<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithMediaAttribute<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithMediaAttribute { props: self.props.children(children) }
    }
}
pub trait ElementWithReadOnlyAttribute: Element {
    fn read_only<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithReadOnlyAttribute::attributes::read_only<V>> {
        Self::append_attributes(self, super::attributes::ElementWithReadOnlyAttribute::attributes::read_only(value))
    }
}
impl<C, A, ELS> ElementWithReadOnlyAttribute for super::props::ElementWithReadOnlyAttribute<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithReadOnlyAttribute<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithReadOnlyAttribute<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithReadOnlyAttribute, Props: ElementWithReadOnlyAttribute> ElementWithReadOnlyAttribute for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithReadOnlyAttribute<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithReadOnlyAttribute<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithReadOnlyAttribute { props: self.props.children(children) }
    }
}
pub trait ElementWithDateTimeAttribute: Element {
    fn date_time<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithDateTimeAttribute::attributes::date_time<V>> {
        Self::append_attributes(self, super::attributes::ElementWithDateTimeAttribute::attributes::date_time(value))
    }
}
impl<C, A, ELS> ElementWithDateTimeAttribute for super::props::ElementWithDateTimeAttribute<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::ElementWithDateTimeAttribute<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::ElementWithDateTimeAttribute<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::ElementWithDateTimeAttribute, Props: ElementWithDateTimeAttribute> ElementWithDateTimeAttribute for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithDateTimeAttribute<crate::Empty, A, ELS> {
    type WithChildren = super::props::ElementWithDateTimeAttribute<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithDateTimeAttribute { props: self.props.children(children) }
    }
}
pub trait HtmlElement: Element {
    fn ref_html_element<V: SetRef::Bounds<frender_dom::node_ref::HtmlElement>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::ref_html_element<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::ref_html_element(value))
    }
    fn access_key<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::access_key<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::access_key(value))
    }
    fn auto_capitalize<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::auto_capitalize<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::auto_capitalize(value))
    }
    fn auto_focus<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::auto_focus<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::auto_focus(value))
    }
    fn content_editable<V: crate::impl_bounds::MaybeValue::Bounds<ContentEditable<'static>>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::content_editable<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::content_editable(value))
    }
    #[deprecated = "See https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/contextMenu"]
    fn context_menu<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::context_menu<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::context_menu(value))
    }
    fn dir<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::dir<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::dir(value))
    }
    fn draggable<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::draggable<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::draggable(value))
    }
    fn enter_key_hint<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::enter_key_hint<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::enter_key_hint(value))
    }
    fn hidden<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::hidden<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::hidden(value))
    }
    fn inert<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::inert<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::inert(value))
    }
    fn input_mode<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::input_mode<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::input_mode(value))
    }
    fn is<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::is<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::is(value))
    }
    fn item_id<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::item_id<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::item_id(value))
    }
    fn item_prop<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::item_prop<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::item_prop(value))
    }
    fn item_ref<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::item_ref<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::item_ref(value))
    }
    fn item_scope<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::item_scope<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::item_scope(value))
    }
    fn item_type<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::item_type<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::item_type(value))
    }
    fn lang<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::lang<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::lang(value))
    }
    fn nonce<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::nonce<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::nonce(value))
    }
    fn role<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::role<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::role(value))
    }
    fn slot<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::slot<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::slot(value))
    }
    fn spellcheck<V: crate::impl_bounds::MaybeValue::Bounds<Spellcheck>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::spellcheck<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::spellcheck(value))
    }
    fn style<V: Style::Bounds>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::style<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::style(value))
    }
    fn tab_index<V: crate::impl_bounds::MaybeValue::Bounds<i32>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::tab_index<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::tab_index(value))
    }
    fn title<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::title<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::title(value))
    }
    fn translate<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::translate<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::translate(value))
    }
    fn virtual_keyboard_policy<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::virtual_keyboard_policy<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::virtual_keyboard_policy(value))
    }
    /// Event [`invalid`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/invalid_event)
    ///
    /// Fired when an element does not satisfy its constraints during constraint validation.
    fn on_invalid<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlElement::attributes::on_invalid<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlElement::attributes::on_invalid(value))
    }
    /// Event [`animationcancel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/animationcancel_event)
    ///
    /// Fired when an animation unexpectedly aborts.
    fn on_animation_cancel<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::AnimationEvent> + 'static>(
        self,
        value: V,
    ) -> Self::AppendEventListeners<super::attributes::HtmlElement::attributes::on_animation_cancel<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlElement::attributes::on_animation_cancel(value))
    }
    /// Event [`animationend`](https://developer.mozilla.org/en-US/docs/Web/API/Element/animationend_event)
    ///
    /// Fired when an animation has completed normally.
    fn on_animation_end<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::AnimationEvent> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlElement::attributes::on_animation_end<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlElement::attributes::on_animation_end(value))
    }
    /// Event [`animationiteration`](https://developer.mozilla.org/en-US/docs/Web/API/Element/animationiteration_event)
    ///
    /// Fired when an animation iteration has completed.
    fn on_animation_iteration<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::AnimationEvent> + 'static>(
        self,
        value: V,
    ) -> Self::AppendEventListeners<super::attributes::HtmlElement::attributes::on_animation_iteration<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlElement::attributes::on_animation_iteration(value))
    }
    /// Event [`animationstart`](https://developer.mozilla.org/en-US/docs/Web/API/Element/animationstart_event)
    ///
    /// Fired when an animation starts.
    fn on_animation_start<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::AnimationEvent> + 'static>(
        self,
        value: V,
    ) -> Self::AppendEventListeners<super::attributes::HtmlElement::attributes::on_animation_start<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlElement::attributes::on_animation_start(value))
    }
    /// Event [`beforeinput`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/beforeinput_event)
    ///
    /// Fired when the value of an [`<input>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input), [`<select>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select), or [`<textarea>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea) element is about to be modified.
    fn on_before_input<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::InputEvent> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlElement::attributes::on_before_input<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlElement::attributes::on_before_input(value))
    }
    /// Event [`input`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/input_event)
    ///
    /// Fired when the `value` of an [`<input>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input), [`<select>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select), or [`<textarea>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea) element has been changed.
    fn on_input<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::InputEvent> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlElement::attributes::on_input<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlElement::attributes::on_input(value))
    }
    /// Event [`change`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/change_event)
    ///
    /// Fired when the `value` of an [`<input>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input), [`<select>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select), or [`<textarea>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea) element has been changed and committed by the user. Unlike the [`input`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/input_event) event, the `change` event is not necessarily fired for each alteration to an element's `value`.
    fn on_change<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlElement::attributes::on_change<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlElement::attributes::on_change(value))
    }
    /// Event [`gotpointercapture`](https://developer.mozilla.org/en-US/docs/Web/API/Element/gotpointercapture_event)
    ///
    /// Fired when an element captures a pointer using [`setPointerCapture()`](https://developer.mozilla.org/en-US/docs/Web/API/Element/setPointerCapture).
    fn on_got_pointer_capture<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::PointerEvent> + 'static>(
        self,
        value: V,
    ) -> Self::AppendEventListeners<super::attributes::HtmlElement::attributes::on_got_pointer_capture<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlElement::attributes::on_got_pointer_capture(value))
    }
    /// Event [`lostpointercapture`](https://developer.mozilla.org/en-US/docs/Web/API/Element/lostpointercapture_event)
    ///
    /// Fired when a [captured pointer](https://developer.mozilla.org/en-US/docs/Web/API/Pointer_events#pointer_capture) is released.
    fn on_lost_pointer_capture<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::PointerEvent> + 'static>(
        self,
        value: V,
    ) -> Self::AppendEventListeners<super::attributes::HtmlElement::attributes::on_lost_pointer_capture<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlElement::attributes::on_lost_pointer_capture(value))
    }
    /// Event [`pointercancel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointercancel_event)
    ///
    /// Fired when a pointer event is canceled.
    fn on_pointer_cancel<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::PointerEvent> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlElement::attributes::on_pointer_cancel<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlElement::attributes::on_pointer_cancel(value))
    }
    /// Event [`pointerdown`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerdown_event)
    ///
    /// Fired when a pointer becomes active.
    fn on_pointer_down<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::PointerEvent> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlElement::attributes::on_pointer_down<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlElement::attributes::on_pointer_down(value))
    }
    /// Event [`pointerenter`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerenter_event)
    ///
    /// Fired when a pointer is moved into the hit test boundaries of an element or one of its descendants.
    fn on_pointer_enter<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::PointerEvent> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlElement::attributes::on_pointer_enter<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlElement::attributes::on_pointer_enter(value))
    }
    /// Event [`pointerleave`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerleave_event)
    ///
    /// Fired when a pointer is moved out of the hit test boundaries of an element.
    fn on_pointer_leave<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::PointerEvent> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlElement::attributes::on_pointer_leave<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlElement::attributes::on_pointer_leave(value))
    }
    /// Event [`pointermove`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointermove_event)
    ///
    /// Fired when a pointer changes coordinates.
    fn on_pointer_move<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::PointerEvent> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlElement::attributes::on_pointer_move<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlElement::attributes::on_pointer_move(value))
    }
    /// Event [`pointerout`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerout_event)
    ///
    /// Fired when a pointer is moved out of the *hit test* boundaries of an element (among other reasons).
    fn on_pointer_out<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::PointerEvent> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlElement::attributes::on_pointer_out<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlElement::attributes::on_pointer_out(value))
    }
    /// Event [`pointerover`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerover_event)
    ///
    /// Fired when a pointer is moved into an element's hit test boundaries.
    fn on_pointer_over<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::PointerEvent> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlElement::attributes::on_pointer_over<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlElement::attributes::on_pointer_over(value))
    }
    /// Event [`pointerup`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerup_event)
    ///
    /// Fired when a pointer is no longer active.
    fn on_pointer_up<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::PointerEvent> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlElement::attributes::on_pointer_up<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlElement::attributes::on_pointer_up(value))
    }
    /// Event [`transitioncancel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/transitioncancel_event)
    ///
    /// Fired when a [CSS transition](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Transitions/Using_CSS_transitions) is canceled.
    fn on_transition_cancel<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::TransitionEvent> + 'static>(
        self,
        value: V,
    ) -> Self::AppendEventListeners<super::attributes::HtmlElement::attributes::on_transition_cancel<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlElement::attributes::on_transition_cancel(value))
    }
    /// Event [`transitionend`](https://developer.mozilla.org/en-US/docs/Web/API/Element/transitionend_event)
    ///
    /// Fired when a [CSS transition](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Transitions/Using_CSS_transitions) has completed.
    fn on_transition_end<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::TransitionEvent> + 'static>(
        self,
        value: V,
    ) -> Self::AppendEventListeners<super::attributes::HtmlElement::attributes::on_transition_end<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlElement::attributes::on_transition_end(value))
    }
    /// Event [`transitionrun`](https://developer.mozilla.org/en-US/docs/Web/API/Element/transitionrun_event)
    ///
    /// Fired when a [CSS transition](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Transitions/Using_CSS_transitions) is first created.
    fn on_transition_run<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::TransitionEvent> + 'static>(
        self,
        value: V,
    ) -> Self::AppendEventListeners<super::attributes::HtmlElement::attributes::on_transition_run<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlElement::attributes::on_transition_run(value))
    }
    /// Event [`transitionstart`](https://developer.mozilla.org/en-US/docs/Web/API/Element/transitionstart_event)
    ///
    /// Fired when a [CSS transition](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Transitions/Using_CSS_transitions) has actually started.
    fn on_transition_start<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::TransitionEvent> + 'static>(
        self,
        value: V,
    ) -> Self::AppendEventListeners<super::attributes::HtmlElement::attributes::on_transition_start<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlElement::attributes::on_transition_start(value))
    }
    /// Event [`drag`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/drag_event)
    ///
    /// This event is fired when an element or text selection is being dragged.
    fn on_drag<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlElement::attributes::on_drag<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlElement::attributes::on_drag(value))
    }
    /// Event [`dragend`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragend_event)
    ///
    /// This event is fired when a drag operation is being ended (by releasing a mouse button or hitting the escape key).
    fn on_drag_end<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlElement::attributes::on_drag_end<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlElement::attributes::on_drag_end(value))
    }
    /// Event [`dragenter`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragenter_event)
    ///
    /// This event is fired when a dragged element or text selection enters a valid drop target.
    fn on_drag_enter<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlElement::attributes::on_drag_enter<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlElement::attributes::on_drag_enter(value))
    }
    /// Event [`dragleave`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragleave_event)
    ///
    /// This event is fired when a dragged element or text selection leaves a valid drop target.
    fn on_drag_leave<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlElement::attributes::on_drag_leave<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlElement::attributes::on_drag_leave(value))
    }
    /// Event [`dragover`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragover_event)
    ///
    /// This event is fired continuously when an element or text selection is being dragged and the mouse pointer is over a valid drop target (every 50 ms WHEN mouse is not moving ELSE much faster between 5 ms (slow movement) and 1ms (fast movement) approximately. This firing pattern is different than [`mouseover`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseover_event) ).
    fn on_drag_over<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlElement::attributes::on_drag_over<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlElement::attributes::on_drag_over(value))
    }
    /// Event [`dragstart`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragstart_event)
    ///
    /// This event is fired when the user starts dragging an element or text selection.
    fn on_drag_start<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlElement::attributes::on_drag_start<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlElement::attributes::on_drag_start(value))
    }
    /// Event [`drop`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/drop_event)
    ///
    /// This event is fired when an element or text selection is dropped on a valid drop target.
    fn on_drop<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlElement::attributes::on_drop<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlElement::attributes::on_drop(value))
    }
}
impl<C, A, ELS> HtmlElement for super::props::HtmlElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlElement, Props: HtmlElement> HtmlElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlElement { props: self.props.children(children) }
    }
}
pub trait HtmlDataListElement: HtmlElement {}
impl<C, A, ELS> HtmlDataListElement for super::props::HtmlDataListElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlDataListElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlDataListElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlDataListElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlDataListElement, Props: HtmlDataListElement> HtmlDataListElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlDataListElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlDataListElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlDataListElement { props: self.props.children(children) }
    }
}
pub trait HtmlDivElement: HtmlElement {}
impl<C, A, ELS> HtmlDivElement for super::props::HtmlDivElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlDivElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlDivElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlDivElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlDivElement, Props: HtmlDivElement> HtmlDivElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlDivElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlDivElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlDivElement { props: self.props.children(children) }
    }
}
pub trait HtmlDListElement: HtmlElement {}
impl<C, A, ELS> HtmlDListElement for super::props::HtmlDListElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlDListElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlDListElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlDListElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlDListElement, Props: HtmlDListElement> HtmlDListElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlDListElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlDListElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlDListElement { props: self.props.children(children) }
    }
}
pub trait HtmlHeadingElement: HtmlElement {}
impl<C, A, ELS> HtmlHeadingElement for super::props::HtmlHeadingElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlHeadingElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlHeadingElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlHeadingElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlHeadingElement, Props: HtmlHeadingElement> HtmlHeadingElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlHeadingElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlHeadingElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlHeadingElement { props: self.props.children(children) }
    }
}
pub trait HtmlHeadElement: HtmlElement {}
impl<C, A, ELS> HtmlHeadElement for super::props::HtmlHeadElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlHeadElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlHeadElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlHeadElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlHeadElement, Props: HtmlHeadElement> HtmlHeadElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlHeadElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlHeadElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlHeadElement { props: self.props.children(children) }
    }
}
pub trait HtmlHrElement: HtmlElement {}
impl<C, A, ELS> HtmlHrElement for super::props::HtmlHrElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlHrElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlHrElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlHrElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlHrElement, Props: HtmlHrElement> HtmlHrElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlHrElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlHrElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlHrElement { props: self.props.children(children) }
    }
}
pub trait HtmlLegendElement: HtmlElement {}
impl<C, A, ELS> HtmlLegendElement for super::props::HtmlLegendElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlLegendElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlLegendElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlLegendElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlLegendElement, Props: HtmlLegendElement> HtmlLegendElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlLegendElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlLegendElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlLegendElement { props: self.props.children(children) }
    }
}
pub trait HtmlMenuElement: HtmlElement {}
impl<C, A, ELS> HtmlMenuElement for super::props::HtmlMenuElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlMenuElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlMenuElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlMenuElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlMenuElement, Props: HtmlMenuElement> HtmlMenuElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlMenuElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlMenuElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlMenuElement { props: self.props.children(children) }
    }
}
pub trait HtmlParagraphElement: HtmlElement {}
impl<C, A, ELS> HtmlParagraphElement for super::props::HtmlParagraphElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlParagraphElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlParagraphElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlParagraphElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlParagraphElement, Props: HtmlParagraphElement> HtmlParagraphElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlParagraphElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlParagraphElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlParagraphElement { props: self.props.children(children) }
    }
}
pub trait HtmlPictureElement: HtmlElement {}
impl<C, A, ELS> HtmlPictureElement for super::props::HtmlPictureElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlPictureElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlPictureElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlPictureElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlPictureElement, Props: HtmlPictureElement> HtmlPictureElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlPictureElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlPictureElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlPictureElement { props: self.props.children(children) }
    }
}
pub trait HtmlPreElement: HtmlElement {}
impl<C, A, ELS> HtmlPreElement for super::props::HtmlPreElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlPreElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlPreElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlPreElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlPreElement, Props: HtmlPreElement> HtmlPreElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlPreElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlPreElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlPreElement { props: self.props.children(children) }
    }
}
pub trait HtmlSpanElement: HtmlElement {}
impl<C, A, ELS> HtmlSpanElement for super::props::HtmlSpanElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlSpanElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlSpanElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlSpanElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlSpanElement, Props: HtmlSpanElement> HtmlSpanElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlSpanElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlSpanElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlSpanElement { props: self.props.children(children) }
    }
}
pub trait HtmlTemplateElement: HtmlElement {}
impl<C, A, ELS> HtmlTemplateElement for super::props::HtmlTemplateElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlTemplateElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlTemplateElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlTemplateElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlTemplateElement, Props: HtmlTemplateElement> HtmlTemplateElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlTemplateElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlTemplateElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlTemplateElement { props: self.props.children(children) }
    }
}
pub trait HtmlTitleElement: HtmlElement {}
impl<C, A, ELS> HtmlTitleElement for super::props::HtmlTitleElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlTitleElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlTitleElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlTitleElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlTitleElement, Props: HtmlTitleElement> HtmlTitleElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlTitleElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlTitleElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlTitleElement { props: self.props.children(children) }
    }
}
pub trait HtmlElementWithHref: HtmlElement + ElementWithHrefAttribute + ElementWithTargetAttribute + ElementWithReferrerPolicyAttribute + ElementWithRelAttribute {
    fn download<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElementWithHref::attributes::download<V>> {
        Self::append_attributes(self, super::attributes::HtmlElementWithHref::attributes::download(value))
    }
    fn ping<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElementWithHref::attributes::ping<V>> {
        Self::append_attributes(self, super::attributes::HtmlElementWithHref::attributes::ping(value))
    }
}
impl<C, A, ELS> HtmlElementWithHref for super::props::HtmlElementWithHref<C, A, ELS> {}
impl<C, A, ELS> ElementWithHrefAttribute for super::props::HtmlElementWithHref<C, A, ELS> {}
impl<C, A, ELS> ElementWithTargetAttribute for super::props::HtmlElementWithHref<C, A, ELS> {}
impl<C, A, ELS> ElementWithReferrerPolicyAttribute for super::props::HtmlElementWithHref<C, A, ELS> {}
impl<C, A, ELS> ElementWithRelAttribute for super::props::HtmlElementWithHref<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlElementWithHref<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlElementWithHref<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlElementWithHref<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlElementWithHref, Props: HtmlElementWithHref> HtmlElementWithHref for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlElementWithHref<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlElementWithHref<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlElementWithHref { props: self.props.children(children) }
    }
}
pub trait HtmlAnchorElement:
    HtmlElement + HtmlElementWithHref + ElementWithTypeAttribute + ElementWithHrefLangAttribute + ElementWithHrefAttribute + ElementWithTargetAttribute + ElementWithReferrerPolicyAttribute + ElementWithRelAttribute
{
}
impl<C, A, ELS> HtmlAnchorElement for super::props::HtmlAnchorElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElementWithHref for super::props::HtmlAnchorElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithTypeAttribute for super::props::HtmlAnchorElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithHrefLangAttribute for super::props::HtmlAnchorElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithHrefAttribute for super::props::HtmlAnchorElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithTargetAttribute for super::props::HtmlAnchorElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithReferrerPolicyAttribute for super::props::HtmlAnchorElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithRelAttribute for super::props::HtmlAnchorElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlAnchorElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlAnchorElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlAnchorElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlAnchorElement, Props: HtmlAnchorElement> HtmlAnchorElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlAnchorElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlAnchorElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlAnchorElement { props: self.props.children(children) }
    }
}
pub trait HtmlAreaElement: HtmlElement + HtmlElementWithHref + ElementWithAltAttribute + ElementWithHrefAttribute + ElementWithTargetAttribute + ElementWithReferrerPolicyAttribute + ElementWithRelAttribute {
    fn coords<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlAreaElement::attributes::coords<V>> {
        Self::append_attributes(self, super::attributes::HtmlAreaElement::attributes::coords(value))
    }
    fn shape<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlAreaElement::attributes::shape<V>> {
        Self::append_attributes(self, super::attributes::HtmlAreaElement::attributes::shape(value))
    }
}
impl<C, A, ELS> HtmlAreaElement for super::props::HtmlAreaElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElementWithHref for super::props::HtmlAreaElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithAltAttribute for super::props::HtmlAreaElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithHrefAttribute for super::props::HtmlAreaElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithTargetAttribute for super::props::HtmlAreaElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithReferrerPolicyAttribute for super::props::HtmlAreaElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithRelAttribute for super::props::HtmlAreaElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlAreaElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlAreaElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlAreaElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlAreaElement, Props: HtmlAreaElement> HtmlAreaElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlAreaElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlAreaElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlAreaElement { props: self.props.children(children) }
    }
}
pub trait HtmlMediaElement: HtmlElement + ElementWithSrcAttribute + ElementWithCrossOriginAttribute {
    fn auto_play<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlMediaElement::attributes::auto_play<V>> {
        Self::append_attributes(self, super::attributes::HtmlMediaElement::attributes::auto_play(value))
    }
    fn controls<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlMediaElement::attributes::controls<V>> {
        Self::append_attributes(self, super::attributes::HtmlMediaElement::attributes::controls(value))
    }
    fn r#loop<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlMediaElement::attributes::r#loop<V>> {
        Self::append_attributes(self, super::attributes::HtmlMediaElement::attributes::r#loop(value))
    }
    fn loop_<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlMediaElement::attributes::r#loop<V>> {
        Self::append_attributes(self, super::attributes::HtmlMediaElement::attributes::r#loop(value))
    }
    fn muted<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlMediaElement::attributes::muted<V>> {
        Self::append_attributes(self, super::attributes::HtmlMediaElement::attributes::muted(value))
    }
    fn preload<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlMediaElement::attributes::preload<V>> {
        Self::append_attributes(self, super::attributes::HtmlMediaElement::attributes::preload(value))
    }
    /// Event [`abort`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/abort_event)
    ///
    /// Fired when the resource was not fully loaded, but not as the result of an error.
    fn on_abort<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlMediaElement::attributes::on_abort<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlMediaElement::attributes::on_abort(value))
    }
    /// Event [`canplay`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/canplay_event)
    ///
    /// Fired when the user agent can play the media, but estimates that **not** enough data has been loaded to play the media up to its end without having to stop for further buffering of content.
    fn on_can_play<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlMediaElement::attributes::on_can_play<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlMediaElement::attributes::on_can_play(value))
    }
    /// Event [`canplaythrough`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/canplaythrough_event)
    ///
    /// Fired when the user agent can play the media, and estimates that enough data has been loaded to play the media up to its end without having to stop for further buffering of content.
    fn on_can_play_through<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(
        self,
        value: V,
    ) -> Self::AppendEventListeners<super::attributes::HtmlMediaElement::attributes::on_can_play_through<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlMediaElement::attributes::on_can_play_through(value))
    }
    /// Event [`durationchange`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/durationchange_event)
    ///
    /// Fired when the duration property has been updated.
    fn on_duration_change<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlMediaElement::attributes::on_duration_change<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlMediaElement::attributes::on_duration_change(value))
    }
    /// Event [`emptied`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/emptied_event)
    ///
    /// Fired when the media has become empty; for example, when the media has already been loaded (or partially loaded), and the [`HTMLMediaElement.load()`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/load) method is called to reload it.
    fn on_emptied<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlMediaElement::attributes::on_emptied<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlMediaElement::attributes::on_emptied(value))
    }
    /// Event [`ended`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/ended_event)
    ///
    /// Fired when playback stops when end of the media (<audio> or <video>) is reached or because no further data is available.
    fn on_ended<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlMediaElement::attributes::on_ended<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlMediaElement::attributes::on_ended(value))
    }
    /// Event [`loadeddata`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/loadeddata_event)
    ///
    /// Fired when the first frame of the media has finished loading.
    fn on_loaded_data<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlMediaElement::attributes::on_loaded_data<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlMediaElement::attributes::on_loaded_data(value))
    }
    /// Event [`loadedmetadata`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/loadedmetadata_event)
    ///
    /// Fired when the metadata has been loaded.
    fn on_loaded_metadata<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlMediaElement::attributes::on_loaded_metadata<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlMediaElement::attributes::on_loaded_metadata(value))
    }
    /// Event [`loadstart`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/loadstart_event)
    ///
    /// Fired when the browser has started to load a resource.
    fn on_load_start<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlMediaElement::attributes::on_load_start<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlMediaElement::attributes::on_load_start(value))
    }
    /// Event [`pause`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/pause_event)
    ///
    /// Fired when a request to pause play is handled and the activity has entered its paused state, most commonly occurring when the media's [`HTMLMediaElement.pause()`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/pause) method is called.
    fn on_pause<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlMediaElement::attributes::on_pause<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlMediaElement::attributes::on_pause(value))
    }
    /// Event [`play`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/play_event)
    ///
    /// Fired when the `paused` property is changed from `true` to `false`, as a result of the [`HTMLMediaElement.play()`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/play) method, or the `autoplay` attribute.
    fn on_play<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlMediaElement::attributes::on_play<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlMediaElement::attributes::on_play(value))
    }
    /// Event [`playing`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/playing_event)
    ///
    /// Fired when playback is ready to start after having been paused or delayed due to lack of data.
    fn on_playing<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlMediaElement::attributes::on_playing<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlMediaElement::attributes::on_playing(value))
    }
    /// Event [`progress`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/progress_event)
    ///
    /// Fired periodically as the browser loads a resource.
    fn on_progress<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlMediaElement::attributes::on_progress<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlMediaElement::attributes::on_progress(value))
    }
    /// Event [`ratechange`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/ratechange_event)
    ///
    /// Fired when the playback rate has changed.
    fn on_rate_change<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlMediaElement::attributes::on_rate_change<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlMediaElement::attributes::on_rate_change(value))
    }
    /// Event [`resize`]()
    ///
    /// Fired when one or both of the `videoWidth` and `videoHeight` properties have just been updated.
    fn on_resize<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlMediaElement::attributes::on_resize<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlMediaElement::attributes::on_resize(value))
    }
    /// Event [`seeked`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/seeked_event)
    ///
    /// Fired when a seek operation completes.
    fn on_seeked<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlMediaElement::attributes::on_seeked<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlMediaElement::attributes::on_seeked(value))
    }
    /// Event [`seeking`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/seeking_event)
    ///
    /// Fired when a seek operation begins.
    fn on_seeking<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlMediaElement::attributes::on_seeking<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlMediaElement::attributes::on_seeking(value))
    }
    /// Event [`stalled`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/stalled_event)
    ///
    /// Fired when the user agent is trying to fetch media data, but data is unexpectedly not forthcoming.
    fn on_stalled<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlMediaElement::attributes::on_stalled<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlMediaElement::attributes::on_stalled(value))
    }
    /// Event [`suspend`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/suspend_event)
    ///
    /// Fired when the media data loading has been suspended.
    fn on_suspend<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlMediaElement::attributes::on_suspend<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlMediaElement::attributes::on_suspend(value))
    }
    /// Event [`timeupdate`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/timeupdate_event)
    ///
    /// Fired when the time indicated by the [`currentTime`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/currentTime) property has been updated.
    fn on_time_update<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlMediaElement::attributes::on_time_update<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlMediaElement::attributes::on_time_update(value))
    }
    /// Event [`volumechange`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/volumechange_event)
    ///
    /// Fired when the volume has changed.
    fn on_volume_change<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlMediaElement::attributes::on_volume_change<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlMediaElement::attributes::on_volume_change(value))
    }
    /// Event [`waiting`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/waiting_event)
    ///
    /// Fired when playback has stopped because of a temporary lack of data.
    fn on_waiting<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlMediaElement::attributes::on_waiting<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlMediaElement::attributes::on_waiting(value))
    }
}
impl<C, A, ELS> HtmlMediaElement for super::props::HtmlMediaElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithSrcAttribute for super::props::HtmlMediaElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithCrossOriginAttribute for super::props::HtmlMediaElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlMediaElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlMediaElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlMediaElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlMediaElement, Props: HtmlMediaElement> HtmlMediaElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlMediaElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlMediaElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlMediaElement { props: self.props.children(children) }
    }
}
pub trait HtmlBaseElement: HtmlElement + ElementWithHrefAttribute + ElementWithTargetAttribute {}
impl<C, A, ELS> HtmlBaseElement for super::props::HtmlBaseElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithHrefAttribute for super::props::HtmlBaseElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithTargetAttribute for super::props::HtmlBaseElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlBaseElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlBaseElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlBaseElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlBaseElement, Props: HtmlBaseElement> HtmlBaseElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlBaseElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlBaseElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlBaseElement { props: self.props.children(children) }
    }
}
pub trait HtmlQuoteElement: HtmlElement + ElementWithCiteAttribute {}
impl<C, A, ELS> HtmlQuoteElement for super::props::HtmlQuoteElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithCiteAttribute for super::props::HtmlQuoteElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlQuoteElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlQuoteElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlQuoteElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlQuoteElement, Props: HtmlQuoteElement> HtmlQuoteElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlQuoteElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlQuoteElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlQuoteElement { props: self.props.children(children) }
    }
}
pub trait HtmlBodyElement: HtmlElement {
    #[deprecated = "Use the CSS color property in conjunction with the :active pseudo-class instead."]
    fn alink<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlBodyElement::attributes::alink<V>> {
        Self::append_attributes(self, super::attributes::HtmlBodyElement::attributes::alink(value))
    }
}
impl<C, A, ELS> HtmlBodyElement for super::props::HtmlBodyElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlBodyElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlBodyElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlBodyElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlBodyElement, Props: HtmlBodyElement> HtmlBodyElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlBodyElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlBodyElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlBodyElement { props: self.props.children(children) }
    }
}
pub trait HtmlBrElement: HtmlElement {
    #[deprecated]
    fn clear<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlBrElement::attributes::clear<V>> {
        Self::append_attributes(self, super::attributes::HtmlBrElement::attributes::clear(value))
    }
}
impl<C, A, ELS> HtmlBrElement for super::props::HtmlBrElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlBrElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlBrElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlBrElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlBrElement, Props: HtmlBrElement> HtmlBrElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlBrElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlBrElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlBrElement { props: self.props.children(children) }
    }
}
pub trait HtmlButtonElement:
    HtmlElement + ElementWithTypeAttribute + ElementWithFormAttributes + ElementWithDisabledAttribute + ElementWithNameAttribute + ElementWithValueStrAttribute + ElementWithFormAttribute
{
}
impl<C, A, ELS> HtmlButtonElement for super::props::HtmlButtonElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithTypeAttribute for super::props::HtmlButtonElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithFormAttributes for super::props::HtmlButtonElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithDisabledAttribute for super::props::HtmlButtonElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithNameAttribute for super::props::HtmlButtonElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithValueStrAttribute for super::props::HtmlButtonElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithFormAttribute for super::props::HtmlButtonElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlButtonElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlButtonElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlButtonElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlButtonElement, Props: HtmlButtonElement> HtmlButtonElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlButtonElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlButtonElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlButtonElement { props: self.props.children(children) }
    }
}
pub trait HtmlCanvasElement: HtmlElement + ElementWithHeightWidthU32Attributes {}
impl<C, A, ELS> HtmlCanvasElement for super::props::HtmlCanvasElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithHeightWidthU32Attributes for super::props::HtmlCanvasElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlCanvasElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlCanvasElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlCanvasElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlCanvasElement, Props: HtmlCanvasElement> HtmlCanvasElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlCanvasElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlCanvasElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlCanvasElement { props: self.props.children(children) }
    }
}
pub trait HtmlTableCaptionElement: HtmlElement + ElementWithAlignAttribute {}
impl<C, A, ELS> HtmlTableCaptionElement for super::props::HtmlTableCaptionElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithAlignAttribute for super::props::HtmlTableCaptionElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlTableCaptionElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlTableCaptionElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlTableCaptionElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlTableCaptionElement, Props: HtmlTableCaptionElement> HtmlTableCaptionElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlTableCaptionElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlTableCaptionElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlTableCaptionElement { props: self.props.children(children) }
    }
}
pub trait HtmlDataElement: HtmlElement + ElementWithValueStrAttribute {}
impl<C, A, ELS> HtmlDataElement for super::props::HtmlDataElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithValueStrAttribute for super::props::HtmlDataElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlDataElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlDataElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlDataElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlDataElement, Props: HtmlDataElement> HtmlDataElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlDataElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlDataElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlDataElement { props: self.props.children(children) }
    }
}
pub trait HtmlModElement: HtmlElement + ElementWithCiteAttribute + ElementWithDateTimeAttribute {}
impl<C, A, ELS> HtmlModElement for super::props::HtmlModElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithCiteAttribute for super::props::HtmlModElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithDateTimeAttribute for super::props::HtmlModElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlModElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlModElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlModElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlModElement, Props: HtmlModElement> HtmlModElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlModElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlModElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlModElement { props: self.props.children(children) }
    }
}
pub trait HtmlDetailsElement: HtmlElement + ElementWithOpenAttribute {}
impl<C, A, ELS> HtmlDetailsElement for super::props::HtmlDetailsElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithOpenAttribute for super::props::HtmlDetailsElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlDetailsElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlDetailsElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlDetailsElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlDetailsElement, Props: HtmlDetailsElement> HtmlDetailsElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlDetailsElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlDetailsElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlDetailsElement { props: self.props.children(children) }
    }
}
pub trait HtmlDialogElement: HtmlElement + ElementWithOpenAttribute {}
impl<C, A, ELS> HtmlDialogElement for super::props::HtmlDialogElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithOpenAttribute for super::props::HtmlDialogElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlDialogElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlDialogElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlDialogElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlDialogElement, Props: HtmlDialogElement> HtmlDialogElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlDialogElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlDialogElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlDialogElement { props: self.props.children(children) }
    }
}
pub trait HtmlEmbedElement: HtmlElement + ElementWithTypeAttribute + ElementWithSrcAttribute + ElementWithHeightWidthStrAttributes {}
impl<C, A, ELS> HtmlEmbedElement for super::props::HtmlEmbedElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithTypeAttribute for super::props::HtmlEmbedElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithSrcAttribute for super::props::HtmlEmbedElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithHeightWidthStrAttributes for super::props::HtmlEmbedElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlEmbedElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlEmbedElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlEmbedElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlEmbedElement, Props: HtmlEmbedElement> HtmlEmbedElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlEmbedElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlEmbedElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlEmbedElement { props: self.props.children(children) }
    }
}
pub trait HtmlFieldSetElement: HtmlElement + ElementWithFormAttribute + ElementWithDisabledAttribute + ElementWithNameAttribute {}
impl<C, A, ELS> HtmlFieldSetElement for super::props::HtmlFieldSetElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithFormAttribute for super::props::HtmlFieldSetElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithDisabledAttribute for super::props::HtmlFieldSetElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithNameAttribute for super::props::HtmlFieldSetElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlFieldSetElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlFieldSetElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlFieldSetElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlFieldSetElement, Props: HtmlFieldSetElement> HtmlFieldSetElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlFieldSetElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlFieldSetElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlFieldSetElement { props: self.props.children(children) }
    }
}
pub trait HtmlFormElement: HtmlElement + ElementWithTargetAttribute + ElementWithAutoCompleteAttribute + ElementWithAcceptAttribute + ElementWithRelAttribute + ElementWithNameAttribute {
    fn accept_charset<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlFormElement::attributes::accept_charset<V>> {
        Self::append_attributes(self, super::attributes::HtmlFormElement::attributes::accept_charset(value))
    }
    fn action<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlFormElement::attributes::action<V>> {
        Self::append_attributes(self, super::attributes::HtmlFormElement::attributes::action(value))
    }
    fn enc_type<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlFormElement::attributes::enc_type<V>> {
        Self::append_attributes(self, super::attributes::HtmlFormElement::attributes::enc_type(value))
    }
    fn method<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlFormElement::attributes::method<V>> {
        Self::append_attributes(self, super::attributes::HtmlFormElement::attributes::method(value))
    }
    fn no_validate<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlFormElement::attributes::no_validate<V>> {
        Self::append_attributes(self, super::attributes::HtmlFormElement::attributes::no_validate(value))
    }
    /// Event [`formdata`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/formdata_event)
    ///
    /// The `formdata` event fires after the entry list representing the form's data is constructed.
    fn on_form_data<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlFormElement::attributes::on_form_data<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlFormElement::attributes::on_form_data(value))
    }
    /// Event [`reset`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/reset_event)
    ///
    /// The `reset` event fires when a form is reset.
    fn on_reset<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlFormElement::attributes::on_reset<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlFormElement::attributes::on_reset(value))
    }
    /// Event [`submit`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/submit_event)
    ///
    /// The `submit` event fires when a form is submitted.
    fn on_submit<V: frender_dom::MaybeHandleEvent<dyn crate::dom::event::Event> + 'static>(self, value: V) -> Self::AppendEventListeners<super::attributes::HtmlFormElement::attributes::on_submit<V>> {
        Self::append_event_listeners(self, super::attributes::HtmlFormElement::attributes::on_submit(value))
    }
}
impl<C, A, ELS> HtmlFormElement for super::props::HtmlFormElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithTargetAttribute for super::props::HtmlFormElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithAutoCompleteAttribute for super::props::HtmlFormElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithAcceptAttribute for super::props::HtmlFormElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithRelAttribute for super::props::HtmlFormElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithNameAttribute for super::props::HtmlFormElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlFormElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlFormElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlFormElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlFormElement, Props: HtmlFormElement> HtmlFormElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlFormElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlFormElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlFormElement { props: self.props.children(children) }
    }
}
pub trait HtmlHtmlElement: HtmlElement {
    fn xmlns<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlHtmlElement::attributes::xmlns<V>> {
        Self::append_attributes(self, super::attributes::HtmlHtmlElement::attributes::xmlns(value))
    }
}
impl<C, A, ELS> HtmlHtmlElement for super::props::HtmlHtmlElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlHtmlElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlHtmlElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlHtmlElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlHtmlElement, Props: HtmlHtmlElement> HtmlHtmlElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlHtmlElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlHtmlElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlHtmlElement { props: self.props.children(children) }
    }
}
pub trait HtmlIFrameElement:
    HtmlElement + ElementWithSrcAttribute + ElementWithFetchPriorityAttribute + ElementWithLoadingAttribute + ElementWithReferrerPolicyAttribute + ElementWithNameAttribute + ElementWithHeightWidthStrAttributes
{
    fn allow<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlIFrameElement::attributes::allow<V>> {
        Self::append_attributes(self, super::attributes::HtmlIFrameElement::attributes::allow(value))
    }
    fn allow_fullscreen<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlIFrameElement::attributes::allow_fullscreen<V>> {
        Self::append_attributes(self, super::attributes::HtmlIFrameElement::attributes::allow_fullscreen(value))
    }
    fn allow_payment_request<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlIFrameElement::attributes::allow_payment_request<V>> {
        Self::append_attributes(self, super::attributes::HtmlIFrameElement::attributes::allow_payment_request(value))
    }
    fn csp<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlIFrameElement::attributes::csp<V>> {
        Self::append_attributes(self, super::attributes::HtmlIFrameElement::attributes::csp(value))
    }
    fn sandbox<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlIFrameElement::attributes::sandbox<V>> {
        Self::append_attributes(self, super::attributes::HtmlIFrameElement::attributes::sandbox(value))
    }
    fn src_doc<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlIFrameElement::attributes::src_doc<V>> {
        Self::append_attributes(self, super::attributes::HtmlIFrameElement::attributes::src_doc(value))
    }
}
impl<C, A, ELS> HtmlIFrameElement for super::props::HtmlIFrameElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithSrcAttribute for super::props::HtmlIFrameElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithFetchPriorityAttribute for super::props::HtmlIFrameElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithLoadingAttribute for super::props::HtmlIFrameElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithReferrerPolicyAttribute for super::props::HtmlIFrameElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithNameAttribute for super::props::HtmlIFrameElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithHeightWidthStrAttributes for super::props::HtmlIFrameElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlIFrameElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlIFrameElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlIFrameElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlIFrameElement, Props: HtmlIFrameElement> HtmlIFrameElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlIFrameElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlIFrameElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlIFrameElement { props: self.props.children(children) }
    }
}
pub trait HtmlImageElement:
    HtmlElement
    + ElementWithSrcsetAttribute
    + ElementWithUseMapAttribute
    + ElementWithSizesAttribute
    + ElementWithLoadingAttribute
    + ElementWithAltAttribute
    + ElementWithReferrerPolicyAttribute
    + ElementWithCrossOriginAttribute
    + ElementWithHeightWidthU32Attributes
    + ElementWithSrcAttribute
{
    fn decoding<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlImageElement::attributes::decoding<V>> {
        Self::append_attributes(self, super::attributes::HtmlImageElement::attributes::decoding(value))
    }
    fn element_timing<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlImageElement::attributes::element_timing<V>> {
        Self::append_attributes(self, super::attributes::HtmlImageElement::attributes::element_timing(value))
    }
    fn is_map<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlImageElement::attributes::is_map<V>> {
        Self::append_attributes(self, super::attributes::HtmlImageElement::attributes::is_map(value))
    }
}
impl<C, A, ELS> HtmlImageElement for super::props::HtmlImageElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithSrcsetAttribute for super::props::HtmlImageElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithUseMapAttribute for super::props::HtmlImageElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithSizesAttribute for super::props::HtmlImageElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithLoadingAttribute for super::props::HtmlImageElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithAltAttribute for super::props::HtmlImageElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithReferrerPolicyAttribute for super::props::HtmlImageElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithCrossOriginAttribute for super::props::HtmlImageElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithHeightWidthU32Attributes for super::props::HtmlImageElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithSrcAttribute for super::props::HtmlImageElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlImageElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlImageElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlImageElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlImageElement, Props: HtmlImageElement> HtmlImageElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlImageElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlImageElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlImageElement { props: self.props.children(children) }
    }
}
pub trait HtmlInputElement:
    HtmlElement
    + ElementWithReadOnlyAttribute
    + ElementWithPlaceHolderAttribute
    + ElementWithMaxMinLengthAttributes
    + ElementWithSrcAttribute
    + ElementWithSizeU32Attribute
    + ElementWithRequiredAttribute
    + ElementWithMultipleAttribute
    + ElementWithFormAttributes
    + ElementWithAutoCompleteAttribute
    + ElementWithAutoCorrectAttribute
    + ElementWithAcceptAttribute
    + ElementWithAltAttribute
    + ElementWithDisabledAttribute
    + ElementWithNameAttribute
    + ElementWithHeightWidthU32Attributes
    + ElementWithFormAttribute
{
    fn capture<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlInputElement::attributes::capture<V>> {
        Self::append_attributes(self, super::attributes::HtmlInputElement::attributes::capture(value))
    }
    fn dirname<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlInputElement::attributes::dirname<V>> {
        Self::append_attributes(self, super::attributes::HtmlInputElement::attributes::dirname(value))
    }
    fn list<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlInputElement::attributes::list<V>> {
        Self::append_attributes(self, super::attributes::HtmlInputElement::attributes::list(value))
    }
    fn max<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlInputElement::attributes::max<V>> {
        Self::append_attributes(self, super::attributes::HtmlInputElement::attributes::max(value))
    }
    fn min<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlInputElement::attributes::min<V>> {
        Self::append_attributes(self, super::attributes::HtmlInputElement::attributes::min(value))
    }
    fn pattern<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlInputElement::attributes::pattern<V>> {
        Self::append_attributes(self, super::attributes::HtmlInputElement::attributes::pattern(value))
    }
    fn step<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlInputElement::attributes::step<V>> {
        Self::append_attributes(self, super::attributes::HtmlInputElement::attributes::step(value))
    }
}
impl<C, A, ELS> HtmlInputElement for super::props::HtmlInputElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithReadOnlyAttribute for super::props::HtmlInputElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithPlaceHolderAttribute for super::props::HtmlInputElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithMaxMinLengthAttributes for super::props::HtmlInputElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithSrcAttribute for super::props::HtmlInputElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithSizeU32Attribute for super::props::HtmlInputElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithRequiredAttribute for super::props::HtmlInputElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithMultipleAttribute for super::props::HtmlInputElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithFormAttributes for super::props::HtmlInputElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithAutoCompleteAttribute for super::props::HtmlInputElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithAutoCorrectAttribute for super::props::HtmlInputElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithAcceptAttribute for super::props::HtmlInputElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithAltAttribute for super::props::HtmlInputElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithDisabledAttribute for super::props::HtmlInputElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithNameAttribute for super::props::HtmlInputElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithHeightWidthU32Attributes for super::props::HtmlInputElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithFormAttribute for super::props::HtmlInputElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlInputElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlInputElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlInputElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlInputElement, Props: HtmlInputElement> HtmlInputElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlInputElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlInputElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlInputElement { props: self.props.children(children) }
    }
}
pub trait HtmlLabelElement: HtmlElement + ElementWithForAttribute {}
impl<C, A, ELS> HtmlLabelElement for super::props::HtmlLabelElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithForAttribute for super::props::HtmlLabelElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlLabelElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlLabelElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlLabelElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlLabelElement, Props: HtmlLabelElement> HtmlLabelElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlLabelElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlLabelElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlLabelElement { props: self.props.children(children) }
    }
}
pub trait HtmlLiElement: HtmlElement {
    fn value<V: crate::impl_bounds::MaybeValue::Bounds<i32>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlLiElement::attributes::value<V>> {
        Self::append_attributes(self, super::attributes::HtmlLiElement::attributes::value(value))
    }
}
impl<C, A, ELS> HtmlLiElement for super::props::HtmlLiElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlLiElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlLiElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlLiElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlLiElement, Props: HtmlLiElement> HtmlLiElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlLiElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlLiElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlLiElement { props: self.props.children(children) }
    }
}
pub trait HtmlLinkElement:
    HtmlElement
    + ElementWithHrefAttribute
    + ElementWithTypeAttribute
    + ElementWithMediaAttribute
    + ElementWithBlockingAttribute
    + ElementWithIntegrityAttribute
    + ElementWithSizesAttribute
    + ElementWithHrefLangAttribute
    + ElementWithFetchPriorityAttribute
    + ElementWithReferrerPolicyAttribute
    + ElementWithRelAttribute
    + ElementWithCrossOriginAttribute
{
    fn r#as<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlLinkElement::attributes::r#as<V>> {
        Self::append_attributes(self, super::attributes::HtmlLinkElement::attributes::r#as(value))
    }
    fn as_<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlLinkElement::attributes::r#as<V>> {
        Self::append_attributes(self, super::attributes::HtmlLinkElement::attributes::r#as(value))
    }
    fn image_sizes<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlLinkElement::attributes::image_sizes<V>> {
        Self::append_attributes(self, super::attributes::HtmlLinkElement::attributes::image_sizes(value))
    }
    fn image_src_set<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlLinkElement::attributes::image_src_set<V>> {
        Self::append_attributes(self, super::attributes::HtmlLinkElement::attributes::image_src_set(value))
    }
    fn prefetch<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlLinkElement::attributes::prefetch<V>> {
        Self::append_attributes(self, super::attributes::HtmlLinkElement::attributes::prefetch(value))
    }
}
impl<C, A, ELS> HtmlLinkElement for super::props::HtmlLinkElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithHrefAttribute for super::props::HtmlLinkElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithTypeAttribute for super::props::HtmlLinkElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithMediaAttribute for super::props::HtmlLinkElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithBlockingAttribute for super::props::HtmlLinkElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithIntegrityAttribute for super::props::HtmlLinkElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithSizesAttribute for super::props::HtmlLinkElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithHrefLangAttribute for super::props::HtmlLinkElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithFetchPriorityAttribute for super::props::HtmlLinkElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithReferrerPolicyAttribute for super::props::HtmlLinkElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithRelAttribute for super::props::HtmlLinkElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithCrossOriginAttribute for super::props::HtmlLinkElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlLinkElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlLinkElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlLinkElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlLinkElement, Props: HtmlLinkElement> HtmlLinkElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlLinkElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlLinkElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlLinkElement { props: self.props.children(children) }
    }
}
pub trait HtmlMapElement: HtmlElement + ElementWithNameAttribute {}
impl<C, A, ELS> HtmlMapElement for super::props::HtmlMapElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithNameAttribute for super::props::HtmlMapElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlMapElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlMapElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlMapElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlMapElement, Props: HtmlMapElement> HtmlMapElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlMapElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlMapElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlMapElement { props: self.props.children(children) }
    }
}
pub trait HtmlMetaElement: HtmlElement + ElementWithNameAttribute {
    fn charset<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlMetaElement::attributes::charset<V>> {
        Self::append_attributes(self, super::attributes::HtmlMetaElement::attributes::charset(value))
    }
    fn content<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlMetaElement::attributes::content<V>> {
        Self::append_attributes(self, super::attributes::HtmlMetaElement::attributes::content(value))
    }
    fn http_equiv<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlMetaElement::attributes::http_equiv<V>> {
        Self::append_attributes(self, super::attributes::HtmlMetaElement::attributes::http_equiv(value))
    }
}
impl<C, A, ELS> HtmlMetaElement for super::props::HtmlMetaElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithNameAttribute for super::props::HtmlMetaElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlMetaElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlMetaElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlMetaElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlMetaElement, Props: HtmlMetaElement> HtmlMetaElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlMetaElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlMetaElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlMetaElement { props: self.props.children(children) }
    }
}
pub trait HtmlMeterElement: HtmlElement + ElementWithMaxF64Attribute + ElementWithValueF64Attribute {
    fn min<V: crate::impl_bounds::MaybeValue::Bounds<f64>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlMeterElement::attributes::min<V>> {
        Self::append_attributes(self, super::attributes::HtmlMeterElement::attributes::min(value))
    }
    fn low<V: crate::impl_bounds::MaybeValue::Bounds<f64>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlMeterElement::attributes::low<V>> {
        Self::append_attributes(self, super::attributes::HtmlMeterElement::attributes::low(value))
    }
    fn high<V: crate::impl_bounds::MaybeValue::Bounds<f64>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlMeterElement::attributes::high<V>> {
        Self::append_attributes(self, super::attributes::HtmlMeterElement::attributes::high(value))
    }
    fn optimum<V: crate::impl_bounds::MaybeValue::Bounds<f64>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlMeterElement::attributes::optimum<V>> {
        Self::append_attributes(self, super::attributes::HtmlMeterElement::attributes::optimum(value))
    }
}
impl<C, A, ELS> HtmlMeterElement for super::props::HtmlMeterElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithMaxF64Attribute for super::props::HtmlMeterElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithValueF64Attribute for super::props::HtmlMeterElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlMeterElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlMeterElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlMeterElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlMeterElement, Props: HtmlMeterElement> HtmlMeterElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlMeterElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlMeterElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlMeterElement { props: self.props.children(children) }
    }
}
pub trait HtmlObjectElement: HtmlElement + ElementWithTypeAttribute + ElementWithUseMapAttribute + ElementWithFormAttribute + ElementWithNameAttribute + ElementWithHeightWidthStrAttributes {
    fn data<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlObjectElement::attributes::data<V>> {
        Self::append_attributes(self, super::attributes::HtmlObjectElement::attributes::data(value))
    }
}
impl<C, A, ELS> HtmlObjectElement for super::props::HtmlObjectElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithTypeAttribute for super::props::HtmlObjectElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithUseMapAttribute for super::props::HtmlObjectElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithFormAttribute for super::props::HtmlObjectElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithNameAttribute for super::props::HtmlObjectElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithHeightWidthStrAttributes for super::props::HtmlObjectElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlObjectElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlObjectElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlObjectElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlObjectElement, Props: HtmlObjectElement> HtmlObjectElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlObjectElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlObjectElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlObjectElement { props: self.props.children(children) }
    }
}
pub trait HtmlOListElement: HtmlElement + ElementWithTypeAttribute {
    fn reversed<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlOListElement::attributes::reversed<V>> {
        Self::append_attributes(self, super::attributes::HtmlOListElement::attributes::reversed(value))
    }
    fn start<V: crate::impl_bounds::MaybeValue::Bounds<i32>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlOListElement::attributes::start<V>> {
        Self::append_attributes(self, super::attributes::HtmlOListElement::attributes::start(value))
    }
}
impl<C, A, ELS> HtmlOListElement for super::props::HtmlOListElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithTypeAttribute for super::props::HtmlOListElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlOListElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlOListElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlOListElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlOListElement, Props: HtmlOListElement> HtmlOListElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlOListElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlOListElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlOListElement { props: self.props.children(children) }
    }
}
pub trait HtmlOptGroupElement: HtmlElement + ElementWithLabelAttribute + ElementWithDisabledAttribute {}
impl<C, A, ELS> HtmlOptGroupElement for super::props::HtmlOptGroupElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithLabelAttribute for super::props::HtmlOptGroupElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithDisabledAttribute for super::props::HtmlOptGroupElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlOptGroupElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlOptGroupElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlOptGroupElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlOptGroupElement, Props: HtmlOptGroupElement> HtmlOptGroupElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlOptGroupElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlOptGroupElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlOptGroupElement { props: self.props.children(children) }
    }
}
pub trait HtmlOptionElement: HtmlElement + ElementWithLabelAttribute + ElementWithDisabledAttribute + ElementWithValueStrAttribute {
    fn selected<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlOptionElement::attributes::selected<V>> {
        Self::append_attributes(self, super::attributes::HtmlOptionElement::attributes::selected(value))
    }
}
impl<C, A, ELS> HtmlOptionElement for super::props::HtmlOptionElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithLabelAttribute for super::props::HtmlOptionElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithDisabledAttribute for super::props::HtmlOptionElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithValueStrAttribute for super::props::HtmlOptionElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlOptionElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlOptionElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlOptionElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlOptionElement, Props: HtmlOptionElement> HtmlOptionElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlOptionElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlOptionElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlOptionElement { props: self.props.children(children) }
    }
}
pub trait HtmlOutputElement: HtmlElement + ElementWithForAttribute + ElementWithFormAttribute + ElementWithNameAttribute {}
impl<C, A, ELS> HtmlOutputElement for super::props::HtmlOutputElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithForAttribute for super::props::HtmlOutputElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithFormAttribute for super::props::HtmlOutputElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithNameAttribute for super::props::HtmlOutputElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlOutputElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlOutputElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlOutputElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlOutputElement, Props: HtmlOutputElement> HtmlOutputElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlOutputElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlOutputElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlOutputElement { props: self.props.children(children) }
    }
}
pub trait HtmlProgressElement: HtmlElement + ElementWithMaxF64Attribute + ElementWithValueF64Attribute {}
impl<C, A, ELS> HtmlProgressElement for super::props::HtmlProgressElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithMaxF64Attribute for super::props::HtmlProgressElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithValueF64Attribute for super::props::HtmlProgressElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlProgressElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlProgressElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlProgressElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlProgressElement, Props: HtmlProgressElement> HtmlProgressElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlProgressElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlProgressElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlProgressElement { props: self.props.children(children) }
    }
}
pub trait HtmlScriptElement:
    HtmlElement
    + ElementWithTypeAttribute
    + ElementWithSrcAttribute
    + ElementWithBlockingAttribute
    + ElementWithIntegrityAttribute
    + ElementWithFetchPriorityAttribute
    + ElementWithReferrerPolicyAttribute
    + ElementWithCrossOriginAttribute
{
    fn r#async<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlScriptElement::attributes::r#async<V>> {
        Self::append_attributes(self, super::attributes::HtmlScriptElement::attributes::r#async(value))
    }
    fn defer<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlScriptElement::attributes::defer<V>> {
        Self::append_attributes(self, super::attributes::HtmlScriptElement::attributes::defer(value))
    }
    fn no_module<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlScriptElement::attributes::no_module<V>> {
        Self::append_attributes(self, super::attributes::HtmlScriptElement::attributes::no_module(value))
    }
}
impl<C, A, ELS> HtmlScriptElement for super::props::HtmlScriptElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithTypeAttribute for super::props::HtmlScriptElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithSrcAttribute for super::props::HtmlScriptElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithBlockingAttribute for super::props::HtmlScriptElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithIntegrityAttribute for super::props::HtmlScriptElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithFetchPriorityAttribute for super::props::HtmlScriptElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithReferrerPolicyAttribute for super::props::HtmlScriptElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithCrossOriginAttribute for super::props::HtmlScriptElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlScriptElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlScriptElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlScriptElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlScriptElement, Props: HtmlScriptElement> HtmlScriptElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_dom::script::IntoScriptContent> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlScriptElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlScriptElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlScriptElement { props: self.props.children(children) }
    }
}
pub trait HtmlSelectElement:
    HtmlElement
    + ElementWithSizeU32Attribute
    + ElementWithRequiredAttribute
    + ElementWithMultipleAttribute
    + ElementWithFormAttribute
    + ElementWithAutoCompleteAttribute
    + ElementWithDisabledAttribute
    + ElementWithNameAttribute
{
}
impl<C, A, ELS> HtmlSelectElement for super::props::HtmlSelectElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithSizeU32Attribute for super::props::HtmlSelectElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithRequiredAttribute for super::props::HtmlSelectElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithMultipleAttribute for super::props::HtmlSelectElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithFormAttribute for super::props::HtmlSelectElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithAutoCompleteAttribute for super::props::HtmlSelectElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithDisabledAttribute for super::props::HtmlSelectElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithNameAttribute for super::props::HtmlSelectElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlSelectElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlSelectElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlSelectElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlSelectElement, Props: HtmlSelectElement> HtmlSelectElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlSelectElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlSelectElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlSelectElement { props: self.props.children(children) }
    }
}
pub trait HtmlSlotElement: HtmlElement + ElementWithNameAttribute {}
impl<C, A, ELS> HtmlSlotElement for super::props::HtmlSlotElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithNameAttribute for super::props::HtmlSlotElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlSlotElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlSlotElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlSlotElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlSlotElement, Props: HtmlSlotElement> HtmlSlotElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlSlotElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlSlotElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlSlotElement { props: self.props.children(children) }
    }
}
pub trait HtmlSourceElement:
    HtmlElement + ElementWithTypeAttribute + ElementWithMediaAttribute + ElementWithSrcsetAttribute + ElementWithSizesAttribute + ElementWithHeightWidthU32Attributes + ElementWithSrcAttribute
{
}
impl<C, A, ELS> HtmlSourceElement for super::props::HtmlSourceElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithTypeAttribute for super::props::HtmlSourceElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithMediaAttribute for super::props::HtmlSourceElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithSrcsetAttribute for super::props::HtmlSourceElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithSizesAttribute for super::props::HtmlSourceElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithHeightWidthU32Attributes for super::props::HtmlSourceElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithSrcAttribute for super::props::HtmlSourceElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlSourceElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlSourceElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlSourceElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlSourceElement, Props: HtmlSourceElement> HtmlSourceElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlSourceElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlSourceElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlSourceElement { props: self.props.children(children) }
    }
}
pub trait HtmlStyleElement: HtmlElement + ElementWithTypeAttribute + ElementWithMediaAttribute + ElementWithBlockingAttribute {}
impl<C, A, ELS> HtmlStyleElement for super::props::HtmlStyleElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithTypeAttribute for super::props::HtmlStyleElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithMediaAttribute for super::props::HtmlStyleElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithBlockingAttribute for super::props::HtmlStyleElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlStyleElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlStyleElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlStyleElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlStyleElement, Props: HtmlStyleElement> HtmlStyleElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlStyleElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlStyleElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlStyleElement { props: self.props.children(children) }
    }
}
pub trait HtmlTableElement: HtmlElement + ElementWithAlignAttribute + ElementWithBgColorAttribute {
    #[deprecated]
    fn border<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlTableElement::attributes::border<V>> {
        Self::append_attributes(self, super::attributes::HtmlTableElement::attributes::border(value))
    }
    #[deprecated]
    fn cell_padding<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlTableElement::attributes::cell_padding<V>> {
        Self::append_attributes(self, super::attributes::HtmlTableElement::attributes::cell_padding(value))
    }
    #[deprecated]
    fn cell_spacing<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlTableElement::attributes::cell_spacing<V>> {
        Self::append_attributes(self, super::attributes::HtmlTableElement::attributes::cell_spacing(value))
    }
    #[deprecated]
    fn frame<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlTableElement::attributes::frame<V>> {
        Self::append_attributes(self, super::attributes::HtmlTableElement::attributes::frame(value))
    }
    #[deprecated]
    fn rules<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlTableElement::attributes::rules<V>> {
        Self::append_attributes(self, super::attributes::HtmlTableElement::attributes::rules(value))
    }
    #[deprecated]
    fn summary<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlTableElement::attributes::summary<V>> {
        Self::append_attributes(self, super::attributes::HtmlTableElement::attributes::summary(value))
    }
    #[deprecated]
    fn width<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlTableElement::attributes::width<V>> {
        Self::append_attributes(self, super::attributes::HtmlTableElement::attributes::width(value))
    }
}
impl<C, A, ELS> HtmlTableElement for super::props::HtmlTableElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithAlignAttribute for super::props::HtmlTableElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithBgColorAttribute for super::props::HtmlTableElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlTableElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlTableElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlTableElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlTableElement, Props: HtmlTableElement> HtmlTableElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlTableElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlTableElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlTableElement { props: self.props.children(children) }
    }
}
pub trait HtmlTableChildElement: HtmlElement + ElementWithAlignAttribute + ElementWithBgColorAttribute {
    #[deprecated]
    fn char<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlTableChildElement::attributes::char<V>> {
        Self::append_attributes(self, super::attributes::HtmlTableChildElement::attributes::char(value))
    }
    #[deprecated]
    fn char_off<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlTableChildElement::attributes::char_off<V>> {
        Self::append_attributes(self, super::attributes::HtmlTableChildElement::attributes::char_off(value))
    }
    #[deprecated]
    fn v_align<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlTableChildElement::attributes::v_align<V>> {
        Self::append_attributes(self, super::attributes::HtmlTableChildElement::attributes::v_align(value))
    }
}
impl<C, A, ELS> HtmlTableChildElement for super::props::HtmlTableChildElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithAlignAttribute for super::props::HtmlTableChildElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithBgColorAttribute for super::props::HtmlTableChildElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlTableChildElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlTableChildElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlTableChildElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlTableChildElement, Props: HtmlTableChildElement> HtmlTableChildElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlTableChildElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlTableChildElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlTableChildElement { props: self.props.children(children) }
    }
}
pub trait HtmlTableSectionElement: HtmlElement + HtmlTableChildElement + ElementWithAlignAttribute + ElementWithBgColorAttribute {}
impl<C, A, ELS> HtmlTableSectionElement for super::props::HtmlTableSectionElement<C, A, ELS> {}
impl<C, A, ELS> HtmlTableChildElement for super::props::HtmlTableSectionElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithAlignAttribute for super::props::HtmlTableSectionElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithBgColorAttribute for super::props::HtmlTableSectionElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlTableSectionElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlTableSectionElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlTableSectionElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlTableSectionElement, Props: HtmlTableSectionElement> HtmlTableSectionElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlTableSectionElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlTableSectionElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlTableSectionElement { props: self.props.children(children) }
    }
}
pub trait HtmlTableRowElement: HtmlElement + HtmlTableChildElement + ElementWithAlignAttribute + ElementWithBgColorAttribute {}
impl<C, A, ELS> HtmlTableRowElement for super::props::HtmlTableRowElement<C, A, ELS> {}
impl<C, A, ELS> HtmlTableChildElement for super::props::HtmlTableRowElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithAlignAttribute for super::props::HtmlTableRowElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithBgColorAttribute for super::props::HtmlTableRowElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlTableRowElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlTableRowElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlTableRowElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlTableRowElement, Props: HtmlTableRowElement> HtmlTableRowElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlTableRowElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlTableRowElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlTableRowElement { props: self.props.children(children) }
    }
}
pub trait HtmlTableColElement: HtmlElement + HtmlTableChildElement + ElementWithAlignAttribute + ElementWithBgColorAttribute {
    fn span<V: crate::impl_bounds::MaybeValue::Bounds<u32>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlTableColElement::attributes::span<V>> {
        Self::append_attributes(self, super::attributes::HtmlTableColElement::attributes::span(value))
    }
    #[deprecated]
    fn width<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlTableColElement::attributes::width<V>> {
        Self::append_attributes(self, super::attributes::HtmlTableColElement::attributes::width(value))
    }
}
impl<C, A, ELS> HtmlTableColElement for super::props::HtmlTableColElement<C, A, ELS> {}
impl<C, A, ELS> HtmlTableChildElement for super::props::HtmlTableColElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithAlignAttribute for super::props::HtmlTableColElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithBgColorAttribute for super::props::HtmlTableColElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlTableColElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlTableColElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlTableColElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlTableColElement, Props: HtmlTableColElement> HtmlTableColElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlTableColElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlTableColElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlTableColElement { props: self.props.children(children) }
    }
}
pub trait HtmlTableCellElement: HtmlElement + ElementWithHeightWidthStrAttributes + HtmlTableChildElement + ElementWithAlignAttribute + ElementWithBgColorAttribute {
    fn col_span<V: crate::impl_bounds::MaybeValue::Bounds<u32>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlTableCellElement::attributes::col_span<V>> {
        Self::append_attributes(self, super::attributes::HtmlTableCellElement::attributes::col_span(value))
    }
    fn headers<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlTableCellElement::attributes::headers<V>> {
        Self::append_attributes(self, super::attributes::HtmlTableCellElement::attributes::headers(value))
    }
    fn row_span<V: crate::impl_bounds::MaybeValue::Bounds<u32>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlTableCellElement::attributes::row_span<V>> {
        Self::append_attributes(self, super::attributes::HtmlTableCellElement::attributes::row_span(value))
    }
    #[deprecated = "Do not use this attribute as it is obsolete in the latest standard. Alternatively, you can put the abbreviated description inside the cell and place the long content in the title attribute."]
    fn abbr<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlTableCellElement::attributes::abbr<V>> {
        Self::append_attributes(self, super::attributes::HtmlTableCellElement::attributes::abbr(value))
    }
    #[deprecated]
    fn axis<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlTableCellElement::attributes::axis<V>> {
        Self::append_attributes(self, super::attributes::HtmlTableCellElement::attributes::axis(value))
    }
    #[deprecated]
    fn scope<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlTableCellElement::attributes::scope<V>> {
        Self::append_attributes(self, super::attributes::HtmlTableCellElement::attributes::scope(value))
    }
}
impl<C, A, ELS> HtmlTableCellElement for super::props::HtmlTableCellElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithHeightWidthStrAttributes for super::props::HtmlTableCellElement<C, A, ELS> {}
impl<C, A, ELS> HtmlTableChildElement for super::props::HtmlTableCellElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithAlignAttribute for super::props::HtmlTableCellElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithBgColorAttribute for super::props::HtmlTableCellElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlTableCellElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlTableCellElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlTableCellElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlTableCellElement, Props: HtmlTableCellElement> HtmlTableCellElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlTableCellElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlTableCellElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlTableCellElement { props: self.props.children(children) }
    }
}
pub trait HtmlTextAreaElement:
    HtmlElement
    + ElementWithReadOnlyAttribute
    + ElementWithPlaceHolderAttribute
    + ElementWithMaxMinLengthAttributes
    + ElementWithRequiredAttribute
    + ElementWithFormAttribute
    + ElementWithAutoCompleteAttribute
    + ElementWithAutoCorrectAttribute
    + ElementWithDisabledAttribute
    + ElementWithNameAttribute
{
    fn cols<V: crate::impl_bounds::MaybeValue::Bounds<u32>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlTextAreaElement::attributes::cols<V>> {
        Self::append_attributes(self, super::attributes::HtmlTextAreaElement::attributes::cols(value))
    }
    fn rows<V: crate::impl_bounds::MaybeValue::Bounds<u32>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlTextAreaElement::attributes::rows<V>> {
        Self::append_attributes(self, super::attributes::HtmlTextAreaElement::attributes::rows(value))
    }
    fn wrap<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlTextAreaElement::attributes::wrap<V>> {
        Self::append_attributes(self, super::attributes::HtmlTextAreaElement::attributes::wrap(value))
    }
}
impl<C, A, ELS> HtmlTextAreaElement for super::props::HtmlTextAreaElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithReadOnlyAttribute for super::props::HtmlTextAreaElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithPlaceHolderAttribute for super::props::HtmlTextAreaElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithMaxMinLengthAttributes for super::props::HtmlTextAreaElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithRequiredAttribute for super::props::HtmlTextAreaElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithFormAttribute for super::props::HtmlTextAreaElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithAutoCompleteAttribute for super::props::HtmlTextAreaElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithAutoCorrectAttribute for super::props::HtmlTextAreaElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithDisabledAttribute for super::props::HtmlTextAreaElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithNameAttribute for super::props::HtmlTextAreaElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlTextAreaElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlTextAreaElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlTextAreaElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlTextAreaElement, Props: HtmlTextAreaElement> HtmlTextAreaElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: crate::form_control::value::FormControlValue<str> + frender_html_common::IntoOneStringOrEmpty> crate::props_builder::PropsBuilderWithChildren<C>
    for super::props::HtmlTextAreaElement<crate::Empty, A, ELS>
{
    type WithChildren = super::props::HtmlTextAreaElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlTextAreaElement { props: self.props.children(children) }
    }
}
pub trait HtmlTimeElement: HtmlElement + ElementWithDateTimeAttribute {}
impl<C, A, ELS> HtmlTimeElement for super::props::HtmlTimeElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithDateTimeAttribute for super::props::HtmlTimeElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlTimeElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlTimeElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlTimeElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlTimeElement, Props: HtmlTimeElement> HtmlTimeElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlTimeElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlTimeElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlTimeElement { props: self.props.children(children) }
    }
}
pub trait HtmlTrackElement: HtmlElement + ElementWithSrcAttribute + ElementWithLabelAttribute {
    fn default<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlTrackElement::attributes::default<V>> {
        Self::append_attributes(self, super::attributes::HtmlTrackElement::attributes::default(value))
    }
    fn kind<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlTrackElement::attributes::kind<V>> {
        Self::append_attributes(self, super::attributes::HtmlTrackElement::attributes::kind(value))
    }
    fn src_lang<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlTrackElement::attributes::src_lang<V>> {
        Self::append_attributes(self, super::attributes::HtmlTrackElement::attributes::src_lang(value))
    }
}
impl<C, A, ELS> HtmlTrackElement for super::props::HtmlTrackElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithSrcAttribute for super::props::HtmlTrackElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithLabelAttribute for super::props::HtmlTrackElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlTrackElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlTrackElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlTrackElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlTrackElement, Props: HtmlTrackElement> HtmlTrackElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlTrackElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlTrackElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlTrackElement { props: self.props.children(children) }
    }
}
pub trait HtmlUListElement: HtmlElement + ElementWithTypeAttribute {
    #[deprecated = "Do not use this attribute, as it has been deprecated: use CSS instead. To give a similar effect as the compact attribute, the CSS property line-height can be used with a value of 80%."]
    fn compact<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlUListElement::attributes::compact<V>> {
        Self::append_attributes(self, super::attributes::HtmlUListElement::attributes::compact(value))
    }
}
impl<C, A, ELS> HtmlUListElement for super::props::HtmlUListElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithTypeAttribute for super::props::HtmlUListElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlUListElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlUListElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlUListElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlUListElement, Props: HtmlUListElement> HtmlUListElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlUListElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlUListElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlUListElement { props: self.props.children(children) }
    }
}
pub trait HtmlAudioElement: HtmlMediaElement {}
impl<C, A, ELS> HtmlAudioElement for super::props::HtmlAudioElement<C, A, ELS> {}
impl<C, A, ELS> HtmlMediaElement for super::props::HtmlAudioElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithSrcAttribute for super::props::HtmlAudioElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithCrossOriginAttribute for super::props::HtmlAudioElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlAudioElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlAudioElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlAudioElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlAudioElement, Props: HtmlAudioElement> HtmlAudioElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlAudioElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlAudioElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlAudioElement { props: self.props.children(children) }
    }
}
pub trait HtmlVideoElement: HtmlMediaElement + ElementWithHeightWidthU32Attributes {
    fn plays_inline<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlVideoElement::attributes::plays_inline<V>> {
        Self::append_attributes(self, super::attributes::HtmlVideoElement::attributes::plays_inline(value))
    }
    fn poster<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlVideoElement::attributes::poster<V>> {
        Self::append_attributes(self, super::attributes::HtmlVideoElement::attributes::poster(value))
    }
}
impl<C, A, ELS> HtmlVideoElement for super::props::HtmlVideoElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithHeightWidthU32Attributes for super::props::HtmlVideoElement<C, A, ELS> {}
impl<C, A, ELS> HtmlMediaElement for super::props::HtmlVideoElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithSrcAttribute for super::props::HtmlVideoElement<C, A, ELS> {}
impl<C, A, ELS> ElementWithCrossOriginAttribute for super::props::HtmlVideoElement<C, A, ELS> {}
impl<C, A, ELS> HtmlElement for super::props::HtmlVideoElement<C, A, ELS> {}
impl<C, A, ELS> Element for super::props::HtmlVideoElement<C, A, ELS> {}
impl<C, A, ELS> Node for super::props::HtmlVideoElement<C, A, ELS> {}
impl<Tag: super::behavior_type_traits::HtmlVideoElement, Props: HtmlVideoElement> HtmlVideoElement for crate::dom::component::IntrinsicElement<Tag, Props> {}
impl<A, ELS, C: frender_ssr::SsrElement> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlVideoElement<crate::Empty, A, ELS> {
    type WithChildren = super::props::HtmlVideoElement<C, A, ELS>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlVideoElement { props: self.props.children(children) }
    }
}
