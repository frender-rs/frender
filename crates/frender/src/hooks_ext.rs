use hooks::{ShareValue, Signal, ToOwnedShareValue, ToOwnedSignal};

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
    use hooks::{ShareValue, SignalHook};

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
            S: SignalHook<SignalShareValue = Val>,
            Val,
            U: UpdateElementWithSharedValue<PEH, R, <S as SignalHook>::SignalShareValue>,
            PEH: ?Sized,
            R: ?Sized,
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
    use std::{borrow::Borrow, marker::PhantomData};

    use async_str_iter::IntoAsyncStrIterator;
    use frender_common::PrimarilyBorrow;
    use frender_csr::render_state::compound::CompoundState;
    use frender_html::form_control::{
        element::FormControlElement,
        value::{
            FormControlValue, FormControlValueKind, FromFormControlValue, HandleFormControlValue,
            MaybeProvideFormControlValue, ProvideFormControlValue,
        },
        InputValue, InputValueKind,
    };
    use hooks::{ShareValue, Signal};

    #[derive(Debug, Clone, Copy)]
    pub struct ControlledSharedValue<S>(pub S);

    impl<S, Val, VK: ?Sized + FormControlValueKind> HandleFormControlValue<VK>
        for ControlledSharedValue<S>
    where
        S: ShareValue<Value = Val>,
        Val: FromFormControlValue<VK>,
    {
        fn handle_form_control_value(
            &mut self,
            v: <VK as FormControlValueKind>::FormControlValue<'_>,
        ) {
            self.0.set(Val::from_form_control_value(v))
        }
    }

    impl<S: ShareValue> frender_html::IntoOneStringOrEmpty for ControlledSharedValue<S>
    where
        S::Value: Clone + Borrow<str>,
    {
        type OneStringOrEmpty = async_str_iter::borrow_str::IterBorrowStr<S::Value>;

        fn into_one_string_or_empty(this: Self) -> Self::OneStringOrEmpty {
            let val = this.0.unwrap_or_get_cloned();
            async_str_iter::borrow_str::BorrowStr(val).into_async_str_iterator()
        }
    }

    impl<S, Val, VK> ProvideFormControlValue<VK> for ControlledSharedValue<S>
    where
        S: ShareValue<Value = Val>,
        Val: Borrow<VK>,
        VK: ?Sized + FormControlValueKind,
    {
        fn provide_form_control_value<R>(&self, receive: impl FnOnce(&VK) -> R) -> R {
            self.0.map(|value| receive(value.borrow()))
        }
    }

    impl<S, Val, VK> MaybeProvideFormControlValue<VK> for ControlledSharedValue<S>
    where
        S: ShareValue<Value = Val>,
        Val: Borrow<VK>,
        VK: ?Sized + FormControlValueKind,
    {
        type ProvideFormControlValue = Self;

        fn maybe_into_provide_form_control_value(
            this: Self,
        ) -> Option<Self::ProvideFormControlValue> {
            Some(this)
        }
    }

    impl<S, Val, VK> InputValue for ControlledSharedValue<S>
    where
        // S: Clone + 'static + Hook + for<'hook> HookValue<'hook, Value = &'hook S> + Unpin,
        S: Signal<Value = Val> + 'static,
        S::SignalHook: Unpin,
        Val: FromFormControlValue<VK> + Borrow<VK>,
        Val: PrimarilyBorrow<Borrowed = VK>,
        VK: ?Sized + FormControlValueKind,
        VK: InputValueKind,
    {
        type ValueKind = VK;
    }

    pub struct UpdateFormControlElement<VK: ?Sized + FormControlValueKind>(PhantomData<VK>);

    impl<VK: ?Sized + FormControlValueKind> UpdateFormControlElement<VK> {
        pub const fn new() -> Self {
            Self(PhantomData)
        }
    }

    impl<
            VK: ?Sized + FormControlValueKind,
            PEH: FormControlElement<VK, R> + ?Sized,
            R: ?Sized,
            SV: Borrow<VK>,
        > super::state::UpdateElementWithSharedValue<PEH, R, SV> for UpdateFormControlElement<VK>
    {
        fn unmount_element_with_shared_value(&mut self, element: &mut PEH, renderer: &mut R) {
            element.remove_value(renderer); // TODO: is this needed?
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

    pub type ReactiveState<S, VK> = super::state::State<S, UpdateFormControlElement<VK>>;

    impl<S, Val, VK> FormControlValue<VK> for ControlledSharedValue<S>
    where
        VK: ?Sized + FormControlValueKind,
        S: Signal<Value = Val> + 'static,
        S::SignalHook: Unpin,
        Val: FromFormControlValue<VK> + Borrow<VK>,
    {
        type State<
            E: frender_html::form_control::element::FormControlElement<VK, R> + ?Sized,
            R: ?Sized,
        > = CompoundState<
            Option<ReactiveState<S::SignalHook, VK>>,
            E::OnValueChangeEventListener<Self>,
        >;

        fn update_with_state<
            E: frender_html::form_control::element::FormControlElement<VK, R> + ?Sized,
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
                if this.0.is_signal_of(&state.inner) {
                    return;
                }
            }

            this.0.map(|shared_value| {
                super::state::UpdateElementWithSharedValue::update_element_with_shared_value(
                    &mut UpdateFormControlElement::new(),
                    element,
                    renderer,
                    shared_value,
                );
            });

            *state = Some(ReactiveState {
                inner: this.0.to_signal_hook(),
                update: UpdateFormControlElement::new(),
            });

            element.on_value_change(renderer, event_listener, this)
        }
    }
}

