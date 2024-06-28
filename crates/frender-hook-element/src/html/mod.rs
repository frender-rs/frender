use std::{marker::PhantomData, pin::Pin, task::Poll};

use frender_html::dom::behaviors::{
    Node as _, NodeRenderSelf, NodeWithRenderContextAfterSelf as _,
};
use frender_html::{
    Element, HtmlRenderContext, RenderHtml, RenderState, RenderStateKind, RenderStateKindPinned,
    RenderStateKindUnpinned, RenderStateOfContext,
};
use frender_ssr::html::assert::HtmlChildren;
use hooks_core::{HookPollNextUpdate, HookUnmount};

use crate::state::{
    CursorPlaceholderWithRenderState, CursorPlaceholderWithRenderStatePinProject,
    MaybeIntoPollNextUpdate, MountState,
};

pub trait UseHookRenderUpdate<HookData> {
    type UseHookRenderStateKindPinned: RenderStateKindPinned;

    fn use_hook_render_update<Ctx: HtmlRenderContext + ?Sized>(
        &mut self,
        hook_data: Pin<&mut HookData>,
        render_context: &mut Ctx,
        render_state: Pin<&mut RenderStateOfContext<Self::UseHookRenderStateKindPinned, Ctx>>,
    );
}

pub struct UseHookWithRenderState<U>(pub U);

impl<HookData, U: FnMutOutputElementOfSameKind<HookData>> UseHookRenderUpdate<HookData>
    for UseHookWithRenderState<U>
{
    type UseHookRenderStateKindPinned = U::OutputElementOfSameRenderStateKind;

    fn use_hook_render_update<Ctx: HtmlRenderContext + ?Sized>(
        &mut self,
        hook_data: Pin<&mut HookData>,
        render_context: &mut Ctx,
        render_state: Pin<&mut RenderStateOfContext<Self::UseHookRenderStateKindPinned, Ctx>>,
    ) {
        self.0(hook_data).render_update(render_context, render_state)
    }
}

pub struct UseHookWithUnpinnedRenderState<U>(pub U);

pub struct KindOfUnpinned<K: RenderStateKindUnpinned>(Never, PhantomData<K>);

impl<K: RenderStateKindUnpinned> RenderStateKindPinned for KindOfUnpinned<K> {
    type RenderState<R: RenderHtml + ?Sized> = K::UnpinnedRenderState<R>;
}

impl<HookData, U: FnMutOutputElementOfSameKind<HookData>> UseHookRenderUpdate<HookData>
    for UseHookWithUnpinnedRenderState<U>
{
    type UseHookRenderStateKindPinned = KindOfUnpinned<U::OutputElementOfSameRenderStateKind>;

    fn use_hook_render_update<Ctx: HtmlRenderContext + ?Sized>(
        &mut self,
        hook_data: Pin<&mut HookData>,
        render_context: &mut Ctx,
        render_state: Pin<&mut RenderStateOfContext<Self::UseHookRenderStateKindPinned, Ctx>>,
    ) {
        self.0(hook_data).unpinned_render_update(render_context, render_state.get_mut())
    }
}

pub struct FnHookElement<HookData: HookPollNextUpdate + HookUnmount + Default, U> {
    use_hook: U,
    _phantom: PhantomData<HookData>,
}

enum Never {}
pub struct Kind<
    HookData: HookPollNextUpdate + HookUnmount + Default,
    U: FnMutOutputElementOfSameKind<HookData>,
>(Never, std::marker::PhantomData<(HookData, U)>);

pub trait FnMutOutputElementOfSameKind<HookData>:
    for<'a> FnMutOutputElement<
    Pin<&'a mut HookData>,
    OutputElementRenderStateKind = Self::OutputElementOfSameRenderStateKind,
    OutputElementHtmlChildren = Self::OutputElementOfSameHtmlChildren,
>
{
    type OutputElementOfSameRenderStateKind: RenderStateKind;
    type OutputElementOfSameHtmlChildren: HtmlChildren;
}

impl<F: ?Sized, HookData, K, C> FnMutOutputElementOfSameKind<HookData> for F
where
    F: for<'a> FnMutOutputElement<
        Pin<&'a mut HookData>,
        OutputElementRenderStateKind = K,
        OutputElementHtmlChildren = C,
    >,
    K: RenderStateKind,
    C: HtmlChildren,
{
    type OutputElementOfSameRenderStateKind = K;
    type OutputElementOfSameHtmlChildren = C;
}

