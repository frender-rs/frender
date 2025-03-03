pub use self::with_fn::WithFn;
#[cfg(feature = "Memo")]
pub use self::with_memo::MemoCallWithRef;
#[cfg(feature = "ToElement")]
pub use self::with_to_element::WithToElement;

use std::{marker::PhantomData, pin::Pin, task::Poll};

use frender_html::{
    csr::experimental::{
        self, HtmlRenderContext, PinnedRenderStateKind, PinnedRenderStateKindPollRender,
        PinnedStateOfKind, PinnedUiHandleOfKind, RenderHtml, RenderInitPinned,
        UnpinnedRenderStateKind, UnpinnedRenderStateKindPollRender, UnpinnedStateOfKind,
        UnpinnedUiHandleOfKind,
    },
    csr::{CsrElement, RenderStateKind},
    dom::csr::StateUnmount,
    ui_handles::CursorPlaceholdersSurrounded,
};

use hooks::{HookUnmount, ShareValue, Signal, SignalHook};
use pin_project_lite::pin_project;

mod with_fn;
#[cfg(feature = "Memo")]
mod with_memo;
#[cfg(feature = "ToElement")]
mod with_to_element;

pub trait CsrElementRenderUpdate {
    type RenderStateKind: RenderStateKind;

    fn pinned_render_update<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        state: Pin<&mut PinnedStateOfKind<Renderer, Self::RenderStateKind>>,
        ui_handle: &mut PinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    );

    fn unpinned_render_update<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        state: &mut UnpinnedStateOfKind<Renderer, Self::RenderStateKind>,
        ui_handle: &mut UnpinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    );
}

impl<E: CsrElement> CsrElementRenderUpdate for E {
    type RenderStateKind = <E as CsrElement>::RenderStateKind;

    fn pinned_render_update<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        state: Pin<&mut PinnedStateOfKind<Renderer, Self::RenderStateKind>>,
        ui_handle: &mut PinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    ) {
        <E as CsrElement>::pinned_render_update(self, renderer, state, ui_handle)
    }

    fn unpinned_render_update<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        state: &mut UnpinnedStateOfKind<Renderer, Self::RenderStateKind>,
        ui_handle: &mut UnpinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    ) {
        <E as CsrElement>::unpinned_render_update(self, renderer, state, ui_handle)
    }
}

pub trait AsMutCsrElementWithValue<V: ?Sized> {
    type ElementWithValueRenderStateKind: RenderStateKind;

    type ElementWithValue<'a>: CsrElementRenderUpdate<
        RenderStateKind = Self::ElementWithValueRenderStateKind,
    >
    where
        Self: 'a,
        V: 'a;

    fn as_mut_csr_element_with_value<'a>(&'a mut self, value: &'a V) -> Self::ElementWithValue<'a>;
}

pub trait IntoAsMutCsrElementWithValue<V: ?Sized> {
    type OwnedPart;
    type MutPart: AsMutCsrElementWithValue<V>;

    fn into_csr_parts(self) -> (Self::MutPart, Self::OwnedPart);

    type OwnedPartIntoCsrElement<'a>: CsrElement<
        RenderStateKind = <Self::MutPart as AsMutCsrElementWithValue<V>>::ElementWithValueRenderStateKind
    >
    where
        Self: 'a,
        V: 'a;

    fn owned_part_into_csr_element<'a>(
        mut_part: &'a mut Self::MutPart,
        value: &'a V,
        owned_part: Self::OwnedPart,
    ) -> Self::OwnedPartIntoCsrElement<'a>;
}

macro_rules! impl_IntoAsMutCsrElementWithValue_with_Self {
    (
        type Value = $Value:ty;
    ) => {
        type OwnedPart = ();
        type MutPart = Self;

        fn into_csr_parts(self) -> (Self::MutPart, Self::OwnedPart) {
            (self, ())
        }

        type OwnedPartIntoCsrElement<'a> = <Self as $crate::hooks_ext::element::AsMutCsrElementWithValue<$Value>>::ElementWithValue<'a>
        where
            Self: 'a,
            $Value: 'a;

        fn owned_part_into_csr_element<'a>(
            mut_part: &'a mut Self::MutPart,
            value: &'a $Value,
            (): Self::OwnedPart,
        ) -> Self::OwnedPartIntoCsrElement<'a> {
            Self::as_mut_csr_element_with_value(mut_part, value)
        }
    };
}

