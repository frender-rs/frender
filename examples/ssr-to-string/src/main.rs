use frender::{prelude::*, ssr::ElementExt};

component_fn!(
    #[component(ssr)]
    fn Main(main_id: &str) -> impl Element + '_ {
        intrinsic!(
            div.class("main").id(main_id).style(
                r#"margin: auto;
padding: 16px;
max-width: 768px;
"#
            )[[h1[[
                "Ssr to String - ",
                i[[a.href("https://github.com/frender-rs/frender")
                    .target("_blank")[[b.children("f"), "render"]]]],
            ]]]]
        )
    }
);

const EXPECTED: &'static str = r##"
<div class="main" id="frender-root" style="margin: auto;
padding: 16px;
max-width: 768px;
"><h1>Ssr to String - <i><a href="https://github.com/frender-rs/frender" target="_blank"><b>f</b>render</a></i></h1></div>
"##;

fn main() {
    let element = Main("frender-root");
    // let output = futures_lite::future::block_on(element.render_as_string()).unwrap();
    let output = futures_lite::future::block_on(frender::ssr::render_element_as_string(element));
    assert_eq!(output, EXPECTED.trim());
}
