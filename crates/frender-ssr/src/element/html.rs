use std::{borrow::Cow, pin::Pin, task::Poll};

use frender_core::{RenderState, UpdateRenderState};
use futures_io::AsyncWrite;

use crate::{
    bytes::{AsyncWritableByteChunks, CowSlicedBytes, IterByteChunks},
    SsrContext, WriterOrError,
};

// TODO: use html_common::
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

// TODO: use html_common
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

pub struct HtmlElementRenderState<'a, Children, Attrs: Iterator<Item = HtmlAttrPair<'a>>> {
    data: HtmlElementRenderStateData<'a, Children, Attrs>,
    is_writing: bool, // TODO: use an enum with Finished state
}

impl<
        'a,
        Children: RenderState<SsrContext<W>> + Unpin,
        Attrs: Unpin + Iterator<Item = HtmlAttrPair<'a>>,
        W: AsyncWrite + Unpin,
    > RenderState<SsrContext<W>> for HtmlElementRenderState<'a, Children, Attrs>
{
    fn unmount(self: std::pin::Pin<&mut Self>) {
        if let Some(children) = &mut self.get_mut().data.children {
            Children::unmount(Pin::new(children))
        }
    }

    fn poll_reactive(
        self: std::pin::Pin<&mut Self>,
        ctx: &mut SsrContext<W>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        let this = self.get_mut();

        let writer_or_error = match ctx.use_writer_and_mark_busy(&mut this.is_writing) {
            Some(w) => w,
            None => return Poll::Pending,
        };

        let writer = match writer_or_error {
            WriterOrError::Writer(w) => w,
            WriterOrError::Error(_) => {
                this.is_writing = false;
                ctx.busy = false;
                return Poll::Ready(());
            }
        };

        let data = &mut this.data;

        macro_rules! ready_ok {
            ($e:expr) => {
                match $e {
                    Poll::Ready(Ok(())) => {}
                    Poll::Ready(Err(error)) => {
                        this.is_writing = false;
                        *writer_or_error = WriterOrError::Error(error);
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
            ctx.busy = false;
            if let Poll::Ready(()) = Pin::new(children).poll_reactive(ctx, cx) {
                #[cfg(debug_assertions)]
                assert!(!ctx.busy);

                ctx.busy = true;
                data.children = None; // TODO: do not remove the children as drop might do something
                cx.waker().wake_by_ref();
            }

            return std::task::Poll::Pending;
        }

        ready_ok!(data
            .after_children
            .poll_write_byte_chunks(Pin::new(writer), cx));

        this.is_writing = false;
        ctx.busy = false;

        Poll::Ready(())
    }
}

impl<'a, Children: UpdateRenderState<crate::SsrContext<W>>, Attrs, W: AsyncWrite + Unpin>
    UpdateRenderState<crate::SsrContext<W>> for HtmlElement<'a, Children, Attrs>
where
    Children::State: Unpin,
    Attrs: Unpin + Iterator<Item = HtmlAttrPair<'a>>,
{
    type State = HtmlElementRenderState<'a, Children::State, Attrs>;

    fn initialize_render_state(self, ctx: &mut crate::SsrContext<W>) -> Self::State {
        HtmlElementRenderState {
            data: HtmlElementRenderStateData::new(
                self.tag,
                self.attributes,
                self.children
                    .map_children(|children| children.initialize_render_state(ctx)),
            ),
            is_writing: false,
        }
    }

    fn update_render_state(self, ctx: &mut crate::SsrContext<W>, state: Pin<&mut Self::State>) {
        let state = state.get_mut();

        if ctx.busy && state.is_writing {
            ctx.busy = false;
            state.is_writing = false;
            // TODO: warn
        }

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
