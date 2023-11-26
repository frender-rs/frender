pub use frender_common::async_str::{chain, empty, option};
pub use frender_ssr_html::encode;

pub mod flat;

#[cfg(test)]
mod test {
    use frender_common::Strings;

    // use super::AsyncStrIterator;
    Strings![
        enum MyDivState {}
        pub struct MyElement<Attrs: crate::AsyncStrIterator>(
            //
            lt!("<"),
            tag!(&'static str),
            attrs!(Attrs),
            gt!(">"),
        );
    ];

    #[allow(non_snake_case)]
    pub fn MyElement<Attrs: crate::AsyncStrIterator>(
        tag: &'static str,
        attrs: Attrs,
    ) -> MyElement<Attrs> {
        MyElement {
            _state: MyDivState(),
            lt: (),
            tag,
            attrs,
            gt: (),
        }
    }

    #[test]
    fn test() {
        let el = MyElement("div", "");
        let mut el = std::pin::pin!(el);

        futures_lite::future::block_on(async {
            let mut s = String::new();
            while let Some(()) = std::future::poll_fn(|cx| {
                use crate::AsyncStrIterator;
                el.as_mut()
                    .poll_next_str(cx)
                    .map(|v| v.map(|v| s.push_str(v)))
            })
            .await
            {}

            assert_eq!(s, "<div>")
        })
    }
}
