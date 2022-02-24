crate::macros::all_intrinsic_component_tags! {
    // html
    a,
    abbr: AbbrComponent,
    address: AddressComponent,
    area,
    article: ArticleComponent,
    aside: AsideComponent,
    audio,
    b: BComponent,
    base,
    bdi: BdiComponent,
    bdo: BdoComponent,
    big: BigComponent,
    blockquote,
    body: { BodyComponent => web_sys::HtmlBodyElement },
    br: { BrComponent => web_sys::HtmlBrElement },
    button,
    canvas,
    caption: CaptionComponent,
    cite: CiteComponent,
    code: CodeComponent,
    col,
    colgroup,
    data,
    datalist: { DataListComponent => web_sys::HtmlDataListElement },
    dd: DdComponent,
    del,
    details,
    dfn: DfnComponent,
    dialog,
    div: { DivComponent => web_sys::HtmlDivElement },
    dl: { DListComponent => web_sys::HtmlDListElement },
    dt: DtComponent,
    em: EmComponent,
    embed,
    fieldset,
    figcaption: FigCaptionComponent,
    figure: FigureComponent,
    footer: FooterComponent,
    form,
    h1: { H1Component => web_sys::HtmlHeadingElement },
    h2: { H2Component => web_sys::HtmlHeadingElement },
    h3: { H3Component => web_sys::HtmlHeadingElement },
    h4: { H4Component => web_sys::HtmlHeadingElement },
    h5: { H5Component => web_sys::HtmlHeadingElement },
    h6: { H6Component => web_sys::HtmlHeadingElement },
    head: { HeadComponent => web_sys::HtmlHeadElement },
    header: HeaderComponent,
    hgroup: HGroupComponent,
    hr: { HrComponent => web_sys::HtmlHrElement },
    html,
    i: IComponent,
    iframe,
    // img: React.ImgHTMLAttributes<HTMLImageElement>, HTMLImageElement;
    // input: React.InputHTMLAttributes<HTMLInputElement>, HTMLInputElement;
    // ins: React.InsHTMLAttributes<HTMLModElement>, HTMLModElement;
    kbd: KbdComponent,
    // keygen: React.KeygenHTMLAttributes<HTMLElement>, HTMLElement;
    // label: React.LabelHTMLAttributes<HTMLLabelElement>, HTMLLabelElement;
    // legend: React.HTMLAttributes<HTMLLegendElement>, HTMLLegendElement;
    // li: React.LiHTMLAttributes<HTMLLIElement>, HTMLLIElement;
    // link: React.LinkHTMLAttributes<HTMLLinkElement>, HTMLLinkElement;
    main: MainComponent,
    // map: React.MapHTMLAttributes<HTMLMapElement>, HTMLMapElement;
    mark: MarkComponent,
    // menu: React.MenuHTMLAttributes<HTMLElement>, HTMLElement;
    menuitem: MenuItemComponent,
    // meta: React.MetaHTMLAttributes<HTMLMetaElement>, HTMLMetaElement;
    // meter: React.MeterHTMLAttributes<HTMLElement>, HTMLElement;
    nav: NavComponent,
    noindex: NoIndexComponent,
    noscript: NoScriptComponent,
    // object: React.ObjectHTMLAttributes<HTMLObjectElement>, HTMLObjectElement;
    // ol: React.OlHTMLAttributes<HTMLOListElement>, HTMLOListElement;
    // optgroup: React.OptgroupHTMLAttributes<HTMLOptGroupElement>, HTMLOptGroupElement;
    // option: React.OptionHTMLAttributes<HTMLOptionElement>, HTMLOptionElement;
    // output: React.OutputHTMLAttributes<HTMLElement>, HTMLElement;
    // p: React.HTMLAttributes<HTMLParagraphElement>, HTMLParagraphElement;
    // param: React.ParamHTMLAttributes<HTMLParamElement>, HTMLParamElement;
    picture: PictureComponent,
    // pre: React.HTMLAttributes<HTMLPreElement>, HTMLPreElement;
    // progress: React.ProgressHTMLAttributes<HTMLProgressElement>, HTMLProgressElement;
    // q: React.QuoteHTMLAttributes<HTMLQuoteElement>, HTMLQuoteElement;
    rp: RpComponent,
    rt: RtComponent,
    ruby: RubyComponent,
    s: SComponent,
    samp: SampComponent,
    // slot: React.SlotHTMLAttributes<HTMLSlotElement>, HTMLSlotElement;
    // script: React.ScriptHTMLAttributes<HTMLScriptElement>, HTMLScriptElement;
    section: SectionComponent,
    // select: React.SelectHTMLAttributes<HTMLSelectElement>, HTMLSelectElement;
    small: SmallComponent,
    // source: React.SourceHTMLAttributes<HTMLSourceElement>, HTMLSourceElement;
    // span: React.HTMLAttributes<HTMLSpanElement>, HTMLSpanElement;
    strong: StrongComponent,
    // style: React.StyleHTMLAttributes<HTMLStyleElement>, HTMLStyleElement;
    sub: SubComponent,
    summary: SummaryComponent,
    sup: SupComponent,
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
    u: UComponent,
    // ul: React.HTMLAttributes<HTMLUListElement>, HTMLUListElement;
    var: VarComponent,
    // video: React.VideoHTMLAttributes<HTMLVideoElement>, HTMLVideoElement;
    wbr: WbrComponent,
    // webview: React.WebViewHTMLAttributes<HTMLWebViewElement>, HTMLWebViewElement;
}
