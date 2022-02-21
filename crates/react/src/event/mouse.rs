// use std::marker::PhantomData;

// use super::js;

// pub struct MouseEvent<TCurrent = js::native::Element, TEvent = js::native::MouseEvent> {
//     base: js::MouseEvent,
//     _phantom: PhantomData<(TCurrent, TEvent)>,
// }

// impl<TCurrent, TEvent> AsRef<super::UiEvent<TCurrent, TEvent>> for MouseEvent<TCurrent, TEvent> {
//     fn as_ref(&self) -> &super::UiEvent {
//         todo!()
//     }
// }
