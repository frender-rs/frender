mod data_types;
pub use data_types::*;

pub mod props;

pub use frender_html_common::{AsyncWritableAttrs, DomTokens, IntoAsyncWritableAttrs};

pub mod renderer;
pub use renderer::{element_type_traits, element_types, RenderHtml};

mod element;
mod update_element;
pub use element::*;
pub use update_element::*;

pub mod elements;
pub mod pin_mut_maybe_uninit;

pub mod csr;

pub mod event;

mod render_element;
pub use render_element::RenderElement;

pub use frender_html_common::dom_token::DomTokenList;

// pub mod data_transfer;
pub mod touch;

use frender_common::expand;

mod shims;

pub mod html;

macro_rules! for_each_trait {
    ({$($vis:vis trait $trait_name:ident $body:tt)*} $commands:tt) => {
        $(crate::expand! {
            {$vis trait $trait_name $body}
            do $commands
        })*
    };
    (($($t:tt)*) $commands:tt) => {
        crate::for_each_trait! { {$($t)*} $commands }
    };
}

use for_each_trait;
