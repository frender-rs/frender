use wasm_bindgen::JsCast;

use crate::{elements::intrinsic::ElementAndMounted, Dom, NextNodePosition};

// TODO: inline
pub fn initialize_element_with_tag<E: JsCast + AsRef<web_sys::Element>, R>(
    ctx: &mut Dom,
    tag: &str,
    update: impl FnOnce(&mut E, &mut Dom) -> R,
) -> (ElementAndMounted<E>, R) {
    initialize_element(
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

pub fn initialize_element<E: JsCast + AsRef<web_sys::Element>, R>(
    ctx: &mut Dom,
    create: impl FnOnce(&mut Dom) -> E,
    update: impl FnOnce(&mut E, &mut Dom) -> R,
) -> (ElementAndMounted<E>, R) {
    let mut element = create(ctx);
    let node: &web_sys::Element = element.as_ref();
    let node = node.clone();

    let ret = ctx.with_new_position(NextNodePosition::FirstChildOf(node.clone()), |ctx| {
        update(&mut element, ctx)
    });
    ctx.next_node_position.add_node(node.into());

    (
        ElementAndMounted {
            element,
            mounted: true,
        },
        ret,
    )
}

pub fn update_element<E: JsCast + AsRef<web_sys::Element>>(
    this: &mut ElementAndMounted<E>,
    ctx: &mut Dom,
    update: impl FnOnce(&mut E, &mut Dom),
) {
    let ElementAndMounted { element, mounted } = this;
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
