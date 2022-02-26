// mod use_closure;
mod take_rc;
// mod use_closure;
mod common_props;
mod component;
mod element;
mod fragment;
mod into_prop_value;
mod key;
mod node;
mod render_into_dom;
mod safe_into_js;
mod strict_mode;
mod use_effect;
mod use_memo;
mod use_ref;
mod use_state;

pub use common_props::*;
pub use component::*;
pub use element::*;
pub use fragment::*;
pub use into_prop_value::*;
pub use key::*;
pub use node::*;
pub use render_into_dom::*;
pub use safe_into_js::*;
pub use strict_mode::*;
pub use take_rc::*;
pub use use_effect::*;
pub use use_memo::*;
pub use use_ref::*;
pub use use_state::*;

pub mod __private;
pub mod any_js_props;
pub mod event;

pub use react_sys as sys;

/// A shorthand to create tuple of react children.
///
/// If there is no children nodes, then it will be interpreted as `()`.
///
/// If there is only one expression, then it will be interpreted as
/// the value it self (not wrapped in a single element tuple).
///
/// If there are multiple expressions separated by `,`,
/// then it represents a tuple of these expressions.
///
/// Note that if a tuple contains more than 17 elements,
/// then `react::Node` is NOT implemented for it!
#[macro_export]
macro_rules! children {
    () => {()};
    ($($e:expr),+ $(,)?) => {
        (
            $($e),*
        )
    };
}

mod js_props_bridge;
pub(crate) use js_props_bridge::*;
