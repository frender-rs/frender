macro_rules! doc_row {
    ($types:expr, $examples:expr $(, $($name:expr $(, $($row_span:expr $(,)?)?)?)?)?) => {
        concat!(
            "<tr>",
            $($(
                "<th",
                $($(
                    " rowspan=\"",
                    $row_span,
                    "\"",
                )?)?
                ">\n\n",
                $name,
                "\n\n</th>",
            )?)?
            "<td>\n\n",
            $types,
            "\n\n</td><td>\n\n",
            $examples,
            "\n\n</td></tr>",
        )
    };
}

macro_rules! stringified_multiline {
    ({$prefix:expr} $($stringified_line:expr),+ $(,)?) => {
        concat!(
            $($prefix, $stringified_line, "\n",)+
            "# "
        )
    };
}

macro_rules! stringify_single_token_ty {
    ($Ty:tt) => {
        stringify_single_token_ty!($Ty "")
    };
    (($Ty:ty) $prefix:expr) => {
        concat!($prefix, stringify!($Ty))
    };
    ([$macro:ident $bang:tt [$($content:tt)*]] $prefix:expr) => {
        $macro $bang ({$prefix} $($content)*)
    };
    ($Ty:tt $prefix:expr) => {
        stringify_single_token_ty!(($Ty) $prefix)
    };
}

macro_rules! stringify_example {
    ($e:tt) => {
        stringify_example!($e "")
    };
    (($e:expr) $prefix:expr) => {
        concat!($prefix, stringify!($e))
    };
    ({$macro:ident $bang:tt ($($content:tt)*)} $prefix:expr) => {
        $macro $bang ({$prefix} $($content)*)
    };
    ($e:tt $prefix:expr) => {
        stringify_example!(($e) $prefix)
    };
}

macro_rules! doc_element_impl {
    (
        $(name($name:expr))?
        $(row_span($row_span:expr))?
        single_token_ty($Ty:tt)
        example($example:tt)
    ) => {
        doc_row!(
            doc_test_element!(
                ty(stringify_single_token_ty!($Ty))
                examples(stringify_example!($example "# "))
            ),
            doc_test_element!(
                ty(stringify_single_token_ty!($Ty "# "))
                examples(stringify_example!($example))
            ),
            $($name,)?
            $($row_span,)?
        )
    };
}

macro_rules! doc_element {
    (
        #[doc = $name:expr]
        $example:tt as $Ty:ty
    ) => {
        doc_element_impl! {
            name($name)
            single_token_ty(($Ty))
            example($example)
        }
    };
    (
        #[doc = $name:expr]
        (
            $example_0:tt as $Ty_0:tt,
            $($($example:tt as $Ty:tt),+ $(,)?)?
        )
    ) => {
        concat!(
            doc_element_impl! {
                name($name)
                row_span(count_tt!($example_0 $($($example)+)?))
                single_token_ty($Ty_0)
                example($example_0)
            },
            $($(
                doc_element_impl!(
                    single_token_ty($Ty)
                    example($example)
                ),
            )+)?
        )
    };
    (
        #[doc = $name:expr]
        [$($example:tt as $Ty:ty),+ $(,)?]
    ) => {
        doc_row!(
            concat!(
                $(
                    "<code>",
                    stringify!($Ty),
                    "</code> ",
                )+
                "\n\n",
                "<span hidden>\n\n",
                doc_test_element!(
                    $(
                        ty(concat!("# ", stringify!($Ty)))
                        examples(stringify_example!($example "# "))
                    )+
                ),
                "\n\n</span>",
            ),
            concat!(
                $(
                    "<code>",
                    stringify_example!($example),
                    "</code> ",
                )+
            ),
            $name,
        )
    };
    (
        #[doc = $name:expr]
        {each![
            $example_0:expr,
            $($($example:expr),+ $(,)?)?
        ] as $Ty:ty}
    ) => {
        doc_element_impl! {
            name($name)
            single_token_ty([tuple_repeat_and_only_show_first![
                (stringify!($Ty))
                {{$example_0} $($({$example})+)?}
            ]])
            example({stringify_tuple_expr_hide_paren_comma!(
                $example_0,
                $($($example),+)?
            )})
        }
    };
}

macro_rules! tuple_repeat_and_only_show_first {
    ({$prefix:expr} ($s:expr) {$_repeater:tt $($repeater:tt)*}) => {
        concat!(
            "# (\n",
            $prefix,
            $s,
            $(
                "\n# ,",
                ignore_first_tt!($repeater $s),
            )*
            ")",
        )
    };
}

macro_rules! stringify_tuple_expr_hide_paren_comma {
    ({$prefix:expr} $($e:expr),+ $(,)?) => {
        concat!(
            "# (\n",
            $(
                $prefix,
                stringify!($e),
                "\n# ,\n",
            )+
            "# )",
        )
    };
}

macro_rules! ignore_first_tt {
    ($first:tt $($rest:tt)*) => {
        $($rest)*
    };
}

#[cfg(not(all(feature = "either", feature = "KeyedElements")))]
macro_rules! if_features_then_empty_else_ignore {
    () => {
        "ignore"
    };
}

#[cfg(all(feature = "either", feature = "KeyedElements"))]
macro_rules! if_features_then_empty_else_ignore {
    () => {
        ""
    };
}

macro_rules! doc_test_element {
    (
        $(
            ty($ty_str:expr)
            examples($($example_str:expr),+ $(,)?)
        )+
    ) => {
        concat!(
            "```",
            if_features_then_empty_else_ignore!(),
            "\n# use std::{borrow::Cow, hash::Hash, rc::Rc, sync::Arc};",
            "use frender_element::Element;",
            "use frender::{Empty, Keyed, KeyedElements};",
            "use either::Either;",
            "const N: usize = 32;",
            "fn _assert_element(_: impl Element) {}\n",
            $(
                "# { fn __(v: \n",
                $ty_str,
                "\n# ) { _assert_element(v) }\n",
                $(
                    concat!(
                        "# let _ = __(\n",
                        $example_str,
                        "\n# );\n"
                    ),
                )+
                "# }\n",
            )+
            "```"
        )
    };
}

macro_rules! doc_elements {
    ($(
        // #   [doc=] $definition
        $t1:tt $t2:tt $t3:tt $(as $Ty:ty)?
    ),+ $(,)?) => {
        concat!(
            $(
                doc_element!($t1 $t2 $t3 $(as $Ty)?),
            )+
        )
    };
}

macro_rules! count_tt {
    () => {
        0
    };
    ($t0:tt) => {
        1
    };
    ($t0:tt $t1:tt) => {
        2
    };
    ($t0:tt $t1:tt $t2:tt) => {
        3
    };
    ($t0:tt $t1:tt $t2:tt $t3:tt) => {
        4
    };
    ($t0:tt $t1:tt $t2:tt $t3:tt $t4:tt) => {
        5
    };
}

pub(super) use {
    count_tt, doc_element, doc_element_impl, doc_elements, doc_row, doc_test_element,
    if_features_then_empty_else_ignore, ignore_first_tt, stringified_multiline, stringify_example,
    stringify_single_token_ty, stringify_tuple_expr_hide_paren_comma,
    tuple_repeat_and_only_show_first,
};
