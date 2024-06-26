#![cfg(feature = "csr")]

use frender_elements::{Elements, Keyed};
use frender_html::{cs, prelude_props_builders::*, Element};
use frender_test::{element::Node, renderer::RendererWithRoot};

fn dom_nodes_div_i32(dom: &RendererWithRoot) -> Vec<i32> {
    dom.nodes()
        .iter()
        .map(Node::as_element)
        .map(|el| {
            let el = el.unwrap().data_cloned();
            assert_eq!(el.tag, "div");
            assert_eq!(el.attrs, []);
            assert_eq!(el.children.len(), 1);
            el.children[0]
                .as_text()
                .unwrap()
                .to_string()
                .parse()
                .unwrap()
        })
        .collect::<Vec<_>>()
}

#[cfg(feature = "csr")]
#[test]
fn prepend() {
    let mut dom = RendererWithRoot::new();
    let mut render_state = Default::default();

    assert!(dom.nodes().is_empty());

    {
        let elements = (0..5).map(|n| Keyed(n, cs::div.children(n)));
        dom.unpinned_render_update(Elements(elements), &mut render_state);
        assert_eq!(dom_nodes_div_i32(&dom), [0, 1, 2, 3, 4]);
    }

    {
        let elements = [9, 0, 1, 2, 3, 4].map(|n| Keyed(n, cs::div.children(n)));
        dom.unpinned_render_update(Elements(elements), &mut render_state);
        assert_eq!(dom_nodes_div_i32(&dom), [9, 0, 1, 2, 3, 4]);
    }

    {
        let elements = [10, 11, 12, 9, 0, 1, 2, 3, 4].map(|n| Keyed(n, cs::div.children(n)));
        dom.unpinned_render_update(Elements(elements), &mut render_state);

        assert_eq!(dom_nodes_div_i32(&dom), [10, 11, 12, 9, 0, 1, 2, 3, 4]);
    }
}
