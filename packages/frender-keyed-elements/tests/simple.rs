#![cfg(feature = "csr")]

use std::str::FromStr;

use frender_html::CsrElement as _;
use frender_keyed_elements::{Keyed, KeyedElements};
use frender_test::{
    element::Node,
    renderer::{unpinned_render_init, Root},
};

fn dom_nodes_parse<T: FromStr + PartialEq>(root: &Root) -> Vec<T>
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    let nodes = root.clone_nodes();
    let mut nodes = nodes.iter();
    assert!(nodes.next().unwrap().is_cursor_placeholder());
    assert!(nodes.next_back().unwrap().is_cursor_placeholder());
    nodes
        .map(Node::as_text)
        .map(|text| T::from_str(&text.unwrap().to_string()).unwrap())
        .collect::<Vec<_>>()
}

#[cfg(feature = "csr")]
#[test]
fn remove() {
    let (ref mut renderer, ref root, (ref mut state, ref mut ui_handle)) =
        unpinned_render_init(KeyedElements((0..5).map(|n| Keyed(n, n))));
    assert_eq!(dom_nodes_parse::<i32>(root), [0, 1, 2, 3, 4]);

    {
        let elements = (0..5).filter(|n| *n != 2).map(|n| Keyed(n, n));
        KeyedElements(elements).unpinned_render_update(renderer, state, ui_handle);
        assert_eq!(dom_nodes_parse::<i32>(root), [0, 1, 3, 4]);
    }

    {
        let elements = std::iter::empty::<Keyed<i32, i32>>();
        KeyedElements(elements).unpinned_render_update(renderer, state, ui_handle);
        assert_eq!(dom_nodes_parse::<i32>(root), []);
    }
}

#[cfg(feature = "csr")]
#[test]
fn replace() {
    let (ref mut renderer, ref root, (ref mut state, ref mut ui_handle)) =
        unpinned_render_init(KeyedElements((0..5).map(|n| Keyed(n, n))));
    assert_eq!(dom_nodes_parse::<i32>(root), [0, 1, 2, 3, 4]);

    {
        let elements = [0, 1, 9, 3, 4].map(|n| Keyed(n, n));
        KeyedElements(elements).unpinned_render_update(renderer, state, ui_handle);
        assert_eq!(dom_nodes_parse::<i32>(root), [0, 1, 9, 3, 4]);
    }

    {
        let elements = [20, 1, 9, 3, 4].map(|n| Keyed(n, n));
        KeyedElements(elements).unpinned_render_update(renderer, state, ui_handle);
        assert_eq!(dom_nodes_parse::<i32>(root), [20, 1, 9, 3, 4]);
    }
}

#[cfg(feature = "csr")]
#[test]
fn move_one() {
    let (ref mut renderer, ref root, (ref mut state, ref mut ui_handle)) =
        unpinned_render_init(KeyedElements((0..5).map(|n| Keyed(n, n))));
    assert_eq!(dom_nodes_parse::<i32>(root), [0, 1, 2, 3, 4]);

    {
        let elements = [0, 4, 1, 2, 3].map(|n| Keyed(n, n));
        KeyedElements(elements).unpinned_render_update(renderer, state, ui_handle);
        assert_eq!(dom_nodes_parse::<i32>(root), [0, 4, 1, 2, 3]);
    }

    {
        let elements = [0, 4, 1, 3, 2].map(|n| Keyed(n, n));
        KeyedElements(elements).unpinned_render_update(renderer, state, ui_handle);
        assert_eq!(dom_nodes_parse::<i32>(root), [0, 4, 1, 3, 2]);
    }
}

