pub use self::element::csr::{Kind, StateWithMemo};
pub use self::Memo as memo;

pub struct Memo<F, Dep>(pub F, pub Dep);

impl<F, Dep> Memo<F, Dep> {
    // The third argument of `run` is `new_dep == memoed_dep`.
    // `true` means new dep equals to the memoed dep.
    fn map_memoed<R>(
        self,
        memoed_dep: &mut Option<Dep>,
        eq: impl FnOnce(&Dep, &Dep) -> bool,
        run: impl FnOnce(F, &mut Dep, bool) -> R,
    ) -> R {
        let Self(f, dep) = self;
        let (memoed_dep, dep_eq_memoed_dep) = match memoed_dep {
            Some(memoed_dep) if eq(memoed_dep, &dep) => {
                *memoed_dep = dep;
                (memoed_dep, true)
            }
            _ => {
                let memoed_dep = memoed_dep.insert(dep);
                (memoed_dep, false)
            }
        };

        run(f, memoed_dep, dep_eq_memoed_dep)
    }
}

/// As [`CsrElement`](crate::CsrElement), this type has the same
/// [`RenderStateKind`](crate::CsrElement::RenderStateKind) as [`Memo<F, Dep>`].
///
/// It will always try to update render state with the element returned by `F`.
///
/// If the `Dep` hasn't been initialized with [`Memo<F, Dep>`]:
///     - if `SKIP_IF_DEP_IS_NONE` is `false`: panic
///     - if `SKIP_IF_DEP_IS_NONE` is `true`: silently skip (`F` is not called and the render state is not updated)
pub struct MemoPhantom<
    F: for<'a> crate::FnOnceOutputElement<&'a Dep>,
    Dep,
    const SKIP_IF_DEP_IS_NONE: bool,
> {
    pub f: F,
    _dep: std::marker::PhantomData<Dep>,
}

impl<F: for<'a> crate::FnOnceOutputElement<&'a Dep>, Dep, const SKIP_IF_DEP_IS_NONE: bool>
    MemoPhantom<F, Dep, SKIP_IF_DEP_IS_NONE>
{
    pub const fn new(f: F) -> Self {
        Self {
            f,
            _dep: std::marker::PhantomData,
        }
    }
}

mod element {
    mod ssr {
        use frender_ssr::SsrElement;

        use crate::FnOnceOutputElement;

        use super::super::Memo;

        impl<
                F: for<'a> FnOnceOutputElement<&'a Dep, OutputElementHtmlChildren = C>,
                Dep,
                C: frender_ssr::html::assert::HtmlChildren,
            > SsrElement for Memo<F, Dep>
        {
            type HtmlChildren = C;

            fn into_html_children(self) -> Self::HtmlChildren {
                (self.0)(&self.1).into_html_children()
            }
        }
    }

    pub(super) mod csr {
        use frender_csr::{render_state::compound::CompoundState, RenderState};
        use frender_html::{
            CsrElement, RenderStateKind, RenderStateKindPinned, RenderStateKindUnpinned,
        };

        use crate::{FnOnce2OutputElement, FnOnceOutputElement};

        use super::super::{
            Memo, MemoAndProvideFirstArgument, MemoPhantom, MemoPhantomAndProvideFirstArgument,
        };

        enum Never {}
        pub struct Kind<K, Dep>(Never, std::marker::PhantomData<(K, Dep)>);

        pub type StateWithMemo<S, Dep> = CompoundState<S, Option<Dep>>;

        impl<K: RenderStateKindPinned, Dep> RenderStateKindPinned for Kind<K, Dep> {
            type RenderState<R: frender_html::RenderHtml + ?Sized> =
                StateWithMemo<K::RenderState<R>, Dep>;
        }

        impl<K: RenderStateKindUnpinned, Dep> RenderStateKindUnpinned for Kind<K, Dep> {
            type UnpinnedRenderState<R: frender_html::RenderHtml + ?Sized> =
                StateWithMemo<K::UnpinnedRenderState<R>, Dep>;
        }

