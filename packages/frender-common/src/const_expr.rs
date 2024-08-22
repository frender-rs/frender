#[doc(hidden)]
#[macro_export]
macro_rules! const_expr_parse_one {
    (
        // literal
        //
        // Output is:
        // literal!($lit:literal)
        {$($finish:tt)*} [$($prepend:tt)*]
        {$s:literal $($rest:tt)*}
        $input:tt
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { literal!($s) }
            { $($rest)* }
            $($append)*
        }
    };
    (
        // const {..}
        //
        // Output is:
        // const_block!(const $block:block)
        {$($finish:tt)*} [$($prepend:tt)*]
        {const        {$($_b:tt)*} $($_rest:tt)*}
        {$const:ident $block:tt    $( $rest:tt)*}
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { const_block!($const $block) }
            { $($rest)* }
            $($append)*
        }
    };
    (
        // {{..}}       native block
        //
        // Output is:
        // native_block!({{ $($native_block_content:tt)* }})
        {$($finish:tt)*} [$($prepend:tt)*]
        {{{$($_expr:tt)*}} $($_rest:tt)*}
        {$native_block:tt $($rest:tt )*}
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { native_block!($native_block) }
            { $($rest)* }
            $($append)*
        }
    };
    (
        // {..}       block
        //
        // Output is:
        // block!({ $($block_content:tt)* })
        {$($finish:tt)*} [$($prepend:tt)*]
        {{$($_block:tt)*} $($_rest:tt)*}
        {$block:tt        $($rest:tt )*}
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { block!($block) }
            { $($rest)* }
            $($append)*
        }
    };
    (
        // [..]     array
        //
        // Output is:
        // array!([$($array_content:tt)*])
        {$($finish:tt)*} [$($prepend:tt)*]
        { [$($_array:tt)*] $($_rest:tt)*}
        { $array:tt        $($rest:tt )*}
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { array!($array) }
            { $($rest)* }
            $($append)*
        }
    };
    (
        // (..)     paren
        //
        // Output is:
        // paren!((..))
        {$($finish:tt)*} [$($prepend:tt)*]
        { ($($_paren:tt)*) $($_rest:tt)*}
        { $paren:tt        $($rest:tt )*}
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { paren!($paren) }
            { $($rest)* }
            $($append)*
        }
    };
    (
        // `if (..) {..}` with optional `else if`s and `else`
        //
        // Output:
        // r#if!($($if_clause:tt)*)
        {$($finish:tt)*} [$($prepend:tt)*]
        { if     ($_predicate:expr) {$($_if_block:tt)*} $($_after_if_block:tt)* }
        { $if:tt $predicate:tt      $if_block:tt        $( $after_if_block:tt)* }
        [$($append:tt)*]
    ) => {
        $crate::__const_expr_syntax_parse_after_if_block! {
            {{$($finish)*} [$($prepend)*]}
            ($if $predicate $if_block)
            {$($after_if_block)*}
            {$($after_if_block)*}
            [$($append)*]
        }
    };
    (
        // match (..) {}        match paren
        //
        // Output is:
        // r#match!(match ($($matched:tt)*) { $($match_body:tt)* })
        {$($finish:tt)*} [$($prepend:tt)*]
        { match         ($($_matched:tt)*) { $($_match_body:tt)* } $($_rest:tt)*}
        { $match:ident  $matched:tt        $match_body:tt          $( $rest:tt)*}
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { r#match!( $match $matched $match_body ) }
            { $($rest)* }
            $($append)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __const_expr_syntax_parse_after_if_block {
    // EOF or comma
    (
        {{$($finish:tt)*} [$($prepend:tt)*]}
        $paren_style_if:tt
        { $(,$($_after_if_block:tt)*)? }
        $after_if_block:tt
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { r#if! $paren_style_if }
            { $($rest)* }
            $($append)*
        }
    };
    // else if
    (
        $finish_and_prepend:tt
        ($($style_if:tt)*)
        { else        if        ($_predicate:expr) {$($_if_block:tt)*} $($_after_if_block:tt)* }
        { $else:ident $if:ident $predicate:tt      $if_block:tt        $( $after_if_block:tt)* }
        $append:tt
    ) => {
        $crate::__const_expr_syntax_parse_after_if_block! {
            $finish_and_prepend
            (
                $($style_if)*
                $else $if $predicate
                $if_block
            )
            {$($after_if_block)*}
            {$($after_if_block)*}
            $append
        }
    };
    // else
    (
        {{$($finish:tt)*} [$($prepend:tt)*]}
        ($($paren_style_if:tt)*)
        { else        {$($_else_block:tt)*} $($_rest:tt)* }
        { $else:ident $else_block:tt        $( $rest:tt)* }
        $after_if_block:tt
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { r#if!( $($paren_style_if)* $else $else_block ) }
            { $($rest)* }
            $($append)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __const_expr_expect_one_and_expand_with {
    (
        {$($with:tt)*}
        {$kind:ident $bang:tt ($($parsed:tt)*)}
        {} // rest should be empty
    ) => {
        $($with)* :: $kind $bang (@{$($with)*} $($parsed)*)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __const_expr_expect_comma_separated_and_chain_with {
    (
        $with:tt
        $parsed:tt
        {$(,)?} // EOF
    ) => {
        $crate::__const_expr_expect_one_and_expand_with! {
            $with
            $parsed
            {}
        }
    };
    (
        {$($with:tt)*}
        $parsed:tt
        {, $($rest:tt)*}
    ) => {
        $($with)*::chain! {
            $crate::__const_expr_expect_one_and_expand_with! {
                {$($with)*}
                $parsed
                {}
            },
            // use default paren macro instead of $with::paren
            $crate::const_expr_syntax_paren! {
                @{$($with)*}
                ($($rest)*)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! const_expr_assert_expr {
    ($e:expr) => {
        $e
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __const_expr_resolve_match {
    (
        {$($with:tt)*}
        {match        ($($matched:tt)*) {}            }
        {$match:ident $paren_matched:tt $match_body:tt}
    ) => {
        $($with)*::never! {
            $match $crate::const_expr_assert_expr!$paren_matched
            $match_body
        }
    };
    (
        {$($with:tt)*}
        {match        ($($matched:tt)*) $_match_body:tt}
        {$match:ident $paren_matched:tt  $match_body:tt}
    ) => {
        $crate::__const_expr_resolve_match_body! {
            {$match $paren_matched}
            {$($with)*}
            $match_body
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __const_expr_resolve_match_body {
    (
        $match_and_matched:tt
        $with:tt
        { $this_pat:pat $(if $guard:expr)? => $($after_pat:tt)* }
    ) => {
        $crate::const_expr_parse_one! {
            {$crate::__const_expr_expect_match_branch_body!}
            [{
                match $match_and_matched
                parsed_patterns {}
                either_paths {}
                with $with
                {$this_pat $(if $guard)?}
            }]
                {$($after_pat)*}
                {$($after_pat)*}
            []
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __const_expr_expect_match_branch_body {
    (
        {
            match {$match:ident $paren_matched:tt}
            parsed_patterns {$($parsed_patterns:tt)*}
            either_paths $either_paths:tt
            with $with:tt
            {$($this_pat:tt)*}
        }
        $parsed:tt
        {$(,)?} // rest
    ) => {
        $match $crate::const_expr_assert_expr!$paren_matched {
            $($parsed_patterns)*
            $($this_pat)* => $crate::__const_expr_call_path_recursively!(
                $either_paths
                $crate::__const_expr_expect_one_and_expand_with!($with $parsed {})
            ),
        }
    };
    (
        $prepend:tt
        $parsed:tt
        $rest:tt
    ) => {
        $crate::__const_expr_expect_match_branch_comma! {
            $prepend
            $parsed
            $parsed
            $rest
        }
    };
}

// This assumes rest is neither `{}` nor `{,}`
#[doc(hidden)]
#[macro_export]
macro_rules! __const_expr_expect_match_branch_comma {
    // if rest starts with comma, consume it
    (
        $match:tt
        $_parsed:tt
        $parsed:tt
        {, $($rest:tt)*}
    ) => {
        $crate::__const_expr_expect_match_branch_continue! {
            $match
            $parsed
            {$($rest)*}
        }
    };
    // If rest doesn't start with comma, only allow the cases that doesn't require comma
    // block or native block or const block
    (
        $match:tt
        {$parsed_kind:ident $bang:tt ($(const)? {$($braced:tt)*})}
        $parsed:tt
        $rest:tt
    ) => {
        $crate::__const_expr_expect_match_branch_continue! {
            $match
            $parsed
            $rest
        }
    };
    // if
    (
        $match:tt
        {r#if $bang:tt ($(const)? {$($braced:tt)*})}
        $parsed:tt
        $rest:tt
    ) => {
        $crate::__const_expr_expect_match_branch_continue! {
            $match
            $parsed
            $rest
        }
    };
    // match
    (
        $match:tt
        {r#match $bang:tt ($(const)? {$($braced:tt)*})}
        $parsed:tt
        $rest:tt
    ) => {
        $crate::__const_expr_expect_match_branch_continue! {
            $match
            $parsed
            $rest
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __const_expr_expect_match_branch_continue {
    (
        {
            match $match:tt
            parsed_patterns {$($parsed_patterns:tt)*}
            either_paths {$($either_paths:tt)*}
            with {$($with:tt)*}
            {$($this_pat:tt)*}
        }
        $parsed:tt
        {
            $pat:pat $(if $guard:expr)? => $($after_pat:tt)*
        } // rest
    ) => {
        $crate::const_expr_parse_one! {
            {$crate::__const_expr_expect_match_branch_body!}
            [{
                match $match
                parsed_patterns {
                    $($parsed_patterns)*
                    $($this_pat)* => $crate::__const_expr_call_path_recursively!(
                        {$($either_paths)*}
                        $($with)*::EitherA!(
                            $crate::__const_expr_expect_one_and_expand_with!({$($with)*} $parsed {})
                        )
                    ),
                }
                either_paths {$($either_paths)* {$($with)*::EitherB!}}
                with {$($with)*}
                {$pat $(if $guard)?}
            }]
                {$($after_pat)*}
                {$($after_pat)*}
            []
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __const_expr_call_path_recursively {
    ({} $e:expr) => {
        $e
    };
    ({{$($p:tt)*} $({$($ps:tt)*})*} $e:expr) => {
        $($p)* ($crate::__const_expr_call_path_recursively!({$({$($ps)*})*} $e))
    };
}

#[doc(hidden)]
pub mod __private {
    pub use {None, Some};
}

#[doc(inline)]
pub use {const_expr_assert_expr as assert_expr, const_expr_parse_one as parse_one};

pub mod syntax {
    #[doc(hidden)]
    #[macro_export]
    macro_rules! const_expr_syntax_empty {
        () => {
            $crate::Empty
        };
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! const_expr_syntax_literal {
        (@{$($with:tt)*} $lit:literal) => {
            $($with)*::r#const! {$lit}
        };
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! const_expr_syntax_const_block {
        (@{$($with:tt)*} const $block:block) => {
            $($with)*::r#const! {$block}
        };
        (@{$($with:tt)*} const $block:tt) => {
            $($with)*::r#const! { $crate::const_expr_assert_expr!($block) }
        };
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! const_expr_syntax_native_block {
        (@{$($with:tt)*} $e:block /* matching a block would prevent unused brace */) => {
            $e
        };
        (@{$($with:tt)*} $t:tt /* allows braced tokens that are not a valid block */) => {
            $t
        };
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! const_expr_syntax_block {
        (@{$($with:tt)*} {}) => {
            $($with)*::empty! {}
        };
        (@{$($with:tt)*} $t:tt /* $t should be braced tokens */) => {
            $crate::const_expr_parse_one! {
                {$crate::__const_expr_expect_one_and_expand_with!} [{$($with)*}]
                    $t
                    $t
                []
            }
        };
    }

    // array doesn't have a default macro

    /// The default macro for `(..)` syntax.
    ///
    /// `()` will be parsed as `$with::empty! {}`
    ///
    /// `($a $(,)?)` will be parsed as `$with::one!($a)`
    ///
    /// `($a, $b, $c)` are recursively chained as
    ///
    /// ```rust,no_compile
    /// $with::chain!(
    ///     $with::one!($a),
    ///     $with::chain!(
    ///         $with::one!($b),
    ///         $with::one!($c)
    ///     )
    /// )
    /// ```
    #[doc(hidden)]
    #[macro_export]
    macro_rules! const_expr_syntax_paren {
        (@{$($with:tt)*} ()) => {
            $($with)*::empty! {}
        };
        (@{$($with:tt)*} ($($t:tt)*)) => {
            $crate::const_expr_parse_one! {
                {$crate::__const_expr_expect_comma_separated_and_chain_with!}
                [{$($with)*}]
                    {$($t)*}
                    {$($t)*}
                []
            }
        };
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! const_expr_syntax_if {
        (
            //  `if (predicate) {..}`
            //
            //  is parsed as
            //
            //  ```
            //  if predicate {
            //      $with::Some!($with::block!({..})))
            //  } else {
            //      $with::None!()
            //  }
            //  ```
            @{$($with:tt)*}
            $if:ident $paren_predicate:tt $if_block:tt
        ) => {
            $if $crate::const_expr_assert_expr!$paren_predicate {
                $crate::const_expr_syntax::__private::Some(
                    $($with)*::block!(@{$($with)*} $if_block)
                )
            } else {
                $crate::const_expr_syntax::__private::None
            }
        };
        (
            //  `if (predicate) {..} else ..`
            //
            //  is parsed as
            //
            //  ```
            //  if predicate {
            //      $with::EitherA!($with::block!({..}))
            //  } else {
            //      $with::EitherB!($with::one!(..))
            //  }
            //  ```
            @{$($with:tt)*}
            $if:ident $paren_predicate:tt $if_block:tt
            $else:ident $($after_else:tt)*
        ) => {
            $if $crate::const_expr_assert_expr!$paren_predicate {
                $($with)*::EitherA!(
                    $($with)*::block!(@{$($with)*} $if_block)
                )
            } $else {
                $($with)*::EitherB!(
                    $($with)*::one!(@{$($with)*} $($after_else)*)
                )
            }
        };
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! const_expr_syntax_one {
        (@{$($with:tt)*} $($t:tt)*) => {
            $crate::const_expr_syntax_block! {
                @{$($with)*}
                {$($t)*}
            }
        };
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! const_expr_syntax_match {
        (
            @$with:tt
            $($match_clause:tt)*
        ) => {
            $crate::__const_expr_resolve_match! {
                $with
                {$($match_clause)*}
                {$($match_clause)*}
            }
        };
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! const_expr_syntax_Some {
        ($e:expr) => {
            $crate::const_expr::__private::Some($e)
        };
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! const_expr_syntax_None {
        () => {
            $crate::const_expr::__private::None
        };
    }

    #[doc(inline)]
    pub use {
        const_expr_syntax_None as None, const_expr_syntax_Some as Some,
        const_expr_syntax_block as block, const_expr_syntax_const_block as const_block,
        const_expr_syntax_empty as empty, const_expr_syntax_if as r#if,
        const_expr_syntax_literal as literal, const_expr_syntax_match as r#match,
        const_expr_syntax_native_block as native_block, const_expr_syntax_one as one,
        const_expr_syntax_paren as paren,
    };
}
