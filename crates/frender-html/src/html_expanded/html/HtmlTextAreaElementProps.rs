#[allow(non_snake_case)]
pub fn HtmlTextAreaElementProps() -> Building<TypesInitial> {
    #[allow(unused_imports)]
    use super::*;
    self::Building(self::Data {
        HtmlElementProps: HtmlElementProps::build(HtmlElementProps()),
        auto_complete: (),
        auto_correct: (),
        cols: (),
        disabled: (),
        form: (),
        max_length: (),
        min_length: (),
        name: (),
        placeholder: (),
        read_only: (),
        required: (),
        rows: (),
        wrap: (),
    })
}
pub mod prelude {}
pub mod overwrite {
    #![allow(non_camel_case_types)]
    pub type HtmlElementProps<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = Value,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        auto_correct = <TypeDefs as super::Types>::auto_correct,
        cols = <TypeDefs as super::Types>::cols,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        max_length = <TypeDefs as super::Types>::max_length,
        min_length = <TypeDefs as super::Types>::min_length,
        name = <TypeDefs as super::Types>::name,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        rows = <TypeDefs as super::Types>::rows,
        wrap = <TypeDefs as super::Types>::wrap,
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
    pub type auto_complete<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_complete = Value,
        auto_correct = <TypeDefs as super::Types>::auto_correct,
        cols = <TypeDefs as super::Types>::cols,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        max_length = <TypeDefs as super::Types>::max_length,
        min_length = <TypeDefs as super::Types>::min_length,
        name = <TypeDefs as super::Types>::name,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        rows = <TypeDefs as super::Types>::rows,
        wrap = <TypeDefs as super::Types>::wrap,
    >;
    pub type auto_correct<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        auto_correct = Value,
        cols = <TypeDefs as super::Types>::cols,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        max_length = <TypeDefs as super::Types>::max_length,
        min_length = <TypeDefs as super::Types>::min_length,
        name = <TypeDefs as super::Types>::name,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        rows = <TypeDefs as super::Types>::rows,
        wrap = <TypeDefs as super::Types>::wrap,
    >;
    pub type cols<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        auto_correct = <TypeDefs as super::Types>::auto_correct,
        cols = Value,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        max_length = <TypeDefs as super::Types>::max_length,
        min_length = <TypeDefs as super::Types>::min_length,
        name = <TypeDefs as super::Types>::name,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        rows = <TypeDefs as super::Types>::rows,
        wrap = <TypeDefs as super::Types>::wrap,
    >;
    pub type disabled<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        auto_correct = <TypeDefs as super::Types>::auto_correct,
        cols = <TypeDefs as super::Types>::cols,
        disabled = Value,
        form = <TypeDefs as super::Types>::form,
        max_length = <TypeDefs as super::Types>::max_length,
        min_length = <TypeDefs as super::Types>::min_length,
        name = <TypeDefs as super::Types>::name,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        rows = <TypeDefs as super::Types>::rows,
        wrap = <TypeDefs as super::Types>::wrap,
    >;
    pub type form<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        auto_correct = <TypeDefs as super::Types>::auto_correct,
        cols = <TypeDefs as super::Types>::cols,
        disabled = <TypeDefs as super::Types>::disabled,
        form = Value,
        max_length = <TypeDefs as super::Types>::max_length,
        min_length = <TypeDefs as super::Types>::min_length,
        name = <TypeDefs as super::Types>::name,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        rows = <TypeDefs as super::Types>::rows,
        wrap = <TypeDefs as super::Types>::wrap,
    >;
    pub type max_length<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        auto_correct = <TypeDefs as super::Types>::auto_correct,
        cols = <TypeDefs as super::Types>::cols,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        max_length = Value,
        min_length = <TypeDefs as super::Types>::min_length,
        name = <TypeDefs as super::Types>::name,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        rows = <TypeDefs as super::Types>::rows,
        wrap = <TypeDefs as super::Types>::wrap,
    >;
    pub type min_length<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        auto_correct = <TypeDefs as super::Types>::auto_correct,
        cols = <TypeDefs as super::Types>::cols,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        max_length = <TypeDefs as super::Types>::max_length,
        min_length = Value,
        name = <TypeDefs as super::Types>::name,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        rows = <TypeDefs as super::Types>::rows,
        wrap = <TypeDefs as super::Types>::wrap,
    >;
    pub type name<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        auto_correct = <TypeDefs as super::Types>::auto_correct,
        cols = <TypeDefs as super::Types>::cols,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        max_length = <TypeDefs as super::Types>::max_length,
        min_length = <TypeDefs as super::Types>::min_length,
        name = Value,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        rows = <TypeDefs as super::Types>::rows,
        wrap = <TypeDefs as super::Types>::wrap,
    >;
    pub type placeholder<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        auto_correct = <TypeDefs as super::Types>::auto_correct,
        cols = <TypeDefs as super::Types>::cols,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        max_length = <TypeDefs as super::Types>::max_length,
        min_length = <TypeDefs as super::Types>::min_length,
        name = <TypeDefs as super::Types>::name,
        placeholder = Value,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        rows = <TypeDefs as super::Types>::rows,
        wrap = <TypeDefs as super::Types>::wrap,
    >;
    pub type read_only<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        auto_correct = <TypeDefs as super::Types>::auto_correct,
        cols = <TypeDefs as super::Types>::cols,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        max_length = <TypeDefs as super::Types>::max_length,
        min_length = <TypeDefs as super::Types>::min_length,
        name = <TypeDefs as super::Types>::name,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = Value,
        required = <TypeDefs as super::Types>::required,
        rows = <TypeDefs as super::Types>::rows,
        wrap = <TypeDefs as super::Types>::wrap,
    >;
    pub type required<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        auto_correct = <TypeDefs as super::Types>::auto_correct,
        cols = <TypeDefs as super::Types>::cols,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        max_length = <TypeDefs as super::Types>::max_length,
        min_length = <TypeDefs as super::Types>::min_length,
        name = <TypeDefs as super::Types>::name,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = Value,
        rows = <TypeDefs as super::Types>::rows,
        wrap = <TypeDefs as super::Types>::wrap,
    >;
    pub type rows<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        auto_correct = <TypeDefs as super::Types>::auto_correct,
        cols = <TypeDefs as super::Types>::cols,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        max_length = <TypeDefs as super::Types>::max_length,
        min_length = <TypeDefs as super::Types>::min_length,
        name = <TypeDefs as super::Types>::name,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        rows = Value,
        wrap = <TypeDefs as super::Types>::wrap,
    >;
    pub type wrap<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        auto_correct = <TypeDefs as super::Types>::auto_correct,
        cols = <TypeDefs as super::Types>::cols,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        max_length = <TypeDefs as super::Types>::max_length,
        min_length = <TypeDefs as super::Types>::min_length,
        name = <TypeDefs as super::Types>::name,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        rows = <TypeDefs as super::Types>::rows,
        wrap = Value,
    >;
}
mod trait_types {
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub trait Types {
        type HtmlElementProps: ?::core::marker::Sized + HtmlElementProps::Types;
        type auto_complete: crate::MaybeUpdateValueByRef<str>;
        type auto_correct: crate::MaybeUpdateValueByRef<str>;
        type cols: crate::MaybeUpdateValue<u32>;
        type disabled: crate::MaybeUpdateValue<bool>;
        type form: crate::MaybeUpdateValueByRef<str>;
        type max_length: crate::MaybeUpdateValue<i32>;
        type min_length: crate::MaybeUpdateValue<i32>;
        type name: crate::MaybeUpdateValueByRef<str>;
        type placeholder: crate::MaybeUpdateValueByRef<str>;
        type read_only: crate::MaybeUpdateValue<bool>;
        type required: crate::MaybeUpdateValue<bool>;
        type rows: crate::MaybeUpdateValue<u32>;
        type wrap: crate::MaybeUpdateValueByRef<str>;
    }
}
pub use trait_types::Types;
pub use trait_types::Types as ValidTypes;
pub mod data_struct {
    #[non_exhaustive]
    pub struct HtmlTextAreaElementProps<TypeDefs: super::Types + ?::core::marker::Sized> {
        pub HtmlElementProps: super::super::HtmlElementProps::Data<TypeDefs::HtmlElementProps>,
        pub auto_complete: TypeDefs::auto_complete,
        pub auto_correct: TypeDefs::auto_correct,
        pub cols: TypeDefs::cols,
        pub disabled: TypeDefs::disabled,
        pub form: TypeDefs::form,
        pub max_length: TypeDefs::max_length,
        pub min_length: TypeDefs::min_length,
        pub name: TypeDefs::name,
        pub placeholder: TypeDefs::placeholder,
        pub read_only: TypeDefs::read_only,
        pub required: TypeDefs::required,
        pub rows: TypeDefs::rows,
        pub wrap: TypeDefs::wrap,
    }
}
pub use data_struct::HtmlTextAreaElementProps as Data;
pub struct Building<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
pub struct Replacing<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
mod types_initial {
    #[allow(unused_imports)]
    use super::super::*;
    pub type TypesInitial = dyn super::Types<
        HtmlElementProps = HtmlElementProps::TypesInitial,
        auto_complete = (),
        auto_correct = (),
        cols = (),
        disabled = (),
        form = (),
        max_length = (),
        min_length = (),
        name = (),
        placeholder = (),
        read_only = (),
        required = (),
        rows = (),
        wrap = (),
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
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
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
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
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
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
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
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
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
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
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
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
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
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
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
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
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
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
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
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
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
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
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
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
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
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
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
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
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
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
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
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
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
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
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
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
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
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
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
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
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
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
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
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
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
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
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
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
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
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
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
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
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
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
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
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
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
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
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
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
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
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
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
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
            })
        }
        #[inline]
        pub fn auto_complete<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            auto_complete: V,
        ) -> super::Building<super::overwrite::auto_complete<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
            })
        }
        #[inline]
        pub fn auto_correct<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            auto_correct: V,
        ) -> super::Building<super::overwrite::auto_correct<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                auto_complete: self.0.auto_complete,
                auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
            })
        }
        #[inline]
        pub fn cols<V: crate::MaybeUpdateValue<u32>>(
            self,
            cols: V,
        ) -> super::Building<super::overwrite::cols<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
            })
        }
        #[inline]
        pub fn disabled<V: crate::MaybeUpdateValue<bool>>(
            self,
            disabled: V,
        ) -> super::Building<super::overwrite::disabled<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
            })
        }
        #[inline]
        pub fn form<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            form: V,
        ) -> super::Building<super::overwrite::form<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
            })
        }
        #[inline]
        pub fn max_length<V: crate::MaybeUpdateValue<i32>>(
            self,
            max_length: V,
        ) -> super::Building<super::overwrite::max_length<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
            })
        }
        #[inline]
        pub fn min_length<V: crate::MaybeUpdateValue<i32>>(
            self,
            min_length: V,
        ) -> super::Building<super::overwrite::min_length<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
            })
        }
        #[inline]
        pub fn name<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            name: V,
        ) -> super::Building<super::overwrite::name<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
            })
        }
        #[inline]
        pub fn placeholder<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            placeholder: V,
        ) -> super::Building<super::overwrite::placeholder<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
            })
        }
        #[inline]
        pub fn read_only<V: crate::MaybeUpdateValue<bool>>(
            self,
            read_only: V,
        ) -> super::Building<super::overwrite::read_only<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap: self.0.wrap,
            })
        }
        #[inline]
        pub fn required<V: crate::MaybeUpdateValue<bool>>(
            self,
            required: V,
        ) -> super::Building<super::overwrite::required<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required,
                rows: self.0.rows,
                wrap: self.0.wrap,
            })
        }
        #[inline]
        pub fn rows<V: crate::MaybeUpdateValue<u32>>(
            self,
            rows: V,
        ) -> super::Building<super::overwrite::rows<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows,
                wrap: self.0.wrap,
            })
        }
        #[inline]
        pub fn wrap<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            wrap: V,
        ) -> super::Building<super::overwrite::wrap<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                auto_complete: self.0.auto_complete,
                auto_correct: self.0.auto_correct,
                cols: self.0.cols,
                disabled: self.0.disabled,
                form: self.0.form,
                max_length: self.0.max_length,
                min_length: self.0.min_length,
                name: self.0.name,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                rows: self.0.rows,
                wrap,
            })
        }
    }
}
#[cfg(feature = "dom")]
mod impl_update_element {
    #[allow(unused_imports)]
    use super::super::*;
    impl<TypeDefs: ?::core::marker::Sized + super::Types>
        crate::props::UpdateElement<web_sys::HtmlTextAreaElement> for super::Data<TypeDefs>
    where
        HtmlElementProps::Data<TypeDefs::HtmlElementProps>:
            crate::props::UpdateElement<web_sys::HtmlElement>,
    {
        type State =
            <HtmlElementProps::Data<TypeDefs::HtmlElementProps> as crate::props::UpdateElement<
                web_sys::HtmlElement,
            >>::State;
        fn initialize_state(
            this: Self,
            element: &web_sys::HtmlTextAreaElement,
            children_ctx: &mut ::frender_dom::Dom,
        ) -> Self::State {
            let dom_element: &::web_sys::Element = element.as_ref();
            < TypeDefs :: auto_complete as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . auto_complete , | v | element . set_autocomplete (v) , | | dom_element . remove_attribute ("autocomplete") . unwrap () ,) ;
            < TypeDefs :: auto_correct as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . auto_correct , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , "auto_correct" ,) , | | dom_element . remove_attribute ("auto_correct") . unwrap () ,) ;
            <TypeDefs::cols as ::frender_dom::props::MaybeUpdateValue<u32>>::maybe_update_value(
                this.cols,
                |v| element.set_cols(v),
                || dom_element.remove_attribute("cols").unwrap(),
            );
            < TypeDefs :: disabled as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (this . disabled , | v | element . set_disabled (v) , | | dom_element . remove_attribute ("disabled") . unwrap () ,) ;
            < TypeDefs :: form as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . form , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , "form" ,) , | | dom_element . remove_attribute ("form") . unwrap () ,) ;
            < TypeDefs :: max_length as :: frender_dom :: props :: MaybeUpdateValue < i32 , > > :: maybe_update_value (this . max_length , | v | element . set_max_length (v) , | | dom_element . remove_attribute ("maxlength") . unwrap () ,) ;
            < TypeDefs :: min_length as :: frender_dom :: props :: MaybeUpdateValue < i32 , > > :: maybe_update_value (this . min_length , | v | element . set_min_length (v) , | | dom_element . remove_attribute ("minlength") . unwrap () ,) ;
            < TypeDefs :: name as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . name , | v | element . set_name (v) , | | dom_element . remove_attribute ("name") . unwrap () ,) ;
            < TypeDefs :: placeholder as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . placeholder , | v | element . set_placeholder (v) , | | dom_element . remove_attribute ("placeholder") . unwrap () ,) ;
            < TypeDefs :: read_only as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (this . read_only , | v | element . set_read_only (v) , | | dom_element . remove_attribute ("readonly") . unwrap () ,) ;
            < TypeDefs :: required as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (this . required , | v | element . set_required (v) , | | dom_element . remove_attribute ("required") . unwrap () ,) ;
            <TypeDefs::rows as ::frender_dom::props::MaybeUpdateValue<u32>>::maybe_update_value(
                this.rows,
                |v| element.set_rows(v),
                || dom_element.remove_attribute("rows").unwrap(),
            );
            < TypeDefs :: wrap as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . wrap , | v | element . set_wrap (v) , | | dom_element . remove_attribute ("wrap") . unwrap () ,) ;
            <HtmlElementProps::Data<TypeDefs::HtmlElementProps> as crate::props::UpdateElement<
                web_sys::HtmlElement,
            >>::initialize_state(this.HtmlElementProps, element, children_ctx)
        }
        fn update_element(
            this: Self,
            element: &web_sys::HtmlTextAreaElement,
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
            < TypeDefs :: auto_complete as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . auto_complete , | v | element . set_autocomplete (v) , | | dom_element . remove_attribute ("autocomplete") . unwrap () ,) ;
            < TypeDefs :: auto_correct as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . auto_correct , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , "auto_correct" ,) , | | dom_element . remove_attribute ("auto_correct") . unwrap () ,) ;
            <TypeDefs::cols as ::frender_dom::props::MaybeUpdateValue<u32>>::maybe_update_value(
                this.cols,
                |v| element.set_cols(v),
                || dom_element.remove_attribute("cols").unwrap(),
            );
            < TypeDefs :: disabled as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (this . disabled , | v | element . set_disabled (v) , | | dom_element . remove_attribute ("disabled") . unwrap () ,) ;
            < TypeDefs :: form as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . form , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , "form" ,) , | | dom_element . remove_attribute ("form") . unwrap () ,) ;
            < TypeDefs :: max_length as :: frender_dom :: props :: MaybeUpdateValue < i32 , > > :: maybe_update_value (this . max_length , | v | element . set_max_length (v) , | | dom_element . remove_attribute ("maxlength") . unwrap () ,) ;
            < TypeDefs :: min_length as :: frender_dom :: props :: MaybeUpdateValue < i32 , > > :: maybe_update_value (this . min_length , | v | element . set_min_length (v) , | | dom_element . remove_attribute ("minlength") . unwrap () ,) ;
            < TypeDefs :: name as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . name , | v | element . set_name (v) , | | dom_element . remove_attribute ("name") . unwrap () ,) ;
            < TypeDefs :: placeholder as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . placeholder , | v | element . set_placeholder (v) , | | dom_element . remove_attribute ("placeholder") . unwrap () ,) ;
            < TypeDefs :: read_only as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (this . read_only , | v | element . set_read_only (v) , | | dom_element . remove_attribute ("readonly") . unwrap () ,) ;
            < TypeDefs :: required as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (this . required , | v | element . set_required (v) , | | dom_element . remove_attribute ("required") . unwrap () ,) ;
            <TypeDefs::rows as ::frender_dom::props::MaybeUpdateValue<u32>>::maybe_update_value(
                this.rows,
                |v| element.set_rows(v),
                || dom_element.remove_attribute("rows").unwrap(),
            );
            < TypeDefs :: wrap as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . wrap , | v | element . set_wrap (v) , | | dom_element . remove_attribute ("wrap") . unwrap () ,) ;
        }
    }
}
