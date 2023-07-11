use std::{rc::Rc, sync::Arc};

use super::SlicedBytes;

macro_rules! define_enum {
    (
        @impl
        $vis:vis enum $ident:ident <$lt:lifetime> {
            $(
                $variant:ident ($variant_ty:ty)
            ),* $(,)?
        }
    ) => {
        $(
            impl<$lt> From<$variant_ty> for $ident<$lt> {
                fn from(value: $variant_ty) -> Self {
                    Self::$variant(value)
                }
            }
        )*

        impl<$lt> AsRef<[u8]> for $ident<$lt> {
            fn as_ref(&self) -> &[u8] {
                match self {$(
                    Self::$variant(b) => b.as_ref(),
                )*}
            }
        }

        impl<$lt> super::AsyncWritableBytes for $ident<$lt> {
            fn truncate_start_at(&mut self, n: usize) {
                match self {$(
                    Self::$variant(b) => b.truncate_start_at(n),
                )*}
            }
        }
    };
    ($($t:tt)*) => {
        $($t)*
        define_enum! { @impl $($t)* }
    };
}

define_enum!(
    pub enum AnyBytes<'a> {
        Borrowed(&'a [u8]),
        Owned(SlicedBytes<Vec<u8>>),
        Rc(SlicedBytes<Rc<[u8]>>),
        Arc(SlicedBytes<Arc<[u8]>>),
    }
);

impl<'a> From<super::CowSlicedBytes<'a>> for AnyBytes<'a> {
    fn from(value: super::CowSlicedBytes<'a>) -> Self {
        match value {
            super::CowSlicedBytes::Borrowed(b) => Self::Borrowed(b),
            super::CowSlicedBytes::Owned(b) => Self::Owned(b),
        }
    }
}
