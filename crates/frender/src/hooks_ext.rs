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
    use hooks::{HookPollNextUpdate, ShareValue, Signal, SignalHook};

    use super::element::OptionSignalHook;

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
            super::element::OptionSignalHook<S::SignalHook>,
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

            *mount_state = MountState::Mounted;
        }
    }
}

pub mod element {
    use std::{marker::PhantomData, pin::Pin, task::Poll};

    use frender_csr::RenderState;
    use frender_hook_element::state::{
        CursorPlaceholderWithRenderStatePinProject, MaybeIntoPollNextUpdate, MountState,
    };

    use frender_html::{
        dom::behaviors::{Node, NodeRenderSelf, NodeWithRenderContextAfterSelf},
        Element, RenderHtml, RenderStateKindPinned, RenderStateKindUnpinned, RenderStateOfContext,
    };
    use hooks::{HookPollNextUpdate, HookUnmount, ShareValue, Signal, SignalHook};

    use crate::ToElement;

    #[derive(Debug, Clone, Copy)]
    pub struct SignalIntoElement<S: ShareValue>(pub S);

    mod ssr {
        use super::*;

        impl<S: ShareValue> frender_ssr::SsrElement for SignalIntoElement<S>
        where
            S::Value: ToElement,
        {
            type HtmlChildren = <S::Value as ToElement>::ToElementHtmlChildren;

            fn into_html_children(self) -> Self::HtmlChildren {
                self.0.map(|s| s.to_element().into_html_children())
            }
        }
    }

    enum Never {}
    pub struct Kind<SH>(Never, std::marker::PhantomData<SH>)
    where
        SH: Unpin + SignalHook,
        SH::SignalShareValue: ToElement;

    type CursorPlaceholderWithRenderState<C, S> =
        frender_hook_element::state::CursorPlaceholderWithRenderState<C, (), S>;

    impl<SH> RenderStateKindUnpinned for Kind<SH>
    where
        SH: Unpin + SignalHook,
        SH::SignalShareValue: ToElement,
    {
        type UnpinnedRenderState<R: RenderHtml + ?Sized> = frender_hook_element::state::State<
            OptionSignalHook<SH>,
            CursorPlaceholderWithRenderState<
                R::CursorPlaceholder,
                UnpinnedRenderStateOfToElement<SH::SignalShareValue, R>,
            >,
            SignalHookToElement<RenderUpdateToElementWithUnpinnedState>,
        >;
    }

    impl<SH> RenderStateKindPinned for Kind<SH>
    where
        SH: Unpin + SignalHook,
        SH::SignalShareValue: ToElement,
    {
        type RenderState<R: RenderHtml + ?Sized> = frender_hook_element::state::State<
            OptionSignalHook<SH>,
            CursorPlaceholderWithRenderState<
                R::CursorPlaceholder,
                RenderStateOfToElement<SH::SignalShareValue, R>,
            >,
            SignalHookToElement<RenderUpdateToElementWithPinnedState>,
        >;
    }

    #[derive(Debug, Default)]
    struct ToElementWithHookData<T>(T);

    #[derive(Debug)]
    pub struct SignalHookToElement<U>(std::marker::PhantomData<U>);

    impl<U> Default for SignalHookToElement<U> {
        fn default() -> Self {
            Self(PhantomData)
        }
    }

    pub struct CursorPlaceholderRender<
        'a,
        R: ?Sized + RenderHtml,
        SH: SignalHook,
        U: RenderUpdateToElement<R, SH::SignalShareValue>,
    >
    where
        SH::SignalShareValue: ToElement,
    {
        renderer: &'a mut R,
        cursor_placeholder: &'a mut R::CursorPlaceholder,
        render_state: Pin<&'a mut U::State>,
        signal_hook: Pin<&'a mut SH>,
    }

    impl<
            'a,
            R: ?Sized + RenderHtml,
            SH: SignalHook,
            U: RenderUpdateToElement<R, SH::SignalShareValue>,
        > Unpin for CursorPlaceholderRender<'a, R, SH, U>
    where
        SH::SignalShareValue: ToElement,
    {
    }

