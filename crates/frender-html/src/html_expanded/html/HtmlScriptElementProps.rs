#[allow(non_snake_case)]
pub fn HtmlScriptElementProps() -> Building<TypesInitial> {
    #[allow(unused_imports)]
    use super::*;
    self::Building(self::Data {
        HtmlElementProps: HtmlElementProps::build(HtmlElementProps()),
        r#async: (),
        cross_origin: (),
        defer: (),
        fetch_priority: (),
        integrity: (),
        no_module: (),
        referrer_policy: (),
        src: (),
        type_: (),
        blocking: (),
    })
}
pub mod prelude {}
pub mod overwrite {
    #![allow(non_camel_case_types)]
    pub type HtmlElementProps<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = Value,
        r#async = <TypeDefs as super::Types>::r#async,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        defer = <TypeDefs as super::Types>::defer,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        integrity = <TypeDefs as super::Types>::integrity,
        no_module = <TypeDefs as super::Types>::no_module,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        src = <TypeDefs as super::Types>::src,
        type_ = <TypeDefs as super::Types>::type_,
        blocking = <TypeDefs as super::Types>::blocking,
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
    pub type r#async<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        r#async = Value,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        defer = <TypeDefs as super::Types>::defer,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        integrity = <TypeDefs as super::Types>::integrity,
        no_module = <TypeDefs as super::Types>::no_module,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        src = <TypeDefs as super::Types>::src,
        type_ = <TypeDefs as super::Types>::type_,
        blocking = <TypeDefs as super::Types>::blocking,
    >;
    pub type cross_origin<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        r#async = <TypeDefs as super::Types>::r#async,
        cross_origin = Value,
        defer = <TypeDefs as super::Types>::defer,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        integrity = <TypeDefs as super::Types>::integrity,
        no_module = <TypeDefs as super::Types>::no_module,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        src = <TypeDefs as super::Types>::src,
        type_ = <TypeDefs as super::Types>::type_,
        blocking = <TypeDefs as super::Types>::blocking,
    >;
    pub type defer<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        r#async = <TypeDefs as super::Types>::r#async,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        defer = Value,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        integrity = <TypeDefs as super::Types>::integrity,
        no_module = <TypeDefs as super::Types>::no_module,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        src = <TypeDefs as super::Types>::src,
        type_ = <TypeDefs as super::Types>::type_,
        blocking = <TypeDefs as super::Types>::blocking,
    >;
    pub type fetch_priority<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        r#async = <TypeDefs as super::Types>::r#async,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        defer = <TypeDefs as super::Types>::defer,
        fetch_priority = Value,
        integrity = <TypeDefs as super::Types>::integrity,
        no_module = <TypeDefs as super::Types>::no_module,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        src = <TypeDefs as super::Types>::src,
        type_ = <TypeDefs as super::Types>::type_,
        blocking = <TypeDefs as super::Types>::blocking,
    >;
    pub type integrity<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        r#async = <TypeDefs as super::Types>::r#async,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        defer = <TypeDefs as super::Types>::defer,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        integrity = Value,
        no_module = <TypeDefs as super::Types>::no_module,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        src = <TypeDefs as super::Types>::src,
        type_ = <TypeDefs as super::Types>::type_,
        blocking = <TypeDefs as super::Types>::blocking,
    >;
    pub type no_module<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        r#async = <TypeDefs as super::Types>::r#async,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        defer = <TypeDefs as super::Types>::defer,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        integrity = <TypeDefs as super::Types>::integrity,
        no_module = Value,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        src = <TypeDefs as super::Types>::src,
        type_ = <TypeDefs as super::Types>::type_,
        blocking = <TypeDefs as super::Types>::blocking,
    >;
    pub type referrer_policy<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        r#async = <TypeDefs as super::Types>::r#async,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        defer = <TypeDefs as super::Types>::defer,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        integrity = <TypeDefs as super::Types>::integrity,
        no_module = <TypeDefs as super::Types>::no_module,
        referrer_policy = Value,
        src = <TypeDefs as super::Types>::src,
        type_ = <TypeDefs as super::Types>::type_,
        blocking = <TypeDefs as super::Types>::blocking,
    >;
    pub type src<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        r#async = <TypeDefs as super::Types>::r#async,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        defer = <TypeDefs as super::Types>::defer,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        integrity = <TypeDefs as super::Types>::integrity,
        no_module = <TypeDefs as super::Types>::no_module,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        src = Value,
        type_ = <TypeDefs as super::Types>::type_,
        blocking = <TypeDefs as super::Types>::blocking,
    >;
    pub type type_<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        r#async = <TypeDefs as super::Types>::r#async,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        defer = <TypeDefs as super::Types>::defer,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        integrity = <TypeDefs as super::Types>::integrity,
        no_module = <TypeDefs as super::Types>::no_module,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        src = <TypeDefs as super::Types>::src,
        type_ = Value,
        blocking = <TypeDefs as super::Types>::blocking,
    >;
    pub type blocking<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        r#async = <TypeDefs as super::Types>::r#async,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        defer = <TypeDefs as super::Types>::defer,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        integrity = <TypeDefs as super::Types>::integrity,
        no_module = <TypeDefs as super::Types>::no_module,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        src = <TypeDefs as super::Types>::src,
        type_ = <TypeDefs as super::Types>::type_,
        blocking = Value,
    >;
}
mod trait_types {
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub trait Types {
        type HtmlElementProps: ?::core::marker::Sized + HtmlElementProps::Types;
        type r#async: crate::MaybeUpdateValue<bool>;
        type cross_origin: crate::MaybeUpdateValue<str>;
        type defer: crate::MaybeUpdateValue<bool>;
        type fetch_priority: crate::MaybeUpdateValue<str>;
        type integrity: crate::MaybeUpdateValue<str>;
        type no_module: crate::MaybeUpdateValue<bool>;
        type referrer_policy: crate::MaybeUpdateValue<str>;
        type src: crate::MaybeUpdateValue<str>;
        type type_: crate::MaybeUpdateValue<str>;
        type blocking: crate::MaybeUpdateValue<str>;
    }
}
pub use trait_types::Types;
pub use trait_types::Types as ValidTypes;
pub mod data_struct {
    #[non_exhaustive]
    pub struct HtmlScriptElementProps<TypeDefs: super::Types + ?::core::marker::Sized> {
        pub HtmlElementProps: super::super::HtmlElementProps::Data<TypeDefs::HtmlElementProps>,
        pub r#async: TypeDefs::r#async,
        pub cross_origin: TypeDefs::cross_origin,
        pub defer: TypeDefs::defer,
        pub fetch_priority: TypeDefs::fetch_priority,
        pub integrity: TypeDefs::integrity,
        pub no_module: TypeDefs::no_module,
        pub referrer_policy: TypeDefs::referrer_policy,
        pub src: TypeDefs::src,
        pub type_: TypeDefs::type_,
        pub blocking: TypeDefs::blocking,
    }
}
pub use data_struct::HtmlScriptElementProps as Data;
pub struct Building<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
pub struct Replacing<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
mod types_initial {
    #[allow(unused_imports)]
    use super::super::*;
    pub type TypesInitial = dyn super::Types<
        HtmlElementProps = HtmlElementProps::TypesInitial,
        r#async = (),
        cross_origin = (),
        defer = (),
        fetch_priority = (),
        integrity = (),
        no_module = (),
        referrer_policy = (),
        src = (),
        type_ = (),
        blocking = (),
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
        type r#async: ::core::default::Default;
        type cross_origin: ::core::default::Default;
        type defer: ::core::default::Default;
        type fetch_priority: ::core::default::Default;
        type integrity: ::core::default::Default;
        type no_module: ::core::default::Default;
        type referrer_policy: ::core::default::Default;
        type src: ::core::default::Default;
        type type_: ::core::default::Default;
        type blocking: ::core::default::Default;
    }
    pub struct RenderState<TypeDefs: RenderStateTypes>
    where
        TypeDefs: ?::core::marker::Sized,
    {
        pub HtmlElementProps: TypeDefs::HtmlElementProps,
        pub r#async: TypeDefs::r#async,
        pub cross_origin: TypeDefs::cross_origin,
        pub defer: TypeDefs::defer,
        pub fetch_priority: TypeDefs::fetch_priority,
        pub integrity: TypeDefs::integrity,
        pub no_module: TypeDefs::no_module,
        pub referrer_policy: TypeDefs::referrer_policy,
        pub src: TypeDefs::src,
        pub type_: TypeDefs::type_,
        pub blocking: TypeDefs::blocking,
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
        pub r#async: &'__pin mut (TypeDefs::r#async),
        pub cross_origin: &'__pin mut (TypeDefs::cross_origin),
        pub defer: &'__pin mut (TypeDefs::defer),
        pub fetch_priority: &'__pin mut (TypeDefs::fetch_priority),
        pub integrity: &'__pin mut (TypeDefs::integrity),
        pub no_module: &'__pin mut (TypeDefs::no_module),
        pub referrer_policy: &'__pin mut (TypeDefs::referrer_policy),
        pub src: &'__pin mut (TypeDefs::src),
        pub type_: &'__pin mut (TypeDefs::type_),
        pub blocking: &'__pin mut (TypeDefs::blocking),
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
            pub r#async: &'__pin (TypeDefs::r#async),
            pub cross_origin: &'__pin (TypeDefs::cross_origin),
            pub defer: &'__pin (TypeDefs::defer),
            pub fetch_priority: &'__pin (TypeDefs::fetch_priority),
            pub integrity: &'__pin (TypeDefs::integrity),
            pub no_module: &'__pin (TypeDefs::no_module),
            pub referrer_policy: &'__pin (TypeDefs::referrer_policy),
            pub src: &'__pin (TypeDefs::src),
            pub type_: &'__pin (TypeDefs::type_),
            pub blocking: &'__pin (TypeDefs::blocking),
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
                        r#async,
                        cross_origin,
                        defer,
                        fetch_priority,
                        integrity,
                        no_module,
                        referrer_policy,
                        src,
                        type_,
                        blocking,
                    } = self.get_unchecked_mut();
                    RenderStateProj {
                        HtmlElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            HtmlElementProps,
                        ),
                        r#async: r#async,
                        cross_origin: cross_origin,
                        defer: defer,
                        fetch_priority: fetch_priority,
                        integrity: integrity,
                        no_module: no_module,
                        referrer_policy: referrer_policy,
                        src: src,
                        type_: type_,
                        blocking: blocking,
                    }
                }
            }
            pub(crate) fn project_ref<'__pin>(
                self: ::pin_project_lite::__private::Pin<&'__pin Self>,
            ) -> ProjectionRef<'__pin, TypeDefs> {
                unsafe {
                    let Self {
                        HtmlElementProps,
                        r#async,
                        cross_origin,
                        defer,
                        fetch_priority,
                        integrity,
                        no_module,
                        referrer_policy,
                        src,
                        type_,
                        blocking,
                    } = self.get_ref();
                    ProjectionRef {
                        HtmlElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            HtmlElementProps,
                        ),
                        r#async: r#async,
                        cross_origin: cross_origin,
                        defer: defer,
                        fetch_priority: fetch_priority,
                        integrity: integrity,
                        no_module: no_module,
                        referrer_policy: referrer_policy,
                        src: src,
                        type_: type_,
                        blocking: blocking,
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
            r#async: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::r#async>,
            cross_origin: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::cross_origin>,
            defer: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::defer>,
            fetch_priority: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::fetch_priority>,
            integrity: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::integrity>,
            no_module: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::no_module>,
            referrer_policy: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::referrer_policy>,
            src: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::src>,
            type_: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::type_>,
            blocking: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::blocking>,
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
            let _ = &this.r#async;
            let _ = &this.cross_origin;
            let _ = &this.defer;
            let _ = &this.fetch_priority;
            let _ = &this.integrity;
            let _ = &this.no_module;
            let _ = &this.referrer_policy;
            let _ = &this.src;
            let _ = &this.type_;
            let _ = &this.blocking;
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
                r#async: ::core::default::Default::default(),
                cross_origin: ::core::default::Default::default(),
                defer: ::core::default::Default::default(),
                fetch_priority: ::core::default::Default::default(),
                integrity: ::core::default::Default::default(),
                no_module: ::core::default::Default::default(),
                referrer_policy: ::core::default::Default::default(),
                src: ::core::default::Default::default(),
                type_: ::core::default::Default::default(),
                blocking: ::core::default::Default::default(),
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
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking: self.0.blocking,
            })
        }
        #[inline]
        pub fn r#async<V: crate::MaybeUpdateValue<bool>>(
            self,
            r#async: V,
        ) -> super::Building<super::overwrite::r#async<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking: self.0.blocking,
            })
        }
        #[inline]
        pub fn cross_origin<V: crate::MaybeUpdateValue<str>>(
            self,
            cross_origin: V,
        ) -> super::Building<super::overwrite::cross_origin<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                r#async: self.0.r#async,
                cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking: self.0.blocking,
            })
        }
        #[inline]
        pub fn defer<V: crate::MaybeUpdateValue<bool>>(
            self,
            defer: V,
        ) -> super::Building<super::overwrite::defer<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking: self.0.blocking,
            })
        }
        #[inline]
        pub fn fetch_priority<V: crate::MaybeUpdateValue<str>>(
            self,
            fetch_priority: V,
        ) -> super::Building<super::overwrite::fetch_priority<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking: self.0.blocking,
            })
        }
        #[inline]
        pub fn integrity<V: crate::MaybeUpdateValue<str>>(
            self,
            integrity: V,
        ) -> super::Building<super::overwrite::integrity<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking: self.0.blocking,
            })
        }
        #[inline]
        pub fn no_module<V: crate::MaybeUpdateValue<bool>>(
            self,
            no_module: V,
        ) -> super::Building<super::overwrite::no_module<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking: self.0.blocking,
            })
        }
        #[inline]
        pub fn referrer_policy<V: crate::MaybeUpdateValue<str>>(
            self,
            referrer_policy: V,
        ) -> super::Building<super::overwrite::referrer_policy<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking: self.0.blocking,
            })
        }
        #[inline]
        pub fn src<V: crate::MaybeUpdateValue<str>>(
            self,
            src: V,
        ) -> super::Building<super::overwrite::src<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src,
                type_: self.0.type_,
                blocking: self.0.blocking,
            })
        }
        #[inline]
        pub fn type_<V: crate::MaybeUpdateValue<str>>(
            self,
            type_: V,
        ) -> super::Building<super::overwrite::type_<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_,
                blocking: self.0.blocking,
            })
        }
        #[inline]
        pub fn blocking<V: crate::MaybeUpdateValue<str>>(
            self,
            blocking: V,
        ) -> super::Building<super::overwrite::blocking<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                r#async: self.0.r#async,
                cross_origin: self.0.cross_origin,
                defer: self.0.defer,
                fetch_priority: self.0.fetch_priority,
                integrity: self.0.integrity,
                no_module: self.0.no_module,
                referrer_policy: self.0.referrer_policy,
                src: self.0.src,
                type_: self.0.type_,
                blocking,
            })
        }
    }
}
#[cfg(feature = "dom")]
mod impl_update_element {
    #[allow(unused_imports)]
    use super::super::*;
    impl<TypeDefs: ?::core::marker::Sized + super::Types>
        crate::props::UpdateElement<web_sys::HtmlScriptElement> for super::Data<TypeDefs>
    where
        HtmlElementProps::Data<TypeDefs::HtmlElementProps>:
            crate::props::UpdateElement<web_sys::HtmlElement>,
    {
        type State = super :: render_state :: RenderState < dyn super :: render_state :: RenderStateTypes < HtmlElementProps = < HtmlElementProps :: Data < TypeDefs :: HtmlElementProps , > as crate :: props :: UpdateElement < web_sys :: HtmlElement > > :: State , r#async = < TypeDefs :: r#async as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: State , cross_origin = < TypeDefs :: cross_origin as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , defer = < TypeDefs :: defer as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: State , fetch_priority = < TypeDefs :: fetch_priority as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , integrity = < TypeDefs :: integrity as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , no_module = < TypeDefs :: no_module as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: State , referrer_policy = < TypeDefs :: referrer_policy as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , src = < TypeDefs :: src as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , type_ = < TypeDefs :: type_ as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , blocking = < TypeDefs :: blocking as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , > , > ;
        fn update_element(
            this: Self,
            element: &web_sys::HtmlScriptElement,
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
                data: this.r#async,
                state: state.r#async,
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
                    const ATTR_NAME: &::core::primitive::str = "r#async";
                    < TypeDefs :: r#async as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (data , state , | v | element . set_async (* v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
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
                data: this.defer,
                state: state.defer,
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
                    const ATTR_NAME: &::core::primitive::str = "defer";
                    < TypeDefs :: defer as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (data , state , | v | element . set_defer (* v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.fetch_priority,
                state: state.fetch_priority,
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
                    const ATTR_NAME: &::core::primitive::str = "fetchpriority";
                    < TypeDefs :: fetch_priority as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.integrity,
                state: state.integrity,
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
                    const ATTR_NAME: &::core::primitive::str = "integrity";
                    < TypeDefs :: integrity as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | element . set_integrity (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.no_module,
                state: state.no_module,
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
                    const ATTR_NAME: &::core::primitive::str = "nomodule";
                    < TypeDefs :: no_module as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (data , state , | v | element . set_no_module (* v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
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
                    < TypeDefs :: referrer_policy as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
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
                data: this.type_,
                state: state.type_,
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
                    const ATTR_NAME: &::core::primitive::str = "type_";
                    < TypeDefs :: type_ as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | element . set_type (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.blocking,
                state: state.blocking,
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
                    const ATTR_NAME: &::core::primitive::str = "blocking";
                    < TypeDefs :: blocking as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
        }
    }
}
