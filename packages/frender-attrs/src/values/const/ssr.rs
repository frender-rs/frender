use crate::ssr::SsrAttributes;

use super::{ConstAttributes, HasConstAttributes};

impl<T: ?Sized + HasConstAttributes> SsrAttributes for ConstAttributes<T> {}
