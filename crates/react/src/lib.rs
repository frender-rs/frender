// mod use_closure;
mod take_rc;
// mod use_closure;
mod closure;
mod component;
mod element;
mod fragment;
mod js_adapter;
mod js_props_bridge;
mod js_runtime;
mod key;
mod node;
mod simple_take;
mod use_effect;
mod use_memo;
mod use_ref;
mod use_ref_cell;
mod use_state;

pub use closure::*;
pub use component::*;
pub use element::*;
pub use fragment::*;
pub use js_adapter::*;
pub use js_props_bridge::*;
pub use js_runtime::*;
pub use key::*;
pub use node::*;
pub use simple_take::*;
pub use take_rc::*;
pub use use_effect::*;
pub use use_memo::*;
pub use use_ref::*;
pub use use_ref_cell::*;
pub use use_state::*;

pub mod html;

mod utils;

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
