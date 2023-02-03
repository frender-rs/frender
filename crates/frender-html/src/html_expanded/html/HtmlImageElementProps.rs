#[allow(non_snake_case)]
pub fn HtmlImageElementProps() -> Building<TypesInitial> {
    #[allow(unused_imports)]
    use super::*;
    self::Building(self::Data {
        HtmlElementProps: HtmlElementProps::build(HtmlElementProps()),
        alt: (),
        cross_origin: (),
        decoding: (),
        element_timing: (),
        height: (),
        is_map: (),
        loading: (),
        referrer_policy: (),
        sizes: (),
        src: (),
        srcset: (),
        width: (),
        use_map: (),
    })
}
pub mod prelude {}
pub mod overwrite {
    #![allow(non_camel_case_types)]
    pub type HtmlElementProps<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = Value,
        alt = <TypeDefs as super::Types>::alt,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        decoding = <TypeDefs as super::Types>::decoding,
        element_timing = <TypeDefs as super::Types>::element_timing,
        height = <TypeDefs as super::Types>::height,
        is_map = <TypeDefs as super::Types>::is_map,
        loading = <TypeDefs as super::Types>::loading,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        sizes = <TypeDefs as super::Types>::sizes,
        src = <TypeDefs as super::Types>::src,
        srcset = <TypeDefs as super::Types>::srcset,
        width = <TypeDefs as super::Types>::width,
        use_map = <TypeDefs as super::Types>::use_map,
    >;
    pub type ElementProps<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::ElementProps<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type children<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::children<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type class<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::class<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type id<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::id<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type part<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::part<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type access_key<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::access_key<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type auto_capitalize<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::auto_capitalize<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type auto_focus<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::auto_focus<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type content_editable<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::content_editable<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type context_menu<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::context_menu<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type dir<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::dir<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type draggable<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::draggable<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type enter_key_hint<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::enter_key_hint<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type hidden<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::hidden<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type inert<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::inert<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type input_mode<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::input_mode<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type is<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::is<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type item_id<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::item_id<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type item_prop<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::item_prop<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type item_ref<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::item_ref<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type item_scope<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::item_scope<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type item_type<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::item_type<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type lang<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::lang<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type nonce<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::nonce<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type role<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::role<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type slot<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::slot<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type spellcheck<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::spellcheck<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type style<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::style<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type tab_index<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::tab_index<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type title<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::title<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type translate<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::translate<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type virtual_keyboard_policy<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::virtual_keyboard_policy<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_click<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_click<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type alt<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        alt = Value,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        decoding = <TypeDefs as super::Types>::decoding,
        element_timing = <TypeDefs as super::Types>::element_timing,
        height = <TypeDefs as super::Types>::height,
        is_map = <TypeDefs as super::Types>::is_map,
        loading = <TypeDefs as super::Types>::loading,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        sizes = <TypeDefs as super::Types>::sizes,
        src = <TypeDefs as super::Types>::src,
        srcset = <TypeDefs as super::Types>::srcset,
        width = <TypeDefs as super::Types>::width,
        use_map = <TypeDefs as super::Types>::use_map,
    >;
    pub type cross_origin<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        alt = <TypeDefs as super::Types>::alt,
        cross_origin = Value,
        decoding = <TypeDefs as super::Types>::decoding,
        element_timing = <TypeDefs as super::Types>::element_timing,
        height = <TypeDefs as super::Types>::height,
        is_map = <TypeDefs as super::Types>::is_map,
        loading = <TypeDefs as super::Types>::loading,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        sizes = <TypeDefs as super::Types>::sizes,
        src = <TypeDefs as super::Types>::src,
        srcset = <TypeDefs as super::Types>::srcset,
        width = <TypeDefs as super::Types>::width,
        use_map = <TypeDefs as super::Types>::use_map,
    >;
    pub type decoding<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        alt = <TypeDefs as super::Types>::alt,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        decoding = Value,
        element_timing = <TypeDefs as super::Types>::element_timing,
        height = <TypeDefs as super::Types>::height,
        is_map = <TypeDefs as super::Types>::is_map,
        loading = <TypeDefs as super::Types>::loading,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        sizes = <TypeDefs as super::Types>::sizes,
        src = <TypeDefs as super::Types>::src,
        srcset = <TypeDefs as super::Types>::srcset,
        width = <TypeDefs as super::Types>::width,
        use_map = <TypeDefs as super::Types>::use_map,
    >;
    pub type element_timing<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        alt = <TypeDefs as super::Types>::alt,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        decoding = <TypeDefs as super::Types>::decoding,
        element_timing = Value,
        height = <TypeDefs as super::Types>::height,
        is_map = <TypeDefs as super::Types>::is_map,
        loading = <TypeDefs as super::Types>::loading,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        sizes = <TypeDefs as super::Types>::sizes,
        src = <TypeDefs as super::Types>::src,
        srcset = <TypeDefs as super::Types>::srcset,
        width = <TypeDefs as super::Types>::width,
        use_map = <TypeDefs as super::Types>::use_map,
    >;
    pub type height<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        alt = <TypeDefs as super::Types>::alt,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        decoding = <TypeDefs as super::Types>::decoding,
        element_timing = <TypeDefs as super::Types>::element_timing,
        height = Value,
        is_map = <TypeDefs as super::Types>::is_map,
        loading = <TypeDefs as super::Types>::loading,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        sizes = <TypeDefs as super::Types>::sizes,
        src = <TypeDefs as super::Types>::src,
        srcset = <TypeDefs as super::Types>::srcset,
        width = <TypeDefs as super::Types>::width,
        use_map = <TypeDefs as super::Types>::use_map,
    >;
    pub type is_map<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        alt = <TypeDefs as super::Types>::alt,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        decoding = <TypeDefs as super::Types>::decoding,
        element_timing = <TypeDefs as super::Types>::element_timing,
        height = <TypeDefs as super::Types>::height,
        is_map = Value,
        loading = <TypeDefs as super::Types>::loading,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        sizes = <TypeDefs as super::Types>::sizes,
        src = <TypeDefs as super::Types>::src,
        srcset = <TypeDefs as super::Types>::srcset,
        width = <TypeDefs as super::Types>::width,
        use_map = <TypeDefs as super::Types>::use_map,
    >;
    pub type loading<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        alt = <TypeDefs as super::Types>::alt,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        decoding = <TypeDefs as super::Types>::decoding,
        element_timing = <TypeDefs as super::Types>::element_timing,
        height = <TypeDefs as super::Types>::height,
        is_map = <TypeDefs as super::Types>::is_map,
        loading = Value,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        sizes = <TypeDefs as super::Types>::sizes,
        src = <TypeDefs as super::Types>::src,
        srcset = <TypeDefs as super::Types>::srcset,
        width = <TypeDefs as super::Types>::width,
        use_map = <TypeDefs as super::Types>::use_map,
    >;
    pub type referrer_policy<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        alt = <TypeDefs as super::Types>::alt,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        decoding = <TypeDefs as super::Types>::decoding,
        element_timing = <TypeDefs as super::Types>::element_timing,
        height = <TypeDefs as super::Types>::height,
        is_map = <TypeDefs as super::Types>::is_map,
        loading = <TypeDefs as super::Types>::loading,
        referrer_policy = Value,
        sizes = <TypeDefs as super::Types>::sizes,
        src = <TypeDefs as super::Types>::src,
        srcset = <TypeDefs as super::Types>::srcset,
        width = <TypeDefs as super::Types>::width,
        use_map = <TypeDefs as super::Types>::use_map,
    >;
    pub type sizes<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        alt = <TypeDefs as super::Types>::alt,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        decoding = <TypeDefs as super::Types>::decoding,
        element_timing = <TypeDefs as super::Types>::element_timing,
        height = <TypeDefs as super::Types>::height,
        is_map = <TypeDefs as super::Types>::is_map,
        loading = <TypeDefs as super::Types>::loading,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        sizes = Value,
        src = <TypeDefs as super::Types>::src,
        srcset = <TypeDefs as super::Types>::srcset,
        width = <TypeDefs as super::Types>::width,
        use_map = <TypeDefs as super::Types>::use_map,
    >;
    pub type src<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        alt = <TypeDefs as super::Types>::alt,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        decoding = <TypeDefs as super::Types>::decoding,
        element_timing = <TypeDefs as super::Types>::element_timing,
        height = <TypeDefs as super::Types>::height,
        is_map = <TypeDefs as super::Types>::is_map,
        loading = <TypeDefs as super::Types>::loading,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        sizes = <TypeDefs as super::Types>::sizes,
        src = Value,
        srcset = <TypeDefs as super::Types>::srcset,
        width = <TypeDefs as super::Types>::width,
        use_map = <TypeDefs as super::Types>::use_map,
    >;
    pub type srcset<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        alt = <TypeDefs as super::Types>::alt,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        decoding = <TypeDefs as super::Types>::decoding,
        element_timing = <TypeDefs as super::Types>::element_timing,
        height = <TypeDefs as super::Types>::height,
        is_map = <TypeDefs as super::Types>::is_map,
        loading = <TypeDefs as super::Types>::loading,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        sizes = <TypeDefs as super::Types>::sizes,
        src = <TypeDefs as super::Types>::src,
        srcset = Value,
        width = <TypeDefs as super::Types>::width,
        use_map = <TypeDefs as super::Types>::use_map,
    >;
    pub type width<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        alt = <TypeDefs as super::Types>::alt,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        decoding = <TypeDefs as super::Types>::decoding,
        element_timing = <TypeDefs as super::Types>::element_timing,
        height = <TypeDefs as super::Types>::height,
        is_map = <TypeDefs as super::Types>::is_map,
        loading = <TypeDefs as super::Types>::loading,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        sizes = <TypeDefs as super::Types>::sizes,
        src = <TypeDefs as super::Types>::src,
        srcset = <TypeDefs as super::Types>::srcset,
        width = Value,
        use_map = <TypeDefs as super::Types>::use_map,
    >;
    pub type use_map<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        alt = <TypeDefs as super::Types>::alt,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        decoding = <TypeDefs as super::Types>::decoding,
        element_timing = <TypeDefs as super::Types>::element_timing,
        height = <TypeDefs as super::Types>::height,
        is_map = <TypeDefs as super::Types>::is_map,
        loading = <TypeDefs as super::Types>::loading,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        sizes = <TypeDefs as super::Types>::sizes,
        src = <TypeDefs as super::Types>::src,
        srcset = <TypeDefs as super::Types>::srcset,
        width = <TypeDefs as super::Types>::width,
        use_map = Value,
    >;
}
mod trait_types {
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub trait Types {
        type HtmlElementProps: ?::core::marker::Sized + HtmlElementProps::Types;
        type alt: crate::MaybeUpdateValue<str>;
        type cross_origin: crate::MaybeUpdateValue<str>;
        type decoding: crate::MaybeUpdateValue<str>;
        type element_timing: crate::MaybeUpdateValue<str>;
        type height: crate::MaybeUpdateValue<u32>;
        type is_map: crate::MaybeUpdateValue<bool>;
        type loading: crate::MaybeUpdateValue<str>;
        type referrer_policy: crate::MaybeUpdateValue<str>;
        type sizes: crate::MaybeUpdateValue<str>;
        type src: crate::MaybeUpdateValue<str>;
        type srcset: crate::MaybeUpdateValue<str>;
        type width: crate::MaybeUpdateValue<u32>;
        type use_map: crate::MaybeUpdateValue<str>;
    }
}
pub use trait_types::Types;
pub use trait_types::Types as ValidTypes;
pub mod data_struct {
    #[non_exhaustive]
    pub struct HtmlImageElementProps<TypeDefs: super::Types + ?::core::marker::Sized> {
        pub HtmlElementProps: super::super::HtmlElementProps::Data<TypeDefs::HtmlElementProps>,
        pub alt: TypeDefs::alt,
        pub cross_origin: TypeDefs::cross_origin,
        pub decoding: TypeDefs::decoding,
        pub element_timing: TypeDefs::element_timing,
        pub height: TypeDefs::height,
        pub is_map: TypeDefs::is_map,
        pub loading: TypeDefs::loading,
        pub referrer_policy: TypeDefs::referrer_policy,
        pub sizes: TypeDefs::sizes,
        pub src: TypeDefs::src,
        pub srcset: TypeDefs::srcset,
        pub width: TypeDefs::width,
        pub use_map: TypeDefs::use_map,
    }
}
pub use data_struct::HtmlImageElementProps as Data;
pub struct Building<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
pub struct Replacing<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
mod types_initial {
    #[allow(unused_imports)]
    use super::super::*;
    pub type TypesInitial = dyn super::Types<
        HtmlElementProps = HtmlElementProps::TypesInitial,
        alt = (),
        cross_origin = (),
        decoding = (),
        element_timing = (),
        height = (),
        is_map = (),
        loading = (),
        referrer_policy = (),
        sizes = (),
        src = (),
        srcset = (),
        width = (),
        use_map = (),
    >;
}
pub use types_initial::TypesInitial;
pub type DataInitial = Data<TypesInitial>;
#[cfg(feature = "dom")]
pub mod render_state {
    #[allow(non_camel_case_types)]
    pub trait RenderStateTypes {
        type HtmlElementProps: ::core::default::Default
            + crate::props::IntrinsicComponentPollReactive;
        type alt: ::core::default::Default;
        type cross_origin: ::core::default::Default;
        type decoding: ::core::default::Default;
        type element_timing: ::core::default::Default;
        type height: ::core::default::Default;
        type is_map: ::core::default::Default;
        type loading: ::core::default::Default;
        type referrer_policy: ::core::default::Default;
        type sizes: ::core::default::Default;
        type src: ::core::default::Default;
        type srcset: ::core::default::Default;
        type width: ::core::default::Default;
        type use_map: ::core::default::Default;
    }
    pub struct RenderState<TypeDefs: RenderStateTypes>
    where
        TypeDefs: ?::core::marker::Sized,
    {
        pub HtmlElementProps: TypeDefs::HtmlElementProps,
        pub alt: TypeDefs::alt,
        pub cross_origin: TypeDefs::cross_origin,
        pub decoding: TypeDefs::decoding,
        pub element_timing: TypeDefs::element_timing,
        pub height: TypeDefs::height,
        pub is_map: TypeDefs::is_map,
        pub loading: TypeDefs::loading,
        pub referrer_policy: TypeDefs::referrer_policy,
        pub sizes: TypeDefs::sizes,
        pub src: TypeDefs::src,
        pub srcset: TypeDefs::srcset,
        pub width: TypeDefs::width,
        pub use_map: TypeDefs::use_map,
    }
    #[allow(dead_code)]
    #[allow(single_use_lifetimes)]
    #[allow(clippy::unknown_clippy_lints)]
    #[allow(clippy::mut_mut)]
    #[allow(clippy::redundant_pub_crate)]
    #[allow(clippy::ref_option_ref)]
    #[allow(clippy::type_repetition_in_bounds)]
    pub(crate) struct RenderStateProj<'__pin, TypeDefs: RenderStateTypes>
    where
        RenderState<TypeDefs>: '__pin,
        TypeDefs: ?::core::marker::Sized,
    {
        pub HtmlElementProps:
            ::pin_project_lite::__private::Pin<&'__pin mut (TypeDefs::HtmlElementProps)>,
        pub alt: &'__pin mut (TypeDefs::alt),
        pub cross_origin: &'__pin mut (TypeDefs::cross_origin),
        pub decoding: &'__pin mut (TypeDefs::decoding),
        pub element_timing: &'__pin mut (TypeDefs::element_timing),
        pub height: &'__pin mut (TypeDefs::height),
        pub is_map: &'__pin mut (TypeDefs::is_map),
        pub loading: &'__pin mut (TypeDefs::loading),
        pub referrer_policy: &'__pin mut (TypeDefs::referrer_policy),
        pub sizes: &'__pin mut (TypeDefs::sizes),
        pub src: &'__pin mut (TypeDefs::src),
        pub srcset: &'__pin mut (TypeDefs::srcset),
        pub width: &'__pin mut (TypeDefs::width),
        pub use_map: &'__pin mut (TypeDefs::use_map),
    }
    #[allow(explicit_outlives_requirements)]
    #[allow(single_use_lifetimes)]
    #[allow(clippy::unknown_clippy_lints)]
    #[allow(clippy::redundant_pub_crate)]
    #[allow(clippy::used_underscore_binding)]
    const _: () = {
        #[allow(dead_code)]
        #[allow(single_use_lifetimes)]
        #[allow(clippy::unknown_clippy_lints)]
        #[allow(clippy::mut_mut)]
        #[allow(clippy::redundant_pub_crate)]
        #[allow(clippy::ref_option_ref)]
        #[allow(clippy::type_repetition_in_bounds)]
        pub(crate) struct ProjectionRef<'__pin, TypeDefs: RenderStateTypes>
        where
            RenderState<TypeDefs>: '__pin,
            TypeDefs: ?::core::marker::Sized,
        {
            pub HtmlElementProps:
                ::pin_project_lite::__private::Pin<&'__pin (TypeDefs::HtmlElementProps)>,
            pub alt: &'__pin (TypeDefs::alt),
            pub cross_origin: &'__pin (TypeDefs::cross_origin),
            pub decoding: &'__pin (TypeDefs::decoding),
            pub element_timing: &'__pin (TypeDefs::element_timing),
            pub height: &'__pin (TypeDefs::height),
            pub is_map: &'__pin (TypeDefs::is_map),
            pub loading: &'__pin (TypeDefs::loading),
            pub referrer_policy: &'__pin (TypeDefs::referrer_policy),
            pub sizes: &'__pin (TypeDefs::sizes),
            pub src: &'__pin (TypeDefs::src),
            pub srcset: &'__pin (TypeDefs::srcset),
            pub width: &'__pin (TypeDefs::width),
            pub use_map: &'__pin (TypeDefs::use_map),
        }
        impl<TypeDefs: RenderStateTypes> RenderState<TypeDefs>
        where
            TypeDefs: ?::core::marker::Sized,
        {
            pub(crate) fn project<'__pin>(
                self: ::pin_project_lite::__private::Pin<&'__pin mut Self>,
            ) -> RenderStateProj<'__pin, TypeDefs> {
                unsafe {
                    let Self {
                        HtmlElementProps,
                        alt,
                        cross_origin,
                        decoding,
                        element_timing,
                        height,
                        is_map,
                        loading,
                        referrer_policy,
                        sizes,
                        src,
                        srcset,
                        width,
                        use_map,
                    } = self.get_unchecked_mut();
                    RenderStateProj {
                        HtmlElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            HtmlElementProps,
                        ),
                        alt: alt,
                        cross_origin: cross_origin,
                        decoding: decoding,
                        element_timing: element_timing,
                        height: height,
                        is_map: is_map,
                        loading: loading,
                        referrer_policy: referrer_policy,
                        sizes: sizes,
                        src: src,
                        srcset: srcset,
                        width: width,
                        use_map: use_map,
                    }
                }
            }
            pub(crate) fn project_ref<'__pin>(
                self: ::pin_project_lite::__private::Pin<&'__pin Self>,
            ) -> ProjectionRef<'__pin, TypeDefs> {
                unsafe {
                    let Self {
                        HtmlElementProps,
                        alt,
                        cross_origin,
                        decoding,
                        element_timing,
                        height,
                        is_map,
                        loading,
                        referrer_policy,
                        sizes,
                        src,
                        srcset,
                        width,
                        use_map,
                    } = self.get_ref();
                    ProjectionRef {
                        HtmlElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            HtmlElementProps,
                        ),
                        alt: alt,
                        cross_origin: cross_origin,
                        decoding: decoding,
                        element_timing: element_timing,
                        height: height,
                        is_map: is_map,
                        loading: loading,
                        referrer_policy: referrer_policy,
                        sizes: sizes,
                        src: src,
                        srcset: srcset,
                        width: width,
                        use_map: use_map,
                    }
                }
            }
        }
        #[allow(non_snake_case)]
        pub struct __Origin<'__pin, TypeDefs: RenderStateTypes>
        where
            TypeDefs: ?::core::marker::Sized,
        {
            __dummy_lifetime: ::pin_project_lite::__private::PhantomData<&'__pin ()>,
            HtmlElementProps: TypeDefs::HtmlElementProps,
            alt: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::alt>,
            cross_origin: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::cross_origin>,
            decoding: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::decoding>,
            element_timing: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::element_timing>,
            height: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::height>,
            is_map: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::is_map>,
            loading: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::loading>,
            referrer_policy: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::referrer_policy>,
            sizes: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::sizes>,
            src: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::src>,
            srcset: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::srcset>,
            width: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::width>,
            use_map: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::use_map>,
        }
        impl<'__pin, TypeDefs: RenderStateTypes> ::pin_project_lite::__private::Unpin
            for RenderState<TypeDefs>
        where
            __Origin<'__pin, TypeDefs>: ::pin_project_lite::__private::Unpin,
            TypeDefs: ?::core::marker::Sized,
        {
        }
        trait MustNotImplDrop {}
        #[allow(clippy::drop_bounds, drop_bounds)]
        impl<T: ::pin_project_lite::__private::Drop> MustNotImplDrop for T {}
        impl<TypeDefs: RenderStateTypes> MustNotImplDrop for RenderState<TypeDefs> where
            TypeDefs: ?::core::marker::Sized
        {
        }
        #[forbid(unaligned_references, safe_packed_borrows)]
        fn __assert_not_repr_packed<TypeDefs: RenderStateTypes>(this: &RenderState<TypeDefs>)
        where
            TypeDefs: ?::core::marker::Sized,
        {
            let _ = &this.HtmlElementProps;
            let _ = &this.alt;
            let _ = &this.cross_origin;
            let _ = &this.decoding;
            let _ = &this.element_timing;
            let _ = &this.height;
            let _ = &this.is_map;
            let _ = &this.loading;
            let _ = &this.referrer_policy;
            let _ = &this.sizes;
            let _ = &this.src;
            let _ = &this.srcset;
            let _ = &this.width;
            let _ = &this.use_map;
        }
    };
    impl<TypeDefs: ?::core::marker::Sized + RenderStateTypes> RenderState<TypeDefs> {
        #[inline]
        pub(crate) fn pin_project(
            self: ::core::pin::Pin<&mut Self>,
        ) -> RenderStateProj<'_, TypeDefs> {
            self.project()
        }
    }
    impl<TypeDefs: ?::core::marker::Sized + RenderStateTypes> ::core::default::Default
        for RenderState<TypeDefs>
    {
        fn default() -> Self {
            Self {
                HtmlElementProps: ::core::default::Default::default(),
                alt: ::core::default::Default::default(),
                cross_origin: ::core::default::Default::default(),
                decoding: ::core::default::Default::default(),
                element_timing: ::core::default::Default::default(),
                height: ::core::default::Default::default(),
                is_map: ::core::default::Default::default(),
                loading: ::core::default::Default::default(),
                referrer_policy: ::core::default::Default::default(),
                sizes: ::core::default::Default::default(),
                src: ::core::default::Default::default(),
                srcset: ::core::default::Default::default(),
                width: ::core::default::Default::default(),
                use_map: ::core::default::Default::default(),
            }
        }
    }
    impl<TypeDefs: ?::core::marker::Sized + RenderStateTypes>
        crate::props::IntrinsicComponentPollReactive for RenderState<TypeDefs>
    {
        #[inline]
        fn intrinsic_component_poll_reactive(
            self: ::core::pin::Pin<&mut Self>,
            cx: &mut ::core::task::Context<'_>,
        ) -> ::core::task::Poll<bool> {
            crate::props::IntrinsicComponentPollReactive::intrinsic_component_poll_reactive(
                self.project().HtmlElementProps,
                cx,
            )
        }
    }
}
#[inline]
pub fn build<TypeDefs: ?::core::marker::Sized + Types>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    building.0
}
mod builder_and_replacer {
    #[allow(unused_imports)]
    use super::super::*;
    impl<TypeDefs: super::Types + ?::core::marker::Sized> super::Building<TypeDefs> {
        #[doc = "See [`HtmlElementProps::children`]"]
        #[inline]
        pub fn children<V>(
            self,
            children: V,
        ) -> super::Building<super::overwrite::children<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).children(children),
                ),
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[doc = "See [`HtmlElementProps::class`]"]
        #[inline]
        pub fn class<V: crate::MaybeUpdateValue<str>>(
            self,
            class: V,
        ) -> super::Building<super::overwrite::class<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).class(class),
                ),
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[doc = "See [`HtmlElementProps::id`]"]
        #[inline]
        pub fn id<V: crate::MaybeUpdateValue<str>>(
            self,
            id: V,
        ) -> super::Building<super::overwrite::id<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).id(id),
                ),
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[doc = "See [`HtmlElementProps::part`]"]
        #[inline]
        pub fn part<V: crate::MaybeUpdateValue<str>>(
            self,
            part: V,
        ) -> super::Building<super::overwrite::part<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).part(part),
                ),
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[doc = "See [`HtmlElementProps::access_key`]"]
        #[inline]
        pub fn access_key<V: crate::MaybeUpdateValue<str>>(
            self,
            access_key: V,
        ) -> super::Building<super::overwrite::access_key<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).access_key(access_key),
                ),
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[doc = "See [`HtmlElementProps::auto_capitalize`]"]
        #[inline]
        pub fn auto_capitalize<V: crate::MaybeUpdateValue<str>>(
            self,
            auto_capitalize: V,
        ) -> super::Building<super::overwrite::auto_capitalize<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps)
                        .auto_capitalize(auto_capitalize),
                ),
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[doc = "See [`HtmlElementProps::auto_focus`]"]
        #[inline]
        pub fn auto_focus<V: crate::MaybeUpdateValue<bool>>(
            self,
            auto_focus: V,
        ) -> super::Building<super::overwrite::auto_focus<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).auto_focus(auto_focus),
                ),
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[doc = "See [`HtmlElementProps::content_editable`]"]
        #[inline]
        pub fn content_editable<V: crate::props::MaybeInherit<bool>>(
            self,
            content_editable: V,
        ) -> super::Building<super::overwrite::content_editable<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps)
                        .content_editable(content_editable),
                ),
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[doc = "See [`HtmlElementProps::context_menu`]"]
        #[inline]
        pub fn context_menu<V: crate::MaybeUpdateValue<str>>(
            self,
            context_menu: V,
        ) -> super::Building<super::overwrite::context_menu<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).context_menu(context_menu),
                ),
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[doc = "See [`HtmlElementProps::dir`]"]
        #[inline]
        pub fn dir<V: crate::MaybeUpdateValue<str>>(
            self,
            dir: V,
        ) -> super::Building<super::overwrite::dir<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).dir(dir),
                ),
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[doc = "See [`HtmlElementProps::draggable`]"]
        #[inline]
        pub fn draggable<V: crate::MaybeUpdateValue<bool>>(
            self,
            draggable: V,
        ) -> super::Building<super::overwrite::draggable<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).draggable(draggable),
                ),
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[doc = "See [`HtmlElementProps::enter_key_hint`]"]
        #[inline]
        pub fn enter_key_hint<V: crate::MaybeUpdateValue<str>>(
            self,
            enter_key_hint: V,
        ) -> super::Building<super::overwrite::enter_key_hint<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps)
                        .enter_key_hint(enter_key_hint),
                ),
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[doc = "See [`HtmlElementProps::hidden`]"]
        #[inline]
        pub fn hidden<V: crate::MaybeUpdateValue<bool>>(
            self,
            hidden: V,
        ) -> super::Building<super::overwrite::hidden<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).hidden(hidden),
                ),
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[doc = "See [`HtmlElementProps::inert`]"]
        #[inline]
        pub fn inert<V: crate::MaybeUpdateValue<bool>>(
            self,
            inert: V,
        ) -> super::Building<super::overwrite::inert<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).inert(inert),
                ),
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[doc = "See [`HtmlElementProps::input_mode`]"]
        #[inline]
        pub fn input_mode<V: crate::MaybeUpdateValue<str>>(
            self,
            input_mode: V,
        ) -> super::Building<super::overwrite::input_mode<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).input_mode(input_mode),
                ),
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[doc = "See [`HtmlElementProps::is`]"]
        #[inline]
        pub fn is<V: crate::MaybeUpdateValue<str>>(
            self,
            is: V,
        ) -> super::Building<super::overwrite::is<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).is(is),
                ),
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[doc = "See [`HtmlElementProps::item_id`]"]
        #[inline]
        pub fn item_id<V: crate::MaybeUpdateValue<str>>(
            self,
            item_id: V,
        ) -> super::Building<super::overwrite::item_id<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).item_id(item_id),
                ),
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[doc = "See [`HtmlElementProps::item_prop`]"]
        #[inline]
        pub fn item_prop<V: crate::MaybeUpdateValue<str>>(
            self,
            item_prop: V,
        ) -> super::Building<super::overwrite::item_prop<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).item_prop(item_prop),
                ),
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[doc = "See [`HtmlElementProps::item_ref`]"]
        #[inline]
        pub fn item_ref<V: crate::MaybeUpdateValue<str>>(
            self,
            item_ref: V,
        ) -> super::Building<super::overwrite::item_ref<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).item_ref(item_ref),
                ),
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[doc = "See [`HtmlElementProps::item_scope`]"]
        #[inline]
        pub fn item_scope<V: crate::MaybeUpdateValue<str>>(
            self,
            item_scope: V,
        ) -> super::Building<super::overwrite::item_scope<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).item_scope(item_scope),
                ),
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[doc = "See [`HtmlElementProps::item_type`]"]
        #[inline]
        pub fn item_type<V: crate::MaybeUpdateValue<str>>(
            self,
            item_type: V,
        ) -> super::Building<super::overwrite::item_type<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).item_type(item_type),
                ),
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[doc = "See [`HtmlElementProps::lang`]"]
        #[inline]
        pub fn lang<V: crate::MaybeUpdateValue<str>>(
            self,
            lang: V,
        ) -> super::Building<super::overwrite::lang<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).lang(lang),
                ),
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[doc = "See [`HtmlElementProps::nonce`]"]
        #[inline]
        pub fn nonce<V: crate::MaybeUpdateValue<str>>(
            self,
            nonce: V,
        ) -> super::Building<super::overwrite::nonce<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).nonce(nonce),
                ),
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[doc = "See [`HtmlElementProps::role`]"]
        #[inline]
        pub fn role<V: crate::MaybeUpdateValue<str>>(
            self,
            role: V,
        ) -> super::Building<super::overwrite::role<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).role(role),
                ),
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[doc = "See [`HtmlElementProps::slot`]"]
        #[inline]
        pub fn slot<V: crate::MaybeUpdateValue<str>>(
            self,
            slot: V,
        ) -> super::Building<super::overwrite::slot<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).slot(slot),
                ),
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[doc = "See [`HtmlElementProps::spellcheck`]"]
        #[inline]
        pub fn spellcheck<V: crate::MaybeUpdateValue<bool>>(
            self,
            spellcheck: V,
        ) -> super::Building<super::overwrite::spellcheck<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).spellcheck(spellcheck),
                ),
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[doc = "See [`HtmlElementProps::style`]"]
        #[inline]
        pub fn style<V: crate::MaybeUpdateValue<str>>(
            self,
            style: V,
        ) -> super::Building<super::overwrite::style<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).style(style),
                ),
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[doc = "See [`HtmlElementProps::tab_index`]"]
        #[inline]
        pub fn tab_index<V: crate::MaybeUpdateValue<i32>>(
            self,
            tab_index: V,
        ) -> super::Building<super::overwrite::tab_index<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).tab_index(tab_index),
                ),
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[doc = "See [`HtmlElementProps::title`]"]
        #[inline]
        pub fn title<V: crate::MaybeUpdateValue<str>>(
            self,
            title: V,
        ) -> super::Building<super::overwrite::title<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).title(title),
                ),
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[doc = "See [`HtmlElementProps::translate`]"]
        #[inline]
        pub fn translate<V: crate::MaybeUpdateValue<str>>(
            self,
            translate: V,
        ) -> super::Building<super::overwrite::translate<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).translate(translate),
                ),
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[doc = "See [`HtmlElementProps::virtual_keyboard_policy`]"]
        #[inline]
        pub fn virtual_keyboard_policy<V: crate::MaybeUpdateValue<str>>(
            self,
            virtual_keyboard_policy: V,
        ) -> super::Building<super::overwrite::virtual_keyboard_policy<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps)
                        .virtual_keyboard_policy(virtual_keyboard_policy),
                ),
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[doc = "See [`HtmlElementProps::on_click`]"]
        #[inline]
        pub fn on_click<V>(
            self,
            on_click: V,
        ) -> super::Building<super::overwrite::on_click<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).on_click(on_click),
                ),
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[inline]
        pub fn alt<V: crate::MaybeUpdateValue<str>>(
            self,
            alt: V,
        ) -> super::Building<super::overwrite::alt<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[inline]
        pub fn cross_origin<V: crate::MaybeUpdateValue<str>>(
            self,
            cross_origin: V,
        ) -> super::Building<super::overwrite::cross_origin<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                alt: self.0.alt,
                cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[inline]
        pub fn decoding<V: crate::MaybeUpdateValue<str>>(
            self,
            decoding: V,
        ) -> super::Building<super::overwrite::decoding<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[inline]
        pub fn element_timing<V: crate::MaybeUpdateValue<str>>(
            self,
            element_timing: V,
        ) -> super::Building<super::overwrite::element_timing<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[inline]
        pub fn height<V: crate::MaybeUpdateValue<u32>>(
            self,
            height: V,
        ) -> super::Building<super::overwrite::height<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[inline]
        pub fn is_map<V: crate::MaybeUpdateValue<bool>>(
            self,
            is_map: V,
        ) -> super::Building<super::overwrite::is_map<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[inline]
        pub fn loading<V: crate::MaybeUpdateValue<str>>(
            self,
            loading: V,
        ) -> super::Building<super::overwrite::loading<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[inline]
        pub fn referrer_policy<V: crate::MaybeUpdateValue<str>>(
            self,
            referrer_policy: V,
        ) -> super::Building<super::overwrite::referrer_policy<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[inline]
        pub fn sizes<V: crate::MaybeUpdateValue<str>>(
            self,
            sizes: V,
        ) -> super::Building<super::overwrite::sizes<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[inline]
        pub fn src<V: crate::MaybeUpdateValue<str>>(
            self,
            src: V,
        ) -> super::Building<super::overwrite::src<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[inline]
        pub fn srcset<V: crate::MaybeUpdateValue<str>>(
            self,
            srcset: V,
        ) -> super::Building<super::overwrite::srcset<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset,
                width: self.0.width,
                use_map: self.0.use_map,
            })
        }
        #[inline]
        pub fn width<V: crate::MaybeUpdateValue<u32>>(
            self,
            width: V,
        ) -> super::Building<super::overwrite::width<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width,
                use_map: self.0.use_map,
            })
        }
        #[inline]
        pub fn use_map<V: crate::MaybeUpdateValue<str>>(
            self,
            use_map: V,
        ) -> super::Building<super::overwrite::use_map<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                alt: self.0.alt,
                cross_origin: self.0.cross_origin,
                decoding: self.0.decoding,
                element_timing: self.0.element_timing,
                height: self.0.height,
                is_map: self.0.is_map,
                loading: self.0.loading,
                referrer_policy: self.0.referrer_policy,
                sizes: self.0.sizes,
                src: self.0.src,
                srcset: self.0.srcset,
                width: self.0.width,
                use_map,
            })
        }
    }
}
#[cfg(feature = "dom")]
mod impl_update_element {
    #[allow(unused_imports)]
    use super::super::*;
    impl<TypeDefs: ?::core::marker::Sized + super::Types>
        crate::props::UpdateElement<web_sys::HtmlImageElement> for super::Data<TypeDefs>
    where
        HtmlElementProps::Data<TypeDefs::HtmlElementProps>:
            crate::props::UpdateElement<web_sys::HtmlElement>,
    {
        type State = super :: render_state :: RenderState < dyn super :: render_state :: RenderStateTypes < HtmlElementProps = < HtmlElementProps :: Data < TypeDefs :: HtmlElementProps , > as crate :: props :: UpdateElement < web_sys :: HtmlElement > > :: State , alt = < TypeDefs :: alt as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , cross_origin = < TypeDefs :: cross_origin as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , decoding = < TypeDefs :: decoding as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , element_timing = < TypeDefs :: element_timing as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , height = < TypeDefs :: height as :: frender_dom :: props :: MaybeUpdateValue < u32 , > > :: State , is_map = < TypeDefs :: is_map as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: State , loading = < TypeDefs :: loading as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , referrer_policy = < TypeDefs :: referrer_policy as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , sizes = < TypeDefs :: sizes as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , src = < TypeDefs :: src as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , srcset = < TypeDefs :: srcset as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , width = < TypeDefs :: width as :: frender_dom :: props :: MaybeUpdateValue < u32 , > > :: State , use_map = < TypeDefs :: use_map as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , > , > ;
        fn update_element(
            this: Self,
            element: &web_sys::HtmlImageElement,
            children_ctx: &mut ::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let state = state.pin_project();
            let dom_element: &::web_sys::Element = element.as_ref();
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.HtmlElementProps,
                state: state.HtmlElementProps,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    state,
                    element,
                    children_ctx,
                    ..
                } => crate::props::UpdateElement::update_element(
                    data,
                    element.as_ref(),
                    children_ctx,
                    state,
                ),
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.alt,
                state: state.alt,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "alt";
                    < TypeDefs :: alt as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | element . set_alt (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.cross_origin,
                state: state.cross_origin,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "crossorigin";
                    < TypeDefs :: cross_origin as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , match element { el => | v : & _ | el . set_cross_origin (Some (v)) , } , match element { el => | | el . set_cross_origin (None) , } ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.decoding,
                state: state.decoding,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "decoding";
                    < TypeDefs :: decoding as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | element . set_decoding (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.element_timing,
                state: state.element_timing,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "elementtiming";
                    < TypeDefs :: element_timing as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.height,
                state: state.height,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "height";
                    < TypeDefs :: height as :: frender_dom :: props :: MaybeUpdateValue < u32 , > > :: maybe_update_value (data , state , | v | element . set_height (* v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.is_map,
                state: state.is_map,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "ismap";
                    < TypeDefs :: is_map as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (data , state , | v | element . set_is_map (* v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.loading,
                state: state.loading,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "loading";
                    < TypeDefs :: loading as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.referrer_policy,
                state: state.referrer_policy,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "referrerpolicy";
                    < TypeDefs :: referrer_policy as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | element . set_referrer_policy (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.sizes,
                state: state.sizes,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "sizes";
                    < TypeDefs :: sizes as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | element . set_sizes (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.src,
                state: state.src,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "src";
                    < TypeDefs :: src as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | element . set_src (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.srcset,
                state: state.srcset,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "srcset";
                    < TypeDefs :: srcset as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | element . set_srcset (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.width,
                state: state.width,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "width";
                    < TypeDefs :: width as :: frender_dom :: props :: MaybeUpdateValue < u32 , > > :: maybe_update_value (data , state , | v | element . set_width (* v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.use_map,
                state: state.use_map,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    dom_element,
                    state,
                    element,
                    ..
                } => {
                    #[allow(unused)]
                    const ATTR_NAME: &::core::primitive::str = "usemap";
                    < TypeDefs :: use_map as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | element . set_use_map (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
        }
    }
}