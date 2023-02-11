#[allow(non_snake_case)]
#[inline(always)]
pub fn HtmlMediaElementProps() -> Building<TypesInitial> {
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
        type HtmlElementProps: ?::core::marker::Sized + HtmlElementProps::Types;
        type auto_play: crate::MaybeUpdateValueWithState<bool>;
        type controls: crate::MaybeUpdateValueWithState<bool>;
        type cross_origin: crate::MaybeUpdateValueWithState<str>;
        type loop_: crate::MaybeUpdateValueWithState<bool>;
        type muted: crate::MaybeUpdateValueWithState<bool>;
        type preload: crate::MaybeUpdateValueWithState<str>;
        type src: crate::MaybeUpdateValueWithState<str>;
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
pub mod render_state {
    #[allow(non_camel_case_types)]
    pub trait RenderStateTypes {
        type HtmlElementProps: crate::props::IntrinsicComponentPollReactive;
        type auto_play;
        type controls;
        type cross_origin;
        type loop_;
        type muted;
        type preload;
        type src;
    }
    pub struct RenderState<TypeDefs: RenderStateTypes>
    where
        TypeDefs: ?::core::marker::Sized,
    {
        pub HtmlElementProps: TypeDefs::HtmlElementProps,
        pub auto_play: TypeDefs::auto_play,
        pub controls: TypeDefs::controls,
        pub cross_origin: TypeDefs::cross_origin,
        pub loop_: TypeDefs::loop_,
        pub muted: TypeDefs::muted,
        pub preload: TypeDefs::preload,
        pub src: TypeDefs::src,
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
        pub auto_play: &'__pin mut (TypeDefs::auto_play),
        pub controls: &'__pin mut (TypeDefs::controls),
        pub cross_origin: &'__pin mut (TypeDefs::cross_origin),
        pub loop_: &'__pin mut (TypeDefs::loop_),
        pub muted: &'__pin mut (TypeDefs::muted),
        pub preload: &'__pin mut (TypeDefs::preload),
        pub src: &'__pin mut (TypeDefs::src),
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
            pub auto_play: &'__pin (TypeDefs::auto_play),
            pub controls: &'__pin (TypeDefs::controls),
            pub cross_origin: &'__pin (TypeDefs::cross_origin),
            pub loop_: &'__pin (TypeDefs::loop_),
            pub muted: &'__pin (TypeDefs::muted),
            pub preload: &'__pin (TypeDefs::preload),
            pub src: &'__pin (TypeDefs::src),
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
                        auto_play,
                        controls,
                        cross_origin,
                        loop_,
                        muted,
                        preload,
                        src,
                    } = self.get_unchecked_mut();
                    RenderStateProj {
                        HtmlElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            HtmlElementProps,
                        ),
                        auto_play: auto_play,
                        controls: controls,
                        cross_origin: cross_origin,
                        loop_: loop_,
                        muted: muted,
                        preload: preload,
                        src: src,
                    }
                }
            }
            pub(crate) fn project_ref<'__pin>(
                self: ::pin_project_lite::__private::Pin<&'__pin Self>,
            ) -> ProjectionRef<'__pin, TypeDefs> {
                unsafe {
                    let Self {
                        HtmlElementProps,
                        auto_play,
                        controls,
                        cross_origin,
                        loop_,
                        muted,
                        preload,
                        src,
                    } = self.get_ref();
                    ProjectionRef {
                        HtmlElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            HtmlElementProps,
                        ),
                        auto_play: auto_play,
                        controls: controls,
                        cross_origin: cross_origin,
                        loop_: loop_,
                        muted: muted,
                        preload: preload,
                        src: src,
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
            auto_play: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::auto_play>,
            controls: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::controls>,
            cross_origin: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::cross_origin>,
            loop_: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::loop_>,
            muted: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::muted>,
            preload: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::preload>,
            src: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::src>,
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
            let _ = &this.auto_play;
            let _ = &this.controls;
            let _ = &this.cross_origin;
            let _ = &this.loop_;
            let _ = &this.muted;
            let _ = &this.preload;
            let _ = &this.src;
        }
    };
    impl<TypeDefs: ?::core::marker::Sized + RenderStateTypes> RenderState<TypeDefs> {
        #[inline(always)]
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
#[inline(always)]
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
        #[inline(always)]
        pub fn children<V>(
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
        ///See [`HtmlElementProps::class`]
        #[inline(always)]
        pub fn class<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::id`]
        #[inline(always)]
        pub fn id<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::part`]
        #[inline(always)]
        pub fn part<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::access_key`]
        #[inline(always)]
        pub fn access_key<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::auto_capitalize`]
        #[inline(always)]
        pub fn auto_capitalize<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::auto_focus`]
        #[inline(always)]
        pub fn auto_focus<V: crate::MaybeUpdateValueWithState<bool>>(
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
        ///See [`HtmlElementProps::content_editable`]
        #[inline(always)]
        pub fn content_editable<V: crate::props::MaybeInherit<bool>>(
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
        ///See [`HtmlElementProps::context_menu`]
        #[inline(always)]
        pub fn context_menu<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::dir`]
        #[inline(always)]
        pub fn dir<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::draggable`]
        #[inline(always)]
        pub fn draggable<V: crate::MaybeUpdateValueWithState<bool>>(
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
        ///See [`HtmlElementProps::enter_key_hint`]
        #[inline(always)]
        pub fn enter_key_hint<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::hidden`]
        #[inline(always)]
        pub fn hidden<V: crate::MaybeUpdateValueWithState<bool>>(
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
        ///See [`HtmlElementProps::inert`]
        #[inline(always)]
        pub fn inert<V: crate::MaybeUpdateValueWithState<bool>>(
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
        ///See [`HtmlElementProps::input_mode`]
        #[inline(always)]
        pub fn input_mode<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::is`]
        #[inline(always)]
        pub fn is<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::item_id`]
        #[inline(always)]
        pub fn item_id<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::item_prop`]
        #[inline(always)]
        pub fn item_prop<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::item_ref`]
        #[inline(always)]
        pub fn item_ref<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::item_scope`]
        #[inline(always)]
        pub fn item_scope<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::item_type`]
        #[inline(always)]
        pub fn item_type<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::lang`]
        #[inline(always)]
        pub fn lang<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::nonce`]
        #[inline(always)]
        pub fn nonce<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::role`]
        #[inline(always)]
        pub fn role<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::slot`]
        #[inline(always)]
        pub fn slot<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::spellcheck`]
        #[inline(always)]
        pub fn spellcheck<V: crate::MaybeUpdateValueWithState<bool>>(
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
        ///See [`HtmlElementProps::style`]
        #[inline(always)]
        pub fn style<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::tab_index`]
        #[inline(always)]
        pub fn tab_index<V: crate::MaybeUpdateValueWithState<i32>>(
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
        ///See [`HtmlElementProps::title`]
        #[inline(always)]
        pub fn title<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::translate`]
        #[inline(always)]
        pub fn translate<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::virtual_keyboard_policy`]
        #[inline(always)]
        pub fn virtual_keyboard_policy<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::on_click`]
        #[inline(always)]
        pub fn on_click<V>(
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
        pub fn auto_play<V: crate::MaybeUpdateValueWithState<bool>>(
            self,
            auto_play: V,
        ) -> super::Building<super::overwrite::auto_play<TypeDefs, V>> {
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
        pub fn controls<V: crate::MaybeUpdateValueWithState<bool>>(
            self,
            controls: V,
        ) -> super::Building<super::overwrite::controls<TypeDefs, V>> {
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
        pub fn cross_origin<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            cross_origin: V,
        ) -> super::Building<super::overwrite::cross_origin<TypeDefs, V>> {
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
        pub fn loop_<V: crate::MaybeUpdateValueWithState<bool>>(
            self,
            loop_: V,
        ) -> super::Building<super::overwrite::loop_<TypeDefs, V>> {
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
        pub fn muted<V: crate::MaybeUpdateValueWithState<bool>>(
            self,
            muted: V,
        ) -> super::Building<super::overwrite::muted<TypeDefs, V>> {
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
        pub fn preload<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            preload: V,
        ) -> super::Building<super::overwrite::preload<TypeDefs, V>> {
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
        pub fn src<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            src: V,
        ) -> super::Building<super::overwrite::src<TypeDefs, V>> {
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
        type State = super::render_state::RenderState<
            dyn super::render_state::RenderStateTypes<
                HtmlElementProps = <HtmlElementProps::Data<
                    TypeDefs::HtmlElementProps,
                > as crate::props::UpdateElement<web_sys::HtmlElement>>::State,
                auto_play = <TypeDefs::auto_play as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::State,
                controls = <TypeDefs::controls as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::State,
                cross_origin = <TypeDefs::cross_origin as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                loop_ = <TypeDefs::loop_ as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::State,
                muted = <TypeDefs::muted as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::State,
                preload = <TypeDefs::preload as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                src = <TypeDefs::src as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
            >,
        >;
        fn initialize_state(
            this: Self,
            element: &web_sys::HtmlMediaElement,
            children_ctx: &mut ::frender_dom::Dom,
        ) -> Self::State {
            let dom_element: &::web_sys::Element = element.as_ref();
            super::render_state::RenderState {
                HtmlElementProps: <HtmlElementProps::Data<
                    TypeDefs::HtmlElementProps,
                > as crate::props::UpdateElement<
                    web_sys::HtmlElement,
                >>::initialize_state(this.HtmlElementProps, element, children_ctx),
                auto_play: <TypeDefs::auto_play as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.auto_play,
                    |v| element.set_autoplay(*v),
                    || dom_element.remove_attribute("autoplay").unwrap(),
                ),
                controls: <TypeDefs::controls as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.controls,
                    |v| element.set_controls(*v),
                    || dom_element.remove_attribute("controls").unwrap(),
                ),
                cross_origin: <TypeDefs::cross_origin as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.cross_origin,
                    match element {
                        el => |v: &_| el.set_cross_origin(Some(v)),
                    },
                    match element {
                        el => || el.set_cross_origin(None),
                    },
                ),
                loop_: <TypeDefs::loop_ as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.loop_,
                    |v| element.set_loop(*v),
                    || dom_element.remove_attribute("loop").unwrap(),
                ),
                muted: <TypeDefs::muted as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.muted,
                    |v| element.set_muted(*v),
                    || dom_element.remove_attribute("muted").unwrap(),
                ),
                preload: <TypeDefs::preload as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.preload,
                    |v| element.set_preload(v),
                    || dom_element.remove_attribute("preload").unwrap(),
                ),
                src: <TypeDefs::src as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.src,
                    |v| element.set_src(v),
                    || dom_element.remove_attribute("src").unwrap(),
                ),
            }
        }
        fn update_element(
            this: Self,
            element: &web_sys::HtmlMediaElement,
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
            <TypeDefs::auto_play as ::frender_dom::props::MaybeUpdateValueWithState<
                bool,
            >>::maybe_update_value_with_state(
                this.auto_play,
                state.auto_play,
                |v| element.set_autoplay(*v),
                || dom_element.remove_attribute("autoplay").unwrap(),
            );
            <TypeDefs::controls as ::frender_dom::props::MaybeUpdateValueWithState<
                bool,
            >>::maybe_update_value_with_state(
                this.controls,
                state.controls,
                |v| element.set_controls(*v),
                || dom_element.remove_attribute("controls").unwrap(),
            );
            <TypeDefs::cross_origin as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.cross_origin,
                state.cross_origin,
                match element {
                    el => |v: &_| el.set_cross_origin(Some(v)),
                },
                match element {
                    el => || el.set_cross_origin(None),
                },
            );
            <TypeDefs::loop_ as ::frender_dom::props::MaybeUpdateValueWithState<
                bool,
            >>::maybe_update_value_with_state(
                this.loop_,
                state.loop_,
                |v| element.set_loop(*v),
                || dom_element.remove_attribute("loop").unwrap(),
            );
            <TypeDefs::muted as ::frender_dom::props::MaybeUpdateValueWithState<
                bool,
            >>::maybe_update_value_with_state(
                this.muted,
                state.muted,
                |v| element.set_muted(*v),
                || dom_element.remove_attribute("muted").unwrap(),
            );
            <TypeDefs::preload as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.preload,
                state.preload,
                |v| element.set_preload(v),
                || dom_element.remove_attribute("preload").unwrap(),
            );
            <TypeDefs::src as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.src,
                state.src,
                |v| element.set_src(v),
                || dom_element.remove_attribute("src").unwrap(),
            );
        }
    }
}
