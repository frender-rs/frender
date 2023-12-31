use frender_events::callable::{self, ArgumentType, CallableWithFixedArguments};
use hooks::ShareValue;

pub mod argument {
    macro_rules! wrap_share_value {
        ($(
            $vis:vis struct $name:ident
            <$tp:ident>
            ($fn_name:ident ![
                |$this:ident, $f:ident|
                -> $arg_ty:tt
                $impl_block:block
            ] $(,)?);
        )*) => {$(
            #[derive(Debug, Clone, Copy)]
            $vis struct $name<$tp: ::hooks::ShareValue>(pub(super) $tp);

            impl<$tp: ::hooks::ShareValue> PartialEq for $name<$tp> {
                fn eq(&self, other: &Self) -> bool {
                    self.0.equivalent_to(&other.0)
                }
            }

            impl<$tp: ::hooks::ShareValue> ::frender_events::callable::argument::ProvideArgument for $name<$tp> {
                type ProvideArgumentType = ::frender_events::callable::ArgumentType! $arg_ty;

                fn $fn_name<
                    Out,
                    F: for<'arg> FnOnce(
                        ::frender_events::callable::argument::ArgumentOfType<'arg, Self::ProvideArgumentType>,
                    ) -> Out,
                >(
                    &$this,
                    $f: F,
                ) -> Out
                    $impl_block
            }
        )*};
    }

    wrap_share_value!(
        pub struct Shared<S>(provide_argument_to![|self, f| -> (&S) { f(&self.0) }]);
        pub struct Mapped<S>(provide_argument_to![|self, f| -> (&S::Value) { self.0.map(f) }]);
        pub struct MappedMut<S>(
            provide_argument_to![|self, f| -> (&mut S::Value) { self.0.map_mut(f) }],
        );
    );
}

pub mod setter {
    use frender_events::callable::{
        ArgumentTypes, Callable, CallableWithFixedArguments, IsCallable,
    };
    use hooks::ShareValue;

    #[derive(Debug, Clone)]
    pub struct Setter<S: ShareValue>(pub S);

    #[derive(Debug, Clone)]
    pub struct SetterConditional<S: ShareValue>(pub S);

    impl<S: ShareValue> PartialEq for Setter<S> {
        fn eq(&self, other: &Self) -> bool {
            S::equivalent_to(&self.0, &other.0)
        }
    }

    impl<S: ShareValue> IsCallable for Setter<S> {}

    impl<S: ShareValue> Callable<(S::Value,)> for Setter<S> {
        type Output = ();

        fn call_fn(&self, (new_value,): (S::Value,)) -> Self::Output {
            self.0.set(new_value)
        }
    }

    impl<S: ShareValue> CallableWithFixedArguments for Setter<S> {
        type FixedArgumentTypes = ArgumentTypes!(S::Value,);
    }

    impl<S: ShareValue> PartialEq for SetterConditional<S> {
        fn eq(&self, other: &Self) -> bool {
            S::equivalent_to(&self.0, &other.0)
        }
    }

    impl<S: ShareValue> IsCallable for SetterConditional<S> {}

    impl<S: ShareValue> Callable<(Option<S::Value>,)> for SetterConditional<S> {
        type Output = ();

        fn call_fn(&self, (new_value,): (Option<S::Value>,)) -> Self::Output {
            if let Some(new_value) = new_value {
                self.0.set(new_value)
            }
        }
    }

    impl<S: ShareValue> CallableWithFixedArguments for SetterConditional<S> {
        type FixedArgumentTypes = ArgumentTypes!(Option<S::Value>,);
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct EventTargetFormControlValue;

    impl IsCallable for EventTargetFormControlValue {}
    impl<E: ?Sized + frender_events::event::Event> Callable<(&E,)> for EventTargetFormControlValue {
        type Output = Option<String>;

        fn call_fn(&self, (e,): (&E,)) -> Self::Output {
            e.target_form_control_value()
                .map(std::borrow::Cow::into_owned)
        }
    }
}

pub mod form_control {
    use std::borrow::Borrow;

    use async_str_iter::IntoAsyncStrIterator;
    use frender_html::{
        form_control::{
            element::FormControlElement,
            value::{FormControlValue, OfValue, Value},
        },
        RenderState,
    };
    use hooks::{Hook, HookValue, ShareValue};

    #[derive(Debug, Clone, Copy)]
    pub struct ControlledSharedValue<S>(pub S);

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
        pub struct CompoundState<S, T> {
            #[pin]
            reactive: S,
            non_reactive: T,
        }
    );

    impl<S: RenderState<R>, T, R> RenderState<R> for CompoundState<S, T> {
        fn unmount(self: std::pin::Pin<&mut Self>, renderer: &mut R) {
            self.project().reactive.unmount(renderer)
        }

        fn state_unmount(self: std::pin::Pin<&mut Self>) {
            self.project().reactive.state_unmount()
        }

        fn poll_render(
            self: std::pin::Pin<&mut Self>,
            renderer: &mut R,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<()> {
            self.project().reactive.poll_render(renderer, cx)
        }
    }

