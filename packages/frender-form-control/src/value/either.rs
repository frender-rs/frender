use std::{marker::PhantomData, pin::Pin};

use frender_dom::StateUnmount;

use super::{FormControlValue, FormControlValueKind, FormControlValueStateKind};

#[derive(Debug, Clone, Copy)]
pub enum EitherFormControlValue<A, B> {
    A(A),
    B(B),
}

enum Never {}
pub struct KindOfEitherFormControlValue<KA, KB>(Never, PhantomData<(KA, KB)>);

pub enum EitherFormControlValueState<A, B> {
    A(A),
    B(B),
}

/// Prefers `A`.
impl<A: Default, B> Default for EitherFormControlValueState<A, B> {
    fn default() -> Self {
        Self::A(A::default())
    }
}

impl<A: Unpin + StateUnmount, B: Unpin + StateUnmount> StateUnmount
    for EitherFormControlValueState<A, B>
{
    fn state_unmount(self: std::pin::Pin<&mut Self>) {
        match self.get_mut() {
            Self::A(this) => Pin::new(this).state_unmount(),
            Self::B(this) => Pin::new(this).state_unmount(),
        }
    }
}

impl<
        KA: FormControlValueStateKind<VK>,
        KB: FormControlValueStateKind<VK>,
        VK: ?Sized + FormControlValueKind,
    > FormControlValueStateKind<VK> for KindOfEitherFormControlValue<KA, KB>
{
    type UnpinnedNonReactiveState<
        E: crate::element::FormControlElement<VK, R> + ?Sized,
        R: ?Sized,
    > = EitherFormControlValueState<
        KA::UnpinnedNonReactiveState<E, R>,
        KB::UnpinnedNonReactiveState<E, R>,
    >;

    type UnpinnedReactiveState =
        EitherFormControlValueState<KA::UnpinnedReactiveState, KB::UnpinnedReactiveState>;

    fn unpinned_poll_render_form_control_value_state<
        E: crate::element::FormControlElement<VK, R> + ?Sized,
        R: ?Sized,
    >(
        renderer: &mut R,
        element: &mut E,
        non_reactive_state: &mut Self::UnpinnedNonReactiveState<E, R>,
        reactive_state: &mut Self::UnpinnedReactiveState,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        match (non_reactive_state, reactive_state) {
            (
                EitherFormControlValueState::A(non_reactive_state),
                EitherFormControlValueState::A(reactive_state),
            ) => KA::unpinned_poll_render_form_control_value_state(
                renderer,
                element,
                non_reactive_state,
                reactive_state,
                cx,
            ),
            (
                EitherFormControlValueState::B(non_reactive_state),
                EitherFormControlValueState::B(reactive_state),
            ) => KB::unpinned_poll_render_form_control_value_state(
                renderer,
                element,
                non_reactive_state,
                reactive_state,
                cx,
            ),
            _ => unreachable!(),
        }
    }
}

impl<A: FormControlValue<VK>, B: FormControlValue<VK>, VK: ?Sized + FormControlValueKind>
    FormControlValue<VK> for EitherFormControlValue<A, B>
{
    type StateKind = KindOfEitherFormControlValue<A::StateKind, B::StateKind>;

    fn render_init<E: crate::element::FormControlElement<VK, R> + ?Sized, R: ?Sized>(
        this: Self,
        renderer: &mut R,
        element: &mut E,
    ) -> (
        <Self::StateKind as super::FormControlValueStateKind<VK>>::UnpinnedNonReactiveState<E, R>,
        <Self::StateKind as super::FormControlValueStateKind<VK>>::UnpinnedReactiveState,
    ) {
        match this {
            Self::A(this) => {
                let (nrs, rs) = A::render_init(this, renderer, element);
                (
                    EitherFormControlValueState::A(nrs),
                    EitherFormControlValueState::A(rs),
                )
            }
            Self::B(this) => {
                let (nrs, rs) = B::render_init(this, renderer, element);
                (
                    EitherFormControlValueState::B(nrs),
                    EitherFormControlValueState::B(rs),
                )
            }
        }
    }

    fn render_update<E: crate::element::FormControlElement<VK, R> + ?Sized, R: ?Sized>(
        this: Self,
        renderer: &mut R,
        element: &mut E,
        non_reactive_state: &mut <Self::StateKind as super::FormControlValueStateKind<VK>>::UnpinnedNonReactiveState<E, R>,
        reactive_state: &mut <Self::StateKind as super::FormControlValueStateKind<VK>>::UnpinnedReactiveState,
    ) {
        match (this, non_reactive_state, reactive_state) {
            (
                Self::A(this),
                EitherFormControlValueState::A(non_reactive_state),
                EitherFormControlValueState::A(reactive_state),
            ) => A::render_update(this, renderer, element, non_reactive_state, reactive_state),
            (
                Self::B(this),
                EitherFormControlValueState::B(non_reactive_state),
                EitherFormControlValueState::B(reactive_state),
            ) => B::render_update(this, renderer, element, non_reactive_state, reactive_state),
            (this, non_reactive_state, reactive_state) => {
                Pin::new(&mut *reactive_state).state_unmount();
                (*non_reactive_state, *reactive_state) = Self::render_init(this, renderer, element);
            }
        }
    }
}

#[cfg(feature = "either")]
mod extern_either {
    use either::Either;

    use crate::value::{FormControlValue, FormControlValueKind};

    use super::EitherFormControlValue;

    fn from_either<A, B>(this: Either<A, B>) -> EitherFormControlValue<A, B> {
        match this {
            Either::Left(this) => EitherFormControlValue::A(this),
            Either::Right(this) => EitherFormControlValue::B(this),
        }
    }

    impl<V: ?Sized + FormControlValueKind, A: FormControlValue<V>, B: FormControlValue<V>>
        FormControlValue<V> for Either<A, B>
    {
        type StateKind = super::KindOfEitherFormControlValue<A::StateKind, B::StateKind>;

        fn render_init<E: crate::element::FormControlElement<V, R> + ?Sized, R: ?Sized>(
            this: Self,
            renderer: &mut R,
            element: &mut E,
        ) -> (
            <Self::StateKind as crate::value::FormControlValueStateKind<V>>::UnpinnedNonReactiveState<E, R>,
            <Self::StateKind as crate::value::FormControlValueStateKind<V>>::UnpinnedReactiveState,
        ){
            EitherFormControlValue::render_init(from_either(this), renderer, element)
        }

        fn render_update<E: crate::element::FormControlElement<V, R> + ?Sized, R: ?Sized>(
            this: Self,
            renderer: &mut R,
            element: &mut E,
            non_reactive_state: &mut <Self::StateKind as crate::value::FormControlValueStateKind<
                V,
            >>::UnpinnedNonReactiveState<E, R>,
            reactive_state: &mut <Self::StateKind as crate::value::FormControlValueStateKind<V>>::UnpinnedReactiveState,
        ) {
            EitherFormControlValue::render_update(
                from_either(this),
                renderer,
                element,
                non_reactive_state,
                reactive_state,
            );
        }
    }
}