pub trait FnMutOutputElement<Arg>: FnMut(Arg) -> Self::OutputElement {
    type OutputElementRenderStateKind: RenderStateKind;
    type OutputElementHtmlChildren: HtmlChildren;
    type OutputElement: Element<
        RenderStateKind = Self::OutputElementRenderStateKind,
        HtmlChildren = Self::OutputElementHtmlChildren,
    >;
}

impl<Arg, F: ?Sized, E: Element> FnMutOutputElement<Arg> for F
where
    F: FnMut(Arg) -> E,
{
    type OutputElementRenderStateKind = E::RenderStateKind;
    type OutputElementHtmlChildren = E::HtmlChildren;
    type OutputElement = E;
}

#[derive(Debug, Default)]
pub struct PollUseHookRenderUpdate;

pub struct PollNextUpdateAndUseHook<
    'a,
    R: ?Sized + RenderHtml,
    HookData,
    U: UseHookRenderUpdate<HookData>,
> {
    cursor_placeholder: &'a mut R::CursorPlaceholder,
    use_hook: &'a mut U,
    renderer: &'a mut R,
    hook_data: Pin<&'a mut HookData>,
    render_state:
        Pin<&'a mut <U::UseHookRenderStateKindPinned as RenderStateKindPinned>::RenderState<R>>,
}

impl<'a, R: ?Sized + RenderHtml, HookData, U: UseHookRenderUpdate<HookData>> Unpin
    for PollNextUpdateAndUseHook<'a, R, HookData, U>
{
}

impl<'a, R: ?Sized + RenderHtml, HookData, U: UseHookRenderUpdate<HookData>> HookPollNextUpdate
    for PollNextUpdateAndUseHook<'a, R, HookData, U>
where
    HookData: HookPollNextUpdate,
{
    fn poll_next_update(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<bool> {
        let Self {
            cursor_placeholder,
            use_hook,
            renderer,
            hook_data,
            render_state,
        } = self.get_mut();
        let render_state = render_state.as_mut();
        match hook_data.as_mut().poll_next_update(cx) {
            Poll::Ready(true) => {
                cursor_placeholder.with_render_context_after_self(renderer, |render_context| {
                    use_hook.use_hook_render_update(
                        hook_data.as_mut(),
                        render_context,
                        render_state,
                    )
                });
                Poll::Ready(true)
            }
            _ => render_state.poll_render(renderer, cx).map(|()| false),
        }
    }
}

impl<R: ?Sized + RenderHtml, U: UseHookRenderUpdate<HookData>, HookData>
    MaybeIntoPollNextUpdate<
        R,
        HookData,
        CursorPlaceholderWithRenderState<
            R::CursorPlaceholder,
            U,
            <U::UseHookRenderStateKindPinned as RenderStateKindPinned>::RenderState<R>,
        >,
    > for PollUseHookRenderUpdate
where
    HookData: HookPollNextUpdate,
{
    type IntoPollNextUpdate<'a> = PollNextUpdateAndUseHook<'a, R, HookData, U>
    where
        Self: 'a,
        R: 'a,
        HookData: 'a,
        U: 'a;

    fn maybe_into_poll_next_update<'a>(
        self: Pin<&'a mut Self>,
        renderer: &'a mut R,
        hook_data: Pin<&'a mut HookData>,
        render_state: Pin<
            &'a mut CursorPlaceholderWithRenderState<
                R::CursorPlaceholder,
                U,
                <U::UseHookRenderStateKindPinned as RenderStateKindPinned>::RenderState<R>,
            >,
        >,
    ) -> Option<Self::IntoPollNextUpdate<'a>> {
        let CursorPlaceholderWithRenderStatePinProject {
            cursor_placeholder_and_data,
            render_state,
        } = render_state.pin_project();
        match cursor_placeholder_and_data {
            Some((cursor_placeholder, use_hook)) => Some(PollNextUpdateAndUseHook {
                cursor_placeholder,
                use_hook,
                renderer,
                hook_data,
                render_state,
            }),
            None => None,
        }
    }
}

impl<HookData, U> RenderStateKindPinned for Kind<HookData, U>
where
    HookData: HookPollNextUpdate + HookUnmount + Default,
    U: FnMutOutputElementOfSameKind<HookData>,
{
    type RenderState<R: RenderHtml + ?Sized> = crate::state::State<
        HookData,
        CursorPlaceholderWithRenderState<
            R::CursorPlaceholder,
            UseHookWithRenderState<U>,
            <U::OutputElementOfSameRenderStateKind as RenderStateKindPinned>::RenderState<R>,
        >,
        PollUseHookRenderUpdate,
    >;
}

