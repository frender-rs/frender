/// The type will always derive the traits regardless of generic params
/// just like [`PhantomData<T>`](core::marker::PhantomData).
pub mod always_derive {
    #[doc(hidden)]
    pub use crate::{
        __define_phantom_wrapper_impl_always_derive_Clone as Clone,
        __define_phantom_wrapper_impl_always_derive_Debug as Debug,
        __define_phantom_wrapper_impl_always_derive_PartialEq as PartialEq,
        __define_phantom_wrapper_impl_always_derive_trait_without_item as Copy,
        __define_phantom_wrapper_impl_always_derive_trait_without_item as Eq, __noop,
    };
}

#[macro_export]
macro_rules! define_phantom_wrapper {
    (
        #[$maybe_always_derive:ident $maybe_always_derive_content:tt]
        $($rest:tt)*
    ) => {
        $crate::__define_phantom_wrapper_after_maybe_always_derive! {
            $maybe_always_derive $maybe_always_derive
            $maybe_always_derive_content
            {$($rest)*}
        }
    };
    (
        $($rest:tt)*
    ) => {
        $crate::__define_phantom_wrapper_after_always_derive! {
            always_derive { always_derive () }
            rest {$($rest)*}
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __define_phantom_wrapper_after_maybe_always_derive {
    (
        always_derive $always_derive:ident
        $always_derive_content:tt
        $rest:tt
    ) => {
        $crate::__define_phantom_wrapper_after_always_derive! {
            always_derive { $always_derive $always_derive_content }
            rest $rest
        }
    };
    (
        $not_always_derive:ident $_not_always_derive:ident
        $not_always_derive_content:tt
        {$($rest:tt)*}
    ) => {
        $crate::__define_phantom_wrapper_after_always_derive! {
            always_derive { always_derive () }
            rest {
                #[$not_always_derive $not_always_derive_content]
                $($rest)*
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __define_phantom_wrapper_after_always_derive {
    (
        always_derive $always_derive:tt
        rest {
            $(#$attrs:tt)*
            $vis:vis struct $name:ident $($generics_and_rest:tt)*
        }
    ) => {
        $crate::define_phantom_wrapper::__private::parse_optional_angle_bracketed_generics! {
            on_finish {$crate::__define_phantom_wrapper_after_parse_generics!}
            prepend {
                always_derive $always_derive
                attrs { $(#$attrs)* }
                vis {$vis}
                name {$name}
            }
            input { $($generics_and_rest)* }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __define_phantom_wrapper_after_parse_generics {
    (
        always_derive { $always_derive:ident $always_derive_content:tt }
        attrs { $($attrs:tt)* }
        vis {$vis:vis}
        name {$name:ident}
        $(
            lt $lt:tt
            parsed_generics {
                generics {$($generics:tt)*}
                impl_generics {$($impl_generics:tt)*}
                type_generics {$($type_generics:tt)*}
                generics_info {
                    $($($generics_info:tt)+)?
                }
            }
            gt $gt:tt
        )?
        rest $rest:tt
    ) => {
        $($attrs)*
        $vis struct $name <$($($generics)*)?> {
            $($(
                __: ($(
                    $crate::__generic_info_to_PhantomData_or_unit! $generics_info,
                )+)
            )?)?
        }

        #[allow(non_snake_case)]
        $vis const fn $name<$($($impl_generics)*)?>() -> $name::<$($($type_generics)*)?> {
            $name {
                $($(
                    __: ($(
                        $crate::__generic_info_to_PhantomData_or_unit! $generics_info,
                    )+)
                )?)?
            }
        }

        const _: () = {
            $crate::__define_phantom_wrapper_impl_always_derive! {
                $always_derive
                $always_derive_content
                $always_derive_content
                {
                    impl_generics {$($($impl_generics)*)?}
                    ty { $name::<$($($type_generics)*)?> }
                }
            }
        };
    };
}

/// this is a $ty and an $expr.
#[doc(hidden)]
#[macro_export]
macro_rules! __generic_info_to_PhantomData_or_unit {
    (
        $(lifetime_attrs $lifetime_attrs:tt)?
        lifetime {$lt:lifetime}
        $($rest:tt)*
    ) => {
        $crate::define_phantom_wrapper::__private::PhantomData::<&$lt()>
    };
    (
        $(const_attrs $const_attrs:tt)?
        const $($rest:tt)*
    ) => {
        ()
    };
    (
        $(type_attrs $type_attrs:tt)?
        name {$name:ident}
        $($rest:tt)*
    ) => {
        $crate::define_phantom_wrapper::__private::PhantomData::<$name>
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __define_phantom_wrapper_impl_always_derive {
    (
        $always_derive:ident
        ($($derive:ident),+ $(,)?)
        $always_derive_content:tt
        $info:tt
    ) => {
        $(
            $crate::define_phantom_wrapper::$always_derive::$derive! {
                $derive
                $info
                $always_derive_content
            }
        )+
    };
    (
        $always_derive:ident
        ()
        ()
        $info:tt
    ) => {
        $crate::define_phantom_wrapper::$always_derive::__noop! {}
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __define_phantom_wrapper_impl_always_derive_trait_without_item {
    (
        $trait:ident
        {
            impl_generics {$($impl_generics:tt)*}
            ty { $for_ty:ty }
        }
        $always_derive_content:tt
    ) => {
        impl<
            $($impl_generics)*
        > $crate::define_phantom_wrapper::__private::$trait for $for_ty {}
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __define_phantom_wrapper_impl_always_derive_Clone {
    (
        $trait:ident
        {
            impl_generics {$($impl_generics:tt)*}
            ty { $for_ty:path }
        }
        $always_derive_content:tt
    ) => {
        impl<
            $($impl_generics)*
        > $crate::define_phantom_wrapper::__private::$trait for $for_ty {
            $crate::__define_phantom_wrapper_impl_always_derive_Clone_clone! $always_derive_content;
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __define_phantom_wrapper_impl_always_derive_Clone_clone {
    () => {
        // no Copy
        fn clone(&self) -> Self {
            Self {
                //
                __: self.__,
            }
        }
    };
    (Copy $(, $($rest:tt)*)?) => {
        fn clone(&self) -> Self {
            *self
        }
    };
    ($other:ident $(, $($rest:tt)*)?) => {
        $crate::__define_phantom_wrapper_impl_always_derive_Clone_clone! {
            $($($rest)*)?
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __define_phantom_wrapper_impl_always_derive_PartialEq {
    (
        $trait:ident
        {
            impl_generics {$($impl_generics:tt)*}
            ty { $for_ty:path }
        }
        $always_derive_content:tt
    ) => {
        impl<
            $($impl_generics)*
        > $crate::define_phantom_wrapper::__private::$trait for $for_ty {
            fn eq(&self, _: &Self) -> $crate::define_phantom_wrapper::__private::bool {
                true
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __define_phantom_wrapper_impl_always_derive_Debug {
    (
        $trait:ident
        {
            impl_generics {$($impl_generics:tt)*}
            ty { $for_ty:path }
        }
        $always_derive_content:tt
    ) => {
        impl<
            $($impl_generics)*
        > $crate::define_phantom_wrapper::__private::$trait for $for_ty {
            fn fmt(&self, f: &mut $crate::define_phantom_wrapper::__private::Formatter<'_>) -> $crate::define_phantom_wrapper::__private::FmtResult {
                self.__.fmt(f)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __noop {
    () => {};
}

#[doc(hidden)]
pub mod __private {
    pub use ::core::{
        clone::Clone,
        cmp::{Eq, PartialEq},
        fmt::{Debug, Formatter, Result as FmtResult},
        marker::{Copy, PhantomData},
        primitive::bool,
    };
    pub use syn_lite::parse_optional_angle_bracketed_generics;
}

#[cfg(test)]
mod tests {

    define_phantom_wrapper!(
        #[always_derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct MyPhantom<T: ?Sized>;
    );

    const _: MyPhantom<str> = MyPhantom();
    #[test]
    fn test() {
        struct T;
        _ = &MyPhantom::<T>() as &dyn core::fmt::Debug;
    }

    const _: () = {
        define_phantom_wrapper!(
            #[always_derive()]
            pub struct MyPhantom<T: ?Sized, R: ?Sized + Fn() = fn()>;
        );
    };
}
