use frender::prelude::*;
use futures_lite::future::block_on;

#[test]
fn script_empty() {
    assert_eq!(
        block_on(rsx!(<script></script>).render_to_string()),
        "<script></script>"
    );
    assert_eq!(
        block_on(rsx!(<script>{()}</script>).render_to_string()),
        "<script></script>"
    );
}

#[test]
fn script_option_empty() {
    assert_eq!(
        block_on(rsx!(<script>{None::<()>}</script>).render_to_string()),
        "<script></script>"
    );
    assert_eq!(
        block_on(rsx!(<script>{Some(())}</script>).render_to_string()),
        "<script></script>"
    );
}

#[test]
fn script_danger() {
    use frender::ScriptInnerTextWronglyEncoded;
    assert_eq!(
        block_on(
            rsx!(<script>{
            ScriptInnerTextWronglyEncoded("</script>")
        }</script>)
            .render_to_string()
        ),
        "<script><\\/script></script>"
    );

    // TODO(encode_script): THIS IS WRONG!
    assert_eq!(
        block_on(
            intrinsic!(script[[{ Some(ScriptInnerTextWronglyEncoded("1</script/")) }]])
                .render_to_string()
        ),
        "<script>1</script/</script>"
    );
}