    pin_project_lite::pin_project!(
        #[project = ProjReactiveState]
        pub struct ReactiveState<S, E> {
            element: E,
            #[pin]
            inner: S,
        }
    );

    impl<
            V: ?Sized + Value,
            E: FormControlElement<V, R>,
            S: ShareValue + Hook + for<'hook> HookValue<'hook, Value = &'hook S>,
            R,
        > RenderState<R> for ReactiveState<S, E>
    where
        <S as ShareValue>::Value: OfValue<Value = V>,
    {
        fn unmount(self: std::pin::Pin<&mut Self>, renderer: &mut R) {
            let ProjReactiveState { element, inner } = self.project();

            element.remove_value(renderer);

            S::unmount(inner);
        }

        fn state_unmount(self: std::pin::Pin<&mut Self>) {
            S::unmount(self.project().inner)
        }

        fn poll_render(
            self: std::pin::Pin<&mut Self>,
            renderer: &mut R,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<()> {
            let ProjReactiveState { element, mut inner } = self.project();

            match inner.as_mut().poll_next_update(cx) {
                std::task::Poll::Ready(active) => {
                    if active {
                        let state = inner.use_hook(); // mark as seen

                        state.map(|value| {
                            element.set_default_value(renderer, value.borrow());
                            element.set_value(renderer, value.borrow());
                        });
                    }

                    std::task::Poll::Ready(())
                }
                std::task::Poll::Pending => std::task::Poll::Pending,
            }
        }
    }

    impl<S, Val> FormControlValue<Val::Value> for ControlledSharedValue<S>
    where
        S: Clone + 'static + Hook + for<'hook> HookValue<'hook, Value = &'hook S> + Unpin,
        S: ShareValue<Value = Val>,
        Val: OfValue,
    {
        type State<E: frender_html::form_control::element::FormControlElement<Val::Value, R>, R> =
            Option<CompoundState<ReactiveState<S, E>, E::OnValueChangeEventListener>>;

        fn update_with_state<
            E: frender_html::form_control::element::FormControlElement<Val::Value, R> + Clone,
            R,
        >(
            this: Self,
            state: &mut Self::State<E, R>,
            element: &mut E,
            renderer: &mut R,
        ) {
            if let Some(state) = state {
                if state.reactive.inner.equivalent_to(&this.0) {
                    return;
                }
            }

            this.0.map(|value| {
                let value = value.borrow();
                element.set_default_value(renderer, value);
                element.set_value(renderer, value);
            });

            *state = Some(CompoundState {
                reactive: ReactiveState {
                    element: element.clone(),
                    inner: this.0.clone(),
                },
                non_reactive: element
                    .on_value_change(renderer, move |value| this.0.set(Val::from(value))),
            });
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

    fn into_argument_shared(self) -> argument::Shared<Self>
    where
        Self: Sized,
    {
        argument::Shared(self)
    }

    fn into_argument_mapped(self) -> argument::Mapped<Self>
    where
        Self: Sized,
    {
        argument::Mapped(self)
    }

    fn into_argument_mapped_mut(self) -> argument::MappedMut<Self>
    where
        Self: Sized,
    {
        argument::MappedMut(self)
    }

    fn into_callback<F>(
        self,
        f: F,
    ) -> callable::argument::FirstArgumentProvided<F, argument::Shared<Self>>
    where
        Self: Sized,
        F: CallableWithFixedArguments,
        F::FixedArgumentTypes: callable::argument::ArgumentTypes<First = ArgumentType![&Self]>,
    {
        f.provide_first_argument(argument::Shared(self))
    }

    fn into_callback_map<F>(
        self,
        f: F,
    ) -> callable::argument::FirstArgumentProvided<F, argument::Mapped<Self>>
    where
        Self: Sized,
        F: CallableWithFixedArguments,
        F::FixedArgumentTypes:
            callable::argument::ArgumentTypes<First = ArgumentType![&Self::Value]>,
    {
        f.provide_first_argument(argument::Mapped(self))
    }

    fn into_callback_map_mut<F>(
        self,
        f: F,
    ) -> callable::argument::FirstArgumentProvided<F, argument::MappedMut<Self>>
    where
        Self: Sized,
        F: CallableWithFixedArguments,
        F::FixedArgumentTypes:
            callable::argument::ArgumentTypes<First = ArgumentType![&mut Self::Value]>,
    {
        f.provide_first_argument(argument::MappedMut(self))
    }

    fn into_setter(self) -> setter::Setter<Self>
    where
        Self: Sized,
    {
        setter::Setter(self)
    }

    fn into_setter_conditional(self) -> setter::SetterConditional<Self>
    where
        Self: Sized,
    {
        setter::SetterConditional(self)
    }

    fn into_setter_form_control_value(
        self,
    ) -> callable::chain::Chain<setter::EventTargetFormControlValue, setter::SetterConditional<Self>>
    where
        Self: Sized + ShareValue<Value = String>,
    {
        callable::chain::Chain(
            setter::EventTargetFormControlValue,
            setter::SetterConditional(self),
        )
    }
}

impl<S: ?Sized> ShareValueExt for S where S: ShareValue {}
