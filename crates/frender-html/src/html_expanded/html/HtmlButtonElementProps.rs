#[allow(non_snake_case)]
#[inline(always)]
pub const fn HtmlButtonElementProps() -> Building<TypesInitial> {
    #[allow(unused_imports)]
    use super::*;
    self::Building(self::Data {
        HtmlElementProps: HtmlElementProps::build(HtmlElementProps()),
        disabled: (),
        form: (),
        form_action: (),
        form_enc_type: (),
        form_method: (),
        form_no_validate: (),
        form_target: (),
        name: (),
        type_: (),
        value: (),
    })
}
pub mod prelude {}
pub mod overwrite {
    #![allow(non_camel_case_types)]
    pub type HtmlElementProps<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = Value,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        name = <TypeDefs as super::Types>::name,
        type_ = <TypeDefs as super::Types>::type_,
        value = <TypeDefs as super::Types>::value,
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
    pub type disabled<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        disabled = Value,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        name = <TypeDefs as super::Types>::name,
        type_ = <TypeDefs as super::Types>::type_,
        value = <TypeDefs as super::Types>::value,
    >;
    pub type form<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        disabled = <TypeDefs as super::Types>::disabled,
        form = Value,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        name = <TypeDefs as super::Types>::name,
        type_ = <TypeDefs as super::Types>::type_,
        value = <TypeDefs as super::Types>::value,
    >;
    pub type form_action<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = Value,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        name = <TypeDefs as super::Types>::name,
        type_ = <TypeDefs as super::Types>::type_,
        value = <TypeDefs as super::Types>::value,
    >;
    pub type form_enc_type<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = Value,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        name = <TypeDefs as super::Types>::name,
        type_ = <TypeDefs as super::Types>::type_,
        value = <TypeDefs as super::Types>::value,
    >;
    pub type form_method<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = Value,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        name = <TypeDefs as super::Types>::name,
        type_ = <TypeDefs as super::Types>::type_,
        value = <TypeDefs as super::Types>::value,
    >;
    pub type form_no_validate<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = Value,
        form_target = <TypeDefs as super::Types>::form_target,
        name = <TypeDefs as super::Types>::name,
        type_ = <TypeDefs as super::Types>::type_,
        value = <TypeDefs as super::Types>::value,
    >;
    pub type form_target<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = Value,
        name = <TypeDefs as super::Types>::name,
        type_ = <TypeDefs as super::Types>::type_,
        value = <TypeDefs as super::Types>::value,
    >;
    pub type name<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        name = Value,
        type_ = <TypeDefs as super::Types>::type_,
        value = <TypeDefs as super::Types>::value,
    >;
    pub type type_<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        name = <TypeDefs as super::Types>::name,
        type_ = Value,
        value = <TypeDefs as super::Types>::value,
    >;
    pub type value<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        name = <TypeDefs as super::Types>::name,
        type_ = <TypeDefs as super::Types>::type_,
        value = Value,
    >;
}
mod trait_types {
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub trait Types {
        type HtmlElementProps: ?::core::marker::Sized
            + HtmlElementProps::Types
            + ~const ::core::marker::Destruct;
        type disabled: crate::MaybeUpdateValue<bool> + ~const ::core::marker::Destruct;
        type form: crate::MaybeUpdateValueByRef<str> + ~const ::core::marker::Destruct;
        type form_action: crate::MaybeUpdateValueByRef<str> + ~const ::core::marker::Destruct;
        type form_enc_type: crate::MaybeUpdateValueByRef<str> + ~const ::core::marker::Destruct;
        type form_method: crate::MaybeUpdateValueByRef<str> + ~const ::core::marker::Destruct;
        type form_no_validate: crate::MaybeUpdateValue<bool> + ~const ::core::marker::Destruct;
        type form_target: crate::MaybeUpdateValueByRef<str> + ~const ::core::marker::Destruct;
        type name: crate::MaybeUpdateValueByRef<str> + ~const ::core::marker::Destruct;
        type type_: crate::MaybeUpdateValueByRef<str> + ~const ::core::marker::Destruct;
        type value: crate::MaybeUpdateValueByRef<str> + ~const ::core::marker::Destruct;
    }
}
pub use trait_types::Types;
pub use trait_types::Types as ValidTypes;
pub mod data_struct {
    #[non_exhaustive]
    pub struct HtmlButtonElementProps<TypeDefs: super::Types + ?::core::marker::Sized> {
        pub HtmlElementProps: super::super::HtmlElementProps::Data<TypeDefs::HtmlElementProps>,
        pub disabled: TypeDefs::disabled,
        pub form: TypeDefs::form,
        pub form_action: TypeDefs::form_action,
        pub form_enc_type: TypeDefs::form_enc_type,
        pub form_method: TypeDefs::form_method,
        pub form_no_validate: TypeDefs::form_no_validate,
        pub form_target: TypeDefs::form_target,
        pub name: TypeDefs::name,
        pub type_: TypeDefs::type_,
        pub value: TypeDefs::value,
    }
}
pub use data_struct::HtmlButtonElementProps as Data;
pub struct Building<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
pub struct Replacing<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
mod types_initial {
    #[allow(unused_imports)]
    use super::super::*;
    pub type TypesInitial = dyn super::Types<
        HtmlElementProps = HtmlElementProps::TypesInitial,
        disabled = (),
        form = (),
        form_action = (),
        form_enc_type = (),
        form_method = (),
        form_no_validate = (),
        form_target = (),
        name = (),
        type_ = (),
        value = (),
    >;
}
pub use types_initial::TypesInitial;
pub type DataInitial = Data<TypesInitial>;
#[cfg(feature = "dom")]
pub use super::HtmlElementProps::render_state;
#[inline(always)]
pub const fn build<TypeDefs: ?::core::marker::Sized + Types>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    building.0
}
mod builder_and_replacer {
    #[allow(unused_imports)]
    use super::super::*;
    impl<TypeDefs: super::Types + ?::core::marker::Sized> super::Building<TypeDefs> {
        #[doc = "See [`HtmlElementProps::children`]"]
        #[inline(always)]
        pub const fn children<V>(
            self,
            children: V,
        ) -> super::Building<super::overwrite::children<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).children(children),
                ),
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[doc = "See [`HtmlElementProps::class`]"]
        #[inline(always)]
        pub const fn class<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            class: V,
        ) -> super::Building<super::overwrite::class<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).class(class),
                ),
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[doc = "See [`HtmlElementProps::id`]"]
        #[inline(always)]
        pub const fn id<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            id: V,
        ) -> super::Building<super::overwrite::id<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).id(id),
                ),
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[doc = "See [`HtmlElementProps::part`]"]
        #[inline(always)]
        pub const fn part<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            part: V,
        ) -> super::Building<super::overwrite::part<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).part(part),
                ),
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[doc = "See [`HtmlElementProps::access_key`]"]
        #[inline(always)]
        pub const fn access_key<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            access_key: V,
        ) -> super::Building<super::overwrite::access_key<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).access_key(access_key),
                ),
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[doc = "See [`HtmlElementProps::auto_capitalize`]"]
        #[inline(always)]
        pub const fn auto_capitalize<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            auto_capitalize: V,
        ) -> super::Building<super::overwrite::auto_capitalize<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps)
                        .auto_capitalize(auto_capitalize),
                ),
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[doc = "See [`HtmlElementProps::auto_focus`]"]
        #[inline(always)]
        pub const fn auto_focus<V: crate::MaybeUpdateValue<bool>>(
            self,
            auto_focus: V,
        ) -> super::Building<super::overwrite::auto_focus<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).auto_focus(auto_focus),
                ),
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[doc = "See [`HtmlElementProps::content_editable`]"]
        #[inline(always)]
        pub const fn content_editable<V: crate::props::MaybeInherit<bool>>(
            self,
            content_editable: V,
        ) -> super::Building<super::overwrite::content_editable<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps)
                        .content_editable(content_editable),
                ),
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[doc = "See [`HtmlElementProps::context_menu`]"]
        #[inline(always)]
        pub const fn context_menu<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            context_menu: V,
        ) -> super::Building<super::overwrite::context_menu<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).context_menu(context_menu),
                ),
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[doc = "See [`HtmlElementProps::dir`]"]
        #[inline(always)]
        pub const fn dir<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            dir: V,
        ) -> super::Building<super::overwrite::dir<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).dir(dir),
                ),
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[doc = "See [`HtmlElementProps::draggable`]"]
        #[inline(always)]
        pub const fn draggable<V: crate::MaybeUpdateValue<bool>>(
            self,
            draggable: V,
        ) -> super::Building<super::overwrite::draggable<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).draggable(draggable),
                ),
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[doc = "See [`HtmlElementProps::enter_key_hint`]"]
        #[inline(always)]
        pub const fn enter_key_hint<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            enter_key_hint: V,
        ) -> super::Building<super::overwrite::enter_key_hint<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps)
                        .enter_key_hint(enter_key_hint),
                ),
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[doc = "See [`HtmlElementProps::hidden`]"]
        #[inline(always)]
        pub const fn hidden<V: crate::MaybeUpdateValue<bool>>(
            self,
            hidden: V,
        ) -> super::Building<super::overwrite::hidden<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).hidden(hidden),
                ),
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[doc = "See [`HtmlElementProps::inert`]"]
        #[inline(always)]
        pub const fn inert<V: crate::MaybeUpdateValue<bool>>(
            self,
            inert: V,
        ) -> super::Building<super::overwrite::inert<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).inert(inert),
                ),
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[doc = "See [`HtmlElementProps::input_mode`]"]
        #[inline(always)]
        pub const fn input_mode<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            input_mode: V,
        ) -> super::Building<super::overwrite::input_mode<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).input_mode(input_mode),
                ),
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[doc = "See [`HtmlElementProps::is`]"]
        #[inline(always)]
        pub const fn is<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            is: V,
        ) -> super::Building<super::overwrite::is<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).is(is),
                ),
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[doc = "See [`HtmlElementProps::item_id`]"]
        #[inline(always)]
        pub const fn item_id<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            item_id: V,
        ) -> super::Building<super::overwrite::item_id<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).item_id(item_id),
                ),
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[doc = "See [`HtmlElementProps::item_prop`]"]
        #[inline(always)]
        pub const fn item_prop<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            item_prop: V,
        ) -> super::Building<super::overwrite::item_prop<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).item_prop(item_prop),
                ),
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[doc = "See [`HtmlElementProps::item_ref`]"]
        #[inline(always)]
        pub const fn item_ref<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            item_ref: V,
        ) -> super::Building<super::overwrite::item_ref<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).item_ref(item_ref),
                ),
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[doc = "See [`HtmlElementProps::item_scope`]"]
        #[inline(always)]
        pub const fn item_scope<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            item_scope: V,
        ) -> super::Building<super::overwrite::item_scope<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).item_scope(item_scope),
                ),
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[doc = "See [`HtmlElementProps::item_type`]"]
        #[inline(always)]
        pub const fn item_type<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            item_type: V,
        ) -> super::Building<super::overwrite::item_type<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).item_type(item_type),
                ),
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[doc = "See [`HtmlElementProps::lang`]"]
        #[inline(always)]
        pub const fn lang<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            lang: V,
        ) -> super::Building<super::overwrite::lang<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).lang(lang),
                ),
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[doc = "See [`HtmlElementProps::nonce`]"]
        #[inline(always)]
        pub const fn nonce<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            nonce: V,
        ) -> super::Building<super::overwrite::nonce<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).nonce(nonce),
                ),
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[doc = "See [`HtmlElementProps::role`]"]
        #[inline(always)]
        pub const fn role<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            role: V,
        ) -> super::Building<super::overwrite::role<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).role(role),
                ),
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[doc = "See [`HtmlElementProps::slot`]"]
        #[inline(always)]
        pub const fn slot<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            slot: V,
        ) -> super::Building<super::overwrite::slot<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).slot(slot),
                ),
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[doc = "See [`HtmlElementProps::spellcheck`]"]
        #[inline(always)]
        pub const fn spellcheck<V: crate::MaybeUpdateValue<bool>>(
            self,
            spellcheck: V,
        ) -> super::Building<super::overwrite::spellcheck<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).spellcheck(spellcheck),
                ),
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[doc = "See [`HtmlElementProps::style`]"]
        #[inline(always)]
        pub const fn style<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            style: V,
        ) -> super::Building<super::overwrite::style<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).style(style),
                ),
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[doc = "See [`HtmlElementProps::tab_index`]"]
        #[inline(always)]
        pub const fn tab_index<V: crate::MaybeUpdateValue<i32>>(
            self,
            tab_index: V,
        ) -> super::Building<super::overwrite::tab_index<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).tab_index(tab_index),
                ),
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[doc = "See [`HtmlElementProps::title`]"]
        #[inline(always)]
        pub const fn title<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            title: V,
        ) -> super::Building<super::overwrite::title<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).title(title),
                ),
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[doc = "See [`HtmlElementProps::translate`]"]
        #[inline(always)]
        pub const fn translate<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            translate: V,
        ) -> super::Building<super::overwrite::translate<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).translate(translate),
                ),
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[doc = "See [`HtmlElementProps::virtual_keyboard_policy`]"]
        #[inline(always)]
        pub const fn virtual_keyboard_policy<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            virtual_keyboard_policy: V,
        ) -> super::Building<super::overwrite::virtual_keyboard_policy<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps)
                        .virtual_keyboard_policy(virtual_keyboard_policy),
                ),
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[doc = "See [`HtmlElementProps::on_click`]"]
        #[inline(always)]
        pub const fn on_click<V>(
            self,
            on_click: V,
        ) -> super::Building<super::overwrite::on_click<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).on_click(on_click),
                ),
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[inline(always)]
        pub const fn disabled<V: crate::MaybeUpdateValue<bool>>(
            self,
            disabled: V,
        ) -> super::Building<super::overwrite::disabled<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[inline(always)]
        pub const fn form<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            form: V,
        ) -> super::Building<super::overwrite::form<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                disabled: self.0.disabled,
                form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[inline(always)]
        pub const fn form_action<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            form_action: V,
        ) -> super::Building<super::overwrite::form_action<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[inline(always)]
        pub const fn form_enc_type<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            form_enc_type: V,
        ) -> super::Building<super::overwrite::form_enc_type<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[inline(always)]
        pub const fn form_method<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            form_method: V,
        ) -> super::Building<super::overwrite::form_method<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[inline(always)]
        pub const fn form_no_validate<V: crate::MaybeUpdateValue<bool>>(
            self,
            form_no_validate: V,
        ) -> super::Building<super::overwrite::form_no_validate<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[inline(always)]
        pub const fn form_target<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            form_target: V,
        ) -> super::Building<super::overwrite::form_target<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target,
                name: self.0.name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[inline(always)]
        pub const fn name<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            name: V,
        ) -> super::Building<super::overwrite::name<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name,
                type_: self.0.type_,
                value: self.0.value,
            })
        }
        #[inline(always)]
        pub const fn type_<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            type_: V,
        ) -> super::Building<super::overwrite::type_<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_,
                value: self.0.value,
            })
        }
        #[inline(always)]
        pub const fn value<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            value: V,
        ) -> super::Building<super::overwrite::value<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                name: self.0.name,
                type_: self.0.type_,
                value,
            })
        }
    }
}
#[cfg(feature = "dom")]
mod impl_update_element {
    #[allow(unused_imports)]
    use super::super::*;
    impl<TypeDefs: ?::core::marker::Sized + super::Types>
        crate::props::UpdateElement<web_sys::HtmlButtonElement> for super::Data<TypeDefs>
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
            element: &web_sys::HtmlButtonElement,
            children_ctx: &mut ::frender_dom::Dom,
        ) -> Self::State {
            let dom_element: &::web_sys::Element = element.as_ref();
            < TypeDefs :: disabled as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (this . disabled , | v | element . set_disabled (v) , | | dom_element . remove_attribute ("disabled") . unwrap () ,) ;
            < TypeDefs :: form as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . form , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , "form" ,) , | | dom_element . remove_attribute ("form") . unwrap () ,) ;
            < TypeDefs :: form_action as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . form_action , | v | element . set_form_action (v) , | | dom_element . remove_attribute ("formaction") . unwrap () ,) ;
            < TypeDefs :: form_enc_type as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . form_enc_type , | v | element . set_form_enctype (v) , | | dom_element . remove_attribute ("formenctype") . unwrap () ,) ;
            < TypeDefs :: form_method as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . form_method , | v | element . set_form_method (v) , | | dom_element . remove_attribute ("formmethod") . unwrap () ,) ;
            < TypeDefs :: form_no_validate as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (this . form_no_validate , | v | element . set_form_no_validate (v) , | | dom_element . remove_attribute ("formnovalidate") . unwrap () ,) ;
            < TypeDefs :: form_target as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . form_target , | v | element . set_form_target (v) , | | dom_element . remove_attribute ("formtarget") . unwrap () ,) ;
            < TypeDefs :: name as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . name , | v | element . set_name (v) , | | dom_element . remove_attribute ("name") . unwrap () ,) ;
            < TypeDefs :: type_ as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . type_ , | v | element . set_type (v) , | | dom_element . remove_attribute ("type") . unwrap () ,) ;
            < TypeDefs :: value as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . value , | v | element . set_value (v) , | | dom_element . remove_attribute ("value") . unwrap () ,) ;
            <HtmlElementProps::Data<TypeDefs::HtmlElementProps> as crate::props::UpdateElement<
                web_sys::HtmlElement,
            >>::initialize_state(this.HtmlElementProps, element, children_ctx)
        }
        fn update_element(
            this: Self,
            element: &web_sys::HtmlButtonElement,
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
            < TypeDefs :: disabled as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (this . disabled , | v | element . set_disabled (v) , | | dom_element . remove_attribute ("disabled") . unwrap () ,) ;
            < TypeDefs :: form as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . form , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , "form" ,) , | | dom_element . remove_attribute ("form") . unwrap () ,) ;
            < TypeDefs :: form_action as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . form_action , | v | element . set_form_action (v) , | | dom_element . remove_attribute ("formaction") . unwrap () ,) ;
            < TypeDefs :: form_enc_type as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . form_enc_type , | v | element . set_form_enctype (v) , | | dom_element . remove_attribute ("formenctype") . unwrap () ,) ;
            < TypeDefs :: form_method as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . form_method , | v | element . set_form_method (v) , | | dom_element . remove_attribute ("formmethod") . unwrap () ,) ;
            < TypeDefs :: form_no_validate as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (this . form_no_validate , | v | element . set_form_no_validate (v) , | | dom_element . remove_attribute ("formnovalidate") . unwrap () ,) ;
            < TypeDefs :: form_target as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . form_target , | v | element . set_form_target (v) , | | dom_element . remove_attribute ("formtarget") . unwrap () ,) ;
            < TypeDefs :: name as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . name , | v | element . set_name (v) , | | dom_element . remove_attribute ("name") . unwrap () ,) ;
            < TypeDefs :: type_ as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . type_ , | v | element . set_type (v) , | | dom_element . remove_attribute ("type") . unwrap () ,) ;
            < TypeDefs :: value as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . value , | v | element . set_value (v) , | | dom_element . remove_attribute ("value") . unwrap () ,) ;
        }
    }
}
