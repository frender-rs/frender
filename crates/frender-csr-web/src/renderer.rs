use std::borrow::Cow;

use frender_html::{csr::web::Node, renderer::RenderTextFrom, RenderHtml};
use wasm_bindgen::{JsCast, UnwrapThrowExt};

use crate::try_behavior::{TryBehavior, TryWithTryBehavior};

#[derive(Debug, Clone)]
enum NextNodePosition<'a> {
    FirstChildOf(Cow<'a, web_sys::Element>),
    InsertAfter(Cow<'a, web_sys::Node>),
}

pub struct Renderer<'a, TB: TryBehavior> {
    document: &'a web_sys::Document,
    next_node_position: NextNodePosition<'a>,
    try_behavior: TB,
}

impl<'a, TB: TryBehavior> Renderer<'a, TB> {
    pub async fn render_element<E: frender_html::Element>(self, element: E) {
        let render = self.into_render_element();

        futures_lite::pin!(render);

        render.as_mut().update_with_element(element);

        render.await
    }
}

impl<'a> Renderer<'a, crate::try_behavior::UnwrapThrow> {
    pub fn new(document: &'a web_sys::Document, root_parent: web_sys::Element) -> Self {
        Self {
            document,
            next_node_position: NextNodePosition::FirstChildOf(Cow::Owned(root_parent)),
            try_behavior: crate::try_behavior::UnwrapThrow,
        }
    }
}

mod js_shims {
    use js_sys::JsString;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        pub type Text;

        #[wasm_bindgen(method, setter)]
        pub fn set_data(this: &Text, val: JsString);

        #[wasm_bindgen(js_name = Document)]
        pub type Document;

        #[wasm_bindgen(method, structural, js_class = "Document", js_name = createTextNode)]
        pub fn create_text_node(this: &Document, data: JsString) -> web_sys::Text;

        /// Calls `String(value)`
        #[wasm_bindgen(js_name = String)]
        pub fn js_string(value: JsValue) -> JsString;
    }
}

mod to_js_string {
    use wasm_bindgen::JsValue;

    pub(super) trait ToJsString {
        fn to_js_string(&self) -> js_sys::JsString;
    }

    impl ToJsString for char {
        fn to_js_string(&self) -> js_sys::JsString {
            From::from(*self)
        }
    }

    macro_rules! impl_for_each_of {
        (
            impl<__> $trait:ident for each_of! {$(
                $for_ty:ty
            ),* $(,)?}
            $t:tt
        ) => {$(
            impl $trait for $for_ty $t
        )*};
    }

    impl_for_each_of!(
        impl<__> ToJsString
            for each_of! {
                i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize,
                f32, f64,
                bool,
            }
        {
            fn to_js_string(&self) -> js_sys::JsString {
                super::js_shims::js_string(JsValue::from(*self))
            }
        }
    );
}

mod to_text_node {
    use super::Renderer;

    pub(super) trait ToTextNode {
        fn to_text_node<TB: crate::try_behavior::TryBehavior>(
            &self,
            renderer: &mut Renderer<TB>,
        ) -> web_sys::Text;

        fn update_text_node<TB: crate::try_behavior::TryBehavior>(
            &self,
            renderer: &mut Renderer<TB>,
            text: &web_sys::Text,
        );
    }

    impl<V: ?Sized + super::to_js_string::ToJsString> ToTextNode for V {
        fn to_text_node<TB: crate::try_behavior::TryBehavior>(
            &self,
            renderer: &mut Renderer<TB>,
        ) -> web_sys::Text {
            use wasm_bindgen::JsCast;
            super::js_shims::Document::create_text_node(
                renderer.document.unchecked_ref(),
                self.to_js_string(),
            )
        }

        fn update_text_node<TB: crate::try_behavior::TryBehavior>(
            &self,
            _: &mut Renderer<TB>,
            text: &web_sys::Text,
        ) {
            use wasm_bindgen::JsCast;
            super::js_shims::Text::set_data(text.unchecked_ref(), self.to_js_string())
        }
    }

