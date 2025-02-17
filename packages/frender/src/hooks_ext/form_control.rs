use std::{borrow::Borrow, marker::PhantomData, pin::Pin, task::Poll};

use frender_common::{IntoStaticStr, PrimarilyBorrow, ToStaticStr};
use frender_html::{
    dom::{RegisterUpdate, StateUnmount},
    form_control::{
        element::FormControlElement,
        input::{InputValue, InputValueKind},
        value::{
            FormControlValue, FormControlValueKind, FormControlValueStateKind,
            FromFormControlValue, HandleFormControlValue, MaybeProvideFormControlValue,
            ProvideFormControlValue,
        },
    },
};
use hooks::{Hook as _, HookPollNextUpdate, HookUnmount, ShareValue, Signal};

mod textarea;

pub struct State<SH, NRS> {
    inner: SH,
    non_reactive_state: NRS,
    #[cfg(debug_assertions)]
    update_times: UpdateTimes,
}

impl<SH, NRS> Unpin for State<SH, NRS> {}

impl<SH, NRS> State<SH, NRS> {
    fn new(inner: SH, non_reactive_state: NRS) -> Self {
        Self {
            inner,
            non_reactive_state,
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

impl<SH: HookUnmount + Unpin, NRS> StateUnmount for State<SH, NRS> {
    fn state_unmount(self: Pin<&mut Self>) {
        let Self {
            inner,
            non_reactive_state: _,
            #[cfg(debug_assertions)]
            update_times,
        } = self.get_mut();

        #[cfg(debug_assertions)]
        {
            *update_times = 0;
        }

        Pin::new(inner).unmount();
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
    type UnpinnedState<E: FormControlElement<VK, R> + ?Sized, R: ?Sized> =
        State<S::SignalHook, E::OnValueChangeEventListenerUnpinned<SignalIntoControlledValue<S>>>;

    fn unpinned_poll_render_form_control_value_state<
        E: FormControlElement<VK, R> + ?Sized,
        R: ?Sized,
    >(
        renderer: &mut R,
        element: &mut E,
        State {
            inner: signal_hook,
            non_reactive_state: _,
            #[cfg(debug_assertions)]
            update_times,
        }: &mut Self::UnpinnedState<E, R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
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
    ) -> <Self::StateKind as FormControlValueStateKind<VK>>::UnpinnedState<E, R> {
        this.0.map(|value| {
            let value = value.borrow();
            element.set_default_value(renderer, value);
            element.set_value(renderer, value);
        });

        let signal_hook = this.0.to_signal_hook();
        State::new(
            signal_hook,
            RegisterUpdate::register(
                element.on_value_change_element_unpinned(),
                renderer,
                From::from(this),
            ),
        )
    }

    fn render_update<E: FormControlElement<VK, R> + ?Sized, R: ?Sized>(
        this: Self,
        renderer: &mut R,
        element: &mut E,
        State {
            inner: signal_hook,
            non_reactive_state: event_listener,
            #[cfg(debug_assertions)]
            update_times,
        }: &mut <Self::StateKind as FormControlValueStateKind<VK>>::UnpinnedState<E, R>,
    ) {
        #[cfg(debug_assertions)]
        {
            *update_times = 0;
        }

        if this.0.is_signal_of(signal_hook) {
            return;
        }

        this.0.map(|value| {
            let value = value.borrow();
            element.set_default_value(renderer, value);
            element.set_value(renderer, value);
        });

        *signal_hook = this.0.to_signal_hook();

        event_listener.update(
            E::on_value_change_element_unpinned(element),
            renderer,
            From::from(this),
        );
    }
}
