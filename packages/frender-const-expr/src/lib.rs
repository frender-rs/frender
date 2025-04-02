#![no_std]

#[macro_export]
macro_rules! parse_one {
    (
        // attributes
        {$($finish:tt)*} [$($prepend:tt)*]
        {$($pre_expr:tt)*}
        {#         [$($_attr:tt)*] $($_rest:tt)*}
        {$pound:tt $attr:tt        $( $rest:tt)*}
        [$($append:tt)*]
    ) => {
        $crate::parse_one! {
            {$($finish)*} [$($prepend)*]
            {$($pre_expr)* $pound $attr}
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
        $pre_expr:tt
        {$_s:literal as        _             $($_rest:tt)*}
        { $s:tt      $as:ident $underline:tt $( $rest:tt)*}
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { literal!($pre_expr {$s $as $underline}) }
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
        $pre_expr:tt
        {$_s:literal as        $_as_ty:ty $(, $($_rest:tt)*)?}
        {$s:tt       $as:ident  $as_ty:ty $(, $( $rest:tt)*)?}
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { literal!($pre_expr {$s $as $as_ty}) }
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
        $pre_expr:tt
        {$_s:literal $($_rest:tt)*}
        { $s:tt      $( $rest:tt)*}
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { literal!($pre_expr {$s}) }
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
        $pre_expr:tt
        {const        {$($_b:tt)*} as        _             $($_rest:tt)*}
        {$const:ident $block:tt    $as:ident $underline:tt $( $rest:tt)*}
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { const_block!($pre_expr {$const $block $as $underline}) }
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
        $pre_expr:tt
        {const        {$($_b:tt)*} as        $_as_ty:ty $(, $($_rest:tt)*)?}
        {$const:ident $block:tt    $as:ident  $as_ty:ty $(, $( $rest:tt)*)?}
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { const_block!($pre_expr {$const $block $as $as_ty}) }
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
        $pre_expr:tt
        {const        {$($_b:tt)*} $($_rest:tt)*}
        {$const:ident $block:tt    $( $rest:tt)*}
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { const_block!($pre_expr {$const $block}) }
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
        $pre_expr:tt
        {{$($_block:tt)*} $($_rest:tt)*}
        {$block:tt        $($rest:tt )*}
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { block!($pre_expr {$block}) }
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
        $pre_expr:tt
        { [$($_array:tt)*] as        _             $($_rest:tt)*}
        { $array:tt        $as:ident $underline:tt $($rest:tt )*}
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { array!($pre_expr {$array $as $underline}) }
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
        $pre_expr:tt
        { [$($_array:tt)*] as        [$($_as_ty:tt)*] $($_rest:tt)*}
        { $array:tt        $as:ident $as_ty:tt        $($rest:tt )*}
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { array!($pre_expr {$array $as $as_ty}) }
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
        $pre_expr:tt
        { [$($_array:tt)*] as        $_as_ty:ty $(, $($_rest:tt)*)?}
        { $array:tt        $as:ident  $as_ty:ty $(, $( $rest:tt)*)?}
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { array!($pre_expr {$array $as $as_ty}) }
            { $(, $($rest)*)? }
            $($append)*
        }
    };
    (
        // [..]     array
        //
        // Output is:
        // array!([$($array_content:tt)*])
        {$($finish:tt)*} [$($prepend:tt)*]
        $pre_expr:tt
        { [$($_array:tt)*] $($_rest:tt)*}
        { $array:tt        $($rest:tt )*}
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { array!($pre_expr {$array}) }
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
        $pre_expr:tt
        { ($($_paren:tt)*) $($_rest:tt)*}
        { $paren:tt        $($rest:tt )*}
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { paren!($pre_expr {$paren}) }
            { $($rest)* }
            $($append)*
        }
    };
    (
        // some_macro! $macro_body:tt as _     simple macro as _
        //
        // Output is:
        // r#macro!($macro_name:ident ! $macro_body:tt as _)
        {$($finish:tt)*} [$($prepend:tt)*]
        $pre_expr:tt
        { $_macro_name:ident !        $_macro_body:tt as     _             $($_rest:tt)*}
        {  $macro_name:ident $bang:tt  $macro_body:tt $as:tt $underline:tt $( $rest:tt)*}
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { r#macro!( $pre_expr {$macro_name $bang $macro_body $as $underline} ) }
            { $($rest)* }
            $($append)*
        }
    };
    (
        // some_macro! $macro_body:tt as $ty:ty     simple macro as type
        //
        // Output is:
        // r#macro!($macro_name:ident ! $macro_body:tt as $ty:ty)
        {$($finish:tt)*} [$($prepend:tt)*]
        $pre_expr:tt
        { $_macro_name:ident !        $_macro_body:tt as     $_as_ty:ty $(, $($_rest:tt)*)?}
        {  $macro_name:ident $bang:tt  $macro_body:tt $as:tt  $as_ty:ty $(, $( $rest:tt)*)?}
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { r#macro!( $pre_expr {$macro_name $bang $macro_body $as $as_ty} ) }
            { $(, $($rest)*)? }
            $($append)*
        }
    };
    (
        // some_macro! $macro_body:tt     simple macro
        //
        // Output is:
        // r#macro!($macro_name:ident ! $macro_body:tt)
        {$($finish:tt)*} [$($prepend:tt)*]
        $pre_expr:tt
        { $_macro_name:ident !        $_macro_body:tt $($_rest:tt)*}
        {  $macro_name:ident $bang:tt  $macro_body:tt $( $rest:tt)*}
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { r#macro!( $pre_expr {$macro_name $bang $macro_body} ) }
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
        $pre_expr:tt
        { if     ($_predicate:expr) {$($_if_block:tt)*} $($_after_if_block:tt)* }
        { $if:tt $predicate:tt      $if_block:tt        $( $after_if_block:tt)* }
        [$($append:tt)*]
    ) => {
        $crate::__syntax_parse_after_if_block! {
            {{$($finish)*} [$($prepend)*]}
            ($pre_expr {$if $predicate $if_block})
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
        $pre_expr:tt
        { match         ($($_matched:tt)*) { $($_match_body:tt)* } $($_rest:tt)*}
        { $match:ident  $matched:tt        $match_body:tt          $( $rest:tt)*}
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { r#match!( $pre_expr {$match $matched $match_body} ) }
            { $($rest)* }
            $($append)*
        }
    };
}

#[macro_export]
macro_rules! expand_parsed {
    (
        with {$($with:tt)*}
        $(attrs {$($attrs:tt)*})?
        parsed {
            $kind:ident $bang:tt (
                {$($other_attr:tt)*}
                $braced_parsed:tt
            )
        }
    ) => {
        $($with)* :: $kind $bang {
            @{$($with)*}
            #{
                $($($attrs)*)?
                $($other_attr)*
            }
            $braced_parsed
            $braced_parsed
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __syntax_parse_after_if_block {
    // EOF or comma
    (
        {{$($finish:tt)*} [$($prepend:tt)*]}
        $paren_attrs_and_braced:tt
        { $(,$($_after_if_block:tt)*)? }
        $after_if_block:tt
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { r#if! $paren_attrs_and_braced }
            { $(,$($_after_if_block)*)? }
            $($append)*
        }
    };
    // else if
    (
        $finish_and_prepend:tt
        ($attrs:tt {$($parsed:tt)*})
        { else        if        ($_predicate:expr) {$($_if_block:tt)*} $($_after_if_block:tt)* }
        { $else:ident $if:ident $predicate:tt      $if_block:tt        $( $after_if_block:tt)* }
        $append:tt
    ) => {
        $crate::__syntax_parse_after_if_block! {
            $finish_and_prepend
            (
                $attrs
                {
                    $($parsed)*
                    $else $if $predicate
                    $if_block
                }
            )
            {$($after_if_block)*}
            {$($after_if_block)*}
            $append
        }
    };
    // else
    (
        {{$($finish:tt)*} [$($prepend:tt)*]}
        ($attrs:tt {$($parsed:tt)*})
        { else        {$($_else_block:tt)*} $($_rest:tt)* }
        { $else:ident $else_block:tt        $( $rest:tt)* }
        [$($append:tt)*]
    ) => {
        $($finish)* {
            $($prepend)*
            { r#if!( $attrs {$($parsed)* $else $else_block} ) }
            { $($rest)* }
            $($append)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __expect_one_and_expand_with {
    (
        $with:tt
        attrs $attrs:tt
        $parsed:tt
        {} // rest should be empty
    ) => {
        $crate::expand_parsed! {
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
        $crate::expand_parsed! {
            with $with
            parsed $parsed
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __expect_comma_separated_and_chain_with {
    (
        $with:tt
        attrs {$($attrs:tt)*}
        $parsed:tt
        {$(,)?} // EOF
    ) => {
        $crate::__expect_one_and_expand_with! {
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

#[macro_export]
macro_rules! assert_expr {
    ($e:expr) => {
        $e
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __resolve_match {
    (
        {$($with:tt)*}
        attrs {$($attrs:tt)*}
        {match        ($($matched:tt)*) {}            }
        {$match:ident $paren_matched:tt $match_body:tt}
    ) => {
        $($with)*::never! {
            @{$($with)*}
            $($attrs)*
            $match $crate::assert_expr!$paren_matched
            $match_body
        }
    };
    (
        {$($with:tt)*}
        attrs $attrs:tt
        {match        ($($matched:tt)*) $_match_body:tt}
        {$match:ident $paren_matched:tt  $match_body:tt}
    ) => {
        $crate::__resolve_match_body! {
            {$match $paren_matched}
            {$($with)*}
            attrs $attrs
            $match_body
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __resolve_match_body {
    (
        $match_and_matched:tt
        $with:tt
        attrs $attrs:tt
        { $this_pat:pat $(if $guard:expr)? => $($after_pat:tt)* }
    ) => {
        $crate::parse_one! {
            {$crate::__expect_match_branch_body!}
            [{
                match $match_and_matched
                parsed_patterns {}
                either_paths {}
                with $with
                attrs $attrs
                {$this_pat $(if $guard)?}
            }]
                {}
                {$($after_pat)*}
                {$($after_pat)*}
            []
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __expect_match_branch_body {
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
        $crate::__expect_match_branch_comma! {
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
macro_rules! __expect_match_branch_comma {
    // if rest starts with comma, consume it
    (
        $match:tt
        $_parsed:tt
        $parsed:tt
        {, $($rest:tt)*}
    ) => {
        $crate::__expect_match_branch_continue! {
            $match
            $parsed
            {$($rest)*}
        }
    };
    // If rest doesn't start with comma, only allow the cases that doesn't require comma
    // block or const block
    (
        $match:tt
        {$parsed_kind:ident $bang:tt ($attrs:tt {$(const)? {$($braced:tt)*}})}
        $parsed:tt
        $rest:tt
    ) => {
        $crate::__expect_match_branch_continue! {
            $match
            $parsed
            $rest
        }
    };
    // if
    (
        $match:tt
        {r#if $bang:tt $paren_attrs_and_braced:tt}
        $parsed:tt
        $rest:tt
    ) => {
        $crate::__expect_match_branch_continue! {
            $match
            $parsed
            $rest
        }
    };
    // match
    (
        $match:tt
        {r#match $bang:tt $paren_attrs_and_braced:tt}
        $parsed:tt
        $rest:tt
    ) => {
        $crate::__expect_match_branch_continue! {
            $match
            $parsed
            $rest
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __expect_match_branch_continue {
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
        $crate::parse_one! {
            {$crate::__expect_match_branch_body!}
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
                {}
                {$($after_pat)*}
                {$($after_pat)*}
            []
        }
    };
}

#[macro_export]
macro_rules! resolve_either_paths {
    (@{$($with:tt)*} {} {$($e:tt)*}) => {
        $($e)*
    };
    (@{$($with:tt)*} {$variant:tt $($vars:tt)*} $e:tt) => {
        $($with)*::Either::$variant(
            $crate::resolve_either_paths!(
                @{$($with)*}
                {$($vars)*} $e
            )
        )
    };
}

#[doc(hidden)]
pub mod __private {
    pub use core::convert::identity;
}

pub mod syntax {
    pub mod parsed {
        pub use {None, Some};
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! syntax_empty {
        (@{$($with:tt)*} #{$($attr:tt)*} {()} {()}) => {
            $($attr)*
            $($with)*::parsed::Empty
        };
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! syntax_literal {
        (
            @{$($with:tt)*} #$attrs:tt
            {$_lit:literal $(as $($as_ty:tt)*)?}
            {$ lit:literal $($rest:tt)*}
        ) => {
            $($with)*::const_block! {
                @{$($with)*}
                #$attrs
                {const { $lit } $($rest)*}
                {const { $lit } $($rest)*}
            }
        };
    }

    /// The default macro for `[..]` syntax.
    ///
    /// `[..]` will be parsed as [`const_block!(const { [..] })`](const_block).
    #[doc(hidden)]
    #[macro_export]
    macro_rules! syntax_array {
        (
            @{$($with:tt)*} #$attrs:tt
            {[$($_array:tt)*] $(as $($as_ty:tt)*)?}
            {$array:tt        $($rest:tt)*        }
        ) => {
            $($with)*::const_block! {
                @{$($with)*}
                #$attrs
                {const { $array } $($rest)*}
                {const { $array } $($rest)*}
            }
        };
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! syntax_const_block {
        (
            @{$($with:tt)*} #{$($attr:tt)*}
            {const {$($const_block:tt)*} $(as $($as_ty:tt)*)?}
            {$($t:tt)*}
        ) => {
            $($with)*::parsed::r#const! {
                $($attr)*
                $($t)*
            }
        };
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! syntax_block {
        (
            @{$($with:tt)*} #$attrs:tt
            {{$($t:tt)+}}
            {$braced:tt}
        ) => {
            $($with)*::one! {
                @{$($with)*}
                #$attrs
                $braced
                $braced
            }
        };
    }

    /// The default macro for `(..)` syntax.
    ///
    /// - `()` will be parsed as `$with::empty! { @{$with} () }`
    ///
    /// - `($a $(,)?)` will be parsed as `$with::one!($a)`
    ///
    /// - `($a, $b, $c)` are recursively [chained](chain!).
    ///
    ///   ```rust,no_compile
    ///   $with::Chain(
    ///       $with::one!($a),
    ///       $with::Chain(
    ///           $with::one!($b),
    ///           $with::one!($c)
    ///       )
    ///   )
    /// ```
    ///
    /// Attributes are propagated to each element.
    #[doc(hidden)]
    #[macro_export]
    macro_rules! syntax_paren {
        // empty
        (
            @{$($with:tt)*} #$attrs:tt
            {()             }
            {$paren_empty:tt}
        ) => {
            $($with)*::empty! { @{$($with)*} #$attrs {$paren_empty} {$paren_empty} }
        };
        // non empty
        (
            @{$($with:tt)*} #$attrs:tt
            {($($t:tt)+)         }
            {$_repeat:tt         }
        ) => {
            $crate::parse_one! {
                {$crate::__expect_comma_separated_and_chain_with!}
                [
                    {$($with)*}
                    attrs $attrs
                ]
                    {}
                    {$($t)+}
                    {$($t)+}
                []
            }
        };
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! syntax_chain {
        (
            with {$($with:tt)*}
            attrs{$($attr:tt)*}
            parsed $parsed:tt
            rest {$($rest:tt)*}
        ) => {
            $($attr)*
            $($with)*::Chain(
                $crate::expand_parsed! {
                    with {$($with)*}
                    parsed $parsed
                },
                $($with)*::paren! {
                    @{$($with)*}
                    #{}
                    {($($rest)*)}
                    {($($rest)*)}
                }
            )
        };
    }

    /// The default macro for `some_macro!(..)` syntax doesn't allow `as Type`.
    #[doc(hidden)]
    #[macro_export]
    macro_rules! syntax_macro {
        (
            @{$($with:tt)*} #{$($attr:tt)*}
            {$_macro_name:ident !        ($($_macro_content:tt)*)}
            {$ macro_name:ident $bang:tt $macro_content:tt       }
        ) => {
            $($attr)*
            $($with)*::macros::$macro_name $bang $macro_content
        };
        (
            @{$($with:tt)*} #{$($attr:tt)*}
            {$_macro_name:ident !        [$($_macro_content:tt)*]}
            {$ macro_name:ident $bang:tt $macro_content:tt       }
        ) => {
            $($attr)*
            $($with)*::macros::$macro_name $bang $macro_content
        };
        (
            @{$($with:tt)*} #{$($attr:tt)*}
            {$_macro_name:ident !        {$($_macro_content:tt)*}}
            {$ macro_name:ident $bang:tt $macro_content:tt       }
        ) => {
            $($attr)*
            $($with)*::macros::$macro_name $bang $macro_content
        };
    }

    /// The default syntax for `if` expand attributes before `if` expression
    #[doc(hidden)]
    #[macro_export]
    macro_rules! syntax_if {
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
            #{$($attr:tt)*}
            {if        ($($predicate:tt)*) {$($if_block_content:tt)*}}
            {$if:ident $paren_predicate:tt $if_block:tt              }
        ) => {
            $($attr)*
            $if $crate::assert_expr!$paren_predicate {
                $($with)*::parsed::Some(
                    $($with)*::block! {
                        @{$($with)*}
                        #{}
                        {$if_block}
                        {$if_block}
                    }
                )
            } else {
                $($with)*::parsed::None
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
            #{$($attr:tt)*}
            {
                if        ($($predicate:tt)*) {$($if_block_content:tt)*}
                else        $($_after_else:tt)*
            }{
                $if:ident $paren_predicate:tt $if_block:tt
                $else:ident $($ after_else:tt)*
            }
        ) => {
            $($attr)*
            $if $crate::assert_expr!$paren_predicate {
                $($with)*::parsed::Either::A(
                    $($with)*::block!(
                        @{$($with)*}
                        #{}
                        {$if_block}
                        {$if_block}
                    )
                )
            } $else {
                $($with)*::parsed::Either::B(
                    $($with)*::one!(
                        @{$($with)*}
                        #{}
                        {$($after_else)*}
                        {$($after_else)*}
                    )
                )
            }
        };
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! syntax_one {
        (@$with:tt #$attrs:tt {$($t:tt)+} $braced:tt) => {
            $crate::parse_one! {
                {$crate::__expect_one_and_expand_with!} [$with]
                    $attrs
                    $braced
                    $braced
                []
            }
        };
        (@$with:tt $braced:tt) => {
            $crate::syntax_one! {
                @$with #{}
                $braced $braced
            }
        };
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! syntax_match {
        (
            @$with:tt
            #$attrs:tt
            { match ($($matched:tt)*) {$($match_body:tt)*} }
            $braced:tt
        ) => {
            $crate::__resolve_match! {
                $with
                attrs $attrs
                $braced
                $braced
            }
        };
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! syntax_never {
        (@{$($with:tt)*} #{$($attr:tt)*} {$e:expr} $_repeat:tt) => {
            // (|| -> $($with)*::Never { $e })()
            // the above cannot be used in const

            {
                $($attr)*
                #[allow(unreachable_code)]
                $crate::__private::identity::<$($with)*::parsed::Never>($e)
            }
        };
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! syntax_match_non_empty {
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
            $match $crate::assert_expr!$paren_matched {
                $(
                    $($pat)* => $crate::resolve_either_paths!(
                        @$with
                        $either_paths
                        {
                            $crate::expand_parsed!(
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
        syntax_array as array, syntax_block as block, syntax_chain as chain,
        syntax_const_block as const_block, syntax_empty as empty, syntax_if as r#if,
        syntax_literal as literal, syntax_macro as r#macro, syntax_match as r#match,
        syntax_match_non_empty as match_non_empty, syntax_never as never, syntax_one as one,
        syntax_paren as paren,
    };

    pub mod macros {
        #[macro_export]
        #[doc(hidden)]
        macro_rules! syntax_macros_verbatim {
            ($($e:tt)*) => { $($e)* };
        }

        pub use syntax_macros_verbatim as verbatim;
    }
}

pub mod const_marker {
    #[doc(hidden)]
    #[macro_export]
    macro_rules! const_marker_const_marker {
        ($ty:ty) => {
            $ty
        };
    }

    #[doc(inline)]
    pub use const_marker_const_marker as const_marker;
}

#[doc(hidden)]
#[macro_export]
macro_rules! __array_item_unit {
    ($e:expr) => {
        ()
    };
}

#[macro_export]
macro_rules! array_len {
    ([]) => {
        0
    };
    ([$($e:expr),+ $(,)?]) => {
        [$(
            $crate::__array_item_unit!($e)
        ),+].len()
    };
}

/// An inline expr of `ConstValue`.
///
/// Supported syntaxes:
///
/// - `$e:expr`
/// - `const $const_block:block`
/// - `const $const_block:block as _`
/// - `const $const_block:block as $Ty:ty`
/// - `$e:literal`
/// - `$e:literal as _`
/// - `$e:literal as $Ty:ty`
#[macro_export]
macro_rules! r#const {
    (
        #[const_impl_mod $const_impl_mod:tt]
        const $($rest:tt)*
    ) => {{
        enum __FrenderConstValueMarker {}
        $crate::r#const! {
            #[const_impl_mod $const_impl_mod]
            #[const_marker(__FrenderConstValueMarker)]
            const $($rest)*
        }
    }};
    (
        #[const_impl_mod $const_impl_mod:tt]
        #[$const_marker:ident $const_marker_body:tt]
        const $s:tt
    ) => {
        $crate::r#const! {
            #[const_impl_mod $const_impl_mod]
            #[$const_marker $const_marker_body]
            const $s as _
        }
    };
    (
        #[const_impl_mod($($const_impl_mod:tt)+)]
        #[$const_marker:ident $const_marker_body:tt]
        const $s:tt as $($const_ty:tt)*
    ) => {{
        const _: () = {
            $($const_impl_mod)+::impl_marker_for! {
                impl $crate::const_marker::$const_marker!$const_marker_body {
                    const _: $($const_ty)* = $s;
                }
            }
        };

        const __FRENDER_CONST_EXPR: $($const_impl_mod)+::ConstValue::<
            $crate::const_marker::$const_marker!$const_marker_body
        > = $($const_impl_mod)+::ConstValue();

        __FRENDER_CONST_EXPR
    }};
    (
        $(#$attr:tt)+
        $e:literal $($rest:tt)*
    ) => {
        $crate::r#const! {
            $(#$attr)+
            const { $e } $($rest)*
        }
    };
    (
        #[const_impl_mod $const_impl_mod:tt]
        #[$const_marker:ident $const_marker_body:tt]
        $e:expr
    ) => {
        $crate::r#const! {
            #[const_impl_mod $const_impl_mod]
            #[$const_marker $const_marker_body]
            const { $e } as _
        }
    };
    (
        #[const_impl_mod $const_impl_mod:tt]
        $e:expr
    ) => {
        $crate::r#const! {
            #[const_impl_mod $const_impl_mod]
            const { $e } as _
        }
    };
}

#[cfg(test)]
mod tests {
    const _: () = {
        enum Never {}
        enum Never2 {}

        const fn test(x: Never2) -> Never {
            #[allow(unreachable_code)]
            crate::__private::identity::<Never>(match x {});
        }
    };
}
