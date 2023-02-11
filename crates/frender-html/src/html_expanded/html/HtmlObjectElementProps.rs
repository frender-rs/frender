#[allow(non_snake_case)]
#[inline(always)]
pub fn HtmlObjectElementProps() -> Building<TypesInitial> {
    #[allow(unused_imports)]
    use super::*;
    self::Building(self::Data {
        HtmlElementProps: HtmlElementProps::build(HtmlElementProps()),
        data: (),
        form: (),
        height: (),
        name: (),
        type_: (),
        use_map: (),
        width: (),
    })
}
pub mod prelude {}
pub mod overwrite {
    #![allow(non_camel_case_types)]
    pub type HtmlElementProps<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = Value,
        data = <TypeDefs as super::Types>::data,
        form = <TypeDefs as super::Types>::form,
        height = <TypeDefs as super::Types>::height,
        name = <TypeDefs as super::Types>::name,
        type_ = <TypeDefs as super::Types>::type_,
        use_map = <TypeDefs as super::Types>::use_map,
        width = <TypeDefs as super::Types>::width,
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
    pub type data<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        data = Value,
        form = <TypeDefs as super::Types>::form,
        height = <TypeDefs as super::Types>::height,
        name = <TypeDefs as super::Types>::name,
        type_ = <TypeDefs as super::Types>::type_,
        use_map = <TypeDefs as super::Types>::use_map,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type form<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        data = <TypeDefs as super::Types>::data,
        form = Value,
        height = <TypeDefs as super::Types>::height,
        name = <TypeDefs as super::Types>::name,
        type_ = <TypeDefs as super::Types>::type_,
        use_map = <TypeDefs as super::Types>::use_map,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type height<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        data = <TypeDefs as super::Types>::data,
        form = <TypeDefs as super::Types>::form,
        height = Value,
        name = <TypeDefs as super::Types>::name,
        type_ = <TypeDefs as super::Types>::type_,
        use_map = <TypeDefs as super::Types>::use_map,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type name<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        data = <TypeDefs as super::Types>::data,
        form = <TypeDefs as super::Types>::form,
        height = <TypeDefs as super::Types>::height,
        name = Value,
        type_ = <TypeDefs as super::Types>::type_,
        use_map = <TypeDefs as super::Types>::use_map,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type type_<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        data = <TypeDefs as super::Types>::data,
        form = <TypeDefs as super::Types>::form,
        height = <TypeDefs as super::Types>::height,
        name = <TypeDefs as super::Types>::name,
        type_ = Value,
        use_map = <TypeDefs as super::Types>::use_map,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type use_map<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        data = <TypeDefs as super::Types>::data,
        form = <TypeDefs as super::Types>::form,
        height = <TypeDefs as super::Types>::height,
        name = <TypeDefs as super::Types>::name,
        type_ = <TypeDefs as super::Types>::type_,
        use_map = Value,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type width<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        data = <TypeDefs as super::Types>::data,
        form = <TypeDefs as super::Types>::form,
        height = <TypeDefs as super::Types>::height,
        name = <TypeDefs as super::Types>::name,
        type_ = <TypeDefs as super::Types>::type_,
        use_map = <TypeDefs as super::Types>::use_map,
        width = Value,
    >;
}
mod trait_types {
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub trait Types {
        type HtmlElementProps: ?::core::marker::Sized + HtmlElementProps::Types;
        type data: crate::MaybeUpdateValueWithState<str>;
        type form: crate::MaybeUpdateValueWithState<str>;
        type height: crate::MaybeUpdateValueWithState<str>;
        type name: crate::MaybeUpdateValueWithState<str>;
        type type_: crate::MaybeUpdateValueWithState<str>;
        type use_map: crate::MaybeUpdateValueWithState<str>;
        type width: crate::MaybeUpdateValueWithState<str>;
    }
}
pub use trait_types::Types;
pub use trait_types::Types as ValidTypes;
pub mod data_struct {
    #[non_exhaustive]
    pub struct HtmlObjectElementProps<TypeDefs: super::Types + ?::core::marker::Sized> {
        pub HtmlElementProps: super::super::HtmlElementProps::Data<TypeDefs::HtmlElementProps>,
        pub data: TypeDefs::data,
        pub form: TypeDefs::form,
        pub height: TypeDefs::height,
        pub name: TypeDefs::name,
        pub type_: TypeDefs::type_,
        pub use_map: TypeDefs::use_map,
        pub width: TypeDefs::width,
    }
}
pub use data_struct::HtmlObjectElementProps as Data;
pub struct Building<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
pub struct Replacing<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
mod types_initial {
    #[allow(unused_imports)]
    use super::super::*;
    pub type TypesInitial = dyn super::Types<
        HtmlElementProps = HtmlElementProps::TypesInitial,
        data = (),
        form = (),
        height = (),
        name = (),
        type_ = (),
        use_map = (),
        width = (),
    >;
}
pub use types_initial::TypesInitial;
pub type DataInitial = Data<TypesInitial>;
#[cfg(feature = "dom")]
pub mod render_state {
    #[allow(non_camel_case_types)]
    pub trait RenderStateTypes {
        type HtmlElementProps: crate::props::IntrinsicComponentPollReactive;
        type data;
        type form;
        type height;
        type name;
        type type_;
        type use_map;
        type width;
    }
    pub struct RenderState<TypeDefs: RenderStateTypes>
    where
        TypeDefs: ?::core::marker::Sized,
    {
        pub HtmlElementProps: TypeDefs::HtmlElementProps,
        pub data: TypeDefs::data,
        pub form: TypeDefs::form,
        pub height: TypeDefs::height,
        pub name: TypeDefs::name,
        pub type_: TypeDefs::type_,
        pub use_map: TypeDefs::use_map,
        pub width: TypeDefs::width,
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
        pub data: &'__pin mut (TypeDefs::data),
        pub form: &'__pin mut (TypeDefs::form),
        pub height: &'__pin mut (TypeDefs::height),
        pub name: &'__pin mut (TypeDefs::name),
        pub type_: &'__pin mut (TypeDefs::type_),
        pub use_map: &'__pin mut (TypeDefs::use_map),
        pub width: &'__pin mut (TypeDefs::width),
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
            pub data: &'__pin (TypeDefs::data),
            pub form: &'__pin (TypeDefs::form),
            pub height: &'__pin (TypeDefs::height),
            pub name: &'__pin (TypeDefs::name),
            pub type_: &'__pin (TypeDefs::type_),
            pub use_map: &'__pin (TypeDefs::use_map),
            pub width: &'__pin (TypeDefs::width),
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
                        data,
                        form,
                        height,
                        name,
                        type_,
                        use_map,
                        width,
                    } = self.get_unchecked_mut();
                    RenderStateProj {
                        HtmlElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            HtmlElementProps,
                        ),
                        data: data,
                        form: form,
                        height: height,
                        name: name,
                        type_: type_,
                        use_map: use_map,
                        width: width,
                    }
                }
            }
            pub(crate) fn project_ref<'__pin>(
                self: ::pin_project_lite::__private::Pin<&'__pin Self>,
            ) -> ProjectionRef<'__pin, TypeDefs> {
                unsafe {
                    let Self {
                        HtmlElementProps,
                        data,
                        form,
                        height,
                        name,
                        type_,
                        use_map,
                        width,
                    } = self.get_ref();
                    ProjectionRef {
                        HtmlElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            HtmlElementProps,
                        ),
                        data: data,
                        form: form,
                        height: height,
                        name: name,
                        type_: type_,
                        use_map: use_map,
                        width: width,
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
            data: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::data>,
            form: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::form>,
            height: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::height>,
            name: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::name>,
            type_: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::type_>,
            use_map: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::use_map>,
            width: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::width>,
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
            let _ = &this.data;
            let _ = &this.form;
            let _ = &this.height;
            let _ = &this.name;
            let _ = &this.type_;
            let _ = &this.use_map;
            let _ = &this.width;
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
                data: self.0.data,
                form: self.0.form,
                height: self.0.height,
                name: self.0.name,
                type_: self.0.type_,
                use_map: self.0.use_map,
                width: self.0.width,
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
                data: self.0.data,
                form: self.0.form,
                height: self.0.height,
                name: self.0.name,
                type_: self.0.type_,
                use_map: self.0.use_map,
                width: self.0.width,
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
                data: self.0.data,
                form: self.0.form,
                height: self.0.height,
                name: self.0.name,
                type_: self.0.type_,
                use_map: self.0.use_map,
                width: self.0.width,
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
                data: self.0.data,
                form: self.0.form,
                height: self.0.height,
                name: self.0.name,
                type_: self.0.type_,
                use_map: self.0.use_map,
                width: self.0.width,
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
                data: self.0.data,
                form: self.0.form,
                height: self.0.height,
                name: self.0.name,
                type_: self.0.type_,
                use_map: self.0.use_map,
                width: self.0.width,
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
                data: self.0.data,
                form: self.0.form,
                height: self.0.height,
                name: self.0.name,
                type_: self.0.type_,
                use_map: self.0.use_map,
                width: self.0.width,
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
                data: self.0.data,
                form: self.0.form,
                height: self.0.height,
                name: self.0.name,
                type_: self.0.type_,
                use_map: self.0.use_map,
                width: self.0.width,
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
                data: self.0.data,
                form: self.0.form,
                height: self.0.height,
                name: self.0.name,
                type_: self.0.type_,
                use_map: self.0.use_map,
                width: self.0.width,
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
                data: self.0.data,
                form: self.0.form,
                height: self.0.height,
                name: self.0.name,
                type_: self.0.type_,
                use_map: self.0.use_map,
                width: self.0.width,
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
                data: self.0.data,
                form: self.0.form,
                height: self.0.height,
                name: self.0.name,
                type_: self.0.type_,
                use_map: self.0.use_map,
                width: self.0.width,
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
                data: self.0.data,
                form: self.0.form,
                height: self.0.height,
                name: self.0.name,
                type_: self.0.type_,
                use_map: self.0.use_map,
                width: self.0.width,
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
                data: self.0.data,
                form: self.0.form,
                height: self.0.height,
                name: self.0.name,
                type_: self.0.type_,
                use_map: self.0.use_map,
                width: self.0.width,
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
                data: self.0.data,
                form: self.0.form,
                height: self.0.height,
                name: self.0.name,
                type_: self.0.type_,
                use_map: self.0.use_map,
                width: self.0.width,
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
                data: self.0.data,
                form: self.0.form,
                height: self.0.height,
                name: self.0.name,
                type_: self.0.type_,
                use_map: self.0.use_map,
                width: self.0.width,
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
                data: self.0.data,
                form: self.0.form,
                height: self.0.height,
                name: self.0.name,
                type_: self.0.type_,
                use_map: self.0.use_map,
                width: self.0.width,
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
                data: self.0.data,
                form: self.0.form,
                height: self.0.height,
                name: self.0.name,
                type_: self.0.type_,
                use_map: self.0.use_map,
                width: self.0.width,
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
                data: self.0.data,
                form: self.0.form,
                height: self.0.height,
                name: self.0.name,
                type_: self.0.type_,
                use_map: self.0.use_map,
                width: self.0.width,
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
                data: self.0.data,
                form: self.0.form,
                height: self.0.height,
                name: self.0.name,
                type_: self.0.type_,
                use_map: self.0.use_map,
                width: self.0.width,
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
                data: self.0.data,
                form: self.0.form,
                height: self.0.height,
                name: self.0.name,
                type_: self.0.type_,
                use_map: self.0.use_map,
                width: self.0.width,
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
                data: self.0.data,
                form: self.0.form,
                height: self.0.height,
                name: self.0.name,
                type_: self.0.type_,
                use_map: self.0.use_map,
                width: self.0.width,
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
                data: self.0.data,
                form: self.0.form,
                height: self.0.height,
                name: self.0.name,
                type_: self.0.type_,
                use_map: self.0.use_map,
                width: self.0.width,
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
                data: self.0.data,
                form: self.0.form,
                height: self.0.height,
                name: self.0.name,
                type_: self.0.type_,
                use_map: self.0.use_map,
                width: self.0.width,
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
                data: self.0.data,
                form: self.0.form,
                height: self.0.height,
                name: self.0.name,
                type_: self.0.type_,
                use_map: self.0.use_map,
                width: self.0.width,
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
                data: self.0.data,
                form: self.0.form,
                height: self.0.height,
                name: self.0.name,
                type_: self.0.type_,
                use_map: self.0.use_map,
                width: self.0.width,
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
                data: self.0.data,
                form: self.0.form,
                height: self.0.height,
                name: self.0.name,
                type_: self.0.type_,
                use_map: self.0.use_map,
                width: self.0.width,
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
                data: self.0.data,
                form: self.0.form,
                height: self.0.height,
                name: self.0.name,
                type_: self.0.type_,
                use_map: self.0.use_map,
                width: self.0.width,
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
                data: self.0.data,
                form: self.0.form,
                height: self.0.height,
                name: self.0.name,
                type_: self.0.type_,
                use_map: self.0.use_map,
                width: self.0.width,
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
                data: self.0.data,
                form: self.0.form,
                height: self.0.height,
                name: self.0.name,
                type_: self.0.type_,
                use_map: self.0.use_map,
                width: self.0.width,
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
                data: self.0.data,
                form: self.0.form,
                height: self.0.height,
                name: self.0.name,
                type_: self.0.type_,
                use_map: self.0.use_map,
                width: self.0.width,
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
                data: self.0.data,
                form: self.0.form,
                height: self.0.height,
                name: self.0.name,
                type_: self.0.type_,
                use_map: self.0.use_map,
                width: self.0.width,
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
                data: self.0.data,
                form: self.0.form,
                height: self.0.height,
                name: self.0.name,
                type_: self.0.type_,
                use_map: self.0.use_map,
                width: self.0.width,
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
                data: self.0.data,
                form: self.0.form,
                height: self.0.height,
                name: self.0.name,
                type_: self.0.type_,
                use_map: self.0.use_map,
                width: self.0.width,
            })
        }
        #[inline(always)]
        pub fn data<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            data: V,
        ) -> super::Building<super::overwrite::data<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                data,
                form: self.0.form,
                height: self.0.height,
                name: self.0.name,
                type_: self.0.type_,
                use_map: self.0.use_map,
                width: self.0.width,
            })
        }
        #[inline(always)]
        pub fn form<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            form: V,
        ) -> super::Building<super::overwrite::form<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                data: self.0.data,
                form,
                height: self.0.height,
                name: self.0.name,
                type_: self.0.type_,
                use_map: self.0.use_map,
                width: self.0.width,
            })
        }
        #[inline(always)]
        pub fn height<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            height: V,
        ) -> super::Building<super::overwrite::height<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                data: self.0.data,
                form: self.0.form,
                height,
                name: self.0.name,
                type_: self.0.type_,
                use_map: self.0.use_map,
                width: self.0.width,
            })
        }
        #[inline(always)]
        pub fn name<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            name: V,
        ) -> super::Building<super::overwrite::name<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                data: self.0.data,
                form: self.0.form,
                height: self.0.height,
                name,
                type_: self.0.type_,
                use_map: self.0.use_map,
                width: self.0.width,
            })
        }
        #[inline(always)]
        pub fn type_<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            type_: V,
        ) -> super::Building<super::overwrite::type_<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                data: self.0.data,
                form: self.0.form,
                height: self.0.height,
                name: self.0.name,
                type_,
                use_map: self.0.use_map,
                width: self.0.width,
            })
        }
        #[inline(always)]
        pub fn use_map<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            use_map: V,
        ) -> super::Building<super::overwrite::use_map<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                data: self.0.data,
                form: self.0.form,
                height: self.0.height,
                name: self.0.name,
                type_: self.0.type_,
                use_map,
                width: self.0.width,
            })
        }
        #[inline(always)]
        pub fn width<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            width: V,
        ) -> super::Building<super::overwrite::width<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                data: self.0.data,
                form: self.0.form,
                height: self.0.height,
                name: self.0.name,
                type_: self.0.type_,
                use_map: self.0.use_map,
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
        crate::props::UpdateElement<web_sys::HtmlObjectElement> for super::Data<TypeDefs>
    where
        HtmlElementProps::Data<TypeDefs::HtmlElementProps>:
            crate::props::UpdateElement<web_sys::HtmlElement>,
    {
        type State = super::render_state::RenderState<
            dyn super::render_state::RenderStateTypes<
                HtmlElementProps = <HtmlElementProps::Data<
                    TypeDefs::HtmlElementProps,
                > as crate::props::UpdateElement<web_sys::HtmlElement>>::State,
                data = <TypeDefs::data as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                form = <TypeDefs::form as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                height = <TypeDefs::height as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                name = <TypeDefs::name as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                type_ = <TypeDefs::type_ as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                use_map = <TypeDefs::use_map as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                width = <TypeDefs::width as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
            >,
        >;
        fn initialize_state(
            this: Self,
            element: &web_sys::HtmlObjectElement,
            children_ctx: &mut ::frender_dom::Dom,
        ) -> Self::State {
            let dom_element: &::web_sys::Element = element.as_ref();
            super::render_state::RenderState {
                HtmlElementProps: <HtmlElementProps::Data<
                    TypeDefs::HtmlElementProps,
                > as crate::props::UpdateElement<
                    web_sys::HtmlElement,
                >>::initialize_state(this.HtmlElementProps, element, children_ctx),
                data: <TypeDefs::data as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.data,
                    |v| element.set_data(v),
                    || dom_element.remove_attribute("data").unwrap(),
                ),
                form: <TypeDefs::form as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.form,
                    |v| crate::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "form",
                    ),
                    || dom_element.remove_attribute("form").unwrap(),
                ),
                height: <TypeDefs::height as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.height,
                    |v| element.set_height(v),
                    || dom_element.remove_attribute("height").unwrap(),
                ),
                name: <TypeDefs::name as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.name,
                    |v| element.set_name(v),
                    || dom_element.remove_attribute("name").unwrap(),
                ),
                type_: <TypeDefs::type_ as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.type_,
                    |v| element.set_type(v),
                    || dom_element.remove_attribute("type").unwrap(),
                ),
                use_map: <TypeDefs::use_map as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.use_map,
                    |v| element.set_use_map(v),
                    || dom_element.remove_attribute("usemap").unwrap(),
                ),
                width: <TypeDefs::width as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.width,
                    |v| element.set_width(v),
                    || dom_element.remove_attribute("width").unwrap(),
                ),
            }
        }
        fn update_element(
            this: Self,
            element: &web_sys::HtmlObjectElement,
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
            <TypeDefs::data as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.data,
                state.data,
                |v| element.set_data(v),
                || dom_element.remove_attribute("data").unwrap(),
            );
            <TypeDefs::form as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.form,
                state.form,
                |v| crate::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "form",
                ),
                || dom_element.remove_attribute("form").unwrap(),
            );
            <TypeDefs::height as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.height,
                state.height,
                |v| element.set_height(v),
                || dom_element.remove_attribute("height").unwrap(),
            );
            <TypeDefs::name as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.name,
                state.name,
                |v| element.set_name(v),
                || dom_element.remove_attribute("name").unwrap(),
            );
            <TypeDefs::type_ as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.type_,
                state.type_,
                |v| element.set_type(v),
                || dom_element.remove_attribute("type").unwrap(),
            );
            <TypeDefs::use_map as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.use_map,
                state.use_map,
                |v| element.set_use_map(v),
                || dom_element.remove_attribute("usemap").unwrap(),
            );
            <TypeDefs::width as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.width,
                state.width,
                |v| element.set_width(v),
                || dom_element.remove_attribute("width").unwrap(),
            );
        }
    }
}
