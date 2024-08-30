mod value_kind;
pub use value_kind::ValueKind;

pub mod maybe_value;
pub use maybe_value::{MaybeValue, ValueUpdater};

pub mod attr;

mod content_editable;
pub use content_editable::ContentEditable;

mod spellcheck;
pub use spellcheck::Spellcheck;

mod string_value;
pub use string_value::{EitherStringValue, MaybeStringValue, StringValue};

mod into_one_string_or_empty;
pub use into_one_string_or_empty::IntoOneStringOrEmpty;

mod empty;

use frender_common::impl_many;

fn bool_to_str(this: bool) -> &'static str {
    if this {
        "true"
    } else {
        "false"
    }
}
