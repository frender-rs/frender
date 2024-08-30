#[doc(hidden)]
#[macro_export]
macro_rules! __const_expr_parse_one_parse_attributes {
    (

    ) => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! const_expr_parse_one {
    (
        // attributes
        {$($finish:tt)*} [$($prepend:tt)*]
        ($($pre_expr:tt)*)
        {#         [$($_attr:tt)*] $($_rest:tt)*}
        {$pound:tt $attr:tt        $( $rest:tt)*}
        [$($append:tt)*]
    ) => {
        $crate::const_expr_parse_one! {
            {$($finish)*} [$($prepend)*]
            ($($pre_expr)* $pound $attr)
            { $($rest)* }
            { $($rest)* }
            [$($append)*]
        }
    };
    (
        // literal as _
        //
        // Output is:
        // literal!($lit:literal as _)
        {$($finish:tt)*} [$($prepend:tt)*]
        ($($pre_expr:tt)*)
        {$_s:literal as        _             $($_rest:tt)*}
        { $s:tt      $as:ident $underline:tt $( $rest:tt)*}
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { literal!($($pre_expr)* $s $as $underline) }
            { $($rest)* }
            $($append)*
        }
    };
    (
        // literal as $ty
        //
        // Output is:
        // literal!($lit:literal as $ty:ty)
        {$($finish:tt)*} [$($prepend:tt)*]
        ($($pre_expr:tt)*)
        {$_s:literal as        $_as_ty:ty $(, $($_rest:tt)*)?}
        {$s:tt       $as:ident  $as_ty:ty $(, $( $rest:tt)*)?}
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { literal!($($pre_expr)* $s $as $as_ty) }
            { $(, $($rest)*)? }
            $($append)*
        }
    };
    (
        // literal
        //
        // Output is:
        // literal!($lit:literal)
        {$($finish:tt)*} [$($prepend:tt)*]
        ($($pre_expr:tt)*)
        {$_s:literal $($_rest:tt)*}
        { $s:tt      $( $rest:tt)*}
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { literal!($($pre_expr)* $s) }
            { $($rest)* }
            $($append)*
        }
    };
    (
        // const {..} as _
        //
        // Output is:
        // const_block!(const $block:block as _)
        {$($finish:tt)*} [$($prepend:tt)*]
        ($($pre_expr:tt)*)
        {const        {$($_b:tt)*} as        _             $($_rest:tt)*}
        {$const:ident $block:tt    $as:ident $underline:tt $( $rest:tt)*}
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { const_block!($($pre_expr)* $const $block $as $underline) }
            { $($rest)* }
            $($append)*
        }
    };
    (
        // const {..} as $ty
        //
        // Output is:
        // const_block!(const $block:block as $ty:ty)
        {$($finish:tt)*} [$($prepend:tt)*]
        ($($pre_expr:tt)*)
        {const        {$($_b:tt)*} as        $_as_ty:ty $(, $($_rest:tt)*)?}
        {$const:ident $block:tt    $as:ident  $as_ty:ty $(, $( $rest:tt)*)?}
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { const_block!($($pre_expr)* $const $block $as $as_ty) }
            { $(, $($rest)*)? }
            $($append)*
        }
    };
    (
        // const {..}
        //
        // Output is:
        // const_block!(const $block:block)
        {$($finish:tt)*} [$($prepend:tt)*]
        ($($pre_expr:tt)*)
        {const        {$($_b:tt)*} $($_rest:tt)*}
        {$const:ident $block:tt    $( $rest:tt)*}
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { const_block!($($pre_expr)* $const $block) }
            { $($rest)* }
            $($append)*
        }
    };
    (
        // {{..}} as $ty      native block as
        //
        // Output is:
        // native_block!({{ $($native_block_content:tt)* }} as $ty:ty)
        {$($finish:tt)*} [$($prepend:tt)*]
        ($($pre_expr:tt)*)
        {{{$($_expr:tt)*}} as      $_as_ty:ty $(, $($_rest:tt)*)?}
        {$native_block:tt  $as:tt  $as_ty:ty $(, $( $rest:tt)*)?}
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { native_block!($($pre_expr)* $native_block $as $as_ty) }
            { $($($rest)*)? }
            $($append)*
        }
    };
    (
        // {{..}}       native block
        //
        // Output is:
        // native_block!({{ $($native_block_content:tt)* }})
        {$($finish:tt)*} [$($prepend:tt)*]
        ($($pre_expr:tt)*)
        {{{$($_expr:tt)*}} $($_rest:tt)*}
        {$native_block:tt $($rest:tt )*}
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { native_block!($($pre_expr)* $native_block) }
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
        ($($pre_expr:tt)*)
        {{$($_block:tt)*} $($_rest:tt)*}
        {$block:tt        $($rest:tt )*}
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { block!($($pre_expr)* $block) }
            { $($rest)* }
            $($append)*
        }
    };
    (
        // [..] as _     array as _
        //
        // Output is:
        // array!([$($array_content:tt)*] as _)
        {$($finish:tt)*} [$($prepend:tt)*]
        ($($pre_expr:tt)*)
        { [$($_array:tt)*] as        _             $($_rest:tt)*}
        { $array:tt        $as:ident $underline:tt $($rest:tt )*}
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { array!($($pre_expr)* $array $as $underline) }
            { $($rest)* }
            $($append)*
        }
    };
    (
        // [..] as [_; _]     array as [_; _]
        //
        // Output is:
        // array!([$($array_content:tt)*] as _)
        {$($finish:tt)*} [$($prepend:tt)*]
        ($($pre_expr:tt)*)
        { [$($_array:tt)*] as        [$($_as_ty:tt)*] $($_rest:tt)*}
        { $array:tt        $as:ident $as_ty:tt        $($rest:tt )*}
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { array!($($pre_expr)* $array $as $as_ty) }
            { $($rest)* }
            $($append)*
        }
    };
    (
        // [..] as $ty     array as
        //
        // Output is:
        // array!([$($array_content:tt)*])
        {$($finish:tt)*} [$($prepend:tt)*]
        ($($pre_expr:tt)*)
        { [$($_array:tt)*] as        $_as_ty:ty $(, $($_rest:tt)*)?}
        { $array:tt        $as:ident  $as_ty:ty $(, $( $rest:tt)*)?}
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { array!($($pre_expr)* $array $as $as_ty) }
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
        ($($pre_expr:tt)*)
        { [$($_array:tt)*] $($_rest:tt)*}
        { $array:tt        $($rest:tt )*}
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { array!($($pre_expr)* $array) }
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
        ($($pre_expr:tt)*)
        { ($($_paren:tt)*) $($_rest:tt)*}
        { $paren:tt        $($rest:tt )*}
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { paren!($($pre_expr)* $paren) }
            { $($rest)* }
            $($append)*
        }
    };
    (
        // some_macro! $macro_body:tt     simple macro
        //
        // Output is:
        // r#macro!($macro_name:ident ! $macro_body:tt)
        {$($finish:tt)*} [$($prepend:tt)*]
        ($($pre_expr:tt)*)
        { $_macro_name:ident !        $_macro_body:tt $($_rest:tt)*}
        {  $macro_name:ident $bang:tt  $macro_body:tt $( $rest:tt)*}
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { r#macro!( $($pre_expr)* $macro_name $bang $macro_body ) }
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
        ($($pre_expr:tt)*)
        { if     ($_predicate:expr) {$($_if_block:tt)*} $($_after_if_block:tt)* }
        { $if:tt $predicate:tt      $if_block:tt        $( $after_if_block:tt)* }
        [$($append:tt)*]
    ) => {
        $crate::__const_expr_syntax_parse_after_if_block! {
            {{$($finish)*} [$($prepend)*]}
            ($($pre_expr)* $if $predicate $if_block)
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
        ($($pre_expr:tt)*)
        { match         ($($_matched:tt)*) { $($_match_body:tt)* } $($_rest:tt)*}
        { $match:ident  $matched:tt        $match_body:tt          $( $rest:tt)*}
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { r#match!( $($pre_expr)* $match $matched $match_body ) }
            { $($rest)* }
            $($append)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! const_expr_expand_parsed {
    (
        with {$($with:tt)*}
        $(attrs {$($attrs:tt)*})?
        parsed {$kind:ident $bang:tt ($($parsed:tt)*)}
    ) => {
        $($with)* :: $kind $bang {
            @{$($with)*}
            $($($attrs)*)?
            $($parsed)*
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
            { $(,$($_after_if_block)*)? }
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
        $with:tt
        attrs $attrs:tt
        $parsed:tt
        {} // rest should be empty
    ) => {
        $crate::const_expr::expand_parsed! {
            with $with
            attrs $attrs
            parsed $parsed
        }
    };
    (
        $with:tt
        $parsed:tt
        {} // rest should be empty
    ) => {
        $crate::const_expr::expand_parsed! {
            with $with
            parsed $parsed
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __const_expr_expect_comma_separated_and_chain_with {
    (
        $with:tt
        attrs {$($attrs:tt)*}
        $parsed:tt
        {$(,)?} // EOF
    ) => {
        $crate::__const_expr_expect_one_and_expand_with! {
            $with
            attrs {$($attrs)*}
            $parsed
            {}
        }
    };
    (
        {$($with:tt)*}
        attrs {$($attrs:tt)*}
        $parsed:tt
        {, $($rest:tt)*}
    ) => {
        $($with)*::chain! {
            with {$($with)*}
            attrs {$($attrs)*}
            parsed $parsed
            rest {$($rest)*}
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
        attrs {$($attrs:tt)*}
        {match        ($($matched:tt)*) {}            }
        {$match:ident $paren_matched:tt $match_body:tt}
    ) => {
        $($with)*::never! {
            @{$($with)*}
            $($attrs)*
            $match $crate::const_expr_assert_expr!$paren_matched
            $match_body
        }
    };
    (
        {$($with:tt)*}
        attrs $attrs:tt
        {match        ($($matched:tt)*) $_match_body:tt}
        {$match:ident $paren_matched:tt  $match_body:tt}
    ) => {
        $crate::__const_expr_resolve_match_body! {
            {$match $paren_matched}
            {$($with)*}
            attrs $attrs
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
        attrs $attrs:tt
        { $this_pat:pat $(if $guard:expr)? => $($after_pat:tt)* }
    ) => {
        $crate::const_expr_parse_one! {
            {$crate::__const_expr_expect_match_branch_body!}
            [{
                match $match_and_matched
                parsed_patterns {}
                either_paths {}
                with $with
                attrs $attrs
                {$this_pat $(if $guard)?}
            }]
                ()
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
            with {$($with:tt)*}
            attrs $attrs:tt
            {$($this_pat:tt)*}
        }
        $parsed:tt
        {$(,)?} // rest
    ) => {
        $($with)*::match_non_empty! {
            with {$($with)*}
            attrs $attrs
            match {$match}
            paren_matched {$paren_matched}
            parsed_patterns {
                $($parsed_patterns)*
                {
                    pat { $($this_pat)* }
                    either_paths $either_paths
                    parsed $parsed
                }
            }
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
            attrs $attrs:tt
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
                    {
                        pat { $($this_pat)* }
                        either_paths {$($either_paths)* A}
                        parsed $parsed
                    }
                }
                either_paths {$($either_paths)* B}
                with {$($with)*}
                attrs $attrs
                {$pat $(if $guard)?}
            }]
                ()
                {$($after_pat)*}
                {$($after_pat)*}
            []
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! const_expr_resolve_either_paths {
    (@{$($with:tt)*} {} {$($e:tt)*}) => {
        $($e)*
    };
    (@{$($with:tt)*} {$variant:tt $($vars:tt)*} $e:tt) => {
        $($with)*::Either::$variant(
            $crate::const_expr::resolve_either_paths!(
                @{$($with)*}
                {$($vars)*} $e
            )
        )
    };
}

#[doc(hidden)]
pub mod __private {
    pub use std::convert::identity;
}

#[doc(inline)]
pub use {
    const_expr_assert_expr as assert_expr, const_expr_expand_parsed as expand_parsed,
    const_expr_parse_one as parse_one, const_expr_resolve_either_paths as resolve_either_paths,
};

pub mod syntax {
    pub use {None, Some};

    #[doc(hidden)]
    #[macro_export]
    macro_rules! const_expr_syntax_empty {
        ($(#$attr:tt)*) => {
            $(#$attr)*
            $crate::Empty
        };
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! const_expr_syntax_literal {
        (@{$($with:tt)*} $(#$attr:tt)+ $lit:literal $(as $($as_ty:tt)*)?) => {
            $($with)*::r#const! { $(#$attr)+ const { $lit } $(as $($as_ty)*)? }
        };
        (@{$($with:tt)*} $lit:tt $(as $($as_ty:tt)*)?) => {
            $($with)*::r#const! { const { $lit } $(as $($as_ty)*)? }
        };
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! const_expr_syntax_const_block {
        (@{$($with:tt)*} $($t:tt)*) => {
            $($with)*::r#const! { $($t)* }
        };
    }

    /// Note that the default macro for native block syntax
    /// doesn't allow `as $ty`.
    #[doc(hidden)]
    #[macro_export]
    macro_rules! const_expr_syntax_native_block {
        // TODO: is this needed?
        (@{$($with:tt)*} $(#$attr:tt)* $e:block /* matching a block would prevent unused brace */) => {
            $(#$attr)*
            $e
        };
        (@{$($with:tt)*} $(#$attr:tt)* {$($t:tt)*} /* allows braced tokens that are not a valid block */) => {
            $(#$attr)*
            {$($t)*}
        };
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! const_expr_syntax_block {
        // empty
        (@{$($with:tt)*} $(#$attr:tt)* {}) => {
            $($with)*::empty! { $(#$attr)* }
        };
        // one
        (@{$($with:tt)*} $(#$attr:tt)* {$($t:tt)*}) => {
            $crate::const_expr_parse_one! {
                {$crate::__const_expr_expect_one_and_expand_with!} [{$($with)*}]
                    ($(#$attr)*)
                    {$($t)*}
                    {$($t)*}
                []
            }
        };
    }

    /// The default macro for `[..]` syntax.
    ///
    /// `[..]` will be parsed as
    #[doc(hidden)]
    #[macro_export]
    macro_rules! const_expr_syntax_array {
        (@{$($with:tt)*} $(#$attr:tt)+ [$($array:tt)*] $(as $($as_ty:tt)*)?) => {
            $($with)*::r#const! {
                $(#$attr)+
                const { [$($array)*] } $(as $($as_ty)*)?
            }
        };
        (@{$($with:tt)*} $array:tt $(as $($as_ty:tt)*)?) => {
            $($with)*::r#const! {
                const { $array } $(as $($as_ty)*)?
            }
        };
    }

    /// The default macro for `(..)` syntax.
    ///
    /// `()` will be parsed as `$with::empty! {}`
    ///
    /// `($a $(,)?)` will be parsed as `$with::one!($a)`
    ///
    /// `($a, $b, $c)` are recursively [chained](chain!).
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
    ///
    /// Attributes are propagated to each element.
    #[doc(hidden)]
    #[macro_export]
    macro_rules! const_expr_syntax_paren {
        // empty
        (@{$($with:tt)*} $(#$attr:tt)* ()) => {
            $($with)*::empty! { $(#$attr)* }
        };
        (@{$($with:tt)*} $(#$attr:tt)* ($($t:tt)*)) => {
            $crate::const_expr_parse_one! {
                {$crate::__const_expr_expect_comma_separated_and_chain_with!}
                [
                    {$($with)*}
                    attrs{$(#$attr)*}
                ]
                    ()
                    {$($t)*}
                    {$($t)*}
                []
            }
        };
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! const_expr_syntax_chain {
        (
            with {$($with:tt)*}
            attrs {$($attrs:tt)*}
            parsed $parsed:tt
            rest {$($rest:tt)*}
        ) => {
            $($with)*::Chain(
                $crate::__const_expr_expect_one_and_expand_with! {
                    {$($with)*}
                    attrs {$($attrs)*}
                    $parsed
                    {}
                },
                $($with)*::paren! {
                    @{$($with)*}
                    $($attrs)*
                    ($($rest)*)
                }
            )
        };
    }

    /// The default macro for `some_macro!(..)` syntax doesn't allow attributes.
    #[doc(hidden)]
    #[macro_export]
    macro_rules! const_expr_syntax_macro {
        (@{$($with:tt)*} $macro_name:ident $bang:tt $macro_content:tt) => {
            $($with)*::macros::$macro_name $bang $macro_content
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
            $(#$attr:tt)*
            $if:ident $paren_predicate:tt $if_block:tt
        ) => {
            $if $crate::const_expr_assert_expr!$paren_predicate {
                $($with)*::Some(
                    $($with)*::block! {
                        @{$($with)*}
                        $(#$attr)*
                        $if_block
                    }
                )
            } else {
                $($with)*::None
            }
        };
        (
            //  `if (predicate) {..} else ..`
            //
            //  is parsed as
            //
            //  ```
            //  if predicate {
            //      $with::Either::A($with::block!({..}))
            //  } else {
            //      $with::Either::B($with::one!(..))
            //  }
            //  ```
            @{$($with:tt)*}
            $(#$attr:tt)*
            $if:ident $paren_predicate:tt $if_block:tt
            $else:ident $($after_else:tt)*
        ) => {
            $if $crate::const_expr_assert_expr!$paren_predicate {
                $($with)*::Either::A(
                    $($with)*::block!(
                        @{$($with)*}
                        $(#$attr)*
                        $if_block
                    )
                )
            } $else {
                $($with)*::Either::B(
                    $($with)*::one!(
                        @{$($with)*}
                        $(#$attr)*
                        $($after_else)*
                    )
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
            $(#$attr:tt)*
            $match:ident
            $($match_clause:tt)*
        ) => {
            $crate::__const_expr_resolve_match! {
                $with
                attrs {$(#$attr)*}
                {$match $($match_clause)*}
                {$match $($match_clause)*}
            }
        };
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! const_expr_syntax_never {
        (@{$($with:tt)*} $e:expr) => {
            // (|| -> $($with)*::Never { $e })()
            // the above cannot be used in const

            {
                #[allow(unreachable_code)]
                $crate::const_expr::__private::identity::<$($with)*::Never>($e)
            }
        };
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! const_expr_syntax_match_non_empty {
        (
            with $with:tt
            attrs $attrs:tt
            match {$match:ident}
            paren_matched {$paren_matched:tt}
            parsed_patterns {
                $(
                    {
                        pat { $($pat:tt)* }
                        either_paths $either_paths:tt
                        parsed $parsed:tt
                    }
                )*
            }
        ) => {
            $match $crate::const_expr_assert_expr!$paren_matched {
                $(
                    $($pat)* => $crate::const_expr::resolve_either_paths!(
                        @$with
                        $either_paths
                        {
                            $crate::const_expr::expand_parsed!(
                                with $with
                                attrs $attrs
                                parsed $parsed
                            )
                        }
                    ),
                )*
            }
        };
    }

    #[doc(inline)]
    pub use {
        const_expr_syntax_array as array, const_expr_syntax_block as block,
        const_expr_syntax_chain as chain, const_expr_syntax_const_block as const_block,
        const_expr_syntax_empty as empty, const_expr_syntax_if as r#if,
        const_expr_syntax_literal as literal, const_expr_syntax_macro as r#macro,
        const_expr_syntax_match as r#match, const_expr_syntax_match_non_empty as match_non_empty,
        const_expr_syntax_native_block as native_block, const_expr_syntax_never as never,
        const_expr_syntax_one as one, const_expr_syntax_paren as paren,
    };
}

const _: () = {
    enum Never {}
    enum Never2 {}

    const fn test(x: Never2) -> Never {
        #[allow(unreachable_code)]
        crate::const_expr::__private::identity::<Never>(match x {});
    }
};
