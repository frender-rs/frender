#![cfg(prototyping)] // TODO: remove

use std::{io, pin::Pin, task::Poll};

use futures_io::AsyncWrite;

pub trait AsyncWritable {
    fn poll_write_all<W: AsyncWrite>(
        this: Pin<&mut Self>,
        writer: Pin<&mut W>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<io::Result<()>>;
}

pub trait IntoAsyncWritable: Sized {
    type AsyncWritable: AsyncWritable;
    fn into_async_writable(this: Self) -> Self::AsyncWritable;
}

impl IntoAsyncWritable for () {
    type AsyncWritable = Nothing;

    fn into_async_writable(_: Self) -> Self::AsyncWritable {
        Nothing
    }
}

impl<A: IntoAsyncWritable, B: IntoAsyncWritable> IntoAsyncWritable for (A, B) {
    type AsyncWritable = Chain<A::AsyncWritable, B::AsyncWritable>;

    fn into_async_writable(this: Self) -> Self::AsyncWritable {
        Chain {
            a: A::into_async_writable(this.0),
            b: B::into_async_writable(this.1),
            stage: Stage::NoneDone,
        }
    }
}

enum Stage {
    NoneDone,
    ADone,
    AllDone,
    Error,
}

pub struct Nothing;

impl AsyncWritable for Nothing {
    fn poll_write_all<W: AsyncWrite>(
        _: Pin<&mut Self>,
        _: Pin<&mut W>,
        _: &mut std::task::Context<'_>,
    ) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}

pin_project_lite::pin_project!(
    pub struct Chain<A, B> {
        #[pin]
        a: A,
        #[pin]
        b: B,
        stage: Stage,
    }
);

impl<A, B> Chain<A, B> {}

impl<A: AsyncWritable, B: AsyncWritable> AsyncWritable for Chain<A, B> {
    fn poll_write_all<W: AsyncWrite>(
        this: Pin<&mut Self>,
        mut writer: Pin<&mut W>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<io::Result<()>> {
        let this = this.project();

        macro_rules! continue_with {
            ($e:expr, $next_stage:expr) => {
                match $e {
                    Poll::Ready(Ok(())) => {
                        *this.stage = $next_stage;
                    }
                    err @ Poll::Ready(Err(_)) => {
                        *this.stage = Stage::Error;
                        return err;
                    }
                    Poll::Pending => return Poll::Pending,
                }
            };
        }

        if let Stage::Error = this.stage {
            return Poll::Ready(Err(io::ErrorKind::Other.into()));
        }

        if let Stage::NoneDone = this.stage {
            continue_with!(A::poll_write_all(this.a, writer.as_mut(), cx), Stage::ADone)
        }

        if let Stage::ADone = this.stage {
            continue_with!(
                B::poll_write_all(this.b, writer.as_mut(), cx),
                Stage::AllDone
            )
        }

        assert!(matches!(this.stage, Stage::AllDone));
        Poll::Ready(Ok(()))
    }
}

mod chain_macro {
    macro_rules! Chain {
        ($ty1:ty $(,)?) => {
            $ty1
        };
        ($ty1:ty , $($ty:ty),+ $(,)?) => {
            $crate::async_writable::Chain::<
                $ty1,
                $crate::async_writable::Chain![$($ty),+]
            >
        };
    }
    pub(crate) use Chain;
}

pub(crate) use chain_macro::*;