use impl_IntoAsMutCsrElementWithValue_with_Self;

pub trait IntoHtmlChildrenWithValue<V: ?Sized> {
    type HtmlChildrenWithValue: frender_ssr::html::assert::HtmlChildren;

    fn into_html_children_with_value(self, value: &V) -> Self::HtmlChildrenWithValue;
}

pub trait IntoMutElementWithValue<V: ?Sized>:
    IntoHtmlChildrenWithValue<V> + IntoAsMutCsrElementWithValue<V>
{
}

impl<E: ?Sized, V: ?Sized> IntoMutElementWithValue<V> for E where
    E: IntoHtmlChildrenWithValue<V> + IntoAsMutCsrElementWithValue<V>
{
}

#[derive(Debug, Clone, Copy)]
pub struct SignalIntoElement<S: ShareValue, F>(pub S, pub F);

mod ssr {
    use super::*;

    impl<S: ShareValue, F> frender_ssr::SsrElement for SignalIntoElement<S, F>
    where
        F: IntoMutElementWithValue<S::Value>,
    {
        type HtmlChildren = F::HtmlChildrenWithValue;

        fn into_html_children(self) -> Self::HtmlChildren {
            self.0.map(|s| self.1.into_html_children_with_value(s))
        }
    }
}

pin_project!(
    #[project = StateProj]
    pub struct State<SH, T, NRS> {
        #[pin]
        signal_hook: SH,
        #[pin]
        other: T,
        non_reactive_state: NRS,
        update_times: UpdateTimes,
    }
);

#[cfg(debug_assertions)]
type UpdateTimes = u8;
#[cfg(not(debug_assertions))]
type UpdateTimes = ();

fn reset_update_times(value: &mut UpdateTimes) {
    #[cfg(debug_assertions)]
    {
        *value = 0;
    }

    #[cfg(not(debug_assertions))]
    let () = value;
}

impl<SH, T, NRS> State<SH, T, NRS> {
    fn new(signal_hook: SH, other: T, non_reactive_state: NRS) -> Self {
        Self {
            signal_hook,
            other,
            non_reactive_state,
            #[cfg(debug_assertions)]
            update_times: 0,
            #[cfg(not(debug_assertions))]
            update_times: (),
        }
    }
}

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
                r##"WARNING: SignalIntoElementUpdateStateTooManyTimes.
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

impl<SH: HookUnmount, T: StateUnmount, NRS> StateProj<'_, SH, T, NRS> {
    fn state_unmount(self) {
        let Self {
            signal_hook,
            other,
            non_reactive_state: _,
            update_times,
        } = self;
        signal_hook.unmount();
        other.state_unmount();
        reset_update_times(update_times);
    }
}

impl<SH: HookUnmount, T: StateUnmount, NRS> StateUnmount for State<SH, T, NRS> {
    fn state_unmount(self: Pin<&mut Self>) {
        self.project().state_unmount()
    }
}

trait SignalHookRenderer<V: ?Sized, S: ?Sized, NRS: ?Sized> {
    fn render_with_value(
        &mut self,
        reactive_state: Pin<&mut S>,
        non_reactive_state: &mut NRS,
        value: &V,
    );

