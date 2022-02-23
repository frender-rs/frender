mod into_ref_value;
mod traits;
mod use_ref;
mod use_ref_cell;
mod use_ref_js;
mod use_ref_readonly;

pub use into_ref_value::*;
pub use traits::*;
pub use use_ref::*;
pub use use_ref_cell::*;
pub use use_ref_js::*;
pub use use_ref_readonly::*;

/// `use_ref` macro will auto wrap value into [`Rc<T>`](std::rc::Rc).
#[macro_export]
macro_rules! use_ref {
    (move || $e:expr) => {
        $crate::use_ref_with(move || $crate::auto_wrap_rc!($e))
    };
    (readonly move || $e:expr) => {
        $crate::use_ref_readonly_with(move || $crate::auto_wrap_rc!($e))
    };
    (|| $e:expr) => {
        $crate::use_ref_with(|| $crate::auto_wrap_rc!($e))
    };
    (readonly || $e:expr) => {
        $crate::use_ref_readonly_with(|| $crate::auto_wrap_rc!($e))
    };
    ($e:expr) => {
        $crate::use_ref($crate::auto_wrap_rc!($e))
    };
    (readonly $e:expr) => {
        $crate::use_ref_readonly($crate::auto_wrap_rc!($e))
    };
    (set_as $e:expr) => {
        $crate::use_ref_set_as($crate::auto_wrap_rc!($e))
    };
}
