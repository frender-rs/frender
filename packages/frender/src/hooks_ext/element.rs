pub use self::{with_fn::WithFn, with_memo::MemoCallWithRef, with_to_element::WithToElement};

use std::{marker::PhantomData, pin::Pin, task::Poll};

use frender_csr::RenderState;
use frender_hook_element::state::{
    CursorPlaceholderWithRenderStatePinProject, MaybeIntoPollNextUpdate, MountState,
};

use frender_html::{
    dom::behaviors::{Node, NodeRenderSelf, NodeWithRenderContextAfterSelf},
    Element, RenderHtml, RenderStateKind, RenderStateKindPinned, RenderStateKindUnpinned,
    RenderStateOfContext, UnpinnedRenderStateOfContext,
};
use frender_ssr::{html::assert::HtmlChildren, SsrElement};
use hooks::{HookPollNextUpdate, HookUnmount, ShareValue, Signal, SignalHook};

mod with_fn;
mod with_memo;
mod with_to_element;

pub trait MutCsrElementWithValueUsingMapToElement {}

pub trait MapToElement<V: ?Sized> {
    type RefToElement<'a>: Element<
        RenderStateKind = Self::RefToElementRenderStateKind,
        HtmlChildren = Self::RefToElementHtmlChildren,
    >
    where
        Self: 'a,
        V: 'a;

    type RefToElementHtmlChildren: HtmlChildren;
    type RefToElementRenderStateKind: RenderStateKind;
    fn map_to_element<'a>(&'a mut self, v: &'a V) -> Self::RefToElement<'a>;
}

pub trait MutCsrElementWithValue<V: ?Sized> {
    type RenderStateKind: RenderStateKind;

    fn mut_render_update<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        &mut self,
        value: &V,
        render_context: &mut Ctx,
        render_state: Pin<&mut RenderStateOfContext<Self::RenderStateKind, Ctx>>,
    );

    fn mut_render_update_maybe_reposition<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        &mut self,
        value: &V,
        render_context: &mut Ctx,
        render_state: Pin<&mut RenderStateOfContext<Self::RenderStateKind, Ctx>>,
        force_reposition: bool,
    );

    fn mut_unpinned_render_update<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        &mut self,
        value: &V,
        render_context: &mut Ctx,
        render_state: &mut frender_html::UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>,
    );

    fn mut_unpinned_render_update_maybe_reposition<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        &mut self,
        value: &V,
        render_context: &mut Ctx,
        render_state: &mut frender_html::UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>,
        force_reposition: bool,
    );
}

impl<V: ?Sized, M: MapToElement<V> + MutCsrElementWithValueUsingMapToElement>
    MutCsrElementWithValue<V> for M
{
    type RenderStateKind = M::RefToElementRenderStateKind;

    fn mut_render_update<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        &mut self,
        value: &V,
        render_context: &mut Ctx,
        render_state: Pin<&mut RenderStateOfContext<Self::RenderStateKind, Ctx>>,
    ) {
        self.map_to_element(value)
            .render_update(render_context, render_state)
    }

    fn mut_render_update_maybe_reposition<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        &mut self,
        value: &V,
        render_context: &mut Ctx,
        render_state: Pin<&mut RenderStateOfContext<Self::RenderStateKind, Ctx>>,
        force_reposition: bool,
    ) {
        self.map_to_element(value).render_update_maybe_reposition(
            render_context,
            render_state,
            force_reposition,
        )
    }

    fn mut_unpinned_render_update<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        &mut self,
        value: &V,
        render_context: &mut Ctx,
        render_state: &mut frender_html::UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>,
    ) {
        self.map_to_element(value)
            .unpinned_render_update(render_context, render_state)
    }

    fn mut_unpinned_render_update_maybe_reposition<
        Ctx: ?Sized + frender_html::HtmlRenderContext,
    >(
        &mut self,
        value: &V,
        render_context: &mut Ctx,
        render_state: &mut frender_html::UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>,
        force_reposition: bool,
    ) {
        self.map_to_element(value)
            .unpinned_render_update_maybe_reposition(render_context, render_state, force_reposition)
    }
}

pub trait IntoMutElementWithValue<V: ?Sized> {
    type MutCsrElementWithValue: MutCsrElementWithValue<V>;

    type HtmlChildren: frender_ssr::html::assert::HtmlChildren;
    fn into_html_children_with_value(self, value: &V) -> Self::HtmlChildren;

    fn render_update_maybe_reposition_with_value_and_into<
        Ctx: ?Sized + frender_html::HtmlRenderContext,
    >(
        self,
        value: &V,
        render_context: &mut Ctx,
        render_state: Pin<
            &mut RenderStateOfContext<
                <Self::MutCsrElementWithValue as MutCsrElementWithValue<V>>::RenderStateKind,
                Ctx,
            >,
        >,
        force_reposition: bool,
    ) -> Self::MutCsrElementWithValue;

