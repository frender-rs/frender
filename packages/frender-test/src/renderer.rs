use std::borrow::Cow;

use frender_html::{
    csr::experimental,
    dom::csr::{
        behaviors::ElementWithChildren as _,
        render::{Render, RenderWithContext},
        ProvideRenderContext,
    },
    ElementProxyAttrs, RenderHtml,
};

use crate::element::{CursorPlaceholder, Element, Node, UnmountedElement};

mod text;

#[non_exhaustive]
pub struct Renderer {}

pub struct Root(crate::element::Element);

impl Root {
    pub fn clone_nodes(&self) -> Vec<Node> {
        self.0.children()
    }
}

struct RendererWithRoot {
    renderer: Renderer,
    root: Root,
}

impl RendererWithRoot {
    pub fn provide_render_context_and_renderer<Out>(
        mut self,
        f: impl FnOnce(&mut RenderContext) -> Out,
    ) -> (Renderer, Root, Out) {
        let out = self
            .root
            .0
            .with_render_context_at_first_child_of_self(&mut self.renderer, f);

        (self.renderer, self.root, out)
    }
    pub fn new() -> Self {
        let root = crate::element::Element::new_dummy();
        Self {
            renderer: Renderer {},
            root: Root(root),
        }
    }

    pub fn clone_nodes(&self) -> Vec<Node> {
        self.root.clone_nodes()
    }
}

impl ProvideRenderContext for RendererWithRoot {
    type Renderer = Renderer;

    fn provide_render_context<Res>(
        &mut self,
        f: impl FnOnce(&mut <Self::Renderer as RenderWithContext>::RenderContext<'_>) -> Res,
    ) -> Res {
        self.root
            .0
            .with_render_context_at_first_child_of_self(&mut self.renderer, f)
    }

    fn renderer_mut(&mut self) -> &mut Self::Renderer {
        &mut self.renderer
    }
}

impl RenderContext<'_> {
    pub(crate) fn move_cursor_after_node(&mut self, node: Node) {
        *self.cursor = Cursor(
            crate::element::Cursor::After {
                node,
                children_index_hint: 0, // TODO: optimize
            },
            false,
        )
    }

    fn readd_node_force_reposition(&mut self, node: Cow<Node>) {
        let node = if let Some(parent) = node.parent() {
            let parent = parent
                .upgrade()
                .expect("node parent should not have been dropped");
            parent.remove_child(&node)
        } else {
            node.into_owned()
        };

        match &mut self.cursor.0 {
            crate::element::Cursor::FirstChildOf(parent) => parent.prepend_child(node.clone()),
            crate::element::Cursor::After {
                node: after,
                children_index_hint: _,
            } => after
                .parent()
                .expect("node should have a parent")
                .upgrade()
                .expect("node parent should not have been dropped")
                .insert_child_after(node.clone(), after),
        }

        self.move_cursor_after_node(node);
    }

    pub(crate) fn readd_node(&mut self, node: Cow<Node>, force_reposition: bool) {
        if force_reposition {
            self.readd_node_force_reposition(node)
        } else {
            self.assert_cursor_is_at_node(&node);
            self.move_cursor_after_node(node.into_owned())
        }
    }

    pub(crate) fn cursor_is_at(&self, f: impl FnOnce(Node) -> bool) -> bool {
        // TODO: check is sibling
        self.cursor.1 || self.current_node().map_or(false, f)
    }

    fn assert_cursor_is_at_node(&self, node: &Node) {
        assert!(
            node.parent()
                .unwrap()
                .upgrade()
                .unwrap()
                .try_position_of_child(&node)
                .is_some(),
            "node should be mounted as a child: {:?}",
            node
        );

        if !self.cursor_is_at(|c| c.is_same_node(node)) {
            panic!(
                "Cursor should be at {:?}\n\nBut cursor is {:?},\n\nwhich is at {:?}",
                node,
                self.cursor,
                self.current_node(),
            )
        }
    }
}

impl Renderer {
    pub(crate) fn with_render_context_at_first_child_of_element<Res>(
        &mut self,
        el: &mut crate::element::Element,
        f: impl FnOnce(&mut RenderContext<'_>) -> Res,
    ) -> Res {
        f(&mut RenderContext {
            renderer: self,
            cursor: &mut Cursor(crate::element::Cursor::FirstChildOf(el.clone()), false),
        })
    }

    pub(crate) fn with_render_context_after_node<Res>(
        &mut self,
        node: Node,
        f: impl FnOnce(&mut RenderContext<'_>) -> Res,
    ) -> Res {
        assert!(
            node.parent()
                .unwrap()
                .upgrade()
                .unwrap()
                .try_position_of_child(&node)
                .is_some(),
            "node should be mounted: {:?}",
            node
        );

        f(&mut RenderContext {
            renderer: self,
            cursor: &mut Cursor(
                crate::element::Cursor::After {
                    node,
                    children_index_hint: 0, // TODO: perf
                },
                false,
            ),
        })
    }
}

macro_rules! html_elements {
    (
        |$tag:pat_param| -> $Element:ty { $e:expr },
        $($name:ident),* $(,)?
    ) => {
        $(
            type $name = $Element;
            fn $name(&mut self) -> ElementProxyAttrs<UnmountedElement> {
                let $tag = const { stringify!($name) };
                $e
            }
        )*
    };
}

pub struct Cursor(crate::element::Cursor, bool);

impl std::fmt::Debug for Cursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cursor({:?}, skipped={:?})", self.0, self.1)
    }
}

