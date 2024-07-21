use hooks::{ShareValue, ToOwnedShareValue};

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

pub mod form_control {
    use std::{borrow::Borrow, marker::PhantomData, pin::Pin};

    use async_str_iter::IntoAsyncStrIterator;
    use frender_common::PrimarilyBorrow;
    use frender_hook_element::state::{MaybeIntoPollNextUpdateWithPeh, MountState};
    use frender_html::{
        elements::non_reactive::NonReactiveRenderState,
        form_control::{
            element::FormControlElement,
            value::{
                FormControlValue, FormControlValueKind, FromFormControlValue,
                HandleFormControlValue, MaybeProvideFormControlValue, ProvideFormControlValue,
            },
            InputValue, InputValueKind,
        },
    };
    use hooks::{HookPollNextUpdate, HookUnmount, ShareValue, Signal, SignalHook};

    pub struct OptionSignalHook<SH> {
        pub(crate) inner: Option<SH>,
    }

    impl<SH> Default for OptionSignalHook<SH> {
        fn default() -> Self {
            Self { inner: None }
        }
    }

    hooks::impl_hook!(
        type For<SH: Unpin + HookUnmount + HookPollNextUpdate> = OptionSignalHook<SH>;

        fn unmount(self) {
            if let Some(ref mut inner) = self.get_mut().inner {
                Pin::new(inner).unmount()
            }
        }

        // TODO: remove
        fn poll_next_update(self, cx: _) {
            if let Some(ref mut inner) = self.get_mut().inner {
                Pin::new(inner).poll_next_update(cx)
            } else {
                std::task::Poll::Ready(false)
            }
        }
    );

    #[derive(Debug, Clone, Copy)]
    pub struct SignalIntoControlledValue<S>(pub S);

    impl<S, Val, VK: ?Sized + FormControlValueKind> HandleFormControlValue<VK>
        for SignalIntoControlledValue<S>
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

    impl<S: ShareValue> frender_html::IntoOneStringOrEmpty for SignalIntoControlledValue<S>
    where
        S::Value: Clone + Borrow<str>,
    {
        type OneStringOrEmpty = async_str_iter::borrow_str::IterBorrowStr<S::Value>;

        fn into_one_string_or_empty(this: Self) -> Self::OneStringOrEmpty {
            let val = this.0.unwrap_or_get_cloned();
            async_str_iter::borrow_str::BorrowStr(val).into_async_str_iterator()
        }
    }

    impl<S, Val, VK> ProvideFormControlValue<VK> for SignalIntoControlledValue<S>
    where
        S: ShareValue<Value = Val>,
        Val: Borrow<VK>,
        VK: ?Sized + FormControlValueKind,
    {
        fn provide_form_control_value<R>(&self, receive: impl FnOnce(&VK) -> R) -> R {
            self.0.map(|value| receive(value.borrow()))
        }
    }

    impl<S, Val, VK> MaybeProvideFormControlValue<VK> for SignalIntoControlledValue<S>
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

    impl<S, Val, VK> InputValue for SignalIntoControlledValue<S>
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

    impl<VK: ?Sized + FormControlValueKind> Unpin for UpdateFormControlElement<VK> {}

    impl<VK: ?Sized + FormControlValueKind> Default for UpdateFormControlElement<VK> {
        fn default() -> Self {
            Self(PhantomData)
        }
    }

