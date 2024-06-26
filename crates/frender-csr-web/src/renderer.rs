use std::borrow::Cow;

use frender_html::{
    dom::{
        csr::web::{Node, Renderer as _},
        render::{Render, RenderWithContext, RenderWithCursor},
    },
    RenderHtml,
};
use wasm_bindgen::UnwrapThrowExt;

mod text;

#[derive(Debug, Clone)]
enum NextNodePosition {
    FirstChildOf(web_sys::Element),
    InsertAfter(web_sys::Node),
}

pub struct Renderer {
    document: web_sys::Document,
    next_node_position: NextNodePosition,
    cursor_skipped: bool,
}

impl Renderer {
    pub fn new(document: web_sys::Document, root_parent: web_sys::Element) -> Self {
        Self {
            document,
            next_node_position: NextNodePosition::FirstChildOf(root_parent),
            cursor_skipped: false,
        }
    }
}

macro_rules! html_elements {
    ($($tag:ident : $ty:ident),* $(,)?) => {$(
        type $tag = Node<web_sys::$ty>;

        fn $tag(&mut self) -> Self::$tag {
            use wasm_bindgen::{JsCast, UnwrapThrowExt};

            let element = self
                .document
                .create_element(<frender_html::html::tags::$tag as frender_html::dom::component::HasIntrinsicComponentTag>::INTRINSIC_COMPONENT_TAG)
                .unwrap_throw();
            Node(element.unchecked_into())
        }
    )*};
}

pub struct Cursor(NextNodePosition, bool);

pub struct CursorPlaceholder(web_sys::Comment);

impl RenderWithCursor for Renderer {
    type Cursor = Cursor;

    fn cursor(&self) -> Self::Cursor {
        Cursor(self.next_node_position.clone(), self.cursor_skipped)
    }

    fn set_cursor(&mut self, cursor: Self::Cursor) {
        self.next_node_position = cursor.0;
        self.cursor_skipped = cursor.1;
    }

    fn cursor_skipped(&self) -> bool {
        self.cursor_skipped
    }

    fn set_cursor_skipped(&mut self, cursor_skipped: bool) {
        self.cursor_skipped = cursor_skipped;
    }

    fn set_cursor_by_ref(&mut self, cursor: &Self::Cursor) {
        self.next_node_position = cursor.0.clone();
        self.cursor_skipped = cursor.1;
    }

    fn cursor_is_same_as(&self, other: &Self::Cursor) -> bool {
        self.cursor_skipped == other.1
            && match (&self.next_node_position, &other.0) {
                (NextNodePosition::FirstChildOf(a), NextNodePosition::FirstChildOf(b)) => a == b,
                (NextNodePosition::InsertAfter(a), NextNodePosition::InsertAfter(b)) => a == b,
                _ => false,
            }
    }

    fn log_cursor(&mut self) {
        let (kind, node, cur) = match &self.next_node_position {
            NextNodePosition::FirstChildOf(node) => {
                ("FirstChildOf", node.as_ref(), node.first_child())
            }
            NextNodePosition::InsertAfter(node) => {
                ("InsertAfter", node.as_ref(), node.next_sibling())
            }
        };

        web_sys::console::log_5(
            &"cursor=".into(),
            &kind.into(),
            node,
            &"=".into(),
            &cur.into(),
        );
    }

    type CursorPlaceholder = CursorPlaceholder;

    fn cursor_placeholder_render(&mut self) -> Self::CursorPlaceholder {
        let node = self.document.create_comment("");

        self.readd_node(&node, true);
        CursorPlaceholder(node)
    }

    fn cursor_placeholder_force_reposition(&mut self, cp: &mut Self::CursorPlaceholder) {
        self.readd_node(&cp.0, true);
    }

    fn cursor_placeholder_unmount(&mut self, cp: &mut Self::CursorPlaceholder) {
        cp.0.remove()
    }

    fn move_cursor_after_placeholder(&mut self, place_holder: &mut Self::CursorPlaceholder) {
        self.move_cursor_after_node(&place_holder.0)
    }
}

impl Render for Renderer {
    fn log(&mut self, v: &str) {
        web_sys::console::log_1(&v.into())
    }
}

impl RenderWithContext for Renderer {
    type RenderContext<'a> = frender_html::dom::csr::web::RenderContext<'a, Self>;
}

impl RenderHtml for Renderer {
    type Text = Node<web_sys::Text>;

