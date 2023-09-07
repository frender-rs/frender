mod intrinsic_component;
mod update_with_state;
pub use intrinsic_component::*;
pub use update_with_state::*;

pub mod attr;
pub mod class;
pub mod dom_token;
pub use dom_token::DomTokens;
pub mod content_editable;
pub use content_editable::MaybeContentEditable;
pub mod css;
pub use css::Css;

pub use frender_common::write::attrs::{AsyncWritableAttrs, IntoAsyncWritableAttrs};

macro_rules! impl_many {
    (
        impl<__> $impl_trait:ident for each_of![$($ty:ty),* $(,)?]
        $impl_block:tt
    ) => {$(
        impl $impl_trait for $ty
        $impl_block
    )*};
    (
        impl<__> $impl_trait:ident <$t:ty> for each_of![$($ty:ty),* $(,)?]
        $impl_block:tt
    ) => {$(
        impl $impl_trait <$t> for $ty
        $impl_block
    )*};
}

pub(crate) use impl_many;
