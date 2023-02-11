#[allow(non_snake_case)]
pub fn HtmlFormElementProps() -> Building<TypesInitial> {
    #[allow(unused_imports)]
    use super::*;
    self::Building(self::Data {
        HtmlElementProps: HtmlElementProps::build(HtmlElementProps()),
        accept: (),
        accept_charset: (),
        auto_complete: (),
        name: (),
        rel: (),
        action: (),
        enc_type: (),
        method: (),
        no_validate: (),
        target: (),
    })
}
pub mod prelude {}
pub mod overwrite {
    #![allow(non_camel_case_types)]
    pub type HtmlElementProps<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = Value,
        accept = <TypeDefs as super::Types>::accept,
        accept_charset = <TypeDefs as super::Types>::accept_charset,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        name = <TypeDefs as super::Types>::name,
        rel = <TypeDefs as super::Types>::rel,
        action = <TypeDefs as super::Types>::action,
        enc_type = <TypeDefs as super::Types>::enc_type,
        method = <TypeDefs as super::Types>::method,
        no_validate = <TypeDefs as super::Types>::no_validate,
        target = <TypeDefs as super::Types>::target,
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
    pub type accept<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = Value,
        accept_charset = <TypeDefs as super::Types>::accept_charset,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        name = <TypeDefs as super::Types>::name,
        rel = <TypeDefs as super::Types>::rel,
        action = <TypeDefs as super::Types>::action,
        enc_type = <TypeDefs as super::Types>::enc_type,
        method = <TypeDefs as super::Types>::method,
        no_validate = <TypeDefs as super::Types>::no_validate,
        target = <TypeDefs as super::Types>::target,
    >;
    pub type accept_charset<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        accept_charset = Value,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        name = <TypeDefs as super::Types>::name,
        rel = <TypeDefs as super::Types>::rel,
        action = <TypeDefs as super::Types>::action,
        enc_type = <TypeDefs as super::Types>::enc_type,
        method = <TypeDefs as super::Types>::method,
        no_validate = <TypeDefs as super::Types>::no_validate,
        target = <TypeDefs as super::Types>::target,
    >;
    pub type auto_complete<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        accept_charset = <TypeDefs as super::Types>::accept_charset,
        auto_complete = Value,
        name = <TypeDefs as super::Types>::name,
        rel = <TypeDefs as super::Types>::rel,
        action = <TypeDefs as super::Types>::action,
        enc_type = <TypeDefs as super::Types>::enc_type,
        method = <TypeDefs as super::Types>::method,
        no_validate = <TypeDefs as super::Types>::no_validate,
        target = <TypeDefs as super::Types>::target,
    >;
    pub type name<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        accept_charset = <TypeDefs as super::Types>::accept_charset,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        name = Value,
        rel = <TypeDefs as super::Types>::rel,
        action = <TypeDefs as super::Types>::action,
        enc_type = <TypeDefs as super::Types>::enc_type,
        method = <TypeDefs as super::Types>::method,
        no_validate = <TypeDefs as super::Types>::no_validate,
        target = <TypeDefs as super::Types>::target,
    >;
    pub type rel<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        accept_charset = <TypeDefs as super::Types>::accept_charset,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        name = <TypeDefs as super::Types>::name,
        rel = Value,
        action = <TypeDefs as super::Types>::action,
        enc_type = <TypeDefs as super::Types>::enc_type,
        method = <TypeDefs as super::Types>::method,
        no_validate = <TypeDefs as super::Types>::no_validate,
        target = <TypeDefs as super::Types>::target,
    >;
    pub type action<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        accept_charset = <TypeDefs as super::Types>::accept_charset,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        name = <TypeDefs as super::Types>::name,
        rel = <TypeDefs as super::Types>::rel,
        action = Value,
        enc_type = <TypeDefs as super::Types>::enc_type,
        method = <TypeDefs as super::Types>::method,
        no_validate = <TypeDefs as super::Types>::no_validate,
        target = <TypeDefs as super::Types>::target,
    >;
    pub type enc_type<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        accept_charset = <TypeDefs as super::Types>::accept_charset,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        name = <TypeDefs as super::Types>::name,
        rel = <TypeDefs as super::Types>::rel,
        action = <TypeDefs as super::Types>::action,
        enc_type = Value,
        method = <TypeDefs as super::Types>::method,
        no_validate = <TypeDefs as super::Types>::no_validate,
        target = <TypeDefs as super::Types>::target,
    >;
    pub type method<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        accept_charset = <TypeDefs as super::Types>::accept_charset,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        name = <TypeDefs as super::Types>::name,
        rel = <TypeDefs as super::Types>::rel,
        action = <TypeDefs as super::Types>::action,
        enc_type = <TypeDefs as super::Types>::enc_type,
        method = Value,
        no_validate = <TypeDefs as super::Types>::no_validate,
        target = <TypeDefs as super::Types>::target,
    >;
    pub type no_validate<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        accept_charset = <TypeDefs as super::Types>::accept_charset,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        name = <TypeDefs as super::Types>::name,
        rel = <TypeDefs as super::Types>::rel,
        action = <TypeDefs as super::Types>::action,
        enc_type = <TypeDefs as super::Types>::enc_type,
        method = <TypeDefs as super::Types>::method,
        no_validate = Value,
        target = <TypeDefs as super::Types>::target,
    >;
    pub type target<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        accept_charset = <TypeDefs as super::Types>::accept_charset,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        name = <TypeDefs as super::Types>::name,
        rel = <TypeDefs as super::Types>::rel,
        action = <TypeDefs as super::Types>::action,
        enc_type = <TypeDefs as super::Types>::enc_type,
        method = <TypeDefs as super::Types>::method,
        no_validate = <TypeDefs as super::Types>::no_validate,
        target = Value,
    >;
}
mod trait_types {
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub trait Types {
        type HtmlElementProps: ?::core::marker::Sized + HtmlElementProps::Types;
        type accept: crate::MaybeUpdateValueWithState<str>;
        type accept_charset: crate::MaybeUpdateValueWithState<str>;
        type auto_complete: crate::MaybeUpdateValueWithState<str>;
        type name: crate::MaybeUpdateValueWithState<str>;
        type rel: crate::MaybeUpdateValueWithState<str>;
        type action: crate::MaybeUpdateValueWithState<str>;
        type enc_type: crate::MaybeUpdateValueWithState<str>;
        type method: crate::MaybeUpdateValueWithState<str>;
        type no_validate: crate::MaybeUpdateValueWithState<bool>;
        type target: crate::MaybeUpdateValueWithState<str>;
    }
}
pub use trait_types::Types;
pub use trait_types::Types as ValidTypes;
pub mod data_struct {
    #[non_exhaustive]
    pub struct HtmlFormElementProps<TypeDefs: super::Types + ?::core::marker::Sized> {
        pub HtmlElementProps: super::super::HtmlElementProps::Data<TypeDefs::HtmlElementProps>,
        pub accept: TypeDefs::accept,
        pub accept_charset: TypeDefs::accept_charset,
        pub auto_complete: TypeDefs::auto_complete,
        pub name: TypeDefs::name,
        pub rel: TypeDefs::rel,
        pub action: TypeDefs::action,
        pub enc_type: TypeDefs::enc_type,
        pub method: TypeDefs::method,
        pub no_validate: TypeDefs::no_validate,
        pub target: TypeDefs::target,
    }
}
pub use data_struct::HtmlFormElementProps as Data;
pub struct Building<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
pub struct Replacing<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
mod types_initial {
    #[allow(unused_imports)]
    use super::super::*;
    pub type TypesInitial = dyn super::Types<
        HtmlElementProps = HtmlElementProps::TypesInitial,
        accept = (),
        accept_charset = (),
        auto_complete = (),
        name = (),
        rel = (),
        action = (),
        enc_type = (),
        method = (),
        no_validate = (),
        target = (),
    >;
}
pub use types_initial::TypesInitial;
pub type DataInitial = Data<TypesInitial>;
#[cfg(feature = "dom")]
pub mod render_state {
    #[allow(non_camel_case_types)]
    pub trait RenderStateTypes {
        type HtmlElementProps: crate::props::IntrinsicComponentPollReactive;
        type accept;
        type accept_charset;
        type auto_complete;
        type name;
        type rel;
        type action;
        type enc_type;
        type method;
        type no_validate;
        type target;
    }
    pub struct RenderState<TypeDefs: RenderStateTypes>
    where
        TypeDefs: ?::core::marker::Sized,
    {
        pub HtmlElementProps: TypeDefs::HtmlElementProps,
        pub accept: TypeDefs::accept,
        pub accept_charset: TypeDefs::accept_charset,
        pub auto_complete: TypeDefs::auto_complete,
        pub name: TypeDefs::name,
        pub rel: TypeDefs::rel,
        pub action: TypeDefs::action,
        pub enc_type: TypeDefs::enc_type,
        pub method: TypeDefs::method,
        pub no_validate: TypeDefs::no_validate,
        pub target: TypeDefs::target,
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
        pub accept: &'__pin mut (TypeDefs::accept),
        pub accept_charset: &'__pin mut (TypeDefs::accept_charset),
        pub auto_complete: &'__pin mut (TypeDefs::auto_complete),
        pub name: &'__pin mut (TypeDefs::name),
        pub rel: &'__pin mut (TypeDefs::rel),
        pub action: &'__pin mut (TypeDefs::action),
        pub enc_type: &'__pin mut (TypeDefs::enc_type),
        pub method: &'__pin mut (TypeDefs::method),
        pub no_validate: &'__pin mut (TypeDefs::no_validate),
        pub target: &'__pin mut (TypeDefs::target),
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
            pub accept: &'__pin (TypeDefs::accept),
            pub accept_charset: &'__pin (TypeDefs::accept_charset),
            pub auto_complete: &'__pin (TypeDefs::auto_complete),
            pub name: &'__pin (TypeDefs::name),
            pub rel: &'__pin (TypeDefs::rel),
            pub action: &'__pin (TypeDefs::action),
            pub enc_type: &'__pin (TypeDefs::enc_type),
            pub method: &'__pin (TypeDefs::method),
            pub no_validate: &'__pin (TypeDefs::no_validate),
            pub target: &'__pin (TypeDefs::target),
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
                        accept,
                        accept_charset,
                        auto_complete,
                        name,
                        rel,
                        action,
                        enc_type,
                        method,
                        no_validate,
                        target,
                    } = self.get_unchecked_mut();
                    RenderStateProj {
                        HtmlElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            HtmlElementProps,
                        ),
                        accept: accept,
                        accept_charset: accept_charset,
                        auto_complete: auto_complete,
                        name: name,
                        rel: rel,
                        action: action,
                        enc_type: enc_type,
                        method: method,
                        no_validate: no_validate,
                        target: target,
                    }
                }
            }
            pub(crate) fn project_ref<'__pin>(
                self: ::pin_project_lite::__private::Pin<&'__pin Self>,
            ) -> ProjectionRef<'__pin, TypeDefs> {
                unsafe {
                    let Self {
                        HtmlElementProps,
                        accept,
                        accept_charset,
                        auto_complete,
                        name,
                        rel,
                        action,
                        enc_type,
                        method,
                        no_validate,
                        target,
                    } = self.get_ref();
                    ProjectionRef {
                        HtmlElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            HtmlElementProps,
                        ),
                        accept: accept,
                        accept_charset: accept_charset,
                        auto_complete: auto_complete,
                        name: name,
                        rel: rel,
                        action: action,
                        enc_type: enc_type,
                        method: method,
                        no_validate: no_validate,
                        target: target,
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
            accept: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::accept>,
            accept_charset: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::accept_charset>,
            auto_complete: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::auto_complete>,
            name: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::name>,
            rel: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::rel>,
            action: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::action>,
            enc_type: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::enc_type>,
            method: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::method>,
            no_validate: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::no_validate>,
            target: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::target>,
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
            let _ = &this.accept;
            let _ = &this.accept_charset;
            let _ = &this.auto_complete;
            let _ = &this.name;
            let _ = &this.rel;
            let _ = &this.action;
            let _ = &this.enc_type;
            let _ = &this.method;
            let _ = &this.no_validate;
            let _ = &this.target;
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
        ///See [`HtmlElementProps::children`]
        #[inline]
        pub fn children<V>(
            self,
            children: V,
        ) -> super::Building<super::overwrite::children<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).children(children),
                ),
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        ///See [`HtmlElementProps::class`]
        #[inline]
        pub fn class<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            class: V,
        ) -> super::Building<super::overwrite::class<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).class(class),
                ),
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        ///See [`HtmlElementProps::id`]
        #[inline]
        pub fn id<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            id: V,
        ) -> super::Building<super::overwrite::id<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).id(id),
                ),
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        ///See [`HtmlElementProps::part`]
        #[inline]
        pub fn part<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            part: V,
        ) -> super::Building<super::overwrite::part<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).part(part),
                ),
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        ///See [`HtmlElementProps::access_key`]
        #[inline]
        pub fn access_key<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            access_key: V,
        ) -> super::Building<super::overwrite::access_key<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).access_key(access_key),
                ),
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        ///See [`HtmlElementProps::auto_capitalize`]
        #[inline]
        pub fn auto_capitalize<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            auto_capitalize: V,
        ) -> super::Building<super::overwrite::auto_capitalize<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps)
                        .auto_capitalize(auto_capitalize),
                ),
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        ///See [`HtmlElementProps::auto_focus`]
        #[inline]
        pub fn auto_focus<V: crate::MaybeUpdateValueWithState<bool>>(
            self,
            auto_focus: V,
        ) -> super::Building<super::overwrite::auto_focus<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).auto_focus(auto_focus),
                ),
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        ///See [`HtmlElementProps::content_editable`]
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
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        ///See [`HtmlElementProps::context_menu`]
        #[inline]
        pub fn context_menu<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            context_menu: V,
        ) -> super::Building<super::overwrite::context_menu<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).context_menu(context_menu),
                ),
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        ///See [`HtmlElementProps::dir`]
        #[inline]
        pub fn dir<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            dir: V,
        ) -> super::Building<super::overwrite::dir<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).dir(dir),
                ),
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        ///See [`HtmlElementProps::draggable`]
        #[inline]
        pub fn draggable<V: crate::MaybeUpdateValueWithState<bool>>(
            self,
            draggable: V,
        ) -> super::Building<super::overwrite::draggable<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).draggable(draggable),
                ),
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        ///See [`HtmlElementProps::enter_key_hint`]
        #[inline]
        pub fn enter_key_hint<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            enter_key_hint: V,
        ) -> super::Building<super::overwrite::enter_key_hint<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps)
                        .enter_key_hint(enter_key_hint),
                ),
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        ///See [`HtmlElementProps::hidden`]
        #[inline]
        pub fn hidden<V: crate::MaybeUpdateValueWithState<bool>>(
            self,
            hidden: V,
        ) -> super::Building<super::overwrite::hidden<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).hidden(hidden),
                ),
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        ///See [`HtmlElementProps::inert`]
        #[inline]
        pub fn inert<V: crate::MaybeUpdateValueWithState<bool>>(
            self,
            inert: V,
        ) -> super::Building<super::overwrite::inert<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).inert(inert),
                ),
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        ///See [`HtmlElementProps::input_mode`]
        #[inline]
        pub fn input_mode<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            input_mode: V,
        ) -> super::Building<super::overwrite::input_mode<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).input_mode(input_mode),
                ),
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        ///See [`HtmlElementProps::is`]
        #[inline]
        pub fn is<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            is: V,
        ) -> super::Building<super::overwrite::is<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).is(is),
                ),
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        ///See [`HtmlElementProps::item_id`]
        #[inline]
        pub fn item_id<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            item_id: V,
        ) -> super::Building<super::overwrite::item_id<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).item_id(item_id),
                ),
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        ///See [`HtmlElementProps::item_prop`]
        #[inline]
        pub fn item_prop<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            item_prop: V,
        ) -> super::Building<super::overwrite::item_prop<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).item_prop(item_prop),
                ),
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        ///See [`HtmlElementProps::item_ref`]
        #[inline]
        pub fn item_ref<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            item_ref: V,
        ) -> super::Building<super::overwrite::item_ref<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).item_ref(item_ref),
                ),
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        ///See [`HtmlElementProps::item_scope`]
        #[inline]
        pub fn item_scope<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            item_scope: V,
        ) -> super::Building<super::overwrite::item_scope<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).item_scope(item_scope),
                ),
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        ///See [`HtmlElementProps::item_type`]
        #[inline]
        pub fn item_type<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            item_type: V,
        ) -> super::Building<super::overwrite::item_type<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).item_type(item_type),
                ),
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        ///See [`HtmlElementProps::lang`]
        #[inline]
        pub fn lang<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            lang: V,
        ) -> super::Building<super::overwrite::lang<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).lang(lang),
                ),
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        ///See [`HtmlElementProps::nonce`]
        #[inline]
        pub fn nonce<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            nonce: V,
        ) -> super::Building<super::overwrite::nonce<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).nonce(nonce),
                ),
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        ///See [`HtmlElementProps::role`]
        #[inline]
        pub fn role<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            role: V,
        ) -> super::Building<super::overwrite::role<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).role(role),
                ),
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        ///See [`HtmlElementProps::slot`]
        #[inline]
        pub fn slot<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            slot: V,
        ) -> super::Building<super::overwrite::slot<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).slot(slot),
                ),
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        ///See [`HtmlElementProps::spellcheck`]
        #[inline]
        pub fn spellcheck<V: crate::MaybeUpdateValueWithState<bool>>(
            self,
            spellcheck: V,
        ) -> super::Building<super::overwrite::spellcheck<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).spellcheck(spellcheck),
                ),
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        ///See [`HtmlElementProps::style`]
        #[inline]
        pub fn style<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            style: V,
        ) -> super::Building<super::overwrite::style<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).style(style),
                ),
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        ///See [`HtmlElementProps::tab_index`]
        #[inline]
        pub fn tab_index<V: crate::MaybeUpdateValueWithState<i32>>(
            self,
            tab_index: V,
        ) -> super::Building<super::overwrite::tab_index<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).tab_index(tab_index),
                ),
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        ///See [`HtmlElementProps::title`]
        #[inline]
        pub fn title<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            title: V,
        ) -> super::Building<super::overwrite::title<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).title(title),
                ),
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        ///See [`HtmlElementProps::translate`]
        #[inline]
        pub fn translate<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            translate: V,
        ) -> super::Building<super::overwrite::translate<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).translate(translate),
                ),
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        ///See [`HtmlElementProps::virtual_keyboard_policy`]
        #[inline]
        pub fn virtual_keyboard_policy<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            virtual_keyboard_policy: V,
        ) -> super::Building<super::overwrite::virtual_keyboard_policy<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps)
                        .virtual_keyboard_policy(virtual_keyboard_policy),
                ),
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        ///See [`HtmlElementProps::on_click`]
        #[inline]
        pub fn on_click<V>(
            self,
            on_click: V,
        ) -> super::Building<super::overwrite::on_click<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).on_click(on_click),
                ),
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        #[deprecated = "This attribute has been deprecated and should not be used. Instead, use the accept attribute on <input type=file> elements."]
        #[inline]
        pub fn accept<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            accept: V,
        ) -> super::Building<super::overwrite::accept<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        #[inline]
        pub fn accept_charset<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            accept_charset: V,
        ) -> super::Building<super::overwrite::accept_charset<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept: self.0.accept,
                accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        #[inline]
        pub fn auto_complete<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            auto_complete: V,
        ) -> super::Building<super::overwrite::auto_complete<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        #[inline]
        pub fn name<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            name: V,
        ) -> super::Building<super::overwrite::name<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        #[inline]
        pub fn rel<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            rel: V,
        ) -> super::Building<super::overwrite::rel<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        #[inline]
        pub fn action<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            action: V,
        ) -> super::Building<super::overwrite::action<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        #[inline]
        pub fn enc_type<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            enc_type: V,
        ) -> super::Building<super::overwrite::enc_type<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        #[inline]
        pub fn method<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            method: V,
        ) -> super::Building<super::overwrite::method<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method,
                no_validate: self.0.no_validate,
                target: self.0.target,
            })
        }
        #[inline]
        pub fn no_validate<V: crate::MaybeUpdateValueWithState<bool>>(
            self,
            no_validate: V,
        ) -> super::Building<super::overwrite::no_validate<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate,
                target: self.0.target,
            })
        }
        #[inline]
        pub fn target<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            target: V,
        ) -> super::Building<super::overwrite::target<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept: self.0.accept,
                accept_charset: self.0.accept_charset,
                auto_complete: self.0.auto_complete,
                name: self.0.name,
                rel: self.0.rel,
                action: self.0.action,
                enc_type: self.0.enc_type,
                method: self.0.method,
                no_validate: self.0.no_validate,
                target,
            })
        }
    }
}
#[cfg(feature = "dom")]
mod impl_update_element {
    #[allow(unused_imports)]
    use super::super::*;
    impl<TypeDefs: ?::core::marker::Sized + super::Types>
        crate::props::UpdateElement<web_sys::HtmlFormElement> for super::Data<TypeDefs>
    where
        HtmlElementProps::Data<TypeDefs::HtmlElementProps>:
            crate::props::UpdateElement<web_sys::HtmlElement>,
    {
        type State = super::render_state::RenderState<
            dyn super::render_state::RenderStateTypes<
                HtmlElementProps = <HtmlElementProps::Data<
                    TypeDefs::HtmlElementProps,
                > as crate::props::UpdateElement<web_sys::HtmlElement>>::State,
                accept = <TypeDefs::accept as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                accept_charset = <TypeDefs::accept_charset as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                auto_complete = <TypeDefs::auto_complete as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                name = <TypeDefs::name as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                rel = <TypeDefs::rel as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                action = <TypeDefs::action as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                enc_type = <TypeDefs::enc_type as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                method = <TypeDefs::method as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                no_validate = <TypeDefs::no_validate as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::State,
                target = <TypeDefs::target as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
            >,
        >;
        fn initialize_state(
            this: Self,
            element: &web_sys::HtmlFormElement,
            children_ctx: &mut ::frender_dom::Dom,
        ) -> Self::State {
            let dom_element: &::web_sys::Element = element.as_ref();
            super::render_state::RenderState {
                HtmlElementProps: <HtmlElementProps::Data<
                    TypeDefs::HtmlElementProps,
                > as crate::props::UpdateElement<
                    web_sys::HtmlElement,
                >>::initialize_state(this.HtmlElementProps, element, children_ctx),
                accept: <TypeDefs::accept as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.accept,
                    |v| crate::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "accept",
                    ),
                    || dom_element.remove_attribute("accept").unwrap(),
                ),
                accept_charset: <TypeDefs::accept_charset as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.accept_charset,
                    |v| element.set_accept_charset(v),
                    || dom_element.remove_attribute("accept-charset").unwrap(),
                ),
                auto_complete: <TypeDefs::auto_complete as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.auto_complete,
                    |v| element.set_autocomplete(v),
                    || dom_element.remove_attribute("autocomplete").unwrap(),
                ),
                name: <TypeDefs::name as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.name,
                    |v| element.set_name(v),
                    || dom_element.remove_attribute("name").unwrap(),
                ),
                rel: <TypeDefs::rel as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.rel,
                    |v| crate::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "rel",
                    ),
                    || dom_element.remove_attribute("rel").unwrap(),
                ),
                action: <TypeDefs::action as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.action,
                    |v| element.set_action(v),
                    || dom_element.remove_attribute("action").unwrap(),
                ),
                enc_type: <TypeDefs::enc_type as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.enc_type,
                    |v| element.set_enctype(v),
                    || dom_element.remove_attribute("enctype").unwrap(),
                ),
                method: <TypeDefs::method as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.method,
                    |v| element.set_method(v),
                    || dom_element.remove_attribute("method").unwrap(),
                ),
                no_validate: <TypeDefs::no_validate as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.no_validate,
                    |v| element.set_no_validate(*v),
                    || dom_element.remove_attribute("novalidate").unwrap(),
                ),
                target: <TypeDefs::target as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.target,
                    |v| element.set_target(v),
                    || dom_element.remove_attribute("target").unwrap(),
                ),
            }
        }
        fn update_element(
            this: Self,
            element: &web_sys::HtmlFormElement,
            children_ctx: &mut ::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let state = state.pin_project();
            let dom_element: &::web_sys::Element = element.as_ref();
            crate::props::UpdateElement::update_element(
                this.HtmlElementProps,
                element.as_ref(),
                children_ctx,
                state.HtmlElementProps,
            );
            <TypeDefs::accept as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.accept,
                state.accept,
                |v| crate::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "accept",
                ),
                || dom_element.remove_attribute("accept").unwrap(),
            );
            <TypeDefs::accept_charset as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.accept_charset,
                state.accept_charset,
                |v| element.set_accept_charset(v),
                || dom_element.remove_attribute("accept-charset").unwrap(),
            );
            <TypeDefs::auto_complete as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.auto_complete,
                state.auto_complete,
                |v| element.set_autocomplete(v),
                || dom_element.remove_attribute("autocomplete").unwrap(),
            );
            <TypeDefs::name as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.name,
                state.name,
                |v| element.set_name(v),
                || dom_element.remove_attribute("name").unwrap(),
            );
            <TypeDefs::rel as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.rel,
                state.rel,
                |v| crate::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "rel",
                ),
                || dom_element.remove_attribute("rel").unwrap(),
            );
            <TypeDefs::action as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.action,
                state.action,
                |v| element.set_action(v),
                || dom_element.remove_attribute("action").unwrap(),
            );
            <TypeDefs::enc_type as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.enc_type,
                state.enc_type,
                |v| element.set_enctype(v),
                || dom_element.remove_attribute("enctype").unwrap(),
            );
            <TypeDefs::method as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.method,
                state.method,
                |v| element.set_method(v),
                || dom_element.remove_attribute("method").unwrap(),
            );
            <TypeDefs::no_validate as ::frender_dom::props::MaybeUpdateValueWithState<
                bool,
            >>::maybe_update_value_with_state(
                this.no_validate,
                state.no_validate,
                |v| element.set_no_validate(*v),
                || dom_element.remove_attribute("novalidate").unwrap(),
            );
            <TypeDefs::target as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.target,
                state.target,
                |v| element.set_target(v),
                || dom_element.remove_attribute("target").unwrap(),
            );
        }
    }
}
