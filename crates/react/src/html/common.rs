use convert_js::ToJs;
use frender_macros::ident_snake_to_camel;
use std::any::Any;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

use crate::IntoJsRuntime;
use crate::PassToJsRuntimeValue;
use crate::TryIntoJsRuntime;
use crate::{AsNullableElement, IntrinsicElement};

macro_rules! js_prop_name {
    ($k_js:literal $k:ident ) => {
        $k_js
    };
    ($k:ident) => {
        ident_snake_to_camel!($k)
    };
}

macro_rules! impl_attr_default {
    (
        $k:ident { $k_js:expr } [ $($fn_generics:tt)* ] : $v_ty:ty : { pass_to_js_runtime_default $impl_expr:expr }
    ) => {
        fn $k $($fn_generics)* (&mut self, v: $v_ty) -> &mut Self {
            if let Some(v) = v {
                let js = $crate::try_into_js_runtime_or_else(v, $impl_expr);
                self.__set_static_prop($k_js, js)
            } else {
                self.__set_static_prop(
                    $k_js,
                    $crate::PassToJsRuntimeValue {
                        js_value: JsValue::UNDEFINED,
                        to_persist: None,
                    },
                )
            }
        }
    };
    (
        $k:ident { $k_js:expr } [ $($fn_generics:tt)* ] : $v_ty:ty : { impl |$impl_this:ident, $impl_v:ident| $impl_expr:expr }
    ) => {
        fn $k $($fn_generics)* (&mut self, $impl_v: $v_ty) -> &mut Self {
            let $impl_this = self;
            $impl_expr
        }
    };
    (
        $k:ident { $k_js:expr } [ $($fn_generics:tt)* ] : $v_ty:ty
    ) => {
        // convert value to JsValue with ::convert_js::ToJs::to_js
        fn $k $($fn_generics)* (&mut self, v: $v_ty) -> &mut Self {
            let js_value = ::convert_js::ToJs::to_js(&v);
            self.__set_static_prop(
                $k_js,
                $crate::PassToJsRuntimeValue {
                    js_value,
                    to_persist: None,
                }
            )
        }
    };
}

macro_rules! def_attrs_traits {
    (
        struct { $struct_name:ident }
        generics { $($generics:tt)* }
        $(where { $($where:tt)+ })?
        $(extends { $($extend_trait:tt)+ })?
        attrs {
            $($k:ident $(@ $k_js:literal)? $([$($fn_generics:tt)*])? : $v_ty:ty $({$($impl_tt:tt)*})? ),* $(,)?
        }
    ) => {
        pub trait AsAttributesBuilder <$($generics)*> $(: $($extend_trait)+)? $(where $($where)+)? {
            fn __set_js_children(
                &mut self,
                js_children: Option<&dyn $crate::Node>,
            ) -> &mut Self;

            fn __set_static_prop(
                &mut self,
                prop_name: &'static str,
                js: $crate::PassToJsRuntimeValue,
            ) -> &mut Self;

            $(
                impl_attr_default! {
                    $k
                    { js_prop_name! ($($k_js)? $k) }
                    [$(<$($fn_generics)*>)?]
                    :
                    $v_ty
                    $(: {$($impl_tt)*})?
                }
            )*
        }

        impl<$($generics)* , TExtends: AsMut<$struct_name<$($generics)*>> $(+ $($extend_trait)+)?> AsAttributesBuilder <$($generics)*>  for TExtends $(where $($where)+)? {
            fn __set_js_children(
                &mut self,
                js_children: Option<&dyn $crate::Node>,
            ) -> &mut Self {
                self.as_mut().__set_js_children(js_children);
                self
            }

            fn __set_static_prop(
                &mut self,
                prop_name: &'static str,
                js: $crate::PassToJsRuntimeValue,
            ) -> &mut Self {
                self.as_mut().__set_static_prop(prop_name, js);
                self
            }

            $(
                fn $k $(<$($fn_generics)*>)? (&mut self, v: $v_ty) -> &mut Self {
                    self.as_mut().$k(v);
                    self
                }
            )*
        }
    };
}

