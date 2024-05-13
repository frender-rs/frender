mod value_kind;
pub use value_kind::ValueKind;

mod intrinsic_component;
pub use intrinsic_component::*;

pub mod maybe_value;
pub use maybe_value::{MaybeValue, ValueUpdater};

pub mod attr;

pub use frender_dom_tokens::{self as dom_tokens, ChainableDomTokens, DomTokenList, DomTokens};

#[cfg(feature = "web")]
pub mod web;

mod content_editable;
pub use content_editable::ContentEditable;

mod spellcheck;
pub use spellcheck::Spellcheck;

pub mod maybe_str;

mod string_value;
pub use string_value::StringValue;

use frender_common::impl_many;

fn bool_to_str(this: bool) -> &'static str {
    if this {
        "true"
    } else {
        "false"
    }
}
