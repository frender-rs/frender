mod async_writable;
pub use async_writable::*;

mod into_static_str;
pub use into_static_str::*;

mod render_state;
pub use render_state::*;

mod trait_element;
pub use trait_element::*;

mod escape_safe;
pub use escape_safe::*;

mod ssr_ext;
pub use ssr_ext::*;

mod truncate_start_at;
pub use truncate_start_at::*;

pub mod bytes;

pub use futures_io::AsyncWrite;
pub use html_escape;

pub mod element;

pub mod utils;

pub mod attrs;

macro_rules! ready_ok {
    ($e:expr) => {
        match $e {
            ::core::task::Poll::Ready(::core::result::Result::Ok(())) => {}
            non_ready_ok => return non_ready_ok,
        }
    };
}

pub(crate) use ready_ok;
