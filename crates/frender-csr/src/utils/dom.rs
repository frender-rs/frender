use std::borrow::Cow;

use wasm_bindgen::JsCast;

use crate::{elements::intrinsic::ElementAndMounted, CsrContext, NextNodePosition};

// TODO: inline
pub fn initialize_element_with_tag<E: JsCast + AsRef<web_sys::Element>, R>(
    ctx: &mut CsrContext,
    tag: &str,
    update: impl FnOnce(&E, &mut CsrContext) -> R,
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
    ctx: &mut CsrContext,
    create: impl FnOnce(&mut CsrContext) -> E,
    update: impl FnOnce(&E, &mut CsrContext) -> R,
) -> (ElementAndMounted<E>, R) {
    let element = create(ctx);
    let node: &web_sys::Element = element.as_ref();

    let ret = {
        let mut ctx = ctx.as_borrowed();
        ctx.next_node_position = NextNodePosition::FirstChildOf(Cow::Borrowed(node));
        update(&element, &mut ctx)
    };
    ctx.next_node_position
        .add_node(Cow::Owned(node.clone().into()));

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
    ctx: &mut CsrContext,
    update: impl FnOnce(&E, &mut CsrContext),
) {
    let ElementAndMounted { element, mounted } = this;
    let node: &web_sys::Element = element.as_ref();

    {
        let mut ctx = ctx.as_borrowed();
        ctx.next_node_position = NextNodePosition::FirstChildOf(Cow::Borrowed(node));
        update(element, &mut ctx)
    };

    let node: web_sys::Node = node.clone().into();
    if *mounted {
        debug_assert!(node.parent_node().is_some());
        ctx.next_node_position.set_as_insert_after(Cow::Owned(node));
    } else {
        debug_assert!(node.parent_node().is_none());
        ctx.next_node_position.add_node(Cow::Owned(node));
        *mounted = true;
    }
}