pub mod element {
    use std::borrow::Borrow;

    use frender_common::PrimarilyBorrow;
    use frender_html::{dom::render::RenderAsText, elements::str::TextNode, RenderHtml};
    use hooks::{ShareValue, Signal};

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

    impl<S: Signal, V: ?Sized> frender_html::Element for SharedStateToElement<S>
    where
        S::SignalHook: Unpin,
        <S as ShareValue>::Value: PrimarilyBorrow<Borrowed = V> + Borrow<V>,
        V: frender_ssr::ToSsrElement,
        V: RenderAsText,
    {
        type RenderState<PEH: ?Sized, R: frender_html::RenderHtml + ?Sized> =
            Option<State<S::SignalHook, R::Text>>;

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
                    if !self.0.is_signal_of(&render_state.inner) {
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
                        inner: self.0.to_signal_hook(),
                    });
                }
            }
        }

        frender_html::impl_unpinned_render_for_unpin! {}
    }
}

pub mod callback {
    use frender_common::{HandleEvent, MaybeHandleEvent};
    use hooks::ShareValue;

    #[derive(Debug, Clone)]
    pub struct Toggle<S: ShareValue<Value = bool>>(pub S);

    impl<S: ShareValue<Value = bool>> PartialEq for Toggle<S> {
        fn eq(&self, other: &Self) -> bool {
            self.0.equivalent_to(&other.0)
        }
    }

    impl<E: ?Sized, S: ShareValue<Value = bool>> HandleEvent<E> for Toggle<S> {
        fn handle_event(&mut self, _: &E) {
            self.0.map_mut(|v| *v = !*v)
        }
    }

    impl<E: ?Sized, S: ShareValue<Value = bool>> MaybeHandleEvent<E> for Toggle<S> {
        type HandleEvent = Self;
    }
}

/// `to_*` methods require [`ToOwnedShareValue`] instead of [`Clone`], so that:
///  - for [`&SharedSignal`](hooks::SharedSignal), the value is cloned as expected
///  - for [`SignalEq<&SharedSignal>`](hooks::SignalEq), the inner value is cloned,
///    so the value becomes `SignalEq<SharedSignal>`
///    (rather than the reference is copied)
///  - for `GenSignal` and `SignalEq<GenSignal>`, the value is copied
pub trait ShareValueExt: ShareValue {
    fn into_controlled(self) -> form_control::ControlledSharedValue<Self>
    where
        Self: Sized,
    {
        form_control::ControlledSharedValue(self)
    }

    fn to_controlled(&self) -> form_control::ControlledSharedValue<Self::OwnedShareValue>
    where
        Self: ToOwnedShareValue,
    {
        self.to_owned_share_value().into_controlled()
    }

    fn into_set_form_control_value(self) -> setter::SetEventTargetFormControlValue<Self>
    where
        Self: Sized,
        for<'a> std::borrow::Cow<'a, str>: Into<Self::Value>,
    {
        setter::SetEventTargetFormControlValue(self)
    }

    fn to_set_form_control_value(
        &self,
    ) -> setter::SetEventTargetFormControlValue<Self::OwnedShareValue>
    where
        Self: ToOwnedShareValue,
        for<'a> std::borrow::Cow<'a, str>: Into<Self::Value>, // TODO: better constraints
    {
        self.to_owned_share_value().into_set_form_control_value()
    }

    fn into_element(self) -> element::SharedStateToElement<Self>
    where
        Self: Sized,
    {
        element::SharedStateToElement(self)
    }

    fn to_element(&self) -> element::SharedStateToElement<Self::OwnedShareValue>
    where
        Self: Sized + ToOwnedShareValue,
    {
        self.to_owned_share_value().into_element()
    }

    fn into_callback_toggle(self) -> callback::Toggle<Self>
    where
        Self: Sized + ShareValue<Value = bool>,
    {
        callback::Toggle(self)
    }

    fn to_callback_toggle(&self) -> callback::Toggle<Self::OwnedShareValue>
    where
        Self: ShareValue<Value = bool> + ToOwnedShareValue,
    {
        self.to_owned_share_value().into_callback_toggle()
    }
}

impl<S: ?Sized> ShareValueExt for S where S: ShareValue {}
