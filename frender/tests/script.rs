use frender::prelude::*;
use frender::ssr::render_element_as_string;
use futures_lite::future::block_on;

#[test]
fn script_empty() {
    assert_eq!(
        block_on(render_element_as_string(rsx!(<script></script>))),
        "<script></script>"
    );
    assert_eq!(
        block_on(render_element_as_string(rsx!(<script>{()}</script>))),
        "<script></script>"
    );
}

#[test]
fn script_option_empty() {
    assert_eq!(
        block_on(render_element_as_string(
            rsx!(<script>{None::<()>}</script>)
        )),
        "<script></script>"
    );
    assert_eq!(
        block_on(render_element_as_string(rsx!(<script>{Some(())}</script>))),
        "<script></script>"
    );
}

#[test]
fn script_danger() {
    use frender::ScriptInnerTextWronglyEncoded;
    assert_eq!(
        block_on(render_element_as_string(rsx!(<script>{
            ScriptInnerTextWronglyEncoded("</script>")
        }</script>))),
        "<script><\\/script></script>"
    );

    // TODO(encode_script): THIS IS WRONG!
    assert_eq!(
        block_on(render_element_as_string(intrinsic!(
            script[[{ Some(ScriptInnerTextWronglyEncoded("1</script/")) }]]
        ))),
        "<script>1</script/</script>"
    );
}
