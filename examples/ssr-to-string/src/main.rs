use frender::prelude::*;

fn div_with_id(s: &str) -> Element![ssr, '_] {
    intrinsic!(div.id(s))
}

fn main() {
    let element = div_with_id("frender-root");
    let output = futures_lite::future::block_on(
        frender::hook_element::frender_ssr::render_to_string(element),
    )
    .unwrap();
    println!("{}", output);
}
