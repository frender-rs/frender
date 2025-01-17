pub use self::{with_fn::WithFn, with_memo::MemoCallWithRef, with_to_element::WithToElement};

use std::{marker::PhantomData, pin::Pin, task::Poll};

use frender_csr::StateUnmount;

use frender_html::{
    dom::ui_handle::UiHandle as _,
    experimental::{
        self, PinMutRenderInitStates, PinnedMutRenderStatesOfKind, PinnedRenderStateKind,
        PinnedRenderStateKindPollRender, RenderStates, UnpinnedMutRenderStatesOfKind,
        UnpinnedRenderStateKind, UnpinnedRenderStateKindPollRender,
    },
    kinds::UiHandleWithNonReactiveState,
    ui_handles::CursorPlaceholdersSurrounded,
    CsrElement, HtmlRenderContext, RenderHtml, RenderStateKind,
};

use hooks::{HookUnmount, ShareValue, Signal, SignalHook};
use pin_project_lite::pin_project;

mod with_fn;
mod with_memo;
mod with_to_element;

pub trait CsrElementRenderUpdate {
    type RenderStateKind: RenderStateKind;

    fn pinned_render_update<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        states: PinnedMutRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    );

    fn unpinned_render_update<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        states: UnpinnedMutRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    );
}

impl<E: CsrElement> CsrElementRenderUpdate for E {
    type RenderStateKind = <E as CsrElement>::RenderStateKind;

