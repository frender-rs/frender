pub trait RenderFrom<V> {
    fn render_from(self, value: V);
}

pub mod str {
    use std::{borrow::Cow, rc::Rc, sync::Arc};

    use frender_reactive_value::{
        static_or_temp_ref::StaticOrTempRef, temp_ref::TempRef, value_kind::ValueKind,
    };

    use crate::string_element::StringElement;

    use super::RenderFrom;

    macro_rules! define {
    (
        $vis:vis trait $Trait:ident : $Bound:ident {
            $(
                type From $(<$lt:lifetime>)? = $From:ty;
                // $(type Kind = $Kind:ty;)+
            )*
        }

        // impl $(<__>)? $KnownValueKind:ident for Kind $body:tt

        impl $(<__>)? $KnownValue:ident for Value $value_body:tt
    ) => {
        $vis trait $Trait:
        $(
            $(for <$lt>)? $Bound<$From> +
        )+
        {}

        impl<T: ?Sized> $Trait for T
        where T:
        $(
            $(for <$lt>)? $Bound<$From> +
        )+
        {}

        // $($(
        //     impl $KnownValueKind for $Kind
        //     $body
        // )+)*

        $(
            impl $(<$lt>)? $KnownValue for $From
            $value_body
        )*
    };
}

    define!(
        pub trait RenderFromKnownStr: RenderFrom {
            type From = &'static str;
            // type Kind = KindOfOwned<&'static str>;

            type From<'a> = TempRef<'a, str>;
            // type Kind = str;

            type From = StringElement;
            // type Kind = KindOfOwned<StringElement>;

            type From<'a> = &'a StringElement;
            // type Kind = KindOfRef<StringElement>;

            type From = String;
            // type Kind = KindOfOwned<String>;

            type From = Cow<'static, str>;
            // type Kind = KindOfOwned<Cow<'static, str>>;

            type From<'a> = StaticOrTempRef<'a, str>;
            // type Kind = KindOfStaticRefOrTempOwned<str>;

            type From = Rc<str>;
            // type Kind = KindOfOwned<Rc<str>>;

            type From<'a> = &'a Rc<str>;
            // type Kind = KindOfRef<Rc<str>>;

            type From = Arc<str>;
            // type Kind = KindOfOwned<Arc<str>>;

            type From<'a> = &'a Arc<str>;
            // type Kind = KindOfRef<Arc<str>>;
        }

        // impl<__> KnownValueKindForInnerHtml for Kind {}

        impl<__> ValueForStr for Value {
            fn render_str_from_self(self, renderer: impl RenderFromKnownStr) {
                renderer.render_from(self);
            }
        }
    );

    pub trait ValueForStr {
        fn render_str_from_self(self, renderer: impl RenderFromKnownStr);
    }

    pub trait ValueKindForStr: for<'a> ValueKind<Value<'a>: ValueForStr> {}
    impl<VK: ?Sized + for<'a> ValueKind<Value<'a>: ValueForStr>> ValueKindForStr for VK {}
}
