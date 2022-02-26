use convert_js::FromJs;
use std::{convert::Infallible, marker::PhantomData, ops::Deref};
use wasm_bindgen::{JsCast, JsValue};

use super::js;

pub struct SyntheticEvent<
    TCurrent = js::native::Element,
    TNativeEvent = js::native::Event,
    TBaseSyntheticEvent = js::BaseSyntheticEvent,
> {
    base: TBaseSyntheticEvent,
    _phantom: PhantomData<(TCurrent, TNativeEvent)>,
}

impl<TCurrent, TNativeEvent, TBaseSyntheticEvent: PartialEq> PartialEq
    for SyntheticEvent<TCurrent, TNativeEvent, TBaseSyntheticEvent>
{
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base
    }
}

impl<TCurrent, TNativeEvent, TBaseSyntheticEvent: Eq> Eq
    for SyntheticEvent<TCurrent, TNativeEvent, TBaseSyntheticEvent>
{
}

impl<TCurrent, TNativeEvent, TBaseSyntheticEvent: std::fmt::Debug> std::fmt::Debug
    for SyntheticEvent<TCurrent, TNativeEvent, TBaseSyntheticEvent>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!(
            "SyntheticEvent<{}>",
            std::any::type_name::<TBaseSyntheticEvent>()
        ))
        .field("event", &self.base)
        .field("current_target", &std::any::type_name::<TCurrent>())
        .field("native_event", &std::any::type_name::<TNativeEvent>())
        .finish()
    }
}

impl<TCurrent, TNativeEvent, TBaseSyntheticEvent: Clone> Clone
    for SyntheticEvent<TCurrent, TNativeEvent, TBaseSyntheticEvent>
{
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone(),
            _phantom: self._phantom,
        }
    }
}

impl<TCurrent, TNativeEvent, TBaseSyntheticEvent: JsCast> FromJs
    for SyntheticEvent<TCurrent, TNativeEvent, TBaseSyntheticEvent>
{
    type Error = Infallible;

    fn from_js(js_value: JsValue) -> Result<Self, Self::Error> {
        let base = js_value.unchecked_into();
        Ok(Self {
            base,
            _phantom: PhantomData,
        })
    }
}

impl<TCurrent, TNativeEvent, TBaseSyntheticEvent> Deref
    for SyntheticEvent<TCurrent, TNativeEvent, TBaseSyntheticEvent>
{
    type Target = TBaseSyntheticEvent;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<TCurrent, TNativeEvent, TBaseSyntheticEvent: AsRef<js::BaseSyntheticEvent>>
    SyntheticEvent<TCurrent, TNativeEvent, TBaseSyntheticEvent>
{
    pub fn native_event(&self) -> TNativeEvent
    where
        TNativeEvent: JsCast,
    {
        let raw = self.base.as_ref().native_event_raw();
        raw.unchecked_into()
    }

    pub fn current_target(&self) -> TCurrent
    where
        TCurrent: JsCast,
    {
        let raw = self.base.as_ref().current_target_raw();
        raw.unchecked_into()
    }
}
