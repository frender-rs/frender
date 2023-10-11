mod intrinsic_component;
mod update_with_state;
pub use intrinsic_component::*;
pub use update_with_state::*;

pub mod attr;
pub mod dom_token;
pub use dom_token::DomTokens;
pub mod content_editable;
pub use content_editable::MaybeContentEditable;
pub mod css;
pub use css::Css;

pub use frender_common::write::attrs::{AsyncWritableAttrs, IntoAsyncWritableAttrs};

use frender_common::impl_many;
