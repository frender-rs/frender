use frender::{prelude::*, SsrElementExt};

component_fn!(
    #[component(ssr)]
    fn Main(main_id: &str) -> impl SsrElement + '_ {
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

fn main() {
    let element = Main("frender-root");
    let output = futures_lite::future::block_on(element.render_as_string()).unwrap();
    println!("{}", output);
}
