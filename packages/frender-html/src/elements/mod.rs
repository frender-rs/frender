pub mod array;
pub mod boxed;
pub mod either;
pub mod option;
pub mod str;
pub mod tuple;

pub use crate::intrinsic::csr as intrinsic;

mod empty;

enum Never {}

pub(crate) struct Kind<K>(std::marker::PhantomData<K>, Never);
