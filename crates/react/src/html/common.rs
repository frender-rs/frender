macro_rules! def_attrs_traits {
    (
        generics { $($generics:tt)* }
        where { $($where:tt)* }
        $(extends { $extend_as:path , $extend_as_mut:path  })?
        attrs {
            $($k:ident : $v_ty:ty),* $(,)?
        }
    ) => {
        pub trait AsAttributes <$($generics)*> $(: $extend_as)? $($where)* {
            $(fn $k(&'a self) -> &'a $v_ty;)*
        }
        pub trait AsMutAttributes <$($generics)*> $(: $extend_as_mut)? $($where)* {
            $(fn $k(&mut self, v: $v_ty);)*
        }

        impl<$($generics)* , TExtends: AsRef<dyn AsAttributes <$($generics)*> + 'a> $(+ $extend_as)? > AsAttributes <$($generics)*>  for TExtends $($where)* {
            $(
                fn $k(&'a self) -> &'a $v_ty {
                    let v = self.as_ref();
                    &v.$k()
                }
            )*
        }

        impl<$($generics)* , TExtends: AsMut<dyn AsMutAttributes <$($generics)*> + 'a> $(+ $extend_as_mut)? > AsMutAttributes <$($generics)*>  for TExtends $($where)* {
            $(
                fn $k(&mut self, v: $v_ty) {
                    self.as_mut().$k(v);
                }
            )*
        }
    };
}

pub(crate) use def_attrs_traits;

macro_rules! def_attrs {
    (@ __impl_get_k @ $k_js:literal $k:ident ) => {
        $k_js
    };
    (@ __impl_get_k @ $k:ident ) => {
        // TODO: camelCase
        stringify!($k)
    };
    (
        generics { $($generics:tt)* }
        generics_for_any { $($generics_for_any:tt)* }
        where { $($where:tt)* }
        struct {
            $($k:ident $(@ $k_js:literal)? : $v_ty:ty),* $(,)?
        }
        any {
            $($any_k:ident $(@ $any_k_js:literal)? : $any_v_ty:ty),* $(,)?
        }
    ) => {
        def_attrs_traits! {
            generics { $($generics)* }
            where { $($where)* }
            attrs {
                $($k : $v_ty ,)*
                $($any_k : $any_v_ty ,)*
            }
        }

        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy, Hash, PartialEq, Eq)]
        enum AttributeKey {
            $(
                $any_k
            ),+
        }

        #[allow(non_camel_case_types)]
        enum Attribute < $($generics_for_any)* > {
            $(
                $any_k($any_v_ty)
            ),+
        }

        impl < $($generics_for_any)* > $crate::html::any_attrs::AnyAttributeValue for Attribute < $($generics_for_any)* > {
            type Key = AttributeKey;

            fn as_attribute_key(&self) -> Self::Key {
                match self {
                    $(
                        Self::$any_k(_) => AttributeKey::$any_k,
                    )+
                }
            }
        }

        pub struct Attributes <$($generics)*> $($where)* {
            _any: $crate::html::any_attrs::AnyAttributes<Attribute < $($generics_for_any)* >>,
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

            $(
                fn $any_k(&self) -> &$any_v_ty {
                    let v = &self._any.get(&AttributeKey::$any_k);
                    match v {
                        Some(Attribute::$any_k(v)) => v,
                        None => &None,
                        _ => {
                            panic! ("attribute name should be {} in ::react::html::common::Attributes", stringify!($any_k));
                        }
                    }
                }
            )*
        }

        impl <$($generics)*> AsMutAttributes <$($generics)*> for Attributes <$($generics)*> $($where)* {
            $(
                fn $k(&mut self, v: $v_ty) {
                    self.$k = v;
                }
            )*

            $(
                fn $any_k(&mut self, v: $any_v_ty) {
                    self._any.set(
                        Attribute::$any_k(v)
                    );
                }
            )*
        }
    };
}

def_attrs! {
    generics { 'a, V }
    generics_for_any { 'a }
    where { where V: 'a }

    struct {
        children: Option<&'a dyn crate::Node>,
        default_checked: Option<bool>,
        default_value: Option<V>,
        class@"className": Option<&'a str>,
        draggable: Option<bool>,
        hidden: Option<bool>,
        id: Option<&'a str>,
        lang: Option<&'a str>,
        placeholder: Option<&'a str>,
        style: Option<&'a crate::html::css::CssProperties>,
        tab_index: Option<i32>,
        title: Option<&'a str>,
        on_click: &'a dyn FnMut(wasm_bindgen::JsValue),
    }

    any {
        // React-specific Attributes
        suppress_content_editable_warning: Option<bool>,
        suppress_hydration_warning: Option<bool>,

        // Standard HTML Attributes
        access_key: Option<&'a str>,
        content_editable: Option<crate::html::Inheritable<bool>>,
        context_menu: Option<&'a str>,
        dir: Option<&'a str>,
        slot: Option<&'a str>,
        spell_check: Option<bool>,
        translate: Option<&'a str>, // TODO: ser: yes | no

        // Unknown
        radio_group: Option<&'a str>, // <command>, <menuitem>

        // WAI-ARIA
        role: Option<crate::html::aria::Role>,

        // RDFa Attributes
        about: Option<&'a str>,
        datatype: Option<&'a str>,
        inlist: Option<&'a wasm_bindgen::JsValue>,
        prefix: Option<&'a str>,
        property: Option<&'a str>,
        resource: Option<&'a str>,
        type_of@"typeof": Option<&'a str>,
        vocab: Option<&'a str>,

        // Non-standard Attributes
        auto_capitalize: Option<&'a str>,
        auto_correct: Option<&'a str>,
        auto_save: Option<&'a str>,
        color: Option<&'a str>,
        item_prop: Option<&'a str>,
        item_scope: Option<bool>,
        item_type: Option<&'a str>,
        item_id: Option<&'a str>,
        item_ref: Option<&'a str>,
        results: Option<i32>,
        security: Option<&'a str>,
        unselectable: Option<&'a str>, // TODO: ser: 'on' | 'off' | undefined;

        // Living Standard
        input_mode: Option<crate::html::HtmlInputMode>,
        is: Option<&'a str>,
    }
}
