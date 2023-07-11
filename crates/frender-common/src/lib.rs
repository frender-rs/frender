mod keyed;
pub use keyed::*;

#[doc(hidden)]
/// This is only for inner usage of frender
pub mod utils;

pub mod write;

#[macro_export]
macro_rules! ready_ok {
    ($e:expr) => {
        match $e {
            ::core::task::Poll::Ready(::core::result::Result::Ok(v)) => v,
            non_ready_ok => return non_ready_ok,
        }
    };
}

#[macro_export]
macro_rules! ready_ok_rewrap_err {
    ($e:expr) => {
        match $e {
            ::core::task::Poll::Ready(::core::result::Result::Ok(v)) => v,
            ::core::task::Poll::Ready(::core::result::Result::Err(e)) => {
                return ::core::task::Poll::Ready(::core::result::Result::Err(e))
            }
            ::core::task::Poll::Pending => return ::core::task::Poll::Pending,
        }
    };
}