    pub struct FormControlElementPollNextUpdate<'a, PEH: ?Sized, R: ?Sized, SH, VK: ?Sized> {
        peh: &'a mut PEH,
        renderer: &'a mut R,
        signal_hook: &'a mut SH,
        _vk: PhantomData<VK>,
    }

    impl<'a, PEH: ?Sized, R: ?Sized, SH, VK: ?Sized> Unpin
        for FormControlElementPollNextUpdate<'a, PEH, R, SH, VK>
    {
    }

    impl<'a, PEH: ?Sized, R: ?Sized, SH, VK> HookPollNextUpdate
        for FormControlElementPollNextUpdate<'a, PEH, R, SH, VK>
    where
        PEH: FormControlElement<VK, R>,
        SH: SignalHook + Unpin,
        SH::SignalShareValue: Borrow<VK>,
        VK: ?Sized + FormControlValueKind,
    {
        fn poll_next_update(
            self: Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<bool> {
            let Self {
                peh,
                renderer,
                signal_hook,
                _vk: _,
            } = self.get_mut();

            match Pin::new(&mut **signal_hook).poll_next_update(cx) {
                std::task::Poll::Ready(true) => {
                    let signal = Pin::new(&mut **signal_hook).use_hook();
                    signal.map(|value| {
                        let value = value.borrow();

                        peh.set_default_value(renderer, value);
                        peh.set_value(renderer, value);
                    });
                    std::task::Poll::Ready(true)
                }
                _ => std::task::Poll::Ready(false), // the render state is NonReactive
            }
        }
    }

    impl<VK: ?Sized + FormControlValueKind, PEH: ?Sized, R: ?Sized, SH, S>
        MaybeIntoPollNextUpdateWithPeh<PEH, R, OptionSignalHook<SH>, S>
        for UpdateFormControlElement<VK>
    where
        PEH: FormControlElement<VK, R>,
        SH: SignalHook + Unpin,
        SH::SignalShareValue: FromFormControlValue<VK> + Borrow<VK>,
    {
        type IntoPollNextUpdate<'a> = FormControlElementPollNextUpdate<'a, PEH, R, SH, VK>
        where
            Self: 'a,
            PEH: 'a,
            R: 'a,
            SH: 'a,
            S: 'a;

        fn maybe_into_poll_next_update_with_peh<'a>(
            self: Pin<&'a mut Self>,
            peh: &'a mut PEH,
            renderer: &'a mut R,
            hook_data: Pin<&'a mut OptionSignalHook<SH>>,
            _: Pin<&'a mut S>,
        ) -> Option<Self::IntoPollNextUpdate<'a>> {
            if let Some(signal_hook) = &mut hook_data.get_mut().inner {
                Some(FormControlElementPollNextUpdate {
                    peh,
                    renderer,
                    signal_hook,
                    _vk: PhantomData,
                })
            } else {
                None
            }
        }
    }

    impl<S, Val, VK> FormControlValue<VK> for SignalIntoControlledValue<S>
    where
        VK: ?Sized + FormControlValueKind,
        S: Signal<Value = Val> + 'static,
        S::SignalHook: Unpin,
        Val: FromFormControlValue<VK> + Borrow<VK>,
    {
        type State<
            E: frender_html::form_control::element::FormControlElement<VK, R> + ?Sized,
            R: ?Sized,
        > = frender_hook_element::state::State<
            OptionSignalHook<S::SignalHook>,
            NonReactiveRenderState<E::OnValueChangeEventListener<Self>>,
            UpdateFormControlElement<VK>,
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
            let frender_hook_element::state::StateMutProject {
                mount_state,
                hook_data,
                render_state: NonReactiveRenderState(event_listener),
                inner: _,
            } = state.as_mut_project();

            *mount_state = MountState::Mounted;

            if let Some(signal_hook) = &mut hook_data.inner {
                if this.0.is_signal_of(signal_hook) {
                    return;
                }
            }

            this.0.map(|value| {
                let value = value.borrow();
                element.set_default_value(renderer, value);
                element.set_value(renderer, value);
            });

            hook_data.inner = Some(this.0.to_signal_hook());

            element.on_value_change(renderer, event_listener, this);
        }
    }
}

pub mod element;

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
    fn into_controlled(self) -> form_control::SignalIntoControlledValue<Self>
    where
        Self: Sized,
    {
        form_control::SignalIntoControlledValue(self)
    }

    fn to_controlled(&self) -> form_control::SignalIntoControlledValue<Self::OwnedShareValue>
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

    fn into_element(self) -> element::SignalIntoElement<Self, element::WithToElement>
    where
        Self: Sized,
        Self::Value: crate::ToElement, // TODO: relax to ToSsrElement
    {
        element::SignalIntoElement(self, element::WithToElement)
    }

    fn to_element(
        &self,
    ) -> element::SignalIntoElement<Self::OwnedShareValue, element::WithToElement>
    where
        Self: Sized + ToOwnedShareValue,
        Self::Value: crate::ToElement, // TODO: relax to ToSsrElement
    {
        self.to_owned_share_value().into_element()
    }

    /// Note that f is considered non reactive
    fn into_element_with_fn<F>(self, f: F) -> element::SignalIntoElement<Self, element::WithFn<F>>
    where
        Self: Sized,
        F: crate::FnMutMapRefToElement<Self::Value>,
    {
        element::SignalIntoElement(self, element::WithFn(f))
    }

    fn to_element_with_fn<F>(
        &self,
        f: F,
    ) -> element::SignalIntoElement<Self::OwnedShareValue, element::WithFn<F>>
    where
        Self: Sized + ToOwnedShareValue,
        F: crate::FnMutMapRefToElement<Self::Value>,
    {
        self.to_owned_share_value().into_element_with_fn(f)
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
