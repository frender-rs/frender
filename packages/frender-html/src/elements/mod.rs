// TODO: refactor
pub use crate::render_state::non_reactive;

pub mod array;
pub mod boxed;
#[cfg(feature = "either")]
pub mod either;
pub mod option;
pub mod str;
pub mod tuple;

pub use crate::intrinsic::csr as intrinsic;

mod empty;

enum Never {}

pub(crate) struct Kind<K>(std::marker::PhantomData<K>, Never);
