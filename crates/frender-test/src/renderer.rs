use std::borrow::Cow;

use frender_html::{
    dom::{
        behaviors::ElementWithChildren as _,
        render::{Render, RenderTextFrom, RenderWithContext},
        ProvideRenderContext,
    },
    RenderHtml, RenderStateKindPinned, RenderStateKindUnpinned,
};

use crate::{
    element::{CursorPlaceholder, Node},
    text::Text,
};

#[non_exhaustive]
pub struct Renderer {}

pub struct RendererWithRoot {
    renderer: Renderer,
    root: crate::element::Element,
}

impl RendererWithRoot {
    pub fn new() -> Self {
        let root = crate::element::Element::new_dummy();
        Self {
            renderer: Renderer {},
            root,
        }
    }

    pub fn nodes(&self) -> Vec<Node> {
        self.root.children()
    }

    pub fn render_update<E: frender_html::Element>(
        &mut self,
        element: E,
        render_state: std::pin::Pin<
            &mut <E::RenderStateKind as RenderStateKindPinned>::RenderState<Renderer>,
        >,
    ) {
        self.provide_render_context(|render_context| {
            frender_html::Element::render_update(element, render_context, render_state)
        })
    }

    pub fn unpinned_render_update<E: frender_html::Element>(
        &mut self,
        element: E,
        render_state: &mut <E::RenderStateKind as RenderStateKindUnpinned>::UnpinnedRenderState<
            Renderer,
        >,
    ) {
        self.provide_render_context(|render_context| {
            frender_html::Element::unpinned_render_update(element, render_context, render_state)
        })
    }
}

impl ProvideRenderContext for RendererWithRoot {
    type Renderer = Renderer;

    fn provide_render_context<Res>(
        &mut self,
        f: impl FnOnce(&mut <Self::Renderer as RenderWithContext>::RenderContext<'_>) -> Res,
    ) -> Res {
        self.root
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
            self.move_cursor_after_node(node.into_owned())
        }
    }

    pub(crate) fn cursor_is_at(&self, f: impl FnOnce(Node) -> bool) -> bool {
        // TODO: check is sibling
        self.cursor.1 || self.current_node().map_or(false, f)
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

impl<S: ?Sized + ToString> RenderTextFrom<Text, S> for Renderer {
    fn render_text_from(&mut self, v: &S) -> Text {
        Text::new(v.to_string())
    }

    fn update_text_from(&mut self, text: &mut Text, v: &S) {
        text.update(v.to_string())
    }
}

macro_rules! html_elements {
    (
        |$tag:pat_param| -> $Element:ty { $e:expr },
        $($name:ident),* $(,)?
    ) => {
        $(
            type $name = $Element;
            fn $name(&mut self) -> Self::$name {
                let $tag = <frender_html::html::tags::$name as frender_html::dom::component::HasIntrinsicComponentTag>::INTRINSIC_COMPONENT_TAG;
                $e
            }
        )*
    };
}

pub struct Cursor(crate::element::Cursor, bool);

impl Cursor {
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

impl frender_html::dom::render::RenderContext for RenderContext<'_> {
    type Renderer = Renderer;

    fn map_mut_render_context<Res>(
        &mut self,
        f: impl FnOnce(&mut <Self::Renderer as RenderWithContext>::RenderContext<'_>) -> Res,
    ) -> Res {
        f(self)
    }

    fn renderer_mut(&mut self) -> &mut Self::Renderer {
        &mut self.renderer
    }

    fn log_cursor(&mut self) {
        eprintln!("{:?} (skipped={:?})", self.cursor.0, self.cursor.1)
    }

    fn mark_cursor_skipped(&mut self) {
        self.cursor.1 = true
    }
}

impl RenderWithContext for Renderer {
    type RenderContext<'a> = RenderContext<'a>;
}

impl RenderHtml for Renderer {
    type Text = Text;

    html_elements!(
        |tag| -> frender_html::ElementProxyAttrs<crate::element::Element> {
            frender_html::ElementProxyAttrs(crate::element::Element::new_with_tag(tag))
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
