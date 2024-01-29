use super::*;
pub trait Node: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> Node for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::Node::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::Node::Building<(), A> {
    type WithChildren = super::props::Node::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::Node::Building(super::props::Node::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::Node::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::Node::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::Node::Building(super::props::Node::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait Element: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn css<V: Css::Bounds>(self, value: V) -> Self::AppendAttributes<super::attributes::Element::attributes::css<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::css(value))
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
    fn on_cancel<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::Element::attributes::on_cancel<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::on_cancel(value))
    }
    /// Event [`error`](https://developer.mozilla.org/en-US/docs/Web/API/Element/error_event)
    ///
    /// Fired when a resource failed to load, or can't be used. For example, if a script has an execution error or an image can't be found or is invalid.
    fn on_error<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::Element::attributes::on_error<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::on_error(value))
    }
    /// Event [`scroll`](https://developer.mozilla.org/en-US/docs/Web/API/Element/scroll_event)
    ///
    /// Fired when the document view or an element has been scrolled.
    fn on_scroll<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::Element::attributes::on_scroll<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::on_scroll(value))
    }
    /// Event [`securitypolicyviolation`](https://developer.mozilla.org/en-US/docs/Web/API/Element/securitypolicyviolation_event)
    ///
    /// Fired when a [Content Security Policy](https://developer.mozilla.org/en-US/docs/Web/HTTP/CSP) is violated.
    fn on_security_policy_violation<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::SecurityPolicyViolationEvent>>(
        self,
        value: V,
    ) -> Self::AppendAttributes<super::attributes::Element::attributes::on_security_policy_violation<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::on_security_policy_violation(value))
    }
    /// Event [`select`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/select_event)
    ///
    /// Fired when some text has been selected.
    fn on_select<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::Element::attributes::on_select<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::on_select(value))
    }
    /// Event [`wheel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/wheel_event)
    ///
    /// Fired when the user rotates a wheel button on a pointing device (typically a mouse).
    fn on_wheel<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::WheelEvent>>(self, value: V) -> Self::AppendAttributes<super::attributes::Element::attributes::on_wheel<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::on_wheel(value))
    }
    /// Event [`copy`](https://developer.mozilla.org/en-US/docs/Web/API/Element/copy_event)
    ///
    /// Fired when the user initiates a copy action through the browser's user interface.
    fn on_copy<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::Element::attributes::on_copy<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::on_copy(value))
    }
    /// Event [`cut`](https://developer.mozilla.org/en-US/docs/Web/API/Element/cut_event)
    ///
    /// Fired when the user initiates a cut action through the browser's user interface.
    fn on_cut<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::Element::attributes::on_cut<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::on_cut(value))
    }
    /// Event [`paste`](https://developer.mozilla.org/en-US/docs/Web/API/Element/paste_event)
    ///
    /// Fired when the user initiates a paste action through the browser's user interface.
    fn on_paste<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::Element::attributes::on_paste<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::on_paste(value))
    }
    /// Event [`compositionend`](https://developer.mozilla.org/en-US/docs/Web/API/Element/compositionend_event)
    ///
    /// Fired when a text composition system such as an [input method editor](https://developer.mozilla.org/en-US/docs/Glossary/Input_method_editor) completes or cancels the current composition session.
    fn on_composition_end<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::CompositionEvent>>(
        self,
        value: V,
    ) -> Self::AppendAttributes<super::attributes::Element::attributes::on_composition_end<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::on_composition_end(value))
    }
    /// Event [`compositionstart`](https://developer.mozilla.org/en-US/docs/Web/API/Element/compositionstart_event)
    ///
    /// Fired when a text composition system such as an [input method editor](https://developer.mozilla.org/en-US/docs/Glossary/Input_method_editor) starts a new composition session.
    fn on_composition_start<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::CompositionEvent>>(
        self,
        value: V,
    ) -> Self::AppendAttributes<super::attributes::Element::attributes::on_composition_start<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::on_composition_start(value))
    }
    /// Event [`compositionupdate`](https://developer.mozilla.org/en-US/docs/Web/API/Element/compositionupdate_event)
    ///
    /// Fired when a new character is received in the context of a text composition session controlled by a text composition system such as an [input method editor](https://developer.mozilla.org/en-US/docs/Glossary/Input_method_editor).
    fn on_composition_update<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::CompositionEvent>>(
        self,
        value: V,
    ) -> Self::AppendAttributes<super::attributes::Element::attributes::on_composition_update<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::on_composition_update(value))
    }
    /// Event [`blur`](https://developer.mozilla.org/en-US/docs/Web/API/Element/blur_event)
    ///
    /// Fired when an element has lost focus.
    fn on_blur<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::FocusEvent>>(self, value: V) -> Self::AppendAttributes<super::attributes::Element::attributes::on_blur<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::on_blur(value))
    }
    /// Event [`focus`](https://developer.mozilla.org/en-US/docs/Web/API/Element/focus_event)
    ///
    /// Fired when an element has gained focus.
    fn on_focus<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::FocusEvent>>(self, value: V) -> Self::AppendAttributes<super::attributes::Element::attributes::on_focus<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::on_focus(value))
    }
    /// Event [`focusin`](https://developer.mozilla.org/en-US/docs/Web/API/Element/focusin_event)
    ///
    /// Fired when an element has gained focus, after [`focus`](https://developer.mozilla.org/en-US/docs/Web/API/Element/focus_event).
    fn on_focus_in<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::FocusEvent>>(self, value: V) -> Self::AppendAttributes<super::attributes::Element::attributes::on_focus_in<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::on_focus_in(value))
    }
    /// Event [`focusout`](https://developer.mozilla.org/en-US/docs/Web/API/Element/focusout_event)
    ///
    /// Fired when an element has lost focus, after [`blur`](https://developer.mozilla.org/en-US/docs/Web/API/Element/blur_event).
    fn on_focus_out<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::FocusEvent>>(self, value: V) -> Self::AppendAttributes<super::attributes::Element::attributes::on_focus_out<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::on_focus_out(value))
    }
    /// Event [`fullscreenchange`](https://developer.mozilla.org/en-US/docs/Web/API/Element/fullscreenchange_event)
    ///
    /// Sent to an [`Element`](https://developer.mozilla.org/en-US/docs/Web/API/Element) when it transitions into or out of [fullscreen](https://developer.mozilla.org/en-US/docs/Web/API/Fullscreen_API/Guide) mode.
    fn on_fullscreen_change<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::Element::attributes::on_fullscreen_change<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::on_fullscreen_change(value))
    }
    /// Event [`fullscreenerror`](https://developer.mozilla.org/en-US/docs/Web/API/Element/fullscreenerror_event)
    ///
    /// Sent to an `Element` if an error occurs while attempting to switch it into or out of [fullscreen](https://developer.mozilla.org/en-US/docs/Web/API/Fullscreen_API/Guide) mode.
    fn on_fullscreen_error<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::Element::attributes::on_fullscreen_error<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::on_fullscreen_error(value))
    }
    /// Event [`keydown`](https://developer.mozilla.org/en-US/docs/Web/API/Element/keydown_event)
    ///
    /// Fired when a key is pressed.
    fn on_key_down<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::KeyboardEvent>>(self, value: V) -> Self::AppendAttributes<super::attributes::Element::attributes::on_key_down<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::on_key_down(value))
    }
    /// Event [`keyup`](https://developer.mozilla.org/en-US/docs/Web/API/Element/keyup_event)
    ///
    /// Fired when a key is released.
    fn on_key_up<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::KeyboardEvent>>(self, value: V) -> Self::AppendAttributes<super::attributes::Element::attributes::on_key_up<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::on_key_up(value))
    }
    /// Event [`auxclick`](https://developer.mozilla.org/en-US/docs/Web/API/Element/auxclick_event)
    ///
    /// Fired when a non-primary pointing device button (e.g., any mouse button other than the left button) has been pressed and released on an element.
    fn on_aux_click<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::MouseEvent>>(self, value: V) -> Self::AppendAttributes<super::attributes::Element::attributes::on_aux_click<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::on_aux_click(value))
    }
    /// Event [`click`](https://developer.mozilla.org/en-US/docs/Web/API/Element/click_event)
    ///
    /// Fired when a pointing device button (e.g., a mouse's primary button) is pressed and released on a single element.
    fn on_click<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::MouseEvent>>(self, value: V) -> Self::AppendAttributes<super::attributes::Element::attributes::on_click<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::on_click(value))
    }
    /// Event [`contextmenu`](https://developer.mozilla.org/en-US/docs/Web/API/Element/contextmenu_event)
    ///
    /// Fired when the user attempts to open a context menu.
    fn on_context_menu<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::MouseEvent>>(self, value: V) -> Self::AppendAttributes<super::attributes::Element::attributes::on_context_menu<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::on_context_menu(value))
    }
    /// Event [`dblclick`](https://developer.mozilla.org/en-US/docs/Web/API/Element/dblclick_event)
    ///
    /// Fired when a pointing device button (e.g., a mouse's primary button) is clicked twice on a single element.
    fn on_double_click<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::MouseEvent>>(self, value: V) -> Self::AppendAttributes<super::attributes::Element::attributes::on_double_click<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::on_double_click(value))
    }
    /// Event [`mousedown`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mousedown_event)
    ///
    /// Fired when a pointing device button is pressed on an element.
    fn on_mouse_down<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::MouseEvent>>(self, value: V) -> Self::AppendAttributes<super::attributes::Element::attributes::on_mouse_down<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::on_mouse_down(value))
    }
    /// Event [`mouseenter`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseenter_event)
    ///
    /// Fired when a pointing device (usually a mouse) is moved over the element that has the listener attached.
    fn on_mouse_enter<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::MouseEvent>>(self, value: V) -> Self::AppendAttributes<super::attributes::Element::attributes::on_mouse_enter<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::on_mouse_enter(value))
    }
    /// Event [`mouseleave`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseleave_event)
    ///
    /// Fired when the pointer of a pointing device (usually a mouse) is moved out of an element that has the listener attached to it.
    fn on_mouse_leave<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::MouseEvent>>(self, value: V) -> Self::AppendAttributes<super::attributes::Element::attributes::on_mouse_leave<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::on_mouse_leave(value))
    }
    /// Event [`mousemove`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mousemove_event)
    ///
    /// Fired when a pointing device (usually a mouse) is moved while over an element.
    fn on_mouse_move<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::MouseEvent>>(self, value: V) -> Self::AppendAttributes<super::attributes::Element::attributes::on_mouse_move<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::on_mouse_move(value))
    }
    /// Event [`mouseout`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseout_event)
    ///
    /// Fired when a pointing device (usually a mouse) is moved off the element to which the listener is attached or off one of its children.
    fn on_mouse_out<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::MouseEvent>>(self, value: V) -> Self::AppendAttributes<super::attributes::Element::attributes::on_mouse_out<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::on_mouse_out(value))
    }
    /// Event [`mouseover`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseover_event)
    ///
    /// Fired when a pointing device is moved onto the element to which the listener is attached or onto one of its children.
    fn on_mouse_over<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::MouseEvent>>(self, value: V) -> Self::AppendAttributes<super::attributes::Element::attributes::on_mouse_over<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::on_mouse_over(value))
    }
    /// Event [`mouseup`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseup_event)
    ///
    /// Fired when a pointing device button is released on an element.
    fn on_mouse_up<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::MouseEvent>>(self, value: V) -> Self::AppendAttributes<super::attributes::Element::attributes::on_mouse_up<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::on_mouse_up(value))
    }
    /// Event [`touchcancel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/touchcancel_event)
    ///
    /// Fired when one or more touch points have been disrupted in an implementation-specific manner (for example, too many touch points are created).
    fn on_touch_cancel<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::TouchEvent>>(self, value: V) -> Self::AppendAttributes<super::attributes::Element::attributes::on_touch_cancel<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::on_touch_cancel(value))
    }
    /// Event [`touchend`](https://developer.mozilla.org/en-US/docs/Web/API/Element/touchend_event)
    ///
    /// Fired when one or more touch points are removed from the touch surface.
    fn on_touch_end<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::TouchEvent>>(self, value: V) -> Self::AppendAttributes<super::attributes::Element::attributes::on_touch_end<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::on_touch_end(value))
    }
    /// Event [`touchmove`](https://developer.mozilla.org/en-US/docs/Web/API/Element/touchmove_event)
    ///
    /// Fired when one or more touch points are moved along the touch surface.
    fn on_touch_move<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::TouchEvent>>(self, value: V) -> Self::AppendAttributes<super::attributes::Element::attributes::on_touch_move<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::on_touch_move(value))
    }
    /// Event [`touchstart`](https://developer.mozilla.org/en-US/docs/Web/API/Element/touchstart_event)
    ///
    /// Fired when one or more touch points are placed on the touch surface.
    fn on_touch_start<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::TouchEvent>>(self, value: V) -> Self::AppendAttributes<super::attributes::Element::attributes::on_touch_start<V>> {
        Self::append_attributes(self, super::attributes::Element::attributes::on_touch_start(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> Element for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::Element::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::Element::Building<(), A> {
    type WithChildren = super::props::Element::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::Element::Building(super::props::Element::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::Element::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::Element::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::Element::Building(super::props::Element::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithHrefAttribute: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn href<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithHrefAttribute::attributes::href<V>> {
        Self::append_attributes(self, super::attributes::ElementWithHrefAttribute::attributes::href(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithHrefAttribute for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithHrefAttribute::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithHrefAttribute::Building<(), A> {
    type WithChildren = super::props::ElementWithHrefAttribute::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithHrefAttribute::Building(super::props::ElementWithHrefAttribute::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithHrefAttribute::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithHrefAttribute::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithHrefAttribute::Building(super::props::ElementWithHrefAttribute::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithTargetAttribute: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn target<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithTargetAttribute::attributes::target<V>> {
        Self::append_attributes(self, super::attributes::ElementWithTargetAttribute::attributes::target(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithTargetAttribute for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithTargetAttribute::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithTargetAttribute::Building<(), A> {
    type WithChildren = super::props::ElementWithTargetAttribute::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithTargetAttribute::Building(super::props::ElementWithTargetAttribute::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithTargetAttribute::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithTargetAttribute::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithTargetAttribute::Building(super::props::ElementWithTargetAttribute::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithTypeAttribute: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn r#type<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithTypeAttribute::attributes::r#type<V>> {
        Self::append_attributes(self, super::attributes::ElementWithTypeAttribute::attributes::r#type(value))
    }
    fn type_<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithTypeAttribute::attributes::r#type<V>> {
        Self::append_attributes(self, super::attributes::ElementWithTypeAttribute::attributes::r#type(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithTypeAttribute for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithTypeAttribute::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithTypeAttribute::Building<(), A> {
    type WithChildren = super::props::ElementWithTypeAttribute::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithTypeAttribute::Building(super::props::ElementWithTypeAttribute::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithTypeAttribute::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithTypeAttribute::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithTypeAttribute::Building(super::props::ElementWithTypeAttribute::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithCiteAttribute: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn cite<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithCiteAttribute::attributes::cite<V>> {
        Self::append_attributes(self, super::attributes::ElementWithCiteAttribute::attributes::cite(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithCiteAttribute for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithCiteAttribute::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithCiteAttribute::Building<(), A> {
    type WithChildren = super::props::ElementWithCiteAttribute::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithCiteAttribute::Building(super::props::ElementWithCiteAttribute::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithCiteAttribute::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithCiteAttribute::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithCiteAttribute::Building(super::props::ElementWithCiteAttribute::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithPlaceHolderAttribute: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn placeholder<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithPlaceHolderAttribute::attributes::placeholder<V>> {
        Self::append_attributes(self, super::attributes::ElementWithPlaceHolderAttribute::attributes::placeholder(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithPlaceHolderAttribute for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithPlaceHolderAttribute::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithPlaceHolderAttribute::Building<(), A> {
    type WithChildren = super::props::ElementWithPlaceHolderAttribute::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithPlaceHolderAttribute::Building(super::props::ElementWithPlaceHolderAttribute::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithPlaceHolderAttribute::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithPlaceHolderAttribute::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithPlaceHolderAttribute::Building(super::props::ElementWithPlaceHolderAttribute::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithMaxMinLengthAttributes: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn max_length<V: crate::impl_bounds::MaybeValue::Bounds<i32>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithMaxMinLengthAttributes::attributes::max_length<V>> {
        Self::append_attributes(self, super::attributes::ElementWithMaxMinLengthAttributes::attributes::max_length(value))
    }
    fn min_length<V: crate::impl_bounds::MaybeValue::Bounds<i32>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithMaxMinLengthAttributes::attributes::min_length<V>> {
        Self::append_attributes(self, super::attributes::ElementWithMaxMinLengthAttributes::attributes::min_length(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithMaxMinLengthAttributes for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithMaxMinLengthAttributes::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithMaxMinLengthAttributes::Building<(), A> {
    type WithChildren = super::props::ElementWithMaxMinLengthAttributes::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithMaxMinLengthAttributes::Building(super::props::ElementWithMaxMinLengthAttributes::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithMaxMinLengthAttributes::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithMaxMinLengthAttributes::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithMaxMinLengthAttributes::Building(super::props::ElementWithMaxMinLengthAttributes::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithHeightWidthStrAttributes: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn height<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithHeightWidthStrAttributes::attributes::height<V>> {
        Self::append_attributes(self, super::attributes::ElementWithHeightWidthStrAttributes::attributes::height(value))
    }
    fn width<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithHeightWidthStrAttributes::attributes::width<V>> {
        Self::append_attributes(self, super::attributes::ElementWithHeightWidthStrAttributes::attributes::width(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithHeightWidthStrAttributes for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithHeightWidthStrAttributes::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithHeightWidthStrAttributes::Building<(), A> {
    type WithChildren = super::props::ElementWithHeightWidthStrAttributes::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithHeightWidthStrAttributes::Building(super::props::ElementWithHeightWidthStrAttributes::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithHeightWidthStrAttributes::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithHeightWidthStrAttributes::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithHeightWidthStrAttributes::Building(super::props::ElementWithHeightWidthStrAttributes::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithHeightWidthU32Attributes: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn height<V: crate::impl_bounds::MaybeValue::Bounds<u32>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithHeightWidthU32Attributes::attributes::height<V>> {
        Self::append_attributes(self, super::attributes::ElementWithHeightWidthU32Attributes::attributes::height(value))
    }
    fn width<V: crate::impl_bounds::MaybeValue::Bounds<u32>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithHeightWidthU32Attributes::attributes::width<V>> {
        Self::append_attributes(self, super::attributes::ElementWithHeightWidthU32Attributes::attributes::width(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithHeightWidthU32Attributes for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithHeightWidthU32Attributes::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithHeightWidthU32Attributes::Building<(), A> {
    type WithChildren = super::props::ElementWithHeightWidthU32Attributes::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithHeightWidthU32Attributes::Building(super::props::ElementWithHeightWidthU32Attributes::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithHeightWidthU32Attributes::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithHeightWidthU32Attributes::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithHeightWidthU32Attributes::Building(super::props::ElementWithHeightWidthU32Attributes::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithMaxF64Attribute: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn max<V: crate::impl_bounds::MaybeValue::Bounds<f64>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithMaxF64Attribute::attributes::max<V>> {
        Self::append_attributes(self, super::attributes::ElementWithMaxF64Attribute::attributes::max(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithMaxF64Attribute for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithMaxF64Attribute::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithMaxF64Attribute::Building<(), A> {
    type WithChildren = super::props::ElementWithMaxF64Attribute::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithMaxF64Attribute::Building(super::props::ElementWithMaxF64Attribute::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithMaxF64Attribute::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithMaxF64Attribute::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithMaxF64Attribute::Building(super::props::ElementWithMaxF64Attribute::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithValueF64Attribute: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn value<V: crate::impl_bounds::MaybeValue::Bounds<f64>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithValueF64Attribute::attributes::value<V>> {
        Self::append_attributes(self, super::attributes::ElementWithValueF64Attribute::attributes::value(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithValueF64Attribute for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithValueF64Attribute::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithValueF64Attribute::Building<(), A> {
    type WithChildren = super::props::ElementWithValueF64Attribute::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithValueF64Attribute::Building(super::props::ElementWithValueF64Attribute::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithValueF64Attribute::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithValueF64Attribute::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithValueF64Attribute::Building(super::props::ElementWithValueF64Attribute::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithValueStrAttribute: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn value<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithValueStrAttribute::attributes::value<V>> {
        Self::append_attributes(self, super::attributes::ElementWithValueStrAttribute::attributes::value(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithValueStrAttribute for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithValueStrAttribute::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithValueStrAttribute::Building<(), A> {
    type WithChildren = super::props::ElementWithValueStrAttribute::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithValueStrAttribute::Building(super::props::ElementWithValueStrAttribute::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithValueStrAttribute::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithValueStrAttribute::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithValueStrAttribute::Building(super::props::ElementWithValueStrAttribute::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithOpenAttribute: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn open<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithOpenAttribute::attributes::open<V>> {
        Self::append_attributes(self, super::attributes::ElementWithOpenAttribute::attributes::open(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithOpenAttribute for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithOpenAttribute::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithOpenAttribute::Building<(), A> {
    type WithChildren = super::props::ElementWithOpenAttribute::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithOpenAttribute::Building(super::props::ElementWithOpenAttribute::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithOpenAttribute::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithOpenAttribute::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithOpenAttribute::Building(super::props::ElementWithOpenAttribute::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithNameAttribute: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn name<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithNameAttribute::attributes::name<V>> {
        Self::append_attributes(self, super::attributes::ElementWithNameAttribute::attributes::name(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithNameAttribute for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithNameAttribute::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithNameAttribute::Building<(), A> {
    type WithChildren = super::props::ElementWithNameAttribute::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithNameAttribute::Building(super::props::ElementWithNameAttribute::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithNameAttribute::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithNameAttribute::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithNameAttribute::Building(super::props::ElementWithNameAttribute::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithDisabledAttribute: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn disabled<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithDisabledAttribute::attributes::disabled<V>> {
        Self::append_attributes(self, super::attributes::ElementWithDisabledAttribute::attributes::disabled(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithDisabledAttribute for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithDisabledAttribute::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithDisabledAttribute::Building<(), A> {
    type WithChildren = super::props::ElementWithDisabledAttribute::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithDisabledAttribute::Building(super::props::ElementWithDisabledAttribute::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithDisabledAttribute::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithDisabledAttribute::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithDisabledAttribute::Building(super::props::ElementWithDisabledAttribute::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithCrossOriginAttribute: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn cross_origin<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithCrossOriginAttribute::attributes::cross_origin<V>> {
        Self::append_attributes(self, super::attributes::ElementWithCrossOriginAttribute::attributes::cross_origin(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithCrossOriginAttribute for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithCrossOriginAttribute::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithCrossOriginAttribute::Building<(), A> {
    type WithChildren = super::props::ElementWithCrossOriginAttribute::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithCrossOriginAttribute::Building(super::props::ElementWithCrossOriginAttribute::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithCrossOriginAttribute::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithCrossOriginAttribute::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithCrossOriginAttribute::Building(super::props::ElementWithCrossOriginAttribute::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithRelAttribute: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn rel<V: DomTokens::Bounds>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithRelAttribute::attributes::rel<V>> {
        Self::append_attributes(self, super::attributes::ElementWithRelAttribute::attributes::rel(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithRelAttribute for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithRelAttribute::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithRelAttribute::Building<(), A> {
    type WithChildren = super::props::ElementWithRelAttribute::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithRelAttribute::Building(super::props::ElementWithRelAttribute::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithRelAttribute::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithRelAttribute::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithRelAttribute::Building(super::props::ElementWithRelAttribute::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithReferrerPolicyAttribute: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn referrer_policy<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithReferrerPolicyAttribute::attributes::referrer_policy<V>> {
        Self::append_attributes(self, super::attributes::ElementWithReferrerPolicyAttribute::attributes::referrer_policy(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithReferrerPolicyAttribute for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithReferrerPolicyAttribute::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithReferrerPolicyAttribute::Building<(), A> {
    type WithChildren = super::props::ElementWithReferrerPolicyAttribute::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithReferrerPolicyAttribute::Building(super::props::ElementWithReferrerPolicyAttribute::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithReferrerPolicyAttribute::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithReferrerPolicyAttribute::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithReferrerPolicyAttribute::Building(super::props::ElementWithReferrerPolicyAttribute::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithAltAttribute: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn alt<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithAltAttribute::attributes::alt<V>> {
        Self::append_attributes(self, super::attributes::ElementWithAltAttribute::attributes::alt(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithAltAttribute for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithAltAttribute::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithAltAttribute::Building<(), A> {
    type WithChildren = super::props::ElementWithAltAttribute::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithAltAttribute::Building(super::props::ElementWithAltAttribute::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithAltAttribute::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithAltAttribute::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithAltAttribute::Building(super::props::ElementWithAltAttribute::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithLoadingAttribute: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn loading<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithLoadingAttribute::attributes::loading<V>> {
        Self::append_attributes(self, super::attributes::ElementWithLoadingAttribute::attributes::loading(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithLoadingAttribute for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithLoadingAttribute::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithLoadingAttribute::Building<(), A> {
    type WithChildren = super::props::ElementWithLoadingAttribute::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithLoadingAttribute::Building(super::props::ElementWithLoadingAttribute::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithLoadingAttribute::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithLoadingAttribute::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithLoadingAttribute::Building(super::props::ElementWithLoadingAttribute::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithAcceptAttribute: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn accept<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithAcceptAttribute::attributes::accept<V>> {
        Self::append_attributes(self, super::attributes::ElementWithAcceptAttribute::attributes::accept(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithAcceptAttribute for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithAcceptAttribute::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithAcceptAttribute::Building<(), A> {
    type WithChildren = super::props::ElementWithAcceptAttribute::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithAcceptAttribute::Building(super::props::ElementWithAcceptAttribute::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithAcceptAttribute::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithAcceptAttribute::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithAcceptAttribute::Building(super::props::ElementWithAcceptAttribute::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithAutoCompleteAttribute: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn auto_complete<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithAutoCompleteAttribute::attributes::auto_complete<V>> {
        Self::append_attributes(self, super::attributes::ElementWithAutoCompleteAttribute::attributes::auto_complete(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithAutoCompleteAttribute for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithAutoCompleteAttribute::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithAutoCompleteAttribute::Building<(), A> {
    type WithChildren = super::props::ElementWithAutoCompleteAttribute::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithAutoCompleteAttribute::Building(super::props::ElementWithAutoCompleteAttribute::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithAutoCompleteAttribute::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithAutoCompleteAttribute::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithAutoCompleteAttribute::Building(super::props::ElementWithAutoCompleteAttribute::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithFormAttribute: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn form<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithFormAttribute::attributes::form<V>> {
        Self::append_attributes(self, super::attributes::ElementWithFormAttribute::attributes::form(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithFormAttribute for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithFormAttribute::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithFormAttribute::Building<(), A> {
    type WithChildren = super::props::ElementWithFormAttribute::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithFormAttribute::Building(super::props::ElementWithFormAttribute::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithFormAttribute::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithFormAttribute::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithFormAttribute::Building(super::props::ElementWithFormAttribute::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithFormAttributes: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
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
impl<PB: crate::props_builder::PropsBuilder> ElementWithFormAttributes for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithFormAttributes::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithFormAttributes::Building<(), A> {
    type WithChildren = super::props::ElementWithFormAttributes::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithFormAttributes::Building(super::props::ElementWithFormAttributes::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithFormAttributes::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithFormAttributes::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithFormAttributes::Building(super::props::ElementWithFormAttributes::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithFetchPriorityAttribute: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn fetch_priority<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithFetchPriorityAttribute::attributes::fetch_priority<V>> {
        Self::append_attributes(self, super::attributes::ElementWithFetchPriorityAttribute::attributes::fetch_priority(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithFetchPriorityAttribute for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithFetchPriorityAttribute::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithFetchPriorityAttribute::Building<(), A> {
    type WithChildren = super::props::ElementWithFetchPriorityAttribute::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithFetchPriorityAttribute::Building(super::props::ElementWithFetchPriorityAttribute::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithFetchPriorityAttribute::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithFetchPriorityAttribute::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithFetchPriorityAttribute::Building(super::props::ElementWithFetchPriorityAttribute::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithHrefLangAttribute: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn href_lang<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithHrefLangAttribute::attributes::href_lang<V>> {
        Self::append_attributes(self, super::attributes::ElementWithHrefLangAttribute::attributes::href_lang(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithHrefLangAttribute for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithHrefLangAttribute::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithHrefLangAttribute::Building<(), A> {
    type WithChildren = super::props::ElementWithHrefLangAttribute::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithHrefLangAttribute::Building(super::props::ElementWithHrefLangAttribute::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithHrefLangAttribute::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithHrefLangAttribute::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithHrefLangAttribute::Building(super::props::ElementWithHrefLangAttribute::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithSizesAttribute: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn sizes<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithSizesAttribute::attributes::sizes<V>> {
        Self::append_attributes(self, super::attributes::ElementWithSizesAttribute::attributes::sizes(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithSizesAttribute for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithSizesAttribute::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithSizesAttribute::Building<(), A> {
    type WithChildren = super::props::ElementWithSizesAttribute::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithSizesAttribute::Building(super::props::ElementWithSizesAttribute::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithSizesAttribute::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithSizesAttribute::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithSizesAttribute::Building(super::props::ElementWithSizesAttribute::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithUseMapAttribute: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn use_map<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithUseMapAttribute::attributes::use_map<V>> {
        Self::append_attributes(self, super::attributes::ElementWithUseMapAttribute::attributes::use_map(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithUseMapAttribute for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithUseMapAttribute::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithUseMapAttribute::Building<(), A> {
    type WithChildren = super::props::ElementWithUseMapAttribute::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithUseMapAttribute::Building(super::props::ElementWithUseMapAttribute::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithUseMapAttribute::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithUseMapAttribute::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithUseMapAttribute::Building(super::props::ElementWithUseMapAttribute::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithLabelAttribute: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn label<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithLabelAttribute::attributes::label<V>> {
        Self::append_attributes(self, super::attributes::ElementWithLabelAttribute::attributes::label(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithLabelAttribute for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithLabelAttribute::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithLabelAttribute::Building<(), A> {
    type WithChildren = super::props::ElementWithLabelAttribute::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithLabelAttribute::Building(super::props::ElementWithLabelAttribute::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithLabelAttribute::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithLabelAttribute::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithLabelAttribute::Building(super::props::ElementWithLabelAttribute::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithForAttribute: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn r#for<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithForAttribute::attributes::r#for<V>> {
        Self::append_attributes(self, super::attributes::ElementWithForAttribute::attributes::r#for(value))
    }
    fn html_for<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithForAttribute::attributes::r#for<V>> {
        Self::append_attributes(self, super::attributes::ElementWithForAttribute::attributes::r#for(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithForAttribute for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithForAttribute::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithForAttribute::Building<(), A> {
    type WithChildren = super::props::ElementWithForAttribute::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithForAttribute::Building(super::props::ElementWithForAttribute::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithForAttribute::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithForAttribute::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithForAttribute::Building(super::props::ElementWithForAttribute::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithIntegrityAttribute: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn integrity<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithIntegrityAttribute::attributes::integrity<V>> {
        Self::append_attributes(self, super::attributes::ElementWithIntegrityAttribute::attributes::integrity(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithIntegrityAttribute for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithIntegrityAttribute::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithIntegrityAttribute::Building<(), A> {
    type WithChildren = super::props::ElementWithIntegrityAttribute::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithIntegrityAttribute::Building(super::props::ElementWithIntegrityAttribute::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithIntegrityAttribute::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithIntegrityAttribute::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithIntegrityAttribute::Building(super::props::ElementWithIntegrityAttribute::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithBlockingAttribute: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn blocking<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithBlockingAttribute::attributes::blocking<V>> {
        Self::append_attributes(self, super::attributes::ElementWithBlockingAttribute::attributes::blocking(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithBlockingAttribute for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithBlockingAttribute::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithBlockingAttribute::Building<(), A> {
    type WithChildren = super::props::ElementWithBlockingAttribute::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithBlockingAttribute::Building(super::props::ElementWithBlockingAttribute::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithBlockingAttribute::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithBlockingAttribute::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithBlockingAttribute::Building(super::props::ElementWithBlockingAttribute::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithMultipleAttribute: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn multiple<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithMultipleAttribute::attributes::multiple<V>> {
        Self::append_attributes(self, super::attributes::ElementWithMultipleAttribute::attributes::multiple(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithMultipleAttribute for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithMultipleAttribute::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithMultipleAttribute::Building<(), A> {
    type WithChildren = super::props::ElementWithMultipleAttribute::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithMultipleAttribute::Building(super::props::ElementWithMultipleAttribute::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithMultipleAttribute::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithMultipleAttribute::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithMultipleAttribute::Building(super::props::ElementWithMultipleAttribute::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithRequiredAttribute: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn required<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithRequiredAttribute::attributes::required<V>> {
        Self::append_attributes(self, super::attributes::ElementWithRequiredAttribute::attributes::required(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithRequiredAttribute for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithRequiredAttribute::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithRequiredAttribute::Building<(), A> {
    type WithChildren = super::props::ElementWithRequiredAttribute::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithRequiredAttribute::Building(super::props::ElementWithRequiredAttribute::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithRequiredAttribute::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithRequiredAttribute::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithRequiredAttribute::Building(super::props::ElementWithRequiredAttribute::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithSizeU32Attribute: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn size<V: crate::impl_bounds::MaybeValue::Bounds<u32>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithSizeU32Attribute::attributes::size<V>> {
        Self::append_attributes(self, super::attributes::ElementWithSizeU32Attribute::attributes::size(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithSizeU32Attribute for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithSizeU32Attribute::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithSizeU32Attribute::Building<(), A> {
    type WithChildren = super::props::ElementWithSizeU32Attribute::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithSizeU32Attribute::Building(super::props::ElementWithSizeU32Attribute::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithSizeU32Attribute::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithSizeU32Attribute::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithSizeU32Attribute::Building(super::props::ElementWithSizeU32Attribute::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithSrcAttribute: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn src<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithSrcAttribute::attributes::src<V>> {
        Self::append_attributes(self, super::attributes::ElementWithSrcAttribute::attributes::src(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithSrcAttribute for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithSrcAttribute::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithSrcAttribute::Building<(), A> {
    type WithChildren = super::props::ElementWithSrcAttribute::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithSrcAttribute::Building(super::props::ElementWithSrcAttribute::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithSrcAttribute::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithSrcAttribute::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithSrcAttribute::Building(super::props::ElementWithSrcAttribute::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithSrcsetAttribute: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn srcset<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithSrcsetAttribute::attributes::srcset<V>> {
        Self::append_attributes(self, super::attributes::ElementWithSrcsetAttribute::attributes::srcset(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithSrcsetAttribute for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithSrcsetAttribute::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithSrcsetAttribute::Building<(), A> {
    type WithChildren = super::props::ElementWithSrcsetAttribute::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithSrcsetAttribute::Building(super::props::ElementWithSrcsetAttribute::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithSrcsetAttribute::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithSrcsetAttribute::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithSrcsetAttribute::Building(super::props::ElementWithSrcsetAttribute::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithBgColorAttribute: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn bg_color<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithBgColorAttribute::attributes::bg_color<V>> {
        Self::append_attributes(self, super::attributes::ElementWithBgColorAttribute::attributes::bg_color(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithBgColorAttribute for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithBgColorAttribute::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithBgColorAttribute::Building<(), A> {
    type WithChildren = super::props::ElementWithBgColorAttribute::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithBgColorAttribute::Building(super::props::ElementWithBgColorAttribute::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithBgColorAttribute::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithBgColorAttribute::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithBgColorAttribute::Building(super::props::ElementWithBgColorAttribute::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithAlignAttribute: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn align<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithAlignAttribute::attributes::align<V>> {
        Self::append_attributes(self, super::attributes::ElementWithAlignAttribute::attributes::align(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithAlignAttribute for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithAlignAttribute::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithAlignAttribute::Building<(), A> {
    type WithChildren = super::props::ElementWithAlignAttribute::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithAlignAttribute::Building(super::props::ElementWithAlignAttribute::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithAlignAttribute::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithAlignAttribute::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithAlignAttribute::Building(super::props::ElementWithAlignAttribute::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithMediaAttribute: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn media<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithMediaAttribute::attributes::media<V>> {
        Self::append_attributes(self, super::attributes::ElementWithMediaAttribute::attributes::media(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithMediaAttribute for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithMediaAttribute::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithMediaAttribute::Building<(), A> {
    type WithChildren = super::props::ElementWithMediaAttribute::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithMediaAttribute::Building(super::props::ElementWithMediaAttribute::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithMediaAttribute::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithMediaAttribute::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithMediaAttribute::Building(super::props::ElementWithMediaAttribute::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithReadOnlyAttribute: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn read_only<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithReadOnlyAttribute::attributes::read_only<V>> {
        Self::append_attributes(self, super::attributes::ElementWithReadOnlyAttribute::attributes::read_only(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithReadOnlyAttribute for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithReadOnlyAttribute::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithReadOnlyAttribute::Building<(), A> {
    type WithChildren = super::props::ElementWithReadOnlyAttribute::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithReadOnlyAttribute::Building(super::props::ElementWithReadOnlyAttribute::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithReadOnlyAttribute::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithReadOnlyAttribute::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithReadOnlyAttribute::Building(super::props::ElementWithReadOnlyAttribute::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait ElementWithDateTimeAttribute: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn date_time<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::ElementWithDateTimeAttribute::attributes::date_time<V>> {
        Self::append_attributes(self, super::attributes::ElementWithDateTimeAttribute::attributes::date_time(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> ElementWithDateTimeAttribute for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::ElementWithDateTimeAttribute::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::ElementWithDateTimeAttribute::Building<(), A> {
    type WithChildren = super::props::ElementWithDateTimeAttribute::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::ElementWithDateTimeAttribute::Building(super::props::ElementWithDateTimeAttribute::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::ElementWithDateTimeAttribute::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::ElementWithDateTimeAttribute::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::ElementWithDateTimeAttribute::Building(super::props::ElementWithDateTimeAttribute::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn access_key<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::access_key<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::access_key(value))
    }
    fn auto_capitalize<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::auto_capitalize<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::auto_capitalize(value))
    }
    fn auto_focus<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::auto_focus<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::auto_focus(value))
    }
    fn content_editable<V: MaybeContentEditable::Bounds>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::content_editable<V>> {
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
    fn spellcheck<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::spellcheck<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::spellcheck(value))
    }
    fn style<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::style<V>> {
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
    fn on_invalid<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::on_invalid<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::on_invalid(value))
    }
    /// Event [`animationcancel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/animationcancel_event)
    ///
    /// Fired when an animation unexpectedly aborts.
    fn on_animation_cancel<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::AnimationEvent>>(
        self,
        value: V,
    ) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::on_animation_cancel<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::on_animation_cancel(value))
    }
    /// Event [`animationend`](https://developer.mozilla.org/en-US/docs/Web/API/Element/animationend_event)
    ///
    /// Fired when an animation has completed normally.
    fn on_animation_end<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::AnimationEvent>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::on_animation_end<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::on_animation_end(value))
    }
    /// Event [`animationiteration`](https://developer.mozilla.org/en-US/docs/Web/API/Element/animationiteration_event)
    ///
    /// Fired when an animation iteration has completed.
    fn on_animation_iteration<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::AnimationEvent>>(
        self,
        value: V,
    ) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::on_animation_iteration<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::on_animation_iteration(value))
    }
    /// Event [`animationstart`](https://developer.mozilla.org/en-US/docs/Web/API/Element/animationstart_event)
    ///
    /// Fired when an animation starts.
    fn on_animation_start<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::AnimationEvent>>(
        self,
        value: V,
    ) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::on_animation_start<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::on_animation_start(value))
    }
    /// Event [`beforeinput`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/beforeinput_event)
    ///
    /// Fired when the value of an [`<input>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input), [`<select>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select), or [`<textarea>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea) element is about to be modified.
    fn on_before_input<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::InputEvent>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::on_before_input<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::on_before_input(value))
    }
    /// Event [`input`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/input_event)
    ///
    /// Fired when the `value` of an [`<input>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input), [`<select>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select), or [`<textarea>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea) element has been changed.
    fn on_input<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::InputEvent>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::on_input<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::on_input(value))
    }
    /// Event [`change`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/change_event)
    ///
    /// Fired when the `value` of an [`<input>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input), [`<select>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select), or [`<textarea>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea) element has been changed and committed by the user. Unlike the [`input`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/input_event) event, the `change` event is not necessarily fired for each alteration to an element's `value`.
    fn on_change<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::on_change<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::on_change(value))
    }
    /// Event [`gotpointercapture`](https://developer.mozilla.org/en-US/docs/Web/API/Element/gotpointercapture_event)
    ///
    /// Fired when an element captures a pointer using [`setPointerCapture()`](https://developer.mozilla.org/en-US/docs/Web/API/Element/setPointerCapture).
    fn on_got_pointer_capture<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::PointerEvent>>(
        self,
        value: V,
    ) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::on_got_pointer_capture<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::on_got_pointer_capture(value))
    }
    /// Event [`lostpointercapture`](https://developer.mozilla.org/en-US/docs/Web/API/Element/lostpointercapture_event)
    ///
    /// Fired when a [captured pointer](https://developer.mozilla.org/en-US/docs/Web/API/Pointer_events#pointer_capture) is released.
    fn on_lost_pointer_capture<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::PointerEvent>>(
        self,
        value: V,
    ) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::on_lost_pointer_capture<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::on_lost_pointer_capture(value))
    }
    /// Event [`pointercancel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointercancel_event)
    ///
    /// Fired when a pointer event is canceled.
    fn on_pointer_cancel<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::PointerEvent>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::on_pointer_cancel<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::on_pointer_cancel(value))
    }
    /// Event [`pointerdown`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerdown_event)
    ///
    /// Fired when a pointer becomes active.
    fn on_pointer_down<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::PointerEvent>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::on_pointer_down<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::on_pointer_down(value))
    }
    /// Event [`pointerenter`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerenter_event)
    ///
    /// Fired when a pointer is moved into the hit test boundaries of an element or one of its descendants.
    fn on_pointer_enter<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::PointerEvent>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::on_pointer_enter<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::on_pointer_enter(value))
    }
    /// Event [`pointerleave`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerleave_event)
    ///
    /// Fired when a pointer is moved out of the hit test boundaries of an element.
    fn on_pointer_leave<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::PointerEvent>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::on_pointer_leave<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::on_pointer_leave(value))
    }
    /// Event [`pointermove`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointermove_event)
    ///
    /// Fired when a pointer changes coordinates.
    fn on_pointer_move<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::PointerEvent>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::on_pointer_move<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::on_pointer_move(value))
    }
    /// Event [`pointerout`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerout_event)
    ///
    /// Fired when a pointer is moved out of the *hit test* boundaries of an element (among other reasons).
    fn on_pointer_out<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::PointerEvent>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::on_pointer_out<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::on_pointer_out(value))
    }
    /// Event [`pointerover`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerover_event)
    ///
    /// Fired when a pointer is moved into an element's hit test boundaries.
    fn on_pointer_over<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::PointerEvent>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::on_pointer_over<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::on_pointer_over(value))
    }
    /// Event [`pointerup`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerup_event)
    ///
    /// Fired when a pointer is no longer active.
    fn on_pointer_up<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::PointerEvent>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::on_pointer_up<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::on_pointer_up(value))
    }
    /// Event [`transitioncancel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/transitioncancel_event)
    ///
    /// Fired when a [CSS transition](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Transitions/Using_CSS_transitions) is canceled.
    fn on_transition_cancel<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::TransitionEvent>>(
        self,
        value: V,
    ) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::on_transition_cancel<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::on_transition_cancel(value))
    }
    /// Event [`transitionend`](https://developer.mozilla.org/en-US/docs/Web/API/Element/transitionend_event)
    ///
    /// Fired when a [CSS transition](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Transitions/Using_CSS_transitions) has completed.
    fn on_transition_end<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::TransitionEvent>>(
        self,
        value: V,
    ) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::on_transition_end<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::on_transition_end(value))
    }
    /// Event [`transitionrun`](https://developer.mozilla.org/en-US/docs/Web/API/Element/transitionrun_event)
    ///
    /// Fired when a [CSS transition](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Transitions/Using_CSS_transitions) is first created.
    fn on_transition_run<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::TransitionEvent>>(
        self,
        value: V,
    ) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::on_transition_run<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::on_transition_run(value))
    }
    /// Event [`transitionstart`](https://developer.mozilla.org/en-US/docs/Web/API/Element/transitionstart_event)
    ///
    /// Fired when a [CSS transition](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Transitions/Using_CSS_transitions) has actually started.
    fn on_transition_start<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::TransitionEvent>>(
        self,
        value: V,
    ) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::on_transition_start<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::on_transition_start(value))
    }
    /// Event [`drag`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/drag_event)
    ///
    /// This event is fired when an element or text selection is being dragged.
    fn on_drag<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::on_drag<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::on_drag(value))
    }
    /// Event [`dragend`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragend_event)
    ///
    /// This event is fired when a drag operation is being ended (by releasing a mouse button or hitting the escape key).
    fn on_drag_end<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::on_drag_end<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::on_drag_end(value))
    }
    /// Event [`dragenter`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragenter_event)
    ///
    /// This event is fired when a dragged element or text selection enters a valid drop target.
    fn on_drag_enter<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::on_drag_enter<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::on_drag_enter(value))
    }
    /// Event [`dragleave`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragleave_event)
    ///
    /// This event is fired when a dragged element or text selection leaves a valid drop target.
    fn on_drag_leave<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::on_drag_leave<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::on_drag_leave(value))
    }
    /// Event [`dragover`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragover_event)
    ///
    /// This event is fired continuously when an element or text selection is being dragged and the mouse pointer is over a valid drop target (every 50 ms WHEN mouse is not moving ELSE much faster between 5 ms (slow movement) and 1ms (fast movement) approximately. This firing pattern is different than [`mouseover`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseover_event) ).
    fn on_drag_over<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::on_drag_over<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::on_drag_over(value))
    }
    /// Event [`dragstart`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragstart_event)
    ///
    /// This event is fired when the user starts dragging an element or text selection.
    fn on_drag_start<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::on_drag_start<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::on_drag_start(value))
    }
    /// Event [`drop`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/drop_event)
    ///
    /// This event is fired when an element or text selection is dropped on a valid drop target.
    fn on_drop<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElement::attributes::on_drop<V>> {
        Self::append_attributes(self, super::attributes::HtmlElement::attributes::on_drop(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> HtmlElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlElement::Building<(), A> {
    type WithChildren = super::props::HtmlElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlElement::Building(super::props::HtmlElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlElement::Building(super::props::HtmlElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlDataListElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> HtmlDataListElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlDataListElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlDataListElement::Building<(), A> {
    type WithChildren = super::props::HtmlDataListElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlDataListElement::Building(super::props::HtmlDataListElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlDataListElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlDataListElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlDataListElement::Building(super::props::HtmlDataListElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlDivElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> HtmlDivElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlDivElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlDivElement::Building<(), A> {
    type WithChildren = super::props::HtmlDivElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlDivElement::Building(super::props::HtmlDivElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlDivElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlDivElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlDivElement::Building(super::props::HtmlDivElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlDListElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> HtmlDListElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlDListElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlDListElement::Building<(), A> {
    type WithChildren = super::props::HtmlDListElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlDListElement::Building(super::props::HtmlDListElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlDListElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlDListElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlDListElement::Building(super::props::HtmlDListElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlHeadingElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> HtmlHeadingElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlHeadingElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlHeadingElement::Building<(), A> {
    type WithChildren = super::props::HtmlHeadingElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlHeadingElement::Building(super::props::HtmlHeadingElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlHeadingElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlHeadingElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlHeadingElement::Building(super::props::HtmlHeadingElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlHeadElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> HtmlHeadElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlHeadElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlHeadElement::Building<(), A> {
    type WithChildren = super::props::HtmlHeadElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlHeadElement::Building(super::props::HtmlHeadElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlHeadElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlHeadElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlHeadElement::Building(super::props::HtmlHeadElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlHrElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> HtmlHrElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlHrElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlHrElement::Building<(), A> {
    type WithChildren = super::props::HtmlHrElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlHrElement::Building(super::props::HtmlHrElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlHrElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlHrElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlHrElement::Building(super::props::HtmlHrElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlLegendElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> HtmlLegendElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlLegendElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlLegendElement::Building<(), A> {
    type WithChildren = super::props::HtmlLegendElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlLegendElement::Building(super::props::HtmlLegendElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlLegendElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlLegendElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlLegendElement::Building(super::props::HtmlLegendElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlMenuElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> HtmlMenuElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlMenuElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlMenuElement::Building<(), A> {
    type WithChildren = super::props::HtmlMenuElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlMenuElement::Building(super::props::HtmlMenuElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlMenuElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlMenuElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlMenuElement::Building(super::props::HtmlMenuElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlParagraphElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> HtmlParagraphElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlParagraphElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlParagraphElement::Building<(), A> {
    type WithChildren = super::props::HtmlParagraphElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlParagraphElement::Building(super::props::HtmlParagraphElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlParagraphElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlParagraphElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlParagraphElement::Building(super::props::HtmlParagraphElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlPictureElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> HtmlPictureElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlPictureElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlPictureElement::Building<(), A> {
    type WithChildren = super::props::HtmlPictureElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlPictureElement::Building(super::props::HtmlPictureElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlPictureElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlPictureElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlPictureElement::Building(super::props::HtmlPictureElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlPreElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> HtmlPreElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlPreElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlPreElement::Building<(), A> {
    type WithChildren = super::props::HtmlPreElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlPreElement::Building(super::props::HtmlPreElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlPreElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlPreElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlPreElement::Building(super::props::HtmlPreElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlSpanElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> HtmlSpanElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlSpanElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlSpanElement::Building<(), A> {
    type WithChildren = super::props::HtmlSpanElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlSpanElement::Building(super::props::HtmlSpanElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlSpanElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlSpanElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlSpanElement::Building(super::props::HtmlSpanElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlTemplateElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> HtmlTemplateElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlTemplateElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlTemplateElement::Building<(), A> {
    type WithChildren = super::props::HtmlTemplateElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlTemplateElement::Building(super::props::HtmlTemplateElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlTemplateElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlTemplateElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlTemplateElement::Building(super::props::HtmlTemplateElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlTitleElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> HtmlTitleElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlTitleElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlTitleElement::Building<(), A> {
    type WithChildren = super::props::HtmlTitleElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlTitleElement::Building(super::props::HtmlTitleElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlTitleElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlTitleElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlTitleElement::Building(super::props::HtmlTitleElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlElementWithHref: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn download<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElementWithHref::attributes::download<V>> {
        Self::append_attributes(self, super::attributes::HtmlElementWithHref::attributes::download(value))
    }
    fn ping<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlElementWithHref::attributes::ping<V>> {
        Self::append_attributes(self, super::attributes::HtmlElementWithHref::attributes::ping(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> HtmlElementWithHref for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlElementWithHref::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlElementWithHref::Building<(), A> {
    type WithChildren = super::props::HtmlElementWithHref::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlElementWithHref::Building(super::props::HtmlElementWithHref::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlElementWithHref::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlElementWithHref::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlElementWithHref::Building(super::props::HtmlElementWithHref::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlAnchorElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> HtmlAnchorElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlAnchorElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlAnchorElement::Building<(), A> {
    type WithChildren = super::props::HtmlAnchorElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlAnchorElement::Building(super::props::HtmlAnchorElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlAnchorElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlAnchorElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlAnchorElement::Building(super::props::HtmlAnchorElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlAreaElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn coords<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlAreaElement::attributes::coords<V>> {
        Self::append_attributes(self, super::attributes::HtmlAreaElement::attributes::coords(value))
    }
    fn shape<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlAreaElement::attributes::shape<V>> {
        Self::append_attributes(self, super::attributes::HtmlAreaElement::attributes::shape(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> HtmlAreaElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlAreaElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlAreaElement::Building<(), A> {
    type WithChildren = super::props::HtmlAreaElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlAreaElement::Building(super::props::HtmlAreaElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlAreaElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlAreaElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlAreaElement::Building(super::props::HtmlAreaElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlMediaElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
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
    fn on_abort<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlMediaElement::attributes::on_abort<V>> {
        Self::append_attributes(self, super::attributes::HtmlMediaElement::attributes::on_abort(value))
    }
    /// Event [`canplay`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/canplay_event)
    ///
    /// Fired when the user agent can play the media, but estimates that **not** enough data has been loaded to play the media up to its end without having to stop for further buffering of content.
    fn on_can_play<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlMediaElement::attributes::on_can_play<V>> {
        Self::append_attributes(self, super::attributes::HtmlMediaElement::attributes::on_can_play(value))
    }
    /// Event [`canplaythrough`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/canplaythrough_event)
    ///
    /// Fired when the user agent can play the media, and estimates that enough data has been loaded to play the media up to its end without having to stop for further buffering of content.
    fn on_can_play_through<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(
        self,
        value: V,
    ) -> Self::AppendAttributes<super::attributes::HtmlMediaElement::attributes::on_can_play_through<V>> {
        Self::append_attributes(self, super::attributes::HtmlMediaElement::attributes::on_can_play_through(value))
    }
    /// Event [`durationchange`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/durationchange_event)
    ///
    /// Fired when the duration property has been updated.
    fn on_duration_change<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlMediaElement::attributes::on_duration_change<V>> {
        Self::append_attributes(self, super::attributes::HtmlMediaElement::attributes::on_duration_change(value))
    }
    /// Event [`emptied`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/emptied_event)
    ///
    /// Fired when the media has become empty; for example, when the media has already been loaded (or partially loaded), and the [`HTMLMediaElement.load()`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/load) method is called to reload it.
    fn on_emptied<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlMediaElement::attributes::on_emptied<V>> {
        Self::append_attributes(self, super::attributes::HtmlMediaElement::attributes::on_emptied(value))
    }
    /// Event [`ended`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/ended_event)
    ///
    /// Fired when playback stops when end of the media (<audio> or <video>) is reached or because no further data is available.
    fn on_ended<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlMediaElement::attributes::on_ended<V>> {
        Self::append_attributes(self, super::attributes::HtmlMediaElement::attributes::on_ended(value))
    }
    /// Event [`loadeddata`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/loadeddata_event)
    ///
    /// Fired when the first frame of the media has finished loading.
    fn on_loaded_data<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlMediaElement::attributes::on_loaded_data<V>> {
        Self::append_attributes(self, super::attributes::HtmlMediaElement::attributes::on_loaded_data(value))
    }
    /// Event [`loadedmetadata`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/loadedmetadata_event)
    ///
    /// Fired when the metadata has been loaded.
    fn on_loaded_metadata<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlMediaElement::attributes::on_loaded_metadata<V>> {
        Self::append_attributes(self, super::attributes::HtmlMediaElement::attributes::on_loaded_metadata(value))
    }
    /// Event [`loadstart`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/loadstart_event)
    ///
    /// Fired when the browser has started to load a resource.
    fn on_load_start<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlMediaElement::attributes::on_load_start<V>> {
        Self::append_attributes(self, super::attributes::HtmlMediaElement::attributes::on_load_start(value))
    }
    /// Event [`pause`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/pause_event)
    ///
    /// Fired when a request to pause play is handled and the activity has entered its paused state, most commonly occurring when the media's [`HTMLMediaElement.pause()`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/pause) method is called.
    fn on_pause<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlMediaElement::attributes::on_pause<V>> {
        Self::append_attributes(self, super::attributes::HtmlMediaElement::attributes::on_pause(value))
    }
    /// Event [`play`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/play_event)
    ///
    /// Fired when the `paused` property is changed from `true` to `false`, as a result of the [`HTMLMediaElement.play()`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/play) method, or the `autoplay` attribute.
    fn on_play<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlMediaElement::attributes::on_play<V>> {
        Self::append_attributes(self, super::attributes::HtmlMediaElement::attributes::on_play(value))
    }
    /// Event [`playing`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/playing_event)
    ///
    /// Fired when playback is ready to start after having been paused or delayed due to lack of data.
    fn on_playing<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlMediaElement::attributes::on_playing<V>> {
        Self::append_attributes(self, super::attributes::HtmlMediaElement::attributes::on_playing(value))
    }
    /// Event [`progress`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/progress_event)
    ///
    /// Fired periodically as the browser loads a resource.
    fn on_progress<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlMediaElement::attributes::on_progress<V>> {
        Self::append_attributes(self, super::attributes::HtmlMediaElement::attributes::on_progress(value))
    }
    /// Event [`ratechange`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/ratechange_event)
    ///
    /// Fired when the playback rate has changed.
    fn on_rate_change<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlMediaElement::attributes::on_rate_change<V>> {
        Self::append_attributes(self, super::attributes::HtmlMediaElement::attributes::on_rate_change(value))
    }
    /// Event [`resize`]()
    ///
    /// Fired when one or both of the `videoWidth` and `videoHeight` properties have just been updated.
    fn on_resize<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlMediaElement::attributes::on_resize<V>> {
        Self::append_attributes(self, super::attributes::HtmlMediaElement::attributes::on_resize(value))
    }
    /// Event [`seeked`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/seeked_event)
    ///
    /// Fired when a seek operation completes.
    fn on_seeked<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlMediaElement::attributes::on_seeked<V>> {
        Self::append_attributes(self, super::attributes::HtmlMediaElement::attributes::on_seeked(value))
    }
    /// Event [`seeking`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/seeking_event)
    ///
    /// Fired when a seek operation begins.
    fn on_seeking<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlMediaElement::attributes::on_seeking<V>> {
        Self::append_attributes(self, super::attributes::HtmlMediaElement::attributes::on_seeking(value))
    }
    /// Event [`stalled`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/stalled_event)
    ///
    /// Fired when the user agent is trying to fetch media data, but data is unexpectedly not forthcoming.
    fn on_stalled<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlMediaElement::attributes::on_stalled<V>> {
        Self::append_attributes(self, super::attributes::HtmlMediaElement::attributes::on_stalled(value))
    }
    /// Event [`suspend`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/suspend_event)
    ///
    /// Fired when the media data loading has been suspended.
    fn on_suspend<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlMediaElement::attributes::on_suspend<V>> {
        Self::append_attributes(self, super::attributes::HtmlMediaElement::attributes::on_suspend(value))
    }
    /// Event [`timeupdate`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/timeupdate_event)
    ///
    /// Fired when the time indicated by the [`currentTime`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/currentTime) property has been updated.
    fn on_time_update<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlMediaElement::attributes::on_time_update<V>> {
        Self::append_attributes(self, super::attributes::HtmlMediaElement::attributes::on_time_update(value))
    }
    /// Event [`volumechange`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/volumechange_event)
    ///
    /// Fired when the volume has changed.
    fn on_volume_change<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlMediaElement::attributes::on_volume_change<V>> {
        Self::append_attributes(self, super::attributes::HtmlMediaElement::attributes::on_volume_change(value))
    }
    /// Event [`waiting`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/waiting_event)
    ///
    /// Fired when playback has stopped because of a temporary lack of data.
    fn on_waiting<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlMediaElement::attributes::on_waiting<V>> {
        Self::append_attributes(self, super::attributes::HtmlMediaElement::attributes::on_waiting(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> HtmlMediaElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlMediaElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlMediaElement::Building<(), A> {
    type WithChildren = super::props::HtmlMediaElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlMediaElement::Building(super::props::HtmlMediaElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlMediaElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlMediaElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlMediaElement::Building(super::props::HtmlMediaElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlBaseElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> HtmlBaseElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlBaseElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlBaseElement::Building<(), A> {
    type WithChildren = super::props::HtmlBaseElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlBaseElement::Building(super::props::HtmlBaseElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlBaseElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlBaseElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlBaseElement::Building(super::props::HtmlBaseElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlQuoteElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> HtmlQuoteElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlQuoteElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlQuoteElement::Building<(), A> {
    type WithChildren = super::props::HtmlQuoteElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlQuoteElement::Building(super::props::HtmlQuoteElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlQuoteElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlQuoteElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlQuoteElement::Building(super::props::HtmlQuoteElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlBodyElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    #[deprecated = "Use the CSS color property in conjunction with the :active pseudo-class instead."]
    fn alink<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlBodyElement::attributes::alink<V>> {
        Self::append_attributes(self, super::attributes::HtmlBodyElement::attributes::alink(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> HtmlBodyElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlBodyElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlBodyElement::Building<(), A> {
    type WithChildren = super::props::HtmlBodyElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlBodyElement::Building(super::props::HtmlBodyElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlBodyElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlBodyElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlBodyElement::Building(super::props::HtmlBodyElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlBrElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    #[deprecated]
    fn clear<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlBrElement::attributes::clear<V>> {
        Self::append_attributes(self, super::attributes::HtmlBrElement::attributes::clear(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> HtmlBrElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlBrElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlBrElement::Building<(), A> {
    type WithChildren = super::props::HtmlBrElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlBrElement::Building(super::props::HtmlBrElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlBrElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlBrElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlBrElement::Building(super::props::HtmlBrElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlButtonElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> HtmlButtonElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlButtonElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlButtonElement::Building<(), A> {
    type WithChildren = super::props::HtmlButtonElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlButtonElement::Building(super::props::HtmlButtonElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlButtonElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlButtonElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlButtonElement::Building(super::props::HtmlButtonElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlCanvasElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> HtmlCanvasElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlCanvasElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlCanvasElement::Building<(), A> {
    type WithChildren = super::props::HtmlCanvasElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlCanvasElement::Building(super::props::HtmlCanvasElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlCanvasElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlCanvasElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlCanvasElement::Building(super::props::HtmlCanvasElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlTableCaptionElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> HtmlTableCaptionElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlTableCaptionElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlTableCaptionElement::Building<(), A> {
    type WithChildren = super::props::HtmlTableCaptionElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlTableCaptionElement::Building(super::props::HtmlTableCaptionElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlTableCaptionElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlTableCaptionElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlTableCaptionElement::Building(super::props::HtmlTableCaptionElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlDataElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> HtmlDataElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlDataElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlDataElement::Building<(), A> {
    type WithChildren = super::props::HtmlDataElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlDataElement::Building(super::props::HtmlDataElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlDataElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlDataElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlDataElement::Building(super::props::HtmlDataElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlModElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> HtmlModElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlModElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlModElement::Building<(), A> {
    type WithChildren = super::props::HtmlModElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlModElement::Building(super::props::HtmlModElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlModElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlModElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlModElement::Building(super::props::HtmlModElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlDetailsElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> HtmlDetailsElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlDetailsElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlDetailsElement::Building<(), A> {
    type WithChildren = super::props::HtmlDetailsElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlDetailsElement::Building(super::props::HtmlDetailsElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlDetailsElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlDetailsElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlDetailsElement::Building(super::props::HtmlDetailsElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlDialogElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> HtmlDialogElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlDialogElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlDialogElement::Building<(), A> {
    type WithChildren = super::props::HtmlDialogElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlDialogElement::Building(super::props::HtmlDialogElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlDialogElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlDialogElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlDialogElement::Building(super::props::HtmlDialogElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlEmbedElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> HtmlEmbedElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlEmbedElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlEmbedElement::Building<(), A> {
    type WithChildren = super::props::HtmlEmbedElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlEmbedElement::Building(super::props::HtmlEmbedElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlEmbedElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlEmbedElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlEmbedElement::Building(super::props::HtmlEmbedElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlFieldSetElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> HtmlFieldSetElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlFieldSetElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlFieldSetElement::Building<(), A> {
    type WithChildren = super::props::HtmlFieldSetElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlFieldSetElement::Building(super::props::HtmlFieldSetElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlFieldSetElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlFieldSetElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlFieldSetElement::Building(super::props::HtmlFieldSetElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlFormElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
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
    fn on_form_data<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlFormElement::attributes::on_form_data<V>> {
        Self::append_attributes(self, super::attributes::HtmlFormElement::attributes::on_form_data(value))
    }
    /// Event [`reset`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/reset_event)
    ///
    /// The `reset` event fires when a form is reset.
    fn on_reset<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlFormElement::attributes::on_reset<V>> {
        Self::append_attributes(self, super::attributes::HtmlFormElement::attributes::on_reset(value))
    }
    /// Event [`submit`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/submit_event)
    ///
    /// The `submit` event fires when a form is submitted.
    fn on_submit<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlFormElement::attributes::on_submit<V>> {
        Self::append_attributes(self, super::attributes::HtmlFormElement::attributes::on_submit(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> HtmlFormElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlFormElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlFormElement::Building<(), A> {
    type WithChildren = super::props::HtmlFormElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlFormElement::Building(super::props::HtmlFormElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlFormElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlFormElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlFormElement::Building(super::props::HtmlFormElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlHtmlElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn xmlns<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlHtmlElement::attributes::xmlns<V>> {
        Self::append_attributes(self, super::attributes::HtmlHtmlElement::attributes::xmlns(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> HtmlHtmlElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlHtmlElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlHtmlElement::Building<(), A> {
    type WithChildren = super::props::HtmlHtmlElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlHtmlElement::Building(super::props::HtmlHtmlElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlHtmlElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlHtmlElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlHtmlElement::Building(super::props::HtmlHtmlElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlIFrameElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
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
impl<PB: crate::props_builder::PropsBuilder> HtmlIFrameElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlIFrameElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlIFrameElement::Building<(), A> {
    type WithChildren = super::props::HtmlIFrameElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlIFrameElement::Building(super::props::HtmlIFrameElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlIFrameElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlIFrameElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlIFrameElement::Building(super::props::HtmlIFrameElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlImageElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
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
impl<PB: crate::props_builder::PropsBuilder> HtmlImageElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlImageElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlImageElement::Building<(), A> {
    type WithChildren = super::props::HtmlImageElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlImageElement::Building(super::props::HtmlImageElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlImageElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlImageElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlImageElement::Building(super::props::HtmlImageElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlInputElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn capture<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlInputElement::attributes::capture<V>> {
        Self::append_attributes(self, super::attributes::HtmlInputElement::attributes::capture(value))
    }
    fn checked<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlInputElement::attributes::checked<V>> {
        Self::append_attributes(self, super::attributes::HtmlInputElement::attributes::checked(value))
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
impl<PB: crate::props_builder::PropsBuilder> HtmlInputElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlInputElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlInputElement::Building<(), A> {
    type WithChildren = super::props::HtmlInputElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlInputElement::Building(super::props::HtmlInputElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlInputElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlInputElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlInputElement::Building(super::props::HtmlInputElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlLabelElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> HtmlLabelElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlLabelElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlLabelElement::Building<(), A> {
    type WithChildren = super::props::HtmlLabelElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlLabelElement::Building(super::props::HtmlLabelElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlLabelElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlLabelElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlLabelElement::Building(super::props::HtmlLabelElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlLiElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn value<V: crate::impl_bounds::MaybeValue::Bounds<i32>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlLiElement::attributes::value<V>> {
        Self::append_attributes(self, super::attributes::HtmlLiElement::attributes::value(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> HtmlLiElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlLiElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlLiElement::Building<(), A> {
    type WithChildren = super::props::HtmlLiElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlLiElement::Building(super::props::HtmlLiElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlLiElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlLiElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlLiElement::Building(super::props::HtmlLiElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlLinkElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
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
impl<PB: crate::props_builder::PropsBuilder> HtmlLinkElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlLinkElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlLinkElement::Building<(), A> {
    type WithChildren = super::props::HtmlLinkElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlLinkElement::Building(super::props::HtmlLinkElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlLinkElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlLinkElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlLinkElement::Building(super::props::HtmlLinkElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlMapElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> HtmlMapElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlMapElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlMapElement::Building<(), A> {
    type WithChildren = super::props::HtmlMapElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlMapElement::Building(super::props::HtmlMapElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlMapElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlMapElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlMapElement::Building(super::props::HtmlMapElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlMetaElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
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
impl<PB: crate::props_builder::PropsBuilder> HtmlMetaElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlMetaElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlMetaElement::Building<(), A> {
    type WithChildren = super::props::HtmlMetaElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlMetaElement::Building(super::props::HtmlMetaElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlMetaElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlMetaElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlMetaElement::Building(super::props::HtmlMetaElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlMeterElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
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
impl<PB: crate::props_builder::PropsBuilder> HtmlMeterElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlMeterElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlMeterElement::Building<(), A> {
    type WithChildren = super::props::HtmlMeterElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlMeterElement::Building(super::props::HtmlMeterElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlMeterElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlMeterElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlMeterElement::Building(super::props::HtmlMeterElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlObjectElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn data<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlObjectElement::attributes::data<V>> {
        Self::append_attributes(self, super::attributes::HtmlObjectElement::attributes::data(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> HtmlObjectElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlObjectElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlObjectElement::Building<(), A> {
    type WithChildren = super::props::HtmlObjectElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlObjectElement::Building(super::props::HtmlObjectElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlObjectElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlObjectElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlObjectElement::Building(super::props::HtmlObjectElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlOListElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn reversed<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlOListElement::attributes::reversed<V>> {
        Self::append_attributes(self, super::attributes::HtmlOListElement::attributes::reversed(value))
    }
    fn start<V: crate::impl_bounds::MaybeValue::Bounds<i32>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlOListElement::attributes::start<V>> {
        Self::append_attributes(self, super::attributes::HtmlOListElement::attributes::start(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> HtmlOListElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlOListElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlOListElement::Building<(), A> {
    type WithChildren = super::props::HtmlOListElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlOListElement::Building(super::props::HtmlOListElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlOListElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlOListElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlOListElement::Building(super::props::HtmlOListElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlOptGroupElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> HtmlOptGroupElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlOptGroupElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlOptGroupElement::Building<(), A> {
    type WithChildren = super::props::HtmlOptGroupElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlOptGroupElement::Building(super::props::HtmlOptGroupElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlOptGroupElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlOptGroupElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlOptGroupElement::Building(super::props::HtmlOptGroupElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlOptionElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn selected<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlOptionElement::attributes::selected<V>> {
        Self::append_attributes(self, super::attributes::HtmlOptionElement::attributes::selected(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> HtmlOptionElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlOptionElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlOptionElement::Building<(), A> {
    type WithChildren = super::props::HtmlOptionElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlOptionElement::Building(super::props::HtmlOptionElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlOptionElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlOptionElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlOptionElement::Building(super::props::HtmlOptionElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlOutputElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> HtmlOutputElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlOutputElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlOutputElement::Building<(), A> {
    type WithChildren = super::props::HtmlOutputElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlOutputElement::Building(super::props::HtmlOutputElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlOutputElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlOutputElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlOutputElement::Building(super::props::HtmlOutputElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlProgressElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> HtmlProgressElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlProgressElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlProgressElement::Building<(), A> {
    type WithChildren = super::props::HtmlProgressElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlProgressElement::Building(super::props::HtmlProgressElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlProgressElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlProgressElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlProgressElement::Building(super::props::HtmlProgressElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlScriptElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
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
impl<PB: crate::props_builder::PropsBuilder> HtmlScriptElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlScriptElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlScriptElement::Building<(), A> {
    type WithChildren = super::props::HtmlScriptElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlScriptElement::Building(super::props::HtmlScriptElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlScriptElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlScriptElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlScriptElement::Building(super::props::HtmlScriptElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlSelectElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> HtmlSelectElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlSelectElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlSelectElement::Building<(), A> {
    type WithChildren = super::props::HtmlSelectElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlSelectElement::Building(super::props::HtmlSelectElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlSelectElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlSelectElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlSelectElement::Building(super::props::HtmlSelectElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlSlotElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> HtmlSlotElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlSlotElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlSlotElement::Building<(), A> {
    type WithChildren = super::props::HtmlSlotElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlSlotElement::Building(super::props::HtmlSlotElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlSlotElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlSlotElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlSlotElement::Building(super::props::HtmlSlotElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlSourceElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> HtmlSourceElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlSourceElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlSourceElement::Building<(), A> {
    type WithChildren = super::props::HtmlSourceElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlSourceElement::Building(super::props::HtmlSourceElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlSourceElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlSourceElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlSourceElement::Building(super::props::HtmlSourceElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlStyleElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> HtmlStyleElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlStyleElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlStyleElement::Building<(), A> {
    type WithChildren = super::props::HtmlStyleElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlStyleElement::Building(super::props::HtmlStyleElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlStyleElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlStyleElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlStyleElement::Building(super::props::HtmlStyleElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlTableElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
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
impl<PB: crate::props_builder::PropsBuilder> HtmlTableElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlTableElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlTableElement::Building<(), A> {
    type WithChildren = super::props::HtmlTableElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlTableElement::Building(super::props::HtmlTableElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlTableElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlTableElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlTableElement::Building(super::props::HtmlTableElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlTableChildElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
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
impl<PB: crate::props_builder::PropsBuilder> HtmlTableChildElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlTableChildElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlTableChildElement::Building<(), A> {
    type WithChildren = super::props::HtmlTableChildElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlTableChildElement::Building(super::props::HtmlTableChildElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlTableChildElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlTableChildElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlTableChildElement::Building(super::props::HtmlTableChildElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlTableSectionElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> HtmlTableSectionElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlTableSectionElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlTableSectionElement::Building<(), A> {
    type WithChildren = super::props::HtmlTableSectionElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlTableSectionElement::Building(super::props::HtmlTableSectionElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlTableSectionElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlTableSectionElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlTableSectionElement::Building(super::props::HtmlTableSectionElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlTableRowElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> HtmlTableRowElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlTableRowElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlTableRowElement::Building<(), A> {
    type WithChildren = super::props::HtmlTableRowElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlTableRowElement::Building(super::props::HtmlTableRowElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlTableRowElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlTableRowElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlTableRowElement::Building(super::props::HtmlTableRowElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlTableColElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn span<V: crate::impl_bounds::MaybeValue::Bounds<u32>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlTableColElement::attributes::span<V>> {
        Self::append_attributes(self, super::attributes::HtmlTableColElement::attributes::span(value))
    }
    #[deprecated]
    fn width<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlTableColElement::attributes::width<V>> {
        Self::append_attributes(self, super::attributes::HtmlTableColElement::attributes::width(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> HtmlTableColElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlTableColElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlTableColElement::Building<(), A> {
    type WithChildren = super::props::HtmlTableColElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlTableColElement::Building(super::props::HtmlTableColElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlTableColElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlTableColElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlTableColElement::Building(super::props::HtmlTableColElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlTableCellElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
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
impl<PB: crate::props_builder::PropsBuilder> HtmlTableCellElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlTableCellElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlTableCellElement::Building<(), A> {
    type WithChildren = super::props::HtmlTableCellElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlTableCellElement::Building(super::props::HtmlTableCellElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlTableCellElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlTableCellElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlTableCellElement::Building(super::props::HtmlTableCellElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlTextAreaElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn auto_correct<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlTextAreaElement::attributes::auto_correct<V>> {
        Self::append_attributes(self, super::attributes::HtmlTextAreaElement::attributes::auto_correct(value))
    }
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
impl<PB: crate::props_builder::PropsBuilder> HtmlTextAreaElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlTextAreaElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlTextAreaElement::Building<(), A> {
    type WithChildren = super::props::HtmlTextAreaElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlTextAreaElement::Building(super::props::HtmlTextAreaElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlTextAreaElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlTextAreaElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlTextAreaElement::Building(super::props::HtmlTextAreaElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlTimeElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> HtmlTimeElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlTimeElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlTimeElement::Building<(), A> {
    type WithChildren = super::props::HtmlTimeElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlTimeElement::Building(super::props::HtmlTimeElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlTimeElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlTimeElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlTimeElement::Building(super::props::HtmlTimeElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlTrackElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
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
impl<PB: crate::props_builder::PropsBuilder> HtmlTrackElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlTrackElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlTrackElement::Building<(), A> {
    type WithChildren = super::props::HtmlTrackElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlTrackElement::Building(super::props::HtmlTrackElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlTrackElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlTrackElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlTrackElement::Building(super::props::HtmlTrackElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlUListElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    #[deprecated = "Do not use this attribute, as it has been deprecated: use CSS instead. To give a similar effect as the compact attribute, the CSS property line-height can be used with a value of 80%."]
    fn compact<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlUListElement::attributes::compact<V>> {
        Self::append_attributes(self, super::attributes::HtmlUListElement::attributes::compact(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> HtmlUListElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlUListElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlUListElement::Building<(), A> {
    type WithChildren = super::props::HtmlUListElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlUListElement::Building(super::props::HtmlUListElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlUListElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlUListElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlUListElement::Building(super::props::HtmlUListElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlAudioElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<PB: crate::props_builder::PropsBuilder> HtmlAudioElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlAudioElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlAudioElement::Building<(), A> {
    type WithChildren = super::props::HtmlAudioElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlAudioElement::Building(super::props::HtmlAudioElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlAudioElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlAudioElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlAudioElement::Building(super::props::HtmlAudioElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
pub trait HtmlVideoElement: crate::props_builder::PropsBuilder + crate::props_builder::PropsBuilderAppendAnySupportedAttributes {
    fn plays_inline<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlVideoElement::attributes::plays_inline<V>> {
        Self::append_attributes(self, super::attributes::HtmlVideoElement::attributes::plays_inline(value))
    }
    fn poster<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> Self::AppendAttributes<super::attributes::HtmlVideoElement::attributes::poster<V>> {
        Self::append_attributes(self, super::attributes::HtmlVideoElement::attributes::poster(value))
    }
}
impl<PB: crate::props_builder::PropsBuilder> HtmlVideoElement for PB where PB: crate::props_builder::PropsBuilderAppendAnySupportedAttributes {}
impl<C, A> crate::props_builder::PropsBuilder for super::props::HtmlVideoElement::Building<C, A> {
    type Attributes = A;
    type Children = C;
}
impl<A, C> crate::props_builder::PropsBuilderWithChildren<C> for super::props::HtmlVideoElement::Building<(), A> {
    type WithChildren = super::props::HtmlVideoElement::Building<C, A>;
    fn children(self, children: C) -> Self::WithChildren {
        super::props::HtmlVideoElement::Building(super::props::HtmlVideoElement::Data { props: self.0.props.children(children) })
    }
}
impl<Children, Attributes> crate::props_builder::PropsBuilderAppendAnySupportedAttributes for super::props::HtmlVideoElement::Building<Children, Attributes> {
    type AppendAttributes<A> = super::props::HtmlVideoElement::Building<Children, (Attributes, A)>;
    fn append_attributes<A>(this: Self, attributes: A) -> Self::AppendAttributes<A> {
        super::props::HtmlVideoElement::Building(super::props::HtmlVideoElement::Data {
            props: this.0.props.chain_prop(attributes),
        })
    }
}