    fn poll_render(
        &mut self,
        reactive_state: Pin<&mut S>,
        non_reactive_state: &mut NRS,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()>;

    fn warn(&mut self, message: &str);
}

impl<SH: SignalHook, T, NRS> StateProj<'_, SH, T, NRS> {
    fn poll_render(
        self,
        mut renderer: impl SignalHookRenderer<SH::SignalShareValue, T, NRS>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        let StateProj {
            mut signal_hook,
            other: mut reactive_state,
            non_reactive_state,
            #[cfg(debug_assertions)]
            update_times,
            #[cfg(not(debug_assertions))]
                update_times: (),
        } = self;

        match signal_hook.as_mut().poll_next_update(cx) {
            Poll::Ready(true) => {
                {
                    let signal = signal_hook.as_mut().use_hook(); // mark as seen

                    signal.map(|value| {
                        renderer.render_with_value(
                            reactive_state.as_mut(),
                            non_reactive_state,
                            value,
                        )
                    });
                }

                match signal_hook.as_mut().poll_next_update(cx) {
                    Poll::Ready(true) => {
                        #[cfg(debug_assertions)]
                        increment_update_times(
                            update_times,
                            |msg| renderer.warn(msg),
                            std::any::type_name::<Self>(),
                        );

                        // Let next poll decide what to do
                        cx.waker().wake_by_ref();
                        Poll::Pending
                    }
                    Poll::Ready(false) => {
                        renderer.poll_render(reactive_state, non_reactive_state, cx)
                    }
                    Poll::Pending => {
                        _ = renderer.poll_render(reactive_state, non_reactive_state, cx);
                        Poll::Pending
                    }
                }
            }
            Poll::Ready(false) => renderer.poll_render(reactive_state, non_reactive_state, cx),
            Poll::Pending => {
                _ = renderer.poll_render(reactive_state, non_reactive_state, cx);
                Poll::Pending
            }
        }
    }

    fn poll_render_with_fn_and_data<Data>(
        self,
        data: Data,
        f_render: impl FnMut(&mut Data, Pin<&mut T>, &mut NRS, &SH::SignalShareValue),
        f_poll: impl FnMut(&mut Data, Pin<&mut T>, &mut NRS, &mut std::task::Context<'_>) -> Poll<()>,
        f_warn: impl FnMut(&mut Data, &str),
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        struct Renderer<Data, FRender, FPoll, FWarn> {
            data: Data,
            f_render: FRender,
            f_poll: FPoll,
            f_warn: FWarn,
        }

        impl<
                Data,
                FRender: FnMut(&mut Data, Pin<&mut S>, &mut NRS, &V),
                FPoll: FnMut(&mut Data, Pin<&mut S>, &mut NRS, &mut std::task::Context<'_>) -> Poll<()>,
                FWarn: FnMut(&mut Data, &str),
                V: ?Sized,
                S: ?Sized,
                NRS: ?Sized,
            > SignalHookRenderer<V, S, NRS> for Renderer<Data, FRender, FPoll, FWarn>
        {
            fn render_with_value(
                &mut self,
                reactive_state: Pin<&mut S>,
                non_reactive_state: &mut NRS,
                value: &V,
            ) {
                (self.f_render)(&mut self.data, reactive_state, non_reactive_state, value)
            }

            fn poll_render(
                &mut self,
                reactive_state: Pin<&mut S>,
                non_reactive_state: &mut NRS,
                cx: &mut std::task::Context<'_>,
            ) -> Poll<()> {
                (self.f_poll)(&mut self.data, reactive_state, non_reactive_state, cx)
            }

            fn warn(&mut self, message: &str) {
                (self.f_warn)(&mut self.data, message)
            }
        }

        self.poll_render(
            Renderer {
                data,
                f_render,
                f_poll,
                f_warn,
            },
            cx,
        )
    }
}

enum Never {}
pub struct Kind<SH, E>(Never, std::marker::PhantomData<(SH, E)>)
where
    SH: SignalHook,
    E: AsMutCsrElementWithValue<SH::SignalShareValue>;

impl<SH, E> UnpinnedRenderStateKind for Kind<SH, E>
where
    SH: SignalHook,
    E: AsMutCsrElementWithValue<SH::SignalShareValue>,
    SH: Unpin,
{
    type UnpinnedUiHandle<R: RenderHtml + ?Sized> = CursorPlaceholdersSurrounded<
        R::CursorPlaceholder,
        <KindOfMutElement<E, SH::SignalShareValue> as UnpinnedRenderStateKind>::UnpinnedUiHandle<R>,
    >;

    type UnpinnedState<R: RenderHtml + ?Sized> = State<
        SH,
        <KindOfMutElement<E, SH::SignalShareValue> as UnpinnedRenderStateKind>::UnpinnedState<R>,
        E,
    >;
}

impl<SH, E> UnpinnedRenderStateKindPollRender for Kind<SH, E>
where
    SH: SignalHook,
    E: AsMutCsrElementWithValue<SH::SignalShareValue>,
    SH: Unpin,
{
    fn unpinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        state: &mut Self::UnpinnedState<R>,
        ui_handle: &mut Self::UnpinnedUiHandle<R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        let ui_handle = ui_handle.surrounded_mut();
        Pin::new(state).project().poll_render_with_fn_and_data(
            (renderer, ui_handle),
            |(renderer, ui_handle), reactive_state, element, value: &_| {
                element
                    .as_mut_csr_element_with_value(value)
                    .unpinned_render_update::<R>(renderer, reactive_state.get_mut(), ui_handle)
            },
            |(renderer, ui_handle), reactive_state, _, cx| {
                E::ElementWithValueRenderStateKind::unpinned_poll_render::<R>(
                    renderer,
                    reactive_state.get_mut(),
                    ui_handle,
                    cx,
                )
            },
            |(renderer, _), message| {
                // TODO: warn instead of log
                renderer.log(message)
            },
            cx,
        )
    }
}

pin_project!(
    pub struct PinnedState<SH, T, NRS> {
        #[pin]
        inner: State<SH, Option<T>, NRS>,
    }
);

impl<SH, T, NRS> PinnedState<SH, T, NRS> {
    fn project_state(self: Pin<&mut Self>) -> StateProj<'_, SH, T, NRS> {
        let StateProj {
            signal_hook,
            other,
            non_reactive_state,
            update_times,
        } = self.project().inner.project();
        StateProj {
            signal_hook,
            other: other.as_pin_mut().unwrap(),
            non_reactive_state,
            update_times,
        }
    }
}

