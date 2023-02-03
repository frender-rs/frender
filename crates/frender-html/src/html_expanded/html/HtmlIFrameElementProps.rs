#[allow(non_snake_case)]
pub fn HtmlIFrameElementProps() -> Building<TypesInitial> {
    #[allow(unused_imports)]
    use super::*;
    self::Building(self::Data {
        HtmlElementProps: HtmlElementProps::build(HtmlElementProps()),
        allow: (),
        allow_fullscreen: (),
        allow_payment_request: (),
        csp: (),
        fetch_priority: (),
        height: (),
        loading: (),
        name: (),
        referrer_policy: (),
        sandbox: (),
        src: (),
        src_doc: (),
        width: (),
    })
}
pub mod prelude {}
pub mod overwrite {
    #![allow(non_camel_case_types)]
    pub type HtmlElementProps<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = Value,
        allow = <TypeDefs as super::Types>::allow,
        allow_fullscreen = <TypeDefs as super::Types>::allow_fullscreen,
        allow_payment_request = <TypeDefs as super::Types>::allow_payment_request,
        csp = <TypeDefs as super::Types>::csp,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        height = <TypeDefs as super::Types>::height,
        loading = <TypeDefs as super::Types>::loading,
        name = <TypeDefs as super::Types>::name,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        sandbox = <TypeDefs as super::Types>::sandbox,
        src = <TypeDefs as super::Types>::src,
        src_doc = <TypeDefs as super::Types>::src_doc,
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
    pub type allow<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        allow = Value,
        allow_fullscreen = <TypeDefs as super::Types>::allow_fullscreen,
        allow_payment_request = <TypeDefs as super::Types>::allow_payment_request,
        csp = <TypeDefs as super::Types>::csp,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        height = <TypeDefs as super::Types>::height,
        loading = <TypeDefs as super::Types>::loading,
        name = <TypeDefs as super::Types>::name,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        sandbox = <TypeDefs as super::Types>::sandbox,
        src = <TypeDefs as super::Types>::src,
        src_doc = <TypeDefs as super::Types>::src_doc,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type allow_fullscreen<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        allow = <TypeDefs as super::Types>::allow,
        allow_fullscreen = Value,
        allow_payment_request = <TypeDefs as super::Types>::allow_payment_request,
        csp = <TypeDefs as super::Types>::csp,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        height = <TypeDefs as super::Types>::height,
        loading = <TypeDefs as super::Types>::loading,
        name = <TypeDefs as super::Types>::name,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        sandbox = <TypeDefs as super::Types>::sandbox,
        src = <TypeDefs as super::Types>::src,
        src_doc = <TypeDefs as super::Types>::src_doc,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type allow_payment_request<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        allow = <TypeDefs as super::Types>::allow,
        allow_fullscreen = <TypeDefs as super::Types>::allow_fullscreen,
        allow_payment_request = Value,
        csp = <TypeDefs as super::Types>::csp,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        height = <TypeDefs as super::Types>::height,
        loading = <TypeDefs as super::Types>::loading,
        name = <TypeDefs as super::Types>::name,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        sandbox = <TypeDefs as super::Types>::sandbox,
        src = <TypeDefs as super::Types>::src,
        src_doc = <TypeDefs as super::Types>::src_doc,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type csp<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        allow = <TypeDefs as super::Types>::allow,
        allow_fullscreen = <TypeDefs as super::Types>::allow_fullscreen,
        allow_payment_request = <TypeDefs as super::Types>::allow_payment_request,
        csp = Value,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        height = <TypeDefs as super::Types>::height,
        loading = <TypeDefs as super::Types>::loading,
        name = <TypeDefs as super::Types>::name,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        sandbox = <TypeDefs as super::Types>::sandbox,
        src = <TypeDefs as super::Types>::src,
        src_doc = <TypeDefs as super::Types>::src_doc,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type fetch_priority<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        allow = <TypeDefs as super::Types>::allow,
        allow_fullscreen = <TypeDefs as super::Types>::allow_fullscreen,
        allow_payment_request = <TypeDefs as super::Types>::allow_payment_request,
        csp = <TypeDefs as super::Types>::csp,
        fetch_priority = Value,
        height = <TypeDefs as super::Types>::height,
        loading = <TypeDefs as super::Types>::loading,
        name = <TypeDefs as super::Types>::name,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        sandbox = <TypeDefs as super::Types>::sandbox,
        src = <TypeDefs as super::Types>::src,
        src_doc = <TypeDefs as super::Types>::src_doc,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type height<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        allow = <TypeDefs as super::Types>::allow,
        allow_fullscreen = <TypeDefs as super::Types>::allow_fullscreen,
        allow_payment_request = <TypeDefs as super::Types>::allow_payment_request,
        csp = <TypeDefs as super::Types>::csp,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        height = Value,
        loading = <TypeDefs as super::Types>::loading,
        name = <TypeDefs as super::Types>::name,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        sandbox = <TypeDefs as super::Types>::sandbox,
        src = <TypeDefs as super::Types>::src,
        src_doc = <TypeDefs as super::Types>::src_doc,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type loading<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        allow = <TypeDefs as super::Types>::allow,
        allow_fullscreen = <TypeDefs as super::Types>::allow_fullscreen,
        allow_payment_request = <TypeDefs as super::Types>::allow_payment_request,
        csp = <TypeDefs as super::Types>::csp,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        height = <TypeDefs as super::Types>::height,
        loading = Value,
        name = <TypeDefs as super::Types>::name,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        sandbox = <TypeDefs as super::Types>::sandbox,
        src = <TypeDefs as super::Types>::src,
        src_doc = <TypeDefs as super::Types>::src_doc,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type name<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        allow = <TypeDefs as super::Types>::allow,
        allow_fullscreen = <TypeDefs as super::Types>::allow_fullscreen,
        allow_payment_request = <TypeDefs as super::Types>::allow_payment_request,
        csp = <TypeDefs as super::Types>::csp,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        height = <TypeDefs as super::Types>::height,
        loading = <TypeDefs as super::Types>::loading,
        name = Value,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        sandbox = <TypeDefs as super::Types>::sandbox,
        src = <TypeDefs as super::Types>::src,
        src_doc = <TypeDefs as super::Types>::src_doc,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type referrer_policy<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        allow = <TypeDefs as super::Types>::allow,
        allow_fullscreen = <TypeDefs as super::Types>::allow_fullscreen,
        allow_payment_request = <TypeDefs as super::Types>::allow_payment_request,
        csp = <TypeDefs as super::Types>::csp,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        height = <TypeDefs as super::Types>::height,
        loading = <TypeDefs as super::Types>::loading,
        name = <TypeDefs as super::Types>::name,
        referrer_policy = Value,
        sandbox = <TypeDefs as super::Types>::sandbox,
        src = <TypeDefs as super::Types>::src,
        src_doc = <TypeDefs as super::Types>::src_doc,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type sandbox<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        allow = <TypeDefs as super::Types>::allow,
        allow_fullscreen = <TypeDefs as super::Types>::allow_fullscreen,
        allow_payment_request = <TypeDefs as super::Types>::allow_payment_request,
        csp = <TypeDefs as super::Types>::csp,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        height = <TypeDefs as super::Types>::height,
        loading = <TypeDefs as super::Types>::loading,
        name = <TypeDefs as super::Types>::name,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        sandbox = Value,
        src = <TypeDefs as super::Types>::src,
        src_doc = <TypeDefs as super::Types>::src_doc,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type src<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        allow = <TypeDefs as super::Types>::allow,
        allow_fullscreen = <TypeDefs as super::Types>::allow_fullscreen,
        allow_payment_request = <TypeDefs as super::Types>::allow_payment_request,
        csp = <TypeDefs as super::Types>::csp,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        height = <TypeDefs as super::Types>::height,
        loading = <TypeDefs as super::Types>::loading,
        name = <TypeDefs as super::Types>::name,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        sandbox = <TypeDefs as super::Types>::sandbox,
        src = Value,
        src_doc = <TypeDefs as super::Types>::src_doc,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type src_doc<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        allow = <TypeDefs as super::Types>::allow,
        allow_fullscreen = <TypeDefs as super::Types>::allow_fullscreen,
        allow_payment_request = <TypeDefs as super::Types>::allow_payment_request,
        csp = <TypeDefs as super::Types>::csp,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        height = <TypeDefs as super::Types>::height,
        loading = <TypeDefs as super::Types>::loading,
        name = <TypeDefs as super::Types>::name,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        sandbox = <TypeDefs as super::Types>::sandbox,
        src = <TypeDefs as super::Types>::src,
        src_doc = Value,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type width<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        allow = <TypeDefs as super::Types>::allow,
        allow_fullscreen = <TypeDefs as super::Types>::allow_fullscreen,
        allow_payment_request = <TypeDefs as super::Types>::allow_payment_request,
        csp = <TypeDefs as super::Types>::csp,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        height = <TypeDefs as super::Types>::height,
        loading = <TypeDefs as super::Types>::loading,
        name = <TypeDefs as super::Types>::name,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        sandbox = <TypeDefs as super::Types>::sandbox,
        src = <TypeDefs as super::Types>::src,
        src_doc = <TypeDefs as super::Types>::src_doc,
        width = Value,
    >;
}
mod trait_types {
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub trait Types {
        type HtmlElementProps: ?::core::marker::Sized + HtmlElementProps::Types;
        type allow: crate::MaybeUpdateValue<str>;
        type allow_fullscreen: crate::MaybeUpdateValue<bool>;
        type allow_payment_request: crate::MaybeUpdateValue<bool>;
        type csp: crate::MaybeUpdateValue<str>;
        type fetch_priority: crate::MaybeUpdateValue<str>;
        type height: crate::MaybeUpdateValue<str>;
        type loading: crate::MaybeUpdateValue<str>;
        type name: crate::MaybeUpdateValue<str>;
        type referrer_policy: crate::MaybeUpdateValue<str>;
        type sandbox: crate::MaybeUpdateValue<str>;
        type src: crate::MaybeUpdateValue<str>;
        type src_doc: crate::MaybeUpdateValue<str>;
        type width: crate::MaybeUpdateValue<str>;
    }
}
pub use trait_types::Types;
pub use trait_types::Types as ValidTypes;
pub mod data_struct {
    #[non_exhaustive]
    pub struct HtmlIFrameElementProps<TypeDefs: super::Types + ?::core::marker::Sized> {
        pub HtmlElementProps: super::super::HtmlElementProps::Data<TypeDefs::HtmlElementProps>,
        pub allow: TypeDefs::allow,
        pub allow_fullscreen: TypeDefs::allow_fullscreen,
        pub allow_payment_request: TypeDefs::allow_payment_request,
        pub csp: TypeDefs::csp,
        pub fetch_priority: TypeDefs::fetch_priority,
        pub height: TypeDefs::height,
        pub loading: TypeDefs::loading,
        pub name: TypeDefs::name,
        pub referrer_policy: TypeDefs::referrer_policy,
        pub sandbox: TypeDefs::sandbox,
        pub src: TypeDefs::src,
        pub src_doc: TypeDefs::src_doc,
        pub width: TypeDefs::width,
    }
}
pub use data_struct::HtmlIFrameElementProps as Data;
pub struct Building<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
pub struct Replacing<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
mod types_initial {
    #[allow(unused_imports)]
    use super::super::*;
    pub type TypesInitial = dyn super::Types<
        HtmlElementProps = HtmlElementProps::TypesInitial,
        allow = (),
        allow_fullscreen = (),
        allow_payment_request = (),
        csp = (),
        fetch_priority = (),
        height = (),
        loading = (),
        name = (),
        referrer_policy = (),
        sandbox = (),
        src = (),
        src_doc = (),
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
        type allow: ::core::default::Default;
        type allow_fullscreen: ::core::default::Default;
        type allow_payment_request: ::core::default::Default;
        type csp: ::core::default::Default;
        type fetch_priority: ::core::default::Default;
        type height: ::core::default::Default;
        type loading: ::core::default::Default;
        type name: ::core::default::Default;
        type referrer_policy: ::core::default::Default;
        type sandbox: ::core::default::Default;
        type src: ::core::default::Default;
        type src_doc: ::core::default::Default;
        type width: ::core::default::Default;
    }
    pub struct RenderState<TypeDefs: RenderStateTypes>
    where
        TypeDefs: ?::core::marker::Sized,
    {
        pub HtmlElementProps: TypeDefs::HtmlElementProps,
        pub allow: TypeDefs::allow,
        pub allow_fullscreen: TypeDefs::allow_fullscreen,
        pub allow_payment_request: TypeDefs::allow_payment_request,
        pub csp: TypeDefs::csp,
        pub fetch_priority: TypeDefs::fetch_priority,
        pub height: TypeDefs::height,
        pub loading: TypeDefs::loading,
        pub name: TypeDefs::name,
        pub referrer_policy: TypeDefs::referrer_policy,
        pub sandbox: TypeDefs::sandbox,
        pub src: TypeDefs::src,
        pub src_doc: TypeDefs::src_doc,
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
        pub allow: &'__pin mut (TypeDefs::allow),
        pub allow_fullscreen: &'__pin mut (TypeDefs::allow_fullscreen),
        pub allow_payment_request: &'__pin mut (TypeDefs::allow_payment_request),
        pub csp: &'__pin mut (TypeDefs::csp),
        pub fetch_priority: &'__pin mut (TypeDefs::fetch_priority),
        pub height: &'__pin mut (TypeDefs::height),
        pub loading: &'__pin mut (TypeDefs::loading),
        pub name: &'__pin mut (TypeDefs::name),
        pub referrer_policy: &'__pin mut (TypeDefs::referrer_policy),
        pub sandbox: &'__pin mut (TypeDefs::sandbox),
        pub src: &'__pin mut (TypeDefs::src),
        pub src_doc: &'__pin mut (TypeDefs::src_doc),
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
            pub allow: &'__pin (TypeDefs::allow),
            pub allow_fullscreen: &'__pin (TypeDefs::allow_fullscreen),
            pub allow_payment_request: &'__pin (TypeDefs::allow_payment_request),
            pub csp: &'__pin (TypeDefs::csp),
            pub fetch_priority: &'__pin (TypeDefs::fetch_priority),
            pub height: &'__pin (TypeDefs::height),
            pub loading: &'__pin (TypeDefs::loading),
            pub name: &'__pin (TypeDefs::name),
            pub referrer_policy: &'__pin (TypeDefs::referrer_policy),
            pub sandbox: &'__pin (TypeDefs::sandbox),
            pub src: &'__pin (TypeDefs::src),
            pub src_doc: &'__pin (TypeDefs::src_doc),
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
                        allow,
                        allow_fullscreen,
                        allow_payment_request,
                        csp,
                        fetch_priority,
                        height,
                        loading,
                        name,
                        referrer_policy,
                        sandbox,
                        src,
                        src_doc,
                        width,
                    } = self.get_unchecked_mut();
                    RenderStateProj {
                        HtmlElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            HtmlElementProps,
                        ),
                        allow: allow,
                        allow_fullscreen: allow_fullscreen,
                        allow_payment_request: allow_payment_request,
                        csp: csp,
                        fetch_priority: fetch_priority,
                        height: height,
                        loading: loading,
                        name: name,
                        referrer_policy: referrer_policy,
                        sandbox: sandbox,
                        src: src,
                        src_doc: src_doc,
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
                        allow,
                        allow_fullscreen,
                        allow_payment_request,
                        csp,
                        fetch_priority,
                        height,
                        loading,
                        name,
                        referrer_policy,
                        sandbox,
                        src,
                        src_doc,
                        width,
                    } = self.get_ref();
                    ProjectionRef {
                        HtmlElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            HtmlElementProps,
                        ),
                        allow: allow,
                        allow_fullscreen: allow_fullscreen,
                        allow_payment_request: allow_payment_request,
                        csp: csp,
                        fetch_priority: fetch_priority,
                        height: height,
                        loading: loading,
                        name: name,
                        referrer_policy: referrer_policy,
                        sandbox: sandbox,
                        src: src,
                        src_doc: src_doc,
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
            allow: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::allow>,
            allow_fullscreen:
                ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::allow_fullscreen>,
            allow_payment_request:
                ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::allow_payment_request>,
            csp: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::csp>,
            fetch_priority: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::fetch_priority>,
            height: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::height>,
            loading: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::loading>,
            name: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::name>,
            referrer_policy: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::referrer_policy>,
            sandbox: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::sandbox>,
            src: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::src>,
            src_doc: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::src_doc>,
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
            let _ = &this.allow;
            let _ = &this.allow_fullscreen;
            let _ = &this.allow_payment_request;
            let _ = &this.csp;
            let _ = &this.fetch_priority;
            let _ = &this.height;
            let _ = &this.loading;
            let _ = &this.name;
            let _ = &this.referrer_policy;
            let _ = &this.sandbox;
            let _ = &this.src;
            let _ = &this.src_doc;
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
                allow: ::core::default::Default::default(),
                allow_fullscreen: ::core::default::Default::default(),
                allow_payment_request: ::core::default::Default::default(),
                csp: ::core::default::Default::default(),
                fetch_priority: ::core::default::Default::default(),
                height: ::core::default::Default::default(),
                loading: ::core::default::Default::default(),
                name: ::core::default::Default::default(),
                referrer_policy: ::core::default::Default::default(),
                sandbox: ::core::default::Default::default(),
                src: ::core::default::Default::default(),
                src_doc: ::core::default::Default::default(),
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
        #[inline]
        pub fn children<V>(
            self,
            children: V,
        ) -> super::Building<super::overwrite::children<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).children(children),
                ),
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
                width: self.0.width,
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
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
                width: self.0.width,
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
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
                width: self.0.width,
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
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
                width: self.0.width,
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
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
                width: self.0.width,
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
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
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
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
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
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
                width: self.0.width,
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
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
                width: self.0.width,
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
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
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
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
                width: self.0.width,
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
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
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
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
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
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
                width: self.0.width,
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
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
                width: self.0.width,
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
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
                width: self.0.width,
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
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
                width: self.0.width,
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
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
                width: self.0.width,
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
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
                width: self.0.width,
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
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
                width: self.0.width,
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
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
                width: self.0.width,
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
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
                width: self.0.width,
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
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
                width: self.0.width,
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
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
                width: self.0.width,
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
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
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
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
                width: self.0.width,
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
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
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
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
                width: self.0.width,
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
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
                width: self.0.width,
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
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
                width: self.0.width,
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
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
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
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn allow<V: crate::MaybeUpdateValue<str>>(
            self,
            allow: V,
        ) -> super::Building<super::overwrite::allow<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn allow_fullscreen<V: crate::MaybeUpdateValue<bool>>(
            self,
            allow_fullscreen: V,
        ) -> super::Building<super::overwrite::allow_fullscreen<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                allow: self.0.allow,
                allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn allow_payment_request<V: crate::MaybeUpdateValue<bool>>(
            self,
            allow_payment_request: V,
        ) -> super::Building<super::overwrite::allow_payment_request<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn csp<V: crate::MaybeUpdateValue<str>>(
            self,
            csp: V,
        ) -> super::Building<super::overwrite::csp<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn fetch_priority<V: crate::MaybeUpdateValue<str>>(
            self,
            fetch_priority: V,
        ) -> super::Building<super::overwrite::fetch_priority<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn height<V: crate::MaybeUpdateValue<str>>(
            self,
            height: V,
        ) -> super::Building<super::overwrite::height<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn loading<V: crate::MaybeUpdateValue<str>>(
            self,
            loading: V,
        ) -> super::Building<super::overwrite::loading<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn name<V: crate::MaybeUpdateValue<str>>(
            self,
            name: V,
        ) -> super::Building<super::overwrite::name<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn referrer_policy<V: crate::MaybeUpdateValue<str>>(
            self,
            referrer_policy: V,
        ) -> super::Building<super::overwrite::referrer_policy<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn sandbox<V: crate::MaybeUpdateValue<str>>(
            self,
            sandbox: V,
        ) -> super::Building<super::overwrite::sandbox<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn src<V: crate::MaybeUpdateValue<str>>(
            self,
            src: V,
        ) -> super::Building<super::overwrite::src<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src,
                src_doc: self.0.src_doc,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn src_doc<V: crate::MaybeUpdateValue<str>>(
            self,
            src_doc: V,
        ) -> super::Building<super::overwrite::src_doc<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn width<V: crate::MaybeUpdateValue<str>>(
            self,
            width: V,
        ) -> super::Building<super::overwrite::width<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                allow: self.0.allow,
                allow_fullscreen: self.0.allow_fullscreen,
                allow_payment_request: self.0.allow_payment_request,
                csp: self.0.csp,
                fetch_priority: self.0.fetch_priority,
                height: self.0.height,
                loading: self.0.loading,
                name: self.0.name,
                referrer_policy: self.0.referrer_policy,
                sandbox: self.0.sandbox,
                src: self.0.src,
                src_doc: self.0.src_doc,
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
        crate::props::UpdateElement<web_sys::HtmlIFrameElement> for super::Data<TypeDefs>
    where
        HtmlElementProps::Data<TypeDefs::HtmlElementProps>:
            crate::props::UpdateElement<web_sys::HtmlElement>,
    {
        type State = super :: render_state :: RenderState < dyn super :: render_state :: RenderStateTypes < HtmlElementProps = < HtmlElementProps :: Data < TypeDefs :: HtmlElementProps , > as crate :: props :: UpdateElement < web_sys :: HtmlElement > > :: State , allow = < TypeDefs :: allow as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , allow_fullscreen = < TypeDefs :: allow_fullscreen as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: State , allow_payment_request = < TypeDefs :: allow_payment_request as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: State , csp = < TypeDefs :: csp as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , fetch_priority = < TypeDefs :: fetch_priority as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , height = < TypeDefs :: height as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , loading = < TypeDefs :: loading as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , name = < TypeDefs :: name as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , referrer_policy = < TypeDefs :: referrer_policy as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , sandbox = < TypeDefs :: sandbox as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , src = < TypeDefs :: src as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , src_doc = < TypeDefs :: src_doc as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , width = < TypeDefs :: width as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: State , > , > ;
        fn update_element(
            this: Self,
            element: &web_sys::HtmlIFrameElement,
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
                data: this.allow,
                state: state.allow,
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
                    const ATTR_NAME: &::core::primitive::str = "allow";
                    < TypeDefs :: allow as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.allow_fullscreen,
                state: state.allow_fullscreen,
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
                    const ATTR_NAME: &::core::primitive::str = "allowfullscreen";
                    < TypeDefs :: allow_fullscreen as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (data , state , | v | element . set_allow_fullscreen (* v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.allow_payment_request,
                state: state.allow_payment_request,
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
                    const ATTR_NAME: &::core::primitive::str = "allowpaymentrequest";
                    <TypeDefs::allow_payment_request as ::frender_dom::props::MaybeUpdateValue<
                        bool,
                    >>::maybe_update_value(
                        data,
                        state,
                        |v| element.set_allow_payment_request(*v),
                        || dom_element.remove_attribute(ATTR_NAME).unwrap(),
                    )
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.csp,
                state: state.csp,
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
                    const ATTR_NAME: &::core::primitive::str = "csp";
                    < TypeDefs :: csp as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.fetch_priority,
                state: state.fetch_priority,
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
                    const ATTR_NAME: &::core::primitive::str = "fetchpriority";
                    < TypeDefs :: fetch_priority as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
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
                data: this.loading,
                state: state.loading,
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
                    const ATTR_NAME: &::core::primitive::str = "loading";
                    < TypeDefs :: loading as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
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
                data: this.referrer_policy,
                state: state.referrer_policy,
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
                    const ATTR_NAME: &::core::primitive::str = "referrerpolicy";
                    < TypeDefs :: referrer_policy as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | element . set_referrer_policy (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.sandbox,
                state: state.sandbox,
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
                    const ATTR_NAME: &::core::primitive::str = "sandbox";
                    < TypeDefs :: sandbox as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.src,
                state: state.src,
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
                    const ATTR_NAME: &::core::primitive::str = "src";
                    < TypeDefs :: src as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | element . set_src (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
                }
            }
            #[allow(unused_variables)]
            match (crate::props::FieldData {
                data: this.src_doc,
                state: state.src_doc,
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
                    const ATTR_NAME: &::core::primitive::str = "srcdoc";
                    < TypeDefs :: src_doc as :: frender_dom :: props :: MaybeUpdateValue < str , > > :: maybe_update_value (data , state , | v | element . set_srcdoc (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
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
