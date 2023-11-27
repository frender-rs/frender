pub use frender_ssr_html as html;

mod maybe_owned;
pub use maybe_owned::*;

mod async_writable;
pub use async_writable::*;

mod into_static_str;
pub use into_static_str::*;

mod render_state;
pub use render_state::*;

mod trait_element;
pub use trait_element::*;

// TODO: refactor
pub use escape_safe::*;
use frender_ssr_html::escape_safe;

mod ssr_ext;
pub use ssr_ext::*;

mod truncate_start_at;
pub use truncate_start_at::*;

pub mod bytes;

pub use futures_io::AsyncWrite;
pub use html_escape;

pub mod element;

pub mod utils;

pub mod write_attrs;

pub mod async_str {
    pub use async_str_iter::{chain, empty, flat, option};
    pub use frender_ssr_html::encode;
}
pub use async_str::{chain::Chain, empty::Empty, encode::Encode};
pub use async_str_iter::{AsyncStrIterator, IntoAsyncStrIterator};

pub(crate) use frender_common::{ready, ready_none, ready_ok, ready_ok_rewrap_err};

pub mod __private {
    pub use frender_common::{expand, ready, ready_none};
    pub use pin_project_lite::pin_project;
}