impl<SH: HookUnmount, T: StateUnmount, NRS> StateUnmount for PinnedState<SH, T, NRS> {
    fn state_unmount(self: Pin<&mut Self>) {
        self.project_state().state_unmount()
    }
}

impl<SH, E> PinnedRenderStateKind for Kind<SH, E>
where
    SH: SignalHook,
    E: AsMutCsrElementWithValue<SH::SignalShareValue>,
    // SH: Unpin,
{
    type PinnedUiHandle<R: RenderHtml + ?Sized> = CursorPlaceholdersSurrounded<
        R::CursorPlaceholder,
        <KindOfMutElement<E, SH::SignalShareValue> as PinnedRenderStateKind>::PinnedUiHandle<R>,
    >;

    type PinnedState<R: RenderHtml + ?Sized> = PinnedState<
        SH,
        <KindOfMutElement<E, SH::SignalShareValue> as PinnedRenderStateKind>::PinnedState<R>,
        E,
    >;
}

impl<SH, E> PinnedRenderStateKindPollRender for Kind<SH, E>
where
    SH: SignalHook,
    E: AsMutCsrElementWithValue<SH::SignalShareValue>,
    // SH: Unpin,
{
    fn pinned_poll_render<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        state: Pin<&mut Self::PinnedState<R>>,
        ui_handle: &mut Self::PinnedUiHandle<R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        let ui_handle = ui_handle.surrounded_mut();
        state.project_state().poll_render_with_fn_and_data(
            (renderer, ui_handle),
            |(renderer, ui_handle), reactive_state, element, value: &_| {
                element
                    .as_mut_csr_element_with_value(value)
                    .pinned_render_update::<R>(renderer, reactive_state, ui_handle)
            },
            |(renderer, ui_handle), reactive_state, _, cx| {
                E::ElementWithValueRenderStateKind::pinned_poll_render::<R>(
                    renderer,
                    reactive_state,
                    ui_handle,
                    cx,
                )
            },
            |(renderer, _), message| {
                // TODO: warn instead of log
                renderer.log(message)
            },
            cx,
        )
    }
}

#[derive(Debug)]
pub struct SignalHookToElement<U>(std::marker::PhantomData<U>);

impl<U> Default for SignalHookToElement<U> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

type KindOfMutElement<E, V> = <E as AsMutCsrElementWithValue<V>>::ElementWithValueRenderStateKind;

pub struct RenderInit<F, OwnedPart> {
    _f: PhantomData<F>,
    owned_part: OwnedPart,
}

impl<SH, F, Ctx: ?Sized + HtmlRenderContext>
    RenderInitPinned<
        &mut Ctx,
        PinnedState<
            SH,
            PinnedStateOfKind<Ctx::Renderer, KindOfMutElement<F::MutPart, SH::SignalShareValue>>,
            F::MutPart,
        >,
    > for RenderInit<F, F::OwnedPart>
