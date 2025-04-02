#![cfg(feature = "components")]
#![cfg(feature = "csr")]
#![cfg(feature = "ssr")]

use std::pin::pin;

use async_str_iter::AsyncStrIterator;
use frender_attrs::attrs;
use frender_html::{cs, csr::CsrElement};
use frender_ssr::SsrElement;

fn el() -> impl CsrElement + SsrElement {
    cs::div().class("wrapper").frender_with_attrs(attrs!(
        r##"
            data-a='&'
            id='my-div'
        "##
    ))
}

fn ssr_all_ready(v: impl AsyncStrIterator) -> String {
    let mut v = pin!(v);

    let mut s = String::new();
    let cx = &mut std::task::Context::from_waker(std::task::Waker::noop());
    loop {
        match v.as_mut().poll_next_str(cx) {
            std::task::Poll::Ready(Some(v)) => s.push_str(v),
            std::task::Poll::Ready(None) => break,
            std::task::Poll::Pending => panic!(),
        }
    }

    s
}

#[test]
fn test() {
    let html = el().into_html_children();
    let html = ssr_all_ready(html);

    assert_eq!(html, "<div class=\"wrapper\" data-a='&' id='my-div'></div>")
}
