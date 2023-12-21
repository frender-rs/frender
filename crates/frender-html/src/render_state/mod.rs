use std::{pin::Pin, task::Poll};

pub mod array;
#[cfg(feature = "either")]
pub mod either;
pub mod non_reactive;
pub mod option;
pub mod tuple;

pub trait RenderState<R> {
    fn unmount(self: Pin<&mut Self>, renderer: &mut R);

    fn state_unmount(self: Pin<&mut Self>);

    fn poll_render(self: Pin<&mut Self>, renderer: &mut R, cx: &mut std::task::Context<'_>) -> Poll<()>;
}