        impl<
                F: for<'a> FnOnceOutputElement<
                    &'a Dep,
                    OutputElementHtmlChildren = C,
                    OutputElementRenderStateKind = K,
                >,
                Dep: PartialEq,
                C: frender_ssr::html::assert::HtmlChildren,
                K: RenderStateKind,
            > CsrElement for Memo<F, Dep>
        {
            type RenderStateKind = Kind<K, Dep>;

            fn render_update_maybe_reposition<Ctx: ?Sized + frender_html::HtmlRenderContext>(
                //
                self,
                render_context: &mut Ctx,
                render_state: std::pin::Pin<
                    &mut StateWithMemo<frender_html::RenderStateOfContext<K, Ctx>, Dep>,
                >,
                force_reposition: bool,
            ) {
                let CompoundState {
                    reactive: render_state,
                    non_reactive: memoed_dep,
                } = render_state.pin_project();
                self.map_memoed(memoed_dep, PartialEq::eq, |f, dep, unchanged| {
                    if unchanged {
                        render_context.map_mut_render_context(|render_context| {
                            render_state.check_and_move_cursor(render_context)
                        });
                    } else {
                        let element = f(dep);
                        element.render_update_maybe_reposition(
                            render_context,
                            render_state,
                            force_reposition,
                        )
                    }
                })
            }

            fn unpinned_render_update_maybe_reposition<
                Ctx: ?Sized + frender_html::HtmlRenderContext,
            >(
                //
                self,
                render_context: &mut Ctx,
                render_state: &mut frender_html::UnpinnedRenderStateOfContext<
                    Self::RenderStateKind,
                    Ctx,
                >,
                force_reposition: bool,
            ) {
                let CompoundState {
                    reactive: render_state,
                    non_reactive: memoed_dep,
                } = render_state;
                self.map_memoed(memoed_dep, PartialEq::eq, |f, dep, unchanged| {
                    if unchanged {
                        render_context.map_mut_render_context(|render_context| {
                            render_state.check_and_move_cursor(render_context)
                        });
                    } else {
                        let element = f(dep);
                        element.unpinned_render_update_maybe_reposition(
                            render_context,
                            render_state,
                            force_reposition,
                        )
                    }
                })
            }
        }

        impl<
                F: for<'a> FnOnce2OutputElement<
                    A,
                    &'a Dep,
                    OutputElementHtmlChildren = C,
                    OutputElementRenderStateKind = K,
                >,
                A,
                Dep: PartialEq,
                C: frender_ssr::html::assert::HtmlChildren,
                K: RenderStateKind,
            > CsrElement for MemoAndProvideFirstArgument<F, A, Dep>
        {
            type RenderStateKind = Kind<K, Dep>;

            frender_html::proxy_csr_element!(|this| this.into_memo());
        }

        fn skip_or_panic<const SKIP: bool>() {
            struct ConstBool<const V: bool>;

            trait ConstNot {
                const NOT: bool;
            }

            impl<const V: bool> ConstNot for ConstBool<V> {
                const NOT: bool = !V;
            }

            if ConstBool::<SKIP>::NOT {
                panic!(
                    "memoed dependency has not been initialized by Memo* but accessed by MemoPhantom*"
                )
            }
        }

        impl<F, Dep, K: RenderStateKind, const SKIP_IF_DEP_IS_NONE: bool> CsrElement
            for MemoPhantom<F, Dep, SKIP_IF_DEP_IS_NONE>
        where
            F: for<'a> FnOnceOutputElement<&'a Dep, OutputElementRenderStateKind = K>,
        {
            type RenderStateKind = crate::memoed::Kind<K, Dep>;

            fn render_update<Ctx: ?Sized + frender_html::HtmlRenderContext>(
                //
                self,
                render_context: &mut Ctx,
                render_state: std::pin::Pin<
                    &mut frender_html::RenderStateOfContext<Self::RenderStateKind, Ctx>,
                >,
            ) where
                Self: Sized,
            {
                let CompoundState {
                    reactive: render_state,
                    non_reactive: dep,
                } = render_state.pin_project();
                if let Some(dep) = dep.as_ref() {
                    (self.f)(dep).render_update(render_context, render_state)
                } else {
                    skip_or_panic::<SKIP_IF_DEP_IS_NONE>()
                }
            }

            fn render_update_force_reposition<Ctx: ?Sized + frender_html::HtmlRenderContext>(
                //
                self,
                render_context: &mut Ctx,
                render_state: std::pin::Pin<
                    &mut frender_html::RenderStateOfContext<Self::RenderStateKind, Ctx>,
                >,
            ) where
                Self: Sized,
            {
                let CompoundState {
                    reactive: render_state,
                    non_reactive: dep,
                } = render_state.pin_project();
                if let Some(dep) = dep.as_ref() {
                    (self.f)(dep).render_update_force_reposition(render_context, render_state)
                } else {
                    skip_or_panic::<SKIP_IF_DEP_IS_NONE>()
                }
            }

            fn render_update_maybe_reposition<Ctx: ?Sized + frender_html::HtmlRenderContext>(
                //
                self,
                render_context: &mut Ctx,
                render_state: std::pin::Pin<
                    &mut frender_html::RenderStateOfContext<Self::RenderStateKind, Ctx>,
                >,
                force_reposition: bool,
            ) {
                let CompoundState {
                    reactive: render_state,
                    non_reactive: dep,
                } = render_state.pin_project();
                if let Some(dep) = dep.as_ref() {
                    (self.f)(dep).render_update_maybe_reposition(
                        render_context,
                        render_state,
                        force_reposition,
                    )
                } else {
                    skip_or_panic::<SKIP_IF_DEP_IS_NONE>()
                }
            }

            fn unpinned_render_update<Ctx: ?Sized + frender_html::HtmlRenderContext>(
                //
                self,
                render_context: &mut Ctx,
                render_state: &mut frender_html::UnpinnedRenderStateOfContext<
                    Self::RenderStateKind,
                    Ctx,
                >,
            ) where
                Self: Sized,
            {
                let CompoundState {
                    reactive: render_state,
                    non_reactive: dep,
                } = render_state;

                if let Some(dep) = dep.as_ref() {
                    (self.f)(dep).unpinned_render_update(render_context, render_state)
                } else {
                    skip_or_panic::<SKIP_IF_DEP_IS_NONE>()
                }
            }

            fn unpinned_render_update_force_reposition<
                Ctx: ?Sized + frender_html::HtmlRenderContext,
            >(
                //
                self,
                render_context: &mut Ctx,
                render_state: &mut frender_html::UnpinnedRenderStateOfContext<
                    Self::RenderStateKind,
                    Ctx,
                >,
            ) where
                Self: Sized,
            {
                let CompoundState {
                    reactive: render_state,
                    non_reactive: dep,
                } = render_state;

                if let Some(dep) = dep.as_ref() {
                    (self.f)(dep)
                        .unpinned_render_update_force_reposition(render_context, render_state)
                } else {
                    skip_or_panic::<SKIP_IF_DEP_IS_NONE>()
                }
            }

            fn unpinned_render_update_maybe_reposition<
                Ctx: ?Sized + frender_html::HtmlRenderContext,
            >(
                //
                self,
                render_context: &mut Ctx,
                render_state: &mut frender_html::UnpinnedRenderStateOfContext<
                    Self::RenderStateKind,
                    Ctx,
                >,
                force_reposition: bool,
            ) {
                let CompoundState {
                    reactive: render_state,
                    non_reactive: dep,
                } = render_state;

                if let Some(dep) = dep.as_ref() {
                    (self.f)(dep).unpinned_render_update_maybe_reposition(
                        render_context,
                        render_state,
                        force_reposition,
                    )
                } else {
                    skip_or_panic::<SKIP_IF_DEP_IS_NONE>()
                }
            }
        }

