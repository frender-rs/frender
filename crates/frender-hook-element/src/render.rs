use std::{marker::PhantomData, pin::Pin};

pub enum OptionPin<'a, S> {
    Option(Pin<&'a mut Option<S>>),
    AlwaysSome(Pin<&'a mut S>),
}

impl<'a, S> OptionPin<'a, S> {
    pub(crate) fn as_mut(&mut self) -> OptionPin<'_, S> {
        match self {
            OptionPin::Option(v) => OptionPin::Option(v.as_mut()),
            OptionPin::AlwaysSome(v) => OptionPin::AlwaysSome(v.as_mut()),
        }
    }

    pub(crate) fn as_pin_mut(self) -> Option<Pin<&'a mut S>> {
        match self {
            OptionPin::Option(v) => v.as_pin_mut(),
            OptionPin::AlwaysSome(v) => Some(v),
        }
    }

    pub(crate) fn set(self, value: S) {
        match self {
            OptionPin::Option(mut v) => v.set(Some(value)),
            OptionPin::AlwaysSome(mut v) => v.set(value),
        }
    }
}

pub struct RenderWith<
    F: FnOnce(csr::CsrRenderContext<'_, '_, S>) -> crate::csr::Rendered<S>,
    S: frender_csr::RenderState,
> {
    f: F,
    _state: PhantomData<S>,
}

#[allow(non_snake_case)]
pub fn RenderWith<
    F: FnOnce(csr::CsrRenderContext<'_, '_, S>) -> crate::csr::Rendered<S>,
    S: frender_csr::RenderState,
>(
    f: F,
) -> RenderWith<F, S> {
    RenderWith {
        f,
        _state: PhantomData,
    }
}

pub mod csr {
    use std::{marker::PhantomData, pin::Pin};

    use frender_csr::{CsrContext, Element, RenderState};

    use crate::OptionPin;

    pin_project_lite::pin_project!(
        pub struct State<S: RenderState> {
            #[pin]
            pub(super) inner: Option<S>,
        }
    );

    impl<S: RenderState> RenderState for State<S> {
        fn unmount(self: Pin<&mut Self>) {
            _ = self.project().inner.as_pin_mut().map(S::unmount)
        }

        fn state_unmount(self: Pin<&mut Self>) {
            _ = self.project().inner.as_pin_mut().map(S::state_unmount)
        }

        fn poll_csr(
            self: Pin<&mut Self>,
            ctx: &mut frender_csr::CsrContext,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<()> {
            self.project()
                .inner
                .as_pin_mut()
                .map_or(std::task::Poll::Ready(()), |state| state.poll_csr(ctx, cx))
        }
    }

    pub struct CsrRenderContext<'a, 'ctx, State: RenderState> {
        context: &'a mut CsrContext<'ctx>,
        state: OptionPin<'a, State>,
        pub(crate) force_reposition: bool,
    }

    impl<'a, 'ctx, State: RenderState> CsrRenderContext<'a, 'ctx, State> {
        pub(super) fn _new(
            context: &'a mut CsrContext<'ctx>,
            state: OptionPin<'a, State>,
            force_reposition: bool,
        ) -> Self {
            Self {
                context,
                state,
                force_reposition,
            }
        }

        pub fn render<E: Element<CsrState = State>>(
            mut self,
            element: E,
        ) -> crate::csr::Rendered<State> {
            if let Some(state) = self.state.as_mut().as_pin_mut() {
                element.update_csr_state_maybe_reposition(
                    self.context,
                    state,
                    self.force_reposition,
                )
            } else {
                self.state.set(element.into_csr_state(self.context))
            }
            crate::csr::Rendered(PhantomData)
        }
    }
}

impl<
        F: FnOnce(csr::CsrRenderContext<'_, '_, S>) -> crate::csr::Rendered<S>,
        S: frender_csr::RenderState + Unpin,
    > frender_csr::Element for RenderWith<F, S>
{
    type CsrState = S;

    fn into_csr_state(self, ctx: &mut frender_csr::CsrContext) -> Self::CsrState {
        let mut state = None;
        let _: crate::csr::Rendered<S> = (self.f)(csr::CsrRenderContext::_new(
            ctx,
            OptionPin::Option(Pin::new(&mut state)),
            false,
        ));

        state.unwrap()
    }

    fn update_csr_state_maybe_reposition(
        self,
        ctx: &mut frender_csr::CsrContext,
        state: std::pin::Pin<&mut Self::CsrState>,
        force_reposition: bool,
    ) {
        let _: crate::csr::Rendered<S> = (self.f)(csr::CsrRenderContext::_new(
            ctx,
            OptionPin::AlwaysSome(state),
            force_reposition,
        ));
    }
}
