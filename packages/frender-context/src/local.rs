pub use self::{
    csr::{Kind, StateWithContext},
    provided::LocalContextKeyProvided,
};

use std::{cell::RefCell, thread::LocalKey};

use frender_ssr::SsrElement;

#[macro_export]
macro_rules! local_context {
    () => {};
    ($(#[$attr:meta])* $vis:vis static $name:ident: $t:ty; $($rest:tt)*) => {
        $(#[$attr])*
        $vis const $name: $crate::local::LocalContextKey<$t> = {
            $crate::local::__private::thread_local! {
                static CTX: $crate::local::__private::RefCell<$crate::local::__private::Option<$t>> = const { $crate::local::__private::RefCell::new($crate::local::__private::Option::None) };
            }

            $crate::local::LocalContextKey::new(CTX)
        };
        $crate::local_context! { $($rest)* }
    };
}

mod provided {
    use std::{cell::RefCell, thread::LocalKey};

    thread_local!(
        static CURRENT_CONTEXT: RefCell<u8> = const { RefCell::new(1) };
    );

    const CONTEXT: LocalContextKeyProvided<u8> = LocalContextKeyProvided {
        local_key: CURRENT_CONTEXT,
    };

    pub struct LocalContextKeyProvided<T: 'static> {
        local_key: LocalKey<RefCell<T>>,
    }

    impl<T: 'static> LocalContextKeyProvided<T> {
        /// Panics if value is not provided
        pub fn get(&'static self) -> T
        where
            T: Copy,
        {
            self.local_key.with_borrow(|v| *v)
        }

        /// Panics if value is not provided
        pub fn get_cloned(&'static self) -> T
        where
            T: Clone,
        {
            self.local_key.with_borrow(T::clone)
        }

        /// Panics if value is not provided
        pub fn map<R>(&'static self, f: impl FnOnce(&T) -> R) -> R {
            self.local_key.with_borrow(f)
        }

        fn is_same_as(&'static self, other: &'static Self) -> bool {
            std::ptr::eq(&self.local_key, &other.local_key)
        }

        fn provide_value<R>(&'static self, value: &mut T, f: impl FnOnce() -> R) -> R {
            self.local_key
                .with_borrow_mut(|context| std::mem::swap(context, value));

            struct Guard<'a, T: 'static> {
                value: &'a mut T,
                ctx: &'static LocalContextKeyProvided<T>,
            }

            impl<T: 'static> Drop for Guard<'_, T> {
                fn drop(&mut self) {
                    self.ctx
                        .local_key
                        .with_borrow_mut(|context| std::mem::swap(context, self.value))
                }
            }

            let guard = Guard { value, ctx: self };
            let res = f();
            drop(guard);
            res
        }
    }
}

thread_local!(
    static CURRENT_CONTEXT: RefCell<Option<u8>> = const { RefCell::new(None) };
);

const CONTEXT: LocalContextKey<u8> = LocalContextKey {
    local_key: CURRENT_CONTEXT,
};

pub struct LocalContextKey<T: 'static> {
    local_key: LocalKey<RefCell<Option<T>>>,
}

const EXPECT_MSG: &str = "Context value should be provided";

impl<T: 'static> LocalContextKey<T> {
    pub const fn new(local_key: LocalKey<RefCell<Option<T>>>) -> Self {
        Self { local_key }
    }

    pub fn try_get(&'static self) -> Option<T>
    where
        T: Copy,
    {
        self.local_key.with_borrow(|v| *v)
    }

    /// Panics if value is not provided
    pub fn get(&'static self) -> T
    where
        T: Copy,
    {
        self.try_get().expect(EXPECT_MSG)
    }

    pub fn try_get_cloned(&'static self) -> Option<T>
    where
        T: Clone,
    {
        self.local_key.with_borrow(Clone::clone)
    }

    /// Panics if value is not provided
    pub fn get_cloned(&'static self) -> T
    where
        T: Clone,
    {
        self.try_get_cloned().expect(EXPECT_MSG)
    }

    pub fn try_map<R>(&'static self, f: impl FnOnce(&T) -> R) -> Option<R> {
        self.local_key.with_borrow(|v| v.as_ref().map(f))
    }

    /// Panics if value is not provided
    pub fn map<R>(&'static self, f: impl FnOnce(&T) -> R) -> R {
        self.try_map(f).expect(EXPECT_MSG)
    }

    /// Shortcut for <code>CTX.[value](Self::value)(value).[children](ElementWithContext::children)(get_element)</code>
    pub fn provide<E: SsrElement, FE: FnOnce() -> E>(
        &'static self,
        value: T,
        get_element: FE,
    ) -> ElementWithContext<T, Value<T>, FE> {
        ElementWithContext {
            context_key: self,
            into_value: Value(value),
            get_element,
        }
    }

    /// Shortcut for <code>CTX.[get_value](Self::get_value)(get_value).[children](ElementWithContext::children)(get_element)</code>
    pub fn provide_with<F: FnOnce() -> T, E: SsrElement, FE: FnOnce() -> E>(
        &'static self,
        get_value: F,
        get_element: FE,
    ) -> ElementWithContext<T, GetValue<F>, FE> {
        ElementWithContext {
            context_key: self,
            into_value: GetValue(get_value),
            get_element,
        }
    }

    pub fn value(&'static self, value: T) -> ElementWithContext<T, Value<T>, ()> {
        ElementWithContext {
            context_key: self,
            into_value: Value(value),
            get_element: (),
        }
    }

    pub fn get_value<F: FnOnce() -> T>(
        &'static self,
        get_value: F,
    ) -> ElementWithContext<T, GetValue<F>, ()> {
        ElementWithContext {
            context_key: self,
            into_value: GetValue(get_value),
            get_element: (),
        }
    }

    fn is_same_as(&'static self, other: &'static Self) -> bool {
        std::ptr::eq(&self.local_key, &other.local_key)
    }

    fn provide_value<R>(&'static self, value: &mut Option<T>, f: impl FnOnce() -> R) -> R {
        self.local_key
            .with_borrow_mut(|context| std::mem::swap(context, value));

        struct Guard<'a, T: 'static> {
            value: &'a mut Option<T>,
            ctx: &'static LocalContextKey<T>,
        }

        impl<T: 'static> Drop for Guard<'_, T> {
            fn drop(&mut self) {
                self.ctx
                    .local_key
                    .with_borrow_mut(|context| std::mem::swap(context, self.value))
            }
        }

        let guard = Guard { value, ctx: self };
        let res = f();
        drop(guard);
        res
    }
}

// WithContext(value, || element)
//
// MY_CTX.use_context()

pub struct ElementWithContext<T: 'static, F: IntoContextValue<ContextValue = T>, FE> {
    pub context_key: &'static LocalContextKey<T>,
    pub into_value: F,
    pub get_element: FE,
}

impl<T: 'static, F: IntoContextValue<ContextValue = T>> ElementWithContext<T, F, ()> {
    pub fn children<E: SsrElement, FE: FnOnce() -> E>(
        self,
        get_element: FE,
    ) -> ElementWithContext<T, F, FE> {
        let Self {
            context_key,
            into_value,
            get_element: (),
        } = self;
        ElementWithContext {
            context_key,
            into_value,
            get_element,
        }
    }
}

pub trait IntoContextValue {
    type ContextValue;

    fn into_context_value(self) -> Self::ContextValue;
    fn update_context_value_lazily(self, value: &mut Self::ContextValue);
}

pub struct Value<T>(pub T);
pub struct GetValue<F>(pub F);

impl<T> IntoContextValue for Value<T> {
    type ContextValue = T;

    fn into_context_value(self) -> Self::ContextValue {
        self.0
    }

    fn update_context_value_lazily(self, value: &mut Self::ContextValue) {
        *value = self.0;
    }
}

impl<F: FnOnce() -> T, T> IntoContextValue for GetValue<F> {
    type ContextValue = T;

    fn into_context_value(self) -> Self::ContextValue {
        self.0()
    }

    fn update_context_value_lazily(self, _: &mut Self::ContextValue) {
        // Doesn't update context value
    }
}

mod ssr {
    use frender_ssr::SsrElement;

    use super::{ElementWithContext, IntoContextValue};

    impl<T: 'static, F: IntoContextValue<ContextValue = T>, FE: FnOnce() -> E, E: SsrElement>
        SsrElement for ElementWithContext<T, F, FE>
    {
        type HtmlChildren = E::HtmlChildren;

        fn into_html_children(self) -> Self::HtmlChildren {
            let Self {
                context_key,
                into_value,
                get_element,
            } = self;
            let mut value = Some(into_value.into_context_value());
            context_key.provide_value(&mut value, || get_element().into_html_children())
        }
    }
}

mod csr {
    use std::pin::Pin;

    use frender_html::{Element, RenderStateKindPinned, RenderStateKindUnpinned};

    use super::{ElementWithContext, IntoContextValue, LocalContextKey};

    pin_project_lite::pin_project!(
        #[project = StateWithContextProj]
        pub struct StateWithContext<T: 'static, U> {
            ctx: Option<&'static LocalContextKey<T>>,
            value: Option<T>,
            #[pin]
            inner: U,
        }
    );

    mod state {
        use std::{pin::Pin, task::Poll};

        use frender_html::RenderState;

        use super::StateWithContext;

        impl<T: 'static, U: Default> Default for StateWithContext<T, U> {
            fn default() -> Self {
                Self {
                    ctx: None,
                    value: None,
                    inner: Default::default(),
                }
            }
        }

        impl<T: 'static, U> StateWithContext<T, U> {
            fn provide_pin(self: Pin<&mut Self>, f: impl FnOnce(Pin<&mut U>)) {
                let this = self.project();
                if let Some(ctx) = this.ctx {
                    ctx.provide_value(this.value, || f(this.inner))
                }
            }
        }

        impl<T: 'static, S: RenderState<R>, R: ?Sized> RenderState<R> for StateWithContext<T, S> {
            fn unmount(self: Pin<&mut Self>, renderer: &mut R) {
                self.provide_pin(|inner| inner.unmount(renderer))
            }

            fn state_unmount(self: Pin<&mut Self>) {
                self.provide_pin(S::state_unmount)
            }

            fn poll_render(
                self: Pin<&mut Self>,
                renderer: &mut R,
                cx: &mut std::task::Context<'_>,
            ) -> Poll<()> {
                let mut res = Poll::Ready(());
                self.provide_pin(|inner| res = inner.poll_render(renderer, cx));
                res
            }

            fn check_and_move_cursor(&self, render_context: &mut R::RenderContext<'_>)
            where
                R: frender_html::dom::render::RenderWithContext,
            {
                // TODO: value not provided
                self.inner.check_and_move_cursor(render_context)
            }
        }
    }

    enum Never {}
    pub struct Kind<T: 'static, K>(Never, T, K);

    impl<T, K: RenderStateKindUnpinned> RenderStateKindUnpinned for Kind<T, K> {
        type UnpinnedRenderState<R: frender_html::RenderHtml + ?Sized> =
            StateWithContext<T, K::UnpinnedRenderState<R>>;
    }

    impl<T, K: RenderStateKindPinned> RenderStateKindPinned for Kind<T, K> {
        type RenderState<R: frender_html::RenderHtml + ?Sized> =
            StateWithContext<T, K::RenderState<R>>;
    }

    impl<T: 'static, F: IntoContextValue<ContextValue = T>, FE> ElementWithContext<T, F, FE> {
        fn provide_get_element<S, R>(
            //
            self,
            render_state: Pin<&mut StateWithContext<T, S>>,
            f: impl FnOnce(FE, Pin<&mut S>) -> R,
        ) -> R {
            let Self {
                context_key,
                into_value,
                get_element,
            } = self;

            let StateWithContextProj {
                ctx,
                value,
                inner: render_state,
            } = render_state.project();

            let ctx = match (ctx, &mut *value) {
                (Some(ctx), Some(value)) if ctx.is_same_as(context_key) => {
                    into_value.update_context_value_lazily(value);
                    *ctx
                }
                (ctx, value) => {
                    let ctx = ctx.insert(context_key);
                    *value = Some(into_value.into_context_value());
                    *ctx
                }
            };

            ctx.provide_value(value, || f(get_element, render_state))
        }
    }

    impl<T: 'static, F: IntoContextValue<ContextValue = T>, E: Element, FE: FnOnce() -> E> Element
        for ElementWithContext<T, F, FE>
    {
        type RenderStateKind = Kind<T, E::RenderStateKind>;

        fn render_update<Ctx: ?Sized + frender_html::HtmlRenderContext>(
            self,
            render_context: &mut Ctx,
            render_state: Pin<&mut frender_html::RenderStateOfContext<Self::RenderStateKind, Ctx>>,
        ) {
            self.provide_get_element(render_state, |get_element, render_state| {
                get_element().render_update(render_context, render_state)
            })
        }

        fn render_update_force_reposition<Ctx: ?Sized + frender_html::HtmlRenderContext>(
            self,
            render_context: &mut Ctx,
            render_state: Pin<&mut frender_html::RenderStateOfContext<Self::RenderStateKind, Ctx>>,
        ) {
            self.provide_get_element(render_state, |get_element, render_state| {
                get_element().render_update_force_reposition(render_context, render_state)
            })
        }

        fn render_update_maybe_reposition<Ctx: ?Sized + frender_html::HtmlRenderContext>(
            self,
            render_context: &mut Ctx,
            render_state: Pin<&mut frender_html::RenderStateOfContext<Self::RenderStateKind, Ctx>>,
            force_reposition: bool,
        ) {
            self.provide_get_element(render_state, |get_element, render_state| {
                get_element().render_update_maybe_reposition(
                    render_context,
                    render_state,
                    force_reposition,
                )
            })
        }

        fn unpinned_render_update<Ctx: ?Sized + frender_html::HtmlRenderContext>(
            //
            self,
            render_context: &mut Ctx,
            render_state: &mut frender_html::UnpinnedRenderStateOfContext<
                Self::RenderStateKind,
                Ctx,
            >,
        ) {
            self.provide_get_element(Pin::new(render_state), |get_element, render_state| {
                get_element().unpinned_render_update(render_context, render_state.get_mut())
            })
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
        ) {
            self.provide_get_element(Pin::new(render_state), |get_element, render_state| {
                get_element()
                    .unpinned_render_update_force_reposition(render_context, render_state.get_mut())
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
            self.provide_get_element(Pin::new(render_state), |get_element, render_state| {
                get_element().unpinned_render_update_maybe_reposition(
                    render_context,
                    render_state.get_mut(),
                    force_reposition,
                )
            })
        }
    }
}

#[doc(hidden)]
pub mod __private {
    pub use std::{cell::RefCell, option::Option, thread_local};
}

#[cfg(feature = "hooks")]
pub mod hooks {
    use hooks::Signal;

    use super::LocalContextKey;

    pub struct UseSignal<S: 'static + Signal>(&'static LocalContextKey<S>);

    hooks::impl_hook!(
        type For<S: 'static + Signal> = UseSignal<S>;

        fn into_hook(self) -> S::SignalHook {
            self.0.map(|signal| signal.to_signal_hook())
        }

        fn update_hook(self, hook: _) {
            self.0.map(|signal| signal.update_signal_hook(hook))
        }

        fn h(self, hook: S::SignalHookUninitialized) {
            self.0.map(|signal| signal.h_signal_hook(hook))
        }
    );

    impl<S: 'static + Signal> LocalContextKey<S> {
        pub fn use_signal(&'static self) -> UseSignal<S> {
            UseSignal(self)
        }
    }
}

#[cfg(test)]
mod tests {
    local_context!(
        static CTX_U8: u8;
    );

    #[test]
    #[should_panic(expected = "Context value should be provided")]
    fn unprovided() {
        CTX_U8.get();
    }

    #[test]
    fn provided() {
        let res = CTX_U8.provide_value(&mut Some(2), || CTX_U8.get());
        assert_eq!(res, 2);
        assert_eq!(CTX_U8.try_get(), None);
    }
}
