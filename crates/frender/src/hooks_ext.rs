use hooks::ShareValue;

pub mod setter {
    use std::borrow::Cow;

    use frender_html::dom::{HandleEvent, MaybeHandleEvent};
    use hooks::ShareValue;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct EventTargetFormControlValue;

    #[derive(Debug)]
    pub struct SetEventTargetFormControlValue<S: ShareValue>(pub S);

    impl<S: ShareValue> SetEventTargetFormControlValue<S>
    where
        // S::Value: for<'a> From<Cow<'a, str>>,
        for<'a> Cow<'a, str>: Into<S::Value>,
    {
        pub fn handle_event_by_ref<E: ?Sized + frender_events::event::Event>(&self, event: &E) {
            if let Some(value) = event.target_form_control_value().map(Into::into) {
                // only update when the event target is form control
                self.0.set(value)
            }
        }

        pub fn into_fn<E: ?Sized + frender_events::event::Event>(self) -> impl Fn(&E) {
            move |e| self.handle_event_by_ref(e)
        }
    }

    impl<E: ?Sized + frender_events::event::Event, S: ShareValue> HandleEvent<E>
        for SetEventTargetFormControlValue<S>
    where
        // S::Value: for<'a> From<Cow<'a, str>>,
        for<'a> Cow<'a, str>: Into<S::Value>,
    {
        fn handle_event(&mut self, event: &E) {
            self.handle_event_by_ref(event)
        }
    }

    impl<E: ?Sized + frender_events::event::Event, S: ShareValue> MaybeHandleEvent<E>
        for SetEventTargetFormControlValue<S>
    where
        // S::Value: for<'a> From<Cow<'a, str>>,
        for<'a> Cow<'a, str>: Into<S::Value>,
    {
        type HandleEvent = Self;
    }
}

pub mod state {
    use frender_html::RenderState;
    use hooks::{Hook, HookValue, ShareValue};

    /// See [`RenderState`] for meaning of `PEH`.
    pub trait UpdateElementWithSharedValue<PEH: ?Sized, R: ?Sized, V: ?Sized> {
        fn unmount_element_with_shared_value(&mut self, peh: &mut PEH, renderer: &mut R);
        fn update_element_with_shared_value(
            &mut self,
            peh: &mut PEH,
            renderer: &mut R,
            shared_value: &V,
        );
    }

    pin_project_lite::pin_project!(
        #[project = StateProj]
        pub struct State<S, U> {
            #[pin]
            pub(crate) inner: S,
            pub(crate) update: U,
        }
    );

    impl<
            U: UpdateElementWithSharedValue<PEH, R, <S as ShareValue>::Value>,
            PEH: ?Sized,
            R: ?Sized,
            S: ShareValue + Hook + for<'hook> HookValue<'hook, Value = &'hook S>,
        > RenderState<PEH, R> for State<S, U>
    {
        fn unmount(self: std::pin::Pin<&mut Self>, peh: &mut PEH, renderer: &mut R) {
            let StateProj { inner, update } = self.project();

            update.unmount_element_with_shared_value(peh, renderer);

            S::unmount(inner);
        }

        fn state_unmount(self: std::pin::Pin<&mut Self>) {
            S::unmount(self.project().inner)
        }

        fn poll_render(
            self: std::pin::Pin<&mut Self>,
            peh: &mut PEH,
            renderer: &mut R,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<()> {
            let StateProj { mut inner, update } = self.project();

            match inner.as_mut().poll_next_update(cx) {
                std::task::Poll::Ready(active) => {
                    if active {
                        let state = inner.use_hook(); // mark as seen

                        state.map(|shared_value| {
                            update.update_element_with_shared_value(peh, renderer, shared_value)
                        });
                    }

                    std::task::Poll::Ready(())
                }
                std::task::Poll::Pending => std::task::Poll::Pending,
            }
        }
    }
}

pub mod form_control {
    use async_str_iter::IntoAsyncStrIterator;
    use frender_html::{
        form_control::{
            element::FormControlElement,
            value::{FormControlValue, HandleValue, OfValue, Value},
        },
        RenderState,
    };
    use hooks::{Hook, HookValue, ShareValue};

    #[derive(Debug, Clone, Copy)]
    pub struct ControlledSharedValue<S>(pub S);

    impl<S, Val, V: ?Sized + Value> HandleValue<V> for ControlledSharedValue<S>
    where
        S: ShareValue<Value = Val>,
        Val: OfValue<Value = V>,
    {
        fn handle_value(&mut self, v: <V as Value>::Passed<'_>) {
            self.0.set(Val::of_value(v))
        }
    }

    impl<S, Val: Clone> frender_html::maybe_str::IntoOneStringOrEmpty for ControlledSharedValue<S>
    where
        S: ShareValue<Value = Val>,
        Val: OfValue<Value = str>,
    {
        type OneStringOrEmpty = async_str_iter::borrow_str::IterBorrowStr<Val>;

        fn into_one_string_or_empty(this: Self) -> Self::OneStringOrEmpty {
            let val = this.0.unwrap_or_get_cloned();
            async_str_iter::borrow_str::BorrowStr(val).into_async_str_iterator()
        }
    }

