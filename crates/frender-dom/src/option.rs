use wasm_bindgen::JsCast;

use crate::render::{Dom, NextNodePosition};

#[inline]
pub fn map_or_insert_with_ctx<T, C, R>(
    this: &mut Option<T>,
    ctx: C,
    map: impl FnOnce(&mut T, C) -> R,
    insert: impl FnOnce(C) -> T,
) -> (Option<R>, &mut T) {
    match this {
        Some(this) => (Some(map(this, ctx)), this),
        None => {
            let this = this.insert(insert(ctx));
            (None, this)
        }
    }
}

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
