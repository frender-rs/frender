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
        type auto_complete: crate::MaybeUpdateValue<str>;
        type auto_correct: crate::MaybeUpdateValue<str>;
        type cols: crate::MaybeUpdateValue<u32>;
        type disabled: crate::MaybeUpdateValue<bool>;
        type form: crate::MaybeUpdateValue<str>;
        type max_length: crate::MaybeUpdateValue<i32>;
        type min_length: crate::MaybeUpdateValue<i32>;
        type name: crate::MaybeUpdateValue<str>;
        type placeholder: crate::MaybeUpdateValue<str>;
        type read_only: crate::MaybeUpdateValue<bool>;
        type required: crate::MaybeUpdateValue<bool>;
        type rows: crate::MaybeUpdateValue<u32>;
        type wrap: crate::MaybeUpdateValue<str>;
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
pub mod render_state {
    #[allow(non_camel_case_types)]
    pub trait RenderStateTypes {
        type HtmlElementProps: ::core::default::Default
            + crate::props::IntrinsicComponentPollReactive;
        type auto_complete: ::core::default::Default;
        type auto_correct: ::core::default::Default;
        type cols: ::core::default::Default;
        type disabled: ::core::default::Default;
        type form: ::core::default::Default;
        type max_length: ::core::default::Default;
        type min_length: ::core::default::Default;
        type name: ::core::default::Default;
        type placeholder: ::core::default::Default;
        type read_only: ::core::default::Default;
        type required: ::core::default::Default;
        type rows: ::core::default::Default;
        type wrap: ::core::default::Default;
    }
    pub struct RenderState<TypeDefs: RenderStateTypes>
    where
        TypeDefs: ?::core::marker::Sized,
    {
        pub HtmlElementProps: TypeDefs::HtmlElementProps,
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
        pub auto_complete: &'__pin mut (TypeDefs::auto_complete),
        pub auto_correct: &'__pin mut (TypeDefs::auto_correct),
        pub cols: &'__pin mut (TypeDefs::cols),
        pub disabled: &'__pin mut (TypeDefs::disabled),
        pub form: &'__pin mut (TypeDefs::form),
        pub max_length: &'__pin mut (TypeDefs::max_length),
        pub min_length: &'__pin mut (TypeDefs::min_length),
        pub name: &'__pin mut (TypeDefs::name),
        pub placeholder: &'__pin mut (TypeDefs::placeholder),
        pub read_only: &'__pin mut (TypeDefs::read_only),
        pub required: &'__pin mut (TypeDefs::required),
        pub rows: &'__pin mut (TypeDefs::rows),
        pub wrap: &'__pin mut (TypeDefs::wrap),
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
            pub auto_complete: &'__pin (TypeDefs::auto_complete),
            pub auto_correct: &'__pin (TypeDefs::auto_correct),
            pub cols: &'__pin (TypeDefs::cols),
            pub disabled: &'__pin (TypeDefs::disabled),
            pub form: &'__pin (TypeDefs::form),
            pub max_length: &'__pin (TypeDefs::max_length),
            pub min_length: &'__pin (TypeDefs::min_length),
            pub name: &'__pin (TypeDefs::name),
            pub placeholder: &'__pin (TypeDefs::placeholder),
            pub read_only: &'__pin (TypeDefs::read_only),
            pub required: &'__pin (TypeDefs::required),
            pub rows: &'__pin (TypeDefs::rows),
            pub wrap: &'__pin (TypeDefs::wrap),
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
                        auto_complete,
                        auto_correct,
                        cols,
                        disabled,
                        form,
                        max_length,
                        min_length,
                        name,
                        placeholder,
                        read_only,
                        required,
                        rows,
                        wrap,
                    } = self.get_unchecked_mut();
                    RenderStateProj {
                        HtmlElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            HtmlElementProps,
                        ),
                        auto_complete: auto_complete,
                        auto_correct: auto_correct,
                        cols: cols,
                        disabled: disabled,
                        form: form,
                        max_length: max_length,
                        min_length: min_length,
                        name: name,
                        placeholder: placeholder,
                        read_only: read_only,
                        required: required,
                        rows: rows,
                        wrap: wrap,
                    }
                }
            }
            pub(crate) fn project_ref<'__pin>(
                self: ::pin_project_lite::__private::Pin<&'__pin Self>,
            ) -> ProjectionRef<'__pin, TypeDefs> {
                unsafe {
                    let Self {
                        HtmlElementProps,
                        auto_complete,
                        auto_correct,
                        cols,
                        disabled,
                        form,
                        max_length,
                        min_length,
                        name,
                        placeholder,
                        read_only,
                        required,
                        rows,
                        wrap,
                    } = self.get_ref();
                    ProjectionRef {
                        HtmlElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            HtmlElementProps,
                        ),
                        auto_complete: auto_complete,
                        auto_correct: auto_correct,
                        cols: cols,
                        disabled: disabled,
                        form: form,
                        max_length: max_length,
                        min_length: min_length,
                        name: name,
                        placeholder: placeholder,
                        read_only: read_only,
                        required: required,
                        rows: rows,
                        wrap: wrap,
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
            auto_complete: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::auto_complete>,
            auto_correct: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::auto_correct>,
            cols: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::cols>,
            disabled: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::disabled>,
            form: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::form>,
            max_length: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::max_length>,
            min_length: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::min_length>,
            name: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::name>,
            placeholder: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::placeholder>,
            read_only: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::read_only>,
            required: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::required>,
            rows: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::rows>,
            wrap: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::wrap>,
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
            let _ = &this.auto_complete;
            let _ = &this.auto_correct;
            let _ = &this.cols;
            let _ = &this.disabled;
            let _ = &this.form;
            let _ = &this.max_length;
            let _ = &this.min_length;
            let _ = &this.name;
            let _ = &this.placeholder;
            let _ = &this.read_only;
            let _ = &this.required;
            let _ = &this.rows;
            let _ = &this.wrap;
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
                auto_complete: ::core::default::Default::default(),
                auto_correct: ::core::default::Default::default(),
                cols: ::core::default::Default::default(),
                disabled: ::core::default::Default::default(),
                form: ::core::default::Default::default(),
                max_length: ::core::default::Default::default(),
                min_length: ::core::default::Default::default(),
                name: ::core::default::Default::default(),
                placeholder: ::core::default::Default::default(),
                read_only: ::core::default::Default::default(),
                required: ::core::default::Default::default(),
                rows: ::core::default::Default::default(),
                wrap: ::core::default::Default::default(),
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
        pub fn class<V: crate::MaybeUpdateValue<str>>(
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
        pub fn id<V: crate::MaybeUpdateValue<str>>(
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
        pub fn part<V: crate::MaybeUpdateValue<str>>(
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
        pub fn access_key<V: crate::MaybeUpdateValue<str>>(
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
        pub fn auto_capitalize<V: crate::MaybeUpdateValue<str>>(
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
        pub fn context_menu<V: crate::MaybeUpdateValue<str>>(
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
        pub fn dir<V: crate::MaybeUpdateValue<str>>(
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
        pub fn enter_key_hint<V: crate::MaybeUpdateValue<str>>(
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
        pub fn input_mode<V: crate::MaybeUpdateValue<str>>(
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
        pub fn is<V: crate::MaybeUpdateValue<str>>(
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
        pub fn item_id<V: crate::MaybeUpdateValue<str>>(
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
        pub fn item_prop<V: crate::MaybeUpdateValue<str>>(
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
        pub fn item_ref<V: crate::MaybeUpdateValue<str>>(
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
        pub fn item_scope<V: crate::MaybeUpdateValue<str>>(
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
        pub fn item_type<V: crate::MaybeUpdateValue<str>>(
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
        pub fn lang<V: crate::MaybeUpdateValue<str>>(
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
        pub fn nonce<V: crate::MaybeUpdateValue<str>>(
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
        pub fn role<V: crate::MaybeUpdateValue<str>>(
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
        pub fn slot<V: crate::MaybeUpdateValue<str>>(
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
        pub fn style<V: crate::MaybeUpdateValue<str>>(
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
        pub fn title<V: crate::MaybeUpdateValue<str>>(
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
        pub fn translate<V: crate::MaybeUpdateValue<str>>(
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
        pub fn virtual_keyboard_policy<V: crate::MaybeUpdateValue<str>>(
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
        pub fn auto_complete<V: crate::MaybeUpdateValue<str>>(
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
        pub fn auto_correct<V: crate::MaybeUpdateValue<str>>(
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
        pub fn form<V: crate::MaybeUpdateValue<str>>(
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
        pub fn name<V: crate::MaybeUpdateValue<str>>(
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
        pub fn placeholder<V: crate::MaybeUpdateValue<str>>(
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
        pub fn wrap<V: crate::MaybeUpdateValue<str>>(
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
        type State = super :: render_state :: RenderState < dyn super :: render_state :: RenderStateTypes < HtmlElementProps = < HtmlElementProps :: Data < TypeDefs :: HtmlElementProps , > as crate :: props :: UpdateElement < web_sys :: HtmlElement > > :: State , auto_complete = < TypeDefs :: auto_complete as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , auto_correct = < TypeDefs :: auto_correct as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , cols = < TypeDefs :: cols as :: frender_dom :: props :: MaybeUpdateValue < u32 , > > :: State , disabled = < TypeDefs :: disabled as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: State , form = < TypeDefs :: form as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , max_length = < TypeDefs :: max_length as :: frender_dom :: props :: MaybeUpdateValue < i32 , > > :: State , min_length = < TypeDefs :: min_length as :: frender_dom :: props :: MaybeUpdateValue < i32 , > > :: State , name = < TypeDefs :: name as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , placeholder = < TypeDefs :: placeholder as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , read_only = < TypeDefs :: read_only as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: State , required = < TypeDefs :: required as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: State , rows = < TypeDefs :: rows as :: frender_dom :: props :: MaybeUpdateValue < u32 , > > :: State , wrap = < TypeDefs :: wrap as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , > , > ;
        fn update_element(
            this: Self,
            element: &web_sys::HtmlTextAreaElement,
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
                data: this.auto_complete,
                state: state.auto_complete,
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
                    const ATTR_NAME: &::core::primitive::str = "autocomplete";
                    < TypeDefs :: auto_complete as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | element . set_autocomplete (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.auto_correct,
                state: state.auto_correct,
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
                    const ATTR_NAME: &::core::primitive::str = "auto_correct";
                    < TypeDefs :: auto_correct as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.cols,
                state: state.cols,
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
                    const ATTR_NAME: &::core::primitive::str = "cols";
                    < TypeDefs :: cols as :: frender_dom :: props :: MaybeUpdateValue < u32 , > > :: maybe_update_value (data , state , | v | element . set_cols (* v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.disabled,
                state: state.disabled,
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
                    const ATTR_NAME: &::core::primitive::str = "disabled";
                    < TypeDefs :: disabled as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (data , state , | v | element . set_disabled (* v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.form,
                state: state.form,
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
                    const ATTR_NAME: &::core::primitive::str = "form";
                    < TypeDefs :: form as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.max_length,
                state: state.max_length,
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
                    const ATTR_NAME: &::core::primitive::str = "maxlength";
                    < TypeDefs :: max_length as :: frender_dom :: props :: MaybeUpdateValue < i32 , > > :: maybe_update_value (data , state , | v | element . set_max_length (* v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.min_length,
                state: state.min_length,
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
                    const ATTR_NAME: &::core::primitive::str = "minlength";
                    < TypeDefs :: min_length as :: frender_dom :: props :: MaybeUpdateValue < i32 , > > :: maybe_update_value (data , state , | v | element . set_min_length (* v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.name,
                state: state.name,
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
                    const ATTR_NAME: &::core::primitive::str = "name";
                    < TypeDefs :: name as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | element . set_name (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.placeholder,
                state: state.placeholder,
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
                    const ATTR_NAME: &::core::primitive::str = "placeholder";
                    < TypeDefs :: placeholder as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | element . set_placeholder (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.read_only,
                state: state.read_only,
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
                    const ATTR_NAME: &::core::primitive::str = "readonly";
                    < TypeDefs :: read_only as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (data , state , | v | element . set_read_only (* v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.required,
                state: state.required,
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
                    const ATTR_NAME: &::core::primitive::str = "required";
                    < TypeDefs :: required as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (data , state , | v | element . set_required (* v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.rows,
                state: state.rows,
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
                    const ATTR_NAME: &::core::primitive::str = "rows";
                    < TypeDefs :: rows as :: frender_dom :: props :: MaybeUpdateValue < u32 , > > :: maybe_update_value (data , state , | v | element . set_rows (* v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.wrap,
                state: state.wrap,
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
                    const ATTR_NAME: &::core::primitive::str = "wrap";
                    < TypeDefs :: wrap as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | element . set_wrap (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
        }
    }
}
