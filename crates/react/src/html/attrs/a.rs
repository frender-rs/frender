use crate::html::common::def_attrs_traits;

macro_rules! extend_common_attrs {
    (@ __impl_get_k @ $k_js:literal $k:ident ) => {
        $k_js
    };
    (@ __impl_get_k @ $k:ident ) => {
        // TODO: camelCase
        stringify!($k)
    };
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
                $crate::html::common::AsAttributes<'a, V>,
                $crate::html::common::AsMutAttributes<'a, V>
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

        impl <$($generics)*> AsRef<dyn $crate::html::common::AsAttributes<'a, V> + 'a> for Attributes <$($generics)*> $($where)* {
            fn as_ref(&self) -> &(dyn $crate::html::common::AsAttributes<'a, V> + 'a) {
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

        impl <$($generics)*> AsMut<dyn $crate::html::common::AsMutAttributes<'a, V> + 'a> for Attributes <$($generics)*> $($where)* {
            fn as_mut(&mut self) -> &mut (dyn $crate::html::common::AsMutAttributes<'a, V> + 'a) {
                &mut self._common
            }
        }
    };
}

extend_common_attrs! {
    generics { 'a, V }
    where { where V: 'a }
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

pub struct Component<'a, V>(&'a dyn AsAttributes<'a, V>);

impl<'a, V> crate::Component for Component<'a, V> {
    type ElementType = react_sys::Element;
    type Props = &'a dyn AsAttributes<'a, V>;

    fn create_with_props(props: Self::Props) -> Self {
        Self(props)
    }

    fn render(self) -> Self::ElementType {
        todo!()
    }
}
