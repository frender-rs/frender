#![cfg(feature = "csr")]

use std::str::FromStr;

use frender_elements::{Elements, Keyed};
use frender_html::Element;
use frender_test::{element::Node, renderer::RendererWithRoot};

fn dom_nodes_parse<T: FromStr + PartialEq>(dom: &RendererWithRoot) -> Vec<T>
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    dom.nodes()
        .iter()
        .map(Node::as_text)
        .map(|text| T::from_str(&text.unwrap().to_string()).unwrap())
        .collect::<Vec<_>>()
}

#[cfg(feature = "csr")]
#[test]
fn remove() {
    let mut dom = RendererWithRoot::new();
    let mut render_state = Default::default();

    assert!(dom.nodes().is_empty());

    {
        let elements = (0..5).map(|n| Keyed(n, n));
        dom.unpinned_render_update(Elements(elements), &mut render_state);
        assert_eq!(dom_nodes_parse::<i32>(&dom), [0, 1, 2, 3, 4]);
    }

    {
        let elements = (0..5).filter(|n| *n != 2).map(|n| Keyed(n, n));
        dom.unpinned_render_update(Elements(elements), &mut render_state);
        assert_eq!(dom_nodes_parse::<i32>(&dom), [0, 1, 3, 4]);
    }

    {
        let elements = std::iter::empty::<Keyed<i32, i32>>();
        dom.unpinned_render_update(Elements(elements), &mut render_state);
        assert_eq!(dom_nodes_parse::<i32>(&dom), []);
    }
}

#[cfg(feature = "csr")]
#[test]
fn replace() {
    let mut dom = RendererWithRoot::new();
    let mut render_state = Default::default();

    assert!(dom.nodes().is_empty());

    {
        let elements = (0..5).map(|n| Keyed(n, n));
        dom.unpinned_render_update(Elements(elements), &mut render_state);
        assert_eq!(dom_nodes_parse::<i32>(&dom), [0, 1, 2, 3, 4]);
    }

    {
        let elements = [0, 1, 9, 3, 4].map(|n| Keyed(n, n));
        dom.unpinned_render_update(Elements(elements), &mut render_state);
        assert_eq!(dom_nodes_parse::<i32>(&dom), [0, 1, 9, 3, 4]);
    }

    {
        let elements = [20, 1, 9, 3, 4].map(|n| Keyed(n, n));
        dom.unpinned_render_update(Elements(elements), &mut render_state);
        assert_eq!(dom_nodes_parse::<i32>(&dom), [20, 1, 9, 3, 4]);
    }
}

#[cfg(feature = "csr")]
#[test]
fn move_one() {
    let mut dom = RendererWithRoot::new();
    let mut render_state = Default::default();

    assert!(dom.nodes().is_empty());

    {
        let elements = (0..5).map(|n| Keyed(n, n));
        dom.unpinned_render_update(Elements(elements), &mut render_state);
        assert_eq!(dom_nodes_parse::<i32>(&dom), [0, 1, 2, 3, 4]);
    }

    {
        let elements = [0, 4, 1, 2, 3].map(|n| Keyed(n, n));
        dom.unpinned_render_update(Elements(elements), &mut render_state);
        assert_eq!(dom_nodes_parse::<i32>(&dom), [0, 4, 1, 2, 3]);
    }

    {
        let elements = [0, 4, 1, 3, 2].map(|n| Keyed(n, n));
        dom.unpinned_render_update(Elements(elements), &mut render_state);
        assert_eq!(dom_nodes_parse::<i32>(&dom), [0, 4, 1, 3, 2]);
    }
}

#[cfg(feature = "csr")]
#[test]
fn prepend() {
    let mut dom = RendererWithRoot::new();
    let mut render_state = Default::default();

    assert!(dom.nodes().is_empty());

    {
        let elements = (0..5).map(|n| Keyed(n, n));
        dom.unpinned_render_update(Elements(elements), &mut render_state);
        assert_eq!(dom_nodes_parse::<i32>(&dom), [0, 1, 2, 3, 4]);
    }

    {
        let elements = [9, 0, 1, 2, 3, 4].map(|n| Keyed(n, n));
        dom.unpinned_render_update(Elements(elements), &mut render_state);
        assert_eq!(dom_nodes_parse::<i32>(&dom), [9, 0, 1, 2, 3, 4]);
    }

    {
        let elements = [10, 11, 12, 9, 0, 1, 2, 3, 4].map(|n| Keyed(n, n));
        dom.unpinned_render_update(Elements(elements), &mut render_state);
        assert_eq!(dom_nodes_parse::<i32>(&dom), [10, 11, 12, 9, 0, 1, 2, 3, 4]);
    }
}

