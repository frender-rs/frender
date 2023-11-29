use std::{mem::MaybeUninit, pin::Pin, task::Poll};

use frender_html::RenderHtml;

pub trait RenderState<R> {
    fn unmount(self: Pin<&mut Self>, renderer: &mut R);

    fn state_unmount(self: Pin<&mut Self>);

    fn poll_render(
        self: Pin<&mut Self>,
        renderer: &mut R,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()>;
}

pub trait FromUnpinned {
    type Unpinned;

    fn from_unpinned(this: &mut Self::Unpinned) -> Pin<&mut Self>;
}

// impl<T: Unpin> FromUnpinned for T {
//     type Unpinned = T;

//     fn from_unpinned(this: &mut Self::Unpinned) -> Pin<&mut Self> {
//         Pin::new(this)
//     }
// }

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

macro_rules! doc_element {
    (
        $cfg_macro_name:ident $bang:tt
        $content:tt
    ) => {
        $cfg_macro_name $bang (
            doc_element!
            $content
        )
    };
    (
        $($({ $row_span:literal })? #[doc = $name:literal])?
        (($($example:expr),+) as $($ty:tt)+)
    ) => {
        concat!(
            "<tr>",
            $(
                "<th",
                $(
                    " rowspan=\"",
                    stringify!($row_span),
                    "\"",
                )?
                ">",
                $name,
                "</th>",
            )?
            "<td style=\"max-width:100%;\">\n\n",
            doc_assert_element!(stringify_ty!($($ty)+) => [$(stringify!($example)),+]),
            "\n\n</td><td>\n\n",
            $(
                "`",
                stringify!($example),
                "`\n\n",
            )+
            "\n\n</td></tr>",
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
            13 = [
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
            ],
            /// Option
            ((None::<&str>, Some(0)) as Option<impl Element>),
            doc_cfg_either!(
                #[doc = "Either (under\n\n`\"either\"` feature)"]
                ((Either::<_, &str>::Left(0), Either::<i32, _>::Right("1"))
                    as Either<impl Element, impl Element>)
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
                ((vec![Keyed(1, "abc"), Keyed(2, "def"), Keyed(2, "ghi"),])
                    as Vec<Keyed<impl Hash + Eq, impl Element>>),
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
pub trait Element: frender_ssr::SsrElement {
    type RenderState<R: RenderHtml>: RenderState<R> + Default;

    #[cfg(feature = "render_into")]
    fn render_into<'s, Renderer: RenderHtml>(
        self,
        renderer: &mut Renderer,
        render_state: PinMutMaybeUninit<'s, Self::RenderState<Renderer>>,
    ) -> Pin<&'s mut Self::RenderState<Renderer>>;

    fn render_update<Renderer: RenderHtml>(
        self,
        renderer: &mut Renderer,
        render_state: Pin<&mut Self::RenderState<Renderer>>,
    ) where
        Self: Sized,
    {
        self.render_update_maybe_reposition(renderer, render_state, false)
    }

    /// The element needs to be repositioned (re-add to the ctx)
    fn render_update_force_reposition<Renderer: RenderHtml>(
        self,
        renderer: &mut Renderer,
        render_state: Pin<&mut Self::RenderState<Renderer>>,
    ) where
        Self: Sized,
    {
        self.render_update_maybe_reposition(renderer, render_state, true)
    }

    fn render_update_maybe_reposition<Renderer: RenderHtml>(
        self,
        renderer: &mut Renderer,
        render_state: Pin<&mut Self::RenderState<Renderer>>,
        force_reposition: bool,
    );

    type UnpinnedRenderState<R: RenderHtml>: RenderState<R> + Default + Unpin;

    fn unpinned_render_update<Renderer: RenderHtml>(
        self,
        renderer: &mut Renderer,
        render_state: &mut Self::UnpinnedRenderState<Renderer>,
    ) where
        Self: Sized,
    {
        self.unpinned_render_update_maybe_reposition(renderer, render_state, false)
    }

    /// The element needs to be repositioned (re-add to the ctx)
    fn unpinned_render_update_force_reposition<Renderer: RenderHtml>(
        self,
        renderer: &mut Renderer,
        render_state: &mut Self::UnpinnedRenderState<Renderer>,
    ) where
        Self: Sized,
    {
        self.unpinned_render_update_maybe_reposition(renderer, render_state, true)
    }

    fn unpinned_render_update_maybe_reposition<Renderer: RenderHtml>(
        self,
        renderer: &mut Renderer,
        render_state: &mut Self::UnpinnedRenderState<Renderer>,
        force_reposition: bool,
    );
}

#[macro_export]
macro_rules! impl_unpinned_render_for_unpin {
    () => {
        type UnpinnedRenderState<R: $crate::__private::RenderHtml> = Self::RenderState<R>;

        fn unpinned_render_update<Renderer: $crate::__private::RenderHtml>(
            self,
            renderer: &mut Renderer,
            render_state: &mut Self::UnpinnedRenderState<Renderer>,
        ) where
            Self: Sized,
        {
            self.render_update(renderer, ::core::pin::Pin::new(render_state))
        }

        /// The element needs to be repositioned (re-add to the ctx)
        fn unpinned_render_update_force_reposition<Renderer: $crate::__private::RenderHtml>(
            self,
            renderer: &mut Renderer,
            render_state: &mut Self::UnpinnedRenderState<Renderer>,
        ) where
            Self: Sized,
        {
            self.render_update_force_reposition(renderer, ::core::pin::Pin::new(render_state))
        }

        fn unpinned_render_update_maybe_reposition<Renderer: $crate::__private::RenderHtml>(
            self,
            renderer: &mut Renderer,
            render_state: &mut Self::UnpinnedRenderState<Renderer>,
            force_reposition: bool,
        ) {
            self.render_update_maybe_reposition(
                renderer,
                ::core::pin::Pin::new(render_state),
                force_reposition,
            )
        }
    };
}

#[cfg(any(test, doctest))]
mod tests {
    /// ```compile_fail
    /// # use frender_element::Element;
    /// # fn __(v: (impl Element,impl Element,impl Element,impl Element,impl Element,impl Element,impl Element,impl Element,impl Element,impl Element,impl Element,impl Element,impl Element,)) -> impl Element {
    /// #   v
    /// # }
    /// # _ = __((1,2,3,4,5,6,7,8,9,10,11,12,13));
    /// ```
    enum _TupleMaxElements {}

    #[test]
    fn tuple_max_elements() {
        use crate::Element;
        fn __(
            v: (
                impl Element,
                impl Element,
                impl Element,
                impl Element,
                impl Element,
                impl Element,
                impl Element,
                impl Element,
                impl Element,
                impl Element,
                impl Element,
                impl Element,
            ),
        ) -> impl Element {
            v
        }
        _ = __((1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12));
    }
}
