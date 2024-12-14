use std::{marker::PhantomData, pin::Pin};

pub mod csr;
pub mod ssr;

pub struct HookElement<F>(pub F);

/// For the returned `HookElement` to impl CsrElement, HookData needs to impl `HookPollNextUpdate + HookUnmount`.
pub const fn new_fn_hook_element<
    //
    HookData: Default,
    F: for<'hook> FnMut1<Pin<&'hook mut HookData>>,
>(
    f: F,
) -> HookElement<F> {
    HookElement(f)
}

// region: FnMut1
/// Trait alias for `FnMut(Arg) -> Self::_Output`
pub trait FnMut1<Arg>: FnMut(Arg) -> Self::_Output {
    type _Output;
}

impl<Arg, F, Out> FnMut1<Arg> for F
where
    F: FnMut(Arg) -> Out,
{
    type _Output = Out;
}
// endregion

/// Split `fn use_hook` out of [`Hook`] from [`HookPollNextUpdate`] + [`HookUnmount`]
///
/// [`Hook`]: hooks_core::Hook
/// [`HookPollNextUpdate`]: hooks_core::HookPollNextUpdate
/// [`HookUnmount`]: hooks_core::HookUnmount
pub trait UseHookData {
    type Value<'hook>
    where
        Self::HookData: 'hook;
    /// Note that [`UseHookData::HookData`] is an associated type instead of generic type parameter.
    type HookData: ?Sized;

    fn use_hook_data<'hook>(
        &mut self,
        hook_data: Pin<&'hook mut Self::HookData>,
    ) -> Self::Value<'hook>;
}

pub struct FnMutUseHookData<F, HookData: ?Sized>(F, PhantomData<HookData>);

impl<F, HookData> UseHookData for FnMutUseHookData<F, HookData>
where
    F: for<'hook> FnMut1<Pin<&'hook mut HookData>>,
{
    type Value<'hook> = <F as FnMut1<Pin<&'hook mut HookData>>>::_Output
    where
        Self::HookData: 'hook;

    type HookData = HookData;

    fn use_hook_data<'hook>(
        &mut self,
        hook_data: Pin<&'hook mut Self::HookData>,
    ) -> Self::Value<'hook> {
        (self.0)(hook_data)
    }
}