impl Cursor {
    fn cloned(&self) -> Self {
        Self(self.0.clone(), self.1)
    }
    fn is_same_cursor(&self, other: &Self) -> bool {
        self.1 == other.1
            && match (&self.0, &other.0) {
                (
                    crate::element::Cursor::FirstChildOf(a),
                    crate::element::Cursor::FirstChildOf(b),
                ) => a.is_same_element(b),

                (
                    crate::element::Cursor::After {
                        node: a,
                        children_index_hint: _,
                    },
                    crate::element::Cursor::After {
                        node: b,
                        children_index_hint: _,
                    },
                ) => a.is_same_node(b),

                _ => false,
            }
    }
}

impl Render for Renderer {
    type CursorPlaceholder = CursorPlaceholder;

    fn log(&mut self, v: &str) {
        eprintln!("{v}");
    }
}

pub struct RenderContext<'a> {
    renderer: &'a mut Renderer,
    cursor: &'a mut Cursor,
}
impl RenderContext<'_> {
    pub(crate) fn current_node(&self) -> Option<Node> {
        self.cursor.0.current_node()
    }
}

impl frender_html::csr::render::RenderContext for RenderContext<'_> {
    type Renderer = Renderer;

    fn map_mut_render_context<Res>(
        &mut self,
        f: impl FnOnce(&mut <Self::Renderer as RenderWithContext>::RenderContext<'_>) -> Res,
    ) -> Res {
        f(self)
    }

    fn map_mut_cloned_render_context<Res>(
        &mut self,
        f: impl FnOnce(&mut <Self::Renderer as RenderWithContext>::RenderContext<'_>) -> Res,
    ) -> Res {
        f(&mut RenderContext {
            renderer: self.renderer,
            cursor: &mut self.cursor.cloned(),
        })
    }

    fn map_mut_unrendered_render_context_and_then_reposition<Res>(
        &mut self,
        f: impl FnOnce(&mut <Self::Renderer as RenderWithContext>::RenderContext<'_>) -> Res,
    ) -> Res {
        todo!()
    }

    fn renderer_mut(&mut self) -> &mut Self::Renderer {
        &mut self.renderer
    }

    fn log_cursor(&mut self) {
        eprintln!("{:?}", self.cursor)
    }

    fn mark_cursor_skipped(&mut self) {
        self.cursor.1 = true
    }
}

impl RenderWithContext for Renderer {
    type RenderContext<'a> = RenderContext<'a>;
}

impl RenderHtml for Renderer {
    html_elements!(
        |tag| -> ElementProxyAttrs<Element> {
            ElementProxyAttrs(UnmountedElement::new_with_tag(tag))
        },
        abbr,
        address,
        article,
        aside,
        b,
        bdi,
        bdo,
        cite,
        code,
        datalist,
        dd,
        dfn,
        div,
        dl,
        dt,
        em,
        figcaption,
        figure,
        footer,
        h1,
        h2,
        h3,
        h4,
        h5,
        h6,
        head,
        header,
        hgroup,
        hr,
        i,
        kbd,
        legend,
        main,
        mark,
        menu,
        nav,
        noscript,
        p,
        picture,
        pre,
        rp,
        rt,
        ruby,
        s,
        samp,
        section,
        small,
        span,
        strong,
        sub,
        summary,
        sup,
        template,
        title,
        u,
        var,
        wbr,
        a,
        area,
        audio,
        video,
        base,
        blockquote,
        q,
        body,
        br,
        button,
        canvas,
        caption,
        data,
        del,
        ins,
        details,
        dialog,
        embed,
        fieldset,
        form,
        html,
        iframe,
        img,
        input,
        label,
        li,
        link,
        map,
        meta,
        meter,
        object,
        ol,
        optgroup,
        option,
        output,
        progress,
        script,
        select,
        slot,
        source,
        style,
        table,
        tbody,
        tfoot,
        thead,
        tr,
        col,
        colgroup,
        td,
        th,
        textarea,
        time,
        track,
        ul,
    );
}

pub fn unpinned_render_init<E: frender_html::CsrElement>(
    element: E,
) -> (
    Renderer,
    Root,
    (
        experimental::UnpinnedStateOfKind<Renderer, E::RenderStateKind>,
        experimental::UnpinnedUiHandleOfKind<Renderer, E::RenderStateKind>,
    ),
) {
    let dom = RendererWithRoot::new();
    assert!(dom.clone_nodes().is_empty());

    dom.provide_render_context_and_renderer(|render_context| {
        element.unpinned_render_init(render_context)
    })
}
