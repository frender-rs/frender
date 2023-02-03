#[allow(non_snake_case)]
pub fn ElementProps() -> Building<TypesInitial> {
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
        type children;
        type class: crate::MaybeUpdateValue<str>;
        type id: crate::MaybeUpdateValue<str>;
        type part: crate::MaybeUpdateValue<str>;
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
    #[allow(unused_imports)]
    use super::super::*;
    impl<TypeDefs: super::Types + ?::core::marker::Sized> super::Building<TypeDefs> {
        pub fn children<V>(
            self,
            children: V,
        ) -> super::Building<super::overwrite::children<TypeDefs, V>> {
            super::Building(super::Data {
                children,
                class: self.0.class,
                id: self.0.id,
                part: self.0.part,
            })
        }
        pub fn class<V: crate::MaybeUpdateValue<str>>(
            self,
            class: V,
        ) -> super::Building<super::overwrite::class<TypeDefs, V>> {
            super::Building(super::Data {
                children: self.0.children,
                class,
                id: self.0.id,
                part: self.0.part,
            })
        }
        pub fn id<V: crate::MaybeUpdateValue<str>>(
            self,
            id: V,
        ) -> super::Building<super::overwrite::id<TypeDefs, V>> {
            super::Building(super::Data {
                children: self.0.children,
                class: self.0.class,
                id,
                part: self.0.part,
            })
        }
        pub fn part<V: crate::MaybeUpdateValue<str>>(
            self,
            part: V,
        ) -> super::Building<super::overwrite::part<TypeDefs, V>> {
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
                class = <TypeDefs::class as ::frender_dom::props::MaybeUpdateValue<str>>::State,
                id = <TypeDefs::id as ::frender_dom::props::MaybeUpdateValue<str>>::State,
                part = <TypeDefs::part as ::frender_dom::props::MaybeUpdateValue<str>>::State,
            >,
        >;
        fn update_element(
            this: Self,
            element: &web_sys::Element,
            children_ctx: &mut ::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let state = state.pin_project();
            let dom_element: &::web_sys::Element = element.as_ref();
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.children,
                state: state.children,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    state,
                    children_ctx,
                    ..
                } => ::frender_core::UpdateRenderState::update_render_state(
                    data,
                    children_ctx,
                    state,
                ),
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.class,
                state: state.class,
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
                    const ATTR_NAME: &::core::primitive::str = "class";
                    < TypeDefs :: class as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.id,
                state: state.id,
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
                    const ATTR_NAME: &::core::primitive::str = "id";
                    < TypeDefs :: id as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | element . set_id (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.part,
                state: state.part,
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
                    const ATTR_NAME: &::core::primitive::str = "part";
                    < TypeDefs :: part as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
        }
    }
}
