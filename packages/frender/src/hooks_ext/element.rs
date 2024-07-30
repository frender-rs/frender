pub use self::{with_fn::WithFn, with_memo::MemoCallWithRef, with_to_element::WithToElement};

use std::{marker::PhantomData, pin::Pin, task::Poll};

use frender_csr::RenderState;
use frender_hook_element::state::{
    CursorPlaceholderWithRenderStatePinProject, MaybeIntoPollNextUpdate, MountState,
};

use frender_html::{
    dom::behaviors::{Node, NodeRenderSelf, NodeWithRenderContextAfterSelf},
    CsrElement, RenderHtml, RenderStateKind, RenderStateKindPinned, RenderStateKindUnpinned,
    RenderStateOfContext,
};
use hooks::{HookPollNextUpdate, HookUnmount, ShareValue, Signal, SignalHook};

mod with_fn;
mod with_memo;
mod with_to_element;

pub trait SelfAsMutCsrElementWithValue {}

pub trait AsMutCsrElementWithValue<V: ?Sized> {
    type ElementWithValueRenderStateKind: RenderStateKind;

    type ElementWithValue<'a>: CsrElement<RenderStateKind = Self::ElementWithValueRenderStateKind>
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
pub struct SignalIntoElement<S: ShareValue, F: IntoAsMutCsrElementWithValue<S::Value>>(
    pub S,
    pub F,
);

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

pub struct OptionSignalHookAndOther<SH, F> {
    inner: Option<(SH, F)>,
}

impl<SH, F> Unpin for OptionSignalHookAndOther<SH, F> {}

impl<SH, F> Default for OptionSignalHookAndOther<SH, F> {
    fn default() -> Self {
        Self { inner: None }
    }
}

impl<SH: HookUnmount + Unpin, F> HookUnmount for OptionSignalHookAndOther<SH, F> {
    fn unmount(self: Pin<&mut Self>) {
        if let Some((signal_hook, _)) = &mut self.get_mut().inner {
            SH::unmount(Pin::new(signal_hook))
        }
    }
}

enum Never {}
pub struct Kind<SH, E>(Never, std::marker::PhantomData<(SH, E)>)
where
    SH: Unpin + SignalHook,
    E: AsMutCsrElementWithValue<SH::SignalShareValue>;

type CursorPlaceholderWithRenderState<C, S> =
    frender_hook_element::state::CursorPlaceholderWithRenderState<C, (), S>;

impl<SH, E> RenderStateKindUnpinned for Kind<SH, E>
where
    SH: Unpin + SignalHook,
    E: AsMutCsrElementWithValue<SH::SignalShareValue>,
{
    type UnpinnedRenderState<R: RenderHtml + ?Sized> = frender_hook_element::state::State<
        OptionSignalHookAndOther<SH, E>,
        CursorPlaceholderWithRenderState<
            R::CursorPlaceholder,
            UnpinnedRenderStateOfMutElement<E, SH::SignalShareValue, R>,
        >,
        SignalHookToElement<RenderUpdateToElementWithUnpinnedState>,
    >;
}

impl<SH, E> RenderStateKindPinned for Kind<SH, E>
where
    SH: Unpin + SignalHook,
    E: AsMutCsrElementWithValue<SH::SignalShareValue>,
{
    type RenderState<R: RenderHtml + ?Sized> = frender_hook_element::state::State<
        OptionSignalHookAndOther<SH, E>,
        CursorPlaceholderWithRenderState<
            R::CursorPlaceholder,
            RenderStateOfMutElement<E, SH::SignalShareValue, R>,
        >,
        SignalHookToElement<RenderUpdateToElementWithPinnedState>,
    >;
}

#[derive(Debug, Default)]
struct ToElementWithHookData<T>(T);

#[derive(Debug)]
pub struct SignalHookToElement<U>(std::marker::PhantomData<U>);

impl<U> Default for SignalHookToElement<U> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

pub struct CursorPlaceholderRender<'a, R, SH, F, U>
where
    R: ?Sized + RenderHtml,
    SH: SignalHook,
    F: ?Sized,
    U: RenderUpdateMapToElement<R, F, SH::SignalShareValue>,
{
    renderer: &'a mut R,
    cursor_placeholder: &'a mut R::CursorPlaceholder,
    render_state: Pin<&'a mut U::State>,
    signal_hook: Pin<&'a mut SH>,
    f: &'a mut F,
}

impl<'a, R, SH, F, U> Unpin for CursorPlaceholderRender<'a, R, SH, F, U>
where
    R: ?Sized + RenderHtml,
    SH: SignalHook,
    F: ?Sized,
    U: RenderUpdateMapToElement<R, F, SH::SignalShareValue>,
{
}

impl<'a, R, SH, F, U> HookPollNextUpdate for CursorPlaceholderRender<'a, R, SH, F, U>
where
    R: ?Sized + RenderHtml,
    SH: SignalHook,
    F: ?Sized,
    U: RenderUpdateMapToElement<R, F, SH::SignalShareValue>,
{
    fn poll_next_update(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<bool> {
        let Self {
            renderer,
            cursor_placeholder,
            render_state,
            signal_hook,
            f,
        } = self.get_mut();

        let render_state = render_state.as_mut();

        match signal_hook.as_mut().poll_next_update(cx) {
            Poll::Ready(true) => {
                let signal = signal_hook.as_mut().use_hook(); // mark as seen

                signal.map(|el| {
                    cursor_placeholder.with_render_context_after_self(renderer, |render_context| {
                        U::render_update_to_element(f, el, render_context, render_state)
                    })
                });
                Poll::Ready(true)
            }
            _ => render_state.poll_render(renderer, cx).map(|()| false),
        }
    }
}

pub enum RenderUpdateToElementWithPinnedState {}

impl<R: ?Sized + RenderHtml, V: ?Sized, F: ?Sized + AsMutCsrElementWithValue<V>>
    RenderUpdateMapToElement<R, F, V> for RenderUpdateToElementWithPinnedState
{
    type State = RenderStateOfMutElement<F, V, R>;
    fn render_update_to_element(
        f: &mut F,
        value: &V,
        render_context: &mut <R>::RenderContext<'_>,
        render_state: Pin<&mut Self::State>,
    ) {
        f.as_mut_csr_element_with_value(value)
            .render_update(render_context, render_state)
    }
}

pub enum RenderUpdateToElementWithUnpinnedState {}

impl<R: ?Sized + RenderHtml, V: ?Sized, F: ?Sized + AsMutCsrElementWithValue<V>>
    RenderUpdateMapToElement<R, F, V> for RenderUpdateToElementWithUnpinnedState
{
    type State = UnpinnedRenderStateOfMutElement<F, V, R>;
    fn render_update_to_element(
        f: &mut F,
        value: &V,
        render_context: &mut <R>::RenderContext<'_>,
        render_state: Pin<&mut Self::State>,
    ) {
        f.as_mut_csr_element_with_value(value)
            .unpinned_render_update(render_context, render_state.get_mut())
    }
}

pub trait RenderUpdateMapToElement<R: ?Sized + RenderHtml, F: ?Sized, V: ?Sized> {
    type State: RenderState<R>;

    fn render_update_to_element(
        f: &mut F,
        el: &V,
        render_context: &mut R::RenderContext<'_>,
        render_state: Pin<&mut Self::State>,
    );
}

type RenderStateOfMutElement<E, V, R> =
    <<E as AsMutCsrElementWithValue<V>>::ElementWithValueRenderStateKind as RenderStateKindPinned>::RenderState<R>;
type UnpinnedRenderStateOfMutElement<E, V, R> =
    <<E as AsMutCsrElementWithValue<V>>::ElementWithValueRenderStateKind as RenderStateKindUnpinned>::UnpinnedRenderState<
        R
    >;

impl<R, SH, F, U>
    MaybeIntoPollNextUpdate<
        R,
        OptionSignalHookAndOther<SH, F>,
        CursorPlaceholderWithRenderState<R::CursorPlaceholder, U::State>,
    > for SignalHookToElement<U>
where
    R: ?Sized + RenderHtml,
    SH: SignalHook + Unpin,
    U: RenderUpdateMapToElement<R, F, SH::SignalShareValue>,
{
    type IntoPollNextUpdate<'a> = CursorPlaceholderRender<'a, R, SH, F, U>
    where
        Self: 'a,
        R: 'a,
        SH: 'a,
        F:'a;

    fn maybe_into_poll_next_update<'a>(
        self: Pin<&'a mut Self>,
        renderer: &'a mut R,
        hook_data: Pin<&'a mut OptionSignalHookAndOther<SH, F>>,
        render_state: Pin<&'a mut CursorPlaceholderWithRenderState<R::CursorPlaceholder, U::State>>,
    ) -> Option<Self::IntoPollNextUpdate<'a>> {
        let render_state = render_state.pin_project();
        match (
            &mut hook_data.get_mut().inner,
            render_state.cursor_placeholder_and_data,
        ) {
            (Some((signal_hook, f)), Some((cursor_placeholder, ()))) => {
                Some(CursorPlaceholderRender {
                    renderer,
                    cursor_placeholder,
                    render_state: render_state.render_state,
                    signal_hook: Pin::new(signal_hook),
                    f,
                })
            }
            _ => None,
        }
    }
}

