use std::{borrow::Borrow, marker::PhantomData, pin::Pin, task::Poll};

use frender_common::{IntoStaticStr, PrimarilyBorrow, ToStaticStr};
use frender_csr::StateUnmount;
use frender_html::form_control::{
    element::FormControlElement,
    input::{InputValue, InputValueKind},
    value::{
        FormControlValue, FormControlValueKind, FormControlValueStateKind, FromFormControlValue,
        HandleFormControlValue, MaybeProvideFormControlValue, ProvideFormControlValue,
    },
};
use hooks::{Hook as _, HookPollNextUpdate, HookUnmount, ShareValue, Signal, SignalHook};

mod textarea;

pub struct OptionSignalHook<SH> {
    pub(crate) inner: Option<SH>,
    #[cfg(debug_assertions)]
    update_times: UpdateTimes,
}

impl<SH> OptionSignalHook<SH> {
    fn new(inner: SH) -> Self {
        Self {
            inner: Some(inner),
            #[cfg(debug_assertions)]
            update_times: 0,
        }
    }
}

#[cfg(debug_assertions)]
type UpdateTimes = u8;
#[cfg(debug_assertions)]
fn increment_update_times(
    update_times: &mut UpdateTimes,
    warn: impl FnOnce(&str),
    f: &'static str,
) {
    const MAX_MINUS_1: UpdateTimes = UpdateTimes::MAX - 1;
    match *update_times {
        MAX_MINUS_1 => {
            *update_times = UpdateTimes::MAX;
            warn(&format!(
                r##"WARNING: SignalIntoControlledValueUpdateStateTooManyTimes.
The signal hook of `{}` has been emitting next update too many times in one time of calling poll_render(),
which means, repeatedly, calling `poll_next_update()` returns `Poll::Ready(true)` even after the hook has been updated.
This might be caused by bugs of hooks and frender."##,
                f,
            ));
        }
        UpdateTimes::MAX => {
            // already warned
        }
        _ => *update_times += 1,
    }
}

impl<SH: HookUnmount + Unpin> StateUnmount for OptionSignalHook<SH> {
    fn state_unmount(self: Pin<&mut Self>) {
        let Self {
            inner,
            #[cfg(debug_assertions)]
            update_times,
        } = self.get_mut();

        #[cfg(debug_assertions)]
        {
            *update_times = 0;
        }

        if let Some(inner) = inner {
            Pin::new(inner).unmount();
        }
    }
}

