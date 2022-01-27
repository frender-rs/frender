use std::{any::Any, rc::Rc};
use wasm_bindgen::JsValue;

pub struct PassToJsRuntimeValue {
    pub js_value: JsValue,
    pub to_persist: Option<Rc<dyn Any>>,
}

pub trait TryBorrowToJsRuntime {
    fn try_borrow_to_js_runtime(&self) -> Option<PassToJsRuntimeValue> {
        None
    }
}

pub trait TryIntoJsRuntime {
    fn try_into_js_runtime(self) -> Result<PassToJsRuntimeValue, Self>
    where
        Self: Sized;
}

pub fn try_borrow_to_js_runtime_or_else<
    T: TryBorrowToJsRuntime,
    F: FnOnce(&T) -> PassToJsRuntimeValue,
>(
    v: &T,
    default: F,
) -> PassToJsRuntimeValue {
    v.try_borrow_to_js_runtime().unwrap_or_else(|| default(v))
}

pub fn try_into_js_runtime_or_else<T: TryIntoJsRuntime, F: FnOnce(T) -> PassToJsRuntimeValue>(
    v: T,
    default: F,
) -> PassToJsRuntimeValue {
    v.try_into_js_runtime().unwrap_or_else(default)
}

pub trait IntoJsRuntime: Sized {
    fn into_js_runtime(self) -> PassToJsRuntimeValue;
}

pub trait BorrowToJsRuntime {
    fn borrow_to_js_runtime(&self) -> PassToJsRuntimeValue;
}

impl<T: ?Sized + BorrowToJsRuntime> TryBorrowToJsRuntime for T {
    fn try_borrow_to_js_runtime(&self) -> Option<PassToJsRuntimeValue> {
        Some(self.borrow_to_js_runtime())
    }
}

impl<T: BorrowToJsRuntime> BorrowToJsRuntime for Box<T> {
    fn borrow_to_js_runtime(&self) -> PassToJsRuntimeValue {
        self.as_ref().borrow_to_js_runtime()
    }
}

impl<T: IntoJsRuntime> TryIntoJsRuntime for T {
    fn try_into_js_runtime(self) -> Result<PassToJsRuntimeValue, Self>
    where
        Self: Sized,
    {
        Ok(self.into_js_runtime())
    }
}

pub fn into_js_runtime_with_cloned<T: ?Sized + BorrowToJsRuntime + Clone>(
    v: &T,
) -> crate::PassToJsRuntimeValue {
    v.clone().borrow_to_js_runtime()
}

#[macro_export]
macro_rules! __impl_pass_to_js_runtime_trait {
    (try_borrow {$name:ty} {$($generics:tt)*} {$($where:tt)*} {$($v:ident)?} {$($impl_code:tt)*} {$($impl_func:path)?} ) => {
        impl $($generics)* $crate::TryBorrowToJsRuntime for $name $($where)* {
            fn try_borrow_to_js_runtime(&self) -> Option<$crate::PassToJsRuntimeValue> {
                $(let $v = self;)?
                $($impl_code)*
                $($impl_func(self))?
            }
        }
    };
    (try_into {$name:ty} {$($generics:tt)*} {$($where:tt)*} {$($v:ident)?} {$($impl_code:tt)*} {$($impl_func:path)?}) => {
        impl $($generics)* $crate::TryIntoJsRuntime for $name $($where)* {
            fn try_into_js_runtime(self) -> Result<$crate::PassToJsRuntimeValue, Self> {
                $(let $v = self;)?
                $($impl_code)*
                $($impl_func(self))?
            }
        }
    };
    (borrow {$name:ty} {$($generics:tt)*} {$($where:tt)*} {$($v:ident)?} {$($impl_code:tt)*} {$($impl_func:path)?}) => {
        impl $($generics)* $crate::BorrowToJsRuntime for $name $($where)* {
            fn borrow_to_js_runtime(&self) -> $crate::PassToJsRuntimeValue {
                $(let $v = self;)?
                $($impl_code)*
                $($impl_func(self))?
            }
        }
    };
    (into {$name:ty} {$($generics:tt)*} {$($where:tt)*} {$($v:ident)?} {$($impl_code:tt)*} {$($impl_func:path)?}) => {
        impl $($generics)* $crate::IntoJsRuntime for $name $($where)* {
            fn into_js_runtime(self) -> $crate::PassToJsRuntimeValue {
                $(let $v = self;)?
                $($impl_code)*
                $($impl_func(self))?
            }
        }
    };
}

#[macro_export]
macro_rules! impl_pass_to_js_runtime {
    ([] $($t:tt)*) => {};
    (
        [$trait_name:ident : clone $($other:tt)*]
        $({$($generics:tt)*})?
        $name:ty
        $(where $($where:tt)+)?
    ) => {
        $crate::__impl_pass_to_js_runtime_trait! {
            $trait_name
            {$name}
            {$(<$($generics)*>)?}
            {$(where $($where)+)?}
            {}
            {}
            {$crate::into_js_runtime_with_cloned}
        }
        $crate::impl_pass_to_js_runtime! { [$($other)*] $({$($generics)*})? $name $(where $($where)+)? }
    };
    (
        [$trait_name:ident : None $($other:tt)*]
        $({$($generics:tt)*})?
        $name:ty
        $(where $($where:tt)+)?
    ) => {
        $crate::__impl_pass_to_js_runtime_trait! {
            $trait_name
            {$name}
            {$(<$($generics)*>)?}
            {$(where $($where)+)?}
            {}
            {None}
            {}
        }
        $crate::impl_pass_to_js_runtime! { [$($other)*] $({$($generics)*})? $name $(where $($where)+)? }
    };
    (
        [$trait_name:ident : $impl_func:ident $($other:tt)*]
        $({$($generics:tt)*})?
        $name:ty
        $(where $($where:tt)+)?
    ) => {
        $crate::__impl_pass_to_js_runtime_trait! {
            $trait_name
            {$name}
            {$(<$($generics)*>)?}
            {$(where $($where)+)?}
            {}
            {}
            {$impl_func}
        }
        $crate::impl_pass_to_js_runtime! { [$($other)*] $({$($generics)*})? $name $(where $($where)+)? }
    };
    (
        [$trait_name:ident ($v:ident) { $($impl_code:tt)* } $($other:tt)*]
        $({$($generics:tt)*})?
        $name:ty
        $(where $($where:tt)+)?
    ) => {
        $crate::__impl_pass_to_js_runtime_trait! {
            $trait_name
            {$name}
            {$(<$($generics)*>)?}
            {$(where $($where)+)?}
            {$v}
            {
                $($impl_code)*
            }
            {}
        }
        $crate::impl_pass_to_js_runtime! { [$($other)*] $({$($generics)*})? $name $(where $($where)+)? }
    };
}
