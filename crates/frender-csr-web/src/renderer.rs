use std::borrow::Cow;

use frender_html::{dom::csr::web::Node, RenderHtml};
use wasm_bindgen::UnwrapThrowExt;

mod text;

#[derive(Debug, Clone)]
enum NextNodePosition<'a> {
    FirstChildOf(Cow<'a, web_sys::Element>),
    InsertAfter(Cow<'a, web_sys::Node>),
}

pub struct Renderer<'a> {
    document: &'a web_sys::Document,
    next_node_position: NextNodePosition<'a>,
}

impl<'a> Renderer<'a> {
    pub fn new(document: &'a web_sys::Document, root_parent: web_sys::Element) -> Self {
        Self {
            document,
            next_node_position: NextNodePosition::FirstChildOf(Cow::Owned(root_parent)),
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

impl RenderHtml for Renderer<'_> {
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

impl<'r> frender_html::dom::csr::web::Renderer for Renderer<'r> {
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
        self.next_node_position = NextNodePosition::InsertAfter(Cow::Owned(node));
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

        self.next_node_position = NextNodePosition::InsertAfter(Cow::Owned(node.clone()));
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
        self.next_node_position = NextNodePosition::FirstChildOf(Cow::Owned(element.clone()));
    }
}
