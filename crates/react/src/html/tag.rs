pub trait IntrinsicElementTag {
    const TAG: &'static str;
}

macro_rules! impl_for_element {
    ($($tag:literal :$(-)? $ty_element:ty),+ $(,)?) => {
        $(
            impl IntrinsicElementTag for $ty_element {
                const TAG: &'static str = $tag;
            }
        )+
    };
}

macro_rules! impl_for_element_one {
    (@$tag:ident) => {
        pub use $crate::html::attrs::$tag;
    };
    ($tag:ident) => {
        pub mod $tag {
            pub type Component = $crate::html::common::Component<
                $crate::html::common::ComponentProps<web_sys::HtmlElement, ()>,
            >;
        }
    };
}

// HTML
impl_for_element! {
    "a": web_sys::HtmlAnchorElement,
    "abbr": web_sys::HtmlElement,
// address: 0;
// area: React.AreaHTMLAttributes<HTMLAreaElement>, HTMLAreaElement;
// article: 0;
// aside: 0;
// audio: React.AudioHTMLAttributes<HTMLAudioElement>, HTMLAudioElement;
// b: 0;
// base: React.BaseHTMLAttributes<HTMLBaseElement>, HTMLBaseElement;
// bdi: 0;
// bdo: 0;
// big: 0;
// blockquote: React.BlockquoteHTMLAttributes<HTMLElement>, HTMLElement;
// body: React.HTMLAttributes<HTMLBodyElement>, HTMLBodyElement;
// br: React.HTMLAttributes<HTMLBRElement>, HTMLBRElement;
// button: React.ButtonHTMLAttributes<HTMLButtonElement>, HTMLButtonElement;
// canvas: React.CanvasHTMLAttributes<HTMLCanvasElement>, HTMLCanvasElement;
// caption: 0;
// cite: 0;
// code: 0;
// col: React.ColHTMLAttributes<HTMLTableColElement>, HTMLTableColElement;
// colgroup: React.ColgroupHTMLAttributes<HTMLTableColElement>, HTMLTableColElement;
// data: React.DataHTMLAttributes<HTMLDataElement>, HTMLDataElement;
// datalist: React.HTMLAttributes<HTMLDataListElement>, HTMLDataListElement;
// dd: 0;
// del: React.DelHTMLAttributes<HTMLElement>, HTMLElement;
// details: React.DetailsHTMLAttributes<HTMLElement>, HTMLElement;
// dfn: 0;
// dialog: React.DialogHTMLAttributes<HTMLDialogElement>, HTMLDialogElement;
// div: React.HTMLAttributes<HTMLDivElement>, HTMLDivElement;
// dl: React.HTMLAttributes<HTMLDListElement>, HTMLDListElement;
// dt: 0;
// em: 0;
// embed: React.EmbedHTMLAttributes<HTMLEmbedElement>, HTMLEmbedElement;
// fieldset: React.FieldsetHTMLAttributes<HTMLFieldSetElement>, HTMLFieldSetElement;
// figcaption: 0;
// figure: 0;
// footer: 0;
// form: React.FormHTMLAttributes<HTMLFormElement>, HTMLFormElement;
// h1: React.HTMLAttributes<HTMLHeadingElement>, HTMLHeadingElement;
// h2: React.HTMLAttributes<HTMLHeadingElement>, HTMLHeadingElement;
// h3: React.HTMLAttributes<HTMLHeadingElement>, HTMLHeadingElement;
// h4: React.HTMLAttributes<HTMLHeadingElement>, HTMLHeadingElement;
// h5: React.HTMLAttributes<HTMLHeadingElement>, HTMLHeadingElement;
// h6: React.HTMLAttributes<HTMLHeadingElement>, HTMLHeadingElement;
// head: React.HTMLAttributes<HTMLHeadElement>, HTMLHeadElement;
// header: 0;
// hgroup: 0;
// hr: React.HTMLAttributes<HTMLHRElement>, HTMLHRElement;
// html: React.HtmlHTMLAttributes<HTMLHtmlElement>, HTMLHtmlElement;
// i: 0;
// iframe: React.IframeHTMLAttributes<HTMLIFrameElement>, HTMLIFrameElement;
// img: React.ImgHTMLAttributes<HTMLImageElement>, HTMLImageElement;
// input: React.InputHTMLAttributes<HTMLInputElement>, HTMLInputElement;
// ins: React.InsHTMLAttributes<HTMLModElement>, HTMLModElement;
// kbd: 0;
// keygen: React.KeygenHTMLAttributes<HTMLElement>, HTMLElement;
// label: React.LabelHTMLAttributes<HTMLLabelElement>, HTMLLabelElement;
// legend: React.HTMLAttributes<HTMLLegendElement>, HTMLLegendElement;
// li: React.LiHTMLAttributes<HTMLLIElement>, HTMLLIElement;
// link: React.LinkHTMLAttributes<HTMLLinkElement>, HTMLLinkElement;
// main: 0;
// map: React.MapHTMLAttributes<HTMLMapElement>, HTMLMapElement;
// mark: 0;
// menu: React.MenuHTMLAttributes<HTMLElement>, HTMLElement;
// menuitem: 0;
// meta: React.MetaHTMLAttributes<HTMLMetaElement>, HTMLMetaElement;
// meter: React.MeterHTMLAttributes<HTMLElement>, HTMLElement;
// nav: 0;
// noindex: 0;
// noscript: 0;
// object: React.ObjectHTMLAttributes<HTMLObjectElement>, HTMLObjectElement;
// ol: React.OlHTMLAttributes<HTMLOListElement>, HTMLOListElement;
// optgroup: React.OptgroupHTMLAttributes<HTMLOptGroupElement>, HTMLOptGroupElement;
// option: React.OptionHTMLAttributes<HTMLOptionElement>, HTMLOptionElement;
// output: React.OutputHTMLAttributes<HTMLElement>, HTMLElement;
// p: React.HTMLAttributes<HTMLParagraphElement>, HTMLParagraphElement;
// param: React.ParamHTMLAttributes<HTMLParamElement>, HTMLParamElement;
// picture: 0;
// pre: React.HTMLAttributes<HTMLPreElement>, HTMLPreElement;
// progress: React.ProgressHTMLAttributes<HTMLProgressElement>, HTMLProgressElement;
// q: React.QuoteHTMLAttributes<HTMLQuoteElement>, HTMLQuoteElement;
// rp: 0;
// rt: 0;
// ruby: 0;
// s: 0;
// samp: 0;
// slot: React.SlotHTMLAttributes<HTMLSlotElement>, HTMLSlotElement;
// script: React.ScriptHTMLAttributes<HTMLScriptElement>, HTMLScriptElement;
// section: 0;
// select: React.SelectHTMLAttributes<HTMLSelectElement>, HTMLSelectElement;
// small: 0;
// source: React.SourceHTMLAttributes<HTMLSourceElement>, HTMLSourceElement;
// span: React.HTMLAttributes<HTMLSpanElement>, HTMLSpanElement;
// strong: 0;
// style: React.StyleHTMLAttributes<HTMLStyleElement>, HTMLStyleElement;
// sub: 0;
// summary: 0;
// sup: 0;
// table: React.TableHTMLAttributes<HTMLTableElement>, HTMLTableElement;
// template: React.HTMLAttributes<HTMLTemplateElement>, HTMLTemplateElement;
// tbody: React.HTMLAttributes<HTMLTableSectionElement>, HTMLTableSectionElement;
// td: React.TdHTMLAttributes<HTMLTableDataCellElement>, HTMLTableDataCellElement;
// textarea: React.TextareaHTMLAttributes<HTMLTextAreaElement>, HTMLTextAreaElement;
// tfoot: React.HTMLAttributes<HTMLTableSectionElement>, HTMLTableSectionElement;
// th: React.ThHTMLAttributes<HTMLTableHeaderCellElement>, HTMLTableHeaderCellElement;
// thead: React.HTMLAttributes<HTMLTableSectionElement>, HTMLTableSectionElement;
// time: React.TimeHTMLAttributes<HTMLElement>, HTMLElement;
// title: React.HTMLAttributes<HTMLTitleElement>, HTMLTitleElement;
// tr: React.HTMLAttributes<HTMLTableRowElement>, HTMLTableRowElement;
// track: React.TrackHTMLAttributes<HTMLTrackElement>, HTMLTrackElement;
// u: 0;
// ul: React.HTMLAttributes<HTMLUListElement>, HTMLUListElement;
// "var": 0;
// video: React.VideoHTMLAttributes<HTMLVideoElement>, HTMLVideoElement;
// wbr: 0;
// webview: React.WebViewHTMLAttributes<HTMLWebViewElement>, HTMLWebViewElement;
}
