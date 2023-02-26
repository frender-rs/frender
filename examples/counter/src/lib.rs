use frender::prelude::*;

mod my_counter;
mod my_timer;

use my_counter::MyCounter;
use my_timer::MyTimer;

#[component(only_dom, main(get_dom_element = "frender-root"))]
pub fn Main() {
    rsx!(
        <div style=r#"
            margin: auto;
            padding: 16px;
            max-width: 768px;
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
    )
}
