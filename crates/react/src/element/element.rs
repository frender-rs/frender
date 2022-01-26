use crate::Node;

pub trait Element: Node {
    fn as_react_element(&self) -> react_sys::Element;
}

impl Element for react_sys::Element {
    fn as_react_element(&self) -> react_sys::Element {
        self.clone()
    }
}
