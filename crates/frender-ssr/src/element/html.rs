use std::{borrow::Cow, pin::Pin, task::Poll};

use frender_core::{RenderState, UpdateRenderState};
use futures_io::AsyncWrite;

use crate::{
    bytes::{AsyncWritableByteChunks, CowSlicedBytes, IterByteChunks},
    WriterOrError,
};

pub enum HtmlAttributeValue<'a> {
    String(Cow<'a, str>),
    /// If a boolean attribute is present, its value is true, and if it's absent, its value is false.
    ///
    /// HTML defines restrictions on the allowed values of boolean attributes. Please see
    /// [boolean_attributes]
    ///
    /// [boolean_attributes]: https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes#boolean_attributes
    BooleanTrue,
}

pub enum HtmlElementChildren<Children> {
    /// See [void element]
    ///
    /// [void element]: https://developer.mozilla.org/en-US/docs/Glossary/Void_element
    Void,
    /// Self-closing tags are required in void elements in XML, XHTML, and SVG (e.g., <circle cx="50" cy="50" r="50" />).
    SelfClosing,
    // `<div></div>`
    Children(Children),
}

impl<Children> HtmlElementChildren<Children> {
    pub fn map_children<R>(self, f: impl FnOnce(Children) -> R) -> HtmlElementChildren<R> {
        match self {
            HtmlElementChildren::Void => HtmlElementChildren::Void,
            HtmlElementChildren::SelfClosing => HtmlElementChildren::SelfClosing,
            HtmlElementChildren::Children(children) => HtmlElementChildren::Children(f(children)),
        }
    }
}

pub struct HtmlElement<'a, Children = (), Attrs = [HtmlAttrPair<'a>; 0]> {
    pub tag: Cow<'a, str>,
    pub attributes: Attrs,
    pub children: HtmlElementChildren<Children>,
}

pub enum HtmlElementChildrenAndClosingRenderState<Children> {
    Children {
        /// `>`
        start_tag_gt_written: bool,
        children: Children,
    },
    /// `</`
    Closing {
        written: usize,
    },
    ClosingTag {
        written: usize,
    },
    /// `>`
    End,
}

pub enum HtmlElementRenderStateStage {
    /// `<`
    Start,
    /// `div`
    StartTag {
        written: usize,
    },
    Attrs,
    Children,
}

pub enum HtmlAttributeValueState<'a> {
    String {
        // `="`
        equals_quote: &'static [u8],
        bytes: CowSlicedBytes<'a>,
        // `"`
        quote: &'static [u8],
    },
    BooleanTrue,
}

pub type HtmlAttrPair<'a> = (Cow<'a, str>, HtmlAttributeValue<'a>);

pub struct AttrsIntoByteChunks<Attrs>(pub Attrs);

impl<'a, Attrs: Iterator<Item = HtmlAttrPair<'a>>> Iterator for AttrsIntoByteChunks<Attrs> {
    type Item = [CowSlicedBytes<'a>; 5];

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(name, value)| {
            let (a, b, c) = match value {
                HtmlAttributeValue::String(value) => (
                    CowSlicedBytes::Borrowed(b"=\""),
                    match html_escape::encode_double_quoted_attribute(&value) {
                        Cow::Borrowed(_) => value.into(),
                        Cow::Owned(value) => CowSlicedBytes::Owned(value.into_bytes().into()),
                    },
                    CowSlicedBytes::Borrowed(b"\""),
                ),
                HtmlAttributeValue::BooleanTrue => (
                    CowSlicedBytes::Borrowed(&[]),
                    CowSlicedBytes::Borrowed(&[]),
                    CowSlicedBytes::Borrowed(&[]),
                ),
            };
            [CowSlicedBytes::Borrowed(b" "), name.into(), a, b, c]
        })
    }
}

pub struct HtmlElementRenderStateData<'a, Children, Attrs: Iterator<Item = HtmlAttrPair<'a>>> {
    pub before_children: IterByteChunks<
        core::iter::Chain<
            core::iter::Chain<
                core::array::IntoIter<CowSlicedBytes<'a>, 2>,
                core::iter::Flatten<AttrsIntoByteChunks<Attrs>>,
            >,
            core::array::IntoIter<CowSlicedBytes<'a>, 3>,
        >,
    >,
    pub children: Option<Children>,
    pub after_children: IterByteChunks<core::array::IntoIter<CowSlicedBytes<'a>, 3>>,
}

