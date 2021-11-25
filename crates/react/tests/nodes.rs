use wasm_bindgen_test::*;

use react::Nodes;

#[wasm_bindgen_test]
fn vec_impl_nodes() {
    let nodes = vec![1];
    let arr = nodes.as_react_node_array_js();

    assert_eq!(arr.length(), 1);
    assert_eq!(arr.get(0), 1);
}

#[wasm_bindgen_test]
fn array_impl_nodes() {
    let nodes = ["a", "b", "c"];
    let arr = nodes.as_react_node_array_js();

    assert_eq!(arr.length(), 3);
    assert_eq!(arr.get(0), "a");
    assert_eq!(arr.get(1), "b");
    assert_eq!(arr.get(2), "c");
}

#[wasm_bindgen_test]
fn slice_impl_nodes() {
    let nodes = &[true, false][..];
    let arr = nodes.as_react_node_array_js();

    assert_eq!(arr.length(), 2);
    assert_eq!(arr.get(0), true);
    assert_eq!(arr.get(1), false);
}

#[wasm_bindgen_test]
fn tuple_impl_nodes() {
    let nodes = (false, 1, 2.0, "3", "4".to_string());
    let arr = nodes.as_react_node_array_js();

    assert_eq!(arr.length(), 5);
    assert_eq!(arr.get(0), false);
    assert_eq!(arr.get(1), 1);
    assert_eq!(arr.get(2), 2.0);
    assert_eq!(arr.get(3), "3");
    assert_eq!(arr.get(4), "4");
}

#[wasm_bindgen_test]
fn collect_vec_nodes() {
    let nodes = vec![1, 2].into_iter().map(|v| v + 1).collect::<Vec<_>>();
    let arr = nodes.as_react_node_array_js();

    assert_eq!(arr.length(), 2);
    assert_eq!(arr.get(0), 2);
    assert_eq!(arr.get(1), 3);
}
