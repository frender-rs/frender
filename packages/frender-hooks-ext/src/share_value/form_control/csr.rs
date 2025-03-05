use std::{marker::PhantomData, pin::Pin, task::Poll};

use frender_csr::{StateUnmount, experimental::event_listener::RegisterUpdate};
use frender_form_control::{
    FormControlValueKind,
    csr::{
        FormControlElement, FormControlValue, FormControlValueStateKind, HandleFormControlValue,
    },
};
use hooks::{Hook as _, HookPollNextUpdate, HookUnmount, ShareValue, Signal};

use super::{FromFormControlValue, SignalIntoControlledValue, ToProvideFormControlValue};

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

enum Never {}
pub struct Kind<S>(Never, PhantomData<S>);

impl<S, Val, VK> FormControlValueStateKind<VK> for Kind<S>
where
    VK: FormControlValueKind,
    S: Signal<Value = Val> + 'static,
    S::SignalHook: Unpin,
    Val: FromFormControlValue<VK> + ToProvideFormControlValue<VK>,
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
                    signal.map(set_default_value_and_value(renderer, element));
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

    fn render_remove<E: FormControlElement<VK, R> + ?Sized, R: ?Sized>(
        renderer: &mut R,
        element: &mut E,
        _: &Self::UnpinnedState<E, R>,
    ) {
        element.remove_default_value(renderer);
        element.remove_value(renderer);
    }
}

fn set_default_value_and_value<
    'a,
    V: ToProvideFormControlValue<FK>,
    FK: FormControlValueKind,
    E: FormControlElement<FK, R> + ?Sized,
    R: ?Sized,
>(
    renderer: &'a mut R,
    element: &'a mut E,
) -> impl 'a + FnMut(&V) {
    |value| {
        value.to_provide_form_control_value(|value| {
            element.set_default_value(renderer, value);
        });
        value.to_provide_form_control_value(|value| {
            element.set_value(renderer, value);
        })
    }
}

impl<S, Val, VK> FormControlValue<VK> for SignalIntoControlledValue<S>
where
    VK: FormControlValueKind,
    S: Signal<Value = Val> + 'static,
    S::SignalHook: Unpin,
    Val: FromFormControlValue<VK> + ToProvideFormControlValue<VK>,
{
    type StateKind = Kind<S>;

    fn render_init<E: FormControlElement<VK, R> + ?Sized, R: ?Sized>(
        this: Self,
        renderer: &mut R,
        element: &mut E,
    ) -> <Self::StateKind as FormControlValueStateKind<VK>>::UnpinnedState<E, R> {
        this.0.map(set_default_value_and_value(renderer, element));

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

        this.0.map(set_default_value_and_value(renderer, element));

        *signal_hook = this.0.to_signal_hook();

        event_listener.update(
            E::on_value_change_element_unpinned(element),
            renderer,
            From::from(this),
        );
    }
}
