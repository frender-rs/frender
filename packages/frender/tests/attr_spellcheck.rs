use frender::prelude::*;

#[test]
fn ssr() {
    {
        let el = cs::div().spellcheck(false);
        let out = futures_lite::future::block_on(el.render_to_string());
        assert_eq!(out, r##"<div spellcheck="false"></div>"##)
    }
    {
        let el = cs::div().spellcheck(true);
        let out = futures_lite::future::block_on(el.render_to_string());
        assert_eq!(out, r##"<div spellcheck="true"></div>"##)
    }
    {
        let el = cs::div().spellcheck(frender::Empty);
        let out = futures_lite::future::block_on(el.render_to_string());
        assert_eq!(out, r##"<div spellcheck></div>"##)
    }
}
