use std::{borrow::Cow, pin::Pin, task::Poll};

use frender_core::{RenderState, UpdateRenderState};
use futures_io::AsyncWrite;

use crate::{
    attrs::IntoIteratorAttrs,
    bytes::{AsyncWritableByteChunks, CowSlicedBytes, IterByteChunks},
    SsrContext, WriterOrError, WritingState,
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

pin_project_lite::pin_project!(
    pub struct HtmlElementRenderStateChildren<Children> {
        children_writing_state: WritingState,
        #[pin]
        children: Children,
    }
);

pin_project_lite::pin_project!(
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
        #[pin]
        pub children: Option<HtmlElementRenderStateChildren<Children>>,
        pub after_children: IterByteChunks<core::array::IntoIter<CowSlicedBytes<'a>, 3>>,
    }
);

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

        let children = children.map(|children| HtmlElementRenderStateChildren {
            children_writing_state: Default::default(),
            children,
        });

        Self {
            before_children,
            children,
            after_children: IterByteChunks::new(after_children.into_iter()),
        }
    }
}

pin_project_lite::pin_project!(
    pub struct HtmlElementRenderState<'a, Children, Attrs: Iterator<Item = HtmlAttrPair<'a>>> {
        #[pin]
        data: HtmlElementRenderStateData<'a, Children, Attrs>,
        writing_state: WritingState,
    }
);

impl<'a, Children, Attrs: Iterator<Item = HtmlAttrPair<'a>>>
    HtmlElementRenderStateData<'a, Children, Attrs>
{
    fn impl_poll<W: AsyncWrite + Unpin>(
        self: Pin<&mut Self>,
        ctx: &mut SsrContext<W>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()>
    where
        Children: RenderState<SsrContext<W>>,
    {
        #[cfg(debug_assertions)]
        assert!(ctx.busy);

        let writer = match &mut ctx.writer_or_error {
            WriterOrError::Writer(w) => w,
            WriterOrError::Error(_) => return Poll::Ready(()),
        };

        macro_rules! ready_ok {
            ($e:expr) => {
                match $e {
                    Poll::Ready(Ok(())) => {}
                    Poll::Ready(Err(error)) => {
                        ctx.writer_or_error = WriterOrError::Error(error);
                        return Poll::Ready(());
                    }
                    Poll::Pending => return Poll::Pending,
                }
            };
        }

        let this = self.project();

        ready_ok!(this
            .before_children
            .poll_write_byte_chunks(Pin::new(writer), cx));

        if let Some(children) = this.children.as_pin_mut() {
            let children = children.project();
            let children_writing_state = children.children_writing_state;
            let children = children.children;

            if !children_writing_state.is_finished() {
                #[cfg(debug_assertions)]
                assert!(ctx.busy);

                if children_writing_state.is_ready_to_start() {
                    ctx.busy = false;
                    *children_writing_state = WritingState::Writing;
                }
                if let Poll::Ready(()) = children.poll_reactive(ctx, cx) {
                    #[cfg(debug_assertions)]
                    assert!(!ctx.busy);

                    ctx.busy = true;
                    *children_writing_state = WritingState::Finished;
                    cx.waker().wake_by_ref();
                }

                #[cfg(debug_assertions)]
                assert!(ctx.busy);

                return Poll::Pending;
            }
        }

        ready_ok!(this
            .after_children
            .poll_write_byte_chunks(Pin::new(writer), cx));

        Poll::Ready(())
    }
}

impl<
        'a,
        Children: RenderState<SsrContext<W>>,
        Attrs: Iterator<Item = HtmlAttrPair<'a>>,
        W: AsyncWrite + Unpin,
    > RenderState<SsrContext<W>> for HtmlElementRenderState<'a, Children, Attrs>
{
    fn unmount(self: std::pin::Pin<&mut Self>) {
        panic!("ssr cannot be unmounted");
    }

    fn poll_reactive(
        self: std::pin::Pin<&mut Self>,
        ctx: &mut SsrContext<W>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        let this = self.project();
        let writing_state = this.writing_state;
        let data = this.data;

        ctx.map(writing_state, |ctx, cx| data.impl_poll(ctx, cx), cx)
    }
}

impl<'a, Children: UpdateRenderState<crate::SsrContext<W>>, Attrs, W: AsyncWrite + Unpin>
    UpdateRenderState<crate::SsrContext<W>> for HtmlElement<'a, Children, Attrs>
where
    Attrs: IntoIteratorAttrs<'a>,
{
    type State = HtmlElementRenderState<'a, Children::State, Attrs::IntoIterAttrs>;

    fn initialize_render_state(self, ctx: &mut crate::SsrContext<W>) -> Self::State {
        HtmlElementRenderState {
            data: HtmlElementRenderStateData::new(
                self.tag,
                Attrs::into_iter_attrs(self.attributes),
                self.children
                    .map_children(|children| children.initialize_render_state(ctx)),
            ),
            writing_state: Default::default(),
        }
    }

    fn update_render_state(self, ctx: &mut crate::SsrContext<W>, state: Pin<&mut Self::State>) {
        panic!("ssr element can not be updated");
    }
}
