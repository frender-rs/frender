#[allow(non_snake_case)]
#[inline(always)]
pub const fn ElementProps() -> Building<TypesInitial> {
    #[allow(unused_imports)]
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
        type children: ~const ::core::marker::Destruct;
        type class: crate::MaybeUpdateValueByRef<str> + ~const ::core::marker::Destruct;
        type id: crate::MaybeUpdateValueByRef<str> + ~const ::core::marker::Destruct;
        type part: crate::MaybeUpdateValueByRef<str> + ~const ::core::marker::Destruct;
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
    }
    pub struct RenderState<TypeDefs: RenderStateTypes>
    where
        TypeDefs: ?::core::marker::Sized,
    {
        pub children: TypeDefs::children,
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
        }
        impl<TypeDefs: RenderStateTypes> RenderState<TypeDefs>
        where
            TypeDefs: ?::core::marker::Sized,
        {
            pub(crate) fn project<'__pin>(
                self: ::pin_project_lite::__private::Pin<&'__pin mut Self>,
            ) -> RenderStateProj<'__pin, TypeDefs> {
                unsafe {
                    let Self { children } = self.get_unchecked_mut();
                    RenderStateProj {
                        children: ::pin_project_lite::__private::Pin::new_unchecked(children),
                    }
                }
            }
            pub(crate) fn project_ref<'__pin>(
                self: ::pin_project_lite::__private::Pin<&'__pin Self>,
            ) -> ProjectionRef<'__pin, TypeDefs> {
                unsafe {
                    let Self { children } = self.get_ref();
                    ProjectionRef {
                        children: ::pin_project_lite::__private::Pin::new_unchecked(children),
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
            crate::__private::RenderState::poll_reactive(self.project().children, cx)
        }
    }
}
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
        #[inline(always)]
        pub const fn children<V>(
            self,
            children: V,
        ) -> super::Building<super::overwrite::children<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
            super::Building(super::Data {
                children,
                class: self.0.class,
                id: self.0.id,
                part: self.0.part,
            })
        }
        #[inline(always)]
        pub const fn class<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            class: V,
        ) -> super::Building<super::overwrite::class<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
            super::Building(super::Data {
                children: self.0.children,
                class,
                id: self.0.id,
                part: self.0.part,
            })
        }
        #[inline(always)]
        pub const fn id<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            id: V,
        ) -> super::Building<super::overwrite::id<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
            super::Building(super::Data {
                children: self.0.children,
                class: self.0.class,
                id,
                part: self.0.part,
            })
        }
        #[inline(always)]
        pub const fn part<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            part: V,
        ) -> super::Building<super::overwrite::part<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
            super::Building(super::Data {
                children: self.0.children,
                class: self.0.class,
                id: self.0.id,
                part,
            })
        }
    }
}
#[cfg(feature = "dom")]
mod impl_update_element {
    #[allow(unused_imports)]
    use super::super::*;
    impl<TypeDefs: ?::core::marker::Sized + super::Types>
        crate::props::UpdateElement<web_sys::Element> for super::Data<TypeDefs>
    where
        TypeDefs::children: ::frender_core::UpdateRenderState<::frender_dom::Dom>,
    {
        type State = super::render_state::RenderState<
            dyn super::render_state::RenderStateTypes<
                children = <TypeDefs::children as frender_core::UpdateRenderState<
                    frender_dom::Dom,
                >>::State,
            >,
        >;
        fn initialize_state(
            this: Self,
            element: &web_sys::Element,
            children_ctx: &mut ::frender_dom::Dom,
        ) -> Self::State {
            let dom_element: &::web_sys::Element = element.as_ref();
            < TypeDefs :: class as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . class , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , "class" ,) , | | dom_element . remove_attribute ("class") . unwrap () ,) ;
            < TypeDefs :: id as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . id , | v | element . set_id (v) , | | dom_element . remove_attribute ("id") . unwrap () ,) ;
            < TypeDefs :: part as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . part , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , "part" ,) , | | dom_element . remove_attribute ("part") . unwrap () ,) ;
            super::render_state::RenderState {
                children: ::frender_core::UpdateRenderState::initialize_render_state(
                    this.children,
                    children_ctx,
                ),
            }
        }
        fn update_element(
            this: Self,
            element: &web_sys::Element,
            children_ctx: &mut ::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let state = state.pin_project();
            let dom_element: &::web_sys::Element = element.as_ref();
            ::frender_core::UpdateRenderState::update_render_state(
                this.children,
                children_ctx,
                state.children,
            );
            < TypeDefs :: class as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . class , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , "class" ,) , | | dom_element . remove_attribute ("class") . unwrap () ,) ;
            < TypeDefs :: id as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . id , | v | element . set_id (v) , | | dom_element . remove_attribute ("id") . unwrap () ,) ;
            < TypeDefs :: part as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . part , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , "part" ,) , | | dom_element . remove_attribute ("part") . unwrap () ,) ;
        }
    }
}
