//! Element(s) that can be html children.
//!
//! ## Notable implementors
//!
//! <div>
//! <table>
//! <thead><tr>
//!     <th></th>
//!     <th>Types</th>
//!     <th>Examples</th>
//! </tr></thead>
//! <tbody>
#![cfg_attr(any(test, doc, doctest), doc = doc_elements!(
/// [`Empty`](crate::Empty)
Empty as Empty,
/// Char
'a' as char,
/// Numbers
[
    0i8 as i8,
    0u8 as u8,
    0u16 as u16,
    0i32 as i32,
    0u32 as u32,
    0i64 as i64,
    0u64 as u64,
    0i128 as i128,
    0u128 as u128,
    0isize as isize,
    0usize as usize,
    0f32 as f32,
    0f64 as f64,
],
/// Static strings
(
    "abc" as (&'static str),
    ("abc".to_string()) as String,
    (Cow::Borrowed("abc")) as (Cow<'static, str>),
    (Rc::from("abc")) as (Rc<str>),
    (Arc::from("abc")) as (Arc<str>),
),
/// Option
{each![
    None::<&str>,
    Some(0),
] as Option<impl Element>},
/// [`EitherElement`](crate::EitherElement)
{each![
    Either::<_, &str>::Left(0),
    Either::<i32, _>::Right("1"),
] as Either<impl Element, impl Element> },
/// [`Either`](either::Either) (under "either" feature)
{each![
    Either::<_, &str>::Left(0),
    Either::<i32, _>::Right("1"),
] as Either<impl Element, impl Element> },
/// Tuple (up to 12 elements)
(
    (1,) as (impl Element,),
    (1, 2) as (impl Element, impl Element),
    (1, 2, 3) as (impl Element, impl Element, impl Element),
),
/// Array
[""; N] as [impl Element; N],
/// [`KeyedElements`](struct@crate::KeyedElements)
(
    {stringified_multiline!(
        "KeyedElements(",
        "    vec![",
        "        Keyed(1, 'a'),",
        "        Keyed(2, 'b'),",
        "    ]",
        ")",
    )} as [stringified_multiline![
        "KeyedElements<Vec<",
        "    Keyed<",
        "        impl Hash + Eq,",
        "        impl Element,",
        "    >",
        ">>",
    ]],
    {stringified_multiline!(
        "KeyedElements(",
        "    (0..10)",
        "        .map(|i| Keyed(i, i))",
        ")",
    )} as [stringified_multiline![
        "KeyedElements<impl IntoIterator<",
        "    Item = Keyed<",
        "        impl Hash + Eq,",
        "        impl Element,",
        "    >",
        ">>",
    ]],
),
))]
//! </tbody>
//! </table>
//! </div>
// Run the above doc tests with:
//      cargo test --doc --package frender -- elements --show-output

#[cfg(feature = "SyncedCollection")]
pub mod synced_collection;

#[cfg(feature = "HookElement")]
pub mod hook_element;

#[cfg(any(test, doc, doctest))]
mod doc_macros;
#[cfg(any(test, doc, doctest))]
use self::doc_macros::*;
