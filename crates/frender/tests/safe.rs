use frender::prelude::*;

fn separated_html() -> impl Element {
    ("<", "script>alert('DANGER')</", "script>")
}

#[test]
fn separated_html_is_safe() {
    let out = futures_lite::future::block_on(separated_html().render_to_string());

    assert_eq!(
        out,
        "&lt;script&gt;alert(&#x27;DANGER&#x27;)&lt;&#x2F;script&gt;"
    );
}

#[test]
fn chars_are_safe() {
    let chars = [
        '<', 's', 'c', 'r', 'i', 'p', 't', '>', 'a', 'l', 'e', 'r', 't', '(', '\'', 'D', 'A', 'N',
        'G', 'E', 'R', '\'', ')', '<', '/', 's', 'c', 'r', 'i', 'p', 't', '>',
    ];
    let out = futures_lite::future::block_on(chars.render_to_string());

    assert_eq!(
        out,
        "&lt;script&gt;alert(&#x27;DANGER&#x27;)&lt;&#x2F;script&gt;"
    );
}
