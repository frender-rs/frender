#[allow(non_snake_case)]
#[inline(always)]
pub fn HtmlTableElementProps() -> Building<TypesInitial> {
    #[allow(unused_imports)]
    use super::*;
    self::Building(self::Data {
        HtmlElementProps: HtmlElementProps::build(HtmlElementProps()),
        align: (),
        bg_color: (),
        border: (),
        cell_padding: (),
        cell_spacing: (),
        frame: (),
        rules: (),
        summary: (),
        width: (),
    })
}
pub mod prelude {}
pub mod overwrite {
    #![allow(non_camel_case_types)]
    pub type HtmlElementProps<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = Value,
        align = <TypeDefs as super::Types>::align,
        bg_color = <TypeDefs as super::Types>::bg_color,
        border = <TypeDefs as super::Types>::border,
        cell_padding = <TypeDefs as super::Types>::cell_padding,
        cell_spacing = <TypeDefs as super::Types>::cell_spacing,
        frame = <TypeDefs as super::Types>::frame,
        rules = <TypeDefs as super::Types>::rules,
        summary = <TypeDefs as super::Types>::summary,
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
    pub type align<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        align = Value,
        bg_color = <TypeDefs as super::Types>::bg_color,
        border = <TypeDefs as super::Types>::border,
        cell_padding = <TypeDefs as super::Types>::cell_padding,
        cell_spacing = <TypeDefs as super::Types>::cell_spacing,
        frame = <TypeDefs as super::Types>::frame,
        rules = <TypeDefs as super::Types>::rules,
        summary = <TypeDefs as super::Types>::summary,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type bg_color<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        align = <TypeDefs as super::Types>::align,
        bg_color = Value,
        border = <TypeDefs as super::Types>::border,
        cell_padding = <TypeDefs as super::Types>::cell_padding,
        cell_spacing = <TypeDefs as super::Types>::cell_spacing,
        frame = <TypeDefs as super::Types>::frame,
        rules = <TypeDefs as super::Types>::rules,
        summary = <TypeDefs as super::Types>::summary,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type border<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        align = <TypeDefs as super::Types>::align,
        bg_color = <TypeDefs as super::Types>::bg_color,
        border = Value,
        cell_padding = <TypeDefs as super::Types>::cell_padding,
        cell_spacing = <TypeDefs as super::Types>::cell_spacing,
        frame = <TypeDefs as super::Types>::frame,
        rules = <TypeDefs as super::Types>::rules,
        summary = <TypeDefs as super::Types>::summary,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type cell_padding<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        align = <TypeDefs as super::Types>::align,
        bg_color = <TypeDefs as super::Types>::bg_color,
        border = <TypeDefs as super::Types>::border,
        cell_padding = Value,
        cell_spacing = <TypeDefs as super::Types>::cell_spacing,
        frame = <TypeDefs as super::Types>::frame,
        rules = <TypeDefs as super::Types>::rules,
        summary = <TypeDefs as super::Types>::summary,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type cell_spacing<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        align = <TypeDefs as super::Types>::align,
        bg_color = <TypeDefs as super::Types>::bg_color,
        border = <TypeDefs as super::Types>::border,
        cell_padding = <TypeDefs as super::Types>::cell_padding,
        cell_spacing = Value,
        frame = <TypeDefs as super::Types>::frame,
        rules = <TypeDefs as super::Types>::rules,
        summary = <TypeDefs as super::Types>::summary,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type frame<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        align = <TypeDefs as super::Types>::align,
        bg_color = <TypeDefs as super::Types>::bg_color,
        border = <TypeDefs as super::Types>::border,
        cell_padding = <TypeDefs as super::Types>::cell_padding,
        cell_spacing = <TypeDefs as super::Types>::cell_spacing,
        frame = Value,
        rules = <TypeDefs as super::Types>::rules,
        summary = <TypeDefs as super::Types>::summary,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type rules<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        align = <TypeDefs as super::Types>::align,
        bg_color = <TypeDefs as super::Types>::bg_color,
        border = <TypeDefs as super::Types>::border,
        cell_padding = <TypeDefs as super::Types>::cell_padding,
        cell_spacing = <TypeDefs as super::Types>::cell_spacing,
        frame = <TypeDefs as super::Types>::frame,
        rules = Value,
        summary = <TypeDefs as super::Types>::summary,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type summary<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        align = <TypeDefs as super::Types>::align,
        bg_color = <TypeDefs as super::Types>::bg_color,
        border = <TypeDefs as super::Types>::border,
        cell_padding = <TypeDefs as super::Types>::cell_padding,
        cell_spacing = <TypeDefs as super::Types>::cell_spacing,
        frame = <TypeDefs as super::Types>::frame,
        rules = <TypeDefs as super::Types>::rules,
        summary = Value,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type width<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        align = <TypeDefs as super::Types>::align,
        bg_color = <TypeDefs as super::Types>::bg_color,
        border = <TypeDefs as super::Types>::border,
        cell_padding = <TypeDefs as super::Types>::cell_padding,
        cell_spacing = <TypeDefs as super::Types>::cell_spacing,
        frame = <TypeDefs as super::Types>::frame,
        rules = <TypeDefs as super::Types>::rules,
        summary = <TypeDefs as super::Types>::summary,
        width = Value,
    >;
}
mod trait_types {
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub trait Types {
        type HtmlElementProps: ?::core::marker::Sized + HtmlElementProps::Types;
        type align: crate::MaybeUpdateValueWithState<str>;
        type bg_color: crate::MaybeUpdateValueWithState<str>;
        type border: crate::MaybeUpdateValueWithState<str>;
        type cell_padding: crate::MaybeUpdateValueWithState<str>;
        type cell_spacing: crate::MaybeUpdateValueWithState<str>;
        type frame: crate::MaybeUpdateValueWithState<str>;
        type rules: crate::MaybeUpdateValueWithState<str>;
        type summary: crate::MaybeUpdateValueWithState<str>;
        type width: crate::MaybeUpdateValueWithState<str>;
    }
}
pub use trait_types::Types;
pub use trait_types::Types as ValidTypes;
pub mod data_struct {
    #[non_exhaustive]
    pub struct HtmlTableElementProps<TypeDefs: super::Types + ?::core::marker::Sized> {
        pub HtmlElementProps: super::super::HtmlElementProps::Data<TypeDefs::HtmlElementProps>,
        pub align: TypeDefs::align,
        pub bg_color: TypeDefs::bg_color,
        pub border: TypeDefs::border,
        pub cell_padding: TypeDefs::cell_padding,
        pub cell_spacing: TypeDefs::cell_spacing,
        pub frame: TypeDefs::frame,
        pub rules: TypeDefs::rules,
        pub summary: TypeDefs::summary,
        pub width: TypeDefs::width,
    }
}
pub use data_struct::HtmlTableElementProps as Data;
pub struct Building<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
pub struct Replacing<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
mod types_initial {
    #[allow(unused_imports)]
    use super::super::*;
    pub type TypesInitial = dyn super::Types<
        HtmlElementProps = HtmlElementProps::TypesInitial,
        align = (),
        bg_color = (),
        border = (),
        cell_padding = (),
        cell_spacing = (),
        frame = (),
        rules = (),
        summary = (),
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
        type align;
        type bg_color;
        type border;
        type cell_padding;
        type cell_spacing;
        type frame;
        type rules;
        type summary;
        type width;
    }
    pub struct RenderState<TypeDefs: RenderStateTypes>
    where
        TypeDefs: ?::core::marker::Sized,
    {
        pub HtmlElementProps: TypeDefs::HtmlElementProps,
        pub align: TypeDefs::align,
        pub bg_color: TypeDefs::bg_color,
        pub border: TypeDefs::border,
        pub cell_padding: TypeDefs::cell_padding,
        pub cell_spacing: TypeDefs::cell_spacing,
        pub frame: TypeDefs::frame,
        pub rules: TypeDefs::rules,
        pub summary: TypeDefs::summary,
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
        pub align: &'__pin mut (TypeDefs::align),
        pub bg_color: &'__pin mut (TypeDefs::bg_color),
        pub border: &'__pin mut (TypeDefs::border),
        pub cell_padding: &'__pin mut (TypeDefs::cell_padding),
        pub cell_spacing: &'__pin mut (TypeDefs::cell_spacing),
        pub frame: &'__pin mut (TypeDefs::frame),
        pub rules: &'__pin mut (TypeDefs::rules),
        pub summary: &'__pin mut (TypeDefs::summary),
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
            pub align: &'__pin (TypeDefs::align),
            pub bg_color: &'__pin (TypeDefs::bg_color),
            pub border: &'__pin (TypeDefs::border),
            pub cell_padding: &'__pin (TypeDefs::cell_padding),
            pub cell_spacing: &'__pin (TypeDefs::cell_spacing),
            pub frame: &'__pin (TypeDefs::frame),
            pub rules: &'__pin (TypeDefs::rules),
            pub summary: &'__pin (TypeDefs::summary),
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
                        align,
                        bg_color,
                        border,
                        cell_padding,
                        cell_spacing,
                        frame,
                        rules,
                        summary,
                        width,
                    } = self.get_unchecked_mut();
                    RenderStateProj {
                        HtmlElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            HtmlElementProps,
                        ),
                        align: align,
                        bg_color: bg_color,
                        border: border,
                        cell_padding: cell_padding,
                        cell_spacing: cell_spacing,
                        frame: frame,
                        rules: rules,
                        summary: summary,
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
                        align,
                        bg_color,
                        border,
                        cell_padding,
                        cell_spacing,
                        frame,
                        rules,
                        summary,
                        width,
                    } = self.get_ref();
                    ProjectionRef {
                        HtmlElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            HtmlElementProps,
                        ),
                        align: align,
                        bg_color: bg_color,
                        border: border,
                        cell_padding: cell_padding,
                        cell_spacing: cell_spacing,
                        frame: frame,
                        rules: rules,
                        summary: summary,
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
            align: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::align>,
            bg_color: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::bg_color>,
            border: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::border>,
            cell_padding: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::cell_padding>,
            cell_spacing: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::cell_spacing>,
            frame: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::frame>,
            rules: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::rules>,
            summary: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::summary>,
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
            let _ = &this.align;
            let _ = &this.bg_color;
            let _ = &this.border;
            let _ = &this.cell_padding;
            let _ = &this.cell_spacing;
            let _ = &this.frame;
            let _ = &this.rules;
            let _ = &this.summary;
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
                align: self.0.align,
                bg_color: self.0.bg_color,
                border: self.0.border,
                cell_padding: self.0.cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame: self.0.frame,
                rules: self.0.rules,
                summary: self.0.summary,
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
                align: self.0.align,
                bg_color: self.0.bg_color,
                border: self.0.border,
                cell_padding: self.0.cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame: self.0.frame,
                rules: self.0.rules,
                summary: self.0.summary,
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
                align: self.0.align,
                bg_color: self.0.bg_color,
                border: self.0.border,
                cell_padding: self.0.cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame: self.0.frame,
                rules: self.0.rules,
                summary: self.0.summary,
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
                align: self.0.align,
                bg_color: self.0.bg_color,
                border: self.0.border,
                cell_padding: self.0.cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame: self.0.frame,
                rules: self.0.rules,
                summary: self.0.summary,
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
                align: self.0.align,
                bg_color: self.0.bg_color,
                border: self.0.border,
                cell_padding: self.0.cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame: self.0.frame,
                rules: self.0.rules,
                summary: self.0.summary,
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
                align: self.0.align,
                bg_color: self.0.bg_color,
                border: self.0.border,
                cell_padding: self.0.cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame: self.0.frame,
                rules: self.0.rules,
                summary: self.0.summary,
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
                align: self.0.align,
                bg_color: self.0.bg_color,
                border: self.0.border,
                cell_padding: self.0.cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame: self.0.frame,
                rules: self.0.rules,
                summary: self.0.summary,
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
                align: self.0.align,
                bg_color: self.0.bg_color,
                border: self.0.border,
                cell_padding: self.0.cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame: self.0.frame,
                rules: self.0.rules,
                summary: self.0.summary,
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
                align: self.0.align,
                bg_color: self.0.bg_color,
                border: self.0.border,
                cell_padding: self.0.cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame: self.0.frame,
                rules: self.0.rules,
                summary: self.0.summary,
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
                align: self.0.align,
                bg_color: self.0.bg_color,
                border: self.0.border,
                cell_padding: self.0.cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame: self.0.frame,
                rules: self.0.rules,
                summary: self.0.summary,
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
                align: self.0.align,
                bg_color: self.0.bg_color,
                border: self.0.border,
                cell_padding: self.0.cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame: self.0.frame,
                rules: self.0.rules,
                summary: self.0.summary,
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
                align: self.0.align,
                bg_color: self.0.bg_color,
                border: self.0.border,
                cell_padding: self.0.cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame: self.0.frame,
                rules: self.0.rules,
                summary: self.0.summary,
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
                align: self.0.align,
                bg_color: self.0.bg_color,
                border: self.0.border,
                cell_padding: self.0.cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame: self.0.frame,
                rules: self.0.rules,
                summary: self.0.summary,
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
                align: self.0.align,
                bg_color: self.0.bg_color,
                border: self.0.border,
                cell_padding: self.0.cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame: self.0.frame,
                rules: self.0.rules,
                summary: self.0.summary,
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
                align: self.0.align,
                bg_color: self.0.bg_color,
                border: self.0.border,
                cell_padding: self.0.cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame: self.0.frame,
                rules: self.0.rules,
                summary: self.0.summary,
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
                align: self.0.align,
                bg_color: self.0.bg_color,
                border: self.0.border,
                cell_padding: self.0.cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame: self.0.frame,
                rules: self.0.rules,
                summary: self.0.summary,
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
                align: self.0.align,
                bg_color: self.0.bg_color,
                border: self.0.border,
                cell_padding: self.0.cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame: self.0.frame,
                rules: self.0.rules,
                summary: self.0.summary,
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
                align: self.0.align,
                bg_color: self.0.bg_color,
                border: self.0.border,
                cell_padding: self.0.cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame: self.0.frame,
                rules: self.0.rules,
                summary: self.0.summary,
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
                align: self.0.align,
                bg_color: self.0.bg_color,
                border: self.0.border,
                cell_padding: self.0.cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame: self.0.frame,
                rules: self.0.rules,
                summary: self.0.summary,
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
                align: self.0.align,
                bg_color: self.0.bg_color,
                border: self.0.border,
                cell_padding: self.0.cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame: self.0.frame,
                rules: self.0.rules,
                summary: self.0.summary,
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
                align: self.0.align,
                bg_color: self.0.bg_color,
                border: self.0.border,
                cell_padding: self.0.cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame: self.0.frame,
                rules: self.0.rules,
                summary: self.0.summary,
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
                align: self.0.align,
                bg_color: self.0.bg_color,
                border: self.0.border,
                cell_padding: self.0.cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame: self.0.frame,
                rules: self.0.rules,
                summary: self.0.summary,
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
                align: self.0.align,
                bg_color: self.0.bg_color,
                border: self.0.border,
                cell_padding: self.0.cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame: self.0.frame,
                rules: self.0.rules,
                summary: self.0.summary,
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
                align: self.0.align,
                bg_color: self.0.bg_color,
                border: self.0.border,
                cell_padding: self.0.cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame: self.0.frame,
                rules: self.0.rules,
                summary: self.0.summary,
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
                align: self.0.align,
                bg_color: self.0.bg_color,
                border: self.0.border,
                cell_padding: self.0.cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame: self.0.frame,
                rules: self.0.rules,
                summary: self.0.summary,
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
                align: self.0.align,
                bg_color: self.0.bg_color,
                border: self.0.border,
                cell_padding: self.0.cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame: self.0.frame,
                rules: self.0.rules,
                summary: self.0.summary,
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
                align: self.0.align,
                bg_color: self.0.bg_color,
                border: self.0.border,
                cell_padding: self.0.cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame: self.0.frame,
                rules: self.0.rules,
                summary: self.0.summary,
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
                align: self.0.align,
                bg_color: self.0.bg_color,
                border: self.0.border,
                cell_padding: self.0.cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame: self.0.frame,
                rules: self.0.rules,
                summary: self.0.summary,
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
                align: self.0.align,
                bg_color: self.0.bg_color,
                border: self.0.border,
                cell_padding: self.0.cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame: self.0.frame,
                rules: self.0.rules,
                summary: self.0.summary,
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
                align: self.0.align,
                bg_color: self.0.bg_color,
                border: self.0.border,
                cell_padding: self.0.cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame: self.0.frame,
                rules: self.0.rules,
                summary: self.0.summary,
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
                align: self.0.align,
                bg_color: self.0.bg_color,
                border: self.0.border,
                cell_padding: self.0.cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame: self.0.frame,
                rules: self.0.rules,
                summary: self.0.summary,
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
                align: self.0.align,
                bg_color: self.0.bg_color,
                border: self.0.border,
                cell_padding: self.0.cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame: self.0.frame,
                rules: self.0.rules,
                summary: self.0.summary,
                width: self.0.width,
            })
        }
        #[deprecated]
        #[inline(always)]
        pub fn align<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            align: V,
        ) -> super::Building<super::overwrite::align<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                align,
                bg_color: self.0.bg_color,
                border: self.0.border,
                cell_padding: self.0.cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame: self.0.frame,
                rules: self.0.rules,
                summary: self.0.summary,
                width: self.0.width,
            })
        }
        #[deprecated]
        #[inline(always)]
        pub fn bg_color<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            bg_color: V,
        ) -> super::Building<super::overwrite::bg_color<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                align: self.0.align,
                bg_color,
                border: self.0.border,
                cell_padding: self.0.cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame: self.0.frame,
                rules: self.0.rules,
                summary: self.0.summary,
                width: self.0.width,
            })
        }
        #[deprecated]
        #[inline(always)]
        pub fn border<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            border: V,
        ) -> super::Building<super::overwrite::border<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                align: self.0.align,
                bg_color: self.0.bg_color,
                border,
                cell_padding: self.0.cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame: self.0.frame,
                rules: self.0.rules,
                summary: self.0.summary,
                width: self.0.width,
            })
        }
        #[deprecated]
        #[inline(always)]
        pub fn cell_padding<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            cell_padding: V,
        ) -> super::Building<super::overwrite::cell_padding<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                align: self.0.align,
                bg_color: self.0.bg_color,
                border: self.0.border,
                cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame: self.0.frame,
                rules: self.0.rules,
                summary: self.0.summary,
                width: self.0.width,
            })
        }
        #[deprecated]
        #[inline(always)]
        pub fn cell_spacing<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            cell_spacing: V,
        ) -> super::Building<super::overwrite::cell_spacing<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                align: self.0.align,
                bg_color: self.0.bg_color,
                border: self.0.border,
                cell_padding: self.0.cell_padding,
                cell_spacing,
                frame: self.0.frame,
                rules: self.0.rules,
                summary: self.0.summary,
                width: self.0.width,
            })
        }
        #[deprecated]
        #[inline(always)]
        pub fn frame<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            frame: V,
        ) -> super::Building<super::overwrite::frame<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                align: self.0.align,
                bg_color: self.0.bg_color,
                border: self.0.border,
                cell_padding: self.0.cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame,
                rules: self.0.rules,
                summary: self.0.summary,
                width: self.0.width,
            })
        }
        #[deprecated]
        #[inline(always)]
        pub fn rules<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            rules: V,
        ) -> super::Building<super::overwrite::rules<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                align: self.0.align,
                bg_color: self.0.bg_color,
                border: self.0.border,
                cell_padding: self.0.cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame: self.0.frame,
                rules,
                summary: self.0.summary,
                width: self.0.width,
            })
        }
        #[deprecated]
        #[inline(always)]
        pub fn summary<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            summary: V,
        ) -> super::Building<super::overwrite::summary<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                align: self.0.align,
                bg_color: self.0.bg_color,
                border: self.0.border,
                cell_padding: self.0.cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame: self.0.frame,
                rules: self.0.rules,
                summary,
                width: self.0.width,
            })
        }
        #[deprecated]
        #[inline(always)]
        pub fn width<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            width: V,
        ) -> super::Building<super::overwrite::width<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                align: self.0.align,
                bg_color: self.0.bg_color,
                border: self.0.border,
                cell_padding: self.0.cell_padding,
                cell_spacing: self.0.cell_spacing,
                frame: self.0.frame,
                rules: self.0.rules,
                summary: self.0.summary,
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
        crate::props::UpdateElement<web_sys::HtmlTableElement> for super::Data<TypeDefs>
    where
        HtmlElementProps::Data<TypeDefs::HtmlElementProps>:
            crate::props::UpdateElement<web_sys::HtmlElement>,
    {
        type State = super::render_state::RenderState<
            dyn super::render_state::RenderStateTypes<
                HtmlElementProps = <HtmlElementProps::Data<
                    TypeDefs::HtmlElementProps,
                > as crate::props::UpdateElement<web_sys::HtmlElement>>::State,
                align = <TypeDefs::align as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                bg_color = <TypeDefs::bg_color as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                border = <TypeDefs::border as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                cell_padding = <TypeDefs::cell_padding as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                cell_spacing = <TypeDefs::cell_spacing as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                frame = <TypeDefs::frame as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                rules = <TypeDefs::rules as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                summary = <TypeDefs::summary as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                width = <TypeDefs::width as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
            >,
        >;
        fn initialize_state(
            this: Self,
            element: &web_sys::HtmlTableElement,
            children_ctx: &mut ::frender_dom::Dom,
        ) -> Self::State {
            let dom_element: &::web_sys::Element = element.as_ref();
            super::render_state::RenderState {
                HtmlElementProps: <HtmlElementProps::Data<
                    TypeDefs::HtmlElementProps,
                > as crate::props::UpdateElement<
                    web_sys::HtmlElement,
                >>::initialize_state(this.HtmlElementProps, element, children_ctx),
                align: <TypeDefs::align as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.align,
                    |v| element.set_align(v),
                    || dom_element.remove_attribute("align").unwrap(),
                ),
                bg_color: <TypeDefs::bg_color as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.bg_color,
                    |v| element.set_bg_color(v),
                    || dom_element.remove_attribute("bgcolor").unwrap(),
                ),
                border: <TypeDefs::border as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.border,
                    |v| element.set_border(v),
                    || dom_element.remove_attribute("border").unwrap(),
                ),
                cell_padding: <TypeDefs::cell_padding as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.cell_padding,
                    |v| element.set_cell_padding(v),
                    || dom_element.remove_attribute("cellpadding").unwrap(),
                ),
                cell_spacing: <TypeDefs::cell_spacing as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.cell_spacing,
                    |v| element.set_cell_spacing(v),
                    || dom_element.remove_attribute("cellspacing").unwrap(),
                ),
                frame: <TypeDefs::frame as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.frame,
                    |v| element.set_frame(v),
                    || dom_element.remove_attribute("frame").unwrap(),
                ),
                rules: <TypeDefs::rules as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.rules,
                    |v| element.set_rules(v),
                    || dom_element.remove_attribute("rules").unwrap(),
                ),
                summary: <TypeDefs::summary as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.summary,
                    |v| element.set_summary(v),
                    || dom_element.remove_attribute("summary").unwrap(),
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
            element: &web_sys::HtmlTableElement,
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
            <TypeDefs::align as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.align,
                state.align,
                |v| element.set_align(v),
                || dom_element.remove_attribute("align").unwrap(),
            );
            <TypeDefs::bg_color as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.bg_color,
                state.bg_color,
                |v| element.set_bg_color(v),
                || dom_element.remove_attribute("bgcolor").unwrap(),
            );
            <TypeDefs::border as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.border,
                state.border,
                |v| element.set_border(v),
                || dom_element.remove_attribute("border").unwrap(),
            );
            <TypeDefs::cell_padding as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.cell_padding,
                state.cell_padding,
                |v| element.set_cell_padding(v),
                || dom_element.remove_attribute("cellpadding").unwrap(),
            );
            <TypeDefs::cell_spacing as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.cell_spacing,
                state.cell_spacing,
                |v| element.set_cell_spacing(v),
                || dom_element.remove_attribute("cellspacing").unwrap(),
            );
            <TypeDefs::frame as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.frame,
                state.frame,
                |v| element.set_frame(v),
                || dom_element.remove_attribute("frame").unwrap(),
            );
            <TypeDefs::rules as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.rules,
                state.rules,
                |v| element.set_rules(v),
                || dom_element.remove_attribute("rules").unwrap(),
            );
            <TypeDefs::summary as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.summary,
                state.summary,
                |v| element.set_summary(v),
                || dom_element.remove_attribute("summary").unwrap(),
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