    pin_project_lite::pin_project!(
        #[derive(Debug, Default)]
        pub struct CompoundState<S, T> {
            #[pin]
            reactive: S,
            non_reactive: T,
        }
    );

    impl<PEH: ?Sized, R: ?Sized, S: RenderState<PEH, R>, T> RenderState<PEH, R>
        for CompoundState<S, T>
    {
        fn unmount(self: std::pin::Pin<&mut Self>, peh: &mut PEH, renderer: &mut R) {
            self.project().reactive.unmount(peh, renderer)
        }

        fn state_unmount(self: std::pin::Pin<&mut Self>) {
            self.project().reactive.state_unmount()
        }

        fn poll_render(
            self: std::pin::Pin<&mut Self>,
            peh: &mut PEH,
            renderer: &mut R,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<()> {
            self.project().reactive.poll_render(peh, renderer, cx)
        }
    }

    pub struct UpdateFormControlElement;

    impl<
            PEH: FormControlElement<V, R> + ?Sized,
            R: ?Sized,
            V: ?Sized + Value,
            SV: OfValue<Value = V>,
        > super::state::UpdateElementWithSharedValue<PEH, R, SV> for UpdateFormControlElement
    {
        fn unmount_element_with_shared_value(&mut self, element: &mut PEH, renderer: &mut R) {
            element.remove_value(renderer);
        }

        fn update_element_with_shared_value(
            &mut self,
            element: &mut PEH,
            renderer: &mut R,
            value: &SV,
        ) {
            element.set_default_value(renderer, value.borrow());
            element.set_value(renderer, value.borrow());
        }
    }

    pub type ReactiveState<S> = super::state::State<S, UpdateFormControlElement>;

    impl<S, Val> FormControlValue<Val::Value> for ControlledSharedValue<S>
    where
        S: Clone + 'static + Hook + for<'hook> HookValue<'hook, Value = &'hook S> + Unpin,
        S: ShareValue<Value = Val>,
        Val: OfValue,
    {
        type State<
            E: frender_html::form_control::element::FormControlElement<Val::Value, R> + ?Sized,
            R: ?Sized,
        > = CompoundState<Option<ReactiveState<S>>, E::OnValueChangeEventListener<Self>>;

        fn update_with_state<
            E: frender_html::form_control::element::FormControlElement<Val::Value, R> + ?Sized,
            R: ?Sized,
        >(
            this: Self,
            state: &mut Self::State<E, R>,
            element: &mut E,
            renderer: &mut R,
        ) {
            let CompoundState {
                reactive: state,
                non_reactive: event_listener,
            } = state;
            if let Some(state) = state {
                if state.inner.equivalent_to(&this.0) {
                    return;
                }
            }

            this.0.map(|shared_value| {
                super::state::UpdateElementWithSharedValue::update_element_with_shared_value(
                    &mut UpdateFormControlElement,
                    element,
                    renderer,
                    shared_value,
                );
            });

            *state = Some(ReactiveState {
                inner: this.0.clone(),
                update: UpdateFormControlElement,
            });

            element.on_value_change(renderer, event_listener, this)
        }
    }
}

pub mod eq {
    use hooks::ShareValue;

    #[derive(Debug, Clone, Copy)]
    pub struct EquivalentShareValue<S: ShareValue>(pub S);

    impl<S: ShareValue> PartialEq for EquivalentShareValue<S> {
        fn eq(&self, other: &Self) -> bool {
            self.0.equivalent_to(&other.0)
        }
    }

    impl<S: ShareValue> std::ops::Deref for EquivalentShareValue<S> {
        type Target = S;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
}

pub mod element {
    use std::borrow::Borrow;

    use frender_common::PrimarilyBorrow;
    use frender_html::{dom::render::RenderAsText, elements::str::TextNode, RenderHtml};
    use hooks::{Hook, HookValue, ShareValue};

    #[derive(Debug, Clone, Copy)]
    pub struct SharedStateToElement<S: ShareValue>(pub S);

    impl<S: ShareValue, V: ?Sized> frender_ssr::SsrElement for SharedStateToElement<S>
    where
        S::Value: PrimarilyBorrow<Borrowed = V>,
        V: frender_ssr::ToSsrElement,
    {
        type HtmlChildren = <V::ToSsrElement as frender_ssr::SsrElement>::HtmlChildren;

        fn into_html_children(self) -> Self::HtmlChildren {
            self.0
                .map(|s| s.borrow().to_ssr_element())
                .into_html_children()
        }
    }

    pub struct UpdateTextNode<TN> {
        text_node: TextNode<TN>,
    }

