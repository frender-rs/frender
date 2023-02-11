#[allow(non_snake_case)]
#[inline(always)]
pub const fn HtmlAnchorElementProps() -> Building<TypesInitial> {
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
        type HtmlElementProps: ?::core::marker::Sized
            + HtmlElementProps::Types
            + ~const ::core::marker::Destruct;
        type download: crate::MaybeUpdateValueByRef<str> + ~const ::core::marker::Destruct;
        type href: crate::MaybeUpdateValueByRef<str> + ~const ::core::marker::Destruct;
        type ping: crate::MaybeUpdateValueByRef<str> + ~const ::core::marker::Destruct;
        type referrer_policy: crate::MaybeUpdateValueByRef<str> + ~const ::core::marker::Destruct;
        type rel: crate::MaybeUpdateValueByRef<str> + ~const ::core::marker::Destruct;
        type target: crate::MaybeUpdateValueByRef<str> + ~const ::core::marker::Destruct;
        type href_lang: crate::MaybeUpdateValueByRef<str> + ~const ::core::marker::Destruct;
        type type_: crate::MaybeUpdateValueByRef<str> + ~const ::core::marker::Destruct;
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
pub use super::HtmlElementProps::render_state;
#[inline(always)]
pub const fn build<TypeDefs: ?::core::marker::Sized + Types>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    building.0
}
mod builder_and_replacer {
    #[allow(unused_imports)]
    use super::super::*;
    impl<TypeDefs: super::Types + ?::core::marker::Sized> super::Building<TypeDefs> {
        #[doc = "See [`HtmlElementProps::children`]"]
        #[inline(always)]
        pub const fn children<V>(
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
        #[inline(always)]
        pub const fn class<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn id<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn part<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn access_key<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn auto_capitalize<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn auto_focus<V: crate::MaybeUpdateValue<bool>>(
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
        #[inline(always)]
        pub const fn content_editable<V: crate::props::MaybeInherit<bool>>(
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
        #[inline(always)]
        pub const fn context_menu<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn dir<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn draggable<V: crate::MaybeUpdateValue<bool>>(
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
        #[inline(always)]
        pub const fn enter_key_hint<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn hidden<V: crate::MaybeUpdateValue<bool>>(
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
        #[inline(always)]
        pub const fn inert<V: crate::MaybeUpdateValue<bool>>(
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
        #[inline(always)]
        pub const fn input_mode<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn is<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn item_id<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn item_prop<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn item_ref<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn item_scope<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn item_type<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn lang<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn nonce<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn role<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn slot<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn spellcheck<V: crate::MaybeUpdateValue<bool>>(
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
        #[inline(always)]
        pub const fn style<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn tab_index<V: crate::MaybeUpdateValue<i32>>(
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
        #[inline(always)]
        pub const fn title<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn translate<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn virtual_keyboard_policy<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn on_click<V>(
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
        #[inline(always)]
        pub const fn download<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            download: V,
        ) -> super::Building<super::overwrite::download<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
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
        #[inline(always)]
        pub const fn href<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            href: V,
        ) -> super::Building<super::overwrite::href<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
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
        #[inline(always)]
        pub const fn ping<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            ping: V,
        ) -> super::Building<super::overwrite::ping<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
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
        #[inline(always)]
        pub const fn referrer_policy<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            referrer_policy: V,
        ) -> super::Building<super::overwrite::referrer_policy<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
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
        #[inline(always)]
        pub const fn rel<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            rel: V,
        ) -> super::Building<super::overwrite::rel<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
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
        #[inline(always)]
        pub const fn target<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            target: V,
        ) -> super::Building<super::overwrite::target<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
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
        #[inline(always)]
        pub const fn href_lang<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            href_lang: V,
        ) -> super::Building<super::overwrite::href_lang<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
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
        #[inline(always)]
        pub const fn type_<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            type_: V,
        ) -> super::Building<super::overwrite::type_<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
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
        type State =
            <HtmlElementProps::Data<TypeDefs::HtmlElementProps> as crate::props::UpdateElement<
                web_sys::HtmlElement,
            >>::State;
        fn initialize_state(
            this: Self,
            element: &web_sys::HtmlAnchorElement,
            children_ctx: &mut ::frender_dom::Dom,
        ) -> Self::State {
            let dom_element: &::web_sys::Element = element.as_ref();
            < TypeDefs :: download as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . download , | v | element . set_download (v) , | | dom_element . remove_attribute ("download") . unwrap () ,) ;
            < TypeDefs :: href as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . href , | v | element . set_href (v) , | | dom_element . remove_attribute ("href") . unwrap () ,) ;
            < TypeDefs :: ping as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . ping , | v | element . set_ping (v) , | | dom_element . remove_attribute ("ping") . unwrap () ,) ;
            < TypeDefs :: referrer_policy as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . referrer_policy , | v | element . set_referrer_policy (v) , | | dom_element . remove_attribute ("referrerpolicy") . unwrap () ,) ;
            < TypeDefs :: rel as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . rel , | v | element . set_rel (v) , | | dom_element . remove_attribute ("rel") . unwrap () ,) ;
            < TypeDefs :: target as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . target , | v | element . set_target (v) , | | dom_element . remove_attribute ("target") . unwrap () ,) ;
            < TypeDefs :: href_lang as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . href_lang , | v | element . set_hreflang (v) , | | dom_element . remove_attribute ("hreflang") . unwrap () ,) ;
            < TypeDefs :: type_ as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . type_ , | v | element . set_type (v) , | | dom_element . remove_attribute ("type") . unwrap () ,) ;
            <HtmlElementProps::Data<TypeDefs::HtmlElementProps> as crate::props::UpdateElement<
                web_sys::HtmlElement,
            >>::initialize_state(this.HtmlElementProps, element, children_ctx)
        }
        fn update_element(
            this: Self,
            element: &web_sys::HtmlAnchorElement,
            children_ctx: &mut ::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element: &::web_sys::Element = element.as_ref();
            crate::props::UpdateElement::update_element(
                this.HtmlElementProps,
                element.as_ref(),
                children_ctx,
                state,
            );
            < TypeDefs :: download as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . download , | v | element . set_download (v) , | | dom_element . remove_attribute ("download") . unwrap () ,) ;
            < TypeDefs :: href as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . href , | v | element . set_href (v) , | | dom_element . remove_attribute ("href") . unwrap () ,) ;
            < TypeDefs :: ping as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . ping , | v | element . set_ping (v) , | | dom_element . remove_attribute ("ping") . unwrap () ,) ;
            < TypeDefs :: referrer_policy as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . referrer_policy , | v | element . set_referrer_policy (v) , | | dom_element . remove_attribute ("referrerpolicy") . unwrap () ,) ;
            < TypeDefs :: rel as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . rel , | v | element . set_rel (v) , | | dom_element . remove_attribute ("rel") . unwrap () ,) ;
            < TypeDefs :: target as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . target , | v | element . set_target (v) , | | dom_element . remove_attribute ("target") . unwrap () ,) ;
            < TypeDefs :: href_lang as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . href_lang , | v | element . set_hreflang (v) , | | dom_element . remove_attribute ("hreflang") . unwrap () ,) ;
            < TypeDefs :: type_ as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . type_ , | v | element . set_type (v) , | | dom_element . remove_attribute ("type") . unwrap () ,) ;
        }
    }
}