where
    SH: Unpin + SignalHook,
    F: IntoAsMutCsrElementWithValue<SH::SignalShareValue>,
{
    type Output = CursorPlaceholdersSurrounded<
        <Ctx::Renderer as frender_html::dom::csr::render::Render>::CursorPlaceholder,
        <KindOfMutElement<F::MutPart, SH::SignalShareValue> as PinnedRenderStateKind>::PinnedUiHandle<
            Ctx::Renderer,
        >,
    >;

    fn render_init_pinned(
        self,
        render_context: &mut Ctx,
        state: Pin<
            &mut PinnedState<
                SH,
                <KindOfMutElement<F::MutPart, SH::SignalShareValue> as PinnedRenderStateKind>::PinnedState<
                    Ctx::Renderer,
                >,
                F::MutPart,
            >,
        >,
    ) -> Self::Output {
        let Self {
            _f: self::PhantomData,
            owned_part,
        } = self;
        let StateProj {
            signal_hook,
            other: mut reactive_state,
            non_reactive_state: mut_part,
            update_times: _,
        } = state.project().inner.project();
        render_context.map_mut_render_context(|render_context| {
            CursorPlaceholdersSurrounded::surround::<Ctx::Renderer>(
                render_context,
                |render_context| {
                    signal_hook.map(|value| {
                        use frender_html::csr::render::RenderContext as _;
                        let element = F::owned_part_into_csr_element(mut_part, value, owned_part);
                        let (state_init, render_init) =
                            element.pinned_render_init(render_context.renderer_mut());

                        reactive_state.set(Some(state_init));
                        let state = reactive_state.as_pin_mut().unwrap();
                        render_init.render_init_pinned(render_context, state)
                    })
                },
            )
        })
    }
}

