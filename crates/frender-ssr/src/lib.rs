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

pub mod write_attrs;

mod str_iter;

pub use str_iter::{AsyncStrIterator, Chain, Encode, IntoAsyncStrIterator};

pub(crate) use frender_common::{ready, ready_ok, ready_ok_rewrap_err};

pub mod __private {
    pub use frender_common::{expand, ready};
    pub use pin_project_lite::pin_project;
}

#[macro_export]
macro_rules! ready_none {
    ($e:expr $(, {$($do_if_ready:tt)*})?) => {
        match $e {
            ret @ ::core::task::Poll::Ready(ref v) => {
                $($($do_if_ready)*)?
                if let ::core::option::Option::Some(_) = v {
                    return ret;
                }
            }
            ret @ ::core::task::Poll::Pending => return ret,
        }
    };
}
