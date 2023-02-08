#[allow(non_snake_case)]
pub fn HtmlVideoElementProps() -> Building<TypesInitial> {
    #[allow(unused_imports)]
    use super::*;
    self::Building(self::Data {
        HtmlMediaElementProps: HtmlMediaElementProps::build(HtmlMediaElementProps()),
        height: (),
        plays_inline: (),
        poster: (),
        width: (),
    })
}
pub mod prelude {}
pub mod overwrite {
    #![allow(non_camel_case_types)]
    pub type HtmlMediaElementProps<TypeDefs, Value> = dyn super::Types<
        HtmlMediaElementProps = Value,
        height = <TypeDefs as super::Types>::height,
        plays_inline = <TypeDefs as super::Types>::plays_inline,
        poster = <TypeDefs as super::Types>::poster,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type HtmlElementProps<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::HtmlElementProps<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type ElementProps<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::ElementProps<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type children<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::children<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type class<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::class<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type id<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::id<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type part<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::part<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type access_key<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::access_key<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type auto_capitalize<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::auto_capitalize<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type auto_focus<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::auto_focus<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type content_editable<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::content_editable<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type context_menu<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::context_menu<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type dir<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::dir<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type draggable<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::draggable<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type enter_key_hint<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::enter_key_hint<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type hidden<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::hidden<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type inert<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::inert<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type input_mode<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::input_mode<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type is<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::is<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type item_id<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::item_id<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type item_prop<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::item_prop<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type item_ref<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::item_ref<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type item_scope<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::item_scope<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type item_type<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::item_type<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type lang<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::lang<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type nonce<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::nonce<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type role<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::role<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type slot<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::slot<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type spellcheck<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::spellcheck<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type style<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::style<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type tab_index<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::tab_index<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type title<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::title<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type translate<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::translate<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type virtual_keyboard_policy<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::virtual_keyboard_policy<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_click<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_click<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type auto_play<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::auto_play<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type controls<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::controls<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type cross_origin<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::cross_origin<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type loop_<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::loop_<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type muted<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::muted<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type preload<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::preload<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type src<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::src<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type height<TypeDefs, Value> = dyn super::Types<
        HtmlMediaElementProps = <TypeDefs as super::Types>::HtmlMediaElementProps,
        height = Value,
        plays_inline = <TypeDefs as super::Types>::plays_inline,
        poster = <TypeDefs as super::Types>::poster,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type plays_inline<TypeDefs, Value> = dyn super::Types<
        HtmlMediaElementProps = <TypeDefs as super::Types>::HtmlMediaElementProps,
        height = <TypeDefs as super::Types>::height,
        plays_inline = Value,
        poster = <TypeDefs as super::Types>::poster,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type poster<TypeDefs, Value> = dyn super::Types<
        HtmlMediaElementProps = <TypeDefs as super::Types>::HtmlMediaElementProps,
        height = <TypeDefs as super::Types>::height,
        plays_inline = <TypeDefs as super::Types>::plays_inline,
        poster = Value,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type width<TypeDefs, Value> = dyn super::Types<
        HtmlMediaElementProps = <TypeDefs as super::Types>::HtmlMediaElementProps,
        height = <TypeDefs as super::Types>::height,
        plays_inline = <TypeDefs as super::Types>::plays_inline,
        poster = <TypeDefs as super::Types>::poster,
        width = Value,
    >;
}
mod trait_types {
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub trait Types {
        type HtmlMediaElementProps: ?::core::marker::Sized + HtmlMediaElementProps::Types;
        type height: crate::MaybeUpdateValue<u32>;
        type plays_inline: crate::MaybeUpdateValue<bool>;
        type poster: crate::MaybeUpdateValueByRef<str>;
        type width: crate::MaybeUpdateValue<u32>;
    }
}
pub use trait_types::Types;
pub use trait_types::Types as ValidTypes;
pub mod data_struct {
    #[non_exhaustive]
    pub struct HtmlVideoElementProps<TypeDefs: super::Types + ?::core::marker::Sized> {
        pub HtmlMediaElementProps:
            super::super::HtmlMediaElementProps::Data<TypeDefs::HtmlMediaElementProps>,
        pub height: TypeDefs::height,
        pub plays_inline: TypeDefs::plays_inline,
        pub poster: TypeDefs::poster,
        pub width: TypeDefs::width,
    }
}
pub use data_struct::HtmlVideoElementProps as Data;
pub struct Building<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
pub struct Replacing<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
mod types_initial {
    #[allow(unused_imports)]
    use super::super::*;
    pub type TypesInitial = dyn super::Types<
        HtmlMediaElementProps = HtmlMediaElementProps::TypesInitial,
        height = (),
        plays_inline = (),
        poster = (),
        width = (),
    >;
}
pub use types_initial::TypesInitial;
pub type DataInitial = Data<TypesInitial>;
#[cfg(feature = "dom")]
pub mod render_state {
    #[allow(non_camel_case_types)]
    pub trait RenderStateTypes {
        type HtmlMediaElementProps: ::core::default::Default
            + crate::props::IntrinsicComponentPollReactive;
    }
    pub struct RenderState<TypeDefs: RenderStateTypes>
    where
        TypeDefs: ?::core::marker::Sized,
    {
        pub HtmlMediaElementProps: TypeDefs::HtmlMediaElementProps,
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
        pub HtmlMediaElementProps:
            ::pin_project_lite::__private::Pin<&'__pin mut (TypeDefs::HtmlMediaElementProps)>,
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
            pub HtmlMediaElementProps:
                ::pin_project_lite::__private::Pin<&'__pin (TypeDefs::HtmlMediaElementProps)>,
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
                        HtmlMediaElementProps,
                    } = self.get_unchecked_mut();
                    RenderStateProj {
                        HtmlMediaElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            HtmlMediaElementProps,
                        ),
                    }
                }
            }
            pub(crate) fn project_ref<'__pin>(
                self: ::pin_project_lite::__private::Pin<&'__pin Self>,
            ) -> ProjectionRef<'__pin, TypeDefs> {
                unsafe {
                    let Self {
                        HtmlMediaElementProps,
                    } = self.get_ref();
                    ProjectionRef {
                        HtmlMediaElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            HtmlMediaElementProps,
                        ),
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
            HtmlMediaElementProps: TypeDefs::HtmlMediaElementProps,
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
            let _ = &this.HtmlMediaElementProps;
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
                HtmlMediaElementProps: ::core::default::Default::default(),
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
                self.project().HtmlMediaElementProps,
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
        #[doc = "See [`HtmlMediaElementProps::children`]"]
        #[inline]
        pub fn children<V>(
            self,
            children: V,
        ) -> super::Building<super::overwrite::children<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .children(children),
                ),
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::class`]"]
        #[inline]
        pub fn class<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            class: V,
        ) -> super::Building<super::overwrite::class<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps).class(class),
                ),
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::id`]"]
        #[inline]
        pub fn id<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            id: V,
        ) -> super::Building<super::overwrite::id<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps).id(id),
                ),
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::part`]"]
        #[inline]
        pub fn part<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            part: V,
        ) -> super::Building<super::overwrite::part<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps).part(part),
                ),
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::access_key`]"]
        #[inline]
        pub fn access_key<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            access_key: V,
        ) -> super::Building<super::overwrite::access_key<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .access_key(access_key),
                ),
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::auto_capitalize`]"]
        #[inline]
        pub fn auto_capitalize<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            auto_capitalize: V,
        ) -> super::Building<super::overwrite::auto_capitalize<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .auto_capitalize(auto_capitalize),
                ),
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::auto_focus`]"]
        #[inline]
        pub fn auto_focus<V: crate::MaybeUpdateValue<bool>>(
            self,
            auto_focus: V,
        ) -> super::Building<super::overwrite::auto_focus<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .auto_focus(auto_focus),
                ),
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::content_editable`]"]
        #[inline]
        pub fn content_editable<V: crate::props::MaybeInherit<bool>>(
            self,
            content_editable: V,
        ) -> super::Building<super::overwrite::content_editable<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .content_editable(content_editable),
                ),
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::context_menu`]"]
        #[inline]
        pub fn context_menu<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            context_menu: V,
        ) -> super::Building<super::overwrite::context_menu<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .context_menu(context_menu),
                ),
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::dir`]"]
        #[inline]
        pub fn dir<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            dir: V,
        ) -> super::Building<super::overwrite::dir<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps).dir(dir),
                ),
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::draggable`]"]
        #[inline]
        pub fn draggable<V: crate::MaybeUpdateValue<bool>>(
            self,
            draggable: V,
        ) -> super::Building<super::overwrite::draggable<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .draggable(draggable),
                ),
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::enter_key_hint`]"]
        #[inline]
        pub fn enter_key_hint<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            enter_key_hint: V,
        ) -> super::Building<super::overwrite::enter_key_hint<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .enter_key_hint(enter_key_hint),
                ),
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::hidden`]"]
        #[inline]
        pub fn hidden<V: crate::MaybeUpdateValue<bool>>(
            self,
            hidden: V,
        ) -> super::Building<super::overwrite::hidden<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps).hidden(hidden),
                ),
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::inert`]"]
        #[inline]
        pub fn inert<V: crate::MaybeUpdateValue<bool>>(
            self,
            inert: V,
        ) -> super::Building<super::overwrite::inert<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps).inert(inert),
                ),
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::input_mode`]"]
        #[inline]
        pub fn input_mode<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            input_mode: V,
        ) -> super::Building<super::overwrite::input_mode<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .input_mode(input_mode),
                ),
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::is`]"]
        #[inline]
        pub fn is<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            is: V,
        ) -> super::Building<super::overwrite::is<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps).is(is),
                ),
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::item_id`]"]
        #[inline]
        pub fn item_id<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            item_id: V,
        ) -> super::Building<super::overwrite::item_id<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps).item_id(item_id),
                ),
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::item_prop`]"]
        #[inline]
        pub fn item_prop<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            item_prop: V,
        ) -> super::Building<super::overwrite::item_prop<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .item_prop(item_prop),
                ),
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::item_ref`]"]
        #[inline]
        pub fn item_ref<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            item_ref: V,
        ) -> super::Building<super::overwrite::item_ref<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .item_ref(item_ref),
                ),
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::item_scope`]"]
        #[inline]
        pub fn item_scope<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            item_scope: V,
        ) -> super::Building<super::overwrite::item_scope<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .item_scope(item_scope),
                ),
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::item_type`]"]
        #[inline]
        pub fn item_type<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            item_type: V,
        ) -> super::Building<super::overwrite::item_type<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .item_type(item_type),
                ),
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::lang`]"]
        #[inline]
        pub fn lang<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            lang: V,
        ) -> super::Building<super::overwrite::lang<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps).lang(lang),
                ),
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::nonce`]"]
        #[inline]
        pub fn nonce<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            nonce: V,
        ) -> super::Building<super::overwrite::nonce<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps).nonce(nonce),
                ),
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::role`]"]
        #[inline]
        pub fn role<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            role: V,
        ) -> super::Building<super::overwrite::role<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps).role(role),
                ),
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::slot`]"]
        #[inline]
        pub fn slot<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            slot: V,
        ) -> super::Building<super::overwrite::slot<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps).slot(slot),
                ),
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::spellcheck`]"]
        #[inline]
        pub fn spellcheck<V: crate::MaybeUpdateValue<bool>>(
            self,
            spellcheck: V,
        ) -> super::Building<super::overwrite::spellcheck<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .spellcheck(spellcheck),
                ),
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::style`]"]
        #[inline]
        pub fn style<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            style: V,
        ) -> super::Building<super::overwrite::style<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps).style(style),
                ),
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::tab_index`]"]
        #[inline]
        pub fn tab_index<V: crate::MaybeUpdateValue<i32>>(
            self,
            tab_index: V,
        ) -> super::Building<super::overwrite::tab_index<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .tab_index(tab_index),
                ),
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::title`]"]
        #[inline]
        pub fn title<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            title: V,
        ) -> super::Building<super::overwrite::title<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps).title(title),
                ),
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::translate`]"]
        #[inline]
        pub fn translate<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            translate: V,
        ) -> super::Building<super::overwrite::translate<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .translate(translate),
                ),
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::virtual_keyboard_policy`]"]
        #[inline]
        pub fn virtual_keyboard_policy<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            virtual_keyboard_policy: V,
        ) -> super::Building<super::overwrite::virtual_keyboard_policy<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .virtual_keyboard_policy(virtual_keyboard_policy),
                ),
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::on_click`]"]
        #[inline]
        pub fn on_click<V>(
            self,
            on_click: V,
        ) -> super::Building<super::overwrite::on_click<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .on_click(on_click),
                ),
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::auto_play`]"]
        #[inline]
        pub fn auto_play<V: crate::MaybeUpdateValue<bool>>(
            self,
            auto_play: V,
        ) -> super::Building<super::overwrite::auto_play<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .auto_play(auto_play),
                ),
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::controls`]"]
        #[inline]
        pub fn controls<V: crate::MaybeUpdateValue<bool>>(
            self,
            controls: V,
        ) -> super::Building<super::overwrite::controls<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .controls(controls),
                ),
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::cross_origin`]"]
        #[inline]
        pub fn cross_origin<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            cross_origin: V,
        ) -> super::Building<super::overwrite::cross_origin<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .cross_origin(cross_origin),
                ),
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::loop_`]"]
        #[inline]
        pub fn loop_<V: crate::MaybeUpdateValue<bool>>(
            self,
            loop_: V,
        ) -> super::Building<super::overwrite::loop_<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps).loop_(loop_),
                ),
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::muted`]"]
        #[inline]
        pub fn muted<V: crate::MaybeUpdateValue<bool>>(
            self,
            muted: V,
        ) -> super::Building<super::overwrite::muted<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps).muted(muted),
                ),
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::preload`]"]
        #[inline]
        pub fn preload<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            preload: V,
        ) -> super::Building<super::overwrite::preload<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps).preload(preload),
                ),
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::src`]"]
        #[inline]
        pub fn src<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            src: V,
        ) -> super::Building<super::overwrite::src<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps).src(src),
                ),
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn height<V: crate::MaybeUpdateValue<u32>>(
            self,
            height: V,
        ) -> super::Building<super::overwrite::height<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: self.0.HtmlMediaElementProps,
                height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn plays_inline<V: crate::MaybeUpdateValue<bool>>(
            self,
            plays_inline: V,
        ) -> super::Building<super::overwrite::plays_inline<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: self.0.HtmlMediaElementProps,
                height: self.0.height,
                plays_inline,
                poster: self.0.poster,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn poster<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            poster: V,
        ) -> super::Building<super::overwrite::poster<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: self.0.HtmlMediaElementProps,
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn width<V: crate::MaybeUpdateValue<u32>>(
            self,
            width: V,
        ) -> super::Building<super::overwrite::width<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: self.0.HtmlMediaElementProps,
                height: self.0.height,
                plays_inline: self.0.plays_inline,
                poster: self.0.poster,
                width,
            })
        }
    }
}
#[cfg(feature = "dom")]
mod impl_update_element {
    #[allow(unused_imports)]
    use super::super::*;
    impl<TypeDefs: ?::core::marker::Sized + super::Types>
        crate::props::UpdateElement<web_sys::HtmlVideoElement> for super::Data<TypeDefs>
    where
        HtmlMediaElementProps::Data<TypeDefs::HtmlMediaElementProps>:
            crate::props::UpdateElement<web_sys::HtmlMediaElement>,
    {
        type State = super::render_state::RenderState<
            dyn super::render_state::RenderStateTypes<
                HtmlMediaElementProps = <HtmlMediaElementProps::Data<
                    TypeDefs::HtmlMediaElementProps,
                > as crate::props::UpdateElement<web_sys::HtmlMediaElement>>::State,
            >,
        >;
        fn update_element(
            this: Self,
            element: &web_sys::HtmlVideoElement,
            children_ctx: &mut ::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let state = state.pin_project();
            let dom_element: &::web_sys::Element = element.as_ref();
            crate::props::UpdateElement::update_element(
                this.HtmlMediaElementProps,
                element.as_ref(),
                children_ctx,
                state.HtmlMediaElementProps,
            );
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "height";
                < TypeDefs :: height as :: frender_dom :: props :: MaybeUpdateValue < u32 , > > :: maybe_update_value (this . height , | v | element . set_height (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "playsinline";
                < TypeDefs :: plays_inline as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (this . plays_inline , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "poster";
                < TypeDefs :: poster as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . poster , | v | element . set_poster (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "width";
                <TypeDefs::width as ::frender_dom::props::MaybeUpdateValue<u32>>::maybe_update_value(
                    this.width,
                    |v| element.set_width(v),
                    || dom_element.remove_attribute(ATTR_NAME).unwrap(),
                )
            }
        }
    }
}
