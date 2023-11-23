use std::{borrow::Cow, pin::Pin, task::Poll};

use futures_io::AsyncWrite;

use crate::{
    bytes::{
        AsyncWritableByteChunks, AsyncWritableBytes, AsyncWriteInto, CowSlicedBytes, IterByteChunks,
    },
    ready_ok, Element, RenderState,
};
use frender_common::write::{
    attrs::{
        states::AsyncWriteAttrNameState, writable::AsyncWritableAttrValue, AsyncWritableAttrs,
        IntoAsyncWritableAttrs,
    },
    str::{AsyncWritableStr, StrWriting},
};

// TODO: remove
pub type HtmlAttributeValue<'a> = frender_html_common::attr::HtmlAttributeValue<Cow<'a, str>>;

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

pub struct HtmlElement<
    'a,
    Children = (),
    Attrs = [HtmlAttrPair<'a, std::iter::Empty<Cow<'a, str>>>; 0],
> {
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

pub type HtmlAttrPair<'a, S: Iterator<Item = Cow<'a, str>>> = (
    Cow<'a, str>,
    frender_html_common::attr::HtmlAttributeValue<S>,
);

pin_project_lite::pin_project!(
    pub struct HtmlElementRenderState<'a, Children, Attrs: AsyncWritableAttrs> {
        before_attrs: IterByteChunks<core::array::IntoIter<CowSlicedBytes<'a>, 2>>,
        attrs: Attrs,
        after_attrs: &'static [u8],
        #[pin]
        children: Option<Children>,
        after_children: Option<IterByteChunks<core::array::IntoIter<CowSlicedBytes<'a>, 3>>>,
    }
);

impl<'a, Children, Attrs: AsyncWritableAttrs> HtmlElementRenderState<'a, Children, Attrs> {
    pub fn new(tag: Cow<'a, str>, attrs: Attrs, children: HtmlElementChildren<Children>) -> Self {
        let after_attrs = if let HtmlElementChildren::SelfClosing = &children {
            b" />".as_slice()
        } else {
            b">"
        };

        let (children, after_children) = match children {
            HtmlElementChildren::Children(v) => (
                Some(v),
                Some(IterByteChunks::new(
                    [
                        CowSlicedBytes::Borrowed(b"</"),
                        tag.clone().into(),
                        CowSlicedBytes::Borrowed(b">"),
                    ]
                    .into_iter(),
                )),
            ),
            _ => (None, None),
        };

        let before_attrs =
            IterByteChunks::new([CowSlicedBytes::Borrowed(b"<"), tag.into()].into_iter());

        Self {
            before_attrs,
            attrs,
            after_attrs,
            children,
            after_children,
        }
    }
}

impl<'a, Children: RenderState, Attrs> RenderState for HtmlElementRenderState<'a, Children, Attrs>
where
    Attrs: AsyncWritableAttrs,
{
    fn poll_render<W: AsyncWrite + ?Sized>(
        self: Pin<&mut Self>,
        mut writer: Pin<&mut W>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<std::io::Result<()>> {
        let this = self.project();

        ready_ok!(this
            .before_attrs
            .poll_write_byte_chunks(writer.as_mut(), cx));

        _ = crate::ready_ok_rewrap_err!(this.attrs.poll_write_attrs_into(
            cx,
            crate::write_attrs::AsyncWriteAttrName::new(writer.as_mut())
        ));

        ready_ok!(this.after_attrs.poll_write_bytes(writer.as_mut(), cx));

        if let Some(children) = this.children.as_pin_mut() {
            ready_ok!(children.poll_render(writer.as_mut(), cx));
        }

        if let Some(after_children) = this.after_children {
            ready_ok!(after_children.poll_write_byte_chunks(writer, cx));
        }

        Poll::Ready(Ok(()))
    }
}

impl<'a, Children: Element, Attrs> Element for HtmlElement<'a, Children, Attrs>
where
    Attrs: IntoAsyncWritableAttrs,
{
    type SsrState = HtmlElementRenderState<'a, Children::SsrState, Attrs::AsyncWritableAttrs>;

    fn into_ssr_state(self) -> Self::SsrState {
        HtmlElementRenderState::new(
            self.tag,
            Attrs::into_async_writable_attrs(self.attributes),
            self.children
                .map_children(|children| children.into_ssr_state()),
        )
    }

    type IntoIterStrings = Option<&'static str>;

    fn into_iter_strings(self) -> Self::IntoIterStrings {
        todo!()
    }
}

pub struct WritableAttrValue<'a, V: Iterator<Item = Cow<'a, str>>> {
    current_chunk: Option<CowSlicedBytes<'a>>,
    rest_chunks: V,
}

impl<'a, V: Iterator<Item = Cow<'a, str>>> AsyncWritableByteChunks for WritableAttrValue<'a, V> {
    type Chunk = CowSlicedBytes<'a>;

    fn as_mut_current_chunk(&mut self) -> Option<&mut Self::Chunk> {
        self.current_chunk.as_mut()
    }

    fn go_to_next_chunk(&mut self) {
        self.current_chunk = self.rest_chunks.next().map(encode_double_quoted_attribute);
    }
}