    fn unpinned_render_update_maybe_reposition_with_value_and_into<
        Ctx: ?Sized + frender_html::HtmlRenderContext,
    >(
        self,
        value: &V,
        render_context: &mut Ctx,
        render_state: &mut UnpinnedRenderStateOfContext<
            <Self::MutCsrElementWithValue as MutCsrElementWithValue<V>>::RenderStateKind,
            Ctx,
        >,
        force_reposition: bool,
    ) -> Self::MutCsrElementWithValue;
}

impl<V: ?Sized, M: MapToElement<V> + MutCsrElementWithValueUsingMapToElement>
    IntoMutElementWithValue<V> for M
{
    type MutCsrElementWithValue = M;
    type HtmlChildren = M::RefToElementHtmlChildren;

    fn into_html_children_with_value(mut self, value: &V) -> Self::HtmlChildren {
        self.map_to_element(value).into_html_children()
    }

    fn render_update_maybe_reposition_with_value_and_into<
        Ctx: ?Sized + frender_html::HtmlRenderContext,
    >(
        mut self,
        value: &V,
        render_context: &mut Ctx,
        render_state: Pin<
            &mut RenderStateOfContext<
                <Self::MutCsrElementWithValue as MutCsrElementWithValue<V>>::RenderStateKind,
                Ctx,
            >,
        >,
        force_reposition: bool,
    ) -> Self::MutCsrElementWithValue {
        self.map_to_element(value).render_update_maybe_reposition(
            render_context,
            render_state,
            force_reposition,
        );
        self
    }

    fn unpinned_render_update_maybe_reposition_with_value_and_into<
        Ctx: ?Sized + frender_html::HtmlRenderContext,
    >(
        mut self,
        value: &V,
        render_context: &mut Ctx,
        render_state: &mut UnpinnedRenderStateOfContext<
            <Self::MutCsrElementWithValue as MutCsrElementWithValue<V>>::RenderStateKind,
            Ctx,
        >,
        force_reposition: bool,
    ) -> Self::MutCsrElementWithValue {
        self.map_to_element(value)
            .unpinned_render_update_maybe_reposition(
                render_context,
                render_state,
                force_reposition,
            );
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SignalIntoElement<S: ShareValue, F: IntoMutElementWithValue<S::Value>>(pub S, pub F);

mod ssr {
    use super::*;

    impl<S: ShareValue, F> frender_ssr::SsrElement for SignalIntoElement<S, F>
    where
        F: IntoMutElementWithValue<S::Value>,
    {
        type HtmlChildren = F::HtmlChildren;

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
    E: MutCsrElementWithValue<SH::SignalShareValue>;

type CursorPlaceholderWithRenderState<C, S> =
    frender_hook_element::state::CursorPlaceholderWithRenderState<C, (), S>;

impl<SH, E> RenderStateKindUnpinned for Kind<SH, E>
where
    SH: Unpin + SignalHook,
    E: MutCsrElementWithValue<SH::SignalShareValue>,
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
    E: MutCsrElementWithValue<SH::SignalShareValue>,
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

impl<R: ?Sized + RenderHtml, V: ?Sized, F: ?Sized + MutCsrElementWithValue<V>>
    RenderUpdateMapToElement<R, F, V> for RenderUpdateToElementWithPinnedState
{
    type State = RenderStateOfMutElement<F, V, R>;
    fn render_update_to_element(
        f: &mut F,
        value: &V,
        render_context: &mut <R>::RenderContext<'_>,
        render_state: Pin<&mut Self::State>,
    ) {
        f.mut_render_update(value, render_context, render_state)
    }
}

pub enum RenderUpdateToElementWithUnpinnedState {}

impl<R: ?Sized + RenderHtml, V: ?Sized, F: ?Sized + MutCsrElementWithValue<V>>
    RenderUpdateMapToElement<R, F, V> for RenderUpdateToElementWithUnpinnedState
{
    type State = UnpinnedRenderStateOfMutElement<F, V, R>;
    fn render_update_to_element(
        f: &mut F,
        value: &V,
        render_context: &mut <R>::RenderContext<'_>,
        render_state: Pin<&mut Self::State>,
    ) {
        f.mut_unpinned_render_update(value, render_context, render_state.get_mut())
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
    <<E as MutCsrElementWithValue<V>>::RenderStateKind as RenderStateKindPinned>::RenderState<R>;
type UnpinnedRenderStateOfMutElement<E, V, R> =
    <<E as MutCsrElementWithValue<V>>::RenderStateKind as RenderStateKindUnpinned>::UnpinnedRenderState<
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

impl<S: Signal, F> frender_html::Element for SignalIntoElement<S, F>
where
    S::SignalHook: Unpin,
    F: IntoMutElementWithValue<<S as ShareValue>::Value>,
{
    type RenderStateKind = Kind<S::SignalHook, F::MutCsrElementWithValue>;

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
                self.1.render_update_maybe_reposition_with_value_and_into(
                    value,
                    render_context,
                    render_state,
                    force_reposition,
                )
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
                self.1
                    .unpinned_render_update_maybe_reposition_with_value_and_into(
                        value,
                        render_context,
                        render_state,
                        force_reposition,
                    )
            },
            render_context,
            force_reposition,
        )
    }
}
