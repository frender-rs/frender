#[allow(non_snake_case)]
#[inline(always)]
pub fn HtmlLinkElementProps() -> Building<TypesInitial> {
    #[allow(unused_imports)]
    use super::*;
    self::Building(self::Data {
        HtmlElementProps: HtmlElementProps::build(HtmlElementProps()),
        as_: (),
        cross_origin: (),
        fetch_priority: (),
        href: (),
        href_lang: (),
        image_sizes: (),
        image_src_set: (),
        integrity: (),
        media: (),
        prefetch: (),
        referrer_policy: (),
        rel: (),
        sizes: (),
        type_: (),
        blocking: (),
    })
}
pub mod prelude {}
pub mod overwrite {
    #![allow(non_camel_case_types)]
    pub type HtmlElementProps<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = Value,
        as_ = <TypeDefs as super::Types>::as_,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        href = <TypeDefs as super::Types>::href,
        href_lang = <TypeDefs as super::Types>::href_lang,
        image_sizes = <TypeDefs as super::Types>::image_sizes,
        image_src_set = <TypeDefs as super::Types>::image_src_set,
        integrity = <TypeDefs as super::Types>::integrity,
        media = <TypeDefs as super::Types>::media,
        prefetch = <TypeDefs as super::Types>::prefetch,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        rel = <TypeDefs as super::Types>::rel,
        sizes = <TypeDefs as super::Types>::sizes,
        type_ = <TypeDefs as super::Types>::type_,
        blocking = <TypeDefs as super::Types>::blocking,
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
    pub type as_<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        as_ = Value,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        href = <TypeDefs as super::Types>::href,
        href_lang = <TypeDefs as super::Types>::href_lang,
        image_sizes = <TypeDefs as super::Types>::image_sizes,
        image_src_set = <TypeDefs as super::Types>::image_src_set,
        integrity = <TypeDefs as super::Types>::integrity,
        media = <TypeDefs as super::Types>::media,
        prefetch = <TypeDefs as super::Types>::prefetch,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        rel = <TypeDefs as super::Types>::rel,
        sizes = <TypeDefs as super::Types>::sizes,
        type_ = <TypeDefs as super::Types>::type_,
        blocking = <TypeDefs as super::Types>::blocking,
    >;
    pub type cross_origin<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        as_ = <TypeDefs as super::Types>::as_,
        cross_origin = Value,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        href = <TypeDefs as super::Types>::href,
        href_lang = <TypeDefs as super::Types>::href_lang,
        image_sizes = <TypeDefs as super::Types>::image_sizes,
        image_src_set = <TypeDefs as super::Types>::image_src_set,
        integrity = <TypeDefs as super::Types>::integrity,
        media = <TypeDefs as super::Types>::media,
        prefetch = <TypeDefs as super::Types>::prefetch,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        rel = <TypeDefs as super::Types>::rel,
        sizes = <TypeDefs as super::Types>::sizes,
        type_ = <TypeDefs as super::Types>::type_,
        blocking = <TypeDefs as super::Types>::blocking,
    >;
    pub type fetch_priority<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        as_ = <TypeDefs as super::Types>::as_,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        fetch_priority = Value,
        href = <TypeDefs as super::Types>::href,
        href_lang = <TypeDefs as super::Types>::href_lang,
        image_sizes = <TypeDefs as super::Types>::image_sizes,
        image_src_set = <TypeDefs as super::Types>::image_src_set,
        integrity = <TypeDefs as super::Types>::integrity,
        media = <TypeDefs as super::Types>::media,
        prefetch = <TypeDefs as super::Types>::prefetch,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        rel = <TypeDefs as super::Types>::rel,
        sizes = <TypeDefs as super::Types>::sizes,
        type_ = <TypeDefs as super::Types>::type_,
        blocking = <TypeDefs as super::Types>::blocking,
    >;
    pub type href<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        as_ = <TypeDefs as super::Types>::as_,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        href = Value,
        href_lang = <TypeDefs as super::Types>::href_lang,
        image_sizes = <TypeDefs as super::Types>::image_sizes,
        image_src_set = <TypeDefs as super::Types>::image_src_set,
        integrity = <TypeDefs as super::Types>::integrity,
        media = <TypeDefs as super::Types>::media,
        prefetch = <TypeDefs as super::Types>::prefetch,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        rel = <TypeDefs as super::Types>::rel,
        sizes = <TypeDefs as super::Types>::sizes,
        type_ = <TypeDefs as super::Types>::type_,
        blocking = <TypeDefs as super::Types>::blocking,
    >;
    pub type href_lang<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        as_ = <TypeDefs as super::Types>::as_,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        href = <TypeDefs as super::Types>::href,
        href_lang = Value,
        image_sizes = <TypeDefs as super::Types>::image_sizes,
        image_src_set = <TypeDefs as super::Types>::image_src_set,
        integrity = <TypeDefs as super::Types>::integrity,
        media = <TypeDefs as super::Types>::media,
        prefetch = <TypeDefs as super::Types>::prefetch,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        rel = <TypeDefs as super::Types>::rel,
        sizes = <TypeDefs as super::Types>::sizes,
        type_ = <TypeDefs as super::Types>::type_,
        blocking = <TypeDefs as super::Types>::blocking,
    >;
    pub type image_sizes<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        as_ = <TypeDefs as super::Types>::as_,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        href = <TypeDefs as super::Types>::href,
        href_lang = <TypeDefs as super::Types>::href_lang,
        image_sizes = Value,
        image_src_set = <TypeDefs as super::Types>::image_src_set,
        integrity = <TypeDefs as super::Types>::integrity,
        media = <TypeDefs as super::Types>::media,
        prefetch = <TypeDefs as super::Types>::prefetch,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        rel = <TypeDefs as super::Types>::rel,
        sizes = <TypeDefs as super::Types>::sizes,
        type_ = <TypeDefs as super::Types>::type_,
        blocking = <TypeDefs as super::Types>::blocking,
    >;
    pub type image_src_set<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        as_ = <TypeDefs as super::Types>::as_,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        href = <TypeDefs as super::Types>::href,
        href_lang = <TypeDefs as super::Types>::href_lang,
        image_sizes = <TypeDefs as super::Types>::image_sizes,
        image_src_set = Value,
        integrity = <TypeDefs as super::Types>::integrity,
        media = <TypeDefs as super::Types>::media,
        prefetch = <TypeDefs as super::Types>::prefetch,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        rel = <TypeDefs as super::Types>::rel,
        sizes = <TypeDefs as super::Types>::sizes,
        type_ = <TypeDefs as super::Types>::type_,
        blocking = <TypeDefs as super::Types>::blocking,
    >;
    pub type integrity<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        as_ = <TypeDefs as super::Types>::as_,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        href = <TypeDefs as super::Types>::href,
        href_lang = <TypeDefs as super::Types>::href_lang,
        image_sizes = <TypeDefs as super::Types>::image_sizes,
        image_src_set = <TypeDefs as super::Types>::image_src_set,
        integrity = Value,
        media = <TypeDefs as super::Types>::media,
        prefetch = <TypeDefs as super::Types>::prefetch,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        rel = <TypeDefs as super::Types>::rel,
        sizes = <TypeDefs as super::Types>::sizes,
        type_ = <TypeDefs as super::Types>::type_,
        blocking = <TypeDefs as super::Types>::blocking,
    >;
    pub type media<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        as_ = <TypeDefs as super::Types>::as_,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        href = <TypeDefs as super::Types>::href,
        href_lang = <TypeDefs as super::Types>::href_lang,
        image_sizes = <TypeDefs as super::Types>::image_sizes,
        image_src_set = <TypeDefs as super::Types>::image_src_set,
        integrity = <TypeDefs as super::Types>::integrity,
        media = Value,
        prefetch = <TypeDefs as super::Types>::prefetch,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        rel = <TypeDefs as super::Types>::rel,
        sizes = <TypeDefs as super::Types>::sizes,
        type_ = <TypeDefs as super::Types>::type_,
        blocking = <TypeDefs as super::Types>::blocking,
    >;
    pub type prefetch<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        as_ = <TypeDefs as super::Types>::as_,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        href = <TypeDefs as super::Types>::href,
        href_lang = <TypeDefs as super::Types>::href_lang,
        image_sizes = <TypeDefs as super::Types>::image_sizes,
        image_src_set = <TypeDefs as super::Types>::image_src_set,
        integrity = <TypeDefs as super::Types>::integrity,
        media = <TypeDefs as super::Types>::media,
        prefetch = Value,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        rel = <TypeDefs as super::Types>::rel,
        sizes = <TypeDefs as super::Types>::sizes,
        type_ = <TypeDefs as super::Types>::type_,
        blocking = <TypeDefs as super::Types>::blocking,
    >;
    pub type referrer_policy<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        as_ = <TypeDefs as super::Types>::as_,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        href = <TypeDefs as super::Types>::href,
        href_lang = <TypeDefs as super::Types>::href_lang,
        image_sizes = <TypeDefs as super::Types>::image_sizes,
        image_src_set = <TypeDefs as super::Types>::image_src_set,
        integrity = <TypeDefs as super::Types>::integrity,
        media = <TypeDefs as super::Types>::media,
        prefetch = <TypeDefs as super::Types>::prefetch,
        referrer_policy = Value,
        rel = <TypeDefs as super::Types>::rel,
        sizes = <TypeDefs as super::Types>::sizes,
        type_ = <TypeDefs as super::Types>::type_,
        blocking = <TypeDefs as super::Types>::blocking,
    >;
    pub type rel<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        as_ = <TypeDefs as super::Types>::as_,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        href = <TypeDefs as super::Types>::href,
        href_lang = <TypeDefs as super::Types>::href_lang,
        image_sizes = <TypeDefs as super::Types>::image_sizes,
        image_src_set = <TypeDefs as super::Types>::image_src_set,
        integrity = <TypeDefs as super::Types>::integrity,
        media = <TypeDefs as super::Types>::media,
        prefetch = <TypeDefs as super::Types>::prefetch,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        rel = Value,
        sizes = <TypeDefs as super::Types>::sizes,
        type_ = <TypeDefs as super::Types>::type_,
        blocking = <TypeDefs as super::Types>::blocking,
    >;
    pub type sizes<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        as_ = <TypeDefs as super::Types>::as_,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        href = <TypeDefs as super::Types>::href,
        href_lang = <TypeDefs as super::Types>::href_lang,
        image_sizes = <TypeDefs as super::Types>::image_sizes,
        image_src_set = <TypeDefs as super::Types>::image_src_set,
        integrity = <TypeDefs as super::Types>::integrity,
        media = <TypeDefs as super::Types>::media,
        prefetch = <TypeDefs as super::Types>::prefetch,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        rel = <TypeDefs as super::Types>::rel,
        sizes = Value,
        type_ = <TypeDefs as super::Types>::type_,
        blocking = <TypeDefs as super::Types>::blocking,
    >;
    pub type type_<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        as_ = <TypeDefs as super::Types>::as_,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        href = <TypeDefs as super::Types>::href,
        href_lang = <TypeDefs as super::Types>::href_lang,
        image_sizes = <TypeDefs as super::Types>::image_sizes,
        image_src_set = <TypeDefs as super::Types>::image_src_set,
        integrity = <TypeDefs as super::Types>::integrity,
        media = <TypeDefs as super::Types>::media,
        prefetch = <TypeDefs as super::Types>::prefetch,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        rel = <TypeDefs as super::Types>::rel,
        sizes = <TypeDefs as super::Types>::sizes,
        type_ = Value,
        blocking = <TypeDefs as super::Types>::blocking,
    >;
    pub type blocking<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        as_ = <TypeDefs as super::Types>::as_,
        cross_origin = <TypeDefs as super::Types>::cross_origin,
        fetch_priority = <TypeDefs as super::Types>::fetch_priority,
        href = <TypeDefs as super::Types>::href,
        href_lang = <TypeDefs as super::Types>::href_lang,
        image_sizes = <TypeDefs as super::Types>::image_sizes,
        image_src_set = <TypeDefs as super::Types>::image_src_set,
        integrity = <TypeDefs as super::Types>::integrity,
        media = <TypeDefs as super::Types>::media,
        prefetch = <TypeDefs as super::Types>::prefetch,
        referrer_policy = <TypeDefs as super::Types>::referrer_policy,
        rel = <TypeDefs as super::Types>::rel,
        sizes = <TypeDefs as super::Types>::sizes,
        type_ = <TypeDefs as super::Types>::type_,
        blocking = Value,
    >;
}
mod trait_types {
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub trait Types {
        type HtmlElementProps: ?::core::marker::Sized + HtmlElementProps::Types;
        type as_: crate::MaybeUpdateValueWithState<str>;
        type cross_origin: crate::MaybeUpdateValueWithState<str>;
        type fetch_priority: crate::MaybeUpdateValueWithState<str>;
        type href: crate::MaybeUpdateValueWithState<str>;
        type href_lang: crate::MaybeUpdateValueWithState<str>;
        type image_sizes: crate::MaybeUpdateValueWithState<str>;
        type image_src_set: crate::MaybeUpdateValueWithState<str>;
        type integrity: crate::MaybeUpdateValueWithState<str>;
        type media: crate::MaybeUpdateValueWithState<str>;
        type prefetch: crate::MaybeUpdateValueWithState<str>;
        type referrer_policy: crate::MaybeUpdateValueWithState<str>;
        type rel: crate::MaybeUpdateValueWithState<str>;
        type sizes: crate::MaybeUpdateValueWithState<str>;
        type type_: crate::MaybeUpdateValueWithState<str>;
        type blocking: crate::MaybeUpdateValueWithState<str>;
    }
}
pub use trait_types::Types;
pub use trait_types::Types as ValidTypes;
pub mod data_struct {
    #[non_exhaustive]
    pub struct HtmlLinkElementProps<TypeDefs: super::Types + ?::core::marker::Sized> {
        pub HtmlElementProps: super::super::HtmlElementProps::Data<TypeDefs::HtmlElementProps>,
        pub as_: TypeDefs::as_,
        pub cross_origin: TypeDefs::cross_origin,
        pub fetch_priority: TypeDefs::fetch_priority,
        pub href: TypeDefs::href,
        pub href_lang: TypeDefs::href_lang,
        pub image_sizes: TypeDefs::image_sizes,
        pub image_src_set: TypeDefs::image_src_set,
        pub integrity: TypeDefs::integrity,
        pub media: TypeDefs::media,
        pub prefetch: TypeDefs::prefetch,
        pub referrer_policy: TypeDefs::referrer_policy,
        pub rel: TypeDefs::rel,
        pub sizes: TypeDefs::sizes,
        pub type_: TypeDefs::type_,
        pub blocking: TypeDefs::blocking,
    }
}
pub use data_struct::HtmlLinkElementProps as Data;
pub struct Building<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
pub struct Replacing<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
mod types_initial {
    #[allow(unused_imports)]
    use super::super::*;
    pub type TypesInitial = dyn super::Types<
        HtmlElementProps = HtmlElementProps::TypesInitial,
        as_ = (),
        cross_origin = (),
        fetch_priority = (),
        href = (),
        href_lang = (),
        image_sizes = (),
        image_src_set = (),
        integrity = (),
        media = (),
        prefetch = (),
        referrer_policy = (),
        rel = (),
        sizes = (),
        type_ = (),
        blocking = (),
    >;
}
pub use types_initial::TypesInitial;
pub type DataInitial = Data<TypesInitial>;
#[cfg(feature = "dom")]
pub mod render_state {
    #[allow(non_camel_case_types)]
    pub trait RenderStateTypes {
        type HtmlElementProps: crate::props::IntrinsicComponentPollReactive;
        type as_;
        type cross_origin;
        type fetch_priority;
        type href;
        type href_lang;
        type image_sizes;
        type image_src_set;
        type integrity;
        type media;
        type prefetch;
        type referrer_policy;
        type rel;
        type sizes;
        type type_;
        type blocking;
    }
    pub struct RenderState<TypeDefs: RenderStateTypes>
    where
        TypeDefs: ?::core::marker::Sized,
    {
        pub HtmlElementProps: TypeDefs::HtmlElementProps,
        pub as_: TypeDefs::as_,
        pub cross_origin: TypeDefs::cross_origin,
        pub fetch_priority: TypeDefs::fetch_priority,
        pub href: TypeDefs::href,
        pub href_lang: TypeDefs::href_lang,
        pub image_sizes: TypeDefs::image_sizes,
        pub image_src_set: TypeDefs::image_src_set,
        pub integrity: TypeDefs::integrity,
        pub media: TypeDefs::media,
        pub prefetch: TypeDefs::prefetch,
        pub referrer_policy: TypeDefs::referrer_policy,
        pub rel: TypeDefs::rel,
        pub sizes: TypeDefs::sizes,
        pub type_: TypeDefs::type_,
        pub blocking: TypeDefs::blocking,
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
        pub as_: &'__pin mut (TypeDefs::as_),
        pub cross_origin: &'__pin mut (TypeDefs::cross_origin),
        pub fetch_priority: &'__pin mut (TypeDefs::fetch_priority),
        pub href: &'__pin mut (TypeDefs::href),
        pub href_lang: &'__pin mut (TypeDefs::href_lang),
        pub image_sizes: &'__pin mut (TypeDefs::image_sizes),
        pub image_src_set: &'__pin mut (TypeDefs::image_src_set),
        pub integrity: &'__pin mut (TypeDefs::integrity),
        pub media: &'__pin mut (TypeDefs::media),
        pub prefetch: &'__pin mut (TypeDefs::prefetch),
        pub referrer_policy: &'__pin mut (TypeDefs::referrer_policy),
        pub rel: &'__pin mut (TypeDefs::rel),
        pub sizes: &'__pin mut (TypeDefs::sizes),
        pub type_: &'__pin mut (TypeDefs::type_),
        pub blocking: &'__pin mut (TypeDefs::blocking),
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
            pub as_: &'__pin (TypeDefs::as_),
            pub cross_origin: &'__pin (TypeDefs::cross_origin),
            pub fetch_priority: &'__pin (TypeDefs::fetch_priority),
            pub href: &'__pin (TypeDefs::href),
            pub href_lang: &'__pin (TypeDefs::href_lang),
            pub image_sizes: &'__pin (TypeDefs::image_sizes),
            pub image_src_set: &'__pin (TypeDefs::image_src_set),
            pub integrity: &'__pin (TypeDefs::integrity),
            pub media: &'__pin (TypeDefs::media),
            pub prefetch: &'__pin (TypeDefs::prefetch),
            pub referrer_policy: &'__pin (TypeDefs::referrer_policy),
            pub rel: &'__pin (TypeDefs::rel),
            pub sizes: &'__pin (TypeDefs::sizes),
            pub type_: &'__pin (TypeDefs::type_),
            pub blocking: &'__pin (TypeDefs::blocking),
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
                        as_,
                        cross_origin,
                        fetch_priority,
                        href,
                        href_lang,
                        image_sizes,
                        image_src_set,
                        integrity,
                        media,
                        prefetch,
                        referrer_policy,
                        rel,
                        sizes,
                        type_,
                        blocking,
                    } = self.get_unchecked_mut();
                    RenderStateProj {
                        HtmlElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            HtmlElementProps,
                        ),
                        as_: as_,
                        cross_origin: cross_origin,
                        fetch_priority: fetch_priority,
                        href: href,
                        href_lang: href_lang,
                        image_sizes: image_sizes,
                        image_src_set: image_src_set,
                        integrity: integrity,
                        media: media,
                        prefetch: prefetch,
                        referrer_policy: referrer_policy,
                        rel: rel,
                        sizes: sizes,
                        type_: type_,
                        blocking: blocking,
                    }
                }
            }
            pub(crate) fn project_ref<'__pin>(
                self: ::pin_project_lite::__private::Pin<&'__pin Self>,
            ) -> ProjectionRef<'__pin, TypeDefs> {
                unsafe {
                    let Self {
                        HtmlElementProps,
                        as_,
                        cross_origin,
                        fetch_priority,
                        href,
                        href_lang,
                        image_sizes,
                        image_src_set,
                        integrity,
                        media,
                        prefetch,
                        referrer_policy,
                        rel,
                        sizes,
                        type_,
                        blocking,
                    } = self.get_ref();
                    ProjectionRef {
                        HtmlElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            HtmlElementProps,
                        ),
                        as_: as_,
                        cross_origin: cross_origin,
                        fetch_priority: fetch_priority,
                        href: href,
                        href_lang: href_lang,
                        image_sizes: image_sizes,
                        image_src_set: image_src_set,
                        integrity: integrity,
                        media: media,
                        prefetch: prefetch,
                        referrer_policy: referrer_policy,
                        rel: rel,
                        sizes: sizes,
                        type_: type_,
                        blocking: blocking,
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
            as_: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::as_>,
            cross_origin: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::cross_origin>,
            fetch_priority: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::fetch_priority>,
            href: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::href>,
            href_lang: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::href_lang>,
            image_sizes: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::image_sizes>,
            image_src_set: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::image_src_set>,
            integrity: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::integrity>,
            media: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::media>,
            prefetch: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::prefetch>,
            referrer_policy: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::referrer_policy>,
            rel: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::rel>,
            sizes: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::sizes>,
            type_: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::type_>,
            blocking: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::blocking>,
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
            let _ = &this.as_;
            let _ = &this.cross_origin;
            let _ = &this.fetch_priority;
            let _ = &this.href;
            let _ = &this.href_lang;
            let _ = &this.image_sizes;
            let _ = &this.image_src_set;
            let _ = &this.integrity;
            let _ = &this.media;
            let _ = &this.prefetch;
            let _ = &this.referrer_policy;
            let _ = &this.rel;
            let _ = &this.sizes;
            let _ = &this.type_;
            let _ = &this.blocking;
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
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
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
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
            })
        }
        #[inline(always)]
        pub fn as_<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            as_: V,
        ) -> super::Building<super::overwrite::as_<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
            })
        }
        #[inline(always)]
        pub fn cross_origin<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            cross_origin: V,
        ) -> super::Building<super::overwrite::cross_origin<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                as_: self.0.as_,
                cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
            })
        }
        #[inline(always)]
        pub fn fetch_priority<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            fetch_priority: V,
        ) -> super::Building<super::overwrite::fetch_priority<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
            })
        }
        #[inline(always)]
        pub fn href<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            href: V,
        ) -> super::Building<super::overwrite::href<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
            })
        }
        #[inline(always)]
        pub fn href_lang<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            href_lang: V,
        ) -> super::Building<super::overwrite::href_lang<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
            })
        }
        #[inline(always)]
        pub fn image_sizes<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            image_sizes: V,
        ) -> super::Building<super::overwrite::image_sizes<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
            })
        }
        #[inline(always)]
        pub fn image_src_set<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            image_src_set: V,
        ) -> super::Building<super::overwrite::image_src_set<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
            })
        }
        #[inline(always)]
        pub fn integrity<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            integrity: V,
        ) -> super::Building<super::overwrite::integrity<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
            })
        }
        #[inline(always)]
        pub fn media<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            media: V,
        ) -> super::Building<super::overwrite::media<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
            })
        }
        #[inline(always)]
        pub fn prefetch<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            prefetch: V,
        ) -> super::Building<super::overwrite::prefetch<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
            })
        }
        #[inline(always)]
        pub fn referrer_policy<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            referrer_policy: V,
        ) -> super::Building<super::overwrite::referrer_policy<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
            })
        }
        #[inline(always)]
        pub fn rel<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            rel: V,
        ) -> super::Building<super::overwrite::rel<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
            })
        }
        #[inline(always)]
        pub fn sizes<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            sizes: V,
        ) -> super::Building<super::overwrite::sizes<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes,
                type_: self.0.type_,
                blocking: self.0.blocking,
            })
        }
        #[inline(always)]
        pub fn type_<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            type_: V,
        ) -> super::Building<super::overwrite::type_<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_,
                blocking: self.0.blocking,
            })
        }
        #[inline(always)]
        pub fn blocking<V: crate::MaybeUpdateValueWithState<str>>(
            self,
            blocking: V,
        ) -> super::Building<super::overwrite::blocking<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                as_: self.0.as_,
                cross_origin: self.0.cross_origin,
                fetch_priority: self.0.fetch_priority,
                href: self.0.href,
                href_lang: self.0.href_lang,
                image_sizes: self.0.image_sizes,
                image_src_set: self.0.image_src_set,
                integrity: self.0.integrity,
                media: self.0.media,
                prefetch: self.0.prefetch,
                referrer_policy: self.0.referrer_policy,
                rel: self.0.rel,
                sizes: self.0.sizes,
                type_: self.0.type_,
                blocking,
            })
        }
    }
}
#[cfg(feature = "dom")]
mod impl_update_element {
    #[allow(unused_imports)]
    use super::super::*;
    impl<TypeDefs: ?::core::marker::Sized + super::Types>
        crate::props::UpdateElement<web_sys::HtmlLinkElement> for super::Data<TypeDefs>
    where
        HtmlElementProps::Data<TypeDefs::HtmlElementProps>:
            crate::props::UpdateElement<web_sys::HtmlElement>,
    {
        type State = super::render_state::RenderState<
            dyn super::render_state::RenderStateTypes<
                HtmlElementProps = <HtmlElementProps::Data<
                    TypeDefs::HtmlElementProps,
                > as crate::props::UpdateElement<web_sys::HtmlElement>>::State,
                as_ = <TypeDefs::as_ as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                cross_origin = <TypeDefs::cross_origin as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                fetch_priority = <TypeDefs::fetch_priority as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                href = <TypeDefs::href as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                href_lang = <TypeDefs::href_lang as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                image_sizes = <TypeDefs::image_sizes as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                image_src_set = <TypeDefs::image_src_set as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                integrity = <TypeDefs::integrity as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                media = <TypeDefs::media as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                prefetch = <TypeDefs::prefetch as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                referrer_policy = <TypeDefs::referrer_policy as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                rel = <TypeDefs::rel as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                sizes = <TypeDefs::sizes as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                type_ = <TypeDefs::type_ as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                blocking = <TypeDefs::blocking as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
            >,
        >;
        fn initialize_state(
            this: Self,
            element: &web_sys::HtmlLinkElement,
            children_ctx: &mut ::frender_dom::Dom,
        ) -> Self::State {
            let dom_element: &::web_sys::Element = element.as_ref();
            super::render_state::RenderState {
                HtmlElementProps: <HtmlElementProps::Data<
                    TypeDefs::HtmlElementProps,
                > as crate::props::UpdateElement<
                    web_sys::HtmlElement,
                >>::initialize_state(this.HtmlElementProps, element, children_ctx),
                as_: <TypeDefs::as_ as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.as_,
                    |v| element.set_as(v),
                    || dom_element.remove_attribute("as").unwrap(),
                ),
                cross_origin: <TypeDefs::cross_origin as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.cross_origin,
                    match element {
                        el => |v: &_| el.set_cross_origin(Some(v)),
                    },
                    match element {
                        el => || el.set_cross_origin(None),
                    },
                ),
                fetch_priority: <TypeDefs::fetch_priority as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.fetch_priority,
                    |v| crate::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "fetchpriority",
                    ),
                    || dom_element.remove_attribute("fetchpriority").unwrap(),
                ),
                href: <TypeDefs::href as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.href,
                    |v| element.set_href(v),
                    || dom_element.remove_attribute("href").unwrap(),
                ),
                href_lang: <TypeDefs::href_lang as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.href_lang,
                    |v| element.set_hreflang(v),
                    || dom_element.remove_attribute("hreflang").unwrap(),
                ),
                image_sizes: <TypeDefs::image_sizes as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.image_sizes,
                    |v| crate::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "imagesizes",
                    ),
                    || dom_element.remove_attribute("imagesizes").unwrap(),
                ),
                image_src_set: <TypeDefs::image_src_set as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.image_src_set,
                    |v| crate::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "imagesrcset",
                    ),
                    || dom_element.remove_attribute("imagesrcset").unwrap(),
                ),
                integrity: <TypeDefs::integrity as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.integrity,
                    |v| element.set_integrity(v),
                    || dom_element.remove_attribute("integrity").unwrap(),
                ),
                media: <TypeDefs::media as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.media,
                    |v| element.set_media(v),
                    || dom_element.remove_attribute("media").unwrap(),
                ),
                prefetch: <TypeDefs::prefetch as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.prefetch,
                    |v| crate::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "prefetch",
                    ),
                    || dom_element.remove_attribute("prefetch").unwrap(),
                ),
                referrer_policy: <TypeDefs::referrer_policy as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.referrer_policy,
                    |v| element.set_referrer_policy(v),
                    || dom_element.remove_attribute("referrerpolicy").unwrap(),
                ),
                rel: <TypeDefs::rel as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.rel,
                    |v| element.set_rel(v),
                    || dom_element.remove_attribute("rel").unwrap(),
                ),
                sizes: <TypeDefs::sizes as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.sizes,
                    |v| crate::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "sizes",
                    ),
                    || dom_element.remove_attribute("sizes").unwrap(),
                ),
                type_: <TypeDefs::type_ as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.type_,
                    |v| element.set_type(v),
                    || dom_element.remove_attribute("type").unwrap(),
                ),
                blocking: <TypeDefs::blocking as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.blocking,
                    |v| crate::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "blocking",
                    ),
                    || dom_element.remove_attribute("blocking").unwrap(),
                ),
            }
        }
        fn update_element(
            this: Self,
            element: &web_sys::HtmlLinkElement,
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
            <TypeDefs::as_ as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.as_,
                state.as_,
                |v| element.set_as(v),
                || dom_element.remove_attribute("as").unwrap(),
            );
            <TypeDefs::cross_origin as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.cross_origin,
                state.cross_origin,
                match element {
                    el => |v: &_| el.set_cross_origin(Some(v)),
                },
                match element {
                    el => || el.set_cross_origin(None),
                },
            );
            <TypeDefs::fetch_priority as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.fetch_priority,
                state.fetch_priority,
                |v| crate::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "fetchpriority",
                ),
                || dom_element.remove_attribute("fetchpriority").unwrap(),
            );
            <TypeDefs::href as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.href,
                state.href,
                |v| element.set_href(v),
                || dom_element.remove_attribute("href").unwrap(),
            );
            <TypeDefs::href_lang as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.href_lang,
                state.href_lang,
                |v| element.set_hreflang(v),
                || dom_element.remove_attribute("hreflang").unwrap(),
            );
            <TypeDefs::image_sizes as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.image_sizes,
                state.image_sizes,
                |v| crate::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "imagesizes",
                ),
                || dom_element.remove_attribute("imagesizes").unwrap(),
            );
            <TypeDefs::image_src_set as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.image_src_set,
                state.image_src_set,
                |v| crate::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "imagesrcset",
                ),
                || dom_element.remove_attribute("imagesrcset").unwrap(),
            );
            <TypeDefs::integrity as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.integrity,
                state.integrity,
                |v| element.set_integrity(v),
                || dom_element.remove_attribute("integrity").unwrap(),
            );
            <TypeDefs::media as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.media,
                state.media,
                |v| element.set_media(v),
                || dom_element.remove_attribute("media").unwrap(),
            );
            <TypeDefs::prefetch as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.prefetch,
                state.prefetch,
                |v| crate::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "prefetch",
                ),
                || dom_element.remove_attribute("prefetch").unwrap(),
            );
            <TypeDefs::referrer_policy as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.referrer_policy,
                state.referrer_policy,
                |v| element.set_referrer_policy(v),
                || dom_element.remove_attribute("referrerpolicy").unwrap(),
            );
            <TypeDefs::rel as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.rel,
                state.rel,
                |v| element.set_rel(v),
                || dom_element.remove_attribute("rel").unwrap(),
            );
            <TypeDefs::sizes as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.sizes,
                state.sizes,
                |v| crate::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "sizes",
                ),
                || dom_element.remove_attribute("sizes").unwrap(),
            );
            <TypeDefs::type_ as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.type_,
                state.type_,
                |v| element.set_type(v),
                || dom_element.remove_attribute("type").unwrap(),
            );
            <TypeDefs::blocking as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.blocking,
                state.blocking,
                |v| crate::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "blocking",
                ),
                || dom_element.remove_attribute("blocking").unwrap(),
            );
        }
    }
}
