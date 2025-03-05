pub use self::with_fn::WithFn;
pub use self::with_memo::WithMemo;
#[cfg(feature = "ToElement")]
pub use self::with_to_element::WithToElement;

use hooks::ShareValue;

mod with_fn;
mod with_memo;
#[cfg(feature = "ToElement")]
mod with_to_element;

mod bound;

#[derive(Debug, Clone, Copy)]
pub struct SignalIntoElement<S: ShareValue, F>(pub S, pub F);

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;
