#[allow(non_snake_case)]
#[inline(always)]
pub const fn HtmlFormElementProps() -> Building<TypesInitial> {
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
        type HtmlElementProps: ?::core::marker::Sized
            + HtmlElementProps::Types
            + ~const ::core::marker::Destruct;
        type accept: crate::MaybeUpdateValueByRef<str> + ~const ::core::marker::Destruct;
        type accept_charset: crate::MaybeUpdateValueByRef<str> + ~const ::core::marker::Destruct;
        type auto_complete: crate::MaybeUpdateValueByRef<str> + ~const ::core::marker::Destruct;
        type name: crate::MaybeUpdateValueByRef<str> + ~const ::core::marker::Destruct;
        type rel: crate::MaybeUpdateValueByRef<str> + ~const ::core::marker::Destruct;
        type action: crate::MaybeUpdateValueByRef<str> + ~const ::core::marker::Destruct;
        type enc_type: crate::MaybeUpdateValueByRef<str> + ~const ::core::marker::Destruct;
        type method: crate::MaybeUpdateValueByRef<str> + ~const ::core::marker::Destruct;
        type no_validate: crate::MaybeUpdateValue<bool> + ~const ::core::marker::Destruct;
        type target: crate::MaybeUpdateValueByRef<str> + ~const ::core::marker::Destruct;
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
        #[inline(always)]
        pub const fn accept<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            accept: V,
        ) -> super::Building<super::overwrite::accept<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
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
        #[inline(always)]
        pub const fn accept_charset<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            accept_charset: V,
        ) -> super::Building<super::overwrite::accept_charset<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
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
        #[inline(always)]
        pub const fn auto_complete<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            auto_complete: V,
        ) -> super::Building<super::overwrite::auto_complete<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
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
        #[inline(always)]
        pub const fn name<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            name: V,
        ) -> super::Building<super::overwrite::name<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
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
        #[inline(always)]
        pub const fn action<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            action: V,
        ) -> super::Building<super::overwrite::action<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
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
        #[inline(always)]
        pub const fn enc_type<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            enc_type: V,
        ) -> super::Building<super::overwrite::enc_type<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
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
        #[inline(always)]
        pub const fn method<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            method: V,
        ) -> super::Building<super::overwrite::method<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
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
        #[inline(always)]
        pub const fn no_validate<V: crate::MaybeUpdateValue<bool>>(
            self,
            no_validate: V,
        ) -> super::Building<super::overwrite::no_validate<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
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
        type State =
            <HtmlElementProps::Data<TypeDefs::HtmlElementProps> as crate::props::UpdateElement<
                web_sys::HtmlElement,
            >>::State;
        fn initialize_state(
            this: Self,
            element: &web_sys::HtmlFormElement,
            children_ctx: &mut ::frender_dom::Dom,
        ) -> Self::State {
            let dom_element: &::web_sys::Element = element.as_ref();
            < TypeDefs :: accept as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . accept , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , "accept" ,) , | | dom_element . remove_attribute ("accept") . unwrap () ,) ;
            < TypeDefs :: accept_charset as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . accept_charset , | v | element . set_accept_charset (v) , | | dom_element . remove_attribute ("accept-charset") . unwrap () ,) ;
            < TypeDefs :: auto_complete as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . auto_complete , | v | element . set_autocomplete (v) , | | dom_element . remove_attribute ("autocomplete") . unwrap () ,) ;
            < TypeDefs :: name as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . name , | v | element . set_name (v) , | | dom_element . remove_attribute ("name") . unwrap () ,) ;
            < TypeDefs :: rel as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . rel , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , "rel" ,) , | | dom_element . remove_attribute ("rel") . unwrap () ,) ;
            < TypeDefs :: action as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . action , | v | element . set_action (v) , | | dom_element . remove_attribute ("action") . unwrap () ,) ;
            < TypeDefs :: enc_type as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . enc_type , | v | element . set_enctype (v) , | | dom_element . remove_attribute ("enctype") . unwrap () ,) ;
            < TypeDefs :: method as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . method , | v | element . set_method (v) , | | dom_element . remove_attribute ("method") . unwrap () ,) ;
            < TypeDefs :: no_validate as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (this . no_validate , | v | element . set_no_validate (v) , | | dom_element . remove_attribute ("novalidate") . unwrap () ,) ;
            < TypeDefs :: target as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . target , | v | element . set_target (v) , | | dom_element . remove_attribute ("target") . unwrap () ,) ;
            <HtmlElementProps::Data<TypeDefs::HtmlElementProps> as crate::props::UpdateElement<
                web_sys::HtmlElement,
            >>::initialize_state(this.HtmlElementProps, element, children_ctx)
        }
        fn update_element(
            this: Self,
            element: &web_sys::HtmlFormElement,
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
            < TypeDefs :: accept as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . accept , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , "accept" ,) , | | dom_element . remove_attribute ("accept") . unwrap () ,) ;
            < TypeDefs :: accept_charset as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . accept_charset , | v | element . set_accept_charset (v) , | | dom_element . remove_attribute ("accept-charset") . unwrap () ,) ;
            < TypeDefs :: auto_complete as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . auto_complete , | v | element . set_autocomplete (v) , | | dom_element . remove_attribute ("autocomplete") . unwrap () ,) ;
            < TypeDefs :: name as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . name , | v | element . set_name (v) , | | dom_element . remove_attribute ("name") . unwrap () ,) ;
            < TypeDefs :: rel as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . rel , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , "rel" ,) , | | dom_element . remove_attribute ("rel") . unwrap () ,) ;
            < TypeDefs :: action as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . action , | v | element . set_action (v) , | | dom_element . remove_attribute ("action") . unwrap () ,) ;
            < TypeDefs :: enc_type as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . enc_type , | v | element . set_enctype (v) , | | dom_element . remove_attribute ("enctype") . unwrap () ,) ;
            < TypeDefs :: method as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . method , | v | element . set_method (v) , | | dom_element . remove_attribute ("method") . unwrap () ,) ;
            < TypeDefs :: no_validate as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (this . no_validate , | v | element . set_no_validate (v) , | | dom_element . remove_attribute ("novalidate") . unwrap () ,) ;
            < TypeDefs :: target as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . target , | v | element . set_target (v) , | | dom_element . remove_attribute ("target") . unwrap () ,) ;
        }
    }
}
