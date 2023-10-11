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
pub mod event_types;

mod render_element;
pub use render_element::RenderElement;

pub use frender_html_common::dom_token::DomTokenList;

// pub mod data_transfer;
pub mod touch;

macro_rules! expand {
    (if (                 ) { $($expand:tt)* }) => {};
    (if ($($predicate:tt)+) { $($expand:tt)* }) => { $($expand)* };
    ({$($t:tt)*}) => { $($t)* };
    ({$($t:tt)*} if (                 ) $($commands:tt)*) => {
        $crate::expand! {{      } $($commands)*}
    };
    ({$($t:tt)*} if ($($predicate:tt)+) $($commands:tt)*) => {
        $crate::expand! {{$($t)*} $($commands)*}
    };
    ({$($t:tt)*} prepend ($($prepend:tt)*) $($commands:tt)*) => {
        $crate::expand! {{$($prepend)* $($t)*} $($commands)*}
    };
    ({$($t:tt)*} do {$($do_commands:tt)*} $($commands:tt)*) => {
        $crate::expand! {{$($t)*} $($do_commands)* $($commands)*}
    };
    ({$($t:tt)*} wrap {} $($commands:tt)*) => {
        $crate::expand! {{{$($t)*}} $($commands)*}
    };
    // $each must be wrapped with `{}`
    // $commands must be wrapped with `{}`
    (while ($($each:tt)*) $commands:tt) => {
        $($crate::expand! { $each do $commands })*
    };
}

use expand;
