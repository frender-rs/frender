use frender::prelude::*;

component_fn!(
    #[component(ssr)]
    fn Main(main_id: &str) -> Element![ssr, '_] {
        intrinsic!(
            div.id(main_id).style(
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
    let output = futures_lite::future::block_on(
        frender::hook_element::frender_ssr::render_to_string(element),
    )
    .unwrap();
    println!("{}", output);
}