#[cfg(feature = "csr")]
#[test]
fn prepend_many() {
    let mut dom = RendererWithRoot::new();
    let mut render_state = Default::default();

    assert!(dom.nodes().is_empty());

    {
        let elements = (0..5).map(|n| Keyed(n, n));
        dom.unpinned_render_update(Elements(elements), &mut render_state);
        assert_eq!(dom_nodes_parse::<i32>(&dom), [0, 1, 2, 3, 4]);
    }

    {
        let elements = [5, 6, 7, 8, 9, 0, 1, 2, 3, 4].map(|n| Keyed(n, n));
        dom.unpinned_render_update(Elements(elements), &mut render_state);
        assert_eq!(dom_nodes_parse::<i32>(&dom), [5, 6, 7, 8, 9, 0, 1, 2, 3, 4]);
    }
}

#[cfg(feature = "csr")]
#[test]
fn append() {
    let mut dom = RendererWithRoot::new();
    let mut render_state = Default::default();

    assert!(dom.nodes().is_empty());

    {
        let elements = (0..5).map(|n| Keyed(n, n));
        dom.unpinned_render_update(Elements(elements), &mut render_state);
        assert_eq!(dom_nodes_parse::<i32>(&dom), [0, 1, 2, 3, 4]);
    }

    {
        let elements = (0..6).map(|n| Keyed(n, n));
        dom.unpinned_render_update(Elements(elements), &mut render_state);
        assert_eq!(dom_nodes_parse::<i32>(&dom), [0, 1, 2, 3, 4, 5]);
    }

    {
        let elements = (0..9).map(|n| Keyed(n, n));
        dom.unpinned_render_update(Elements(elements), &mut render_state);
        assert_eq!(dom_nodes_parse::<i32>(&dom), [0, 1, 2, 3, 4, 5, 6, 7, 8]);
    }
}

#[cfg(feature = "csr")]
#[test]
fn remove_one_by_one() {
    let mut dom = RendererWithRoot::new();
    let mut render_state = Default::default();

    assert!(dom.nodes().is_empty());

    {
        let elements = (0..5).map(|n| Keyed(n, n));
        dom.unpinned_render_update(Elements(elements), &mut render_state);
        assert_eq!(dom_nodes_parse::<i32>(&dom), [0, 1, 2, 3, 4]);
    }

    {
        let elements = [0, 1, 2, 4].map(|n| Keyed(n, n));
        dom.unpinned_render_update(Elements(elements), &mut render_state);
        assert_eq!(dom_nodes_parse::<i32>(&dom), [0, 1, 2, 4]);
    }

    {
        let elements = [0, 1, 4].map(|n| Keyed(n, n));
        dom.unpinned_render_update(Elements(elements), &mut render_state);
        assert_eq!(dom_nodes_parse::<i32>(&dom), [0, 1, 4]);
    }
}

#[cfg(feature = "csr")]
#[test]
fn swap() {
    let mut dom = RendererWithRoot::new();
    let mut render_state = Default::default();

    assert!(dom.nodes().is_empty());

    {
        let elements = [0, 1].map(|n| Keyed(n, n));
        dom.unpinned_render_update(Elements(elements), &mut render_state);
        assert_eq!(dom_nodes_parse::<i32>(&dom), [0, 1]);
    }

    {
        let elements = [1, 0].map(|n| Keyed(n, n));
        dom.unpinned_render_update(Elements(elements), &mut render_state);
        assert_eq!(dom_nodes_parse::<i32>(&dom), [1, 0]);
    }

    {
        let elements = [0, 1].map(|n| Keyed(n, n));
        dom.unpinned_render_update(Elements(elements), &mut render_state);
        assert_eq!(dom_nodes_parse::<i32>(&dom), [0, 1]);
    }
}
