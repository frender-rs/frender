use std::{marker::PhantomData, pin::Pin};

use frender_csr::{Element, RenderState};
use hooks_core::{HookPollNextUpdate, HookUnmount};
use lazy_pinned::LazyPinned;

use crate::fn_wrapper;

mod prelude_names {
    pub(super) use std::pin::Pin;

    pub(super) use hooks_core::{HookPollNextUpdate, HookUnmount};

    pub(super) use super::{fn_wrapper, prelude_names, FnHookElement};
}

pub mod new_fn_hook_element {
    use super::prelude_names::*;

    #[inline]
    #[cfg(feature = "csr")]
    pub fn csr<HookData: HookPollNextUpdate + HookUnmount + Default, U, E: frender_csr::Element>(
        use_hook: U,
    ) -> FnHookElement<HookData, fn_wrapper::FnMutOutputElement<U>>
    where
        U: FnMut(Pin<&mut HookData>) -> E,
    {
        FnHookElement {
            use_hook: fn_wrapper::FnMutOutputElement(use_hook),
            _phantom: std::marker::PhantomData,
        }
    }

    #[inline]
    #[cfg(feature = "ssr")]
    pub fn ssr<HookData: HookPollNextUpdate + HookUnmount + Default, U, E: frender_ssr::Element>(
        use_hook: U,
    ) -> FnHookElement<HookData, fn_wrapper::FnMutOutputElement<U>>
    where
        U: FnMut(Pin<&mut HookData>) -> E,
    {
        FnHookElement {
            use_hook: fn_wrapper::FnMutOutputElement(use_hook),
            _phantom: std::marker::PhantomData,
        }
    }

    #[inline]
    #[cfg(all(feature = "csr", feature = "ssr"))]
    pub fn ssr_and_csr<
        HookData: HookPollNextUpdate + HookUnmount + Default,
        U,
        E: frender_csr::Element + frender_ssr::Element,
    >(
        use_hook: U,
    ) -> FnHookElement<HookData, fn_wrapper::FnMutOutputElement<U>>
    where
        U: FnMut(Pin<&mut HookData>) -> E,
    {
        FnHookElement {
            use_hook: fn_wrapper::FnMutOutputElement(use_hook),
            _phantom: std::marker::PhantomData,
        }
    }
}

pub mod with_render_context {
    use super::prelude_names::*;
    use std::marker::PhantomData;

    #[inline]
    #[cfg(feature = "csr")]
    pub fn csr<
        InnerHook: HookPollNextUpdate + HookUnmount + Default,
        U,
        S: frender_csr::RenderState,
    >(
        use_hook: U,
    ) -> FnHookElement<InnerHook, fn_wrapper::FnMutWithRenderContext<U, S>>
    where
        U: for<'a, 'ctx> FnMut(
            Pin<&'a mut InnerHook>,
            crate::csr::CsrRenderContext<'a, 'ctx, S>,
        ) -> crate::csr::Rendered<S>,
    {
        FnHookElement {
            use_hook: fn_wrapper::FnMutWithRenderContext(use_hook, PhantomData),
            _phantom: PhantomData,
        }
    }

    #[inline]
    #[cfg(feature = "ssr")]
    pub fn ssr<
        InnerHook: HookPollNextUpdate + HookUnmount + Default,
        U,
        S: frender_ssr::RenderState,
    >(
        use_hook: U,
    ) -> FnHookElement<InnerHook, fn_wrapper::FnMutWithRenderContext<U, S>>
    where
        U: for<'cs> FnMut(
            Pin<&mut InnerHook>,
            crate::ssr::SsrRenderContext<'cs, S>,
        ) -> crate::ssr::Rendered<'cs, S>,
    {
        FnHookElement {
            use_hook: fn_wrapper::FnMutWithRenderContext(use_hook, PhantomData),
            _phantom: std::marker::PhantomData,
        }
    }

    // TODO: ssr_and_csr
    #[inline]
    #[cfg(all(feature = "csr", feature = "ssr"))]
    pub fn ssr_and_csr<
        InnerHook: HookPollNextUpdate + HookUnmount + Default,
        U,
        E: frender_csr::Element + frender_ssr::Element,
    >(
        use_hook: U,
    ) -> FnHookElement<InnerHook, fn_wrapper::FnMutOutputElement<U>>
    where
        U: FnMut(Pin<&mut InnerHook>) -> E,
    {
        FnHookElement {
            use_hook: fn_wrapper::FnMutOutputElement(use_hook),
            _phantom: std::marker::PhantomData,
        }
    }
}

#[cfg(all(feature = "csr", feature = "ssr"))]
pub use new_fn_hook_element::ssr_and_csr as new_fn_hook_element;

use crate::FnMutHookElementState;

pub struct FnHookElement<HookData: HookPollNextUpdate + HookUnmount + Default, U> {
    use_hook: U,
    _phantom: PhantomData<HookData>,
}

#[cfg(feature = "ssr")]
impl<HookData: HookPollNextUpdate + HookUnmount + Default, U> frender_ssr::Element
    for FnHookElement<HookData, U>
where
    U: crate::ssr::UseSsr<HookData>,
{
    type SsrState = FnMutHookElementState<HookData, U, U::SsrState>;

    fn into_ssr_state(self) -> Self::SsrState {
        FnMutHookElementState::_new(self.use_hook)
    }
}

// TODO: rename
mod impl_prelude {
    pub(super) use hooks_core::{HookPollNextUpdate, HookUnmount};
    pub(super) use lazy_pinned::LazyPinned;

    pub(super) use crate::FnMutHookElementState;

    pub(super) use super::FnHookElement;
}

mod impl_csr {
    use frender_csr::{CsrContext, Element};

    use super::impl_prelude::*;

    use crate::csr;

    impl<HookData: HookPollNextUpdate + HookUnmount, U> Element for FnHookElement<HookData, U>
    where
        HookData: Default,
        U: csr::UseCsr<HookData>,
    {
        type CsrState = FnMutHookElementState<HookData, U, U::CsrState, u8>;

        fn into_csr_state(self, _ctx: &mut CsrContext) -> Self::CsrState {
            // TODO: eagerly initialize for Unpin HookData
            FnMutHookElementState::_new(self.use_hook)
        }

        fn update_csr_state_maybe_reposition(
            self,
            ctx: &mut frender_csr::CsrContext,
            state: std::pin::Pin<&mut Self::CsrState>,
            force_reposition: bool,
        ) where
            Self: Sized,
        {
            state.csr_update(self.use_hook, ctx, force_reposition)
        }
    }
}
