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
        Element, RenderHtml, RenderStateKind, RenderStateKindPinned, RenderStateKindUnpinned,
        RenderStateOfContext,
    };
    use frender_ssr::html::assert::HtmlChildren;
    use hooks::{HookPollNextUpdate, HookUnmount, ShareValue, Signal, SignalHook};

    use crate::{FnMutMapRefToElement, FnMutOutputElement, ToElement};

    pub trait MapToElement<V: ?Sized> {
        type RefToElement<'a>: Element<
            RenderStateKind = Self::RefToElementRenderStateKind,
            HtmlChildren = Self::RefToElementHtmlChildren,
        >
        where
            V: 'a;

        type RefToElementHtmlChildren: HtmlChildren;
        type RefToElementRenderStateKind: RenderStateKind;
        fn map_to_element<'a>(&mut self, v: &'a V) -> Self::RefToElement<'a>;
    }

    #[derive(Debug, Clone, Copy)]
    pub struct WithToElement;

    impl<V: ?Sized + ToElement> MapToElement<V> for WithToElement {
        type RefToElement<'a> = V::ToElement<'a>
        where
            V: 'a;

        type RefToElementHtmlChildren = V::ToElementHtmlChildren;
        type RefToElementRenderStateKind = V::ToElementRenderStateKind;

        fn map_to_element<'a>(&mut self, v: &'a V) -> Self::RefToElement<'a> {
            v.to_element()
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct WithFn<F>(pub F);

    impl<V, F> MapToElement<V> for WithFn<F>
    where
        V: ?Sized,
        F: FnMutMapRefToElement<V>,
    {
        type RefToElement<'a> = <F as FnMutOutputElement<&'a V>>::OutputElement
        where
            V: 'a;

        type RefToElementHtmlChildren = F::RefToElementHtmlChildren;
        type RefToElementRenderStateKind = F::RefToElementRenderStateKind;

        fn map_to_element<'a>(&mut self, v: &'a V) -> Self::RefToElement<'a> {
            (self.0)(v)
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct SignalIntoElement<S: ShareValue, F: MapToElement<S::Value> = WithToElement>(
        pub S,
        pub F,
    );

    mod ssr {
        use super::*;

        impl<S: ShareValue, F> frender_ssr::SsrElement for SignalIntoElement<S, F>
        where
            F: MapToElement<S::Value>,
        {
            type HtmlChildren = F::RefToElementHtmlChildren;

            fn into_html_children(mut self) -> Self::HtmlChildren {
                self.0
                    .map(|s| self.1.map_to_element(s).into_html_children())
            }
        }
    }

    pub struct OptionSignalHookAndMapToElement<SH, F> {
        inner: Option<(SH, F)>,
    }

    impl<SH, F> Unpin for OptionSignalHookAndMapToElement<SH, F> {}

    impl<SH, F> Default for OptionSignalHookAndMapToElement<SH, F> {
        fn default() -> Self {
            Self { inner: None }
        }
    }

    impl<SH: HookUnmount + Unpin, F> HookUnmount for OptionSignalHookAndMapToElement<SH, F> {
        fn unmount(self: Pin<&mut Self>) {
            if let Some((signal_hook, _)) = &mut self.get_mut().inner {
                SH::unmount(Pin::new(signal_hook))
            }
        }
    }

    enum Never {}
    pub struct Kind<SH, F>(Never, std::marker::PhantomData<(SH, F)>)
    where
        SH: Unpin + SignalHook,
        F: MapToElement<SH::SignalShareValue>;

    type CursorPlaceholderWithRenderState<C, S> =
        frender_hook_element::state::CursorPlaceholderWithRenderState<C, (), S>;

    impl<SH, F> RenderStateKindUnpinned for Kind<SH, F>
    where
        SH: Unpin + SignalHook,
        F: MapToElement<SH::SignalShareValue>,
    {
        type UnpinnedRenderState<R: RenderHtml + ?Sized> = frender_hook_element::state::State<
            OptionSignalHookAndMapToElement<SH, F>,
            CursorPlaceholderWithRenderState<
                R::CursorPlaceholder,
                UnpinnedRenderStateOfMapToElement<F, SH::SignalShareValue, R>,
            >,
            SignalHookToElement<RenderUpdateToElementWithUnpinnedState>,
        >;
    }

    impl<SH, F> RenderStateKindPinned for Kind<SH, F>
    where
        SH: Unpin + SignalHook,
        F: MapToElement<SH::SignalShareValue>,
    {
        type RenderState<R: RenderHtml + ?Sized> = frender_hook_element::state::State<
            OptionSignalHookAndMapToElement<SH, F>,
            CursorPlaceholderWithRenderState<
                R::CursorPlaceholder,
                RenderStateOfMapToElement<F, SH::SignalShareValue, R>,
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

    pub struct CursorPlaceholderRender<'a, R, SH, F, U>
    where
        R: ?Sized + RenderHtml,
        SH: SignalHook,
        F: ?Sized,
        U: RenderUpdateMapToElement<R, F, SH::SignalShareValue>,
    {
        renderer: &'a mut R,
        cursor_placeholder: &'a mut R::CursorPlaceholder,
        render_state: Pin<&'a mut U::State>,
        signal_hook: Pin<&'a mut SH>,
        f: &'a mut F,
    }

    impl<'a, R, SH, F, U> Unpin for CursorPlaceholderRender<'a, R, SH, F, U>
    where
        R: ?Sized + RenderHtml,
        SH: SignalHook,
        F: ?Sized,
        U: RenderUpdateMapToElement<R, F, SH::SignalShareValue>,
    {
    }

    impl<'a, R, SH, F, U> HookPollNextUpdate for CursorPlaceholderRender<'a, R, SH, F, U>
    where
        R: ?Sized + RenderHtml,
        SH: SignalHook,
        F: ?Sized,
        U: RenderUpdateMapToElement<R, F, SH::SignalShareValue>,
    {
        fn poll_next_update(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<bool> {
            let Self {
                renderer,
                cursor_placeholder,
                render_state,
                signal_hook,
                f,
            } = self.get_mut();

            let render_state = render_state.as_mut();

            match signal_hook.as_mut().poll_next_update(cx) {
                Poll::Ready(true) => {
                    let signal = signal_hook.as_mut().use_hook(); // mark as seen

                    signal.map(|el| {
                        cursor_placeholder.with_render_context_after_self(
                            renderer,
                            |render_context| {
                                U::render_update_to_element(f, el, render_context, render_state)
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

    impl<R: ?Sized + RenderHtml, V: ?Sized, F: ?Sized + MapToElement<V>>
        RenderUpdateMapToElement<R, F, V> for RenderUpdateToElementWithPinnedState
    {
        type State = RenderStateOfMapToElement<F, V, R>;
        fn render_update_to_element(
            f: &mut F,
            el: &V,
            render_context: &mut <R>::RenderContext<'_>,
            render_state: Pin<&mut Self::State>,
        ) {
            f.map_to_element(el)
                .render_update(render_context, render_state)
        }
    }

    pub enum RenderUpdateToElementWithUnpinnedState {}

    impl<R: ?Sized + RenderHtml, V: ?Sized, F: ?Sized + MapToElement<V>>
        RenderUpdateMapToElement<R, F, V> for RenderUpdateToElementWithUnpinnedState
    {
        type State = UnpinnedRenderStateOfMapToElement<F, V, R>;
        fn render_update_to_element(
            f: &mut F,
            el: &V,
            render_context: &mut <R>::RenderContext<'_>,
            render_state: Pin<&mut Self::State>,
        ) {
            f.map_to_element(el)
                .unpinned_render_update(render_context, render_state.get_mut())
        }
    }

    pub trait RenderUpdateMapToElement<R: ?Sized + RenderHtml, F: ?Sized, V: ?Sized> {
        type State: RenderState<R>;

        fn render_update_to_element(
            f: &mut F,
            el: &V,
            render_context: &mut R::RenderContext<'_>,
            render_state: Pin<&mut Self::State>,
        );
    }

    type RenderStateOfMapToElement<F, E, R> =
        <<F as MapToElement<E>>::RefToElementRenderStateKind as RenderStateKindPinned>::RenderState<
            R,
        >;
    type UnpinnedRenderStateOfMapToElement<F, E, R> =
        <<F as MapToElement<E>>::RefToElementRenderStateKind as RenderStateKindUnpinned>::UnpinnedRenderState<
            R
        >;

    impl<R, SH, F, U>
        MaybeIntoPollNextUpdate<
            R,
            OptionSignalHookAndMapToElement<SH, F>,
            CursorPlaceholderWithRenderState<R::CursorPlaceholder, U::State>,
        > for SignalHookToElement<U>
    where
        R: ?Sized + RenderHtml,
        SH: SignalHook + Unpin,
        U: RenderUpdateMapToElement<R, F, SH::SignalShareValue>,
    {
        type IntoPollNextUpdate<'a> = CursorPlaceholderRender<'a, R, SH, F, U>
        where
            Self: 'a,
            R: 'a,
            SH: 'a,
            F:'a;

        fn maybe_into_poll_next_update<'a>(
            self: Pin<&'a mut Self>,
            renderer: &'a mut R,
            hook_data: Pin<&'a mut OptionSignalHookAndMapToElement<SH, F>>,
            render_state: Pin<
                &'a mut CursorPlaceholderWithRenderState<R::CursorPlaceholder, U::State>,
            >,
        ) -> Option<Self::IntoPollNextUpdate<'a>> {
            let render_state = render_state.pin_project();
            match (
                &mut hook_data.get_mut().inner,
                render_state.cursor_placeholder_and_data,
            ) {
                (Some((signal_hook, f)), Some((cursor_placeholder, ()))) => {
                    Some(CursorPlaceholderRender {
                        renderer,
                        cursor_placeholder,
                        render_state: render_state.render_state,
                        signal_hook: Pin::new(signal_hook),
                        f,
                    })
                }
                _ => None,
            }
        }
    }

    impl<S: Signal, F> frender_html::Element for SignalIntoElement<S, F>
    where
        S::SignalHook: Unpin,
        F: MapToElement<<S as ShareValue>::Value>,
    {
        type RenderStateKind = Kind<S::SignalHook, F>;

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
                Some((signal_hook, _)) if self.0.is_signal_of(signal_hook) => {
                    // signal hasn't changed. no need to update
                }
                signal_hook => {
                    // new signal
                    // force_reposition = true;
                    let mut f = self.1;
                    self.0.map(|el| {
                        f.map_to_element(el).render_update_maybe_reposition(
                            render_context,
                            render_state,
                            force_reposition,
                        )
                    });
                    *signal_hook = Some((self.0.to_signal_hook(), f))
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
                Some((signal_hook, _)) if self.0.is_signal_of(signal_hook) => {
                    // signal hasn't changed. no need to update
                }
                signal_hook => {
                    // new signal
                    // force_reposition = true;
                    let mut f = self.1;
                    self.0.map(|el| {
                        f.map_to_element(el)
                            .unpinned_render_update_maybe_reposition(
                                render_context,
                                render_state,
                                force_reposition,
                            )
                    });
                    *signal_hook = Some((self.0.to_signal_hook(), f))
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
        Self::Value: crate::ToElement, // TODO: relax to ToSsrElement
    {
        element::SignalIntoElement(self, element::WithToElement)
    }

    fn to_element(&self) -> element::SignalIntoElement<Self::OwnedShareValue>
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