        impl<F, V, Dep, K: RenderStateKind, const SKIP_IF_DEP_IS_NONE: bool> CsrElement
            for MemoPhantomAndProvideFirstArgument<F, V, Dep, SKIP_IF_DEP_IS_NONE>
        where
            F: for<'a> FnOnce2OutputElement<V, &'a Dep, OutputElementRenderStateKind = K>,
        {
            type RenderStateKind = crate::memoed::Kind<K, Dep>;

            frender_html::proxy_csr_element!(|this| this.f);
        }
    }
}

pub struct MemoAndProvideFirstArgument<F: for<'a> crate::FnOnce2OutputElement<A, &'a Dep>, A, Dep>(
    pub F,
    pub A,
    pub Dep,
);

impl<F, A, Dep> MemoAndProvideFirstArgument<F, A, Dep>
where
    F: for<'a> crate::FnOnce2OutputElement<A, &'a Dep>,
{
    fn into_f_and_dep(
        self,
    ) -> (
        impl FnOnce(&Dep) -> <F as crate::FnOnce2OutputElement<A, &Dep>>::OutputElement,
        Dep,
    ) {
        let Self(f, arg, dep) = self;

        (move |dep| f(arg, dep), dep)
    }

    pub fn into_memo(
        self,
    ) -> Memo<impl FnOnce(&Dep) -> <F as crate::FnOnce2OutputElement<A, &Dep>>::OutputElement, Dep>
    {
        let (f, dep) = self.into_f_and_dep();
        Memo(f, dep)
    }
}

/// Works like [`MemoPhantom`]
pub struct MemoPhantomAndProvideFirstArgument<
    F: for<'a> crate::FnOnce2OutputElement<A, &'a Dep>,
    A,
    Dep,
    const SKIP_IF_DEP_IS_NONE: bool,
> {
    pub f: F,
    pub first_argument: A,
    _dep: std::marker::PhantomData<Dep>,
}

impl<
        F: for<'a> crate::FnOnce2OutputElement<V, &'a Dep>,
        V,
        Dep,
        const SKIP_IF_DEP_IS_NONE: bool,
    > MemoPhantomAndProvideFirstArgument<F, V, Dep, SKIP_IF_DEP_IS_NONE>
{
    pub const fn new(f: F, first_argument: V) -> Self {
        Self {
            f,
            first_argument,
            _dep: std::marker::PhantomData,
        }
    }

    fn into_f(
        self,
    ) -> impl FnOnce(&Dep) -> <F as crate::FnOnce2OutputElement<V, &Dep>>::OutputElement {
        move |dep: &_| (self.f)(self.first_argument, dep)
    }

    pub fn into_memo_phantom(
        self,
    ) -> MemoPhantom<
        impl FnOnce(&Dep) -> <F as crate::FnOnce2OutputElement<V, &Dep>>::OutputElement,
        Dep,
        SKIP_IF_DEP_IS_NONE,
    > {
        MemoPhantom::new(self.into_f())
    }
}