impl<'a, Children, Attrs: Iterator<Item = HtmlAttrPair<'a>>>
    HtmlElementRenderStateData<'a, Children, Attrs>
{
    pub fn new(tag: Cow<'a, str>, attrs: Attrs, children: HtmlElementChildren<Children>) -> Self {
        let mut start_tag_end = [
            CowSlicedBytes::Borrowed(&[]),
            CowSlicedBytes::Borrowed(&[]),
            CowSlicedBytes::Borrowed(b">"),
        ];
        if let HtmlElementChildren::SelfClosing = &children {
            start_tag_end[0] = CowSlicedBytes::Borrowed(b" ");
            start_tag_end[1] = CowSlicedBytes::Borrowed(b"/");
        };

        let (children, after_children) = match children {
            HtmlElementChildren::Children(v) => (
                Some(v),
                [
                    CowSlicedBytes::Borrowed(b"</"),
                    tag.clone().into(),
                    CowSlicedBytes::Borrowed(b">"),
                ],
            ),
            _ => (None, [[].as_slice(); 3].map(CowSlicedBytes::Borrowed)),
        };

        let before_children = IterByteChunks::new(
            [CowSlicedBytes::Borrowed(b"<"), tag.into()]
                .into_iter()
                .chain(AttrsIntoByteChunks(attrs).flatten())
                .chain(start_tag_end),
        );

        Self {
            before_children,
            children,
            after_children: IterByteChunks::new(after_children.into_iter()),
        }
    }
}

pub struct HtmlElementRenderState<'a, Children, Attrs: Iterator<Item = HtmlAttrPair<'a>>, W> {
    data: HtmlElementRenderStateData<'a, Children, Attrs>,
    writer_or_error: WriterOrError<W>,
}

impl<
        'a,
        Children: RenderState + Unpin,
        Attrs: Unpin + Iterator<Item = HtmlAttrPair<'a>>,
        W: AsyncWrite + Unpin,
    > RenderState for HtmlElementRenderState<'a, Children, Attrs, W>
{
    fn unmount(self: std::pin::Pin<&mut Self>) {
        if let Some(children) = &mut self.get_mut().data.children {
            Children::unmount(Pin::new(children))
        }
    }

    fn poll_reactive(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        let this = self.get_mut();

        let writer = match &mut this.writer_or_error {
            WriterOrError::Writer(w) => w,
            WriterOrError::Error(_) => return Poll::Ready(()),
        };

        let data = &mut this.data;

        macro_rules! ready_ok {
            ($e:expr) => {
                match $e {
                    Poll::Ready(Ok(())) => {}
                    Poll::Ready(Err(error)) => {
                        this.writer_or_error = WriterOrError::Error(error);
                        return Poll::Ready(());
                    }
                    Poll::Pending => return Poll::Pending,
                }
            };
        }

        ready_ok!(data
            .before_children
            .poll_write_byte_chunks(Pin::new(writer), cx));

        if let Some(children) = &mut data.children {
            match Pin::new(children).poll_reactive(cx) {
                Poll::Ready(()) => {}
                res => return res,
            }
        }

        ready_ok!(data
            .after_children
            .poll_write_byte_chunks(Pin::new(writer), cx));

        Poll::Ready(())
    }
}

impl<'a, Children: UpdateRenderState<crate::SsrContext<W>>, Attrs, W: AsyncWrite + Unpin>
    UpdateRenderState<crate::SsrContext<W>> for HtmlElement<'a, Children, Attrs>
where
    Children::State: Unpin,
    Attrs: Unpin + Iterator<Item = HtmlAttrPair<'a>>,
{
    type State = HtmlElementRenderState<'a, Children::State, Attrs, W>;

    fn initialize_render_state(self, ctx: &mut crate::SsrContext<W>) -> Self::State {
        let writer_or_error = ctx.expect_to_take_writer();

        HtmlElementRenderState {
            data: HtmlElementRenderStateData::new(
                self.tag,
                self.attributes,
                self.children
                    .map_children(|children| children.initialize_render_state(ctx)),
            ),
            writer_or_error,
        }
    }

    fn update_render_state(self, ctx: &mut crate::SsrContext<W>, state: Pin<&mut Self::State>) {
        let state = state.get_mut();
        let writer_or_error = ctx.expect_to_take_writer();

        state.writer_or_error = writer_or_error;

        let old_children_state = state.data.children.take();

        state.data = HtmlElementRenderStateData::new(
            self.tag,
            self.attributes,
            self.children.map_children(|children| {
                if let Some(mut old_children_state) = old_children_state {
                    children.update_render_state(ctx, Pin::new(&mut old_children_state));
                    old_children_state
                } else {
                    children.initialize_render_state(ctx)
                }
            }),
        );
    }
}