    fn pinned_render_update<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        states: PinnedMutRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) {
        <E as CsrElement>::pinned_render_update(self, render_context, states)
    }

    fn unpinned_render_update<Ctx: ?Sized + HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        states: UnpinnedMutRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) {
        <E as CsrElement>::unpinned_render_update(self, render_context, states)
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
    #[project = OptionSignalHookAndOtherProj]
    pub struct OptionSignalHookAndOther<SH, T> {
        #[pin]
        signal_hook: Option<SH>,
        #[pin]
        other: T,
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

impl<SH, T> OptionSignalHookAndOther<SH, T> {
    fn new(signal_hook: SH, other: T) -> Self {
        Self {
            signal_hook: Some(signal_hook),
            other,
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

impl<SH, T: Default> Default for OptionSignalHookAndOther<SH, T> {
    fn default() -> Self {
        Self {
            signal_hook: None,
            other: Default::default(),
            update_times: Default::default(),
        }
    }
}

impl<SH: HookUnmount, T: StateUnmount> StateUnmount for OptionSignalHookAndOther<SH, T> {
    fn state_unmount(self: Pin<&mut Self>) {
        let this = self.project();
        if let Some(signal_hook) = this.signal_hook.as_pin_mut() {
            signal_hook.unmount();
        }

        #[cfg(debug_assertions)]
        {
            *this.update_times = 0;
        }

        this.other.state_unmount();
    }
}

trait OptionSignalHookRenderer<V: ?Sized, S: ?Sized> {
    fn render_with_value(&mut self, reactive_state: Pin<&mut S>, value: &V);

    fn poll_render(
        &mut self,
        reactive_state: Pin<&mut S>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()>;

    fn warn(&mut self, message: &str);
}

impl<SH: SignalHook, T> OptionSignalHookAndOther<SH, T> {
    fn poll_render(
        self: Pin<&mut Self>,
        mut renderer: impl OptionSignalHookRenderer<SH::SignalShareValue, T>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        let OptionSignalHookAndOtherProj {
            signal_hook,
            other: mut reactive_state,
            #[cfg(debug_assertions)]
            update_times,
            #[cfg(not(debug_assertions))]
                update_times: (),
        } = self.project();
        let Some(mut signal_hook) = signal_hook.as_pin_mut() else {
            return Poll::Ready(());
        };

        match signal_hook.as_mut().poll_next_update(cx) {
            Poll::Ready(true) => {
                {
                    let signal = signal_hook.as_mut().use_hook(); // mark as seen

                    signal.map(|value| renderer.render_with_value(reactive_state.as_mut(), value));
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
                    Poll::Ready(false) => renderer.poll_render(reactive_state, cx),
                    Poll::Pending => {
                        _ = renderer.poll_render(reactive_state, cx);
                        Poll::Pending
                    }
                }
            }
            Poll::Ready(false) => renderer.poll_render(reactive_state, cx),
            Poll::Pending => {
                _ = renderer.poll_render(reactive_state, cx);
                Poll::Pending
            }
        }
    }

    fn poll_render_with_fn_and_data<Data>(
        self: Pin<&mut Self>,
        data: Data,
        f_render: impl FnMut(&mut Data, Pin<&mut T>, &SH::SignalShareValue),
        f_poll: impl FnMut(&mut Data, Pin<&mut T>, &mut std::task::Context<'_>) -> Poll<()>,
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
                FRender: FnMut(&mut Data, Pin<&mut S>, &V),
                FPoll: FnMut(&mut Data, Pin<&mut S>, &mut std::task::Context<'_>) -> Poll<()>,
                FWarn: FnMut(&mut Data, &str),
                V: ?Sized,
                S: ?Sized,
            > OptionSignalHookRenderer<V, S> for Renderer<Data, FRender, FPoll, FWarn>
        {
            fn render_with_value(&mut self, reactive_state: Pin<&mut S>, value: &V) {
                (self.f_render)(&mut self.data, reactive_state, value)
            }

            fn poll_render(
                &mut self,
                reactive_state: Pin<&mut S>,
                cx: &mut std::task::Context<'_>,
            ) -> Poll<()> {
                (self.f_poll)(&mut self.data, reactive_state, cx)
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

    type UnpinnedNonReactiveState<R: RenderHtml + ?Sized> = (
        E,
        <KindOfMutElement<E, SH::SignalShareValue> as UnpinnedRenderStateKind>::UnpinnedNonReactiveState<
            R,
        >,
    );

    type UnpinnedReactiveState = OptionSignalHookAndOther<
        SH,
        <KindOfMutElement<E, SH::SignalShareValue> as UnpinnedRenderStateKind>::UnpinnedReactiveState,
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
        RenderStates {
            ui_handle,
            non_reactive_state: (element, non_reactive_state),
            reactive_state,
        }: experimental::UnpinnedMutRenderStatesOfKind<Self, R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        Pin::new(reactive_state).poll_render_with_fn_and_data(
            (renderer, ui_handle, element, non_reactive_state),
            |(renderer, ui_handle, element, non_reactive_state), reactive_state, value: &_| {
                ui_handle.use_surrounded_render_context(renderer, |ui_handle, render_context| {
                    element
                        .as_mut_csr_element_with_value(value)
                        .unpinned_render_update(
                            render_context,
                            RenderStates {
                                ui_handle,
                                non_reactive_state,
                                reactive_state: reactive_state.get_mut(),
                            },
                        )
                })
            },
            |(renderer, ui_handle, _, non_reactive_state), reactive_state, cx| {
                E::ElementWithValueRenderStateKind::unpinned_poll_render::<R>(
                    renderer,
                    RenderStates {
                        ui_handle: ui_handle.surrounded_mut(),
                        non_reactive_state,
                        reactive_state: reactive_state.get_mut(),
                    },
                    cx,
                )
            },
            |(renderer, _, _, _), message| {
                // TODO: warn instead of log
                renderer.log(message)
            },
            cx,
        )
    }
}

impl<SH, E> PinnedRenderStateKind for Kind<SH, E>
where
    SH: SignalHook,
    E: AsMutCsrElementWithValue<SH::SignalShareValue>,
    // SH: Unpin,
{
    type PinnedUiHandle<R: RenderHtml + ?Sized> = UiHandleWithNonReactiveState<
        CursorPlaceholdersSurrounded<
            R::CursorPlaceholder,
            <KindOfMutElement<E, SH::SignalShareValue> as PinnedRenderStateKind>::PinnedUiHandle<R>,
        >,
        E,
    >;

    type PinnedNonReactiveState<R: RenderHtml + ?Sized> =
        <KindOfMutElement<E, SH::SignalShareValue> as PinnedRenderStateKind>::PinnedNonReactiveState<
            R,
        >;

    type PinnedReactiveState = OptionSignalHookAndOther<
        SH,
        <KindOfMutElement<E, SH::SignalShareValue> as PinnedRenderStateKind>::PinnedReactiveState,
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
        RenderStates {
            ui_handle:
                UiHandleWithNonReactiveState {
                    ui_handle,
                    non_reactive_state: element,
                },
            non_reactive_state,
            reactive_state,
        }: experimental::PinnedMutRenderStatesOfKind<Self, R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        reactive_state.poll_render_with_fn_and_data(
            (renderer, ui_handle, element, non_reactive_state),
            |(renderer, ui_handle, element, non_reactive_state), reactive_state, value: &_| {
                ui_handle.use_surrounded_render_context(renderer, |ui_handle, render_context| {
                    element
                        .as_mut_csr_element_with_value(value)
                        .pinned_render_update(
                            render_context,
                            RenderStates {
                                ui_handle,
                                non_reactive_state: non_reactive_state.as_mut(),
                                reactive_state,
                            },
                        )
                })
            },
            |(renderer, ui_handle, _, non_reactive_state), reactive_state, cx| {
                E::ElementWithValueRenderStateKind::pinned_poll_render::<R>(
                    renderer,
                    RenderStates {
                        ui_handle: ui_handle.surrounded_mut(),
                        non_reactive_state: non_reactive_state.as_mut(),
                        reactive_state,
                    },
                    cx,
                )
            },
            |(renderer, _, _, _), message| {
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

fn use_signal_hook_map<SH: SignalHook, R>(
    sh: Pin<&mut SH>,
    f: impl FnOnce(&SH::SignalShareValue) -> R,
) -> R {
    let signal = hooks::Hook::use_hook(sh);
    signal.map(f)
}

impl<S: Signal, F> CsrElement for SignalIntoElement<S, F>
where
    S::SignalHook: Unpin,
    F: IntoAsMutCsrElementWithValue<<S as ShareValue>::Value>,
{
    type RenderStateKind = Kind<S::SignalHook, F::MutPart>;

    fn pinned_render_init<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        PinMutRenderInitStates {
            non_reactive_state,
            reactive_state,
        }: experimental::PinMutRenderInitStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) -> experimental::PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind> {
        let Self(signal, f) = self;
        let (mut mut_part, owned_part) = f.into_csr_parts();
        let mut reactive_state = reactive_state.project();
        let ui_handle = render_context.map_mut_render_context(|render_context| {
            CursorPlaceholdersSurrounded::surround(render_context, |render_context| {
                signal.map(|value| {
                    let element = F::owned_part_into_csr_element(&mut mut_part, value, owned_part);
                    element.pinned_render_init(
                        render_context,
                        PinMutRenderInitStates {
                            non_reactive_state,
                            reactive_state: reactive_state.other,
                        },
                    )
                })
            })
        });

        reactive_state
            .signal_hook
            .set(Some(signal.to_signal_hook()));
        UiHandleWithNonReactiveState {
            ui_handle,
            non_reactive_state: mut_part,
        }
    }

    fn pinned_render_update<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        RenderStates {
            ui_handle:
                UiHandleWithNonReactiveState {
                    ui_handle,
                    non_reactive_state: mut_part,
                },
            non_reactive_state,
            reactive_state,
        }: experimental::PinnedMutRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>,
    ) {
        let reactive_state = reactive_state.project();
        let Some(mut signal_hook) = reactive_state.signal_hook.as_pin_mut() else {
            panic!()
        };

        reset_update_times(reactive_state.update_times);

        let Self(signal, f) = self;

        let owned_part;
        (*mut_part, owned_part) = f.into_csr_parts();

        if signal.is_signal_of(&signal_hook) {
            // f is never considered as changed.
            // The state has been kept up-to-date in poll_render.
            // So we just skip render_update.
            return render_context.map_mut_render_context(|render_context| {
                ui_handle.check_and_move_cursor(render_context)
            });
        }

        render_context.map_mut_render_context(|render_context| {
            ui_handle.map_mut_surrounded_with_render_context(
                render_context,
                |ui_handle, render_context| {
                    signal.map(|value| {
                        let element = F::owned_part_into_csr_element(mut_part, value, owned_part);
                        CsrElement::pinned_render_update(
                            element,
                            render_context,
                            RenderStates {
                                ui_handle,
                                non_reactive_state,
                                reactive_state: reactive_state.other,
                            },
                        )
                    })
                },
            )
        });

        signal_hook.set(signal.to_signal_hook()); // TODO: signal.update_into_signal_hook?
    }

    fn unpinned_render_init<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
    ) -> experimental::UnpinnedRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer> {
        let Self(signal, f) = self;
        let (mut mut_part, owned_part) = f.into_csr_parts();
        let (ui_handle, (non_reactive_state, reactive_state)) = render_context
            .map_mut_render_context(|render_context| {
                CursorPlaceholdersSurrounded::surround_and_output(
                    render_context,
                    |render_context| {
                        let RenderStates {
                            ui_handle,
                            non_reactive_state,
                            reactive_state,
                        } = signal.map(|value| {
                            let element =
                                F::owned_part_into_csr_element(&mut mut_part, value, owned_part);
                            element.unpinned_render_init(render_context)
                        });
                        (ui_handle, (non_reactive_state, reactive_state))
                    },
                )
            });
        RenderStates {
            ui_handle,
            non_reactive_state: (mut_part, non_reactive_state),
            reactive_state: OptionSignalHookAndOther::new(signal.to_signal_hook(), reactive_state),
        }
    }

    fn unpinned_render_update<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        RenderStates {
            ui_handle,
            non_reactive_state: (mut_part, non_reactive_state),
            reactive_state:
                OptionSignalHookAndOther {
                    signal_hook,
                    other: reactive_state,
                    update_times,
                },
        }: experimental::UnpinnedMutRenderStatesOfKind<
            Self::RenderStateKind,
            Ctx::Renderer,
        >,
    ) {
        let Some(signal_hook) = signal_hook else {
            panic!()
        };

        reset_update_times(update_times);

        let Self(signal, f) = self;

        let owned_part;
        (*mut_part, owned_part) = f.into_csr_parts();

        if signal.is_signal_of(signal_hook) {
            // f is never considered as changed.
            // The state has been kept up-to-date in poll_render.
            // So we just skip render_update.
            return render_context.map_mut_render_context(|render_context| {
                ui_handle.check_and_move_cursor(render_context)
            });
        }

        render_context.map_mut_render_context(|render_context| {
            ui_handle.map_mut_surrounded_with_render_context(
                render_context,
                |ui_handle, render_context| {
                    signal.map(|value| {
                        let element = F::owned_part_into_csr_element(mut_part, value, owned_part);
                        CsrElement::unpinned_render_update(
                            element,
                            render_context,
                            RenderStates {
                                ui_handle,
                                non_reactive_state,
                                reactive_state,
                            },
                        )
                    })
                },
            )
        });

        *signal_hook = signal.to_signal_hook(); // TODO: signal.update_into_signal_hook?
    }
}
