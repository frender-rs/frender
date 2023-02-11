#[allow(non_snake_case)]
#[inline(always)]
pub const fn HtmlAudioElementProps() -> Building<TypesInitial> {
    #[allow(unused_imports)]
    use super::*;
    self::Building(self::Data {
        HtmlMediaElementProps: HtmlMediaElementProps::build(HtmlMediaElementProps()),
        __: (),
    })
}
pub mod prelude {}
pub mod overwrite {
    #![allow(non_camel_case_types)]
    pub type HtmlMediaElementProps<TypeDefs, Value> =
        dyn super::Types<HtmlMediaElementProps = Value, __ = <TypeDefs as super::Types>::__>;
    pub type HtmlElementProps<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::HtmlElementProps<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type ElementProps<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::ElementProps<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type children<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::children<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type class<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::class<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type id<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::id<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type part<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::part<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type access_key<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::access_key<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type auto_capitalize<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::auto_capitalize<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type auto_focus<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::auto_focus<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type content_editable<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::content_editable<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type context_menu<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::context_menu<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type dir<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::dir<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type draggable<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::draggable<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type enter_key_hint<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::enter_key_hint<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type hidden<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::hidden<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type inert<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::inert<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type input_mode<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::input_mode<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type is<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::is<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type item_id<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::item_id<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type item_prop<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::item_prop<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type item_ref<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::item_ref<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type item_scope<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::item_scope<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type item_type<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::item_type<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type lang<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::lang<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type nonce<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::nonce<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type role<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::role<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type slot<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::slot<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type spellcheck<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::spellcheck<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type style<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::style<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type tab_index<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::tab_index<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type title<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::title<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type translate<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::translate<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type virtual_keyboard_policy<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::virtual_keyboard_policy<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type on_click<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::on_click<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type auto_play<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::auto_play<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type controls<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::controls<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type cross_origin<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::cross_origin<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type loop_<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::loop_<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type muted<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::muted<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type preload<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::preload<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type src<TypeDefs, Value> = self::HtmlMediaElementProps<
        TypeDefs,
        super::super::HtmlMediaElementProps::overwrite::src<
            <TypeDefs as super::Types>::HtmlMediaElementProps,
            Value,
        >,
    >;
    pub type __<TypeDefs, Value> = dyn super::Types<
        HtmlMediaElementProps = <TypeDefs as super::Types>::HtmlMediaElementProps,
        __ = Value,
    >;
}
mod trait_types {
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub trait Types {
        type HtmlMediaElementProps: ?::core::marker::Sized
            + HtmlMediaElementProps::Types
            + ~const ::core::marker::Destruct;
        type __: crate::MaybeUpdateValueByRef<str> + ~const ::core::marker::Destruct;
    }
}
pub use trait_types::Types;
pub use trait_types::Types as ValidTypes;
pub mod data_struct {
    #[non_exhaustive]
    pub struct HtmlAudioElementProps<TypeDefs: super::Types + ?::core::marker::Sized> {
        pub HtmlMediaElementProps:
            super::super::HtmlMediaElementProps::Data<TypeDefs::HtmlMediaElementProps>,
        pub __: TypeDefs::__,
    }
}
pub use data_struct::HtmlAudioElementProps as Data;
pub struct Building<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
pub struct Replacing<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
mod types_initial {
    #[allow(unused_imports)]
    use super::super::*;
    pub type TypesInitial =
        dyn super::Types<HtmlMediaElementProps = HtmlMediaElementProps::TypesInitial, __ = ()>;
}
pub use types_initial::TypesInitial;
pub type DataInitial = Data<TypesInitial>;
#[cfg(feature = "dom")]
pub use super::HtmlMediaElementProps::render_state;
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
        #[doc = "See [`HtmlMediaElementProps::children`]"]
        #[inline(always)]
        pub const fn children<V>(
            self,
            children: V,
        ) -> super::Building<super::overwrite::children<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .children(children),
                ),
                __: self.0.__,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::class`]"]
        #[inline(always)]
        pub const fn class<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            class: V,
        ) -> super::Building<super::overwrite::class<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps).class(class),
                ),
                __: self.0.__,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::id`]"]
        #[inline(always)]
        pub const fn id<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            id: V,
        ) -> super::Building<super::overwrite::id<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps).id(id),
                ),
                __: self.0.__,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::part`]"]
        #[inline(always)]
        pub const fn part<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            part: V,
        ) -> super::Building<super::overwrite::part<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps).part(part),
                ),
                __: self.0.__,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::access_key`]"]
        #[inline(always)]
        pub const fn access_key<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            access_key: V,
        ) -> super::Building<super::overwrite::access_key<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .access_key(access_key),
                ),
                __: self.0.__,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::auto_capitalize`]"]
        #[inline(always)]
        pub const fn auto_capitalize<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            auto_capitalize: V,
        ) -> super::Building<super::overwrite::auto_capitalize<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .auto_capitalize(auto_capitalize),
                ),
                __: self.0.__,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::auto_focus`]"]
        #[inline(always)]
        pub const fn auto_focus<V: crate::MaybeUpdateValue<bool>>(
            self,
            auto_focus: V,
        ) -> super::Building<super::overwrite::auto_focus<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .auto_focus(auto_focus),
                ),
                __: self.0.__,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::content_editable`]"]
        #[inline(always)]
        pub const fn content_editable<V: crate::props::MaybeInherit<bool>>(
            self,
            content_editable: V,
        ) -> super::Building<super::overwrite::content_editable<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .content_editable(content_editable),
                ),
                __: self.0.__,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::context_menu`]"]
        #[inline(always)]
        pub const fn context_menu<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            context_menu: V,
        ) -> super::Building<super::overwrite::context_menu<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .context_menu(context_menu),
                ),
                __: self.0.__,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::dir`]"]
        #[inline(always)]
        pub const fn dir<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            dir: V,
        ) -> super::Building<super::overwrite::dir<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps).dir(dir),
                ),
                __: self.0.__,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::draggable`]"]
        #[inline(always)]
        pub const fn draggable<V: crate::MaybeUpdateValue<bool>>(
            self,
            draggable: V,
        ) -> super::Building<super::overwrite::draggable<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .draggable(draggable),
                ),
                __: self.0.__,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::enter_key_hint`]"]
        #[inline(always)]
        pub const fn enter_key_hint<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            enter_key_hint: V,
        ) -> super::Building<super::overwrite::enter_key_hint<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .enter_key_hint(enter_key_hint),
                ),
                __: self.0.__,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::hidden`]"]
        #[inline(always)]
        pub const fn hidden<V: crate::MaybeUpdateValue<bool>>(
            self,
            hidden: V,
        ) -> super::Building<super::overwrite::hidden<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps).hidden(hidden),
                ),
                __: self.0.__,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::inert`]"]
        #[inline(always)]
        pub const fn inert<V: crate::MaybeUpdateValue<bool>>(
            self,
            inert: V,
        ) -> super::Building<super::overwrite::inert<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps).inert(inert),
                ),
                __: self.0.__,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::input_mode`]"]
        #[inline(always)]
        pub const fn input_mode<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            input_mode: V,
        ) -> super::Building<super::overwrite::input_mode<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .input_mode(input_mode),
                ),
                __: self.0.__,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::is`]"]
        #[inline(always)]
        pub const fn is<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            is: V,
        ) -> super::Building<super::overwrite::is<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps).is(is),
                ),
                __: self.0.__,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::item_id`]"]
        #[inline(always)]
        pub const fn item_id<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            item_id: V,
        ) -> super::Building<super::overwrite::item_id<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps).item_id(item_id),
                ),
                __: self.0.__,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::item_prop`]"]
        #[inline(always)]
        pub const fn item_prop<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            item_prop: V,
        ) -> super::Building<super::overwrite::item_prop<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .item_prop(item_prop),
                ),
                __: self.0.__,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::item_ref`]"]
        #[inline(always)]
        pub const fn item_ref<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            item_ref: V,
        ) -> super::Building<super::overwrite::item_ref<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .item_ref(item_ref),
                ),
                __: self.0.__,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::item_scope`]"]
        #[inline(always)]
        pub const fn item_scope<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            item_scope: V,
        ) -> super::Building<super::overwrite::item_scope<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .item_scope(item_scope),
                ),
                __: self.0.__,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::item_type`]"]
        #[inline(always)]
        pub const fn item_type<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            item_type: V,
        ) -> super::Building<super::overwrite::item_type<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .item_type(item_type),
                ),
                __: self.0.__,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::lang`]"]
        #[inline(always)]
        pub const fn lang<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            lang: V,
        ) -> super::Building<super::overwrite::lang<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps).lang(lang),
                ),
                __: self.0.__,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::nonce`]"]
        #[inline(always)]
        pub const fn nonce<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            nonce: V,
        ) -> super::Building<super::overwrite::nonce<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps).nonce(nonce),
                ),
                __: self.0.__,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::role`]"]
        #[inline(always)]
        pub const fn role<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            role: V,
        ) -> super::Building<super::overwrite::role<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps).role(role),
                ),
                __: self.0.__,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::slot`]"]
        #[inline(always)]
        pub const fn slot<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            slot: V,
        ) -> super::Building<super::overwrite::slot<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps).slot(slot),
                ),
                __: self.0.__,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::spellcheck`]"]
        #[inline(always)]
        pub const fn spellcheck<V: crate::MaybeUpdateValue<bool>>(
            self,
            spellcheck: V,
        ) -> super::Building<super::overwrite::spellcheck<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .spellcheck(spellcheck),
                ),
                __: self.0.__,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::style`]"]
        #[inline(always)]
        pub const fn style<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            style: V,
        ) -> super::Building<super::overwrite::style<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps).style(style),
                ),
                __: self.0.__,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::tab_index`]"]
        #[inline(always)]
        pub const fn tab_index<V: crate::MaybeUpdateValue<i32>>(
            self,
            tab_index: V,
        ) -> super::Building<super::overwrite::tab_index<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .tab_index(tab_index),
                ),
                __: self.0.__,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::title`]"]
        #[inline(always)]
        pub const fn title<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            title: V,
        ) -> super::Building<super::overwrite::title<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps).title(title),
                ),
                __: self.0.__,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::translate`]"]
        #[inline(always)]
        pub const fn translate<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            translate: V,
        ) -> super::Building<super::overwrite::translate<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .translate(translate),
                ),
                __: self.0.__,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::virtual_keyboard_policy`]"]
        #[inline(always)]
        pub const fn virtual_keyboard_policy<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            virtual_keyboard_policy: V,
        ) -> super::Building<super::overwrite::virtual_keyboard_policy<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .virtual_keyboard_policy(virtual_keyboard_policy),
                ),
                __: self.0.__,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::on_click`]"]
        #[inline(always)]
        pub const fn on_click<V>(
            self,
            on_click: V,
        ) -> super::Building<super::overwrite::on_click<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .on_click(on_click),
                ),
                __: self.0.__,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::auto_play`]"]
        #[inline(always)]
        pub const fn auto_play<V: crate::MaybeUpdateValue<bool>>(
            self,
            auto_play: V,
        ) -> super::Building<super::overwrite::auto_play<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .auto_play(auto_play),
                ),
                __: self.0.__,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::controls`]"]
        #[inline(always)]
        pub const fn controls<V: crate::MaybeUpdateValue<bool>>(
            self,
            controls: V,
        ) -> super::Building<super::overwrite::controls<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .controls(controls),
                ),
                __: self.0.__,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::cross_origin`]"]
        #[inline(always)]
        pub const fn cross_origin<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            cross_origin: V,
        ) -> super::Building<super::overwrite::cross_origin<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps)
                        .cross_origin(cross_origin),
                ),
                __: self.0.__,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::loop_`]"]
        #[inline(always)]
        pub const fn loop_<V: crate::MaybeUpdateValue<bool>>(
            self,
            loop_: V,
        ) -> super::Building<super::overwrite::loop_<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps).loop_(loop_),
                ),
                __: self.0.__,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::muted`]"]
        #[inline(always)]
        pub const fn muted<V: crate::MaybeUpdateValue<bool>>(
            self,
            muted: V,
        ) -> super::Building<super::overwrite::muted<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps).muted(muted),
                ),
                __: self.0.__,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::preload`]"]
        #[inline(always)]
        pub const fn preload<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            preload: V,
        ) -> super::Building<super::overwrite::preload<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps).preload(preload),
                ),
                __: self.0.__,
            })
        }
        #[doc = "See [`HtmlMediaElementProps::src`]"]
        #[inline(always)]
        pub const fn src<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            src: V,
        ) -> super::Building<super::overwrite::src<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlMediaElementProps: HtmlMediaElementProps::build(
                    HtmlMediaElementProps::Building(self.0.HtmlMediaElementProps).src(src),
                ),
                __: self.0.__,
            })
        }
        #[inline(always)]
        pub const fn __<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            __: V,
        ) -> super::Building<super::overwrite::__<TypeDefs, V>>
        where
            Self: ~const ::core::marker::Destruct,
        {
            super::Building(super::Data {
                HtmlMediaElementProps: self.0.HtmlMediaElementProps,
                __,
            })
        }
    }
}
#[cfg(feature = "dom")]
mod impl_update_element {
    #[allow(unused_imports)]
    use super::super::*;
    impl<TypeDefs: ?::core::marker::Sized + super::Types>
        crate::props::UpdateElement<web_sys::HtmlAudioElement> for super::Data<TypeDefs>
    where
        HtmlMediaElementProps::Data<TypeDefs::HtmlMediaElementProps>:
            crate::props::UpdateElement<web_sys::HtmlMediaElement>,
    {
        type State = < HtmlMediaElementProps :: Data < TypeDefs :: HtmlMediaElementProps , > as crate :: props :: UpdateElement < web_sys :: HtmlMediaElement > > :: State ;
        fn initialize_state(
            this: Self,
            element: &web_sys::HtmlAudioElement,
            children_ctx: &mut ::frender_dom::Dom,
        ) -> Self::State {
            let dom_element: &::web_sys::Element = element.as_ref();
            < TypeDefs :: __ as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . __ , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , "__" ,) , | | dom_element . remove_attribute ("__") . unwrap () ,) ;
            < HtmlMediaElementProps :: Data < TypeDefs :: HtmlMediaElementProps , > as crate :: props :: UpdateElement < web_sys :: HtmlMediaElement , > > :: initialize_state (this . HtmlMediaElementProps , element , children_ctx ,)
        }
        fn update_element(
            this: Self,
            element: &web_sys::HtmlAudioElement,
            children_ctx: &mut ::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element: &::web_sys::Element = element.as_ref();
            crate::props::UpdateElement::update_element(
                this.HtmlMediaElementProps,
                element.as_ref(),
                children_ctx,
                state,
            );
            < TypeDefs :: __ as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . __ , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , "__" ,) , | | dom_element . remove_attribute ("__") . unwrap () ,) ;
        }
    }
}