    impl ToTextNode for str {
        fn to_text_node<TB: crate::try_behavior::TryBehavior>(
            &self,
            renderer: &mut Renderer<TB>,
        ) -> web_sys::Text {
            renderer.document.create_text_node(self)
        }

        fn update_text_node<TB: crate::try_behavior::TryBehavior>(
            &self,
            _: &mut Renderer<TB>,
            text: &web_sys::Text,
        ) {
            text.set_data(self)
        }
    }
}

impl<V: ?Sized + to_text_node::ToTextNode, TB: TryBehavior> RenderTextFrom<Node<web_sys::Text>, V>
    for Renderer<'_, TB>
{
    fn render_text_from(&mut self, v: &V) -> Node<web_sys::Text> {
        Node(v.to_text_node(self))
    }

    fn update_text_from(&mut self, text: &mut Node<web_sys::Text>, v: &V) {
        v.update_text_node(self, &text.0)
    }
}

macro_rules! html_elements {
    ($($tag:ident : $ty:ident),* $(,)?) => {$(
        type $tag = Node<web_sys::$ty>;

        fn $tag(&mut self) -> Self::$tag {
            let element = self
                .document
                .create_element(<frender_html::element_types::$tag as frender_html::renderer::HasIntrinsicComponentTag>::INTRINSIC_COMPONENT_TAG)
                .unwrap_with_behavior(&mut self.try_behavior);
            Node(element.unchecked_into())
        }
    )*};
}

impl<TB: TryBehavior> RenderHtml for Renderer<'_, TB> {
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

mod node {
    use std::borrow::Cow;

    use frender_html::event::{EventType, HasEventTypeName};
    use js_sys::global;

    use crate::try_behavior::{TryBehavior, TryWithTryBehavior};

    use super::{NextNodePosition, Renderer};

