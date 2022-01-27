use crate::html::common::def_attrs_traits;

macro_rules! extend_common_attrs {
    (
        generics { $($generics:tt)* }
        where { $($where:tt)* }
        attrs {
            $($k:ident $(@ $k_js:literal)? : $v_ty:ty),* $(,)?
        }
    ) => {
        def_attrs_traits! {
            generics { $($generics)* }
            where { $($where)* }
            extends {
                $crate::html::common::AsAttributes<'a, TElement, TValue>,
                $crate::html::common::AsMutAttributes<'a, TElement, TValue>
            }
            attrs {
                $($k : $v_ty ,)*
            }
        }

        struct Attributes <$($generics)*> $($where)* {
            _common: $crate::html::common::Attributes<$($generics)*>,
            $(
                $k: $v_ty,
            )+
        }

        impl <$($generics)*> AsAttributes <$($generics)*> for Attributes <$($generics)*> $($where)* {
            $(
                fn $k(&self) -> &$v_ty {
                    &self.$k
                }
            )*
        }

        impl <$($generics)*> AsRef<dyn $crate::html::common::AsAttributes<'a, TElement, TValue> + 'a> for Attributes <$($generics)*> $($where)* {
            fn as_ref(&self) -> &(dyn $crate::html::common::AsAttributes<'a, TElement, TValue> + 'a) {
                &self._common
            }
        }

        impl <$($generics)*> AsMutAttributes <$($generics)*> for Attributes <$($generics)*> $($where)* {
            $(
                fn $k(&mut self, v: $v_ty) {
                    self.$k = v;
                }
            )*
        }

        impl <$($generics)*> AsMut<dyn $crate::html::common::AsMutAttributes<'a, TElement, TValue> + 'a> for Attributes <$($generics)*> $($where)* {
            fn as_mut(&mut self) -> &mut (dyn $crate::html::common::AsMutAttributes<'a, TElement, TValue> + 'a) {
                &mut self._common
            }
        }
    };
}

extend_common_attrs! {
    generics { 'a, TElement, TValue }
    where { where TElement:'a, TValue: 'a }
    attrs {
        download: Option<wasm_bindgen::JsValue>,
        href: Option<&'a str>,
        href_lang: Option<&'a str>,
        media: Option<&'a str>,
        ping: Option<&'a str>,
        rel: Option<&'a str>,
        target: Option<crate::html::AnchorTarget>,
        kind@"type": Option<&'a str>,
        referrer_policy: Option<crate::html::ReferrerPolicy>,
    }
}

pub struct Component<'a, TElement, TValue>(&'a dyn AsAttributes<'a, TElement, TValue>);

impl<'a, TElement, TValue> crate::Component for Component<'a, TElement, TValue> {}
