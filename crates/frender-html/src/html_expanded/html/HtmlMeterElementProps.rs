#[allow(non_snake_case)]
#[inline(always)]
pub fn HtmlMeterElementProps() -> Building<TypesInitial> {
    #[allow(unused_imports)]
    use super::*;
    self::Building(self::Data {
        HtmlElementProps: HtmlElementProps::build(HtmlElementProps()),
        value: (),
        min: (),
        max: (),
        low: (),
        high: (),
        optimum: (),
    })
}
pub mod prelude {}
pub mod overwrite {
    #![allow(non_camel_case_types)]
    pub type HtmlElementProps<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = Value,
        value = <TypeDefs as super::Types>::value,
        min = <TypeDefs as super::Types>::min,
        max = <TypeDefs as super::Types>::max,
        low = <TypeDefs as super::Types>::low,
        high = <TypeDefs as super::Types>::high,
        optimum = <TypeDefs as super::Types>::optimum,
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
    pub type value<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        value = Value,
        min = <TypeDefs as super::Types>::min,
        max = <TypeDefs as super::Types>::max,
        low = <TypeDefs as super::Types>::low,
        high = <TypeDefs as super::Types>::high,
        optimum = <TypeDefs as super::Types>::optimum,
    >;
    pub type min<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        value = <TypeDefs as super::Types>::value,
        min = Value,
        max = <TypeDefs as super::Types>::max,
        low = <TypeDefs as super::Types>::low,
        high = <TypeDefs as super::Types>::high,
        optimum = <TypeDefs as super::Types>::optimum,
    >;
    pub type max<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        value = <TypeDefs as super::Types>::value,
        min = <TypeDefs as super::Types>::min,
        max = Value,
        low = <TypeDefs as super::Types>::low,
        high = <TypeDefs as super::Types>::high,
        optimum = <TypeDefs as super::Types>::optimum,
    >;
    pub type low<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        value = <TypeDefs as super::Types>::value,
        min = <TypeDefs as super::Types>::min,
        max = <TypeDefs as super::Types>::max,
        low = Value,
        high = <TypeDefs as super::Types>::high,
        optimum = <TypeDefs as super::Types>::optimum,
    >;
    pub type high<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        value = <TypeDefs as super::Types>::value,
        min = <TypeDefs as super::Types>::min,
        max = <TypeDefs as super::Types>::max,
        low = <TypeDefs as super::Types>::low,
        high = Value,
        optimum = <TypeDefs as super::Types>::optimum,
    >;
    pub type optimum<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        value = <TypeDefs as super::Types>::value,
        min = <TypeDefs as super::Types>::min,
        max = <TypeDefs as super::Types>::max,
        low = <TypeDefs as super::Types>::low,
        high = <TypeDefs as super::Types>::high,
        optimum = Value,
    >;
}
mod trait_types {
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub trait Types {
        type HtmlElementProps: ?::core::marker::Sized + HtmlElementProps::Types;
        type value: crate::MaybeUpdateValueWithState<f64>;
        type min: crate::MaybeUpdateValueWithState<f64>;
        type max: crate::MaybeUpdateValueWithState<f64>;
        type low: crate::MaybeUpdateValueWithState<f64>;
        type high: crate::MaybeUpdateValueWithState<f64>;
        type optimum: crate::MaybeUpdateValueWithState<f64>;
    }
}
pub use trait_types::Types;
pub use trait_types::Types as ValidTypes;
pub mod data_struct {
    #[non_exhaustive]
    pub struct HtmlMeterElementProps<TypeDefs: super::Types + ?::core::marker::Sized> {
        pub HtmlElementProps: super::super::HtmlElementProps::Data<TypeDefs::HtmlElementProps>,
        pub value: TypeDefs::value,
        pub min: TypeDefs::min,
        pub max: TypeDefs::max,
        pub low: TypeDefs::low,
        pub high: TypeDefs::high,
        pub optimum: TypeDefs::optimum,
    }
}
pub use data_struct::HtmlMeterElementProps as Data;
pub struct Building<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
pub struct Replacing<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
mod types_initial {
    #[allow(unused_imports)]
    use super::super::*;
    pub type TypesInitial = dyn super::Types<
        HtmlElementProps = HtmlElementProps::TypesInitial,
        value = (),
        min = (),
        max = (),
        low = (),
        high = (),
        optimum = (),
    >;
}
pub use types_initial::TypesInitial;
pub type DataInitial = Data<TypesInitial>;
#[cfg(feature = "dom")]
pub mod render_state {
    #[allow(non_camel_case_types)]
    pub trait RenderStateTypes {
        type HtmlElementProps: crate::props::IntrinsicComponentPollReactive;
        type value;
        type min;
        type max;
        type low;
        type high;
        type optimum;
    }
    pub struct RenderState<TypeDefs: RenderStateTypes>
    where
        TypeDefs: ?::core::marker::Sized,
    {
        pub HtmlElementProps: TypeDefs::HtmlElementProps,
        pub value: TypeDefs::value,
        pub min: TypeDefs::min,
        pub max: TypeDefs::max,
        pub low: TypeDefs::low,
        pub high: TypeDefs::high,
        pub optimum: TypeDefs::optimum,
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
        pub value: &'__pin mut (TypeDefs::value),
        pub min: &'__pin mut (TypeDefs::min),
        pub max: &'__pin mut (TypeDefs::max),
        pub low: &'__pin mut (TypeDefs::low),
        pub high: &'__pin mut (TypeDefs::high),
        pub optimum: &'__pin mut (TypeDefs::optimum),
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
            pub value: &'__pin (TypeDefs::value),
            pub min: &'__pin (TypeDefs::min),
            pub max: &'__pin (TypeDefs::max),
            pub low: &'__pin (TypeDefs::low),
            pub high: &'__pin (TypeDefs::high),
            pub optimum: &'__pin (TypeDefs::optimum),
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
                        value,
                        min,
                        max,
                        low,
                        high,
                        optimum,
                    } = self.get_unchecked_mut();
                    RenderStateProj {
                        HtmlElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            HtmlElementProps,
                        ),
                        value: value,
                        min: min,
                        max: max,
                        low: low,
                        high: high,
                        optimum: optimum,
                    }
                }
            }
            pub(crate) fn project_ref<'__pin>(
                self: ::pin_project_lite::__private::Pin<&'__pin Self>,
            ) -> ProjectionRef<'__pin, TypeDefs> {
                unsafe {
                    let Self {
                        HtmlElementProps,
                        value,
                        min,
                        max,
                        low,
                        high,
                        optimum,
                    } = self.get_ref();
                    ProjectionRef {
                        HtmlElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            HtmlElementProps,
                        ),
                        value: value,
                        min: min,
                        max: max,
                        low: low,
                        high: high,
                        optimum: optimum,
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
            value: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::value>,
            min: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::min>,
            max: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::max>,
            low: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::low>,
            high: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::high>,
            optimum: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::optimum>,
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
            let _ = &this.value;
            let _ = &this.min;
            let _ = &this.max;
            let _ = &this.low;
            let _ = &this.high;
            let _ = &this.optimum;
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
                value: self.0.value,
                min: self.0.min,
                max: self.0.max,
                low: self.0.low,
                high: self.0.high,
                optimum: self.0.optimum,
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
                value: self.0.value,
                min: self.0.min,
                max: self.0.max,
                low: self.0.low,
                high: self.0.high,
                optimum: self.0.optimum,
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
                value: self.0.value,
                min: self.0.min,
                max: self.0.max,
                low: self.0.low,
                high: self.0.high,
                optimum: self.0.optimum,
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
                value: self.0.value,
                min: self.0.min,
                max: self.0.max,
                low: self.0.low,
                high: self.0.high,
                optimum: self.0.optimum,
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
                value: self.0.value,
                min: self.0.min,
                max: self.0.max,
                low: self.0.low,
                high: self.0.high,
                optimum: self.0.optimum,
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
                value: self.0.value,
                min: self.0.min,
                max: self.0.max,
                low: self.0.low,
                high: self.0.high,
                optimum: self.0.optimum,
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
                value: self.0.value,
                min: self.0.min,
                max: self.0.max,
                low: self.0.low,
                high: self.0.high,
                optimum: self.0.optimum,
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
                value: self.0.value,
                min: self.0.min,
                max: self.0.max,
                low: self.0.low,
                high: self.0.high,
                optimum: self.0.optimum,
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
                value: self.0.value,
                min: self.0.min,
                max: self.0.max,
                low: self.0.low,
                high: self.0.high,
                optimum: self.0.optimum,
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
                value: self.0.value,
                min: self.0.min,
                max: self.0.max,
                low: self.0.low,
                high: self.0.high,
                optimum: self.0.optimum,
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
                value: self.0.value,
                min: self.0.min,
                max: self.0.max,
                low: self.0.low,
                high: self.0.high,
                optimum: self.0.optimum,
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
                value: self.0.value,
                min: self.0.min,
                max: self.0.max,
                low: self.0.low,
                high: self.0.high,
                optimum: self.0.optimum,
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
                value: self.0.value,
                min: self.0.min,
                max: self.0.max,
                low: self.0.low,
                high: self.0.high,
                optimum: self.0.optimum,
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
                value: self.0.value,
                min: self.0.min,
                max: self.0.max,
                low: self.0.low,
                high: self.0.high,
                optimum: self.0.optimum,
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
                value: self.0.value,
                min: self.0.min,
                max: self.0.max,
                low: self.0.low,
                high: self.0.high,
                optimum: self.0.optimum,
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
                value: self.0.value,
                min: self.0.min,
                max: self.0.max,
                low: self.0.low,
                high: self.0.high,
                optimum: self.0.optimum,
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
                value: self.0.value,
                min: self.0.min,
                max: self.0.max,
                low: self.0.low,
                high: self.0.high,
                optimum: self.0.optimum,
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
                value: self.0.value,
                min: self.0.min,
                max: self.0.max,
                low: self.0.low,
                high: self.0.high,
                optimum: self.0.optimum,
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
                value: self.0.value,
                min: self.0.min,
                max: self.0.max,
                low: self.0.low,
                high: self.0.high,
                optimum: self.0.optimum,
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
                value: self.0.value,
                min: self.0.min,
                max: self.0.max,
                low: self.0.low,
                high: self.0.high,
                optimum: self.0.optimum,
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
                value: self.0.value,
                min: self.0.min,
                max: self.0.max,
                low: self.0.low,
                high: self.0.high,
                optimum: self.0.optimum,
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
                value: self.0.value,
                min: self.0.min,
                max: self.0.max,
                low: self.0.low,
                high: self.0.high,
                optimum: self.0.optimum,
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
                value: self.0.value,
                min: self.0.min,
                max: self.0.max,
                low: self.0.low,
                high: self.0.high,
                optimum: self.0.optimum,
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
                value: self.0.value,
                min: self.0.min,
                max: self.0.max,
                low: self.0.low,
                high: self.0.high,
                optimum: self.0.optimum,
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
                value: self.0.value,
                min: self.0.min,
                max: self.0.max,
                low: self.0.low,
                high: self.0.high,
                optimum: self.0.optimum,
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
                value: self.0.value,
                min: self.0.min,
                max: self.0.max,
                low: self.0.low,
                high: self.0.high,
                optimum: self.0.optimum,
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
                value: self.0.value,
                min: self.0.min,
                max: self.0.max,
                low: self.0.low,
                high: self.0.high,
                optimum: self.0.optimum,
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
                value: self.0.value,
                min: self.0.min,
                max: self.0.max,
                low: self.0.low,
                high: self.0.high,
                optimum: self.0.optimum,
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
                value: self.0.value,
                min: self.0.min,
                max: self.0.max,
                low: self.0.low,
                high: self.0.high,
                optimum: self.0.optimum,
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
                value: self.0.value,
                min: self.0.min,
                max: self.0.max,
                low: self.0.low,
                high: self.0.high,
                optimum: self.0.optimum,
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
                value: self.0.value,
                min: self.0.min,
                max: self.0.max,
                low: self.0.low,
                high: self.0.high,
                optimum: self.0.optimum,
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
                value: self.0.value,
                min: self.0.min,
                max: self.0.max,
                low: self.0.low,
                high: self.0.high,
                optimum: self.0.optimum,
            })
        }
        #[inline(always)]
        pub fn value<V: crate::MaybeUpdateValueWithState<f64>>(
            self,
            value: V,
        ) -> super::Building<super::overwrite::value<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                value,
                min: self.0.min,
                max: self.0.max,
                low: self.0.low,
                high: self.0.high,
                optimum: self.0.optimum,
            })
        }
        #[inline(always)]
        pub fn min<V: crate::MaybeUpdateValueWithState<f64>>(
            self,
            min: V,
        ) -> super::Building<super::overwrite::min<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                value: self.0.value,
                min,
                max: self.0.max,
                low: self.0.low,
                high: self.0.high,
                optimum: self.0.optimum,
            })
        }
        #[inline(always)]
        pub fn max<V: crate::MaybeUpdateValueWithState<f64>>(
            self,
            max: V,
        ) -> super::Building<super::overwrite::max<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                value: self.0.value,
                min: self.0.min,
                max,
                low: self.0.low,
                high: self.0.high,
                optimum: self.0.optimum,
            })
        }
        #[inline(always)]
        pub fn low<V: crate::MaybeUpdateValueWithState<f64>>(
            self,
            low: V,
        ) -> super::Building<super::overwrite::low<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                value: self.0.value,
                min: self.0.min,
                max: self.0.max,
                low,
                high: self.0.high,
                optimum: self.0.optimum,
            })
        }
        #[inline(always)]
        pub fn high<V: crate::MaybeUpdateValueWithState<f64>>(
            self,
            high: V,
        ) -> super::Building<super::overwrite::high<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                value: self.0.value,
                min: self.0.min,
                max: self.0.max,
                low: self.0.low,
                high,
                optimum: self.0.optimum,
            })
        }
        #[inline(always)]
        pub fn optimum<V: crate::MaybeUpdateValueWithState<f64>>(
            self,
            optimum: V,
        ) -> super::Building<super::overwrite::optimum<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                value: self.0.value,
                min: self.0.min,
                max: self.0.max,
                low: self.0.low,
                high: self.0.high,
                optimum,
            })
        }
    }
}
#[cfg(feature = "dom")]
mod impl_update_element {
    #[allow(unused_imports)]
    use super::super::*;
    impl<TypeDefs: ?::core::marker::Sized + super::Types>
        crate::props::UpdateElement<web_sys::HtmlMeterElement> for super::Data<TypeDefs>
    where
        HtmlElementProps::Data<TypeDefs::HtmlElementProps>:
            crate::props::UpdateElement<web_sys::HtmlElement>,
    {
        type State = super::render_state::RenderState<
            dyn super::render_state::RenderStateTypes<
                HtmlElementProps = <HtmlElementProps::Data<
                    TypeDefs::HtmlElementProps,
                > as crate::props::UpdateElement<web_sys::HtmlElement>>::State,
                value = <TypeDefs::value as ::frender_dom::props::MaybeUpdateValueWithState<
                    f64,
                >>::State,
                min = <TypeDefs::min as ::frender_dom::props::MaybeUpdateValueWithState<
                    f64,
                >>::State,
                max = <TypeDefs::max as ::frender_dom::props::MaybeUpdateValueWithState<
                    f64,
                >>::State,
                low = <TypeDefs::low as ::frender_dom::props::MaybeUpdateValueWithState<
                    f64,
                >>::State,
                high = <TypeDefs::high as ::frender_dom::props::MaybeUpdateValueWithState<
                    f64,
                >>::State,
                optimum = <TypeDefs::optimum as ::frender_dom::props::MaybeUpdateValueWithState<
                    f64,
                >>::State,
            >,
        >;
        fn initialize_state(
            this: Self,
            element: &web_sys::HtmlMeterElement,
            children_ctx: &mut ::frender_dom::Dom,
        ) -> Self::State {
            let dom_element: &::web_sys::Element = element.as_ref();
            super::render_state::RenderState {
                HtmlElementProps: <HtmlElementProps::Data<
                    TypeDefs::HtmlElementProps,
                > as crate::props::UpdateElement<
                    web_sys::HtmlElement,
                >>::initialize_state(this.HtmlElementProps, element, children_ctx),
                value: <TypeDefs::value as ::frender_dom::props::MaybeUpdateValueWithState<
                    f64,
                >>::initialize_state_and_update(
                    this.value,
                    |v| element.set_value(*v),
                    || dom_element.remove_attribute("value").unwrap(),
                ),
                min: <TypeDefs::min as ::frender_dom::props::MaybeUpdateValueWithState<
                    f64,
                >>::initialize_state_and_update(
                    this.min,
                    |v| element.set_min(*v),
                    || dom_element.remove_attribute("min").unwrap(),
                ),
                max: <TypeDefs::max as ::frender_dom::props::MaybeUpdateValueWithState<
                    f64,
                >>::initialize_state_and_update(
                    this.max,
                    |v| element.set_max(*v),
                    || dom_element.remove_attribute("max").unwrap(),
                ),
                low: <TypeDefs::low as ::frender_dom::props::MaybeUpdateValueWithState<
                    f64,
                >>::initialize_state_and_update(
                    this.low,
                    |v| element.set_low(*v),
                    || dom_element.remove_attribute("low").unwrap(),
                ),
                high: <TypeDefs::high as ::frender_dom::props::MaybeUpdateValueWithState<
                    f64,
                >>::initialize_state_and_update(
                    this.high,
                    |v| element.set_high(*v),
                    || dom_element.remove_attribute("high").unwrap(),
                ),
                optimum: <TypeDefs::optimum as ::frender_dom::props::MaybeUpdateValueWithState<
                    f64,
                >>::initialize_state_and_update(
                    this.optimum,
                    |v| element.set_optimum(*v),
                    || dom_element.remove_attribute("optimum").unwrap(),
                ),
            }
        }
        fn update_element(
            this: Self,
            element: &web_sys::HtmlMeterElement,
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
            <TypeDefs::value as ::frender_dom::props::MaybeUpdateValueWithState<
                f64,
            >>::maybe_update_value_with_state(
                this.value,
                state.value,
                |v| element.set_value(*v),
                || dom_element.remove_attribute("value").unwrap(),
            );
            <TypeDefs::min as ::frender_dom::props::MaybeUpdateValueWithState<
                f64,
            >>::maybe_update_value_with_state(
                this.min,
                state.min,
                |v| element.set_min(*v),
                || dom_element.remove_attribute("min").unwrap(),
            );
            <TypeDefs::max as ::frender_dom::props::MaybeUpdateValueWithState<
                f64,
            >>::maybe_update_value_with_state(
                this.max,
                state.max,
                |v| element.set_max(*v),
                || dom_element.remove_attribute("max").unwrap(),
            );
            <TypeDefs::low as ::frender_dom::props::MaybeUpdateValueWithState<
                f64,
            >>::maybe_update_value_with_state(
                this.low,
                state.low,
                |v| element.set_low(*v),
                || dom_element.remove_attribute("low").unwrap(),
            );
            <TypeDefs::high as ::frender_dom::props::MaybeUpdateValueWithState<
                f64,
            >>::maybe_update_value_with_state(
                this.high,
                state.high,
                |v| element.set_high(*v),
                || dom_element.remove_attribute("high").unwrap(),
            );
            <TypeDefs::optimum as ::frender_dom::props::MaybeUpdateValueWithState<
                f64,
            >>::maybe_update_value_with_state(
                this.optimum,
                state.optimum,
                |v| element.set_optimum(*v),
                || dom_element.remove_attribute("optimum").unwrap(),
            );
        }
    }
}
