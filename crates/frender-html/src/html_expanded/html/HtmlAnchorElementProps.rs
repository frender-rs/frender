#[allow(non_snake_case)]
pub fn HtmlAnchorElementProps() -> Building<TypesInitial> {
    #[allow(unused_imports)]
    use super::*;
    self::Building(self::Data {
        HtmlElementProps: HtmlElementProps::build(HtmlElementProps()),
        download: (),
        href: (),
        ping: (),
        referrer_policy: (),
        rel: (),
        target: (),
        href_lang: (),
        type_: (),
    })
}
pub mod prelude {}
pub mod overwrite {
    #![allow(non_camel_case_types)]
    pub type HtmlElementProps<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = Value,
        download = <TypeDefs as super::Types>::download,
        href = <TypeDefs as super::Types>::href,
        ping = <TypeDefs as super::Types>::ping,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        rel = <TypeDefs as super::Types>::rel,
        target = <TypeDefs as super::Types>::target,
        href_lang = <TypeDefs as super::Types>::href_lang,
        type_ = <TypeDefs as super::Types>::type_,
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
    pub type download<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        download = Value,
        href = <TypeDefs as super::Types>::href,
        ping = <TypeDefs as super::Types>::ping,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        rel = <TypeDefs as super::Types>::rel,
        target = <TypeDefs as super::Types>::target,
        href_lang = <TypeDefs as super::Types>::href_lang,
        type_ = <TypeDefs as super::Types>::type_,
    >;
    pub type href<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        download = <TypeDefs as super::Types>::download,
        href = Value,
        ping = <TypeDefs as super::Types>::ping,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        rel = <TypeDefs as super::Types>::rel,
        target = <TypeDefs as super::Types>::target,
        href_lang = <TypeDefs as super::Types>::href_lang,
        type_ = <TypeDefs as super::Types>::type_,
    >;
    pub type ping<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        download = <TypeDefs as super::Types>::download,
        href = <TypeDefs as super::Types>::href,
        ping = Value,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        rel = <TypeDefs as super::Types>::rel,
        target = <TypeDefs as super::Types>::target,
        href_lang = <TypeDefs as super::Types>::href_lang,
        type_ = <TypeDefs as super::Types>::type_,
    >;
    pub type referrer_policy<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        download = <TypeDefs as super::Types>::download,
        href = <TypeDefs as super::Types>::href,
        ping = <TypeDefs as super::Types>::ping,
        referrer_policy = Value,
        rel = <TypeDefs as super::Types>::rel,
        target = <TypeDefs as super::Types>::target,
        href_lang = <TypeDefs as super::Types>::href_lang,
        type_ = <TypeDefs as super::Types>::type_,
    >;
    pub type rel<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        download = <TypeDefs as super::Types>::download,
        href = <TypeDefs as super::Types>::href,
        ping = <TypeDefs as super::Types>::ping,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        rel = Value,
        target = <TypeDefs as super::Types>::target,
        href_lang = <TypeDefs as super::Types>::href_lang,
        type_ = <TypeDefs as super::Types>::type_,
    >;
    pub type target<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        download = <TypeDefs as super::Types>::download,
        href = <TypeDefs as super::Types>::href,
        ping = <TypeDefs as super::Types>::ping,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        rel = <TypeDefs as super::Types>::rel,
        target = Value,
        href_lang = <TypeDefs as super::Types>::href_lang,
        type_ = <TypeDefs as super::Types>::type_,
    >;
    pub type href_lang<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        download = <TypeDefs as super::Types>::download,
        href = <TypeDefs as super::Types>::href,
        ping = <TypeDefs as super::Types>::ping,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        rel = <TypeDefs as super::Types>::rel,
        target = <TypeDefs as super::Types>::target,
        href_lang = Value,
        type_ = <TypeDefs as super::Types>::type_,
    >;
    pub type type_<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        download = <TypeDefs as super::Types>::download,
        href = <TypeDefs as super::Types>::href,
        ping = <TypeDefs as super::Types>::ping,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        rel = <TypeDefs as super::Types>::rel,
        target = <TypeDefs as super::Types>::target,
        href_lang = <TypeDefs as super::Types>::href_lang,
        type_ = Value,
    >;
}
mod trait_types {
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub trait Types {
        type HtmlElementProps: ?::core::marker::Sized + HtmlElementProps::Types;
        type download: crate::MaybeUpdateValue<str>;
        type href: crate::MaybeUpdateValue<str>;
        type ping: crate::MaybeUpdateValue<str>;
        type referrer_policy: crate::MaybeUpdateValue<str>;
        type rel: crate::MaybeUpdateValue<str>;
        type target: crate::MaybeUpdateValue<str>;
        type href_lang: crate::MaybeUpdateValue<str>;
        type type_: crate::MaybeUpdateValue<str>;
    }
}
pub use trait_types::Types;
pub use trait_types::Types as ValidTypes;
pub mod data_struct {
    #[non_exhaustive]
    pub struct HtmlAnchorElementProps<TypeDefs: super::Types + ?::core::marker::Sized> {
        pub HtmlElementProps: super::super::HtmlElementProps::Data<TypeDefs::HtmlElementProps>,
        pub download: TypeDefs::download,
        pub href: TypeDefs::href,
        pub ping: TypeDefs::ping,
        pub referrer_policy: TypeDefs::referrer_policy,
        pub rel: TypeDefs::rel,
        pub target: TypeDefs::target,
        pub href_lang: TypeDefs::href_lang,
        pub type_: TypeDefs::type_,
    }
}
pub use data_struct::HtmlAnchorElementProps as Data;
pub struct Building<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
pub struct Replacing<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
mod types_initial {
    #[allow(unused_imports)]
    use super::super::*;
    pub type TypesInitial = dyn super::Types<
        HtmlElementProps = HtmlElementProps::TypesInitial,
        download = (),
        href = (),
        ping = (),
        referrer_policy = (),
        rel = (),
        target = (),
        href_lang = (),
        type_ = (),
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
        type download: ::core::default::Default;
        type href: ::core::default::Default;
        type ping: ::core::default::Default;
        type referrer_policy: ::core::default::Default;
        type rel: ::core::default::Default;
        type target: ::core::default::Default;
        type href_lang: ::core::default::Default;
        type type_: ::core::default::Default;
    }
    pub struct RenderState<TypeDefs: RenderStateTypes>
    where
        TypeDefs: ?::core::marker::Sized,
    {
        pub HtmlElementProps: TypeDefs::HtmlElementProps,
        pub download: TypeDefs::download,
        pub href: TypeDefs::href,
        pub ping: TypeDefs::ping,
        pub referrer_policy: TypeDefs::referrer_policy,
        pub rel: TypeDefs::rel,
        pub target: TypeDefs::target,
        pub href_lang: TypeDefs::href_lang,
        pub type_: TypeDefs::type_,
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
        pub download: &'__pin mut (TypeDefs::download),
        pub href: &'__pin mut (TypeDefs::href),
        pub ping: &'__pin mut (TypeDefs::ping),
        pub referrer_policy: &'__pin mut (TypeDefs::referrer_policy),
        pub rel: &'__pin mut (TypeDefs::rel),
        pub target: &'__pin mut (TypeDefs::target),
        pub href_lang: &'__pin mut (TypeDefs::href_lang),
        pub type_: &'__pin mut (TypeDefs::type_),
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
            pub download: &'__pin (TypeDefs::download),
            pub href: &'__pin (TypeDefs::href),
            pub ping: &'__pin (TypeDefs::ping),
            pub referrer_policy: &'__pin (TypeDefs::referrer_policy),
            pub rel: &'__pin (TypeDefs::rel),
            pub target: &'__pin (TypeDefs::target),
            pub href_lang: &'__pin (TypeDefs::href_lang),
            pub type_: &'__pin (TypeDefs::type_),
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
                        download,
                        href,
                        ping,
                        referrer_policy,
                        rel,
                        target,
                        href_lang,
                        type_,
                    } = self.get_unchecked_mut();
                    RenderStateProj {
                        HtmlElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            HtmlElementProps,
                        ),
                        download: download,
                        href: href,
                        ping: ping,
                        referrer_policy: referrer_policy,
                        rel: rel,
                        target: target,
                        href_lang: href_lang,
                        type_: type_,
                    }
                }
            }
            pub(crate) fn project_ref<'__pin>(
                self: ::pin_project_lite::__private::Pin<&'__pin Self>,
            ) -> ProjectionRef<'__pin, TypeDefs> {
                unsafe {
                    let Self {
                        HtmlElementProps,
                        download,
                        href,
                        ping,
                        referrer_policy,
                        rel,
                        target,
                        href_lang,
                        type_,
                    } = self.get_ref();
                    ProjectionRef {
                        HtmlElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            HtmlElementProps,
                        ),
                        download: download,
                        href: href,
                        ping: ping,
                        referrer_policy: referrer_policy,
                        rel: rel,
                        target: target,
                        href_lang: href_lang,
                        type_: type_,
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
            download: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::download>,
            href: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::href>,
            ping: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::ping>,
            referrer_policy: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::referrer_policy>,
            rel: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::rel>,
            target: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::target>,
            href_lang: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::href_lang>,
            type_: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::type_>,
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
            let _ = &this.download;
            let _ = &this.href;
            let _ = &this.ping;
            let _ = &this.referrer_policy;
            let _ = &this.rel;
            let _ = &this.target;
            let _ = &this.href_lang;
            let _ = &this.type_;
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
                download: ::core::default::Default::default(),
                href: ::core::default::Default::default(),
                ping: ::core::default::Default::default(),
                referrer_policy: ::core::default::Default::default(),
                rel: ::core::default::Default::default(),
                target: ::core::default::Default::default(),
                href_lang: ::core::default::Default::default(),
                type_: ::core::default::Default::default(),
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
                download: self.0.download,
                href: self.0.href,
                ping: self.0.ping,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                target: self.0.target,
                href_lang: self.0.href_lang,
                type_: self.0.type_,
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
                download: self.0.download,
                href: self.0.href,
                ping: self.0.ping,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                target: self.0.target,
                href_lang: self.0.href_lang,
                type_: self.0.type_,
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
                download: self.0.download,
                href: self.0.href,
                ping: self.0.ping,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                target: self.0.target,
                href_lang: self.0.href_lang,
                type_: self.0.type_,
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
                download: self.0.download,
                href: self.0.href,
                ping: self.0.ping,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                target: self.0.target,
                href_lang: self.0.href_lang,
                type_: self.0.type_,
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
                download: self.0.download,
                href: self.0.href,
                ping: self.0.ping,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                target: self.0.target,
                href_lang: self.0.href_lang,
                type_: self.0.type_,
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
                download: self.0.download,
                href: self.0.href,
                ping: self.0.ping,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                target: self.0.target,
                href_lang: self.0.href_lang,
                type_: self.0.type_,
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
                download: self.0.download,
                href: self.0.href,
                ping: self.0.ping,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                target: self.0.target,
                href_lang: self.0.href_lang,
                type_: self.0.type_,
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
                download: self.0.download,
                href: self.0.href,
                ping: self.0.ping,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                target: self.0.target,
                href_lang: self.0.href_lang,
                type_: self.0.type_,
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
                download: self.0.download,
                href: self.0.href,
                ping: self.0.ping,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                target: self.0.target,
                href_lang: self.0.href_lang,
                type_: self.0.type_,
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
                download: self.0.download,
                href: self.0.href,
                ping: self.0.ping,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                target: self.0.target,
                href_lang: self.0.href_lang,
                type_: self.0.type_,
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
                download: self.0.download,
                href: self.0.href,
                ping: self.0.ping,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                target: self.0.target,
                href_lang: self.0.href_lang,
                type_: self.0.type_,
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
                download: self.0.download,
                href: self.0.href,
                ping: self.0.ping,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                target: self.0.target,
                href_lang: self.0.href_lang,
                type_: self.0.type_,
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
                download: self.0.download,
                href: self.0.href,
                ping: self.0.ping,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                target: self.0.target,
                href_lang: self.0.href_lang,
                type_: self.0.type_,
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
                download: self.0.download,
                href: self.0.href,
                ping: self.0.ping,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                target: self.0.target,
                href_lang: self.0.href_lang,
                type_: self.0.type_,
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
                download: self.0.download,
                href: self.0.href,
                ping: self.0.ping,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                target: self.0.target,
                href_lang: self.0.href_lang,
                type_: self.0.type_,
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
                download: self.0.download,
                href: self.0.href,
                ping: self.0.ping,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                target: self.0.target,
                href_lang: self.0.href_lang,
                type_: self.0.type_,
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
                download: self.0.download,
                href: self.0.href,
                ping: self.0.ping,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                target: self.0.target,
                href_lang: self.0.href_lang,
                type_: self.0.type_,
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
                download: self.0.download,
                href: self.0.href,
                ping: self.0.ping,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                target: self.0.target,
                href_lang: self.0.href_lang,
                type_: self.0.type_,
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
                download: self.0.download,
                href: self.0.href,
                ping: self.0.ping,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                target: self.0.target,
                href_lang: self.0.href_lang,
                type_: self.0.type_,
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
                download: self.0.download,
                href: self.0.href,
                ping: self.0.ping,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                target: self.0.target,
                href_lang: self.0.href_lang,
                type_: self.0.type_,
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
                download: self.0.download,
                href: self.0.href,
                ping: self.0.ping,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                target: self.0.target,
                href_lang: self.0.href_lang,
                type_: self.0.type_,
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
                download: self.0.download,
                href: self.0.href,
                ping: self.0.ping,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                target: self.0.target,
                href_lang: self.0.href_lang,
                type_: self.0.type_,
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
                download: self.0.download,
                href: self.0.href,
                ping: self.0.ping,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                target: self.0.target,
                href_lang: self.0.href_lang,
                type_: self.0.type_,
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
                download: self.0.download,
                href: self.0.href,
                ping: self.0.ping,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                target: self.0.target,
                href_lang: self.0.href_lang,
                type_: self.0.type_,
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
                download: self.0.download,
                href: self.0.href,
                ping: self.0.ping,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                target: self.0.target,
                href_lang: self.0.href_lang,
                type_: self.0.type_,
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
                download: self.0.download,
                href: self.0.href,
                ping: self.0.ping,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                target: self.0.target,
                href_lang: self.0.href_lang,
                type_: self.0.type_,
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
                download: self.0.download,
                href: self.0.href,
                ping: self.0.ping,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                target: self.0.target,
                href_lang: self.0.href_lang,
                type_: self.0.type_,
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
                download: self.0.download,
                href: self.0.href,
                ping: self.0.ping,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                target: self.0.target,
                href_lang: self.0.href_lang,
                type_: self.0.type_,
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
                download: self.0.download,
                href: self.0.href,
                ping: self.0.ping,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                target: self.0.target,
                href_lang: self.0.href_lang,
                type_: self.0.type_,
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
                download: self.0.download,
                href: self.0.href,
                ping: self.0.ping,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                target: self.0.target,
                href_lang: self.0.href_lang,
                type_: self.0.type_,
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
                download: self.0.download,
                href: self.0.href,
                ping: self.0.ping,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                target: self.0.target,
                href_lang: self.0.href_lang,
                type_: self.0.type_,
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
                download: self.0.download,
                href: self.0.href,
                ping: self.0.ping,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                target: self.0.target,
                href_lang: self.0.href_lang,
                type_: self.0.type_,
            })
        }
        #[inline]
        pub fn download<V: crate::MaybeUpdateValue<str>>(
            self,
            download: V,
        ) -> super::Building<super::overwrite::download<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                download,
                href: self.0.href,
                ping: self.0.ping,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                target: self.0.target,
                href_lang: self.0.href_lang,
                type_: self.0.type_,
            })
        }
        #[inline]
        pub fn href<V: crate::MaybeUpdateValue<str>>(
            self,
            href: V,
        ) -> super::Building<super::overwrite::href<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                download: self.0.download,
                href,
                ping: self.0.ping,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                target: self.0.target,
                href_lang: self.0.href_lang,
                type_: self.0.type_,
            })
        }
        #[inline]
        pub fn ping<V: crate::MaybeUpdateValue<str>>(
            self,
            ping: V,
        ) -> super::Building<super::overwrite::ping<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                download: self.0.download,
                href: self.0.href,
                ping,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                target: self.0.target,
                href_lang: self.0.href_lang,
                type_: self.0.type_,
            })
        }
        #[inline]
        pub fn referrer_policy<V: crate::MaybeUpdateValue<str>>(
            self,
            referrer_policy: V,
        ) -> super::Building<super::overwrite::referrer_policy<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                download: self.0.download,
                href: self.0.href,
                ping: self.0.ping,
                referrer_policy,
                rel: self.0.rel,
                target: self.0.target,
                href_lang: self.0.href_lang,
                type_: self.0.type_,
            })
        }
        #[inline]
        pub fn rel<V: crate::MaybeUpdateValue<str>>(
            self,
            rel: V,
        ) -> super::Building<super::overwrite::rel<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                download: self.0.download,
                href: self.0.href,
                ping: self.0.ping,
                referrer_policy: self.0.referrer_policy,
                rel,
                target: self.0.target,
                href_lang: self.0.href_lang,
                type_: self.0.type_,
            })
        }
        #[inline]
        pub fn target<V: crate::MaybeUpdateValue<str>>(
            self,
            target: V,
        ) -> super::Building<super::overwrite::target<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                download: self.0.download,
                href: self.0.href,
                ping: self.0.ping,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                target,
                href_lang: self.0.href_lang,
                type_: self.0.type_,
            })
        }
        #[inline]
        pub fn href_lang<V: crate::MaybeUpdateValue<str>>(
            self,
            href_lang: V,
        ) -> super::Building<super::overwrite::href_lang<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                download: self.0.download,
                href: self.0.href,
                ping: self.0.ping,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                target: self.0.target,
                href_lang,
                type_: self.0.type_,
            })
        }
        #[inline]
        pub fn type_<V: crate::MaybeUpdateValue<str>>(
            self,
            type_: V,
        ) -> super::Building<super::overwrite::type_<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                download: self.0.download,
                href: self.0.href,
                ping: self.0.ping,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                target: self.0.target,
                href_lang: self.0.href_lang,
                type_,
            })
        }
    }
}
#[cfg(feature = "dom")]
mod impl_update_element {
    #[allow(unused_imports)]
    use super::super::*;
    impl<TypeDefs: ?::core::marker::Sized + super::Types>
        crate::props::UpdateElement<web_sys::HtmlAnchorElement> for super::Data<TypeDefs>
    where
        HtmlElementProps::Data<TypeDefs::HtmlElementProps>:
            crate::props::UpdateElement<web_sys::HtmlElement>,
    {
        type State = super :: render_state :: RenderState < dyn super :: render_state :: RenderStateTypes < HtmlElementProps = < HtmlElementProps :: Data < TypeDefs :: HtmlElementProps , > as crate :: props :: UpdateElement < web_sys :: HtmlElement > > :: State , download = < TypeDefs :: download as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , href = < TypeDefs :: href as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , ping = < TypeDefs :: ping as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , referrer_policy = < TypeDefs :: referrer_policy as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , rel = < TypeDefs :: rel as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , target = < TypeDefs :: target as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , href_lang = < TypeDefs :: href_lang as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , type_ = < TypeDefs :: type_ as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , > , > ;
        fn update_element(
            this: Self,
            element: &web_sys::HtmlAnchorElement,
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
                data: this.download,
                state: state.download,
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
                    const ATTR_NAME: &::core::primitive::str = "download";
                    < TypeDefs :: download as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | element . set_download (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.href,
                state: state.href,
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
                    const ATTR_NAME: &::core::primitive::str = "href";
                    < TypeDefs :: href as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | element . set_href (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.ping,
                state: state.ping,
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
                    const ATTR_NAME: &::core::primitive::str = "ping";
                    < TypeDefs :: ping as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | element . set_ping (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
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
                data: this.rel,
                state: state.rel,
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
                    const ATTR_NAME: &::core::primitive::str = "rel";
                    < TypeDefs :: rel as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | element . set_rel (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.target,
                state: state.target,
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
                    const ATTR_NAME: &::core::primitive::str = "target";
                    < TypeDefs :: target as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | element . set_target (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.href_lang,
                state: state.href_lang,
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
                    const ATTR_NAME: &::core::primitive::str = "hreflang";
                    < TypeDefs :: href_lang as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | element . set_hreflang (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
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
                    const ATTR_NAME: &::core::primitive::str = "type";
                    < TypeDefs :: type_ as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | element . set_type (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
        }
    }
}