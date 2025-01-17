use std::pin::Pin;

#[cfg(remove)]
use ui_handle_maybe::UiHandleMaybeUnmounted;

pub trait RendererWithContext {
    type RenderContext;
}

pub trait UiHandle<R: ?Sized> {
    type Maybe: UiHandleMaybeMounted<R, UiHandle = Self> + Default;

    fn into_ui_handle_maybe(self) -> Self::Maybe;

    fn check_and_move_cursor_after_self(&self, render_context: &mut R::RenderContext)
    where
        R: RendererWithContext;
}

pub trait UiHandleWithContext {}

pub trait ReactiveState {}

pub trait UiHandleMaybeMounted<R: ?Sized>: UiHandle<R, Maybe = Self> {
    type UiHandle: UiHandle<R, Maybe = Self>;
    // fn is_mounted(&self) -> bool;

    fn unmount_if_mounted(&mut self, renderer: &mut R);

    fn mount_with_if_unmounted<E>(
        &mut self,
        el: E,
        mount: impl FnOnce(E) -> Self::UiHandle,
        update: impl FnOnce(E, &mut Self::UiHandle),
    );
}

pub trait RenderHtml {}

pub trait HtmlRenderContext {
    type HtmlRenderer: RenderHtml;

    fn renderer_mut(&mut self) -> &mut Self::HtmlRenderer;
}

pub trait CsrElement {
    type UiHandle<R: ?Sized + RenderHtml>: UiHandle<R>;
    type NonReactiveState: Default;
    type ReactiveState: ReactiveState + Default;

    fn render_init<Ctx: ?Sized + HtmlRenderContext>(
        self,
        render_context: &mut Ctx,
        non_reactive_state: Pin<&mut Self::NonReactiveState>,
        reactive_state: Pin<&mut Self::ReactiveState>,
    ) -> Self::UiHandle<Ctx::HtmlRenderer>;

    fn render_update<Ctx: ?Sized + HtmlRenderContext>(
        self,
        render_context: &mut Ctx,
        ui_handle: &mut Self::UiHandle<Ctx::HtmlRenderer>,
        non_reactive_state: Pin<&mut Self::NonReactiveState>,
        reactive_state: Pin<&mut Self::ReactiveState>,
    );
}

#[cfg(remove)]
mod ui_handle_maybe {
    use super::UiHandle;

    enum Repr<UH> {
        BeforeCreated,
        MaybeUnmounted { ui_handle: UH, unmounted: bool },
    }

    impl<UH> Repr<UH> {
        fn mounted_ui_handle(&self) -> Option<&UH> {
            match self {
                Repr::MaybeUnmounted {
                    ui_handle,
                    unmounted: false,
                } => Some(ui_handle),
                _ => None,
            }
        }
    }

    pub struct UiHandleMaybeUnmounted<UH> {
        repr: Repr<UH>,
    }

    impl<UH> UiHandleMaybeUnmounted<UH> {
        const BEFORE_CREATED: Self = UiHandleMaybeUnmounted {
            repr: Repr::BeforeCreated,
        };
        fn new_mounted(ui_handle: UH) -> Self {
            Self {
                repr: Repr::MaybeUnmounted {
                    ui_handle,
                    unmounted: false,
                },
            }
        }

        fn unmount_if_mounted<R: ?Sized>(&mut self, renderer: &mut R)
        where
            UH: UiHandle<R>,
        {
            todo!()
        }
    }

    impl<UH: UiHandle<R>, R: ?Sized> UiHandle<R> for UiHandleMaybeUnmounted<UH> {
        fn check_and_move_cursor_after_self(&self, render_context: &mut R::RenderContext)
        where
            R: super::RendererWithContext,
        {
            if let Some(ui_handle) = self.repr.mounted_ui_handle() {
                ui_handle.check_and_move_cursor_after_self(render_context);
            }
        }
    }
}

pub mod option {
    use super::{CsrElement, HtmlRenderContext, RenderHtml, UiHandle, UiHandleMaybeMounted};

    impl<E: CsrElement> CsrElement for Option<E> {
        type UiHandle<R: ?Sized + RenderHtml> = <E::UiHandle<R> as UiHandle<R>>::Maybe;
        type NonReactiveState = E::NonReactiveState;
        type ReactiveState = E::ReactiveState;

        fn render_init<Ctx: ?Sized + super::HtmlRenderContext>(
            self,
            render_context: &mut Ctx,
            non_reactive_state: std::pin::Pin<&mut Self::NonReactiveState>,
            reactive_state: std::pin::Pin<&mut Self::ReactiveState>,
        ) -> Self::UiHandle<Ctx::HtmlRenderer> {
            match self {
                Some(el) => {
                    let ui_handle =
                        el.render_init(render_context, non_reactive_state, reactive_state);

                    ui_handle.into_ui_handle_maybe()
                }
                None => Default::default(),
            }
        }

        fn render_update<Ctx: ?Sized + HtmlRenderContext>(
            self,
            render_context: &mut Ctx,
            ui_handle: &mut Self::UiHandle<Ctx::HtmlRenderer>,
            non_reactive_state: std::pin::Pin<&mut Self::NonReactiveState>,
            reactive_state: std::pin::Pin<&mut Self::ReactiveState>,
        ) {
            match self {
                Some(el) => ui_handle.mount_with_if_unmounted(
                    (el, render_context, non_reactive_state, reactive_state),
                    |(el, render_context, non_reactive_state, reactive_state)| {
                        let ui_handle =
                            el.render_init(render_context, non_reactive_state, reactive_state);
                        ui_handle
                    },
                    |(el, render_context, non_reactive_state, reactive_state), ui_handle| {
                        el.render_update(
                            render_context,
                            ui_handle,
                            non_reactive_state,
                            reactive_state,
                        )
                    },
                ),
                None => ui_handle.unmount_if_mounted(render_context.renderer_mut()),
            }
        }
    }
}