impl<S: Signal, F> CsrElement for SignalIntoElement<S, F>
where
    S::SignalHook: Unpin,
    F: IntoAsMutCsrElementWithValue<<S as ShareValue>::Value>,
{
    type RenderStateKind = Kind<S::SignalHook, F::MutPart>;
    type PinnedRenderInit<R: ?Sized + RenderHtml> = RenderInit<F, F::OwnedPart>;

    fn pinned_render_init<Renderer: ?Sized + RenderHtml>(
        //
        self,
        _: &mut Renderer,
    ) -> (
        //
        PinnedStateOfKind<Renderer, Self::RenderStateKind>,
        Self::PinnedRenderInit<Renderer>,
    ) {
        let Self(signal, f) = self;
        let (mut_part, owned_part) = f.into_csr_parts();

        (
            PinnedState {
                inner: State {
                    signal_hook: signal.to_signal_hook(),
                    other: None,
                    non_reactive_state: mut_part,
                    #[cfg(debug_assertions)]
                    update_times: 0,
                    #[cfg(not(debug_assertions))]
                    update_times: (),
                },
            },
            RenderInit {
                _f: PhantomData,
                owned_part,
            },
        )
    }

    fn pinned_render_init_by_reusing<Ctx: ?Sized + HtmlRenderContext>(
        self,
        render_context: &mut Ctx,
        reused_state: Pin<&mut PinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>>,
        unmounted_ui_handle: experimental::PinnedUnmountedUiHandleOfKind<
            Ctx::Renderer,
            Self::RenderStateKind,
        >,
    ) -> PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        let StateProj {
            signal_hook,
            other: state,
            non_reactive_state: mut_part,
            update_times,
        } = reused_state.project_state();
        let signal_hook = signal_hook.get_mut();

        reset_update_times(update_times);

        let Self(signal, f) = self;

        let owned_part;
        (*mut_part, owned_part) = f.into_csr_parts();

        let ui_handle = render_context.map_mut_render_context(|render_context| {
            unmounted_ui_handle.mount_and_map(
                render_context,
                |unmounted_ui_handle, render_context| {
                    signal.map(|value| {
                        let element = F::owned_part_into_csr_element(mut_part, value, owned_part);
                        CsrElement::pinned_render_init_by_reusing(
                            element,
                            render_context,
                            state,
                            unmounted_ui_handle,
                        )
                    })
                },
            )
        });

        *signal_hook = signal.to_signal_hook(); // TODO: signal.update_into_signal_hook?

        ui_handle
    }

    fn pinned_render_update<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        state: Pin<&mut PinnedStateOfKind<Renderer, Self::RenderStateKind>>,
        ui_handle: &mut PinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    ) {
        let StateProj {
            signal_hook,
            other: state,
            non_reactive_state: mut_part,
            update_times,
        } = state.project_state();
        let signal_hook = signal_hook.get_mut();

        reset_update_times(update_times);

        let Self(signal, f) = self;

        let owned_part;
        (*mut_part, owned_part) = f.into_csr_parts();

        if signal.is_signal_of(signal_hook) {
            // f is never considered as changed.
            // The state has been kept up-to-date in poll_render.
            // So we just skip render_update.
            return;
        }

        let ui_handle = ui_handle.surrounded_mut();
        signal.map(|value| {
            let element = F::owned_part_into_csr_element(mut_part, value, owned_part);
            CsrElement::pinned_render_update(element, renderer, state, ui_handle)
        });

        *signal_hook = signal.to_signal_hook(); // TODO: signal.update_into_signal_hook?
    }

    fn unpinned_render_init<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
    ) -> (
        //
        UnpinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>,
        UnpinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>,
    ) {
        let Self(signal, f) = self;
        let (mut mut_part, owned_part) = f.into_csr_parts();
        let (state, ui_handle) = render_context.map_mut_render_context(|render_context| {
            CursorPlaceholdersSurrounded::output_and_surround(render_context, |render_context| {
                signal.map(|value| {
                    let element = F::owned_part_into_csr_element(&mut mut_part, value, owned_part);
                    element.unpinned_render_init(render_context)
                })
            })
        });
        (
            State::new(signal.to_signal_hook(), state, mut_part),
            ui_handle,
        )
    }

    fn unpinned_render_init_by_reusing<Ctx: ?Sized + HtmlRenderContext>(
        self,
        render_context: &mut Ctx,
        State {
            signal_hook,
            other: state,
            non_reactive_state: mut_part,
            update_times,
        }: &mut UnpinnedStateOfKind<Ctx::Renderer, Self::RenderStateKind>,
        unmounted_ui_handle: experimental::UnpinnedUnmountedUiHandleOfKind<
            Ctx::Renderer,
            Self::RenderStateKind,
        >,
    ) -> UnpinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        reset_update_times(update_times);

        let Self(signal, f) = self;

        let owned_part;
        (*mut_part, owned_part) = f.into_csr_parts();

        let ui_handle = render_context.map_mut_render_context(|render_context| {
            unmounted_ui_handle.mount_and_map(
                render_context,
                |unmounted_ui_handle, render_context| {
                    signal.map(|value| {
                        let element = F::owned_part_into_csr_element(mut_part, value, owned_part);
                        CsrElement::unpinned_render_init_by_reusing(
                            element,
                            render_context,
                            state,
                            unmounted_ui_handle,
                        )
                    })
                },
            )
        });

        *signal_hook = signal.to_signal_hook(); // TODO: signal.update_into_signal_hook?

        ui_handle
    }

    fn unpinned_render_update<Renderer: ?Sized + RenderHtml>(
        //
        self,
        renderer: &mut Renderer,
        State {
            signal_hook,
            other: state,
            non_reactive_state: mut_part,
            update_times,
        }: &mut UnpinnedStateOfKind<Renderer, Self::RenderStateKind>,
        ui_handle: &mut UnpinnedUiHandleOfKind<Renderer, Self::RenderStateKind>,
    ) {
        reset_update_times(update_times);

        let Self(signal, f) = self;

        let owned_part;
        (*mut_part, owned_part) = f.into_csr_parts();

        if signal.is_signal_of(signal_hook) {
            // f is never considered as changed.
            // The state has been kept up-to-date in poll_render.
            // So we just skip render_update.
            return;
        }

        let ui_handle = ui_handle.surrounded_mut();
        signal.map(|value| {
            let element = F::owned_part_into_csr_element(mut_part, value, owned_part);
            CsrElement::unpinned_render_update(element, renderer, state, ui_handle)
        });

        *signal_hook = signal.to_signal_hook(); // TODO: signal.update_into_signal_hook?
    }
}
