mod keyed;
pub use keyed::*;

pub mod convert;
pub mod try_behavior;

#[doc(hidden)]
/// This is only for inner usage of frender
pub mod utils;

#[macro_export]
macro_rules! ready {
    ($e:expr $(,)?) => {
        match $e {
            ::core::task::Poll::Ready(t) => t,
            ::core::task::Poll::Pending => return ::core::task::Poll::Pending,
        }
    };
}

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
macro_rules! ready_some {
    ($e:expr) => {
        match $e {
            ::core::task::Poll::Ready(::core::option::Option::Some(v)) => v,
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

#[macro_export]
macro_rules! ready_none {
    ($e:expr) => {
        match $e {
            ::core::task::Poll::Ready(::core::option::Option::None) => {}
            ret => return ret,
        }
    };
}

#[macro_export]
macro_rules! expand {
    (if (                 ) { $($expand:tt)* } $(else { $($else:tt)* })?) => { $($($else)*)? };
    (if ($($predicate:tt)+) { $($expand:tt)* } $(else { $($else:tt)* })?) => { $($expand)* };
    ({$($t:tt)*}) => { $($t)* };
    ({$($t:tt)*} if (                 ) $($commands:tt)*) => {
        $crate::expand! {{      } $($commands)*}
    };
    ({$($t:tt)*} if ($($predicate:tt)+) $($commands:tt)*) => {
        $crate::expand! {{$($t)*} $($commands)*}
    };
    ({$($t:tt)*} unless (           ) $($commands:tt)*) => {
        $crate::expand! {{$($t)*} $($commands)*}
    };
    ({$($t:tt)*} unless ($($not:tt)+) $($commands:tt)*) => {
        $crate::expand! {{      } $($commands)*}
    };
    ({$($t:tt)*} prepend ($($prepend:tt)*) $($commands:tt)*) => {
        $crate::expand! {{$($prepend)* $($t)*} $($commands)*}
    };
    ({$($t:tt)*} prepend {$($prepend:tt)*} $($commands:tt)*) => {
        $crate::expand! {{$($prepend)* $($t)*} $($commands)*}
    };
    ({$($t:tt)*} append  ($($append:tt )*) $($commands:tt)*) => {
        $crate::expand! {{$($t)* $($append )*} $($commands)*}
    };
    ({$($t:tt)*} append  {$($append:tt )*} $($commands:tt)*) => {
        $crate::expand! {{$($t)* $($append )*} $($commands)*}
    };
    ({$($t:tt)*} do {$($do_commands:tt)*} $($commands:tt)*) => {
        $crate::expand! {{$($t)*} $($do_commands)* $($commands)*}
    };
    ({$($t:tt)*} wrap {} $($commands:tt)*) => {
        $crate::expand! {{{$($t)*}} $($commands)*}
    };
    ({$($t:tt)*} wrap () $($commands:tt)*) => {
        $crate::expand! {{($($t)*)} $($commands)*}
    };
    ({$($t:tt)+} or ($($or:tt)*) $($commands:tt)*) => {
        $crate::expand! {{$($t )+} $($commands)*}
    };
    ({         } or ($($or:tt)*) $($commands:tt)*) => {
        $crate::expand! {{$($or)*} $($commands)*}
    };
    ({$($t:tt)*} duplex_concat ({$($do_a:tt)*}{$($do_b:tt)*}) $($commands:tt)*) => {
        $crate::expand! {
            {$($t)*}
            $($do_a)*
            wrap ()
            prepend({$($t)*} $($do_b)* prepend)
            append($($commands)*)
            wrap {}
            prepend($crate::expand! )
        }
    };
    ({{$($first:tt)*} $({$($others:tt)*})*} trim {first} $($commands:tt)*) => {
        $crate::expand! { {$({$($others)*})*} $($commands)* }
    };
    ({{$($first:tt)*} $({$($others:tt)*})*} get {0} $($commands:tt)*) => {
        $crate::expand! { {$($first)*} $($commands)* }
    };
    ({{$($last:tt)*}                   } get {-1} $($commands:tt)*) => {
        $crate::expand! { {$($last)*} $($commands)* }
    };
    ({{$($item:tt)*} $({$($last:tt)*})+} get {-1} $($commands:tt)*) => {
        $crate::expand! { {$({$($last)*})+} get {-1} $($commands)* }
    };
    ({         } get_or_exit {0} $($commands:tt)*) => {};
    ({$($t:tt)+} get_or_exit {0} $($commands:tt)*) => {
        $crate::expand! { {$($t)+} get {0} $($commands)* }
    };
    ({$($t:tt)+} reset {} $($commands:tt)*) => {
        $crate::expand! { $($commands)* }
    };
    ({$({$($t:tt)*})*} for_each {$($for_each_commands:tt)*} $($commands:tt)*) => {
        $crate::expand! { while ($({$($t)*})*) {
            $($for_each_commands)*
            $($commands)*
        } }
    };
    // $each must be wrapped with `{}`
    // $commands must be wrapped with `{}`
    (while ($($each:tt)*) $commands:tt) => {
        $($crate::expand! { $each do $commands })*
    };
}

#[macro_export]
macro_rules! impl_many {
    (
        impl<__> $impl_trait:ident for each_of![$($ty:ty),* $(,)?]
        $impl_block:tt
    ) => {$(
        impl $impl_trait for $ty
        $impl_block
    )*};
    (
        impl<__> $impl_trait:ident <$t:ty> for each_of![$($ty:ty),* $(,)?]
        $impl_block:tt
    ) => {$(
        impl $impl_trait <$t> for $ty
        $impl_block
    )*};
    (
        impl $(<__>)? (
            $(Generics![$($generics:tt)*],)?
            Trait![$($impl_trait:tt)+],
            each_of![$($ty:ty),* $(,)?]
            $(, $(Where![$($where_clause:tt)*] $(,)?)? )?
        )
        $impl_block:tt
    ) => {
        $crate::expand! {
            while ($({$ty})*) {
                prepend( impl $(<$($generics)*>)? $($impl_trait)+ for  )
                append(
                    $($(where $($where_clause)*)?)?
                    $impl_block
                )
            }
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn expand_duplex_concat() {
        let a = crate::expand! {{"a",} duplex_concat({append("b",)}{}) wrap ()};
        assert_eq!(a, ("a", "b", "a"));
    }
}
