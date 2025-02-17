#![cfg(feature = "csr")]

use frender_html::cs;
use frender_keyed_elements::{Keyed, KeyedElements};
use frender_test::{element::Node, renderer::Root};

fn dom_nodes_div_i32(root: &Root) -> Vec<i32> {
    let nodes = root.clone_nodes();
    let mut nodes = nodes.iter();
    assert!(nodes.next().unwrap().is_cursor_placeholder());
    assert!(nodes.next_back().unwrap().is_cursor_placeholder());
    nodes
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
    use frender_html::CsrElement;
    use frender_test::renderer::unpinned_render_init;

    let (ref mut renderer, ref root, (ref mut state, ref mut ui_handle)) = unpinned_render_init(
        KeyedElements((0..5).map(|n| Keyed(n, cs::div().children(n)))),
    );

    assert_eq!(dom_nodes_div_i32(root), [0, 1, 2, 3, 4]);

    {
        let elements = [9, 0, 1, 2, 3, 4].map(|n| Keyed(n, cs::div().children(n)));
        KeyedElements(elements).unpinned_render_update(renderer, state, ui_handle);
        assert_eq!(dom_nodes_div_i32(root), [9, 0, 1, 2, 3, 4]);
    }

    {
        let elements = [10, 11, 12, 9, 0, 1, 2, 3, 4].map(|n| Keyed(n, cs::div().children(n)));
        KeyedElements(elements).unpinned_render_update(renderer, state, ui_handle);

        assert_eq!(dom_nodes_div_i32(root), [10, 11, 12, 9, 0, 1, 2, 3, 4]);
    }
}
