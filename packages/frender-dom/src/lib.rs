pub use self::tag::HasIntrinsicComponentTag;

pub use frender_common::Empty;

pub use frender_events::{event, event_types, HasEventTypeName};

pub mod style {
    pub mod csr {
        pub use frender_style::css_style_declaration::{CssStyleDeclaration, Priority};
    }
}

mod tag;

#[cfg(feature = "csr")]
pub mod csr;

#[cfg(feature = "ssr")]
pub mod ssr;

pub mod script;

pub use frender_attr_value as attr_value;

pub mod special;

pub mod node_ref;

pub mod string_element;

pub mod attrs;

#[cfg(feature = "web")]
mod shims;