impl<SH> Default for OptionSignalHook<SH> {
    fn default() -> Self {
        Self {
            inner: None,
            #[cfg(debug_assertions)]
            update_times: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SignalIntoControlledValue<S>(pub S);

impl<S, Val, VK: ?Sized + FormControlValueKind> HandleFormControlValue<VK>
    for SignalIntoControlledValue<S>
where
    S: ShareValue<Value = Val>,
    Val: FromFormControlValue<VK>,
{
    fn handle_form_control_value(&mut self, v: <VK as FormControlValueKind>::FormControlValue<'_>) {
        self.0.set(Val::from_form_control_value(v))
    }
}

pub struct SignalIntoControlledValueToStaticStr<S>(pub S);

impl<S> IntoStaticStr for SignalIntoControlledValueToStaticStr<S>
where
    S: ShareValue,
    S::Value: ToStaticStr,
{
    type StaticStr = <S::Value as IntoStaticStr>::StaticStr;

    fn into_static_str(self) -> Self::StaticStr {
        self.to_static_str()
    }
}

impl<S> ToStaticStr for SignalIntoControlledValueToStaticStr<S>
where
    S: ShareValue,
    S::Value: ToStaticStr,
{
    fn to_static_str(&self) -> Self::StaticStr {
        self.0.map(ToStaticStr::to_static_str)
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

    fn maybe_into_provide_form_control_value(this: Self) -> Option<Self::ProvideFormControlValue> {
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

enum Never {}
pub struct Kind<S>(Never, PhantomData<S>);

impl<S, Val, VK> FormControlValueStateKind<VK> for Kind<S>
where
    VK: ?Sized + FormControlValueKind,
    S: Signal<Value = Val> + 'static,
    S::SignalHook: Unpin,
    Val: FromFormControlValue<VK> + Borrow<VK>,
{
    type UnpinnedNonReactiveState<E: FormControlElement<VK, R> + ?Sized, R: ?Sized> =
        E::OnValueChangeEventListener<SignalIntoControlledValue<S>>;

    type UnpinnedReactiveState = OptionSignalHook<S::SignalHook>;

    fn unpinned_poll_render_form_control_value_state<
        E: FormControlElement<VK, R> + ?Sized,
        R: ?Sized,
    >(
        renderer: &mut R,
        element: &mut E,
        _: &mut Self::UnpinnedNonReactiveState<E, R>,
        OptionSignalHook {
            inner,
            #[cfg(debug_assertions)]
            update_times,
        }: &mut Self::UnpinnedReactiveState,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        let Some(signal_hook) = inner else {
            return Poll::Ready(());
        };
        match Pin::new(&mut *signal_hook).poll_next_update(cx) {
            Poll::Ready(true) => {
                {
                    let signal = Pin::new(&mut *signal_hook).use_hook();
                    signal.map(|value| {
                        let value = value.borrow();

                        element.set_default_value(renderer, value);
                        element.set_value(renderer, value);
                    });
                }

                // Then, we re-check if signal_hook still emits new value
                match Pin::new(signal_hook).poll_next_update(cx) {
                    // signal_hook still has a new value!
                    Poll::Ready(true) => {
                        // Increment update times
                        #[cfg(debug_assertions)]
                        increment_update_times(
                            update_times,
                            |message| element.warn_self_with_message(renderer, message),
                            std::any::type_name::<SignalIntoControlledValue<S>>(),
                        );

                        // Let next poll decide what to do
                        cx.waker().wake_by_ref();
                        Poll::Pending
                    }
                    Poll::Ready(false) => Poll::Ready(()),
                    Poll::Pending => Poll::Pending,
                }
            }
            Poll::Ready(false) => Poll::Ready(()),
            Poll::Pending => Poll::Pending,
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
    type StateKind = Kind<S>;

    fn render_init<E: FormControlElement<VK, R> + ?Sized, R: ?Sized>(
        this: Self,
        renderer: &mut R,
        element: &mut E,
    ) -> (
        <Self::StateKind as FormControlValueStateKind<VK>>::UnpinnedNonReactiveState<E, R>,
        <Self::StateKind as FormControlValueStateKind<VK>>::UnpinnedReactiveState,
    ) {
        this.0.map(|value| {
            let value = value.borrow();
            element.set_default_value(renderer, value);
            element.set_value(renderer, value);
        });

        let mut res = (
            Default::default(),
            OptionSignalHook::new(this.0.to_signal_hook()),
        );

        let event_listener = &mut res.0;
        element.on_value_change(renderer, event_listener, this);

        res
    }

    fn render_update<E: FormControlElement<VK, R> + ?Sized, R: ?Sized>(
        this: Self,
        renderer: &mut R,
        element: &mut E,
        event_listener: &mut <Self::StateKind as FormControlValueStateKind<VK>>::UnpinnedNonReactiveState<E, R>,
        reactive_state: &mut <Self::StateKind as FormControlValueStateKind<VK>>::UnpinnedReactiveState,
    ) {
        if let Some(signal_hook) = &mut reactive_state.inner {
            if this.0.is_signal_of(signal_hook) {
                return;
            }
        }

        this.0.map(|value| {
            let value = value.borrow();
            element.set_default_value(renderer, value);
            element.set_value(renderer, value);
        });

        reactive_state.inner = Some(this.0.to_signal_hook());
        element.on_value_change(renderer, event_listener, this);
    }
}