impl<HookData, U> RenderStateKindUnpinned for Kind<HookData, U>
where
    HookData: HookPollNextUpdate + HookUnmount + Default,
    U: FnMutOutputElementOfSameKind<HookData>,
    HookData: Unpin,
{
    type UnpinnedRenderState<R: RenderHtml + ?Sized> = crate::state::State<
        HookData,
        CursorPlaceholderWithRenderState<
            R::CursorPlaceholder,
            UseHookWithUnpinnedRenderState<U>,
            <U::OutputElementOfSameRenderStateKind as RenderStateKindUnpinned>::UnpinnedRenderState<
                R,
            >,
        >,
        PollUseHookRenderUpdate,
    >;
}

impl<HookData, U> Element for FnHookElement<HookData, U>
where
    HookData: HookPollNextUpdate + HookUnmount + Default,
    U: FnMutOutputElementOfSameKind<HookData>,
    HookData: Unpin,
{
    type RenderStateKind = Kind<HookData, U>;

    fn render_update_maybe_reposition<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        self,
        render_context: &mut Ctx,
        render_state: Pin<&mut frender_html::RenderStateOfContext<Self::RenderStateKind, Ctx>>,
        mut force_reposition: bool,
    ) {
        let crate::state::StatePinProject {
            mount_state,
            hook_data,
            render_state,
            inner: _,
        } = render_state.pin_project();
        let CursorPlaceholderWithRenderStatePinProject {
            cursor_placeholder_and_data,
            render_state,
        } = render_state.pin_project();

        let use_hook = if let Some((cp, use_hook)) = cursor_placeholder_and_data {
            force_reposition = force_reposition || matches!(mount_state, MountState::Unmounted);

            render_context.map_mut_render_context(|render_context: &mut _| {
                cp.readd_self(render_context, force_reposition)
            });

            use_hook.0 = self.use_hook;

            use_hook
        } else {
            force_reposition = true;
            let cp = render_context.map_mut_render_context(|render_context: &mut _| {
                NodeRenderSelf::render_self(render_context)
            });
            let (_, use_hook) =
                cursor_placeholder_and_data.insert((cp, UseHookWithRenderState(self.use_hook)));
            use_hook
        };

        (use_hook.0)(hook_data).render_update_maybe_reposition(
            render_context,
            render_state,
            force_reposition,
        );

        *mount_state = MountState::Mounted;
    }

    fn unpinned_render_update_maybe_reposition<Ctx: ?Sized + frender_html::HtmlRenderContext>(
        //
        self,
        render_context: &mut Ctx,
        render_state: &mut frender_html::UnpinnedRenderStateOfContext<Self::RenderStateKind, Ctx>,
        mut force_reposition: bool,
    ) {
        let crate::state::StateMutProject {
            mount_state,
            hook_data,
            render_state,
            inner: _,
        } = render_state.as_mut_project();
        let CursorPlaceholderWithRenderState {
            cursor_placeholder_and_data,
            render_state,
        } = render_state;

        let use_hook = if let Some((cp, use_hook)) = cursor_placeholder_and_data {
            force_reposition = force_reposition || matches!(mount_state, MountState::Unmounted);

            render_context.map_mut_render_context(|render_context: &mut _| {
                cp.readd_self(render_context, force_reposition)
            });

            use_hook.0 = self.use_hook;

            use_hook
        } else {
            force_reposition = true;
            let cp = render_context.map_mut_render_context(|render_context: &mut _| {
                NodeRenderSelf::render_self(render_context)
            });
            let (_, use_hook) = cursor_placeholder_and_data
                .insert((cp, UseHookWithUnpinnedRenderState(self.use_hook)));
            use_hook
        };

        (use_hook.0)(Pin::new(hook_data)).unpinned_render_update_maybe_reposition(
            render_context,
            render_state,
            force_reposition,
        );

        *mount_state = MountState::Mounted;
    }
}

impl<HookData: HookPollNextUpdate + HookUnmount + Default, U> frender_ssr::SsrElement
    for FnHookElement<HookData, U>
where
    U: FnMutOutputElementOfSameKind<HookData>,
{
    type HtmlChildren = U::OutputElementOfSameHtmlChildren;

    fn into_html_children(mut self) -> Self::HtmlChildren {
        let hook_data = HookData::default();
        let hook_data = std::pin::pin!(hook_data); // TODO: compatibility
        (self.use_hook)(hook_data).into_html_children()
    }
}

pub fn new_fn_hook_element<HookData: HookPollNextUpdate + HookUnmount + Default, U>(
    use_hook: U,
) -> FnHookElement<HookData, U>
where
    U: FnMutOutputElementOfSameKind<HookData>,
{
    FnHookElement {
        use_hook,
        _phantom: PhantomData,
    }
}
