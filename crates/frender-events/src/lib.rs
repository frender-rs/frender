mod event;
pub use event::*;

pub mod events;

pub mod callback;

pub fn callback<Out>(f: fn() -> Out) -> fn() -> Out {
    f
}

pub use callback::{Callable, Callback, IsCallable};

#[doc(hidden)]
#[macro_export]
macro_rules! __expand_or {
    ([         ] $($b:tt)*) => { $($b)* };
    ([$($a:tt)+] $($b:tt)*) => { $($a)+ };
}

macro_rules! impl_callback_parse_input {
    ($dollar:tt $([$($match:tt)*][$($clone:tt)*] => $output_path:ident $output_input:tt )*) => {
        #[macro_export]
        macro_rules! __callback_parse_input {
            // end input
            ([| $dollar($other:tt)*] $_clone:tt $path:tt $input:tt ) => {
                $crate::callback! { ![$path $input] $dollar($other)*  }
            };
            $(
                (   [$($match)* , $dollar($_rest:tt)*]
                    [$($clone)* , $dollar($ rest:tt)*]
                    {$dollar($paths:ident)*}
                    {$dollar($inputs:tt  )*}
                ) => {
                    $crate::__callback_parse_input! {
                        [$dollar($_rest)*]
                        [$dollar($ rest)*]
                        {$dollar($paths )* $output_path }
                        {$dollar($inputs)* $output_input}
                    }
                };
                (   [$($match)* | $dollar($_rest:tt)*]
                    [$($clone)* | $dollar($ rest:tt)*]
                    {$dollar($paths:ident)*}
                    {$dollar($inputs:tt  )*}
                ) => {
                    $crate::callback! { ![
                        {$dollar($paths )* $output_path }
                        {$dollar($inputs)* $output_input}
                    ] $dollar($rest)* }
                };
            )*
        }
    };
}

impl_callback_parse_input! { $
    // &mut _
    [&     mut   $_input:pat_param][$r:tt $m:tt $ input:pat_param] => r#mut[$r $m $input]
    // &mut v: _
    [&       mut     $($_input:ident)+ :                $_input_ty:ty][$r:tt $m:tt $($ input:ident)+ :                $ input_ty:ty] => r#mut[$r $m $($input)+ : $input_ty]
    // &mut _: _
    [&       mut       $_input:tt      :      $_input_ty:ty][$r:tt $m:tt   $ input:tt      :      $ input_ty:ty]=> r#mut [$r $m   $input   : $input_ty]
    // v: &mut _
    [                $($_input:ident)+ : &mut $_input_ty:ty][                $($ input:ident)+ : $r:tt $m:tt $ input_ty:ty]=> r#mut [$r $m   $input   : $input_ty]
    // _: &mut _
    [                  $_input:tt      : &mut $_input_ty:ty , $($_rest:tt)*][                  $ input:tt      : &mut $ input_ty:ty , $($ rest:tt)*]=> r#mut[$input : &mut $input_ty]

    // &_
    [&    $_input:pat_param][$r:tt $input:pat_param] => r#ref[$r $input]
    // &v: _
    [&    $($_input:ident)+ : $_input_ty:ty][$r:tt    $($input:ident)+ : $input_ty:ty] => r#ref[$r $($input)+ : $input_ty]
    // &_: _
    [&    $_input:tt : $_input_ty:ty][$r:tt    $input:tt : $input_ty:ty] => r#ref[$r $input : $input_ty]
    // v: &_
    [$($_input:ident)+ : & $_input_ty:ty ][$($input:ident)+ : $r:tt $input_ty:ty ] => r#ref[$($input)+ : $r $input_ty]
    // _: &_
    [$_input:tt : & $_input_ty:ty ][$input:tt : $r:tt $input_ty:ty ] => r#ref[$input : $r $input_ty]

    // |_| {}
    [$_input:pat_param][$input:pat_param] => value[$input]
    // |v: _| {}
    [$($_input:ident)+ :   $_input_ty:ty][$($input:ident)+ :   $input_ty:ty] => value[$($input)+ : $input_ty]
    // |_: _| {}
    [$_input:tt :   $_input_ty:ty][$input:tt :   $input_ty:ty] => value[$input : $input_ty]
}

#[macro_export]
macro_rules! callback {
    (|| $($rest:tt)*) => {
        $crate::callback! { ![{} {}] $($rest)*  }
    };
    (|  $($rest:tt)* ) => {
        $crate::__callback_parse_input! { [$($rest)*][$($rest)*]{}{} }
    };
    // - implementation details
    //
    // -- input resolved
    //
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
    (@[{$($method_path:tt)*}{$([$($input:tt)*])*}][$($output:ty)?][$($body:tt)*]) => {
        $crate::callback $(::$method_path)*                                (
            |$($($input)*),*               | $(-> $output)? $($body)*
        )
    };
    // one  state
    (@[{$($method_path:tt)*}{$([$($input:tt)*])*}][$($output:ty)?][$($body:tt)*]   $state:ident $(= $state_expr:expr)?    $(,)?) => {
        $crate::callback $(::$method_path)* ::r#ref::provide_last_argument (
            |$($($input)* ,)* $state       | $(-> $output)? $($body)* ,
            $crate::__expand_or!([$($state_expr)?] $state),
        )
    };
    // many states
    (@[{$($method_path:tt)*}{$([$($input:tt)*])*}][$($output:ty)?][$($body:tt)*] $($state:ident $(= $state_expr:expr)?),+ $(,)?) => {
        $crate::callback $(::$method_path)* ::r#ref::provide_last_argument (
            |$($($input)* ,)* ($($state,)+)| $(-> $output)? $($body)* ,
            ($( $crate::__expand_or!([$($state_expr)?] $state) ,)+),
        )
    };
}