    impl<
            'a,
            R: ?Sized + RenderHtml,
            SH: SignalHook,
            U: RenderUpdateToElement<R, SH::SignalShareValue>,
        > HookPollNextUpdate for CursorPlaceholderRender<'a, R, SH, U>
    where
        SH::SignalShareValue: ToElement,
    {
        fn poll_next_update(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<bool> {
            let Self {
                renderer,
                cursor_placeholder,
                render_state,
                signal_hook,
            } = self.get_mut();

            let render_state = render_state.as_mut();

            match signal_hook.as_mut().poll_next_update(cx) {
                Poll::Ready(true) => {
                    let signal = signal_hook.as_mut().use_hook(); // mark as seen

                    signal.map(|el| {
                        cursor_placeholder.with_render_context_after_self(
                            renderer,
                            |render_context| {
                                U::render_update_to_element(el, render_context, render_state)
                            },
                        )
                    });
                    Poll::Ready(true)
                }
                _ => render_state.poll_render(renderer, cx).map(|()| false),
            }
        }
    }

    pub enum RenderUpdateToElementWithPinnedState {}

    impl<R: ?Sized + RenderHtml, V: ?Sized + ToElement> RenderUpdateToElement<R, V>
        for RenderUpdateToElementWithPinnedState
    {
        type State = RenderStateOfToElement<V, R>;
        fn render_update_to_element(
            el: &V,
            render_context: &mut <R>::RenderContext<'_>,
            render_state: Pin<&mut Self::State>,
        ) {
            el.to_element().render_update(render_context, render_state)
        }
    }

    pub enum RenderUpdateToElementWithUnpinnedState {}

    impl<R: ?Sized + RenderHtml, V: ?Sized + ToElement> RenderUpdateToElement<R, V>
        for RenderUpdateToElementWithUnpinnedState
    {
        type State = UnpinnedRenderStateOfToElement<V, R>;
        fn render_update_to_element(
            el: &V,
            render_context: &mut <R>::RenderContext<'_>,
            render_state: Pin<&mut Self::State>,
        ) {
            el.to_element()
                .unpinned_render_update(render_context, render_state.get_mut())
        }
    }

    pub trait RenderUpdateToElement<R: ?Sized + RenderHtml, V: ?Sized + ToElement> {
        type State: RenderState<R>;

        fn render_update_to_element(
            el: &V,
            render_context: &mut R::RenderContext<'_>,
            render_state: Pin<&mut Self::State>,
        );
    }

    type RenderStateOfToElement<E, R> =
        <<E as ToElement>::ToElementRenderStateKind as RenderStateKindPinned>::RenderState<R>;
    type UnpinnedRenderStateOfToElement<E, R> =
        <<E as ToElement>::ToElementRenderStateKind as RenderStateKindUnpinned>::UnpinnedRenderState<R>;

    impl<R, SH, U>
        MaybeIntoPollNextUpdate<
            R,
            OptionSignalHook<SH>,
            CursorPlaceholderWithRenderState<R::CursorPlaceholder, U::State>,
        > for SignalHookToElement<U>
    where
        R: ?Sized + RenderHtml,
        SH: SignalHook + Unpin,
        SH::SignalShareValue: ToElement,
        U: RenderUpdateToElement<R, SH::SignalShareValue>,
    {
        type IntoPollNextUpdate<'a> = CursorPlaceholderRender<'a, R, SH, U>
        where
            Self: 'a,
            R: 'a,
            SH: 'a;

        fn maybe_into_poll_next_update<'a>(
            self: Pin<&'a mut Self>,
            renderer: &'a mut R,
            hook_data: Pin<&'a mut OptionSignalHook<SH>>,
            render_state: Pin<
                &'a mut CursorPlaceholderWithRenderState<R::CursorPlaceholder, U::State>,
            >,
        ) -> Option<Self::IntoPollNextUpdate<'a>> {
            let render_state = render_state.pin_project();
            match (
                &mut hook_data.get_mut().inner,
                render_state.cursor_placeholder_and_data,
            ) {
                (Some(signal_hook), Some((cursor_placeholder, ()))) => {
                    Some(CursorPlaceholderRender {
                        renderer,
                        cursor_placeholder,
                        render_state: render_state.render_state,
                        signal_hook: Pin::new(signal_hook),
                    })
                }
                _ => None,
            }
        }
    }

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

        fn poll_next_update(self, cx: _) {
            if let Some(ref mut inner) = self.get_mut().inner {
                Pin::new(inner).poll_next_update(cx)
            } else {
                std::task::Poll::Ready(false)
            }
        }
    );

    impl<S: Signal> frender_html::Element for SignalIntoElement<S>
    where
        S::SignalHook: Unpin,
        <S as ShareValue>::Value: ToElement,
    {
        type RenderStateKind = Kind<S::SignalHook>;

        fn render_update_maybe_reposition<Ctx: ?Sized + frender_html::HtmlRenderContext>(
            //
            self,
            render_context: &mut Ctx,
            render_state: Pin<&mut RenderStateOfContext<Self::RenderStateKind, Ctx>>,
            mut force_reposition: bool,
        ) {
            let frender_hook_element::state::StatePinProject {
                mount_state,
                hook_data,
                render_state,
                inner: _,
            } = render_state.pin_project();

            let CursorPlaceholderWithRenderStatePinProject {
                cursor_placeholder_and_data,
                render_state,
            } = render_state.pin_project();

            // mount cursor placeholder
            {
                if let Some((cursor_placeholder, ())) = cursor_placeholder_and_data {
                    force_reposition =
                        force_reposition || matches!(mount_state, MountState::Unmounted);
                    render_context.map_mut_render_context(|render_context: &mut _| {
                        cursor_placeholder.readd_self(render_context, force_reposition)
                    });
                } else {
                    force_reposition = true;
                    let node = render_context.map_mut_render_context(|render_context: &mut _| {
                        NodeRenderSelf::render_self(render_context)
                    });
                    *cursor_placeholder_and_data = Some((node, ()));
                }
            }

            match &mut hook_data.get_mut().inner {
                Some(signal_hook) if self.0.is_signal_of(signal_hook) => {
                    // signal hasn't changed. no need to update
                }
                signal_hook => {
                    // new signal
                    // force_reposition = true;
                    self.0.map(|el| {
                        el.to_element().render_update_maybe_reposition(
                            render_context,
                            render_state,
                            force_reposition,
                        )
                    });
                    *signal_hook = Some(self.0.to_signal_hook())
                }
            }
        }

        fn unpinned_render_update_maybe_reposition<
            Ctx: ?Sized + frender_html::HtmlRenderContext,
        >(
            //
            self,
            render_context: &mut Ctx,
            render_state: &mut frender_html::UnpinnedRenderStateOfContext<
                Self::RenderStateKind,
                Ctx,
            >,
            mut force_reposition: bool,
        ) {
            let frender_hook_element::state::StateMutProject {
                mount_state,
                hook_data,
                render_state,
                inner: _,
            } = render_state.as_mut_project();

            let CursorPlaceholderWithRenderState {
                cursor_placeholder_and_data,
                render_state,
            } = render_state;

            // mount cursor placeholder
            {
                if let Some((cursor_placeholder, ())) = cursor_placeholder_and_data {
                    force_reposition =
                        force_reposition || matches!(mount_state, MountState::Unmounted);
                    render_context.map_mut_render_context(|render_context: &mut _| {
                        cursor_placeholder.readd_self(render_context, force_reposition)
                    });
                } else {
                    force_reposition = true;
                    let node = render_context.map_mut_render_context(|render_context: &mut _| {
                        NodeRenderSelf::render_self(render_context)
                    });
                    *cursor_placeholder_and_data = Some((node, ()));
                }
            }

            match &mut hook_data.inner {
                Some(signal_hook) if self.0.is_signal_of(signal_hook) => {
                    // signal hasn't changed. no need to update
                }
                signal_hook => {
                    // new signal
                    // force_reposition = true;
                    self.0.map(|el| {
                        el.to_element().unpinned_render_update_maybe_reposition(
                            render_context,
                            render_state,
                            force_reposition,
                        )
                    });
                    *signal_hook = Some(self.0.to_signal_hook())
                }
            }
        }
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

    fn into_element(self) -> element::SignalIntoElement<Self>
    where
        Self: Sized,
    {
        element::SignalIntoElement(self)
    }

    fn to_element(&self) -> element::SignalIntoElement<Self::OwnedShareValue>
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
