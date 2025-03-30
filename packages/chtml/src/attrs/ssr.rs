use std::{pin::Pin, task::Poll};

use async_str_iter::{AsyncStrIterator, IntoAsyncStrIterator};

use super::SpaceAndAttributesStr;

impl<'a> IntoAsyncStrIterator for SpaceAndAttributesStr<'a> {
    type IntoAsyncStrIterator = SpaceAndAttributesStrIntoIter<'a>;

    fn into_async_str_iterator(self) -> Self::IntoAsyncStrIterator {
        SpaceAndAttributesStrIntoIter(self)
    }
}

pub struct SpaceAndAttributesStrIntoIter<'a>(SpaceAndAttributesStr<'a>);

impl<'a> SpaceAndAttributesStrIntoIter<'a> {
    pub const fn new(s: SpaceAndAttributesStr<'a>) -> Self {
        Self(s)
    }
}

impl<'a> AsyncStrIterator for SpaceAndAttributesStrIntoIter<'a> {
    fn poll_next_str(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<&str>> {
        let this = &mut self.get_mut().0 .0;
        <&'a str as AsyncStrIterator>::poll_next_str(Pin::new(this), cx)
    }
}
