use std::{borrow::Cow, pin::Pin, task::Poll};

use futures_io::AsyncWrite;

use crate::{
    attrs::IntoIteratorAttrs,
    bytes::{AsyncWritableByteChunks, CowSlicedBytes, IterByteChunks},
    ready_ok, Element, RenderState,
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
    pub struct HtmlElementRenderState<'a, Children, Attrs: Iterator<Item = HtmlAttrPair<'a>>> {
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
        pub children: Option<Children>,
        pub after_children: IterByteChunks<core::array::IntoIter<CowSlicedBytes<'a>, 3>>,
    }
);

impl<'a, Children, Attrs: Iterator<Item = HtmlAttrPair<'a>>>
    HtmlElementRenderState<'a, Children, Attrs>
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

impl<'a, Children: RenderState, Attrs: Iterator<Item = HtmlAttrPair<'a>>> RenderState
    for HtmlElementRenderState<'a, Children, Attrs>
{
    fn poll_render<W: AsyncWrite + ?Sized>(
        self: Pin<&mut Self>,
        mut writer: Pin<&mut W>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<std::io::Result<()>> {
        let this = self.project();

        ready_ok!(this
            .before_children
            .poll_write_byte_chunks(writer.as_mut(), cx));

        if let Some(children) = this.children.as_pin_mut() {
            ready_ok!(children.poll_render(writer.as_mut(), cx));
        }

        ready_ok!(this.after_children.poll_write_byte_chunks(writer, cx));

        Poll::Ready(Ok(()))
    }
}

impl<'a, Children: Element, Attrs> Element for HtmlElement<'a, Children, Attrs>
where
    Attrs: IntoIteratorAttrs<'a>,
{
    type SsrState = HtmlElementRenderState<'a, Children::SsrState, Attrs::IntoIterAttrs>;

    fn into_ssr_state(self) -> Self::SsrState {
        HtmlElementRenderState::new(
            self.tag,
            Attrs::into_iter_attrs(self.attributes),
            self.children
                .map_children(|children| children.into_ssr_state()),
        )
    }
}
