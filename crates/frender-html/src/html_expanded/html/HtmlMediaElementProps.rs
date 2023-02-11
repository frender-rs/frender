#[allow(non_snake_case)]
#[inline(always)]
pub const fn HtmlMediaElementProps() -> Building<TypesInitial> {
    #[allow(unused_imports)]
    use super::*;
    self::Building(self::Data {
        HtmlElementProps: HtmlElementProps::build(HtmlElementProps()),
        auto_play: (),
        controls: (),
        cross_origin: (),
        loop_: (),
        muted: (),
        preload: (),
        src: (),
    })
}
pub mod prelude {}
pub mod overwrite {
    #![allow(non_camel_case_types)]
    pub type HtmlElementProps<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = Value,
        auto_play = <TypeDefs as super::Types>::auto_play,
        controls = <TypeDefs as super::Types>::controls,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        loop_ = <TypeDefs as super::Types>::loop_,
        muted = <TypeDefs as super::Types>::muted,
        preload = <TypeDefs as super::Types>::preload,
        src = <TypeDefs as super::Types>::src,
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
    pub type auto_play<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_play = Value,
        controls = <TypeDefs as super::Types>::controls,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        loop_ = <TypeDefs as super::Types>::loop_,
        muted = <TypeDefs as super::Types>::muted,
        preload = <TypeDefs as super::Types>::preload,
        src = <TypeDefs as super::Types>::src,
    >;
    pub type controls<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_play = <TypeDefs as super::Types>::auto_play,
        controls = Value,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        loop_ = <TypeDefs as super::Types>::loop_,
        muted = <TypeDefs as super::Types>::muted,
        preload = <TypeDefs as super::Types>::preload,
        src = <TypeDefs as super::Types>::src,
    >;
    pub type cross_origin<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_play = <TypeDefs as super::Types>::auto_play,
        controls = <TypeDefs as super::Types>::controls,
        cross_origin = Value,
        loop_ = <TypeDefs as super::Types>::loop_,
        muted = <TypeDefs as super::Types>::muted,
        preload = <TypeDefs as super::Types>::preload,
        src = <TypeDefs as super::Types>::src,
    >;
    pub type loop_<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_play = <TypeDefs as super::Types>::auto_play,
        controls = <TypeDefs as super::Types>::controls,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        loop_ = Value,
        muted = <TypeDefs as super::Types>::muted,
        preload = <TypeDefs as super::Types>::preload,
        src = <TypeDefs as super::Types>::src,
    >;
    pub type muted<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_play = <TypeDefs as super::Types>::auto_play,
        controls = <TypeDefs as super::Types>::controls,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        loop_ = <TypeDefs as super::Types>::loop_,
        muted = Value,
        preload = <TypeDefs as super::Types>::preload,
        src = <TypeDefs as super::Types>::src,
    >;
    pub type preload<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_play = <TypeDefs as super::Types>::auto_play,
        controls = <TypeDefs as super::Types>::controls,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        loop_ = <TypeDefs as super::Types>::loop_,
        muted = <TypeDefs as super::Types>::muted,
        preload = Value,
        src = <TypeDefs as super::Types>::src,
    >;
    pub type src<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_play = <TypeDefs as super::Types>::auto_play,
        controls = <TypeDefs as super::Types>::controls,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        loop_ = <TypeDefs as super::Types>::loop_,
        muted = <TypeDefs as super::Types>::muted,
        preload = <TypeDefs as super::Types>::preload,
        src = Value,
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
        type auto_play: crate::MaybeUpdateValue<bool> + ~const ::core::marker::Destruct;
        type controls: crate::MaybeUpdateValue<bool> + ~const ::core::marker::Destruct;
        type cross_origin: crate::MaybeUpdateValueByRef<str> + ~const ::core::marker::Destruct;
        type loop_: crate::MaybeUpdateValue<bool> + ~const ::core::marker::Destruct;
        type muted: crate::MaybeUpdateValue<bool> + ~const ::core::marker::Destruct;
        type preload: crate::MaybeUpdateValueByRef<str> + ~const ::core::marker::Destruct;
        type src: crate::MaybeUpdateValueByRef<str> + ~const ::core::marker::Destruct;
    }
}
pub use trait_types::Types;
pub use trait_types::Types as ValidTypes;
pub mod data_struct {
    #[non_exhaustive]
    pub struct HtmlMediaElementProps<TypeDefs: super::Types + ?::core::marker::Sized> {
        pub HtmlElementProps: super::super::HtmlElementProps::Data<TypeDefs::HtmlElementProps>,
        pub auto_play: TypeDefs::auto_play,
        pub controls: TypeDefs::controls,
        pub cross_origin: TypeDefs::cross_origin,
        pub loop_: TypeDefs::loop_,
        pub muted: TypeDefs::muted,
        pub preload: TypeDefs::preload,
        pub src: TypeDefs::src,
    }
}
pub use data_struct::HtmlMediaElementProps as Data;
pub struct Building<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
pub struct Replacing<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
mod types_initial {
    #[allow(unused_imports)]
    use super::super::*;
    pub type TypesInitial = dyn super::Types<
        HtmlElementProps = HtmlElementProps::TypesInitial,
        auto_play = (),
        controls = (),
        cross_origin = (),
        loop_ = (),
        muted = (),
        preload = (),
        src = (),
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
                auto_play: self.0.auto_play,
                controls: self.0.controls,
                cross_origin: self.0.cross_origin,
                loop_: self.0.loop_,
                muted: self.0.muted,
                preload: self.0.preload,
                src: self.0.src,
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
                auto_play: self.0.auto_play,
                controls: self.0.controls,
                cross_origin: self.0.cross_origin,
                loop_: self.0.loop_,
                muted: self.0.muted,
                preload: self.0.preload,
                src: self.0.src,
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
                auto_play: self.0.auto_play,
                controls: self.0.controls,
                cross_origin: self.0.cross_origin,
                loop_: self.0.loop_,
                muted: self.0.muted,
                preload: self.0.preload,
                src: self.0.src,
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
                auto_play: self.0.auto_play,
                controls: self.0.controls,
                cross_origin: self.0.cross_origin,
                loop_: self.0.loop_,
                muted: self.0.muted,
                preload: self.0.preload,
                src: self.0.src,
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
                auto_play: self.0.auto_play,
                controls: self.0.controls,
                cross_origin: self.0.cross_origin,
                loop_: self.0.loop_,
                muted: self.0.muted,
                preload: self.0.preload,
                src: self.0.src,
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
                auto_play: self.0.auto_play,
                controls: self.0.controls,
                cross_origin: self.0.cross_origin,
                loop_: self.0.loop_,
                muted: self.0.muted,
                preload: self.0.preload,
                src: self.0.src,
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
                auto_play: self.0.auto_play,
                controls: self.0.controls,
                cross_origin: self.0.cross_origin,
                loop_: self.0.loop_,
                muted: self.0.muted,
                preload: self.0.preload,
                src: self.0.src,
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
                auto_play: self.0.auto_play,
                controls: self.0.controls,
                cross_origin: self.0.cross_origin,
                loop_: self.0.loop_,
                muted: self.0.muted,
                preload: self.0.preload,
                src: self.0.src,
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
                auto_play: self.0.auto_play,
                controls: self.0.controls,
                cross_origin: self.0.cross_origin,
                loop_: self.0.loop_,
                muted: self.0.muted,
                preload: self.0.preload,
                src: self.0.src,
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
                auto_play: self.0.auto_play,
                controls: self.0.controls,
                cross_origin: self.0.cross_origin,
                loop_: self.0.loop_,
                muted: self.0.muted,
                preload: self.0.preload,
                src: self.0.src,
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
                auto_play: self.0.auto_play,
                controls: self.0.controls,
                cross_origin: self.0.cross_origin,
                loop_: self.0.loop_,
                muted: self.0.muted,
                preload: self.0.preload,
                src: self.0.src,
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
                auto_play: self.0.auto_play,
                controls: self.0.controls,
                cross_origin: self.0.cross_origin,
                loop_: self.0.loop_,
                muted: self.0.muted,
                preload: self.0.preload,
                src: self.0.src,
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
                auto_play: self.0.auto_play,
                controls: self.0.controls,
                cross_origin: self.0.cross_origin,
                loop_: self.0.loop_,
                muted: self.0.muted,
                preload: self.0.preload,
                src: self.0.src,
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
                auto_play: self.0.auto_play,
                controls: self.0.controls,
                cross_origin: self.0.cross_origin,
                loop_: self.0.loop_,
                muted: self.0.muted,
                preload: self.0.preload,
                src: self.0.src,
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
                auto_play: self.0.auto_play,
                controls: self.0.controls,
                cross_origin: self.0.cross_origin,
                loop_: self.0.loop_,
                muted: self.0.muted,
                preload: self.0.preload,
                src: self.0.src,
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
                auto_play: self.0.auto_play,
                controls: self.0.controls,
                cross_origin: self.0.cross_origin,
                loop_: self.0.loop_,
                muted: self.0.muted,
                preload: self.0.preload,
                src: self.0.src,
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
                auto_play: self.0.auto_play,
                controls: self.0.controls,
                cross_origin: self.0.cross_origin,
                loop_: self.0.loop_,
                muted: self.0.muted,
                preload: self.0.preload,
                src: self.0.src,
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
                auto_play: self.0.auto_play,
                controls: self.0.controls,
                cross_origin: self.0.cross_origin,
                loop_: self.0.loop_,
                muted: self.0.muted,
                preload: self.0.preload,
                src: self.0.src,
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
                auto_play: self.0.auto_play,
                controls: self.0.controls,
                cross_origin: self.0.cross_origin,
                loop_: self.0.loop_,
                muted: self.0.muted,
                preload: self.0.preload,
                src: self.0.src,
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
                auto_play: self.0.auto_play,
                controls: self.0.controls,
                cross_origin: self.0.cross_origin,
                loop_: self.0.loop_,
                muted: self.0.muted,
                preload: self.0.preload,
                src: self.0.src,
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
                auto_play: self.0.auto_play,
                controls: self.0.controls,
                cross_origin: self.0.cross_origin,
                loop_: self.0.loop_,
                muted: self.0.muted,
                preload: self.0.preload,
                src: self.0.src,
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
                auto_play: self.0.auto_play,
                controls: self.0.controls,
                cross_origin: self.0.cross_origin,
                loop_: self.0.loop_,
                muted: self.0.muted,
                preload: self.0.preload,
                src: self.0.src,
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
                auto_play: self.0.auto_play,
                controls: self.0.controls,
                cross_origin: self.0.cross_origin,
                loop_: self.0.loop_,
                muted: self.0.muted,
                preload: self.0.preload,
                src: self.0.src,
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
                auto_play: self.0.auto_play,
                controls: self.0.controls,
                cross_origin: self.0.cross_origin,
                loop_: self.0.loop_,
                muted: self.0.muted,
                preload: self.0.preload,
                src: self.0.src,
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
                auto_play: self.0.auto_play,
                controls: self.0.controls,
                cross_origin: self.0.cross_origin,
                loop_: self.0.loop_,
                muted: self.0.muted,
                preload: self.0.preload,
                src: self.0.src,
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
                auto_play: self.0.auto_play,
                controls: self.0.controls,
                cross_origin: self.0.cross_origin,
                loop_: self.0.loop_,
                muted: self.0.muted,
                preload: self.0.preload,
                src: self.0.src,
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
                auto_play: self.0.auto_play,
                controls: self.0.controls,
                cross_origin: self.0.cross_origin,
                loop_: self.0.loop_,
                muted: self.0.muted,
                preload: self.0.preload,
                src: self.0.src,
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
                auto_play: self.0.auto_play,
                controls: self.0.controls,
                cross_origin: self.0.cross_origin,
                loop_: self.0.loop_,
                muted: self.0.muted,
                preload: self.0.preload,
                src: self.0.src,
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
                auto_play: self.0.auto_play,
                controls: self.0.controls,
                cross_origin: self.0.cross_origin,
                loop_: self.0.loop_,
                muted: self.0.muted,
                preload: self.0.preload,
                src: self.0.src,
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
                auto_play: self.0.auto_play,
                controls: self.0.controls,
                cross_origin: self.0.cross_origin,
                loop_: self.0.loop_,
                muted: self.0.muted,
                preload: self.0.preload,
                src: self.0.src,
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
                auto_play: self.0.auto_play,
                controls: self.0.controls,
                cross_origin: self.0.cross_origin,
                loop_: self.0.loop_,
                muted: self.0.muted,
                preload: self.0.preload,
                src: self.0.src,
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
                auto_play: self.0.auto_play,
                controls: self.0.controls,
                cross_origin: self.0.cross_origin,
                loop_: self.0.loop_,
                muted: self.0.muted,
                preload: self.0.preload,
                src: self.0.src,
            })
        }
        #[inline(always)]
        pub const fn auto_play<V: crate::MaybeUpdateValue<bool>>(
            self,
            auto_play: V,
        ) -> super::Building<super::overwrite::auto_play<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                auto_play,
                controls: self.0.controls,
                cross_origin: self.0.cross_origin,
                loop_: self.0.loop_,
                muted: self.0.muted,
                preload: self.0.preload,
                src: self.0.src,
            })
        }
        #[inline(always)]
        pub const fn controls<V: crate::MaybeUpdateValue<bool>>(
            self,
            controls: V,
        ) -> super::Building<super::overwrite::controls<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                auto_play: self.0.auto_play,
                controls,
                cross_origin: self.0.cross_origin,
                loop_: self.0.loop_,
                muted: self.0.muted,
                preload: self.0.preload,
                src: self.0.src,
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
                auto_play: self.0.auto_play,
                controls: self.0.controls,
                cross_origin,
                loop_: self.0.loop_,
                muted: self.0.muted,
                preload: self.0.preload,
                src: self.0.src,
            })
        }
        #[inline(always)]
        pub const fn loop_<V: crate::MaybeUpdateValue<bool>>(
            self,
            loop_: V,
        ) -> super::Building<super::overwrite::loop_<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                auto_play: self.0.auto_play,
                controls: self.0.controls,
                cross_origin: self.0.cross_origin,
                loop_,
                muted: self.0.muted,
                preload: self.0.preload,
                src: self.0.src,
            })
        }
        #[inline(always)]
        pub const fn muted<V: crate::MaybeUpdateValue<bool>>(
            self,
            muted: V,
        ) -> super::Building<super::overwrite::muted<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                auto_play: self.0.auto_play,
                controls: self.0.controls,
                cross_origin: self.0.cross_origin,
                loop_: self.0.loop_,
                muted,
                preload: self.0.preload,
                src: self.0.src,
            })
        }
        #[inline(always)]
        pub const fn preload<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            preload: V,
        ) -> super::Building<super::overwrite::preload<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                auto_play: self.0.auto_play,
                controls: self.0.controls,
                cross_origin: self.0.cross_origin,
                loop_: self.0.loop_,
                muted: self.0.muted,
                preload,
                src: self.0.src,
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
                auto_play: self.0.auto_play,
                controls: self.0.controls,
                cross_origin: self.0.cross_origin,
                loop_: self.0.loop_,
                muted: self.0.muted,
                preload: self.0.preload,
                src,
            })
        }
    }
}
#[cfg(feature = "dom")]
mod impl_update_element {
    #[allow(unused_imports)]
    use super::super::*;
    impl<TypeDefs: ?::core::marker::Sized + super::Types>
        crate::props::UpdateElement<web_sys::HtmlMediaElement> for super::Data<TypeDefs>
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
            element: &web_sys::HtmlMediaElement,
            children_ctx: &mut ::frender_dom::Dom,
        ) -> Self::State {
            let dom_element: &::web_sys::Element = element.as_ref();
            < TypeDefs :: auto_play as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (this . auto_play , | v | element . set_autoplay (v) , | | dom_element . remove_attribute ("autoplay") . unwrap () ,) ;
            < TypeDefs :: controls as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (this . controls , | v | element . set_controls (v) , | | dom_element . remove_attribute ("controls") . unwrap () ,) ;
            < TypeDefs :: cross_origin as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . cross_origin , match element { el => | v : & _ | el . set_cross_origin (Some (v)) , } , match element { el => | | el . set_cross_origin (None) , } ,) ;
            <TypeDefs::loop_ as ::frender_dom::props::MaybeUpdateValue<bool>>::maybe_update_value(
                this.loop_,
                |v| element.set_loop(v),
                || dom_element.remove_attribute("loop").unwrap(),
            );
            <TypeDefs::muted as ::frender_dom::props::MaybeUpdateValue<bool>>::maybe_update_value(
                this.muted,
                |v| element.set_muted(v),
                || dom_element.remove_attribute("muted").unwrap(),
            );
            < TypeDefs :: preload as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . preload , | v | element . set_preload (v) , | | dom_element . remove_attribute ("preload") . unwrap () ,) ;
            < TypeDefs :: src as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . src , | v | element . set_src (v) , | | dom_element . remove_attribute ("src") . unwrap () ,) ;
            <HtmlElementProps::Data<TypeDefs::HtmlElementProps> as crate::props::UpdateElement<
                web_sys::HtmlElement,
            >>::initialize_state(this.HtmlElementProps, element, children_ctx)
        }
        fn update_element(
            this: Self,
            element: &web_sys::HtmlMediaElement,
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
            < TypeDefs :: auto_play as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (this . auto_play , | v | element . set_autoplay (v) , | | dom_element . remove_attribute ("autoplay") . unwrap () ,) ;
            < TypeDefs :: controls as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (this . controls , | v | element . set_controls (v) , | | dom_element . remove_attribute ("controls") . unwrap () ,) ;
            < TypeDefs :: cross_origin as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . cross_origin , match element { el => | v : & _ | el . set_cross_origin (Some (v)) , } , match element { el => | | el . set_cross_origin (None) , } ,) ;
            <TypeDefs::loop_ as ::frender_dom::props::MaybeUpdateValue<bool>>::maybe_update_value(
                this.loop_,
                |v| element.set_loop(v),
                || dom_element.remove_attribute("loop").unwrap(),
            );
            <TypeDefs::muted as ::frender_dom::props::MaybeUpdateValue<bool>>::maybe_update_value(
                this.muted,
                |v| element.set_muted(v),
                || dom_element.remove_attribute("muted").unwrap(),
            );
            < TypeDefs :: preload as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . preload , | v | element . set_preload (v) , | | dom_element . remove_attribute ("preload") . unwrap () ,) ;
            < TypeDefs :: src as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . src , | v | element . set_src (v) , | | dom_element . remove_attribute ("src") . unwrap () ,) ;
        }
    }
}
