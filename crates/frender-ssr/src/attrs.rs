// pub struct Chain<A, B>(A, B);

use std::borrow::Cow;

use frender_html_common::attr::HtmlAttributeValue;

use crate::{bytes::IntoAsyncWritableBytes, element::html::HtmlAttrPair};

pub trait ReceiveStringChunk {
    fn receive_string_chunk<S: IntoAsyncWritableBytes>(&mut self, chunk: Option<S>);
}

pub trait GiveStringChunks {
    fn give_next_string_chunk(&mut self, receiver: &mut impl ReceiveStringChunk);
}

impl<S: IntoAsyncWritableBytes> GiveStringChunks for Option<S> {
    fn give_next_string_chunk(&mut self, receiver: &mut impl ReceiveStringChunk) {
        receiver.receive_string_chunk(self.take())
    }
}

pub enum NoStringChunks {}

impl GiveStringChunks for NoStringChunks {
    fn give_next_string_chunk(&mut self, receiver: &mut impl ReceiveStringChunk) {
        match *self {}
    }
}

pub trait ReceiveAttr {
    fn receive_attr<N: GiveStringChunks, V: GiveStringChunks>(
        &mut self,
        attr_pair: Option<(N, HtmlAttributeValue<V>)>,
    );
}

pub trait GiveAttrs {
    fn give_next_attr(&mut self, receiver: &mut impl ReceiveAttr);
}

pub trait IntoGiveAttrs {
    type GiveAttrs: GiveAttrs;

    fn into_give_attrs(self) -> Self::GiveAttrs;
}

pub struct OneAttrPair<N: GiveStringChunks, V: GiveStringChunks>(pub N, pub HtmlAttributeValue<V>);

impl<N: GiveStringChunks, V: GiveStringChunks> IntoGiveAttrs for OneAttrPair<N, V> {
    type GiveAttrs = Option<Self>;

    fn into_give_attrs(self) -> Self::GiveAttrs {
        Some(self)
    }
}

impl<N: GiveStringChunks, V: GiveStringChunks> GiveAttrs for Option<OneAttrPair<N, V>> {
    fn give_next_attr(&mut self, receiver: &mut impl ReceiveAttr) {
        receiver.receive_attr(self.take().map(|OneAttrPair(name, value)| (name, value)))
    }
}

impl GiveAttrs for () {
    fn give_next_attr(&mut self, receiver: &mut impl ReceiveAttr) {
        receiver.receive_attr(None::<(NoStringChunks, HtmlAttributeValue<NoStringChunks>)>)
    }
}

impl<A: GiveAttrs, B: GiveAttrs> GiveAttrs for (A, B) {
    fn give_next_attr(&mut self, receiver: &mut impl ReceiveAttr) {
        let (a, b) = self;

        fused::FusedReceiver::new(receiver)
            .do_and_continue_if_ended(|rx| a.give_next_attr(rx), |rx| b.give_next_attr(rx))
    }
}

mod fused {
    use frender_html_common::attr::HtmlAttributeValue;

    use super::{GiveStringChunks, ReceiveAttr};

    pub(super) struct FusedReceiver<'a, R> {
        ended: bool,
        receiver: &'a mut R,
    }

    impl<'a, R> FusedReceiver<'a, R> {
        pub(super) fn new(receiver: &'a mut R) -> Self {
            Self {
                ended: false,
                receiver,
            }
        }

        pub(super) fn do_and_continue_if_ended(
            &mut self,
            mut f_do: impl FnMut(&mut Self),
            mut f_continue: impl FnMut(&mut R),
        ) {
            f_do(self);
            if self.ended {
                f_continue(self.receiver);
            }
        }
    }

    impl<R: ReceiveAttr> ReceiveAttr for FusedReceiver<'_, R> {
        fn receive_attr<N: GiveStringChunks, V: GiveStringChunks>(
            &mut self,
            attr_pair: Option<(N, HtmlAttributeValue<V>)>,
        ) {
            if attr_pair.is_none() {
                self.ended = true;
            } else {
                self.receiver.receive_attr(attr_pair)
            }
        }
    }
}
