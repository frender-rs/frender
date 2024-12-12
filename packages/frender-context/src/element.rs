use frender_ssr::SsrElement;

use crate::{ContextKey, ContextKeyInner};

// TODO: `state_unmount()` doesn't get the correct context value. Is this a problem?
pub mod csr;
mod ssr;

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