#[cfg(feature = "csr")]
#[test]
fn prepend() {
    let (ref mut renderer, ref root, (ref mut state, ref mut ui_handle)) =
        unpinned_render_init(KeyedElements((0..5).map(|n| Keyed(n, n))));
    assert_eq!(dom_nodes_parse::<i32>(root), [0, 1, 2, 3, 4]);

    {
        let elements = [9, 0, 1, 2, 3, 4].map(|n| Keyed(n, n));
        KeyedElements(elements).unpinned_render_update(renderer, state, ui_handle);
        assert_eq!(dom_nodes_parse::<i32>(root), [9, 0, 1, 2, 3, 4]);
    }

    {
        let elements = [10, 11, 12, 9, 0, 1, 2, 3, 4].map(|n| Keyed(n, n));
        KeyedElements(elements).unpinned_render_update(renderer, state, ui_handle);
        assert_eq!(dom_nodes_parse::<i32>(root), [10, 11, 12, 9, 0, 1, 2, 3, 4]);
    }
}

#[cfg(feature = "csr")]
#[test]
fn prepend_many() {
    let (ref mut renderer, ref root, (ref mut state, ref mut ui_handle)) =
        unpinned_render_init(KeyedElements((0..5).map(|n| Keyed(n, n))));
    assert_eq!(dom_nodes_parse::<i32>(root), [0, 1, 2, 3, 4]);

    {
        let elements = [5, 6, 7, 8, 9, 0, 1, 2, 3, 4].map(|n| Keyed(n, n));
        KeyedElements(elements).unpinned_render_update(renderer, state, ui_handle);
        assert_eq!(dom_nodes_parse::<i32>(root), [5, 6, 7, 8, 9, 0, 1, 2, 3, 4]);
    }
}

#[cfg(feature = "csr")]
#[test]
fn append() {
    let (ref mut renderer, ref root, (ref mut state, ref mut ui_handle)) =
        unpinned_render_init(KeyedElements((0..5).map(|n| Keyed(n, n))));
    assert_eq!(dom_nodes_parse::<i32>(root), [0, 1, 2, 3, 4]);

    {
        let elements = (0..6).map(|n| Keyed(n, n));
        KeyedElements(elements).unpinned_render_update(renderer, state, ui_handle);
        assert_eq!(dom_nodes_parse::<i32>(root), [0, 1, 2, 3, 4, 5]);
    }

    {
        let elements = (0..9).map(|n| Keyed(n, n));
        KeyedElements(elements).unpinned_render_update(renderer, state, ui_handle);
        assert_eq!(dom_nodes_parse::<i32>(root), [0, 1, 2, 3, 4, 5, 6, 7, 8]);
    }
}

#[cfg(feature = "csr")]
#[test]
fn remove_one_by_one() {
    let (ref mut renderer, ref root, (ref mut state, ref mut ui_handle)) =
        unpinned_render_init(KeyedElements((0..5).map(|n| Keyed(n, n))));
    assert_eq!(dom_nodes_parse::<i32>(root), [0, 1, 2, 3, 4]);

    {
        let elements = [0, 1, 2, 4].map(|n| Keyed(n, n));
        KeyedElements(elements).unpinned_render_update(renderer, state, ui_handle);
        assert_eq!(dom_nodes_parse::<i32>(root), [0, 1, 2, 4]);
    }

    {
        let elements = [0, 1, 4].map(|n| Keyed(n, n));
        KeyedElements(elements).unpinned_render_update(renderer, state, ui_handle);
        assert_eq!(dom_nodes_parse::<i32>(root), [0, 1, 4]);
    }
}

#[cfg(feature = "csr")]
#[test]
fn swap() {
    let (ref mut renderer, ref root, (ref mut state, ref mut ui_handle)) =
        unpinned_render_init(KeyedElements([0, 1].map(|n| Keyed(n, n))));
    assert_eq!(dom_nodes_parse::<i32>(root), [0, 1]);

    {
        let elements = [1, 0].map(|n| Keyed(n, n));
        KeyedElements(elements).unpinned_render_update(renderer, state, ui_handle);
        assert_eq!(dom_nodes_parse::<i32>(root), [1, 0]);
    }

    {
        let elements = [0, 1].map(|n| Keyed(n, n));
        KeyedElements(elements).unpinned_render_update(renderer, state, ui_handle);
        assert_eq!(dom_nodes_parse::<i32>(root), [0, 1]);
    }
}
