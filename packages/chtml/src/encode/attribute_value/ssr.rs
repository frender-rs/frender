use std::pin::Pin;

use async_str_iter::{AsyncStrIterator, IntoAsyncStrIterator};

use super::EqValueStr;

pub struct EqValueStrIntoIter<'a>(&'a str);

impl<'a> EqValueStrIntoIter<'a> {
    pub const fn new(s: EqValueStr<'a>) -> Self {
        Self(s.0)
    }
}

impl<'a> IntoAsyncStrIterator for EqValueStr<'a> {
    type IntoAsyncStrIterator = EqValueStrIntoIter<'a>;

    fn into_async_str_iterator(self) -> Self::IntoAsyncStrIterator {
        EqValueStrIntoIter(self.0)
    }
}

impl<'a> AsyncStrIterator for EqValueStrIntoIter<'a> {
    fn poll_next_str(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<&str>> {
        <&str>::poll_next_str(Pin::new(&mut self.get_mut().0), cx)
    }
}
