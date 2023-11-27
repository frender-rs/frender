use crate::Element;

pub async fn render_element_as_string<E: Element>(element: E) -> String {
    use async_str_iter::AsyncStrIterator;
    let s = element.into_html_children();
    let mut s = std::pin::pin!(s);

    let mut ret = String::new();
    while let Some(()) = std::future::poll_fn(|cx| {
        s.as_mut()
            .poll_next_str(cx)
            .map(|chunk| chunk.map(|chunk| ret.push_str(chunk)))
    })
    .await
    {}

    ret
}
