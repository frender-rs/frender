#[allow(non_snake_case)]
pub fn ElementProps() -> Building<TypesInitial> {
    use super::*;
    self::Building(self::Data {
        children: (),
        class: (),
        id: (),
        part: (),
    })
}
pub mod prelude {}
pub mod overwrite {
    #![allow(non_camel_case_types)]
    pub type children<TypeDefs, Value> = dyn super::Types<
        children = Value,
        class = <TypeDefs as super::Types>::class,
        id = <TypeDefs as super::Types>::id,
        part = <TypeDefs as super::Types>::part,
    >;
    pub type class<TypeDefs, Value> = dyn super::Types<
        children = <TypeDefs as super::Types>::children,
        class = Value,
        id = <TypeDefs as super::Types>::id,
        part = <TypeDefs as super::Types>::part,
    >;
    pub type id<TypeDefs, Value> = dyn super::Types<
        children = <TypeDefs as super::Types>::children,
        class = <TypeDefs as super::Types>::class,
        id = Value,
        part = <TypeDefs as super::Types>::part,
    >;
    pub type part<TypeDefs, Value> = dyn super::Types<
        children = <TypeDefs as super::Types>::children,
        class = <TypeDefs as super::Types>::class,
        id = <TypeDefs as super::Types>::id,
        part = Value,
    >;
}
mod trait_types {
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub trait Types {
        type children;
        type class: ::frender_dom::props::MaybeUpdateValue<str>;
        type id: ::frender_dom::props::MaybeUpdateValue<str>;
        type part: ::frender_dom::props::MaybeUpdateValue<str>;
    }
}
pub use trait_types::Types;
pub use trait_types::Types as ValidTypes;
pub mod data_struct {
    #[non_exhaustive]
    pub struct ElementProps<TypeDefs: super::Types + ?::core::marker::Sized> {
        pub children: TypeDefs::children,
        pub class: TypeDefs::class,
        pub id: TypeDefs::id,
        pub part: TypeDefs::part,
    }
}
pub use data_struct::ElementProps as Data;
pub struct Building<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
pub struct Replacing<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
mod types_initial {
    #[allow(unused_imports)]
    use super::super::*;
    pub type TypesInitial = dyn super::Types<children = (), class = (), id = (), part = ()>;
}
pub use types_initial::TypesInitial;
pub type DataInitial = Data<TypesInitial>;
#[cfg(feature = "dom")]
pub mod render_state {
    #[allow(non_camel_case_types)]
    pub trait RenderStateTypes {
        type children: ::frender_core::RenderState;
        type class: ::core::default::Default;
        type id: ::core::default::Default;
        type part: ::core::default::Default;
    }
    pub struct RenderState<TypeDefs: RenderStateTypes>
    where
        TypeDefs: ?::core::marker::Sized,
    {
        pub children: TypeDefs::children,
        pub class: TypeDefs::class,
        pub id: TypeDefs::id,
        pub part: TypeDefs::part,
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
        pub children: ::pin_project_lite::__private::Pin<&'__pin mut (TypeDefs::children)>,
        pub class: &'__pin mut (TypeDefs::class),
        pub id: &'__pin mut (TypeDefs::id),
        pub part: &'__pin mut (TypeDefs::part),
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
            pub children: ::pin_project_lite::__private::Pin<&'__pin (TypeDefs::children)>,
            pub class: &'__pin (TypeDefs::class),
            pub id: &'__pin (TypeDefs::id),
            pub part: &'__pin (TypeDefs::part),
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
                        children,
                        class,
                        id,
                        part,
                    } = self.get_unchecked_mut();
                    RenderStateProj {
                        children: ::pin_project_lite::__private::Pin::new_unchecked(children),
                        class: class,
                        id: id,
                        part: part,
                    }
                }
            }
            pub(crate) fn project_ref<'__pin>(
                self: ::pin_project_lite::__private::Pin<&'__pin Self>,
            ) -> ProjectionRef<'__pin, TypeDefs> {
                unsafe {
                    let Self {
                        children,
                        class,
                        id,
                        part,
                    } = self.get_ref();
                    ProjectionRef {
                        children: ::pin_project_lite::__private::Pin::new_unchecked(children),
                        class: class,
                        id: id,
                        part: part,
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
            children: TypeDefs::children,
            class: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::class>,
            id: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::id>,
            part: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::part>,
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
            let _ = &this.children;
            let _ = &this.class;
            let _ = &this.id;
            let _ = &this.part;
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
                children: ::frender_core::RenderState::new_uninitialized(),
                class: ::core::default::Default::default(),
                id: ::core::default::Default::default(),
                part: ::core::default::Default::default(),
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
            ::frender_core::RenderState::poll_reactive(self.project().children, cx)
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
    use super::super::*;
    impl<TypeDefs: ElementProps::Types + ?::core::marker::Sized> ElementProps::Building<TypeDefs> {
        pub fn children<V>(
            self,
            children: V,
        ) -> ElementProps::Building<ElementProps::overwrite::children<TypeDefs, V>> {
            let __tmp_value = children;
            let ElementProps::Data {
                children,
                class,
                id,
                part,
            } = self.0;
            let _ = children;
            let children = __tmp_value;
            ElementProps::Building(ElementProps::Data {
                children,
                class,
                id,
                part,
            })
        }
        pub fn class<V: ::frender_dom::props::MaybeUpdateValue<str>>(
            self,
            class: V,
        ) -> ElementProps::Building<ElementProps::overwrite::class<TypeDefs, V>> {
            let __tmp_value = class;
            let ElementProps::Data {
                children,
                class,
                id,
                part,
            } = self.0;
            let _ = class;
            let class = __tmp_value;
            ElementProps::Building(ElementProps::Data {
                children,
                class,
                id,
                part,
            })
        }
        pub fn id<V: ::frender_dom::props::MaybeUpdateValue<str>>(
            self,
            id: V,
        ) -> ElementProps::Building<ElementProps::overwrite::id<TypeDefs, V>> {
            let __tmp_value = id;
            let ElementProps::Data {
                children,
                class,
                id,
                part,
            } = self.0;
            let _ = id;
            let id = __tmp_value;
            ElementProps::Building(ElementProps::Data {
                children,
                class,
                id,
                part,
            })
        }
        pub fn part<V: ::frender_dom::props::MaybeUpdateValue<str>>(
            self,
            part: V,
        ) -> ElementProps::Building<ElementProps::overwrite::part<TypeDefs, V>> {
            let __tmp_value = part;
            let ElementProps::Data {
                children,
                class,
                id,
                part,
            } = self.0;
            let _ = part;
            let part = __tmp_value;
            ElementProps::Building(ElementProps::Data {
                children,
                class,
                id,
                part,
            })
        }
    }
}
