use std::borrow::Cow;

use frender_html::{
    dom::{
        csr::web::{CursorPlaceholder, Node, RenderContext},
        render::{Render, RenderWithContext},
        ProvideRenderContext,
    },
    RenderHtml,
};

mod text;

pub struct Renderer {
    document: web_sys::Document,
}

pub struct RendererWithRoot {
    renderer: Renderer,
    root: web_sys::Element,
}

impl ProvideRenderContext for RendererWithRoot {
    type Renderer = Renderer;

    fn provide_render_context<Res>(
        &mut self,
        f: impl FnOnce(&mut <Self::Renderer as RenderWithContext>::RenderContext<'_>) -> Res,
    ) -> Res {
        let mut ctx = RenderContext {
            renderer: &mut self.renderer,
            cursor: &mut frender_html::dom::csr::web::Cursor::first_child_of(Cow::Borrowed(
                &self.root,
            )),
        };
        f(&mut ctx)
    }

    fn renderer_mut(&mut self) -> &mut Self::Renderer {
        todo!()
    }
}

impl RendererWithRoot {
    pub fn new(document: web_sys::Document, root_parent: web_sys::Element) -> Self {
        Self {
            renderer: Renderer { document },
            root: root_parent,
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

impl Render for Renderer {
    fn log(&mut self, v: &str) {
        web_sys::console::log_1(&v.into())
    }

    type CursorPlaceholder = CursorPlaceholder;
}

impl RenderWithContext for Renderer {
    type RenderContext<'a> = RenderContext<'a, Self>;
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

    fn cursor_is_at_node(
        render_context: &<Self as RenderWithContext>::RenderContext<'_>,
        node: &web_sys::Node,
    ) -> bool
    where
        Self: RenderWithContext,
    {
        render_context.cursor.cursor_is_at_node(node)
    }

    fn readd_node(
        render_context: &mut <Self as RenderWithContext>::RenderContext<'_>,
        node: &web_sys::Node,
        force_reposition: bool,
    ) where
        Self: RenderWithContext,
    {
        render_context.cursor.readd_node(node, force_reposition)
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
}
