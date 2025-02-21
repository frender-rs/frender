#![cfg(all(feature = "csr", feature = "ssr"))]

use frender_csr::CsrElement;
use frender_ssr::SsrElement;

#[cfg(feature = "either")]
macro_rules! doc_cfg_either {
    ($e:expr) => {
        $e
    };
}
#[cfg(not(feature = "either"))]
macro_rules! doc_cfg_either {
    ($e:expr) => {
        ""
    };
}

macro_rules! doc_assert_element {
    ($($ty_str:expr => [$($example_str:expr),+]),+) => {
        concat!(
            "```\n# use std::{borrow::Cow, hash::Hash}; use frender_element::Element; use frender_common::{Keyed, Elements};",
            doc_cfg_either!("use either::Either;"),
            "const N: usize = 33;",
            "fn _assert_element(_: impl Element) {}\n",
            $(
                "# { fn __(v: \n",
                $ty_str,
                "\n# ) { _assert_element(v) }\n",
                $(
                    concat!("# let _ = __(", $example_str, ");\n")
                ,)+
                "# }\n",
            )+
            "```"
        )
    };
}

macro_rules! stringify_ty {
    (stringified![$stringified:expr]) => {
        $stringified
    };
    ($ty:ty) => {
        stringify!($ty)
    };
}

macro_rules! doc_inline_multiple {
    (doc_element!(
        #[doc = $name:expr]
        $((($example:expr) as $($ty:tt)+)),+ $(,)?
    )) => {
        doc_row!(
            concat!($("`", stringify_ty!($($ty)+), "` ",)+),
            concat!($("`", stringify!($example)  , "` ",)+),
            concat!(
                $name,
                "\n<div hidden>\n\n",
                doc_assert_element!($(
                    concat!("# ", stringify_ty!($($ty)+)) => [stringify!($example)]
                ),+),
                "\n\n</div>"
            )
        )
    };
}

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
                ">",
                $name,
                "</th>",
            )?)?
            "<td>\n\n",
            $types,
            "\n\n</td><td>\n\n",
            $examples,
            "\n\n</td></tr>",
        )
    };
}

macro_rules! doc_element {
    (
        $doc_macro_name:ident $bang:tt
        $content:tt
    ) => {
        $doc_macro_name $bang (
            doc_element!
            $content
        )
    };
    (
        $($({ $row_span:literal })? #[doc = $name:literal])?
        (($($example:expr),+) as $($ty:tt)+)
    ) => {
        doc_row!(
            doc_assert_element!(stringify_ty!($($ty)+) => [$(stringify!($example)),+]),
            concat!($(
                "`",
                stringify!($example),
                "`\n\n",
            )+),
            $($name, $($row_span)?)?
        )
    };
    (
        $row_span:literal = [$(
            $(#$doc:tt)?
            ($($content:tt)+)
        ),+ $(,)?]
    ) => {
        concat!($(
            doc_element!(
                $( { $row_span } #$doc)?
                ($($content)+)
            ),
        )+)
    };
}

macro_rules! doc_elements {
    ($(
        $t1:tt $t2:tt $t3:tt
    ),+ $(,)?) => {
        concat!($(
            doc_element!($t1 $t2 $t3),
        )+)
    };
}

macro_rules! doc_elements_all {
    () => {
        doc_elements!(
            /// Char
            (('a') as char),
            3 = [
                /// Strings
                (("abc") as &str),
                (("abc".to_string()) as String),
                ((Cow::Borrowed("abc")) as Cow<'_, str>),
            ],
            doc_inline_multiple!(
                /// Numbers
                ((0i8) as i8),
                ((0u8) as u8),
                ((0u16) as u16),
                ((0i32) as i32),
                ((0u32) as u32),
                ((0i64) as i64),
                ((0u64) as u64),
                ((0i128) as i128),
                ((0u128) as u128),
                ((0isize) as isize),
                ((0usize) as usize),
                ((0f32) as f32),
                ((0f64) as f64),
            ),
            /// Option
            ((None::<&str>, Some(0)) as Option<impl Element>),
            doc_cfg_either!(
                #[doc = "Either (under\n\n`\"either\"` feature)"]
                ((Either::<_, &str>::Left(0), Either::<i32, _>::Right("1")) as Either<impl Element, impl Element>)
            ),
            /// Box
            ((Box::new("")) as Box<impl Element>),
            4 = [
                #[doc = "Tuple\n\n(up to 12\n\nelements)"]
                ((()) as ()),
                (((1,)) as (impl Element,)),
                (((1, 2)) as (impl Element, impl Element)),
                (((1, 2, 3)) as (impl Element, impl Element, impl Element)),
            ],
            /// Array
            (([""; N]) as [impl Element; N]),
            2 = [
                /// Keyed elements
                ((vec![Keyed(1, "abc"), Keyed(2, "def"), Keyed(2, "ghi"),]) as Vec<Keyed<impl Hash + Eq, impl Element>>),
                ((Elements((0..10).map(|i| Keyed(i, i))))
                    as stringified![
                        r##"Elements<impl IntoIterator<
    Item = Keyed<
        impl Hash + Eq,
        impl Element,
    >
>>"##
                    ])
            ],
        )
    };
}

/// Element(s) that can be html children.
///
/// ## Notable implementors
///
/// <table>
/// <thead><tr>
///     <th></th>
///     <th>Types</th>
///     <th>Examples</th>
/// </tr></thead>
/// <tbody>
#[doc = doc_elements_all!()]
/// </tbody>
/// </table>
pub trait Element: SsrElement + CsrElement {}

impl<E: ?Sized + SsrElement + CsrElement> Element for E {}
