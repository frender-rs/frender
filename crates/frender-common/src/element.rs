use std::{pin::Pin, task::Poll};

use crate::write::element::AsyncWritableElement;

pub trait IntoAsyncWritableElement {
    type IntoAsyncWritableElement: AsyncWritableElement;

    fn into_async_writable_element(self) -> Self::IntoAsyncWritableElement;
}