    html_elements!(
        abbr: HtmlElement,
        address: HtmlElement,
        article: HtmlElement,
        aside: HtmlElement,
        b: HtmlElement,
        bdi: HtmlElement,
        bdo: HtmlElement,
        cite: HtmlElement,
        code: HtmlElement,
        datalist: HtmlDataListElement,
        dd: HtmlElement,
        dfn: HtmlElement,
        div: HtmlDivElement,
        dl: HtmlDListElement,
        dt: HtmlElement,
        em: HtmlElement,
        figcaption: HtmlElement,
        figure: HtmlElement,
        footer: HtmlElement,
        h1: HtmlHeadingElement,
        h2: HtmlHeadingElement,
        h3: HtmlHeadingElement,
        h4: HtmlHeadingElement,
        h5: HtmlHeadingElement,
        h6: HtmlHeadingElement,
        head: HtmlHeadElement,
        header: HtmlElement,
        hgroup: HtmlElement,
        hr: HtmlHrElement,
        i: HtmlElement,
        kbd: HtmlElement,
        legend: HtmlLegendElement,
        main: HtmlElement,
        mark: HtmlElement,
        menu: HtmlMenuElement,
        nav: HtmlElement,
        noscript: HtmlElement,
        p: HtmlParagraphElement,
        picture: HtmlPictureElement,
        pre: HtmlPreElement, // TODO: non-standard attributes
        rp: HtmlElement,
        rt: HtmlElement,
        ruby: HtmlElement,
        s: HtmlElement,
        samp: HtmlElement,
        section: HtmlElement,
        small: HtmlElement,
        span: HtmlSpanElement,
        strong: HtmlElement,
        sub: HtmlElement,
        summary: HtmlElement,
        sup: HtmlElement,
        template: HtmlTemplateElement,
        title: HtmlTitleElement,
        u: HtmlElement,
        var: HtmlElement,
        wbr: HtmlElement,
        a: HtmlAnchorElement,
        area: HtmlAreaElement,
        audio: HtmlAudioElement,
        video: HtmlVideoElement,
        base: HtmlBaseElement,
        blockquote: HtmlQuoteElement,
        q: HtmlQuoteElement,
        body: HtmlBodyElement,
        br: HtmlBrElement,
        button: HtmlButtonElement,
        canvas: HtmlCanvasElement,
        caption: HtmlTableCaptionElement,
        data: HtmlDataElement,
        del: HtmlModElement,
        ins: HtmlModElement,
        details: HtmlDetailsElement,
        dialog: HtmlDialogElement,
        embed: HtmlEmbedElement,
        fieldset: HtmlFieldSetElement,
        form: HtmlFormElement,
        html: HtmlHtmlElement,
        iframe: HtmlIFrameElement,
        img: HtmlImageElement,
        input: HtmlInputElement,
        label: HtmlLabelElement,
        li: HtmlLiElement,
        link: HtmlLinkElement,
        map: HtmlMapElement,
        meta: HtmlMetaElement,
        meter: HtmlMeterElement,
        object: HtmlObjectElement,
        ol: HtmlOListElement,
        optgroup: HtmlOptGroupElement,
        option: HtmlOptionElement,
        output: HtmlOutputElement,
        progress: HtmlProgressElement,
        script: HtmlScriptElement,
        select: HtmlSelectElement,
        slot: HtmlSlotElement,
        source: HtmlSourceElement,
        style: HtmlStyleElement,
        table: HtmlTableElement,
        tbody: HtmlTableSectionElement,
        tfoot: HtmlTableSectionElement,
        thead: HtmlTableSectionElement,
        tr: HtmlTableRowElement,
        col: HtmlTableColElement,
        colgroup: HtmlTableColElement,
        td: HtmlTableCellElement,
        th: HtmlTableCellElement,
        textarea: HtmlTextAreaElement,
        time: HtmlTimeElement,
        track: HtmlTrackElement,
        ul: HtmlUListElement,
    );
}

impl frender_html::dom::csr::web::Renderer for Renderer {
    fn document(&self) -> Cow<web_sys::Document> {
        Cow::Borrowed(&self.document)
    }

    fn cursor_is_at_node(&self, node: &web_sys::Node) -> bool {
        match &self.next_node_position {
            NextNodePosition::FirstChildOf(parent) => parent.first_child(),
            NextNodePosition::InsertAfter(previous) => previous.next_sibling(),
        }
        .map_or(false, |c| *node == c)
    }

    fn move_cursor_after_node(&mut self, node: &web_sys::Node) {
        let node = node.clone();
        self.next_node_position = NextNodePosition::InsertAfter(node);
        self.cursor_skipped = false;
    }

    fn readd_node(&mut self, node: &web_sys::Node, force_reposition: bool) {
        // web_sys::console::log_3(&"readd_node".into(), node, &force_reposition.into());
        if force_reposition {
            match &self.next_node_position {
                NextNodePosition::FirstChildOf(parent) => {
                    // web_sys::console::log_2(&"FirstChildOf".into(), parent);

                    parent.prepend_with_node_1(node).unwrap_throw()
                }
                NextNodePosition::InsertAfter(pre) => {
                    // web_sys::console::log_2(&"InsertAfter".into(), pre);

                    pre.parent_node()
                        .expect_throw("the previous node should have a parent node")
                        .insert_before(node, pre.next_sibling().as_ref())
                        .unwrap_throw();
                }
            }
        }

        self.next_node_position = NextNodePosition::InsertAfter(node.clone());
        self.cursor_skipped = false;
    }

    fn remove_node(&mut self, node: &web_sys::Node) {
        use wasm_bindgen::prelude::*;

        #[wasm_bindgen]
        extern "C" {
            type Removable;

            #[wasm_bindgen(method, structural)]
            pub fn remove(this: &Removable);
        }

        node.unchecked_ref::<Removable>().remove()
    }

    fn move_cursor_at_the_first_child_of_element(&mut self, element: &web_sys::Element) {
        self.next_node_position = NextNodePosition::FirstChildOf(element.clone());
        self.cursor_skipped = false;
    }
}
