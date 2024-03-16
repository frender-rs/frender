use wasm_bindgen::JsValue;

pub trait DebugProps {
    fn as_debug_props(&self) -> Option<JsValue>;
}

impl<T: DebugProps> DebugProps for &T {
    fn as_debug_props(&self) -> Option<JsValue> {
        DebugProps::as_debug_props(*self)
    }
}

/// `auto_debug_props!(props)` can auto output debug info for `props`.
///
/// If `props impl DebugProps`, then [`DebugProps::as_debug_props`] will be used;
///
/// Else if `props impl Debug`, then `format!("{:#?}", props)` will be used;
///
/// Else, the typename of `props` will be used.
///
/// Note that the type of the expr
/// **SHOULD NOT** be references unless you know what it means.
///
/// The implementation is inspired by dtolnay's [autoref-specialization](https://github.com/dtolnay/case-studies/blob/master/autoref-specialization/README.md).
///
#[macro_export]
macro_rules! auto_debug_props {
    ($v:expr) => {{
        #[allow(unused_imports)]
        use ::core::fmt::Debug;
        #[allow(unused_imports)]
        use $crate::{impl_auto_debug_props::*, DebugProps};
        match $v {
            ref v => match v
                .is_debug_props_or_not()
                .intermediate_value_to_debug_props(v)
            {
                tag_or_v => tag_or_v.is_impl_debug_or_not().output_debug_props(v),
            },
        }
    }};
}

pub mod impl_auto_debug_props {
    use wasm_bindgen::JsValue;

    fn debug_props<T: super::DebugProps>(v: &T) -> Option<JsValue> {
        v.as_debug_props()
    }
    fn debug_debug<T: ::core::fmt::Debug>(v: &T) -> Option<JsValue> {
        Some(JsValue::from(format!("{v:#?}")))
    }
    fn debug_any<T>(_: &T) -> Option<JsValue> {
        let ty = std::any::type_name::<T>();
        Some(JsValue::from(format!("{ty} {{ ... }}")))
    }

    pub trait IsDebugPropsOrNot {
        type Output;
        fn is_debug_props_or_not(&self) -> Self::Output;
    }

    pub struct IsDebugPropsTag;

    impl IsDebugPropsTag {
        #[inline]
        pub fn intermediate_value_to_debug_props<T>(self, _: &T) -> Self {
            self
        }
        #[inline]
        pub fn is_impl_debug_or_not(self) -> Self {
            self
        }
        #[inline]
        pub fn output_debug_props<T: super::DebugProps>(self, v: &T) -> Option<JsValue> {
            debug_props(v)
        }
    }

    pub struct NotImplTag;

    impl NotImplTag {
        #[inline]
        pub fn intermediate_value_to_debug_props<T>(self, v: &T) -> &T {
            v
        }
        #[inline]
        pub fn output_debug_props<T>(self, v: &T) -> Option<JsValue> {
            debug_any(v)
        }
    }

    pub trait IsDebugProps {
        #[inline]
        fn is_debug_props_or_not(&self) -> IsDebugPropsTag {
            IsDebugPropsTag
        }
    }

    impl<T: super::DebugProps> IsDebugProps for T {}

    pub trait NotImpl {
        #[inline]
        fn is_debug_props_or_not(&self) -> NotImplTag {
            NotImplTag
        }
        #[inline]
        fn is_impl_debug_or_not(&self) -> NotImplTag {
            NotImplTag
        }
    }

    impl<T> NotImpl for &T {}

    pub struct IsImplDebugTag;

    impl IsImplDebugTag {
        #[inline]
        pub fn output_debug_props<T: ::core::fmt::Debug>(self, v: &T) -> Option<JsValue> {
            debug_debug(v)
        }
    }

    pub trait IsImplDebug {
        #[inline]
        fn is_impl_debug_or_not(&self) -> IsImplDebugTag {
            IsImplDebugTag
        }
    }

    impl<T: ::core::fmt::Debug> IsImplDebug for T {}
}
