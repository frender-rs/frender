macro_rules! macro_component_type {
    ($($tag:ident)+) => {
        #[macro_export]
        macro_rules! component_type {
            $(
                ($tag) => {
                    $crate::intrinsic::$tag::Component
                };
            )+
            ($ty_comp:ty) => {
                $ty_comp
            };
        }

        #[macro_export]
        macro_rules! __impl_component_auto_import {
            $(
                ($tag) => {
                    #[allow(unused_imports)]
                    use $crate::intrinsic::$tag::AsPropsBuilder;
                    #[allow(unused_imports)]
                    use $crate::react::html::common::AsPropsBuilder as AsCommonPropsBuilder;
                };
            )+
            ($ty_comp:ty) => {};
        }

        #[macro_export]
        macro_rules! __impl_component_props_pre_build {
            $(
                ({$v:ident} $tag) => {
                    $v.__set_intrinsic_component(stringify!($tag))
                };
            )+
            ({$v:ident} $ty_comp:ty) => {
                $v
            };
        }
    };
}

macro_rules! intrinsic_component_tag {
    ($tag:ident) => {
        pub use $crate::react::html::$tag;
    };
    ($tag:ident : html) => {
        pub mod $tag {
            pub type Component = $crate::react::html::common::Component<
                $crate::react::html::common::ComponentProps<web_sys::HtmlElement, ()>,
            >;
        }
    };
}

macro_rules! all_intrinsic_component_tags {
    ($($tag:ident $(: $option:tt)?),+ $(,)?) => {
        macro_component_type! {
            $($tag)+
        }
        $(
            intrinsic_component_tag! {
                $tag
                $(: $option)?
            }
        )+
    };
}

