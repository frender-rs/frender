pub use self::{with_fn::WithFn, with_memo::MemoCallWithRef, with_to_element::WithToElement};

use std::{marker::PhantomData, pin::Pin, task::Poll};

use frender_csr::{RenderState, StateUnmount};

use frender_html::{
    dom::behaviors::{Node, NodeRenderSelf, NodeWithRenderContextAfterSelf},
    experimental::{
        PinnedMutRenderStatesOfKind, PinnedRenderStateKind, RenderStates,
        UnpinnedMutRenderStatesOfKind, UnpinnedRenderStateKind, UnpinnedRenderStateKindPollRender,
    },
    ui_handles::CursorPlaceholdersSurrounded,
    CsrElement, HtmlRenderContext, RenderHtml, RenderStateKind,
};
use hooks::{HookPollNextUpdate, HookUnmount, ShareValue, Signal, SignalHook};
use pin_project_lite::pin_project;

use super::form_control::OptionSignalHook;

mod with_fn;
mod with_memo;
mod with_to_element;

pub trait SelfAsMutCsrElementWithValue {}

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

    type OwnedPartIntoCsrElement<'a>: CsrElementRenderUpdate<
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

impl<V: ?Sized, M: AsMutCsrElementWithValue<V> + SelfAsMutCsrElementWithValue>
    IntoAsMutCsrElementWithValue<V> for M
{
    type OwnedPart = ();
    type MutPart = Self;

    fn into_csr_parts(self) -> (Self::MutPart, Self::OwnedPart) {
        (self, ())
    }

    type OwnedPartIntoCsrElement<'a> = M::ElementWithValue<'a>
    where
        V: 'a,
        Self: 'a;

    fn owned_part_into_csr_element<'a>(
        this: &'a mut Self,
        value: &'a V,
        (): Self::OwnedPart,
    ) -> Self::OwnedPartIntoCsrElement<'a> {
        this.as_mut_csr_element_with_value(value)
    }
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
            reactive_state:
                OptionSignalHookAndOther {
                    signal_hook,
                    other: reactive_state,
                    #[cfg(debug_assertions)]
                    update_times,
                    #[cfg(not(debug_assertions))]
                        update_times: (),
                },
        }: frender_html::experimental::UnpinnedMutRenderStatesOfKind<Self, R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        let Some(signal_hook) = signal_hook else {
            return Poll::Ready(());
        };

        match Pin::new(&mut *signal_hook).poll_next_update(cx) {
            Poll::Ready(true) => {
                {
                    let signal = Pin::new(&mut *signal_hook).use_hook(); // mark as seen

                    signal.map(|value| {
                        ui_handle.use_surrounded_render_context(
                            renderer,
                            |ui_handle, render_context| {
                                element
                                    .as_mut_csr_element_with_value(value)
                                    .unpinned_render_update(
                                        render_context,
                                        RenderStates {
                                            ui_handle,
                                            non_reactive_state,
                                            reactive_state,
                                        },
                                    )
                            },
                        )
                    });
                }

                match Pin::new(&mut *signal_hook).poll_next_update(cx) {
                    Poll::Ready(true) => {
                        #[cfg(debug_assertions)]
                        increment_update_times(
                            update_times,
                            |message| {
                                // TODO: warn instead of log
                                renderer.log(message)
                            },
                            std::any::type_name::<Self>(),
                        );

                        // Let next poll decide what to do
                        cx.waker().wake_by_ref();
                        Poll::Pending
                    }
                    Poll::Ready(false) => {
                        <E::ElementWithValueRenderStateKind>::unpinned_poll_render(
                            renderer,
                            RenderStates {
                                ui_handle: ui_handle.surrounded_mut(),
                                non_reactive_state,
                                reactive_state,
                            },
                            cx,
                        )
                    }
                    Poll::Pending => {
                        _ = <E::ElementWithValueRenderStateKind>::unpinned_poll_render(
                            renderer,
                            RenderStates {
                                ui_handle: ui_handle.surrounded_mut(),
                                non_reactive_state,
                                reactive_state,
                            },
                            cx,
                        );
                        Poll::Pending
                    }
                }
            }
            Poll::Ready(false) => <E::ElementWithValueRenderStateKind>::unpinned_poll_render(
                renderer,
                RenderStates {
                    ui_handle: ui_handle.surrounded_mut(),
                    non_reactive_state,
                    reactive_state,
                },
                cx,
            ),
            Poll::Pending => {
                _ = <E::ElementWithValueRenderStateKind>::unpinned_poll_render(
                    renderer,
                    RenderStates {
                        ui_handle: ui_handle.surrounded_mut(),
                        non_reactive_state,
                        reactive_state,
                    },
                    cx,
                );
                Poll::Pending
            }
        }
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

