use frender::prelude::*;

mod my_counter;
mod my_timer;

use my_counter::MyCounter;
use my_timer::MyTimer;

#[component(only_dom)]
fn Main(ctx: _) {
    ctx.render(rsx!(
        <div style=r#"
            margin: auto;
            padding: 16;
            max-width: 768;
        "#>
            <h1>
                "Counter & Timer - "
                <i>
                    <a href="https://github.com/frender-rs/frender" target="_blank">
                        <b children="f" />
                        "render"
                    </a>
                </i>
            </h1>
            <main>
                <div>
                    <code>"<MyCounter />"</code>
                    <MyCounter />
                </div>
                <div>
                    <code>"<MyCounter initial_value={3} />"</code>
                    <MyCounter initial_value={3} />
                </div>
                <div>
                    <code>"<MyTimer initial_interval={1000} />"</code>
                    <MyTimer initial_interval={1000} />
                </div>
                <div>
                    <code>"<MyTimer initial_interval={500} />"</code>
                    <MyTimer initial_interval={500} />
                </div>
            </main>
        </div>
    ))
}

// #[component(only_dom)]
// fn Main(ctx: _) {
//     ctx.render(rsx!(
//         <>
//             "<MyCounter />"
//             <MyCounter />
//             "<MyCounter initial_value=5 />"
//             <MyCounter initial_value=5 />

//             "<MyTimer />"
//             <MyTimer />

//             "<MyTimer initial_interval=2000 />"
//             <MyTimer initial_interval=2000 />
//         </>
//     ))
// }

fn main() {
    frender::hook_element::frender_dom::spawn_mount_to_dom_element_by_id(
        rsx!(Main()),
        "frender-root",
    );
}
