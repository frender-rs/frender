#[allow(non_snake_case)]
#[inline(always)]
pub const fn HtmlScriptElementProps() -> Building<TypesInitial> {
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
        type HtmlElementProps: ?::core::marker::Sized
            + HtmlElementProps::Types
            + ~const ::core::marker::Destruct;
        type r#async: crate::MaybeUpdateValue<bool> + ~const ::core::marker::Destruct;
        type cross_origin: crate::MaybeUpdateValueByRef<str> + ~const ::core::marker::Destruct;
        type defer: crate::MaybeUpdateValue<bool> + ~const ::core::marker::Destruct;
        type fetch_priority: crate::MaybeUpdateValueByRef<str> + ~const ::core::marker::Destruct;
        type integrity: crate::MaybeUpdateValueByRef<str> + ~const ::core::marker::Destruct;
        type no_module: crate::MaybeUpdateValue<bool> + ~const ::core::marker::Destruct;
        type referrer_policy: crate::MaybeUpdateValueByRef<str> + ~const ::core::marker::Destruct;
        type src: crate::MaybeUpdateValueByRef<str> + ~const ::core::marker::Destruct;
        type type_: crate::MaybeUpdateValueByRef<str> + ~const ::core::marker::Destruct;
        type blocking: crate::MaybeUpdateValueByRef<str> + ~const ::core::marker::Destruct;
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
        #[inline(always)]
        pub const fn class<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn id<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn part<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn access_key<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn auto_focus<V: crate::MaybeUpdateValue<bool>>(
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
        #[inline(always)]
        pub const fn context_menu<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn dir<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn draggable<V: crate::MaybeUpdateValue<bool>>(
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
        #[inline(always)]
        pub const fn hidden<V: crate::MaybeUpdateValue<bool>>(
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
        #[inline(always)]
        pub const fn inert<V: crate::MaybeUpdateValue<bool>>(
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
        #[inline(always)]
        pub const fn input_mode<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn is<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn item_id<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn item_prop<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn item_ref<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn item_scope<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn item_type<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn lang<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn nonce<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn role<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn slot<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn spellcheck<V: crate::MaybeUpdateValue<bool>>(
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
        #[inline(always)]
        pub const fn style<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn tab_index<V: crate::MaybeUpdateValue<i32>>(
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
        #[inline(always)]
        pub const fn title<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn translate<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline(always)]
        pub const fn on_click<V>(
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
        #[inline(always)]
        pub const fn r#async<V: crate::MaybeUpdateValue<bool>>(
            self,
            r#async: V,
        ) -> super::Building<super::overwrite::r#async<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
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
        #[inline(always)]
        pub const fn cross_origin<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            cross_origin: V,
        ) -> super::Building<super::overwrite::cross_origin<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
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
        #[inline(always)]
        pub const fn defer<V: crate::MaybeUpdateValue<bool>>(
            self,
            defer: V,
        ) -> super::Building<super::overwrite::defer<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
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
        #[inline(always)]
        pub const fn fetch_priority<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            fetch_priority: V,
        ) -> super::Building<super::overwrite::fetch_priority<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
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
        #[inline(always)]
        pub const fn integrity<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            integrity: V,
        ) -> super::Building<super::overwrite::integrity<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
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
        #[inline(always)]
        pub const fn no_module<V: crate::MaybeUpdateValue<bool>>(
            self,
            no_module: V,
        ) -> super::Building<super::overwrite::no_module<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
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
        #[inline(always)]
        pub const fn src<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            src: V,
        ) -> super::Building<super::overwrite::src<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
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
        #[inline(always)]
        pub const fn blocking<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            blocking: V,
        ) -> super::Building<super::overwrite::blocking<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
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
        type State =
            <HtmlElementProps::Data<TypeDefs::HtmlElementProps> as crate::props::UpdateElement<
                web_sys::HtmlElement,
            >>::State;
        fn initialize_state(
            this: Self,
            element: &web_sys::HtmlScriptElement,
            children_ctx: &mut ::frender_dom::Dom,
        ) -> Self::State {
            let dom_element: &::web_sys::Element = element.as_ref();
            <TypeDefs::r#async as ::frender_dom::props::MaybeUpdateValue<bool>>::maybe_update_value(
                this.r#async,
                |v| element.set_async(v),
                || dom_element.remove_attribute("r#async").unwrap(),
            );
            < TypeDefs :: cross_origin as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . cross_origin , match element { el => | v : & _ | el . set_cross_origin (Some (v)) , } , match element { el => | | el . set_cross_origin (None) , } ,) ;
            <TypeDefs::defer as ::frender_dom::props::MaybeUpdateValue<bool>>::maybe_update_value(
                this.defer,
                |v| element.set_defer(v),
                || dom_element.remove_attribute("defer").unwrap(),
            );
            < TypeDefs :: fetch_priority as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . fetch_priority , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , "fetchpriority" ,) , | | dom_element . remove_attribute ("fetchpriority") . unwrap () ,) ;
            < TypeDefs :: integrity as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . integrity , | v | element . set_integrity (v) , | | dom_element . remove_attribute ("integrity") . unwrap () ,) ;
            < TypeDefs :: no_module as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (this . no_module , | v | element . set_no_module (v) , | | dom_element . remove_attribute ("nomodule") . unwrap () ,) ;
            < TypeDefs :: referrer_policy as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . referrer_policy , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , "referrerpolicy" ,) , | | dom_element . remove_attribute ("referrerpolicy") . unwrap () ,) ;
            < TypeDefs :: src as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . src , | v | element . set_src (v) , | | dom_element . remove_attribute ("src") . unwrap () ,) ;
            < TypeDefs :: type_ as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . type_ , | v | element . set_type (v) , | | dom_element . remove_attribute ("type_") . unwrap () ,) ;
            < TypeDefs :: blocking as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . blocking , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , "blocking" ,) , | | dom_element . remove_attribute ("blocking") . unwrap () ,) ;
            <HtmlElementProps::Data<TypeDefs::HtmlElementProps> as crate::props::UpdateElement<
                web_sys::HtmlElement,
            >>::initialize_state(this.HtmlElementProps, element, children_ctx)
        }
        fn update_element(
            this: Self,
            element: &web_sys::HtmlScriptElement,
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
            <TypeDefs::r#async as ::frender_dom::props::MaybeUpdateValue<bool>>::maybe_update_value(
                this.r#async,
                |v| element.set_async(v),
                || dom_element.remove_attribute("r#async").unwrap(),
            );
            < TypeDefs :: cross_origin as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . cross_origin , match element { el => | v : & _ | el . set_cross_origin (Some (v)) , } , match element { el => | | el . set_cross_origin (None) , } ,) ;
            <TypeDefs::defer as ::frender_dom::props::MaybeUpdateValue<bool>>::maybe_update_value(
                this.defer,
                |v| element.set_defer(v),
                || dom_element.remove_attribute("defer").unwrap(),
            );
            < TypeDefs :: fetch_priority as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . fetch_priority , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , "fetchpriority" ,) , | | dom_element . remove_attribute ("fetchpriority") . unwrap () ,) ;
            < TypeDefs :: integrity as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . integrity , | v | element . set_integrity (v) , | | dom_element . remove_attribute ("integrity") . unwrap () ,) ;
            < TypeDefs :: no_module as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (this . no_module , | v | element . set_no_module (v) , | | dom_element . remove_attribute ("nomodule") . unwrap () ,) ;
            < TypeDefs :: referrer_policy as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . referrer_policy , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , "referrerpolicy" ,) , | | dom_element . remove_attribute ("referrerpolicy") . unwrap () ,) ;
            < TypeDefs :: src as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . src , | v | element . set_src (v) , | | dom_element . remove_attribute ("src") . unwrap () ,) ;
            < TypeDefs :: type_ as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . type_ , | v | element . set_type (v) , | | dom_element . remove_attribute ("type_") . unwrap () ,) ;
            < TypeDefs :: blocking as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . blocking , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , "blocking" ,) , | | dom_element . remove_attribute ("blocking") . unwrap () ,) ;
        }
    }
}
