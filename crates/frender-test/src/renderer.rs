use std::borrow::Cow;

use frender_html::{
    dom::render::{Render, RenderTextFrom, RenderWithContext, RenderWithCursor},
    RenderHtml,
};

use crate::{
    element::{CursorPlaceholder, Node},
    text::Text,
};

pub struct VirtualDom {
    renderer: Renderer,
}

impl VirtualDom {
    pub fn new() -> Self {
        Self {
            renderer: Renderer::new(),
        }
    }

    pub fn nodes(&self) -> Vec<Node> {
        self.renderer.root.children()
    }

    pub fn start_render_context(&mut self) -> &mut Renderer {
        let renderer = &mut self.renderer;

        if !matches!(&renderer.cursor, crate::element::Cursor::FirstChildOf(parent) if parent.is_same_element(&renderer.root))
        {
            renderer.cursor = crate::element::Cursor::FirstChildOf(renderer.root.clone());
        }

        renderer
    }
}

pub struct Renderer {
    root: crate::element::Element,
    pub(crate) cursor: crate::element::Cursor,
    pub(crate) cursor_skipped: bool,
}

impl Renderer {
    fn new() -> Self {
        let root = crate::element::Element::new_dummy();
        Self {
            cursor: crate::element::Cursor::FirstChildOf(root.clone()),
            root,
            cursor_skipped: false,
        }
    }

    pub(crate) fn move_cursor_after_node(&mut self, node: Node) {
        self.cursor = crate::element::Cursor::After {
            node,
            children_index_hint: 0, // TODO: optimize
        };
        self.cursor_skipped = false;
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

        match &mut self.cursor {
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

    pub(crate) fn with_render_context_at_first_child_of_element<R>(
        &mut self,
        el: &mut crate::element::Element,
        f: impl FnOnce(RenderContext<'_>) -> R,
    ) -> R {
        f(RenderContext {
            cursor: &mut Cursor(crate::element::Cursor::FirstChildOf(el.clone()), false),
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

impl RenderWithCursor for Renderer {
    type Cursor = Cursor;

    fn cursor(&self) -> Self::Cursor {
        Cursor(self.cursor.cloned_cursor(), self.cursor_skipped)
    }

    fn set_cursor(&mut self, cursor: Self::Cursor) {
        self.cursor = cursor.0;
        self.cursor_skipped = cursor.1;
    }

    fn cursor_skipped(&self) -> bool {
        self.cursor_skipped
    }

    fn set_cursor_skipped(&mut self, cursor_skipped: bool) {
        self.cursor_skipped = cursor_skipped
    }

    fn set_cursor_by_ref(&mut self, cursor: &Self::Cursor) {
        self.cursor = cursor.0.cloned_cursor();
        self.cursor_skipped = cursor.1;
    }

    fn cursor_is_same_as(&self, other: &Self::Cursor) -> bool {
        self.cursor_skipped == other.1
            && match (&self.cursor, &other.0) {
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

    fn log_cursor(&mut self) {
        eprintln!("{:?}", self.cursor)
    }

    type CursorPlaceholder = CursorPlaceholder;

    fn cursor_placeholder_render(&mut self) -> Self::CursorPlaceholder {
        let cp = CursorPlaceholder::new();
        self.readd_node_force_reposition(Cow::Owned(Node::CursorPlaceholder(cp.clone())));
        cp
    }

    fn cursor_placeholder_force_reposition(&mut self, cp: &mut Self::CursorPlaceholder) {
        self.readd_node_force_reposition(Cow::Owned(Node::CursorPlaceholder(cp.clone())));
    }

    fn cursor_placeholder_unmount(&mut self, placeholder: &mut Self::CursorPlaceholder) {
        placeholder
            .parent()
            .expect("CursorPlaceholder should have a parent")
            .upgrade()
            .expect("CursorPlaceholder's parent should not have been dropped")
            .remove_child(&Node::CursorPlaceholder(placeholder.clone()));
    }

    fn move_cursor_after_placeholder(&mut self, placeholder: &mut Self::CursorPlaceholder) {
        self.move_cursor_after_node(Node::CursorPlaceholder(placeholder.clone()))
    }
}

impl Render for Renderer {
    fn log(&mut self, v: &str) {
        eprintln!("{v}");
    }
}

pub struct RenderContext<'a> {
    cursor: &'a mut Cursor,
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