impl<'a, V: Iterator<Item = Cow<'a, str>>> WritableAttrValue<'a, V> {
    fn new(mut value: V) -> Self {
        Self {
            current_chunk: value.next().map(encode_double_quoted_attribute),
            rest_chunks: value,
        }
    }
}

fn encode_double_quoted_attribute(value: Cow<str>) -> CowSlicedBytes {
    match html_escape::encode_double_quoted_attribute(&value) {
        Cow::Borrowed(_) => value.into(),
        Cow::Owned(value) => CowSlicedBytes::Owned(value.into_bytes().into()),
    }
}

pub struct WritableAttrEqValue<'a, V: Iterator<Item = Cow<'a, str>>> {
    /// `="`
    eq_dq: &'static [u8],
    value: WritableAttrValue<'a, V>,
    /// double-quote `"`
    dq: &'static [u8],
}

impl<'a, V: Iterator<Item = Cow<'a, str>>> AsyncWriteInto for WritableAttrEqValue<'a, V> {
    fn poll_write_into<W: crate::AsyncWrite + ?Sized>(
        &mut self,
        mut writer: Pin<&mut W>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<std::io::Result<()>> {
        ready_ok!(self.eq_dq.poll_write_bytes(writer.as_mut(), cx));
        ready_ok!(self.value.poll_write_byte_chunks(writer.as_mut(), cx));
        self.dq.poll_write_bytes(writer, cx)
    }
}

impl<'a, V: Iterator<Item = Cow<'a, str>>> WritableAttrEqValue<'a, V> {
    fn new(value: V) -> Self {
        Self {
            eq_dq: b"=\"",
            value: WritableAttrValue::new(value),
            dq: b"\"",
        }
    }
}

pub mod simple {
    use frender_common::write::{
        attrs::{
            writable::AsyncWritableAttrValue, AsyncWritableAttrValueBooleanTrue,
            AsyncWritableAttrValueStr,
        },
        str::StrWriting,
    };

    pub type AttrValueStr<'a> = AsyncWritableAttrValueStr<StrWriting<&'a str>>;
    pub type AttrPair<V> = super::AsyncWritableAttrPair<StrWriting<&'static str>, V>;
    pub type AttrPairStr<S> = AttrPair<AsyncWritableAttrValueStr<S>>;
    pub type AttrPairBooleanTrue = AttrPair<AsyncWritableAttrValueBooleanTrue>;
}

pub struct AsyncWritableAttrPair<N: AsyncWritableStr, V: AsyncWritableAttrValue> {
    pub name: N,
    pub name_state: AsyncWriteAttrNameState,
    pub value: V,
}

impl<S: std::borrow::Borrow<str>, V: AsyncWritableAttrValue>
    AsyncWritableAttrPair<StrWriting<S>, V>
{
    pub fn new_from_str(name: S, value: V) -> Self {
        Self {
            name: StrWriting::new(name),
            name_state: AsyncWriteAttrNameState::default(),
            value,
        }
    }
}