    impl<PEH: ?Sized, R: ?Sized, SV: ?Sized + PrimarilyBorrow<Borrowed = V>, V: ?Sized>
        super::state::UpdateElementWithSharedValue<PEH, R, SV> for UpdateTextNode<R::Text>
    where
        R: RenderHtml,
        V: RenderAsText,
    {
        fn unmount_element_with_shared_value(&mut self, _: &mut PEH, renderer: &mut R) {
            self.text_node.unmount(renderer)
        }

        fn update_element_with_shared_value(
            &mut self,
            _: &mut PEH,
            renderer: &mut R,
            shared_value: &SV,
        ) {
            V::render_as_text_update(shared_value.borrow(), renderer, &mut self.text_node.node)
        }
    }

    pub type State<S, TextNode> = super::state::State<S, UpdateTextNode<TextNode>>;

    impl<S: ShareValue, V: ?Sized> frender_html::Element for SharedStateToElement<S>
    where
        S: ShareValue + Hook + for<'hook> HookValue<'hook, Value = &'hook S> + Unpin,
        <S as ShareValue>::Value: PrimarilyBorrow<Borrowed = V> + Borrow<V>,
        V: frender_ssr::ToSsrElement,
        V: RenderAsText,
    {
        type RenderState<PEH: ?Sized, R: frender_html::RenderHtml + ?Sized> =
            Option<State<S, R::Text>>;

        fn render_update_maybe_reposition<
            PEH: ?Sized,
            Renderer: frender_html::RenderHtml + ?Sized,
        >(
            //
            self,
            _: &mut PEH,
            renderer: &mut Renderer,
            render_state: std::pin::Pin<&mut Self::RenderState<PEH, Renderer>>,
            force_reposition: bool,
        ) {
            let render_state = render_state.get_mut();

            match render_state {
                Some(render_state) => {
                    if !render_state.inner.equivalent_to(&self.0) {
                        self.0.map(|value| {
                            value.borrow().render_as_text_update(
                                renderer,
                                &mut render_state.update.text_node.node,
                            )
                        })
                    }

                    render_state
                        .update
                        .text_node
                        .readd_self(renderer, force_reposition)
                }
                render_state => {
                    *render_state = Some(State {
                        update: UpdateTextNode {
                            text_node: {
                                let node =
                                    self.0.map(|value| value.borrow().render_as_text(renderer));
                                TextNode::mount(renderer, node)
                            },
                        },
                        inner: self.0,
                    });
                }
            }
        }

        frender_html::impl_unpinned_render_for_unpin! {}
    }
}

pub mod callback {
    use frender_common::HandleEvent;
    use hooks::ShareValue;

    #[derive(Debug, Clone)]
    pub struct Toggle<S: ShareValue<Value = bool>>(pub S);

    impl<S: ShareValue<Value = bool>> PartialEq for Toggle<S> {
        fn eq(&self, other: &Self) -> bool {
            self.0.equivalent_to(&other.0)
        }
    }

    impl<E: ?Sized, S: ShareValue<Value = bool>> HandleEvent<E> for Toggle<S> {
        fn handle_event(&mut self, event: &E) {
            self.0.map_mut(|v| *v = !*v)
        }
    }
}

pub trait ShareValueExt: ShareValue {
    fn into_controlled(self) -> form_control::ControlledSharedValue<Self>
    where
        Self: Sized,
    {
        form_control::ControlledSharedValue(self)
    }

    fn to_eq(&self) -> eq::EquivalentShareValue<Self>
    where
        Self: Sized + Clone,
    {
        eq::EquivalentShareValue(Self::clone(self))
    }

    fn into_eq(self) -> eq::EquivalentShareValue<Self>
    where
        Self: Sized,
    {
        eq::EquivalentShareValue(self)
    }

    fn into_set_form_control_value(self) -> setter::SetEventTargetFormControlValue<Self>
    where
        Self: Sized,
        for<'a> std::borrow::Cow<'a, str>: Into<Self::Value>,
    {
        setter::SetEventTargetFormControlValue(self)
    }

    fn to_set_form_control_value(&self) -> setter::SetEventTargetFormControlValue<Self>
    where
        Self: Sized + Clone,
        for<'a> std::borrow::Cow<'a, str>: Into<Self::Value>,
    {
        self.clone().into_set_form_control_value()
    }

    fn to_element(&self) -> element::SharedStateToElement<Self>
    where
        Self: Sized + Clone,
    {
        element::SharedStateToElement(self.clone())
    }

    fn into_callback_toggle(self) -> callback::Toggle<Self>
    where
        Self: Sized + ShareValue<Value = bool>,
    {
        callback::Toggle(self)
    }

    fn to_callback_toggle(&self) -> callback::Toggle<Self>
    where
        Self: Sized + ShareValue<Value = bool> + Clone,
    {
        self.clone().into_callback_toggle()
    }
}

impl<S: ?Sized> ShareValueExt for S where S: ShareValue {}
