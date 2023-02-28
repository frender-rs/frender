pub mod kw;

mod event_listener;
mod field;
mod inherit;
mod maybe;
mod maybe_details;
mod mod_info;
mod virtual_fields;

pub use event_listener::*;
pub use field::*;
pub use inherit::*;
pub use maybe::*;
pub use maybe_details::*;
pub use mod_info::*;
pub use virtual_fields::*;

mod data;
pub use data::*;

mod simple;
mod to_tokens;
pub use simple::*;
pub use to_tokens::*;

mod with_mod_info;
pub use with_mod_info::*;

pub use frender_macro_utils as utils;

pub fn into_ts(
    value: utils::prefix_path::PrefixPath<IntrinsicComponentPropsDataWithModInfo>,
) -> proc_macro2::TokenStream {
    value.map(IntrinsicComponentPropsDataWithModInfo::into_ts)
}