impl<N: AsyncWritableStr, V: AsyncWritableAttrValue> AsyncWritableAttrs
    for AsyncWritableAttrPair<N, V>
{
    fn poll_write_attrs_into<W: frender_common::write::attrs::write_traits::AsyncWriteAttrName>(
        &mut self,
        cx: &mut std::task::Context,
        write: W,
    ) -> Poll<std::io::Result<W>> {
        let write = crate::ready_ok_rewrap_err!(write.poll_write_attr_name(
            cx,
            &mut self.name,
            &mut self.name_state
        ));

        self.value.poll_write_attr_value_into(cx, write)
    }
}

// TODO: remove
pub struct WritableAttrPair<'a, V: Iterator<Item = Cow<'a, str>>> {
    space: &'static [u8],
    name: CowSlicedBytes<'a>,
    eq_value: Option<WritableAttrEqValue<'a, V>>,
}

impl<'a, V: Iterator<Item = Cow<'a, str>>> WritableAttrPair<'a, V> {
    fn new(name: Cow<'a, str>, value: frender_html_common::attr::HtmlAttributeValue<V>) -> Self {
        Self {
            space: b" ",
            name: name.into(),
            eq_value: match value {
                frender_html_common::attr::HtmlAttributeValue::String(value) => {
                    Some(WritableAttrEqValue::new(value))
                }
                frender_html_common::attr::HtmlAttributeValue::BooleanTrue => None,
            },
        }
    }

    fn from_pair(
        (name, value): (
            Cow<'a, str>,
            frender_html_common::attr::HtmlAttributeValue<V>,
        ),
    ) -> Self {
        Self::new(name, value)
    }
}

impl<'a, V: Iterator<Item = Cow<'a, str>>> AsyncWriteInto for WritableAttrPair<'a, V> {
    fn poll_write_into<W: AsyncWrite + ?Sized>(
        &mut self,
        mut writer: Pin<&mut W>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<std::io::Result<()>> {
        ready_ok!(self.space.poll_write_bytes(writer.as_mut(), cx));
        ready_ok!(self.name.poll_write_bytes(writer.as_mut(), cx));

        if let Some(ref mut eq_value) = self.eq_value {
            ready_ok!(eq_value.poll_write_into(writer.as_mut(), cx));
        }

        Poll::Ready(Ok(()))
    }
}

pub struct WritableAttrs<'a, Attrs, V>
where
    Attrs: Iterator<
        Item = (
            Cow<'a, str>,
            frender_html_common::attr::HtmlAttributeValue<V>,
        ),
    >,
    V: Iterator<Item = Cow<'a, str>>,
{
    current_attr: Option<WritableAttrPair<'a, V>>,
    rest_attrs: Attrs,
}

impl<'a, Attrs, V> WritableAttrs<'a, Attrs, V>
where
    Attrs: Iterator<
        Item = (
            Cow<'a, str>,
            frender_html_common::attr::HtmlAttributeValue<V>,
        ),
    >,
    V: Iterator<Item = Cow<'a, str>>,
{
    fn new(mut attrs: Attrs) -> Self {
        Self {
            current_attr: attrs.next().map(WritableAttrPair::from_pair),
            rest_attrs: attrs,
        }
    }
}

impl<'a, Attrs, V> AsyncWriteInto for WritableAttrs<'a, Attrs, V>
where
    Attrs: Iterator<
        Item = (
            Cow<'a, str>,
            frender_html_common::attr::HtmlAttributeValue<V>,
        ),
    >,
    V: Iterator<Item = Cow<'a, str>>,
{
    fn poll_write_into<W: crate::AsyncWrite + ?Sized>(
        &mut self,
        mut writer: Pin<&mut W>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<std::io::Result<()>> {
        while let Some(current_attr) = &mut self.current_attr {
            match current_attr.poll_write_into(writer.as_mut(), cx) {
                Poll::Ready(Ok(_)) => {
                    self.current_attr = self.rest_attrs.next().map(WritableAttrPair::from_pair)
                }
                res => return res,
            }
        }

        Poll::Ready(Ok(()))
    }
}
