#[allow(non_snake_case)]
pub fn HtmlTableCellElementProps() -> Building<TypesInitial> {
    #[allow(unused_imports)]
    use super::*;
    self::Building(self::Data {
        HtmlElementProps: HtmlElementProps::build(HtmlElementProps()),
        align: (),
        bg_color: (),
        char: (),
        char_off: (),
        v_align: (),
        col_span: (),
        headers: (),
        row_span: (),
        abbr: (),
        axis: (),
        height: (),
        scope: (),
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
        char = <TypeDefs as super::Types>::char,
        char_off = <TypeDefs as super::Types>::char_off,
        v_align = <TypeDefs as super::Types>::v_align,
        col_span = <TypeDefs as super::Types>::col_span,
        headers = <TypeDefs as super::Types>::headers,
        row_span = <TypeDefs as super::Types>::row_span,
        abbr = <TypeDefs as super::Types>::abbr,
        axis = <TypeDefs as super::Types>::axis,
        height = <TypeDefs as super::Types>::height,
        scope = <TypeDefs as super::Types>::scope,
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
        char = <TypeDefs as super::Types>::char,
        char_off = <TypeDefs as super::Types>::char_off,
        v_align = <TypeDefs as super::Types>::v_align,
        col_span = <TypeDefs as super::Types>::col_span,
        headers = <TypeDefs as super::Types>::headers,
        row_span = <TypeDefs as super::Types>::row_span,
        abbr = <TypeDefs as super::Types>::abbr,
        axis = <TypeDefs as super::Types>::axis,
        height = <TypeDefs as super::Types>::height,
        scope = <TypeDefs as super::Types>::scope,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type bg_color<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        align = <TypeDefs as super::Types>::align,
        bg_color = Value,
        char = <TypeDefs as super::Types>::char,
        char_off = <TypeDefs as super::Types>::char_off,
        v_align = <TypeDefs as super::Types>::v_align,
        col_span = <TypeDefs as super::Types>::col_span,
        headers = <TypeDefs as super::Types>::headers,
        row_span = <TypeDefs as super::Types>::row_span,
        abbr = <TypeDefs as super::Types>::abbr,
        axis = <TypeDefs as super::Types>::axis,
        height = <TypeDefs as super::Types>::height,
        scope = <TypeDefs as super::Types>::scope,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type char<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        align = <TypeDefs as super::Types>::align,
        bg_color = <TypeDefs as super::Types>::bg_color,
        char = Value,
        char_off = <TypeDefs as super::Types>::char_off,
        v_align = <TypeDefs as super::Types>::v_align,
        col_span = <TypeDefs as super::Types>::col_span,
        headers = <TypeDefs as super::Types>::headers,
        row_span = <TypeDefs as super::Types>::row_span,
        abbr = <TypeDefs as super::Types>::abbr,
        axis = <TypeDefs as super::Types>::axis,
        height = <TypeDefs as super::Types>::height,
        scope = <TypeDefs as super::Types>::scope,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type char_off<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        align = <TypeDefs as super::Types>::align,
        bg_color = <TypeDefs as super::Types>::bg_color,
        char = <TypeDefs as super::Types>::char,
        char_off = Value,
        v_align = <TypeDefs as super::Types>::v_align,
        col_span = <TypeDefs as super::Types>::col_span,
        headers = <TypeDefs as super::Types>::headers,
        row_span = <TypeDefs as super::Types>::row_span,
        abbr = <TypeDefs as super::Types>::abbr,
        axis = <TypeDefs as super::Types>::axis,
        height = <TypeDefs as super::Types>::height,
        scope = <TypeDefs as super::Types>::scope,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type v_align<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        align = <TypeDefs as super::Types>::align,
        bg_color = <TypeDefs as super::Types>::bg_color,
        char = <TypeDefs as super::Types>::char,
        char_off = <TypeDefs as super::Types>::char_off,
        v_align = Value,
        col_span = <TypeDefs as super::Types>::col_span,
        headers = <TypeDefs as super::Types>::headers,
        row_span = <TypeDefs as super::Types>::row_span,
        abbr = <TypeDefs as super::Types>::abbr,
        axis = <TypeDefs as super::Types>::axis,
        height = <TypeDefs as super::Types>::height,
        scope = <TypeDefs as super::Types>::scope,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type col_span<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        align = <TypeDefs as super::Types>::align,
        bg_color = <TypeDefs as super::Types>::bg_color,
        char = <TypeDefs as super::Types>::char,
        char_off = <TypeDefs as super::Types>::char_off,
        v_align = <TypeDefs as super::Types>::v_align,
        col_span = Value,
        headers = <TypeDefs as super::Types>::headers,
        row_span = <TypeDefs as super::Types>::row_span,
        abbr = <TypeDefs as super::Types>::abbr,
        axis = <TypeDefs as super::Types>::axis,
        height = <TypeDefs as super::Types>::height,
        scope = <TypeDefs as super::Types>::scope,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type headers<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        align = <TypeDefs as super::Types>::align,
        bg_color = <TypeDefs as super::Types>::bg_color,
        char = <TypeDefs as super::Types>::char,
        char_off = <TypeDefs as super::Types>::char_off,
        v_align = <TypeDefs as super::Types>::v_align,
        col_span = <TypeDefs as super::Types>::col_span,
        headers = Value,
        row_span = <TypeDefs as super::Types>::row_span,
        abbr = <TypeDefs as super::Types>::abbr,
        axis = <TypeDefs as super::Types>::axis,
        height = <TypeDefs as super::Types>::height,
        scope = <TypeDefs as super::Types>::scope,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type row_span<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        align = <TypeDefs as super::Types>::align,
        bg_color = <TypeDefs as super::Types>::bg_color,
        char = <TypeDefs as super::Types>::char,
        char_off = <TypeDefs as super::Types>::char_off,
        v_align = <TypeDefs as super::Types>::v_align,
        col_span = <TypeDefs as super::Types>::col_span,
        headers = <TypeDefs as super::Types>::headers,
        row_span = Value,
        abbr = <TypeDefs as super::Types>::abbr,
        axis = <TypeDefs as super::Types>::axis,
        height = <TypeDefs as super::Types>::height,
        scope = <TypeDefs as super::Types>::scope,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type abbr<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        align = <TypeDefs as super::Types>::align,
        bg_color = <TypeDefs as super::Types>::bg_color,
        char = <TypeDefs as super::Types>::char,
        char_off = <TypeDefs as super::Types>::char_off,
        v_align = <TypeDefs as super::Types>::v_align,
        col_span = <TypeDefs as super::Types>::col_span,
        headers = <TypeDefs as super::Types>::headers,
        row_span = <TypeDefs as super::Types>::row_span,
        abbr = Value,
        axis = <TypeDefs as super::Types>::axis,
        height = <TypeDefs as super::Types>::height,
        scope = <TypeDefs as super::Types>::scope,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type axis<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        align = <TypeDefs as super::Types>::align,
        bg_color = <TypeDefs as super::Types>::bg_color,
        char = <TypeDefs as super::Types>::char,
        char_off = <TypeDefs as super::Types>::char_off,
        v_align = <TypeDefs as super::Types>::v_align,
        col_span = <TypeDefs as super::Types>::col_span,
        headers = <TypeDefs as super::Types>::headers,
        row_span = <TypeDefs as super::Types>::row_span,
        abbr = <TypeDefs as super::Types>::abbr,
        axis = Value,
        height = <TypeDefs as super::Types>::height,
        scope = <TypeDefs as super::Types>::scope,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type height<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        align = <TypeDefs as super::Types>::align,
        bg_color = <TypeDefs as super::Types>::bg_color,
        char = <TypeDefs as super::Types>::char,
        char_off = <TypeDefs as super::Types>::char_off,
        v_align = <TypeDefs as super::Types>::v_align,
        col_span = <TypeDefs as super::Types>::col_span,
        headers = <TypeDefs as super::Types>::headers,
        row_span = <TypeDefs as super::Types>::row_span,
        abbr = <TypeDefs as super::Types>::abbr,
        axis = <TypeDefs as super::Types>::axis,
        height = Value,
        scope = <TypeDefs as super::Types>::scope,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type scope<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        align = <TypeDefs as super::Types>::align,
        bg_color = <TypeDefs as super::Types>::bg_color,
        char = <TypeDefs as super::Types>::char,
        char_off = <TypeDefs as super::Types>::char_off,
        v_align = <TypeDefs as super::Types>::v_align,
        col_span = <TypeDefs as super::Types>::col_span,
        headers = <TypeDefs as super::Types>::headers,
        row_span = <TypeDefs as super::Types>::row_span,
        abbr = <TypeDefs as super::Types>::abbr,
        axis = <TypeDefs as super::Types>::axis,
        height = <TypeDefs as super::Types>::height,
        scope = Value,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type width<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        align = <TypeDefs as super::Types>::align,
        bg_color = <TypeDefs as super::Types>::bg_color,
        char = <TypeDefs as super::Types>::char,
        char_off = <TypeDefs as super::Types>::char_off,
        v_align = <TypeDefs as super::Types>::v_align,
        col_span = <TypeDefs as super::Types>::col_span,
        headers = <TypeDefs as super::Types>::headers,
        row_span = <TypeDefs as super::Types>::row_span,
        abbr = <TypeDefs as super::Types>::abbr,
        axis = <TypeDefs as super::Types>::axis,
        height = <TypeDefs as super::Types>::height,
        scope = <TypeDefs as super::Types>::scope,
        width = Value,
    >;
}
mod trait_types {
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub trait Types {
        type HtmlElementProps: ?::core::marker::Sized + HtmlElementProps::Types;
        type align: crate::MaybeUpdateValue<str>;
        type bg_color: crate::MaybeUpdateValue<str>;
        type char: crate::MaybeUpdateValue<str>;
        type char_off: crate::MaybeUpdateValue<str>;
        type v_align: crate::MaybeUpdateValue<str>;
        type col_span: crate::MaybeUpdateValue<u32>;
        type headers: crate::MaybeUpdateValue<str>;
        type row_span: crate::MaybeUpdateValue<u32>;
        type abbr: crate::MaybeUpdateValue<str>;
        type axis: crate::MaybeUpdateValue<str>;
        type height: crate::MaybeUpdateValue<str>;
        type scope: crate::MaybeUpdateValue<str>;
        type width: crate::MaybeUpdateValue<str>;
    }
}
pub use trait_types::Types;
pub use trait_types::Types as ValidTypes;
pub mod data_struct {
    #[non_exhaustive]
    pub struct HtmlTableCellElementProps<TypeDefs: super::Types + ?::core::marker::Sized> {
        pub HtmlElementProps: super::super::HtmlElementProps::Data<TypeDefs::HtmlElementProps>,
        pub align: TypeDefs::align,
        pub bg_color: TypeDefs::bg_color,
        pub char: TypeDefs::char,
        pub char_off: TypeDefs::char_off,
        pub v_align: TypeDefs::v_align,
        pub col_span: TypeDefs::col_span,
        pub headers: TypeDefs::headers,
        pub row_span: TypeDefs::row_span,
        pub abbr: TypeDefs::abbr,
        pub axis: TypeDefs::axis,
        pub height: TypeDefs::height,
        pub scope: TypeDefs::scope,
        pub width: TypeDefs::width,
    }
}
pub use data_struct::HtmlTableCellElementProps as Data;
pub struct Building<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
pub struct Replacing<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
mod types_initial {
    #[allow(unused_imports)]
    use super::super::*;
    pub type TypesInitial = dyn super::Types<
        HtmlElementProps = HtmlElementProps::TypesInitial,
        align = (),
        bg_color = (),
        char = (),
        char_off = (),
        v_align = (),
        col_span = (),
        headers = (),
        row_span = (),
        abbr = (),
        axis = (),
        height = (),
        scope = (),
        width = (),
    >;
}
pub use types_initial::TypesInitial;
pub type DataInitial = Data<TypesInitial>;
#[cfg(feature = "dom")]
pub mod render_state {
    #[allow(non_camel_case_types)]
    pub trait RenderStateTypes {
        type HtmlElementProps: ::core::default::Default
            + crate::props::IntrinsicComponentPollReactive;
        type align: ::core::default::Default;
        type bg_color: ::core::default::Default;
        type char: ::core::default::Default;
        type char_off: ::core::default::Default;
        type v_align: ::core::default::Default;
        type col_span: ::core::default::Default;
        type headers: ::core::default::Default;
        type row_span: ::core::default::Default;
        type abbr: ::core::default::Default;
        type axis: ::core::default::Default;
        type height: ::core::default::Default;
        type scope: ::core::default::Default;
        type width: ::core::default::Default;
    }
    pub struct RenderState<TypeDefs: RenderStateTypes>
    where
        TypeDefs: ?::core::marker::Sized,
    {
        pub HtmlElementProps: TypeDefs::HtmlElementProps,
        pub align: TypeDefs::align,
        pub bg_color: TypeDefs::bg_color,
        pub char: TypeDefs::char,
        pub char_off: TypeDefs::char_off,
        pub v_align: TypeDefs::v_align,
        pub col_span: TypeDefs::col_span,
        pub headers: TypeDefs::headers,
        pub row_span: TypeDefs::row_span,
        pub abbr: TypeDefs::abbr,
        pub axis: TypeDefs::axis,
        pub height: TypeDefs::height,
        pub scope: TypeDefs::scope,
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
        pub char: &'__pin mut (TypeDefs::char),
        pub char_off: &'__pin mut (TypeDefs::char_off),
        pub v_align: &'__pin mut (TypeDefs::v_align),
        pub col_span: &'__pin mut (TypeDefs::col_span),
        pub headers: &'__pin mut (TypeDefs::headers),
        pub row_span: &'__pin mut (TypeDefs::row_span),
        pub abbr: &'__pin mut (TypeDefs::abbr),
        pub axis: &'__pin mut (TypeDefs::axis),
        pub height: &'__pin mut (TypeDefs::height),
        pub scope: &'__pin mut (TypeDefs::scope),
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
            pub char: &'__pin (TypeDefs::char),
            pub char_off: &'__pin (TypeDefs::char_off),
            pub v_align: &'__pin (TypeDefs::v_align),
            pub col_span: &'__pin (TypeDefs::col_span),
            pub headers: &'__pin (TypeDefs::headers),
            pub row_span: &'__pin (TypeDefs::row_span),
            pub abbr: &'__pin (TypeDefs::abbr),
            pub axis: &'__pin (TypeDefs::axis),
            pub height: &'__pin (TypeDefs::height),
            pub scope: &'__pin (TypeDefs::scope),
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
                        char,
                        char_off,
                        v_align,
                        col_span,
                        headers,
                        row_span,
                        abbr,
                        axis,
                        height,
                        scope,
                        width,
                    } = self.get_unchecked_mut();
                    RenderStateProj {
                        HtmlElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            HtmlElementProps,
                        ),
                        align: align,
                        bg_color: bg_color,
                        char: char,
                        char_off: char_off,
                        v_align: v_align,
                        col_span: col_span,
                        headers: headers,
                        row_span: row_span,
                        abbr: abbr,
                        axis: axis,
                        height: height,
                        scope: scope,
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
                        char,
                        char_off,
                        v_align,
                        col_span,
                        headers,
                        row_span,
                        abbr,
                        axis,
                        height,
                        scope,
                        width,
                    } = self.get_ref();
                    ProjectionRef {
                        HtmlElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            HtmlElementProps,
                        ),
                        align: align,
                        bg_color: bg_color,
                        char: char,
                        char_off: char_off,
                        v_align: v_align,
                        col_span: col_span,
                        headers: headers,
                        row_span: row_span,
                        abbr: abbr,
                        axis: axis,
                        height: height,
                        scope: scope,
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
            char: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::char>,
            char_off: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::char_off>,
            v_align: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::v_align>,
            col_span: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::col_span>,
            headers: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::headers>,
            row_span: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::row_span>,
            abbr: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::abbr>,
            axis: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::axis>,
            height: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::height>,
            scope: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::scope>,
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
            let _ = &this.char;
            let _ = &this.char_off;
            let _ = &this.v_align;
            let _ = &this.col_span;
            let _ = &this.headers;
            let _ = &this.row_span;
            let _ = &this.abbr;
            let _ = &this.axis;
            let _ = &this.height;
            let _ = &this.scope;
            let _ = &this.width;
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
                HtmlElementProps: ::core::default::Default::default(),
                align: ::core::default::Default::default(),
                bg_color: ::core::default::Default::default(),
                char: ::core::default::Default::default(),
                char_off: ::core::default::Default::default(),
                v_align: ::core::default::Default::default(),
                col_span: ::core::default::Default::default(),
                headers: ::core::default::Default::default(),
                row_span: ::core::default::Default::default(),
                abbr: ::core::default::Default::default(),
                axis: ::core::default::Default::default(),
                height: ::core::default::Default::default(),
                scope: ::core::default::Default::default(),
                width: ::core::default::Default::default(),
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
                self.project().HtmlElementProps,
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
        #[doc = "See [`HtmlElementProps::children`]"]
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
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::class`]"]
        pub fn class<V: crate::MaybeUpdateValue<str>>(
            self,
            class: V,
        ) -> super::Building<super::overwrite::class<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).class(class),
                ),
                align: self.0.align,
                bg_color: self.0.bg_color,
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::id`]"]
        pub fn id<V: crate::MaybeUpdateValue<str>>(
            self,
            id: V,
        ) -> super::Building<super::overwrite::id<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).id(id),
                ),
                align: self.0.align,
                bg_color: self.0.bg_color,
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::part`]"]
        pub fn part<V: crate::MaybeUpdateValue<str>>(
            self,
            part: V,
        ) -> super::Building<super::overwrite::part<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).part(part),
                ),
                align: self.0.align,
                bg_color: self.0.bg_color,
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::access_key`]"]
        pub fn access_key<V: crate::MaybeUpdateValue<str>>(
            self,
            access_key: V,
        ) -> super::Building<super::overwrite::access_key<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).access_key(access_key),
                ),
                align: self.0.align,
                bg_color: self.0.bg_color,
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::auto_capitalize`]"]
        pub fn auto_capitalize<V: crate::MaybeUpdateValue<str>>(
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
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::auto_focus`]"]
        pub fn auto_focus<V: crate::MaybeUpdateValue<bool>>(
            self,
            auto_focus: V,
        ) -> super::Building<super::overwrite::auto_focus<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).auto_focus(auto_focus),
                ),
                align: self.0.align,
                bg_color: self.0.bg_color,
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::content_editable`]"]
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
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::context_menu`]"]
        pub fn context_menu<V: crate::MaybeUpdateValue<str>>(
            self,
            context_menu: V,
        ) -> super::Building<super::overwrite::context_menu<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).context_menu(context_menu),
                ),
                align: self.0.align,
                bg_color: self.0.bg_color,
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::dir`]"]
        pub fn dir<V: crate::MaybeUpdateValue<str>>(
            self,
            dir: V,
        ) -> super::Building<super::overwrite::dir<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).dir(dir),
                ),
                align: self.0.align,
                bg_color: self.0.bg_color,
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::draggable`]"]
        pub fn draggable<V: crate::MaybeUpdateValue<bool>>(
            self,
            draggable: V,
        ) -> super::Building<super::overwrite::draggable<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).draggable(draggable),
                ),
                align: self.0.align,
                bg_color: self.0.bg_color,
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::enter_key_hint`]"]
        pub fn enter_key_hint<V: crate::MaybeUpdateValue<str>>(
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
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::hidden`]"]
        pub fn hidden<V: crate::MaybeUpdateValue<bool>>(
            self,
            hidden: V,
        ) -> super::Building<super::overwrite::hidden<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).hidden(hidden),
                ),
                align: self.0.align,
                bg_color: self.0.bg_color,
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::inert`]"]
        pub fn inert<V: crate::MaybeUpdateValue<bool>>(
            self,
            inert: V,
        ) -> super::Building<super::overwrite::inert<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).inert(inert),
                ),
                align: self.0.align,
                bg_color: self.0.bg_color,
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::input_mode`]"]
        pub fn input_mode<V: crate::MaybeUpdateValue<str>>(
            self,
            input_mode: V,
        ) -> super::Building<super::overwrite::input_mode<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).input_mode(input_mode),
                ),
                align: self.0.align,
                bg_color: self.0.bg_color,
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::is`]"]
        pub fn is<V: crate::MaybeUpdateValue<str>>(
            self,
            is: V,
        ) -> super::Building<super::overwrite::is<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).is(is),
                ),
                align: self.0.align,
                bg_color: self.0.bg_color,
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::item_id`]"]
        pub fn item_id<V: crate::MaybeUpdateValue<str>>(
            self,
            item_id: V,
        ) -> super::Building<super::overwrite::item_id<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).item_id(item_id),
                ),
                align: self.0.align,
                bg_color: self.0.bg_color,
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::item_prop`]"]
        pub fn item_prop<V: crate::MaybeUpdateValue<str>>(
            self,
            item_prop: V,
        ) -> super::Building<super::overwrite::item_prop<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).item_prop(item_prop),
                ),
                align: self.0.align,
                bg_color: self.0.bg_color,
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::item_ref`]"]
        pub fn item_ref<V: crate::MaybeUpdateValue<str>>(
            self,
            item_ref: V,
        ) -> super::Building<super::overwrite::item_ref<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).item_ref(item_ref),
                ),
                align: self.0.align,
                bg_color: self.0.bg_color,
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::item_scope`]"]
        pub fn item_scope<V: crate::MaybeUpdateValue<str>>(
            self,
            item_scope: V,
        ) -> super::Building<super::overwrite::item_scope<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).item_scope(item_scope),
                ),
                align: self.0.align,
                bg_color: self.0.bg_color,
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::item_type`]"]
        pub fn item_type<V: crate::MaybeUpdateValue<str>>(
            self,
            item_type: V,
        ) -> super::Building<super::overwrite::item_type<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).item_type(item_type),
                ),
                align: self.0.align,
                bg_color: self.0.bg_color,
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::lang`]"]
        pub fn lang<V: crate::MaybeUpdateValue<str>>(
            self,
            lang: V,
        ) -> super::Building<super::overwrite::lang<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).lang(lang),
                ),
                align: self.0.align,
                bg_color: self.0.bg_color,
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::nonce`]"]
        pub fn nonce<V: crate::MaybeUpdateValue<str>>(
            self,
            nonce: V,
        ) -> super::Building<super::overwrite::nonce<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).nonce(nonce),
                ),
                align: self.0.align,
                bg_color: self.0.bg_color,
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::role`]"]
        pub fn role<V: crate::MaybeUpdateValue<str>>(
            self,
            role: V,
        ) -> super::Building<super::overwrite::role<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).role(role),
                ),
                align: self.0.align,
                bg_color: self.0.bg_color,
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::slot`]"]
        pub fn slot<V: crate::MaybeUpdateValue<str>>(
            self,
            slot: V,
        ) -> super::Building<super::overwrite::slot<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).slot(slot),
                ),
                align: self.0.align,
                bg_color: self.0.bg_color,
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::spellcheck`]"]
        pub fn spellcheck<V: crate::MaybeUpdateValue<bool>>(
            self,
            spellcheck: V,
        ) -> super::Building<super::overwrite::spellcheck<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).spellcheck(spellcheck),
                ),
                align: self.0.align,
                bg_color: self.0.bg_color,
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::style`]"]
        pub fn style<V: crate::MaybeUpdateValue<str>>(
            self,
            style: V,
        ) -> super::Building<super::overwrite::style<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).style(style),
                ),
                align: self.0.align,
                bg_color: self.0.bg_color,
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::tab_index`]"]
        pub fn tab_index<V: crate::MaybeUpdateValue<i32>>(
            self,
            tab_index: V,
        ) -> super::Building<super::overwrite::tab_index<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).tab_index(tab_index),
                ),
                align: self.0.align,
                bg_color: self.0.bg_color,
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::title`]"]
        pub fn title<V: crate::MaybeUpdateValue<str>>(
            self,
            title: V,
        ) -> super::Building<super::overwrite::title<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).title(title),
                ),
                align: self.0.align,
                bg_color: self.0.bg_color,
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::translate`]"]
        pub fn translate<V: crate::MaybeUpdateValue<str>>(
            self,
            translate: V,
        ) -> super::Building<super::overwrite::translate<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).translate(translate),
                ),
                align: self.0.align,
                bg_color: self.0.bg_color,
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::virtual_keyboard_policy`]"]
        pub fn virtual_keyboard_policy<V: crate::MaybeUpdateValue<str>>(
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
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::on_click`]"]
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
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[deprecated]
        pub fn align<V: crate::MaybeUpdateValue<str>>(
            self,
            align: V,
        ) -> super::Building<super::overwrite::align<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                align,
                bg_color: self.0.bg_color,
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[deprecated]
        pub fn bg_color<V: crate::MaybeUpdateValue<str>>(
            self,
            bg_color: V,
        ) -> super::Building<super::overwrite::bg_color<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                align: self.0.align,
                bg_color,
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[deprecated]
        pub fn char<V: crate::MaybeUpdateValue<str>>(
            self,
            char: V,
        ) -> super::Building<super::overwrite::char<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                align: self.0.align,
                bg_color: self.0.bg_color,
                char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[deprecated]
        pub fn char_off<V: crate::MaybeUpdateValue<str>>(
            self,
            char_off: V,
        ) -> super::Building<super::overwrite::char_off<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                align: self.0.align,
                bg_color: self.0.bg_color,
                char: self.0.char,
                char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[deprecated]
        pub fn v_align<V: crate::MaybeUpdateValue<str>>(
            self,
            v_align: V,
        ) -> super::Building<super::overwrite::v_align<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                align: self.0.align,
                bg_color: self.0.bg_color,
                char: self.0.char,
                char_off: self.0.char_off,
                v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        pub fn col_span<V: crate::MaybeUpdateValue<u32>>(
            self,
            col_span: V,
        ) -> super::Building<super::overwrite::col_span<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                align: self.0.align,
                bg_color: self.0.bg_color,
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        pub fn headers<V: crate::MaybeUpdateValue<str>>(
            self,
            headers: V,
        ) -> super::Building<super::overwrite::headers<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                align: self.0.align,
                bg_color: self.0.bg_color,
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        pub fn row_span<V: crate::MaybeUpdateValue<u32>>(
            self,
            row_span: V,
        ) -> super::Building<super::overwrite::row_span<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                align: self.0.align,
                bg_color: self.0.bg_color,
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[deprecated = "Do not use this attribute as it is obsolete in the latest standard. Alternatively, you can put the abbreviated description inside the cell and place the long content in the title attribute."]
        pub fn abbr<V: crate::MaybeUpdateValue<str>>(
            self,
            abbr: V,
        ) -> super::Building<super::overwrite::abbr<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                align: self.0.align,
                bg_color: self.0.bg_color,
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[deprecated]
        pub fn axis<V: crate::MaybeUpdateValue<str>>(
            self,
            axis: V,
        ) -> super::Building<super::overwrite::axis<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                align: self.0.align,
                bg_color: self.0.bg_color,
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis,
                height: self.0.height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[deprecated = "Use the CSS height property instead."]
        pub fn height<V: crate::MaybeUpdateValue<str>>(
            self,
            height: V,
        ) -> super::Building<super::overwrite::height<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                align: self.0.align,
                bg_color: self.0.bg_color,
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height,
                scope: self.0.scope,
                width: self.0.width,
            })
        }
        #[deprecated]
        pub fn scope<V: crate::MaybeUpdateValue<str>>(
            self,
            scope: V,
        ) -> super::Building<super::overwrite::scope<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                align: self.0.align,
                bg_color: self.0.bg_color,
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope,
                width: self.0.width,
            })
        }
        #[deprecated]
        pub fn width<V: crate::MaybeUpdateValue<str>>(
            self,
            width: V,
        ) -> super::Building<super::overwrite::width<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                align: self.0.align,
                bg_color: self.0.bg_color,
                char: self.0.char,
                char_off: self.0.char_off,
                v_align: self.0.v_align,
                col_span: self.0.col_span,
                headers: self.0.headers,
                row_span: self.0.row_span,
                abbr: self.0.abbr,
                axis: self.0.axis,
                height: self.0.height,
                scope: self.0.scope,
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
        crate::props::UpdateElement<web_sys::HtmlTableCellElement> for super::Data<TypeDefs>
    where
        HtmlElementProps::Data<TypeDefs::HtmlElementProps>:
            crate::props::UpdateElement<web_sys::HtmlElement>,
    {
        type State = super :: render_state :: RenderState < dyn super :: render_state :: RenderStateTypes < HtmlElementProps = < HtmlElementProps :: Data < TypeDefs :: HtmlElementProps , > as crate :: props :: UpdateElement < web_sys :: HtmlElement > > :: State , align = < TypeDefs :: align as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , bg_color = < TypeDefs :: bg_color as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , char = < TypeDefs :: char as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , char_off = < TypeDefs :: char_off as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , v_align = < TypeDefs :: v_align as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , col_span = < TypeDefs :: col_span as :: frender_dom :: props :: MaybeUpdateValue < u32 , > > :: State , headers = < TypeDefs :: headers as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , row_span = < TypeDefs :: row_span as :: frender_dom :: props :: MaybeUpdateValue < u32 , > > :: State , abbr = < TypeDefs :: abbr as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , axis = < TypeDefs :: axis as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , height = < TypeDefs :: height as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , scope = < TypeDefs :: scope as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , width = < TypeDefs :: width as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , > , > ;
        fn update_element(
            this: Self,
            element: &web_sys::HtmlTableCellElement,
            children_ctx: &mut ::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let state = state.pin_project();
            let dom_element: &::web_sys::Element = element.as_ref();
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.HtmlElementProps,
                state: state.HtmlElementProps,
                element,
                dom_element,
                children_ctx: &mut *children_ctx,
            }) {
                crate::props::FieldData {
                    data,
                    state,
                    element,
                    children_ctx,
                    ..
                } => crate::props::UpdateElement::update_element(
                    data,
                    element.as_ref(),
                    children_ctx,
                    state,
                ),
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.align,
                state: state.align,
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
                    const ATTR_NAME: &::core::primitive::str = "align";
                    < TypeDefs :: align as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | element . set_align (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.bg_color,
                state: state.bg_color,
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
                    const ATTR_NAME: &::core::primitive::str = "bgcolor";
                    < TypeDefs :: bg_color as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.char,
                state: state.char,
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
                    const ATTR_NAME: &::core::primitive::str = "char";
                    < TypeDefs :: char as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | element . set_ch (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.char_off,
                state: state.char_off,
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
                    const ATTR_NAME: &::core::primitive::str = "charoff";
                    < TypeDefs :: char_off as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | element . set_ch_off (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.v_align,
                state: state.v_align,
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
                    const ATTR_NAME: &::core::primitive::str = "valign";
                    < TypeDefs :: v_align as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | element . set_v_align (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.col_span,
                state: state.col_span,
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
                    const ATTR_NAME: &::core::primitive::str = "colspan";
                    < TypeDefs :: col_span as :: frender_dom :: props :: MaybeUpdateValue < u32 , > > :: maybe_update_value (data , state , | v | element . set_col_span (* v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.headers,
                state: state.headers,
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
                    const ATTR_NAME: &::core::primitive::str = "headers";
                    < TypeDefs :: headers as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | element . set_headers (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.row_span,
                state: state.row_span,
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
                    const ATTR_NAME: &::core::primitive::str = "rowspan";
                    < TypeDefs :: row_span as :: frender_dom :: props :: MaybeUpdateValue < u32 , > > :: maybe_update_value (data , state , | v | element . set_row_span (* v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.abbr,
                state: state.abbr,
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
                    const ATTR_NAME: &::core::primitive::str = "abbr";
                    < TypeDefs :: abbr as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.axis,
                state: state.axis,
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
                    const ATTR_NAME: &::core::primitive::str = "axis";
                    < TypeDefs :: axis as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | element . set_axis (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.height,
                state: state.height,
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
                    const ATTR_NAME: &::core::primitive::str = "height";
                    < TypeDefs :: height as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | element . set_height (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.scope,
                state: state.scope,
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
                    const ATTR_NAME: &::core::primitive::str = "scope";
                    < TypeDefs :: scope as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.width,
                state: state.width,
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
                    const ATTR_NAME: &::core::primitive::str = "width";
                    < TypeDefs :: width as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | element . set_width (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
        }
    }
}
