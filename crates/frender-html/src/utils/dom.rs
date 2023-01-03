use wasm_bindgen::JsCast;

use frender_dom::{Dom, NextNodePosition};

#[inline]
pub fn insert_element_and_update<E: JsCast + AsRef<web_sys::Element>>(
    this: &mut Option<E>,
    ctx: &mut Dom,
    tag: &str,
    update: impl FnOnce(&mut E, &mut Dom),
) {
    let mut new_created = false;
    let element = this.get_or_insert_with(|| {
        new_created = true;
        ctx.document
            .create_element(tag)
            .unwrap()
            .dyn_into::<E>()
            .unwrap()
    });
    let node: &web_sys::Element = element.as_ref();
    let node = node.clone();

    ctx.with_new_position(NextNodePosition::FirstChildOf(node.clone()), |ctx| {
        update(element, ctx)
    });

    if new_created {
        ctx.next_node_position.add_node(node.into());
    } else {
        ctx.next_node_position.set_as_insert_after(node.into());
    }
}