#[cfg(todo)]
impl<S: Signal, F> CsrElement for SignalIntoElement<S, F>
where
    S::SignalHook: Unpin,
    F: IntoAsMutCsrElementWithValue<<S as ShareValue>::Value>,
{
    type RenderStateKind = Kind<S::SignalHook, F::MutPart>;

    #[cfg(todo)]
    fn render_update_maybe_reposition<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        render_state: Pin<&mut RenderStateOfContext<Self::RenderStateKind, Ctx>>,
        force_reposition: bool,
    ) {
        let frender_hook_element::state::StatePinProject {
            mount_state,
            hook_data,
            render_state,
            inner: _,
        } = render_state.pin_project();

        let CursorPlaceholderWithRenderStatePinProject {
            cursor_placeholder_and_data,
            render_state,
        } = render_state.pin_project();

        render_update(
            self.0,
            mount_state,
            hook_data.get_mut(),
            cursor_placeholder_and_data,
            |value, render_context, force_reposition| {
                let (mut mut_part, owned_part) = self.1.into_csr_parts();

                F::owned_part_into_csr_element(&mut mut_part, value, owned_part)
                    .render_update_maybe_reposition(render_context, render_state, force_reposition);

                mut_part
            },
            render_context,
            force_reposition,
        )
    }

    #[cfg(todo)]
    fn unpinned_render_update_maybe_reposition<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        render_state: &mut frender_html::UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>,
        force_reposition: bool,
    ) {
        let frender_hook_element::state::StateMutProject {
            mount_state,
            hook_data,
            render_state:
                CursorPlaceholderWithRenderState {
                    cursor_placeholder_and_data,
                    render_state,
                },
            inner: _,
        } = render_state.as_mut_project();

        render_update(
            self.0,
            mount_state,
            hook_data,
            cursor_placeholder_and_data,
            |value, render_context, force_reposition| {
                let (mut mut_part, owned_part) = self.1.into_csr_parts();

                F::owned_part_into_csr_element(&mut mut_part, value, owned_part)
                    .unpinned_render_update_maybe_reposition(
                        render_context,
                        render_state,
                        force_reposition,
                    );

                mut_part
            },
            render_context,
            force_reposition,
        )
    }

    fn pinned_render_init<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        states: frender_html::experimental::PinMutRenderInitStatesOfKind<
            Self::RenderStateKind,
            Ctx::Renderer,
        >,
    ) -> frender_html::experimental::PinnedUiHandleOfKind<Ctx::Renderer, Self::RenderStateKind>
    {
        todo!()
    }

    fn pinned_render_update<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        states: frender_html::experimental::PinnedMutRenderStatesOfKind<
            Self::RenderStateKind,
            Ctx::Renderer,
        >,
    ) {
        todo!()
    }

    fn unpinned_render_init<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
    ) -> frender_html::experimental::UnpinnedRenderStatesOfKind<Self::RenderStateKind, Ctx::Renderer>
    {
        todo!()
    }

    fn unpinned_render_update<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        states: frender_html::experimental::UnpinnedMutRenderStatesOfKind<
            Self::RenderStateKind,
            Ctx::Renderer,
        >,
    ) {
        todo!()
    }
}
