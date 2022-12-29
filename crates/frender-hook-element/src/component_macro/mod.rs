#[cfg(all(feature = "ssr", feature = "dom"))]
pub mod dom_ssr;

#[cfg(all(feature = "ssr", not(feature = "dom")))]
pub mod only_ssr;

#[cfg(all(feature = "dom", not(feature = "ssr")))]
pub mod only_dom;

pub use hooks::core as hooks_core;
pub use hooks::hook;
