use element::IntoContextValue;

pub mod local;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ContextValueNotProvided;

impl std::fmt::Display for ContextValueNotProvided {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Context value not provided")
    }
}

impl std::error::Error for ContextValueNotProvided {}

pub trait ContextKeyInner {
    type Value;
    type Error: std::fmt::Debug;

    fn try_get(&'static self) -> Result<Self::Value, Self::Error>
    where
        Self::Value: Copy;

    /// Panics if value is not provided
    #[inline]
    fn get(&'static self) -> Self::Value
    where
        Self::Value: Copy,
    {
        self.try_get().unwrap()
    }

    fn try_get_cloned(&'static self) -> Result<Self::Value, Self::Error>
    where
        Self::Value: Clone;

    /// Panics if value is not provided
    #[inline]
    fn get_cloned(&'static self) -> Self::Value
    where
        Self::Value: Clone,
    {
        self.try_get_cloned().unwrap()
    }

    fn try_map<R>(&'static self, f: impl FnOnce(&Self::Value) -> R) -> Result<R, Self::Error>;

    /// Panics if value is not provided
    #[inline]
    fn map<R>(&'static self, f: impl FnOnce(&Self::Value) -> R) -> R {
        self.try_map(f).unwrap()
    }

    fn is_same_as(&'static self, other: &'static Self) -> bool;

    type SwapValue;

    fn swap_value(&'static self, value: &mut Self::SwapValue);

    fn make_swap_value(value: Self::Value) -> Self::SwapValue;

    type MaybeContextKeyAndValue: Default + MaybeContextKeyAndValue<ContextKeyInner = Self>;
}

pub trait MaybeContextKeyAndValue {
    type ContextKeyInner: ContextKeyInner + 'static;
    fn maybe_context_key_and_value(
        &mut self,
    ) -> Option<(
        &'static ContextKey<Self::ContextKeyInner>,
        &mut <Self::ContextKeyInner as ContextKeyInner>::SwapValue,
    )>;

    fn same_context_key_or_insert(
        &mut self,
        context_key: &'static ContextKey<Self::ContextKeyInner>,
        into_value: impl IntoContextValue<
            ContextValue = <Self::ContextKeyInner as ContextKeyInner>::Value,
        >,
    ) -> (
        &'static ContextKey<Self::ContextKeyInner>,
        &mut <Self::ContextKeyInner as ContextKeyInner>::SwapValue,
    );
}

pub struct MaybeContextKeyAndValueUnprovided<
    Inner: ContextKeyInner<Error = ContextValueNotProvided> + 'static,
> {
    context_key: Option<&'static ContextKey<Inner>>,
    value: Result<Inner::Value, ContextValueNotProvided>,
}

impl<Inner: ContextKeyInner<Error = ContextValueNotProvided> + 'static> Default
    for MaybeContextKeyAndValueUnprovided<Inner>
{
    fn default() -> Self {
        Self {
            context_key: None,
            value: Err(ContextValueNotProvided),
        }
    }
}

impl<
        T,
        Inner: ContextKeyInner<
                Value = T,
                Error = ContextValueNotProvided,
                SwapValue = Result<T, ContextValueNotProvided>,
            > + 'static,
    > MaybeContextKeyAndValue for MaybeContextKeyAndValueUnprovided<Inner>
{
    type ContextKeyInner = Inner;

    fn maybe_context_key_and_value(
        &mut self,
    ) -> Option<(
        &'static ContextKey<Self::ContextKeyInner>,
        &mut <Self::ContextKeyInner as ContextKeyInner>::SwapValue,
    )> {
        match self {
            Self {
                context_key: Some(ctx),
                value: value @ Ok(_),
            } => Some((ctx, value)),
            _ => None,
        }
    }

    fn same_context_key_or_insert(
        &mut self,
        context_key: &'static ContextKey<Self::ContextKeyInner>,
        into_value: impl IntoContextValue<
            ContextValue = <Self::ContextKeyInner as ContextKeyInner>::Value,
        >,
    ) -> (
        &'static ContextKey<Self::ContextKeyInner>,
        &mut <Self::ContextKeyInner as ContextKeyInner>::SwapValue,
    ) {
        let ctx = match self {
            Self {
                context_key: Some(ctx),
                value: Ok(value_ok),
            } if ctx.is_same_as(context_key) => {
                into_value.update_context_value_lazily(value_ok);

                ctx
            }
            Self {
                context_key: old_ctx,
                value,
            } => {
                let ctx = old_ctx.insert(context_key);
                *value = Inner::make_swap_value(into_value.into_context_value());

                ctx
            }
        };
        (ctx, &mut self.value)
    }
}

impl<T, Inner: ContextKeyInner<Value = T, SwapValue = T>> MaybeContextKeyAndValue
    for Option<(&'static ContextKey<Inner>, T)>
{
    type ContextKeyInner = Inner;

    fn maybe_context_key_and_value(
        &mut self,
    ) -> Option<(
        &'static ContextKey<Self::ContextKeyInner>,
        &mut <Self::ContextKeyInner as ContextKeyInner>::SwapValue,
    )> {
        match self {
            Some((ctx, value)) => Some((ctx, value)),
            _ => None,
        }
    }

    fn same_context_key_or_insert(
        &mut self,
        context_key: &'static ContextKey<Self::ContextKeyInner>,
        into_value: impl IntoContextValue<
            ContextValue = <Self::ContextKeyInner as ContextKeyInner>::Value,
        >,
    ) -> (
        &'static ContextKey<Self::ContextKeyInner>,
        &mut <Self::ContextKeyInner as ContextKeyInner>::SwapValue,
    ) {
        match self {
            Some((ctx, value)) => {
                if ctx.is_same_as(context_key) {
                    into_value.update_context_value_lazily(value);
                } else {
                    *ctx = context_key;
                    *value = into_value.into_context_value();
                }
                (ctx, value)
            }
            this => {
                let (ctx, value) = this.insert((context_key, into_value.into_context_value()));
                (ctx, value)
            }
        }
    }
}

pub struct ContextKey<Inner: ContextKeyInner>(Inner);

impl<T, Inner: ContextKeyInner<Value = T>> ContextKey<Inner> {
    pub const fn new(inner: Inner) -> Self {
        Self(inner)
    }

    #[inline]
    pub fn try_get(&'static self) -> Result<T, Inner::Error>
    where
        T: Copy,
    {
        self.0.try_get()
    }

    /// Panics if value is not provided
    #[inline]
    pub fn get(&'static self) -> T
    where
        T: Copy,
    {
        self.0.get()
    }

    #[inline]
    pub fn try_get_cloned(&'static self) -> Result<T, Inner::Error>
    where
        T: Clone,
    {
        self.0.try_get_cloned()
    }

    /// Panics if value is not provided
    #[inline]
    pub fn get_cloned(&'static self) -> T
    where
        T: Clone,
    {
        self.0.get_cloned()
    }

    #[inline]
    pub fn try_map<R>(&'static self, f: impl FnOnce(&T) -> R) -> Result<R, Inner::Error> {
        self.0.try_map(f)
    }

    /// Panics if value is not provided
    #[inline]
    pub fn map<R>(&'static self, f: impl FnOnce(&T) -> R) -> R {
        self.0.map(f)
    }

    fn is_same_as(&'static self, other: &'static Self) -> bool {
        self.0.is_same_as(&other.0)
    }

    fn is_same_as_or(&'static self, other: &'static Self, f: impl FnOnce(&T, &T) -> bool) -> bool {
        self.0.is_same_as(&other.0) || self.map(|this| other.map(|other| f(this, other)))
    }

    fn provide_value<R>(&'static self, value: &mut Inner::SwapValue, f: impl FnOnce() -> R) -> R {
        self.0.swap_value(value);

        struct Guard<'a, Inner: 'static + ContextKeyInner> {
            value: &'a mut Inner::SwapValue,
            ctx: &'static ContextKey<Inner>,
        }

        impl<Inner: 'static + ContextKeyInner> Drop for Guard<'_, Inner> {
            fn drop(&mut self) {
                self.ctx.0.swap_value(self.value)
            }
        }

        let guard = Guard { value, ctx: self };
        let res = f();
        drop(guard);
        res
    }
}

#[cfg(feature = "hooks")]
pub mod hooks {
    use hooks::{ShareValue, Signal, SignalHook, ToOwnedShareValue};

    use super::{ContextKey, ContextKeyInner};

    impl<S: ShareValue, Inner: 'static + ContextKeyInner<Value = S>> ShareValue
        for &'static ContextKey<Inner>
    {
        type Value = S::Value;

        hooks::proxy_share_value_with_provide!(ContextKey::map);

        fn try_unwrap(self) -> Result<Self::Value, Self> {
            Err(self)
        }

        fn equivalent_to(&self, other: &Self) -> bool {
            self.is_same_as_or(other, S::equivalent_to)
        }
    }

    impl<S: ShareValue, Inner: 'static + ContextKeyInner<Value = S>> ToOwnedShareValue
        for &'static ContextKey<Inner>
    {
        type OwnedShareValue = Self;

        fn to_owned_share_value(&self) -> Self::OwnedShareValue {
            *self
        }
    }

    impl<S: Signal, Inner: 'static + ContextKeyInner<Value = S>> Signal for &'static ContextKey<Inner> {
        type SignalHook = ContextKeySignalHook<Inner>;

        fn is_signal_of(&self, signal_hook: &Self::SignalHook) -> bool {
            self.is_same_as(signal_hook.context_key)
                || ContextKey::map(self, |s| s.is_signal_of(&signal_hook.signal_hook))
        }

        fn to_signal_hook(&self) -> Self::SignalHook {
            ContextKeySignalHook::_new(self)
        }

        fn update_signal_hook(&self, hook: std::pin::Pin<&mut Self::SignalHook>) {
            if self.is_same_as(hook.context_key) {
                return;
            }

            let hook = hook.project();
            *hook.context_key = self;
            ContextKey::map(self, |s| s.update_signal_hook(hook.signal_hook));
        }

        fn h_signal_hook<'hook>(
            &self,
            hook: std::pin::Pin<
                &'hook mut <Self::SignalHook as SignalHook>::SignalHookUninitialized,
            >,
        ) -> hooks::Value<'hook, Self::SignalHook> {
            let hook = hook.project();
            *hook.context_key = Some(self);
            ContextKey::map(self, |s| s.h_signal_hook(hook.signal_hook));
            self
        }

        fn notify_changed(&self) {
            ContextKey::map(self, S::notify_changed)
        }

        fn map_mut_and_notify_if<R>(&self, f: impl FnOnce(&mut Self::Value) -> (R, bool)) -> R {
            ContextKey::map(self, |s| s.map_mut_and_notify_if(f))
        }
    }

    pin_project_lite::pin_project!(
        // This has the same pattern as how LocalKey<S> implements SignalHook
        pub struct ContextKeySignalHook<Inner>
        where
            Inner: 'static,
            Inner: ContextKeyInner,
            Inner::Value: Signal,
        {
            context_key: &'static ContextKey<Inner>,
            #[pin]
            signal_hook: <Inner::Value as Signal>::SignalHook,
        }
    );

    impl<S: Signal, Inner: 'static + ContextKeyInner<Value = S>> ContextKeySignalHook<Inner> {
        fn _new(context_key: &'static ContextKey<Inner>) -> Self {
            Self {
                context_key,
                signal_hook: context_key.map(S::to_signal_hook),
            }
        }

        fn _clone_signal_hook(&self) -> Self {
            Self {
                context_key: self.context_key,
                signal_hook: self.signal_hook.to_signal_hook(),
            }
        }
    }

    impl<S: Signal, Inner: 'static + ContextKeyInner<Value = S>> ShareValue
        for ContextKeySignalHook<Inner>
    {
        type Value = S::Value;

        hooks::proxy_share_value!(|self| -> <Inner::Value as Signal>::SignalHook {
            &self.signal_hook
        });

        fn try_unwrap(self) -> Result<Self::Value, Self> {
            Err(self)
        }

        fn equivalent_to(&self, other: &Self) -> bool {
            self.context_key.is_same_as(other.context_key)
                || self.signal_hook.equivalent_to(&other.signal_hook)
        }
    }

    impl<S: Signal, Inner: 'static + ContextKeyInner<Value = S>> Signal
        for ContextKeySignalHook<Inner>
    {
        type SignalHook = Self;

        fn is_signal_of(&self, signal_hook: &Self::SignalHook) -> bool {
            self.context_key.is_same_as(signal_hook.context_key)
                || self.signal_hook.is_signal_of(&signal_hook.signal_hook)
        }

        fn to_signal_hook(&self) -> Self::SignalHook {
            self._clone_signal_hook()
        }

        fn update_signal_hook(&self, hook: std::pin::Pin<&mut Self::SignalHook>) {
            if self.context_key.is_same_as(hook.context_key) {
                return;
            }

            let hook = hook.project();

            self.signal_hook.update_signal_hook(hook.signal_hook);
            *hook.context_key = self.context_key;
        }

        fn h_signal_hook<'hook>(
            &self,
            hook: std::pin::Pin<
                &'hook mut <Self::SignalHook as SignalHook>::SignalHookUninitialized,
            >,
        ) -> hooks::Value<'hook, Self::SignalHook> {
            let hook = hook.project();

            *hook.context_key = Some(self.context_key);
            self.signal_hook.h_signal_hook(hook.signal_hook);

            self.context_key
        }

        fn notify_changed(&self) {
            self.signal_hook.notify_changed()
        }

        fn map_mut_and_notify_if<R>(&self, f: impl FnOnce(&mut Self::Value) -> (R, bool)) -> R {
            self.signal_hook.map_mut_and_notify_if(f)
        }
    }

    impl<S: Signal, Inner: 'static + ContextKeyInner<Value = S>> SignalHook
        for ContextKeySignalHook<Inner>
    {
        type SignalShareValue = S::Value;
        type SignalHookUninitialized = ContextKeySignalHookUninitialized<Inner>;

        fn to_signal(&self) -> hooks::Value<'_, Self> {
            self.context_key
        }
    }

    hooks::impl_hook!(
        type For<S: Signal, Inner: 'static + ContextKeyInner<Value = S>> =
            ContextKeySignalHook<Inner>;

        fn unmount(self) {
            self.project().signal_hook.unmount()
        }

        fn poll_next_update(self, cx: _) {
            self.project().signal_hook.poll_next_update(cx)
        }

        fn use_hook(self) -> &'static ContextKey<Inner> {
            let this = self.project();
            let _ = this.signal_hook.use_hook();
            this.context_key
        }
    );

    pin_project_lite::pin_project!(
        pub struct ContextKeySignalHookUninitialized<Inner>
        where
            Inner: 'static,
            Inner: ContextKeyInner,
            Inner::Value: Signal,
        {
            context_key: Option<&'static ContextKey<Inner>>,
            #[pin]
            signal_hook:
                <<Inner::Value as Signal>::SignalHook as SignalHook>::SignalHookUninitialized,
        }
    );

    impl<S: Signal, Inner: 'static + ContextKeyInner<Value = S>> Default
        for ContextKeySignalHookUninitialized<Inner>
    {
        fn default() -> Self {
            Self {
                context_key: None,
                signal_hook: Default::default(),
            }
        }
    }

    hooks::impl_hook!(
        type For<S: Signal, Inner: 'static + ContextKeyInner<Value = S>> =
            ContextKeySignalHookUninitialized<Inner>;

        fn unmount(self) {
            self.project().signal_hook.unmount()
        }

        fn poll_next_update(self, cx: _) {
            self.project().signal_hook.poll_next_update(cx)
        }
    );

    #[cfg(test)]
    mod tests {
        #[test]
        fn method_not_ambiguous() {
            use hooks::ShareValue;
            {
                trait Method1 {
                    fn method(&'static self) -> u8 {
                        1
                    }
                }

                trait Method2 {
                    fn method(&self) -> u8 {
                        2
                    }
                }

                struct Data;

                impl Method1 for Data {}
                impl Method2 for &'static Data {}

                assert_eq!(Data.method(), 1);
                assert_eq!((&&Data).method(), 2);
            }

            crate::local_context!(
                static CTX: hooks::GenSignalHook<()> = hooks::GenSignalHook::new(());
            );

            CTX.map(|_: &hooks::GenSignalHook<_>| {});

            (&&CTX).map(|()| {});

            ShareValue::map(&&CTX, |()| {});
        }
    }
}

pub mod element {
    use frender_ssr::SsrElement;

    pub use self::csr::{Kind, StateWithContext};

    use crate::{ContextKey, ContextKeyInner};

    impl<T, Inner: ContextKeyInner<Value = T>> ContextKey<Inner> {
        /// Shortcut for <code>CTX.[value](Self::value)(value).[children](ElementWithContext::children)(get_element)</code>
        pub fn provide<E: SsrElement, FE: FnOnce() -> E>(
            &'static self,
            value: T,
            get_element: FE,
        ) -> ElementWithContext<Inner, Value<T>, FE> {
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
        ) -> ElementWithContext<Inner, GetValue<F>, FE> {
            ElementWithContext {
                context_key: self,
                into_value: GetValue(get_value),
                get_element,
            }
        }

        pub fn value(&'static self, value: T) -> ElementWithContext<Inner, Value<T>, ()> {
            ElementWithContext {
                context_key: self,
                into_value: Value(value),
                get_element: (),
            }
        }

        pub fn get_value<F: FnOnce() -> T>(
            &'static self,
            get_value: F,
        ) -> ElementWithContext<Inner, GetValue<F>, ()> {
            ElementWithContext {
                context_key: self,
                into_value: GetValue(get_value),
                get_element: (),
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

    pub struct ElementWithContext<
        Inner: ContextKeyInner + 'static,
        F: IntoContextValue<ContextValue = Inner::Value>,
        FE,
    > {
        pub context_key: &'static ContextKey<Inner>,
        pub into_value: F,
        pub get_element: FE,
    }

    impl<T, Inner: ContextKeyInner<Value = T> + 'static, F: IntoContextValue<ContextValue = T>>
        ElementWithContext<Inner, F, ()>
    {
        pub fn children<E: SsrElement, FE: FnOnce() -> E>(
            self,
            get_element: FE,
        ) -> ElementWithContext<Inner, F, FE> {
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

    mod ssr {
        use frender_ssr::SsrElement;

        use crate::ContextKeyInner;

        use super::{ElementWithContext, IntoContextValue};

        impl<
                T,
                Inner: ContextKeyInner<Value = T> + 'static,
                F: IntoContextValue<ContextValue = T>,
                FE: FnOnce() -> E,
                E: SsrElement,
            > SsrElement for ElementWithContext<Inner, F, FE>
        {
            type HtmlChildren = E::HtmlChildren;

            fn into_html_children(self) -> Self::HtmlChildren {
                let Self {
                    context_key,
                    into_value,
                    get_element,
                } = self;
                let mut value = Inner::make_swap_value(into_value.into_context_value());
                context_key.provide_value(&mut value, || get_element().into_html_children())
            }
        }
    }

    mod csr {
        use std::{marker::PhantomData, pin::Pin};

        use frender_html::{CsrElement, RenderStateKindPinned, RenderStateKindUnpinned};

        use crate::{ContextKeyInner, MaybeContextKeyAndValue};

        use super::{ElementWithContext, IntoContextValue};

        pin_project_lite::pin_project!(
            #[project = StateWithContextProj]
            #[derive(Debug, Default)]
            pub struct StateWithContext<C: MaybeContextKeyAndValue, S> {
                context_key_and_value: C,
                #[pin]
                render_state: S,
            }
        );

        mod state {
            use std::{pin::Pin, task::Poll};

            use frender_html::RenderState;

            use super::{MaybeContextKeyAndValue, StateWithContext};

            impl<C: MaybeContextKeyAndValue, S> StateWithContext<C, S> {
                fn provide_render_state(self: Pin<&mut Self>, f: impl FnOnce(Pin<&mut S>)) {
                    let this = self.project();
                    if let Some((ctx, value)) =
                        this.context_key_and_value.maybe_context_key_and_value()
                    {
                        ctx.provide_value(value, || f(this.render_state))
                    }
                }
            }

            impl<C: MaybeContextKeyAndValue, S: RenderState<R>, R: ?Sized> RenderState<R>
                for StateWithContext<C, S>
            {
                fn unmount(self: Pin<&mut Self>, renderer: &mut R) {
                    self.provide_render_state(|inner| inner.unmount(renderer))
                }

                fn state_unmount(self: Pin<&mut Self>) {
                    self.provide_render_state(S::state_unmount)
                }

                fn poll_render(
                    self: Pin<&mut Self>,
                    renderer: &mut R,
                    cx: &mut std::task::Context<'_>,
                ) -> Poll<()> {
                    let mut res = Poll::Ready(());
                    self.provide_render_state(|inner| res = inner.poll_render(renderer, cx));
                    res
                }

                fn check_and_move_cursor(&self, render_context: &mut R::RenderContext<'_>)
                where
                    R: frender_html::dom::render::RenderWithContext,
                {
                    // TODO: value not provided
                    self.render_state.check_and_move_cursor(render_context)
                }
            }
        }

        enum Never {}
        pub struct Kind<C: Default + MaybeContextKeyAndValue, K>(Never, PhantomData<(C, K)>);

        impl<C: Default + MaybeContextKeyAndValue, K: RenderStateKindUnpinned>
            RenderStateKindUnpinned for Kind<C, K>
        {
            type UnpinnedRenderState<R: frender_html::RenderHtml + ?Sized> =
                StateWithContext<C, K::UnpinnedRenderState<R>>;
        }

        impl<C: Default + MaybeContextKeyAndValue, K: RenderStateKindPinned> RenderStateKindPinned
            for Kind<C, K>
        {
            type RenderState<R: frender_html::RenderHtml + ?Sized> =
                StateWithContext<C, K::RenderState<R>>;
        }

        impl<
                T,
                Inner: 'static + ContextKeyInner<Value = T>,
                F: IntoContextValue<ContextValue = T>,
                FE,
            > ElementWithContext<Inner, F, FE>
        {
            fn provide_get_element<S, R>(
                //
                self,
                render_state: Pin<&mut StateWithContext<Inner::MaybeContextKeyAndValue, S>>,
                f: impl FnOnce(FE, Pin<&mut S>) -> R,
            ) -> R {
                let Self {
                    context_key,
                    into_value,
                    get_element,
                } = self;

                let StateWithContextProj {
                    context_key_and_value,
                    render_state,
                } = render_state.project();

                let (ctx, value) =
                    context_key_and_value.same_context_key_or_insert(context_key, into_value);

                ctx.provide_value(value, || f(get_element, render_state))
            }
        }

        impl<
                T,
                Inner: 'static + ContextKeyInner<Value = T>,
                F: IntoContextValue<ContextValue = T>,
                E: CsrElement,
                FE: FnOnce() -> E,
            > CsrElement for ElementWithContext<Inner, F, FE>
        {
            type RenderStateKind = Kind<Inner::MaybeContextKeyAndValue, E::RenderStateKind>;

            fn render_update<Ctx: ?Sized + frender_html::HtmlRenderContext>(
                self,
                render_context: &mut Ctx,
                render_state: Pin<
                    &mut frender_html::RenderStateOfContext<Self::RenderStateKind, Ctx>,
                >,
            ) {
                self.provide_get_element(render_state, |get_element, render_state| {
                    get_element().render_update(render_context, render_state)
                })
            }

            fn render_update_force_reposition<Ctx: ?Sized + frender_html::HtmlRenderContext>(
                self,
                render_context: &mut Ctx,
                render_state: Pin<
                    &mut frender_html::RenderStateOfContext<Self::RenderStateKind, Ctx>,
                >,
            ) {
                self.provide_get_element(render_state, |get_element, render_state| {
                    get_element().render_update_force_reposition(render_context, render_state)
                })
            }

            fn render_update_maybe_reposition<Ctx: ?Sized + frender_html::HtmlRenderContext>(
                self,
                render_context: &mut Ctx,
                render_state: Pin<
                    &mut frender_html::RenderStateOfContext<Self::RenderStateKind, Ctx>,
                >,
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
                    get_element().unpinned_render_update_force_reposition(
                        render_context,
                        render_state.get_mut(),
                    )
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
}