    impl<'a, TB: TryBehavior> Renderer<'a, TB> {
        fn readd_node(&mut self, node: &web_sys::Node, force_reposition: bool) {
            web_sys::console::log_3(&"readd_node".into(), node, &force_reposition.into());
            if force_reposition {
                match &self.next_node_position {
                    NextNodePosition::FirstChildOf(parent) => {
                        web_sys::console::log_2(&"FirstChildOf".into(), parent);

                        parent
                            .prepend_with_node_1(node)
                            .unwrap_with_behavior(&mut self.try_behavior)
                    }
                    NextNodePosition::InsertAfter(pre) => {
                        web_sys::console::log_2(&"InsertAfter".into(), pre);

                        pre.parent_node()
                            .unwrap()
                            .insert_before(node, pre.next_sibling().as_ref())
                            .unwrap_with_behavior(&mut self.try_behavior);
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
    }

    impl<N: AsRef<web_sys::Node>, TB: TryBehavior>
        frender_html::renderer::node_behaviors::Node<Renderer<'_, TB>> for super::Node<N>
    {
        fn cursor_is_at_self(&self, renderer: &Renderer<'_, TB>) -> bool {
            match &renderer.next_node_position {
                NextNodePosition::FirstChildOf(parent) => parent.first_child(),
                NextNodePosition::InsertAfter(previous) => previous.next_sibling(),
            }
            .map_or(false, |c| *self.0.as_ref() == c)
        }

        fn move_cursor_after_self(&mut self, renderer: &mut Renderer<'_, TB>) {
            let node = self.0.as_ref().clone();
            renderer.next_node_position = NextNodePosition::InsertAfter(Cow::Owned(node));
        }

        fn readd_self(&mut self, renderer: &mut Renderer<'_, TB>, force_reposition: bool) {
            renderer.readd_node(self.0.as_ref(), force_reposition)
        }

        fn remove_self(&mut self, renderer: &mut Renderer<'_, TB>) {
            renderer.remove_node(self.0.as_ref())
        }
    }

    impl<'r, N: AsRef<web_sys::Element> + AsRef<web_sys::Node>, TB: TryBehavior>
        frender_html::renderer::node_behaviors::Element<Renderer<'r, TB>> for super::Node<N>
    {
        fn move_cursor_at_the_first_child_of_self(&mut self, renderer: &mut Renderer<'_, TB>) {
            let element: &web_sys::Element = self.0.as_ref();

            renderer.next_node_position =
                NextNodePosition::FirstChildOf(Cow::Owned(element.clone()));
        }

        fn set_attribute(&mut self, renderer: &mut Renderer<'_, TB>, name: &str, value: &str) {
            let element: &web_sys::Element = self.0.as_ref();

            element
                .set_attribute(name, value)
                .unwrap_with_behavior(&mut renderer.try_behavior)
        }

        fn remove_attribute(&mut self, renderer: &mut Renderer<'_, TB>, name: &str) {
            let element: &web_sys::Element = self.0.as_ref();
            element
                .remove_attribute(name)
                .unwrap_with_behavior(&mut renderer.try_behavior)
        }

        type ClassList<'a> = super::dom_token_list::DomTokenList<&'a mut TB>
        where
            Self: 'a,
            Renderer<'r, TB>: 'a;

        fn class_list<'a>(&'a mut self, renderer: &'a mut Renderer<'r, TB>) -> Self::ClassList<'a> {
            let element: &web_sys::Element = self.0.as_ref();
            super::dom_token_list::DomTokenList(element.class_list(), &mut renderer.try_behavior)
        }

        fn set_id(&mut self, renderer: &mut Renderer<'_, TB>, id: &str) {
            let element: &web_sys::Element = self.0.as_ref();

            element.set_id(id)
        }
    }

    impl<
            N: AsRef<web_sys::HtmlElement> + AsRef<web_sys::Element> + AsRef<web_sys::Node>,
            TB: TryBehavior,
        > frender_html::renderer::node_behaviors::HtmlElement<Renderer<'_, TB>> for super::Node<N>
    {
        fn set_access_key(&mut self, renderer: &mut Renderer<'_, TB>, value: &str) {
            let element: &web_sys::HtmlElement = self.0.as_ref();
            element.set_access_key(value)
        }

        fn set_content_editable(&mut self, renderer: &mut Renderer<'_, TB>, value: &str) {
            let element: &web_sys::HtmlElement = self.0.as_ref();
            element.set_content_editable(value)
        }

        fn set_dir(&mut self, renderer: &mut Renderer<'_, TB>, value: &str) {
            let element: &web_sys::HtmlElement = self.0.as_ref();
            element.set_dir(value);
        }

        fn set_draggable(&mut self, renderer: &mut Renderer<'_, TB>, value: bool) {
            let element: &web_sys::HtmlElement = self.0.as_ref();
            element.set_draggable(value)
        }

        fn set_hidden(&mut self, renderer: &mut Renderer<'_, TB>, value: bool) {
            let element: &web_sys::HtmlElement = self.0.as_ref();
            element.set_hidden(value)
        }

        fn set_lang(&mut self, renderer: &mut Renderer<'_, TB>, value: &str) {
            let element: &web_sys::HtmlElement = self.0.as_ref();
            element.set_lang(value)
        }

        fn set_spellcheck(&mut self, renderer: &mut Renderer<'_, TB>, value: bool) {
            let element: &web_sys::HtmlElement = self.0.as_ref();
            element.set_spellcheck(value)
        }

        fn set_tab_index(&mut self, renderer: &mut Renderer<'_, TB>, value: i32) {
            let element: &web_sys::HtmlElement = self.0.as_ref();
            element.set_tab_index(value)
        }

        fn set_title(&mut self, renderer: &mut Renderer<'_, TB>, value: &str) {
            let element: &web_sys::HtmlElement = self.0.as_ref();
            element.set_title(value)
        }
    }

    impl<'r, TB: TryBehavior>
        frender_html::renderer::node_behaviors::HtmlElementWithHref<Renderer<'r, TB>>
        for super::Node<web_sys::HtmlAnchorElement>
    {
        fn set_download(&mut self, renderer: &mut Renderer<'_, TB>, value: &str) {
            self.0.set_download(value)
        }

        fn set_href(&mut self, renderer: &mut Renderer<'_, TB>, value: &str) {
            self.0.set_href(value)
        }

        fn set_ping(&mut self, renderer: &mut Renderer<'_, TB>, value: &str) {
            self.0.set_ping(value)
        }

        fn set_referrer_policy(&mut self, renderer: &mut Renderer<'_, TB>, value: &str) {
            self.0.set_referrer_policy(value)
        }

        type RelList<'a> = super::dom_token_list::DomTokenList<&'a mut TB>
        where
            Self: 'a,
            Renderer<'r, TB>: 'a;

        fn rel_list<'a>(&'a mut self, renderer: &'a mut Renderer<'r, TB>) -> Self::RelList<'a> {
            let element = &self.0;
            super::dom_token_list::DomTokenList(element.rel_list(), &mut renderer.try_behavior)
        }

