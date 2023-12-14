use super::super::props::HtmlQuoteElement::*;
mod props_builder {
    #[allow(unused_imports)]
    use super::super::super::*;
    impl<Attrs> super::Building<(), Attrs> {
        pub fn children<Children: Sized>(self, value: Children) -> super::Building<Children, Attrs> {
            super::Building(super::Data { props: self.0.props.children(value) })
        }
    }
    impl<Children, Attrs> super::Building<Children, Attrs> {
        pub fn access_key<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::access_key<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::access_key(value)),
            })
        }
        pub fn auto_capitalize<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::auto_capitalize<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::auto_capitalize(value)),
            })
        }
        pub fn auto_focus<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::auto_focus<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::auto_focus(value)),
            })
        }
        pub fn content_editable<V: MaybeContentEditable::Bounds>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::content_editable<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::content_editable(value)),
            })
        }
        #[deprecated = "See https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/contextMenu"]
        pub fn context_menu<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::context_menu<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::context_menu(value)),
            })
        }
        pub fn dir<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::dir<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::dir(value)),
            })
        }
        pub fn draggable<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::draggable<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::draggable(value)),
            })
        }
        pub fn enter_key_hint<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::enter_key_hint<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::enter_key_hint(value)),
            })
        }
        pub fn hidden<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::hidden<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::hidden(value)),
            })
        }
        pub fn inert<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::inert<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::inert(value)),
            })
        }
        pub fn input_mode<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::input_mode<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::input_mode(value)),
            })
        }
        pub fn is<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::is<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::is(value)),
            })
        }
        pub fn item_id<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::item_id<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::item_id(value)),
            })
        }
        pub fn item_prop<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::item_prop<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::item_prop(value)),
            })
        }
        pub fn item_ref<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::item_ref<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::item_ref(value)),
            })
        }
        pub fn item_scope<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::item_scope<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::item_scope(value)),
            })
        }
        pub fn item_type<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::item_type<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::item_type(value)),
            })
        }
        pub fn lang<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::lang<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::lang(value)),
            })
        }
        pub fn nonce<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::nonce<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::nonce(value)),
            })
        }
        pub fn role<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::role<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::role(value)),
            })
        }
        pub fn slot<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::slot<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::slot(value)),
            })
        }
        pub fn spellcheck<V: crate::impl_bounds::MaybeValue::Bounds<bool>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::spellcheck<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::spellcheck(value)),
            })
        }
        pub fn style<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::style<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::style(value)),
            })
        }
        pub fn tab_index<V: crate::impl_bounds::MaybeValue::Bounds<i32>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::tab_index<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::tab_index(value)),
            })
        }
        pub fn title<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::title<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::title(value)),
            })
        }
        pub fn translate<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::translate<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::translate(value)),
            })
        }
        pub fn virtual_keyboard_policy<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::virtual_keyboard_policy<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::virtual_keyboard_policy(value)),
            })
        }
        /// Event [`invalid`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/invalid_event)
        ///
        /// Fired when an element does not satisfy its constraints during constraint validation.
        pub fn on_invalid<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_invalid<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_invalid(value)),
            })
        }
        /// Event [`animationcancel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/animationcancel_event)
        ///
        /// Fired when an animation unexpectedly aborts.
        pub fn on_animation_cancel<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::AnimationEvent>>(
            self,
            value: V,
        ) -> super::Building<Children, (Attrs, super::attributes::on_animation_cancel<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_animation_cancel(value)),
            })
        }
        /// Event [`animationend`](https://developer.mozilla.org/en-US/docs/Web/API/Element/animationend_event)
        ///
        /// Fired when an animation has completed normally.
        pub fn on_animation_end<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::AnimationEvent>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_animation_end<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_animation_end(value)),
            })
        }
        /// Event [`animationiteration`](https://developer.mozilla.org/en-US/docs/Web/API/Element/animationiteration_event)
        ///
        /// Fired when an animation iteration has completed.
        pub fn on_animation_iteration<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::AnimationEvent>>(
            self,
            value: V,
        ) -> super::Building<Children, (Attrs, super::attributes::on_animation_iteration<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_animation_iteration(value)),
            })
        }
        /// Event [`animationstart`](https://developer.mozilla.org/en-US/docs/Web/API/Element/animationstart_event)
        ///
        /// Fired when an animation starts.
        pub fn on_animation_start<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::AnimationEvent>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_animation_start<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_animation_start(value)),
            })
        }
        /// Event [`beforeinput`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/beforeinput_event)
        ///
        /// Fired when the value of an [`<input>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input), [`<select>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select), or [`<textarea>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea) element is about to be modified.
        pub fn on_before_input<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::InputEvent>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_before_input<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_before_input(value)),
            })
        }
        /// Event [`input`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/input_event)
        ///
        /// Fired when the `value` of an [`<input>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input), [`<select>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select), or [`<textarea>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea) element has been changed.
        pub fn on_input<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::InputEvent>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_input<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_input(value)),
            })
        }
        /// Event [`change`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/change_event)
        ///
        /// Fired when the `value` of an [`<input>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input), [`<select>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select), or [`<textarea>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea) element has been changed and committed by the user. Unlike the [`input`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/input_event) event, the `change` event is not necessarily fired for each alteration to an element's `value`.
        pub fn on_change<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_change<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_change(value)),
            })
        }
        /// Event [`gotpointercapture`](https://developer.mozilla.org/en-US/docs/Web/API/Element/gotpointercapture_event)
        ///
        /// Fired when an element captures a pointer using [`setPointerCapture()`](https://developer.mozilla.org/en-US/docs/Web/API/Element/setPointerCapture).
        pub fn on_got_pointer_capture<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::PointerEvent>>(
            self,
            value: V,
        ) -> super::Building<Children, (Attrs, super::attributes::on_got_pointer_capture<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_got_pointer_capture(value)),
            })
        }
        /// Event [`lostpointercapture`](https://developer.mozilla.org/en-US/docs/Web/API/Element/lostpointercapture_event)
        ///
        /// Fired when a [captured pointer](https://developer.mozilla.org/en-US/docs/Web/API/Pointer_events#pointer_capture) is released.
        pub fn on_lost_pointer_capture<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::PointerEvent>>(
            self,
            value: V,
        ) -> super::Building<Children, (Attrs, super::attributes::on_lost_pointer_capture<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_lost_pointer_capture(value)),
            })
        }
        /// Event [`pointercancel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointercancel_event)
        ///
        /// Fired when a pointer event is canceled.
        pub fn on_pointer_cancel<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::PointerEvent>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_pointer_cancel<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_pointer_cancel(value)),
            })
        }
        /// Event [`pointerdown`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerdown_event)
        ///
        /// Fired when a pointer becomes active.
        pub fn on_pointer_down<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::PointerEvent>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_pointer_down<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_pointer_down(value)),
            })
        }
        /// Event [`pointerenter`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerenter_event)
        ///
        /// Fired when a pointer is moved into the hit test boundaries of an element or one of its descendants.
        pub fn on_pointer_enter<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::PointerEvent>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_pointer_enter<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_pointer_enter(value)),
            })
        }
        /// Event [`pointerleave`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerleave_event)
        ///
        /// Fired when a pointer is moved out of the hit test boundaries of an element.
        pub fn on_pointer_leave<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::PointerEvent>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_pointer_leave<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_pointer_leave(value)),
            })
        }
        /// Event [`pointermove`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointermove_event)
        ///
        /// Fired when a pointer changes coordinates.
        pub fn on_pointer_move<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::PointerEvent>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_pointer_move<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_pointer_move(value)),
            })
        }
        /// Event [`pointerout`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerout_event)
        ///
        /// Fired when a pointer is moved out of the *hit test* boundaries of an element (among other reasons).
        pub fn on_pointer_out<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::PointerEvent>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_pointer_out<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_pointer_out(value)),
            })
        }
        /// Event [`pointerover`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerover_event)
        ///
        /// Fired when a pointer is moved into an element's hit test boundaries.
        pub fn on_pointer_over<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::PointerEvent>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_pointer_over<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_pointer_over(value)),
            })
        }
        /// Event [`pointerup`](https://developer.mozilla.org/en-US/docs/Web/API/Element/pointerup_event)
        ///
        /// Fired when a pointer is no longer active.
        pub fn on_pointer_up<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::PointerEvent>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_pointer_up<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_pointer_up(value)),
            })
        }
        /// Event [`transitioncancel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/transitioncancel_event)
        ///
        /// Fired when a [CSS transition](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Transitions/Using_CSS_transitions) is canceled.
        pub fn on_transition_cancel<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::TransitionEvent>>(
            self,
            value: V,
        ) -> super::Building<Children, (Attrs, super::attributes::on_transition_cancel<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_transition_cancel(value)),
            })
        }
        /// Event [`transitionend`](https://developer.mozilla.org/en-US/docs/Web/API/Element/transitionend_event)
        ///
        /// Fired when a [CSS transition](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Transitions/Using_CSS_transitions) has completed.
        pub fn on_transition_end<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::TransitionEvent>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_transition_end<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_transition_end(value)),
            })
        }
        /// Event [`transitionrun`](https://developer.mozilla.org/en-US/docs/Web/API/Element/transitionrun_event)
        ///
        /// Fired when a [CSS transition](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Transitions/Using_CSS_transitions) is first created.
        pub fn on_transition_run<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::TransitionEvent>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_transition_run<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_transition_run(value)),
            })
        }
        /// Event [`transitionstart`](https://developer.mozilla.org/en-US/docs/Web/API/Element/transitionstart_event)
        ///
        /// Fired when a [CSS transition](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Transitions/Using_CSS_transitions) has actually started.
        pub fn on_transition_start<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::TransitionEvent>>(
            self,
            value: V,
        ) -> super::Building<Children, (Attrs, super::attributes::on_transition_start<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_transition_start(value)),
            })
        }
        /// Event [`drag`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/drag_event)
        ///
        /// This event is fired when an element or text selection is being dragged.
        pub fn on_drag<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_drag<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_drag(value)),
            })
        }
        /// Event [`dragend`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragend_event)
        ///
        /// This event is fired when a drag operation is being ended (by releasing a mouse button or hitting the escape key).
        pub fn on_drag_end<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_drag_end<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_drag_end(value)),
            })
        }
        /// Event [`dragenter`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragenter_event)
        ///
        /// This event is fired when a dragged element or text selection enters a valid drop target.
        pub fn on_drag_enter<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_drag_enter<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_drag_enter(value)),
            })
        }
        /// Event [`dragleave`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragleave_event)
        ///
        /// This event is fired when a dragged element or text selection leaves a valid drop target.
        pub fn on_drag_leave<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_drag_leave<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_drag_leave(value)),
            })
        }
        /// Event [`dragover`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragover_event)
        ///
        /// This event is fired continuously when an element or text selection is being dragged and the mouse pointer is over a valid drop target (every 50 ms WHEN mouse is not moving ELSE much faster between 5 ms (slow movement) and 1ms (fast movement) approximately. This firing pattern is different than [`mouseover`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseover_event) ).
        pub fn on_drag_over<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_drag_over<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_drag_over(value)),
            })
        }
        /// Event [`dragstart`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragstart_event)
        ///
        /// This event is fired when the user starts dragging an element or text selection.
        pub fn on_drag_start<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_drag_start<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_drag_start(value)),
            })
        }
        /// Event [`drop`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/drop_event)
        ///
        /// This event is fired when an element or text selection is dropped on a valid drop target.
        pub fn on_drop<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_drop<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_drop(value)),
            })
        }
        pub fn css<V: Css::Bounds>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::css<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::css(value)),
            })
        }
        pub fn class<V: DomTokens::Bounds>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::class<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::class(value)),
            })
        }
        pub fn id<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::id<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::id(value)),
            })
        }
        pub fn part<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::part<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::part(value)),
            })
        }
        /// Event [`cancel`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/cancel_event)
        ///
        /// Fires on a [`<dialog>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dialog) when the user instructs the browser that they wish to dismiss the currently open modal dialog. The browser fires this event when the user presses the <kbd>Esc</kbd> key to close the modal dialog.
        pub fn on_cancel<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_cancel<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_cancel(value)),
            })
        }
        /// Event [`error`](https://developer.mozilla.org/en-US/docs/Web/API/Element/error_event)
        ///
        /// Fired when a resource failed to load, or can't be used. For example, if a script has an execution error or an image can't be found or is invalid.
        pub fn on_error<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_error<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_error(value)),
            })
        }
        /// Event [`scroll`](https://developer.mozilla.org/en-US/docs/Web/API/Element/scroll_event)
        ///
        /// Fired when the document view or an element has been scrolled.
        pub fn on_scroll<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_scroll<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_scroll(value)),
            })
        }
        /// Event [`securitypolicyviolation`](https://developer.mozilla.org/en-US/docs/Web/API/Element/securitypolicyviolation_event)
        ///
        /// Fired when a [Content Security Policy](https://developer.mozilla.org/en-US/docs/Web/HTTP/CSP) is violated.
        pub fn on_security_policy_violation<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::SecurityPolicyViolationEvent>>(
            self,
            value: V,
        ) -> super::Building<Children, (Attrs, super::attributes::on_security_policy_violation<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_security_policy_violation(value)),
            })
        }
        /// Event [`select`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/select_event)
        ///
        /// Fired when some text has been selected.
        pub fn on_select<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_select<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_select(value)),
            })
        }
        /// Event [`wheel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/wheel_event)
        ///
        /// Fired when the user rotates a wheel button on a pointing device (typically a mouse).
        pub fn on_wheel<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::WheelEvent>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_wheel<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_wheel(value)),
            })
        }
        /// Event [`copy`](https://developer.mozilla.org/en-US/docs/Web/API/Element/copy_event)
        ///
        /// Fired when the user initiates a copy action through the browser's user interface.
        pub fn on_copy<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_copy<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_copy(value)),
            })
        }
        /// Event [`cut`](https://developer.mozilla.org/en-US/docs/Web/API/Element/cut_event)
        ///
        /// Fired when the user initiates a cut action through the browser's user interface.
        pub fn on_cut<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_cut<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_cut(value)),
            })
        }
        /// Event [`paste`](https://developer.mozilla.org/en-US/docs/Web/API/Element/paste_event)
        ///
        /// Fired when the user initiates a paste action through the browser's user interface.
        pub fn on_paste<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_paste<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_paste(value)),
            })
        }
        /// Event [`compositionend`](https://developer.mozilla.org/en-US/docs/Web/API/Element/compositionend_event)
        ///
        /// Fired when a text composition system such as an [input method editor](https://developer.mozilla.org/en-US/docs/Glossary/Input_method_editor) completes or cancels the current composition session.
        pub fn on_composition_end<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::CompositionEvent>>(
            self,
            value: V,
        ) -> super::Building<Children, (Attrs, super::attributes::on_composition_end<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_composition_end(value)),
            })
        }
        /// Event [`compositionstart`](https://developer.mozilla.org/en-US/docs/Web/API/Element/compositionstart_event)
        ///
        /// Fired when a text composition system such as an [input method editor](https://developer.mozilla.org/en-US/docs/Glossary/Input_method_editor) starts a new composition session.
        pub fn on_composition_start<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::CompositionEvent>>(
            self,
            value: V,
        ) -> super::Building<Children, (Attrs, super::attributes::on_composition_start<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_composition_start(value)),
            })
        }
        /// Event [`compositionupdate`](https://developer.mozilla.org/en-US/docs/Web/API/Element/compositionupdate_event)
        ///
        /// Fired when a new character is received in the context of a text composition session controlled by a text composition system such as an [input method editor](https://developer.mozilla.org/en-US/docs/Glossary/Input_method_editor).
        pub fn on_composition_update<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::CompositionEvent>>(
            self,
            value: V,
        ) -> super::Building<Children, (Attrs, super::attributes::on_composition_update<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_composition_update(value)),
            })
        }
        /// Event [`blur`](https://developer.mozilla.org/en-US/docs/Web/API/Element/blur_event)
        ///
        /// Fired when an element has lost focus.
        pub fn on_blur<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::FocusEvent>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_blur<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_blur(value)),
            })
        }
        /// Event [`focus`](https://developer.mozilla.org/en-US/docs/Web/API/Element/focus_event)
        ///
        /// Fired when an element has gained focus.
        pub fn on_focus<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::FocusEvent>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_focus<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_focus(value)),
            })
        }
        /// Event [`focusin`](https://developer.mozilla.org/en-US/docs/Web/API/Element/focusin_event)
        ///
        /// Fired when an element has gained focus, after [`focus`](https://developer.mozilla.org/en-US/docs/Web/API/Element/focus_event).
        pub fn on_focus_in<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::FocusEvent>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_focus_in<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_focus_in(value)),
            })
        }
        /// Event [`focusout`](https://developer.mozilla.org/en-US/docs/Web/API/Element/focusout_event)
        ///
        /// Fired when an element has lost focus, after [`blur`](https://developer.mozilla.org/en-US/docs/Web/API/Element/blur_event).
        pub fn on_focus_out<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::FocusEvent>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_focus_out<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_focus_out(value)),
            })
        }
        /// Event [`fullscreenchange`](https://developer.mozilla.org/en-US/docs/Web/API/Element/fullscreenchange_event)
        ///
        /// Sent to an [`Element`](https://developer.mozilla.org/en-US/docs/Web/API/Element) when it transitions into or out of [fullscreen](https://developer.mozilla.org/en-US/docs/Web/API/Fullscreen_API/Guide) mode.
        pub fn on_fullscreen_change<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_fullscreen_change<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_fullscreen_change(value)),
            })
        }
        /// Event [`fullscreenerror`](https://developer.mozilla.org/en-US/docs/Web/API/Element/fullscreenerror_event)
        ///
        /// Sent to an `Element` if an error occurs while attempting to switch it into or out of [fullscreen](https://developer.mozilla.org/en-US/docs/Web/API/Fullscreen_API/Guide) mode.
        pub fn on_fullscreen_error<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::Event>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_fullscreen_error<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_fullscreen_error(value)),
            })
        }
        /// Event [`keydown`](https://developer.mozilla.org/en-US/docs/Web/API/Element/keydown_event)
        ///
        /// Fired when a key is pressed.
        pub fn on_key_down<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::KeyboardEvent>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_key_down<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_key_down(value)),
            })
        }
        /// Event [`keyup`](https://developer.mozilla.org/en-US/docs/Web/API/Element/keyup_event)
        ///
        /// Fired when a key is released.
        pub fn on_key_up<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::KeyboardEvent>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_key_up<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_key_up(value)),
            })
        }
        /// Event [`auxclick`](https://developer.mozilla.org/en-US/docs/Web/API/Element/auxclick_event)
        ///
        /// Fired when a non-primary pointing device button (e.g., any mouse button other than the left button) has been pressed and released on an element.
        pub fn on_aux_click<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::MouseEvent>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_aux_click<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_aux_click(value)),
            })
        }
        /// Event [`click`](https://developer.mozilla.org/en-US/docs/Web/API/Element/click_event)
        ///
        /// Fired when a pointing device button (e.g., a mouse's primary button) is pressed and released on a single element.
        pub fn on_click<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::MouseEvent>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_click<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_click(value)),
            })
        }
        /// Event [`contextmenu`](https://developer.mozilla.org/en-US/docs/Web/API/Element/contextmenu_event)
        ///
        /// Fired when the user attempts to open a context menu.
        pub fn on_context_menu<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::MouseEvent>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_context_menu<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_context_menu(value)),
            })
        }
        /// Event [`dblclick`](https://developer.mozilla.org/en-US/docs/Web/API/Element/dblclick_event)
        ///
        /// Fired when a pointing device button (e.g., a mouse's primary button) is clicked twice on a single element.
        pub fn on_double_click<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::MouseEvent>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_double_click<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_double_click(value)),
            })
        }
        /// Event [`mousedown`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mousedown_event)
        ///
        /// Fired when a pointing device button is pressed on an element.
        pub fn on_mouse_down<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::MouseEvent>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_mouse_down<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_mouse_down(value)),
            })
        }
        /// Event [`mouseenter`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseenter_event)
        ///
        /// Fired when a pointing device (usually a mouse) is moved over the element that has the listener attached.
        pub fn on_mouse_enter<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::MouseEvent>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_mouse_enter<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_mouse_enter(value)),
            })
        }
        /// Event [`mouseleave`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseleave_event)
        ///
        /// Fired when the pointer of a pointing device (usually a mouse) is moved out of an element that has the listener attached to it.
        pub fn on_mouse_leave<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::MouseEvent>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_mouse_leave<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_mouse_leave(value)),
            })
        }
        /// Event [`mousemove`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mousemove_event)
        ///
        /// Fired when a pointing device (usually a mouse) is moved while over an element.
        pub fn on_mouse_move<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::MouseEvent>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_mouse_move<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_mouse_move(value)),
            })
        }
        /// Event [`mouseout`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseout_event)
        ///
        /// Fired when a pointing device (usually a mouse) is moved off the element to which the listener is attached or off one of its children.
        pub fn on_mouse_out<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::MouseEvent>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_mouse_out<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_mouse_out(value)),
            })
        }
        /// Event [`mouseover`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseover_event)
        ///
        /// Fired when a pointing device is moved onto the element to which the listener is attached or onto one of its children.
        pub fn on_mouse_over<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::MouseEvent>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_mouse_over<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_mouse_over(value)),
            })
        }
        /// Event [`mouseup`](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseup_event)
        ///
        /// Fired when a pointing device button is released on an element.
        pub fn on_mouse_up<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::MouseEvent>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_mouse_up<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_mouse_up(value)),
            })
        }
        /// Event [`touchcancel`](https://developer.mozilla.org/en-US/docs/Web/API/Element/touchcancel_event)
        ///
        /// Fired when one or more touch points have been disrupted in an implementation-specific manner (for example, too many touch points are created).
        pub fn on_touch_cancel<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::TouchEvent>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_touch_cancel<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_touch_cancel(value)),
            })
        }
        /// Event [`touchend`](https://developer.mozilla.org/en-US/docs/Web/API/Element/touchend_event)
        ///
        /// Fired when one or more touch points are removed from the touch surface.
        pub fn on_touch_end<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::TouchEvent>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_touch_end<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_touch_end(value)),
            })
        }
        /// Event [`touchmove`](https://developer.mozilla.org/en-US/docs/Web/API/Element/touchmove_event)
        ///
        /// Fired when one or more touch points are moved along the touch surface.
        pub fn on_touch_move<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::TouchEvent>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_touch_move<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_touch_move(value)),
            })
        }
        /// Event [`touchstart`](https://developer.mozilla.org/en-US/docs/Web/API/Element/touchstart_event)
        ///
        /// Fired when one or more touch points are placed on the touch surface.
        pub fn on_touch_start<V: crate::impl_bounds::MaybeHandleEvent::Bounds<dyn crate::dom::event::TouchEvent>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::on_touch_start<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::on_touch_start(value)),
            })
        }
        pub fn cite<V: crate::impl_bounds::MaybeValue::Bounds<str>>(self, value: V) -> super::Building<Children, (Attrs, super::attributes::cite<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::attributes::cite(value)),
            })
        }
    }
}