def_attrs_traits! {
    struct { ComponentProps }
    generics { TElement, TValue }
    where { TElement: 'static + JsCast, TValue: convert_js::ToJs }

    extends { crate::IntoJsAdapterComponentProps + crate::Props }
    attrs {
        children: Option<&dyn crate::Node> {
            impl |this, v| this.__set_js_children(v)
        },
        ref_el[TWriteRef: 'static + crate::WriteRef<TElement> + crate::TryIntoJsRuntime]: Option<TWriteRef> {
            pass_to_js_runtime_default pass_any_write_ref
        },
        default_checked: Option<bool>,
        default_value: Option<TValue>,
        class@"className": Option<&str>,
        draggable: Option<bool>,
        hidden: Option<bool>,
        id: Option<&str>,
        lang: Option<&str>,
        placeholder: Option<&str>,
        style: Option<&crate::html::css::CssProperties> {
            impl |this, v| { todo!() }
        },
        tab_index: Option<i32>,
        title: Option<&str>,

        // React-specific Attributes
        suppress_content_editable_warning: Option<bool>,
        suppress_hydration_warning: Option<bool>,

        // Standard HTML Attributes
        access_key: Option<&str>,
        content_editable: Option<crate::html::Inheritable<bool>>,
        context_menu: Option<&str>,
        dir: Option<&str>,
        slot: Option<&str>,
        spell_check: Option<bool>,
        translate: Option<&str>, // TODO: ser: yes | no

        // Unknown
        radio_group: Option<&str>, // <command>, <menuitem>

        // WAI-ARIA
        role: Option<crate::html::aria::Role>,

        // RDFa Attributes
        about: Option<&str>,
        datatype: Option<&str>,
        inlist: Option<&wasm_bindgen::JsValue>,
        prefix: Option<&str>,
        property: Option<&str>,
        resource: Option<&str>,
        type_of@"typeof": Option<&str>,
        vocab: Option<&str>,

        // Non-standard Attributes
        auto_capitalize: Option<&str>,
        auto_correct: Option<&str>,
        auto_save: Option<&str>,
        color: Option<&str>,
        item_prop: Option<&str>,
        item_scope: Option<bool>,
        item_type: Option<&str>,
        item_id: Option<&str>,
        item_ref: Option<&str>,
        results: Option<i32>,
        security: Option<&str>,
        unselectable: Option<&str>, // TODO: ser: 'on' | 'off' | undefined;

        // Living Standard
        input_mode: Option<crate::html::HtmlInputMode>,
        is: Option<&str>,

        // events
        // TODO: def all events
        on_click: Option<Closure<dyn FnMut(wasm_bindgen::JsValue)>> {
            pass_to_js_runtime_default |_| {
                panic!("on_click should be able to passed to js runtime")
            }
        },
    }
}

fn pass_any_write_ref<
    TElement: 'static + JsCast,
    TWriteRef: 'static + crate::WriteRef<TElement> + TryIntoJsRuntime,
>(
    wr: TWriteRef,
) -> PassToJsRuntimeValue {
    wasm_bindgen::closure::Closure::wrap(Box::new(move |js_value: JsValue| {
        use convert_js::FromJs;
        let el = convert_js::WrapJsCast::<TElement>::from_js(js_value)
            .unwrap()
            .0;
        wr.set_current(el);
    }) as Box<dyn Fn(JsValue)>)
    .into_js_runtime()
}

pub struct ComponentProps<TElement, TValue> {
    _phantom: PhantomData<(TElement, TValue)>,
    js_children: Option<js_sys::Array>,
    js_props: Option<js_sys::Object>,
    js_component: Option<JsValue>,
    to_persist: Option<HashMap<&'static str, Rc<dyn Any>>>,
}

impl<TElement, TValue> crate::Props for ComponentProps<TElement, TValue> {
    type InitialBuilder = Self;

    fn init_builder() -> Self::InitialBuilder {
        Self {
            _phantom: PhantomData,
            js_children: None,
            js_props: None,
            js_component: None,
            to_persist: None,
        }
    }
}

impl<TElement, TValue> crate::IntoJsAdapterComponentProps for ComponentProps<TElement, TValue> {
    fn into_js_adapter_props(self) -> crate::JsAdapterComponentProps {
        crate::JsAdapterComponentProps {
            js_component: self.js_component.unwrap(),
            js_props: self.js_props,
            js_children: self.js_children,
            to_persist: self.to_persist.map(|v| Rc::new(v) as Rc<dyn Any>),
        }
    }
}

impl<TElement: 'static + JsCast, TValue: ToJs> AsAttributesBuilder<TElement, TValue>
    for ComponentProps<TElement, TValue>
{
    fn __set_static_prop(
        &mut self,
        prop_name: &'static str,
        js: PassToJsRuntimeValue,
    ) -> &mut Self {
        let obj = self.js_props.get_or_insert_with(|| js_sys::Object::new());
        js_sys::Reflect::set(obj.as_ref(), &JsValue::from_str(prop_name), &js.js_value).unwrap();

        let map = self.to_persist.get_or_insert_with(Default::default);
        if let Some(to_persist) = js.to_persist {
            map.insert(prop_name, to_persist);
        } else {
            map.remove(prop_name);
        }

        self
    }

    fn __set_js_children(&mut self, js_children: Option<&dyn crate::Node>) -> &mut Self {
        self.js_children = js_children.and_then(crate::Node::as_react_children_js);
        self
    }
}

pub struct Component<
    TProps: AsAttributesBuilder<TElement, TValue>,
    TElement: 'static + JsCast,
    TValue: ToJs,
> {
    pub props: TProps,
    _el: PhantomData<TElement>,
    _value: PhantomData<TValue>,
}

impl<TProps: AsAttributesBuilder<TElement, TValue>, TElement: 'static + JsCast, TValue: ToJs>
    crate::Component for Component<TProps, TElement, TValue>
{
    type Props = TProps;
    type ElementType = react_sys::Element;

    fn use_render(self) -> Self::ElementType
    where
        Self: Sized,
    {
        self.call_create_element(None)
    }

    fn new_with_props(props: Self::Props) -> Self {
        Self {
            props,
            _el: PhantomData,
            _value: PhantomData,
        }
    }

    fn call_create_element(self, key: Option<JsValue>) -> react_sys::Element
    where
        Self: Sized,
    {
        let props = self.props.into_js_adapter_props();
        crate::JsAdapterComponent::new_with_props(props).call_create_element(key)
    }
}

pub(crate) use def_attrs_traits;
pub(crate) use js_prop_name;