        fn set_target(&mut self, renderer: &mut Renderer<'_, TB>, value: &str) {
            self.0.set_target(value)
        }
    }

    impl<'r, TB: TryBehavior>
        frender_html::renderer::node_behaviors::HtmlElementWithHref<Renderer<'r, TB>>
        for super::Node<web_sys::HtmlAreaElement>
    {
        fn set_download(&mut self, renderer: &mut Renderer<'_, TB>, value: &str) {
            self.0.set_download(value)
        }

        fn set_href(&mut self, renderer: &mut Renderer<'_, TB>, value: &str) {
            self.0.set_href(value)
        }

        fn set_ping(&mut self, renderer: &mut Renderer<'_, TB>, value: &str) {
            self.0.set_ping(value)
        }

        fn set_referrer_policy(&mut self, renderer: &mut Renderer<'_, TB>, value: &str) {
            self.0.set_referrer_policy(value)
        }

        type RelList<'a> = super::dom_token_list::DomTokenList<&'a mut TB>
    where
        Self: 'a,
        Renderer<'r, TB>: 'a;

        fn rel_list<'a>(&'a mut self, renderer: &'a mut Renderer<'r, TB>) -> Self::RelList<'a> {
            let element = &self.0;
            super::dom_token_list::DomTokenList(element.rel_list(), &mut renderer.try_behavior)
        }

        fn set_target(&mut self, renderer: &mut Renderer<'_, TB>, value: &str) {
            self.0.set_target(value)
        }
    }

    impl<'r, TB: TryBehavior>
        frender_html::renderer::node_behaviors::HtmlAnchorElement<Renderer<'r, TB>>
        for super::Node<web_sys::HtmlAnchorElement>
    {
        fn set_href_lang(&mut self, renderer: &mut Renderer<'r, TB>, value: &str) {
            self.0.set_hreflang(value)
        }

        fn set_type(&mut self, renderer: &mut Renderer<'r, TB>, value: &str) {
            self.0.set_type(value)
        }
    }

    impl<'r, TB: TryBehavior>
        frender_html::renderer::node_behaviors::HtmlAreaElement<Renderer<'r, TB>>
        for super::Node<web_sys::HtmlAreaElement>
    {
        fn set_alt(&mut self, renderer: &mut Renderer<'r, TB>, value: &str) {
            self.0.set_alt(value)
        }

        fn set_coords(&mut self, renderer: &mut Renderer<'r, TB>, value: &str) {
            self.0.set_coords(value)
        }

        fn set_shape(&mut self, renderer: &mut Renderer<'r, TB>, value: &str) {
            self.0.set_shape(value)
        }
    }

    impl<
            N: AsRef<web_sys::HtmlMediaElement>
                + AsRef<web_sys::HtmlElement>
                + AsRef<web_sys::Element>
                + AsRef<web_sys::Node>,
            TB: TryBehavior,
        > frender_html::renderer::node_behaviors::HtmlMediaElement<Renderer<'_, TB>>
        for super::Node<N>
    {
        fn set_auto_play(&mut self, renderer: &mut Renderer<'_, TB>, value: bool) {
            let element: &web_sys::HtmlMediaElement = self.0.as_ref();
            element.set_autoplay(value)
        }

        fn set_controls(&mut self, renderer: &mut Renderer<'_, TB>, value: bool) {
            let element: &web_sys::HtmlMediaElement = self.0.as_ref();
            element.set_controls(value)
        }

        fn set_cross_origin(&mut self, renderer: &mut Renderer<'_, TB>, value: Option<&str>) {
            let element: &web_sys::HtmlMediaElement = self.0.as_ref();
            element.set_cross_origin(value)
        }

        fn set_loop(&mut self, renderer: &mut Renderer<'_, TB>, value: bool) {
            let element: &web_sys::HtmlMediaElement = self.0.as_ref();
            element.set_loop(value)
        }

        fn set_muted(&mut self, renderer: &mut Renderer<'_, TB>, value: bool) {
            let element: &web_sys::HtmlMediaElement = self.0.as_ref();
            element.set_muted(value)
        }

        fn set_preload(&mut self, renderer: &mut Renderer<'_, TB>, value: &str) {
            let element: &web_sys::HtmlMediaElement = self.0.as_ref();
            element.set_preload(value)
        }

        fn set_src(&mut self, renderer: &mut Renderer<'_, TB>, value: &str) {
            let element: &web_sys::HtmlMediaElement = self.0.as_ref();
            element.set_src(value)
        }
    }

    impl<
            N: AsRef<web_sys::HtmlAudioElement>
                + AsRef<web_sys::HtmlMediaElement>
                + AsRef<web_sys::HtmlElement>
                + AsRef<web_sys::Element>
                + AsRef<web_sys::Node>,
            TB: TryBehavior,
        > frender_html::renderer::node_behaviors::HtmlAudioElement<Renderer<'_, TB>>
        for super::Node<N>
    {
    }

    impl<
            N: AsRef<web_sys::HtmlVideoElement>
                + AsRef<web_sys::HtmlMediaElement>
                + AsRef<web_sys::HtmlElement>
                + AsRef<web_sys::Element>
                + AsRef<web_sys::Node>,
            TB: TryBehavior,
        > frender_html::renderer::node_behaviors::HtmlVideoElement<Renderer<'_, TB>>
        for super::Node<N>
    {
        fn set_height(&mut self, renderer: &mut Renderer<'_, TB>, value: u32) {
            let element: &web_sys::HtmlVideoElement = self.0.as_ref();
            element.set_height(value)
        }

        fn set_poster(&mut self, renderer: &mut Renderer<'_, TB>, value: &str) {
            let element: &web_sys::HtmlVideoElement = self.0.as_ref();
            element.set_poster(value)
        }

        fn set_width(&mut self, renderer: &mut Renderer<'_, TB>, value: u32) {
            let element: &web_sys::HtmlVideoElement = self.0.as_ref();
            element.set_width(value)
        }
    }

    impl<
            N: AsRef<web_sys::HtmlBaseElement>
                + AsRef<web_sys::HtmlElement>
                + AsRef<web_sys::Element>
                + AsRef<web_sys::Node>,
            TB: TryBehavior,
        > frender_html::renderer::node_behaviors::HtmlBaseElement<Renderer<'_, TB>>
        for super::Node<N>
    {
        fn set_href(&mut self, renderer: &mut Renderer<'_, TB>, value: &str) {
            web_sys::HtmlBaseElement::set_href(self.0.as_ref(), value)
        }

        fn set_target(&mut self, renderer: &mut Renderer<'_, TB>, value: &str) {
            web_sys::HtmlBaseElement::set_target(self.0.as_ref(), value)
        }
    }
    impl<
            N: AsRef<web_sys::HtmlQuoteElement>
                + AsRef<web_sys::HtmlElement>
                + AsRef<web_sys::Element>
                + AsRef<web_sys::Node>,
            TB: TryBehavior,
        > frender_html::renderer::node_behaviors::HtmlQuoteElement<Renderer<'_, TB>>
        for super::Node<N>
    {
        fn set_cite(&mut self, renderer: &mut Renderer<'_, TB>, value: &str) {
            web_sys::HtmlQuoteElement::set_cite(self.0.as_ref(), value)
        }
    }
    impl<
            N: AsRef<web_sys::HtmlBodyElement>
                + AsRef<web_sys::HtmlElement>
                + AsRef<web_sys::Element>
                + AsRef<web_sys::Node>,
            TB: TryBehavior,
        > frender_html::renderer::node_behaviors::HtmlBodyElement<Renderer<'_, TB>>
        for super::Node<N>
    {
    }
    impl<
            N: AsRef<web_sys::HtmlBrElement>
                + AsRef<web_sys::HtmlElement>
                + AsRef<web_sys::Element>
                + AsRef<web_sys::Node>,
            TB: TryBehavior,
        > frender_html::renderer::node_behaviors::HtmlBrElement<Renderer<'_, TB>>
        for super::Node<N>
    {
        fn set_clear(&mut self, renderer: &mut Renderer<'_, TB>, value: &str) {
            web_sys::HtmlBrElement::set_clear(self.0.as_ref(), value)
        }
    }
    impl<
            N: AsRef<web_sys::HtmlButtonElement>
                + AsRef<web_sys::HtmlElement>
                + AsRef<web_sys::Element>
                + AsRef<web_sys::Node>,
            TB: TryBehavior,
        > frender_html::renderer::node_behaviors::HtmlButtonElement<Renderer<'_, TB>>
        for super::Node<N>
    {
        fn set_disabled(&mut self, renderer: &mut Renderer<'_, TB>, value: bool) {
            web_sys::HtmlButtonElement::set_disabled(self.0.as_ref(), value)
        }

        fn set_form_action(&mut self, renderer: &mut Renderer<'_, TB>, value: &str) {
            web_sys::HtmlButtonElement::set_form_action(self.0.as_ref(), value)
        }

        fn set_form_enctype(&mut self, renderer: &mut Renderer<'_, TB>, value: &str) {
            web_sys::HtmlButtonElement::set_form_enctype(self.0.as_ref(), value)
        }

        fn set_form_method(&mut self, renderer: &mut Renderer<'_, TB>, value: &str) {
            web_sys::HtmlButtonElement::set_form_method(self.0.as_ref(), value)
        }

        fn set_form_no_validate(&mut self, renderer: &mut Renderer<'_, TB>, value: bool) {
            web_sys::HtmlButtonElement::set_form_no_validate(self.0.as_ref(), value)
        }

        fn set_form_target(&mut self, renderer: &mut Renderer<'_, TB>, value: &str) {
            web_sys::HtmlButtonElement::set_form_target(self.0.as_ref(), value)
        }

        fn set_name(&mut self, renderer: &mut Renderer<'_, TB>, value: &str) {
            web_sys::HtmlButtonElement::set_name(self.0.as_ref(), value)
        }

        fn set_type(&mut self, renderer: &mut Renderer<'_, TB>, value: &str) {
            web_sys::HtmlButtonElement::set_type(self.0.as_ref(), value)
        }

        fn set_value(&mut self, renderer: &mut Renderer<'_, TB>, value: &str) {
            web_sys::HtmlButtonElement::set_value(self.0.as_ref(), value)
        }
    }
}

mod dom_token_list {
    use crate::try_behavior::{TryBehavior, TryWithTryBehavior};

    pub struct DomTokenList<TB: TryBehavior>(pub(super) web_sys::DomTokenList, pub(super) TB);

    impl<TB: TryBehavior> frender_html::DomTokenList for DomTokenList<TB> {
        fn set_value(&mut self, value: &str) {
            self.0.set_value(value)
        }

        fn add_1(&mut self, token: &str) {
            self.0.add_1(token).unwrap_with_behavior(&mut self.1)
        }

        fn remove_1(&mut self, token: &str) {
            self.0.remove_1(token).unwrap_with_behavior(&mut self.1)
        }

        fn replace(&mut self, old_token: &str, new_token: &str) {
            _ = self
                .0
                .replace(old_token, new_token)
                .unwrap_with_behavior(&mut self.1)
        }
    }
}
