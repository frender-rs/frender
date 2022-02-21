use std::{any::Any, borrow::Cow, collections::HashMap, rc::Rc};
use wasm_bindgen::JsValue;

use crate::PassedToJsRuntime;

#[derive(Debug, Clone, Default)]
pub struct AnyJsProps {
    pub js_props_without_children: Option<js_sys::Object>,
    pub children: Option<crate::Children>,
}

impl AnyJsPropsBuilder for AnyJsProps {
    #[inline]
    fn set_prop(&mut self, name: &str, value: &JsValue) -> &mut Self {
        let obj = self
            .js_props_without_children
            .get_or_insert_with(|| js_sys::Object::new());
        js_sys::Reflect::set(obj.as_ref(), &JsValue::from_str(name), &value).unwrap();

        self
    }

    #[inline]
    fn set_children(&mut self, children: Option<crate::Children>) -> &mut Self {
        self.children = children;
        self
    }

    #[inline]
    fn remove_prop(&mut self, name: &str) -> &mut Self {
        let obj = self
            .js_props_without_children
            .get_or_insert_with(|| js_sys::Object::new());
        js_sys::Reflect::delete_property(obj.as_ref(), &JsValue::from_str(name)).unwrap();

        self
    }
}

pub trait AnyJsPropsBuilder {
    fn set_prop(&mut self, name: &str, value: &JsValue) -> &mut Self;
    fn remove_prop(&mut self, name: &str) -> &mut Self;
    fn set_children(&mut self, children: Option<crate::Children>) -> &mut Self;
}

#[derive(Debug, Default, Clone)]
pub struct AnyJsStaticPropsPersistedValue(HashMap<Cow<'static, str>, Rc<dyn Any>>);

impl AnyJsStaticPropsPersistedValue {
    pub fn replace_static(&mut self, name: &'static str, value: Rc<dyn Any>) {
        self.0.insert(Cow::Borrowed(name), value);
    }
    pub fn replace(&mut self, name: String, value: Rc<dyn Any>) {
        self.0.insert(Cow::Owned(name), value);
    }
    pub fn remove(&mut self, name: &str) {
        self.0.remove(name);
    }
}

#[derive(Debug, Default, Clone)]
pub struct AnyJsStaticProps {
    pub props: AnyJsProps,
    pub persisted: AnyJsStaticPropsPersistedValue,
}

impl AnyJsStaticProps {
    #[inline]
    pub fn set_static_prop_and_persist(
        &mut self,
        name: &'static str,
        value: PassedToJsRuntime,
    ) -> &mut Self {
        self.props.set_prop(name, &value.js_value);
        if let Some(p) = value.to_persist {
            self.persisted.replace_static(name, p.into());
        } else {
            self.persisted.remove(name);
        }
        self
    }
}

impl AnyJsPropsBuilder for AnyJsStaticProps {
    #[inline]
    fn set_prop(&mut self, name: &str, value: &JsValue) -> &mut Self {
        self.props.set_prop(name, value);
        self.persisted.remove(name);
        self
    }

    #[inline]
    fn set_children(&mut self, children: Option<crate::Children>) -> &mut Self {
        self.props.set_children(children);
        self
    }

    fn remove_prop(&mut self, name: &str) -> &mut Self {
        self.props.remove_prop(name);
        self.persisted.remove(name);

        self
    }
}

impl crate::Props for AnyJsProps {
    type InitialBuilder = Self;

    #[inline]
    fn init_builder() -> Self::InitialBuilder {
        Default::default()
    }
}

impl crate::Props for AnyJsStaticProps {
    type InitialBuilder = Self;

    #[inline]
    fn init_builder() -> Self::InitialBuilder {
        Default::default()
    }
}

impl crate::PropsBuilder<Self> for AnyJsProps {
    #[inline]
    fn build(self) -> Self {
        self
    }
}

impl crate::PropsBuilder<Self> for AnyJsStaticProps {
    #[inline]
    fn build(self) -> Self {
        self
    }
}
