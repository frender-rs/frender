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
        type align: crate::MaybeUpdateValueByRef<str>;
        type bg_color: crate::MaybeUpdateValueByRef<str>;
        type char: crate::MaybeUpdateValueByRef<str>;
        type char_off: crate::MaybeUpdateValueByRef<str>;
        type v_align: crate::MaybeUpdateValueByRef<str>;
        type col_span: crate::MaybeUpdateValue<u32>;
        type headers: crate::MaybeUpdateValueByRef<str>;
        type row_span: crate::MaybeUpdateValue<u32>;
        type abbr: crate::MaybeUpdateValueByRef<str>;
        type axis: crate::MaybeUpdateValueByRef<str>;
        type height: crate::MaybeUpdateValueByRef<str>;
        type scope: crate::MaybeUpdateValueByRef<str>;
        type width: crate::MaybeUpdateValueByRef<str>;
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
pub use super::HtmlElementProps::render_state;
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
        #[inline]
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
        #[inline]
        pub fn class<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline]
        pub fn id<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline]
        pub fn part<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline]
        pub fn access_key<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline]
        pub fn auto_capitalize<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline]
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
        #[inline]
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
        #[inline]
        pub fn context_menu<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline]
        pub fn dir<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline]
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
        #[inline]
        pub fn enter_key_hint<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline]
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
        #[inline]
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
        #[inline]
        pub fn input_mode<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline]
        pub fn is<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline]
        pub fn item_id<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline]
        pub fn item_prop<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline]
        pub fn item_ref<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline]
        pub fn item_scope<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline]
        pub fn item_type<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline]
        pub fn lang<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline]
        pub fn nonce<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline]
        pub fn role<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline]
        pub fn slot<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline]
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
        #[inline]
        pub fn style<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline]
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
        #[inline]
        pub fn title<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline]
        pub fn translate<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline]
        pub fn virtual_keyboard_policy<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline]
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
        #[inline]
        pub fn align<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline]
        pub fn bg_color<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline]
        pub fn char<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline]
        pub fn char_off<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline]
        pub fn v_align<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline]
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
        #[inline]
        pub fn headers<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline]
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
        #[inline]
        pub fn abbr<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline]
        pub fn axis<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline]
        pub fn height<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline]
        pub fn scope<V: crate::MaybeUpdateValueByRef<str>>(
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
        #[inline]
        pub fn width<V: crate::MaybeUpdateValueByRef<str>>(
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
        type State =
            <HtmlElementProps::Data<TypeDefs::HtmlElementProps> as crate::props::UpdateElement<
                web_sys::HtmlElement,
            >>::State;
        fn update_element(
            this: Self,
            element: &web_sys::HtmlTableCellElement,
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
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "align";
                < TypeDefs :: align as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . align , | v | element . set_align (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "bgcolor";
                < TypeDefs :: bg_color as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . bg_color , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "char";
                < TypeDefs :: char as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . char , | v | element . set_ch (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "charoff";
                < TypeDefs :: char_off as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . char_off , | v | element . set_ch_off (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "valign";
                < TypeDefs :: v_align as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . v_align , | v | element . set_v_align (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "colspan";
                < TypeDefs :: col_span as :: frender_dom :: props :: MaybeUpdateValue < u32 , > > :: maybe_update_value (this . col_span , | v | element . set_col_span (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "headers";
                < TypeDefs :: headers as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . headers , | v | element . set_headers (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "rowspan";
                < TypeDefs :: row_span as :: frender_dom :: props :: MaybeUpdateValue < u32 , > > :: maybe_update_value (this . row_span , | v | element . set_row_span (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "abbr";
                < TypeDefs :: abbr as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . abbr , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "axis";
                < TypeDefs :: axis as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . axis , | v | element . set_axis (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "height";
                < TypeDefs :: height as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . height , | v | element . set_height (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "scope";
                < TypeDefs :: scope as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . scope , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "width";
                < TypeDefs :: width as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . width , | v | element . set_width (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
        }
    }
}
