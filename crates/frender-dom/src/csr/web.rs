use std::borrow::Cow;

pub use frender_events::web::Event;

use frender_common::try_behavior::TryBehavior;

pub use dom_token_list::DomTokenList;

pub mod event;

mod dom_token_list;

#[derive(Debug)]
pub struct Node<N>(pub N);

pub trait Renderer {
    fn document(&self) -> Cow<web_sys::Document>;

    type TryBehavior<'a>: TryBehavior
    where
        Self: 'a;
    fn try_behavior(&mut self) -> Self::TryBehavior<'_>;

    fn cursor_is_at_node(&self, node: &web_sys::Node) -> bool;
    fn move_cursor_after_node(&mut self, node: &web_sys::Node);
    fn readd_node(&mut self, node: &web_sys::Node, force_reposition: bool);
    fn remove_node(&mut self, node: &web_sys::Node);

    fn move_cursor_at_the_first_child_of_element(&mut self, element: &web_sys::Element);
}

pub struct UnwrapThrow;

impl TryBehavior for UnwrapThrow {
    fn unwrap_result<T, E>(&mut self, result: Result<T, E>) -> T
    where
        E: std::fmt::Debug,
    {
        use wasm_bindgen::UnwrapThrowExt;
        result.unwrap_throw()
    }

    fn unwrap_option<T>(&mut self, option: Option<T>) -> T {
        use wasm_bindgen::UnwrapThrowExt;
        option.unwrap_throw()
    }
}

#[cfg(aaa)]
mod impl_node_behaviors {
    use frender_common::try_behavior::TryWithTryBehavior;

    use crate::renderer::node_behaviors;
    impl<E: AsRef<web_sys::Node>, R: ?Sized + super::Renderer> node_behaviors::Node<R>
        for super::Node<E>
    {
        fn cursor_is_at_self(&self, renderer: &R) -> bool {
            renderer.cursor_is_at_node(self.0.as_ref())
        }

        fn move_cursor_after_self(&mut self, renderer: &mut R) {
            renderer.move_cursor_after_node(self.0.as_ref())
        }

        fn readd_self(&mut self, renderer: &mut R, force_reposition: bool) {
            renderer.readd_node(self.0.as_ref(), force_reposition)
        }

        fn remove_self(&mut self, renderer: &mut R) {
            renderer.remove_node(self.0.as_ref())
        }
    }

    impl<E: AsRef<web_sys::Element> + AsRef<web_sys::Node>, R: ?Sized + super::Renderer>
        node_behaviors::Element<R> for super::Node<E>
    {
        fn move_cursor_at_the_first_child_of_self(&mut self, renderer: &mut R) {
            renderer.move_cursor_at_the_first_child_of_element(self.0.as_ref())
        }

        fn set_attribute(&mut self, renderer: &mut R, name: &str, value: &str) {
            AsRef::<web_sys::Element>::as_ref(&self.0)
                .set_attribute(name, value)
                .unwrap_with_behavior(&mut renderer.try_behavior())
        }

        fn remove_attribute(&mut self, renderer: &mut R, name: &str) {
            AsRef::<web_sys::Element>::as_ref(&self.0)
                .remove_attribute(name)
                .unwrap_with_behavior(&mut renderer.try_behavior())
        }

        type ClassList<'a> = super::DomTokenList<R::TryBehavior<'a>>
        where
            Self: 'a,
            R: 'a;

        fn class_list<'a>(&'a mut self, renderer: &'a mut R) -> Self::ClassList<'a> {
            let element: &web_sys::Element = self.0.as_ref();
            super::DomTokenList(element.class_list(), renderer.try_behavior())
        }

        fn set_id(&mut self, renderer: &mut R, id: &str) {
            AsRef::<web_sys::Element>::as_ref(&self.0).set_id(id)
        }
    }
}
