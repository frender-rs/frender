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

    fn update_swap_value_lazily(
        swap_value: &mut Self::SwapValue,
        into_value: impl IntoContextValue<ContextValue = Self::Value>,
    );
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
        impl<S: Signal, Inner: 'static + ContextKeyInner<Value = S>> ContextKeySignalHook<Inner> {
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
        impl<S: Signal, Inner: 'static + ContextKeyInner<Value = S>>
            ContextKeySignalHookUninitialized<Inner>
        {
            fn unmount(self) {
                self.project().signal_hook.unmount()
            }

            fn poll_next_update(self, cx: _) {
                self.project().signal_hook.poll_next_update(cx)
            }
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

pub mod element;