fn use_signal_hook_map<SH: SignalHook, R>(
    sh: Pin<&mut SH>,
    f: impl FnOnce(&SH::SignalShareValue) -> R,
) -> R {
    let signal = hooks::Hook::use_hook(sh);
    signal.map(f)
}

fn render_update<'a, S: Signal, E, Ctx: ?Sized + frender_html::HtmlRenderContext>(
    signal: S,
    mount_state: &'a mut MountState,
    hook_data: &'a mut OptionSignalHookAndOther<S::SignalHook, E>,
    cursor_placeholder_and_data: &'a mut Option<(
        <Ctx::HtmlRenderer as frender_html::dom::render::Render>::CursorPlaceholder,
        (),
    )>,
    update: impl FnOnce(&S::Value, &mut Ctx, bool) -> E,
    render_context: &mut Ctx,
    mut force_reposition: bool,
) where
    S::SignalHook: Unpin,
{
    // mount cursor placeholder
    {
        if let Some((cursor_placeholder, ())) = cursor_placeholder_and_data {
            force_reposition = force_reposition || matches!(mount_state, MountState::Unmounted);
            render_context.map_mut_render_context(|render_context: &mut _| {
                cursor_placeholder.readd_self(render_context, force_reposition)
            });
        } else {
            force_reposition = true;
            let node = render_context.map_mut_render_context(|render_context: &mut _| {
                NodeRenderSelf::render_self(render_context)
            });
            *cursor_placeholder_and_data = Some((node, ()));
        }
    }

    match &mut hook_data.inner {
        Some((signal_hook, other)) if signal.is_signal_of(signal_hook) => {
            // signal hasn't changed. no need to update

            // mark signal as seen if it has a new value, to prevent unnecessary re-render
            use_signal_hook_map(Pin::new(signal_hook), |value| {
                *other = update(value, render_context, force_reposition)
            });
        }
        signal_hook => {
            // new signal
            let mut sh = signal.to_signal_hook();
            let el = use_signal_hook_map(Pin::new(&mut sh), |value| {
                update(value, render_context, force_reposition)
            });
            *signal_hook = Some((sh, el))
        }
    }

    *mount_state = MountState::Mounted;
}

impl<S: Signal, F> frender_html::CsrElement for SignalIntoElement<S, F>
where
    S::SignalHook: Unpin,
    F: IntoAsMutCsrElementWithValue<<S as ShareValue>::Value>,
{
    type RenderStateKind = Kind<S::SignalHook, F::MutPart>;

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
}
