//! # Design principles
//!
//! props builder methods should accept all of the following types:
//!
//! ```
//! # use std::rc::Rc;
//! # use frender::IntoPropsValue;
//! # struct MyPropsBuilder;
//! # impl MyPropsBuilder {
//! #     fn on_click<H>(&self, _handler: H) {}
//! # }
//! # let builder = MyPropsBuilder;
//! builder.on_click( || {} );
//! builder.on_click( |_event| {} );
//! builder.on_click( Rc::new(|| {}) );
//! builder.on_click( Rc::new(|_event| {}) );
//! builder.on_click( &Rc::new(|| {}) );
//! builder.on_click( &Rc::new(|_event| {}) );
//! ```
//!
//! And also [`Option<T>`] of the above types if allowed.

// use super::MouseEvent;

// pub trait MouseEventHandler<TCurrent, TEvent> {
//     fn handle_mouse_event(&self, event: MouseEvent<TCurrent, TEvent>);
// }

// impl<TCurrent, TEvent, F: Fn(MouseEvent<TCurrent, TEvent>)> MouseEventHandler<TCurrent, TEvent>
//     for F
// {
//     fn handle_mouse_event(&self, event: MouseEvent<TCurrent, TEvent>) {
//         self(event)
//     }
// }

// impl<TCurrent, TEvent, F: Fn(MouseEvent<TCurrent, TEvent>)> crate::IntoJsRuntime for F
// where
//     TCurrent: std::any::Any,
//     TEvent: std::any::Any,
// {
//     fn into_js_runtime(self) -> crate::PassToJsRuntimeValue {
//         todo!()
//     }
// }
