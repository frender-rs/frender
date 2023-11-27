pub use frender_ssr_html as html;

mod trait_element;
pub use trait_element::*;

mod ssr_ext;
pub use ssr_ext::*;

pub mod element;

pub mod async_str {
    pub use async_str_iter::{chain, empty, flat, option};
    pub use frender_ssr_html::encode;
}
pub use async_str::{chain::Chain, empty::Empty, encode::Encode};
pub use async_str_iter::{AsyncStrIterator, IntoAsyncStrIterator};
