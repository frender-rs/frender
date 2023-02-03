use wasm_bindgen::JsCast;

use frender_dom::{Dom, NextNodePosition};

use crate::props::ElementAndMounted;

pub fn insert_element_and_update_with_tag<E: JsCast + AsRef<web_sys::Element>>(
    this: &mut Option<ElementAndMounted<E>>,
    ctx: &mut Dom,
    tag: &str,
    update: impl FnOnce(&mut E, &mut Dom),
) {
    insert_element_and_update(
        this,
        ctx,
        |ctx| {
            ctx.document
                .create_element(tag)
                .unwrap()
                .dyn_into::<E>()
                .unwrap()
        },
        update,
    )
}

pub fn insert_element_and_update<E: JsCast + AsRef<web_sys::Element>>(
    this: &mut Option<ElementAndMounted<E>>,
    ctx: &mut Dom,
    create: impl FnOnce(&mut Dom) -> E,
    update: impl FnOnce(&mut E, &mut Dom),
) {
    let ElementAndMounted { element, mounted } = this.get_or_insert_with(|| ElementAndMounted {
        element: create(ctx),
        mounted: false,
    });
    let node: &web_sys::Element = element.as_ref();
    let node = node.clone();

    ctx.with_new_position(NextNodePosition::FirstChildOf(node.clone()), |ctx| {
        update(element, ctx)
    });

    if *mounted {
        ctx.next_node_position.set_as_insert_after(node.into());
    } else {
        ctx.next_node_position.add_node(node.into());
        *mounted = true;
    }
}
