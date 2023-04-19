mod event;
pub use event::*;

pub mod events;

pub mod callback;

/// An identity function which implicit casts the closure as `fn(IN) -> Out`.
pub fn callback<IN, Out>(f: fn(IN) -> Out) -> fn(IN) -> Out {
    f
}

pub use callback::{Callback, CallbackExt, IsCallback};

#[doc(hidden)]
#[macro_export]
macro_rules! __expand_or {
    ([         ] $($b:tt)*) => { $($b)* };
    ([$($a:tt)+] $($b:tt)*) => { $($a)+ };
}

#[macro_export]
macro_rules! callback {
    // callback!(|&mut _| {})
    (|&mut $input:pat_param                     $(,)? | $($rest:tt)*) => {
        $crate::callback! {
            ![[$crate::callback::accept_mut][&mut $input]]
            $($rest)*
        }
    };
    // callback!(|&mut v: _| {})
    (|&mut $($input:ident)+ :      $input_ty:ty $(,)? | $($rest:tt)*) => {
        $crate::callback! {
            ![[$crate::callback::accept_mut][&mut $($input)+ : $input_ty]]
            $($rest)*
        }
    };
    // callback!(|&mut _: _| {})
    (|&mut $input:tt        :      $input_ty:ty $(,)? | $($rest:tt)*) => {
        $crate::callback! {
            ![[$crate::callback::accept_mut][&mut $input : $input_ty]]
            $($rest)*
        }
    };
    // callback!(|v: &mut _| {})
    (|     $($input:ident)+ : &mut $input_ty:ty $(,)? | $($rest:tt)*) => {
        $crate::callback! {
            ![[$crate::callback::accept_mut][$($input)+ : &mut $input_ty]]
            $($rest)*
        }
    };
    // callback!(|_: &mut _| {})
    (|     $input:tt        : &mut $input_ty:ty $(,)? | $($rest:tt)*) => {
        $crate::callback! {
            ![[$crate::callback::accept_mut][$input : &mut $input_ty]]
            $($rest)*
        }
    };

    // callback!(|&_| {})
    (|&    $input:pat_param                  $(,)? | $($rest:tt)*) => {
        $crate::callback! {
            ![[$crate::callback::accept_ref][&$input]]
            $($rest)*
        }
    };
    // callback!(|&v: _| {})
    (|&    $($input:ident)+ :   $input_ty:ty $(,)? | $($rest:tt)*) => {
        $crate::callback! {
            ![[$crate::callback::accept_ref][&$($input)+ : $input_ty]]
            $($rest)*
        }
    };
    // callback!(|&_: _| {})
    (|&    $input:tt        :   $input_ty:ty $(,)? | $($rest:tt)*) => {
        $crate::callback! {
            ![[$crate::callback::accept_ref][&$input : $input_ty]]
            $($rest)*
        }
    };
    // callback!(|v: &_| {})
    (|     $($input:ident)+ : & $input_ty:ty $(,)? | $($rest:tt)*) => {
        $crate::callback! {
            ![[$crate::callback::accept_ref][$($input)+ : &$input_ty]]
            $($rest)*
        }
    };
    // callback!(|_: &_| {})
    (|     $input:tt        : & $input_ty:ty $(,)? | $($rest:tt)*) => {
        $crate::callback! {
            ![[$crate::callback::accept_ref][$input : &$input_ty]]
            $($rest)*
        }
    };
    // callback!(|_| {})
    (|  $input:pat_param                  $(,)? | $($rest:tt)*) => {
        $crate::callback! {
            ![[$crate::callback][$input]]
            $($rest)*
        }
    };
    // callback!(|v: _| {})
    (|  $($input:ident)+ :   $input_ty:ty $(,)? | $($rest:tt)*) => {
        $crate::callback! {
            ![[$crate::callback][$($input)+ : $input_ty]]
            $($rest)*
        }
    };
    // callback!(|_: _| {})
    (|  $input:tt        :   $input_ty:ty $(,)? | $($rest:tt)*) => {
        $crate::callback! {
            ![[$crate::callback][$input : $input_ty]]
            $($rest)*
        }
    };

    // - implementation details
    //
    // -- input resolved

    // --- no explicit output type
    (!$input:tt                   $body:expr   $(, $($rest:tt)*)?) => {
        $crate::callback! { @$input [       ] [   $body   ] $($($rest)*)? }
    };
    // This branch is for dev experience
    (!$input:tt               { $($body:tt)* } $(, $($rest:tt)*)?) => {
        $crate::callback! { @$input [       ] [{$($body)*}] $($($rest)*)? }
    };
    // --- explicit output
    (!$input:tt -> $output:ty { $($body:tt)* } $(, $($rest:tt)*)?) => {
        $crate::callback! { @$input [$output] [{$($body)*}] $($($rest)*)? }
    };

    // -- output and body resolved
    // no   state
    (@[[$($method_path:tt)*][$($input:tt)*]][$($output:ty)?][$($body:tt)*]) => {
        $($method_path)*             (|$($input)*                | $(-> $output)? $($body)*)
    };
    // one  state
    (@[[$($method_path:tt)*][$($input:tt)*]][$($output:ty)?][$($body:tt)*]   $state:ident $(= $state_expr:expr)?    $(,)?) => {
        $($method_path)* ::with_state(|$($input)* , $state       | $(-> $output)? $($body)* ,     $crate::__expand_or!([$($state_expr)?] $state)      )
    };
    // many states
    (@[[$($method_path:tt)*][$($input:tt)*]][$($output:ty)?][$($body:tt)*] $($state:ident $(= $state_expr:expr)?),+ $(,)?) => {
        $($method_path)* ::with_state(|$($input)* , ($($state,)+)| $(-> $output)? $($body)* , ($( $crate::__expand_or!([$($state_expr)?] $state) ,)+) )
    };
}
