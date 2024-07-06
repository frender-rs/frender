#[cfg(test)]
mod test {
    use async_str_iter::Strings;

    // use super::AsyncStrIterator;
    Strings![
        enum MyDivState {}
        pub struct MyElement<Attrs: async_str_iter::AsyncStrIterator>(
            //
            lt!("<"),
            tag!(&'static str),
            attrs!(Attrs),
            gt!(">"),
        );
    ];

    #[allow(non_snake_case)]
    pub fn MyElement<Attrs: async_str_iter::AsyncStrIterator>(
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
                use async_str_iter::AsyncStrIterator;
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