all_intrinsic_component_tags! {
    // html
    a,
    abbr: html,
    address: html,
    // area: React.AreaHTMLAttributes<HTMLAreaElement>, HTMLAreaElement;
    article: html,
    aside: html,
    // audio: React.AudioHTMLAttributes<HTMLAudioElement>, HTMLAudioElement;
    b: html,
    // base: React.BaseHTMLAttributes<HTMLBaseElement>, HTMLBaseElement;
    bdi: html,
    bdo: html,
    big: html,
    // blockquote: React.BlockquoteHTMLAttributes<HTMLElement>, HTMLElement;
    // body: React.HTMLAttributes<HTMLBodyElement>, HTMLBodyElement;
    // br: React.HTMLAttributes<HTMLBRElement>, HTMLBRElement;
    // button: React.ButtonHTMLAttributes<HTMLButtonElement>, HTMLButtonElement;
    // canvas: React.CanvasHTMLAttributes<HTMLCanvasElement>, HTMLCanvasElement;
    caption: html,
    cite: html,
    code: html,
    // col: React.ColHTMLAttributes<HTMLTableColElement>, HTMLTableColElement;
    // colgroup: React.ColgroupHTMLAttributes<HTMLTableColElement>, HTMLTableColElement;
    // data: React.DataHTMLAttributes<HTMLDataElement>, HTMLDataElement;
    // datalist: React.HTMLAttributes<HTMLDataListElement>, HTMLDataListElement;
    dd: html,
    // del: React.DelHTMLAttributes<HTMLElement>, HTMLElement;
    // details: React.DetailsHTMLAttributes<HTMLElement>, HTMLElement;
    dfn: html,
    // dialog: React.DialogHTMLAttributes<HTMLDialogElement>, HTMLDialogElement;
    // div: React.HTMLAttributes<HTMLDivElement>, HTMLDivElement;
    // dl: React.HTMLAttributes<HTMLDListElement>, HTMLDListElement;
    dt: html,
    em: html,
    // embed: React.EmbedHTMLAttributes<HTMLEmbedElement>, HTMLEmbedElement;
    // fieldset: React.FieldsetHTMLAttributes<HTMLFieldSetElement>, HTMLFieldSetElement;
    figcaption: html,
    figure: html,
    footer: html,
    // form: React.FormHTMLAttributes<HTMLFormElement>, HTMLFormElement;
    // h1: React.HTMLAttributes<HTMLHeadingElement>, HTMLHeadingElement;
    // h2: React.HTMLAttributes<HTMLHeadingElement>, HTMLHeadingElement;
    // h3: React.HTMLAttributes<HTMLHeadingElement>, HTMLHeadingElement;
    // h4: React.HTMLAttributes<HTMLHeadingElement>, HTMLHeadingElement;
    // h5: React.HTMLAttributes<HTMLHeadingElement>, HTMLHeadingElement;
    // h6: React.HTMLAttributes<HTMLHeadingElement>, HTMLHeadingElement;
    // head: React.HTMLAttributes<HTMLHeadElement>, HTMLHeadElement;
    header: html,
    hgroup: html,
    // hr: React.HTMLAttributes<HTMLHRElement>, HTMLHRElement;
    // html: React.HtmlHTMLAttributes<HTMLHtmlElement>, HTMLHtmlElement;
    i: html,
    // iframe: React.IframeHTMLAttributes<HTMLIFrameElement>, HTMLIFrameElement;
    // img: React.ImgHTMLAttributes<HTMLImageElement>, HTMLImageElement;
    // input: React.InputHTMLAttributes<HTMLInputElement>, HTMLInputElement;
    // ins: React.InsHTMLAttributes<HTMLModElement>, HTMLModElement;
    kbd: html,
    // keygen: React.KeygenHTMLAttributes<HTMLElement>, HTMLElement;
    // label: React.LabelHTMLAttributes<HTMLLabelElement>, HTMLLabelElement;
    // legend: React.HTMLAttributes<HTMLLegendElement>, HTMLLegendElement;
    // li: React.LiHTMLAttributes<HTMLLIElement>, HTMLLIElement;
    // link: React.LinkHTMLAttributes<HTMLLinkElement>, HTMLLinkElement;
    main: html,
    // map: React.MapHTMLAttributes<HTMLMapElement>, HTMLMapElement;
    mark: html,
    // menu: React.MenuHTMLAttributes<HTMLElement>, HTMLElement;
    menuitem: html,
    // meta: React.MetaHTMLAttributes<HTMLMetaElement>, HTMLMetaElement;
    // meter: React.MeterHTMLAttributes<HTMLElement>, HTMLElement;
    nav: html,
    noindex: html,
    noscript: html,
    // object: React.ObjectHTMLAttributes<HTMLObjectElement>, HTMLObjectElement;
    // ol: React.OlHTMLAttributes<HTMLOListElement>, HTMLOListElement;
    // optgroup: React.OptgroupHTMLAttributes<HTMLOptGroupElement>, HTMLOptGroupElement;
    // option: React.OptionHTMLAttributes<HTMLOptionElement>, HTMLOptionElement;
    // output: React.OutputHTMLAttributes<HTMLElement>, HTMLElement;
    // p: React.HTMLAttributes<HTMLParagraphElement>, HTMLParagraphElement;
    // param: React.ParamHTMLAttributes<HTMLParamElement>, HTMLParamElement;
    picture: html,
    // pre: React.HTMLAttributes<HTMLPreElement>, HTMLPreElement;
    // progress: React.ProgressHTMLAttributes<HTMLProgressElement>, HTMLProgressElement;
    // q: React.QuoteHTMLAttributes<HTMLQuoteElement>, HTMLQuoteElement;
    rp: html,
    rt: html,
    ruby: html,
    s: html,
    samp: html,
    // slot: React.SlotHTMLAttributes<HTMLSlotElement>, HTMLSlotElement;
    // script: React.ScriptHTMLAttributes<HTMLScriptElement>, HTMLScriptElement;
    section: html,
    // select: React.SelectHTMLAttributes<HTMLSelectElement>, HTMLSelectElement;
    small: html,
    // source: React.SourceHTMLAttributes<HTMLSourceElement>, HTMLSourceElement;
    // span: React.HTMLAttributes<HTMLSpanElement>, HTMLSpanElement;
    strong: html,
    // style: React.StyleHTMLAttributes<HTMLStyleElement>, HTMLStyleElement;
    sub: html,
    summary: html,
    sup: html,
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
    u: html,
    // ul: React.HTMLAttributes<HTMLUListElement>, HTMLUListElement;
    var: html,
    // video: React.VideoHTMLAttributes<HTMLVideoElement>, HTMLVideoElement;
    wbr: html,
    // webview: React.WebViewHTMLAttributes<HTMLWebViewElement>, HTMLWebViewElement;
}
