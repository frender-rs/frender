#[allow(unused_imports)]
use super::*;
#[allow(non_snake_case)]
pub mod ElementProps;
pub use ElementProps::ElementProps;
#[allow(non_snake_case)]
pub mod HtmlElementProps;
pub use HtmlElementProps::HtmlElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlElementProps;
    type Element = web_sys::HtmlElement;
    pub struct abbr;
    pub struct address;
    pub struct article;
    pub struct aside;
    pub struct b;
    pub struct bdi;
    pub struct bdo;
    pub struct cite;
    pub struct code;
    pub struct datalist;
    pub struct dd;
    pub struct dfn;
    pub struct div;
    pub struct dl;
    pub struct dt;
    pub struct em;
    pub struct figcaption;
    pub struct figure;
    pub struct footer;
    pub struct h1;
    pub struct h2;
    pub struct h3;
    pub struct h4;
    pub struct h5;
    pub struct h6;
    pub struct head;
    pub struct header;
    pub struct hgroup;
    pub struct hr;
    pub struct i;
    pub struct kbd;
    pub struct legend;
    pub struct main;
    pub struct mark;
    pub struct menu;
    pub struct nav;
    pub struct noscript;
    pub struct p;
    pub struct picture;
    pub struct pre;
    pub struct rp;
    pub struct rt;
    pub struct ruby;
    pub struct s;
    pub struct samp;
    pub struct section;
    pub struct small;
    pub struct span;
    pub struct strong;
    pub struct sub;
    pub struct summary;
    pub struct sup;
    pub struct template;
    pub struct title;
    pub struct u;
    pub struct var;
    pub struct wbr;
);
#[allow(non_snake_case)]
pub mod HtmlAnchorElementProps;
pub use HtmlAnchorElementProps::HtmlAnchorElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlAnchorElementProps;
    type Element = web_sys::HtmlAnchorElement;
    pub struct a;
);
#[allow(non_snake_case)]
pub mod HtmlAreaElementProps;
pub use HtmlAreaElementProps::HtmlAreaElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlAreaElementProps;
    type Element = web_sys::HtmlAreaElement;
    pub struct area;
);
#[allow(non_snake_case)]
pub mod HtmlMediaElementProps;
pub use HtmlMediaElementProps::HtmlMediaElementProps;
#[allow(non_snake_case)]
pub mod HtmlAudioElementProps;
pub use HtmlAudioElementProps::HtmlAudioElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlAudioElementProps;
    type Element = web_sys::HtmlAudioElement;
    pub struct audio;
);
#[allow(non_snake_case)]
pub mod HtmlVideoElementProps;
pub use HtmlVideoElementProps::HtmlVideoElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlVideoElementProps;
    type Element = web_sys::HtmlVideoElement;
    pub struct video;
);
#[allow(non_snake_case)]
pub mod HtmlBaseElementProps;
pub use HtmlBaseElementProps::HtmlBaseElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlBaseElementProps;
    type Element = web_sys::HtmlBaseElement;
    pub struct base;
);
#[allow(non_snake_case)]
pub mod HtmlQuoteElementProps;
pub use HtmlQuoteElementProps::HtmlQuoteElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlQuoteElementProps;
    type Element = web_sys::HtmlQuoteElement;
    pub struct blockquote;
    pub struct q;
);
#[allow(non_snake_case)]
pub mod HtmlBodyElementProps;
pub use HtmlBodyElementProps::HtmlBodyElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlBodyElementProps;
    type Element = web_sys::HtmlBodyElement;
    pub struct body;
);
#[allow(non_snake_case)]
pub mod HtmlBrElementProps;
pub use HtmlBrElementProps::HtmlBrElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlBrElementProps;
    type Element = web_sys::HtmlBrElement;
    pub struct br;
);
#[allow(non_snake_case)]
pub mod HtmlButtonElementProps;
pub use HtmlButtonElementProps::HtmlButtonElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlButtonElementProps;
    type Element = web_sys::HtmlButtonElement;
    pub struct button;
);
#[allow(non_snake_case)]
pub mod HtmlCanvasElementProps;
pub use HtmlCanvasElementProps::HtmlCanvasElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlCanvasElementProps;
    type Element = web_sys::HtmlCanvasElement;
    pub struct canvas;
);
#[allow(non_snake_case)]
pub mod HtmlTableCaptionElementProps;
pub use HtmlTableCaptionElementProps::HtmlTableCaptionElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlTableCaptionElementProps;
    type Element = web_sys::HtmlTableCaptionElement;
    pub struct caption;
);
#[allow(non_snake_case)]
pub mod HtmlDataElementProps;
pub use HtmlDataElementProps::HtmlDataElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlDataElementProps;
    type Element = web_sys::HtmlDataElement;
    pub struct data;
);
#[allow(non_snake_case)]
pub mod HtmlModElementProps;
pub use HtmlModElementProps::HtmlModElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlModElementProps;
    type Element = web_sys::HtmlModElement;
    pub struct del;
    pub struct ins;
);
#[allow(non_snake_case)]
pub mod HtmlDetailsElementProps;
pub use HtmlDetailsElementProps::HtmlDetailsElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlDetailsElementProps;
    type Element = web_sys::HtmlDetailsElement;
    pub struct details;
);
#[allow(non_snake_case)]
pub mod HtmlDialogElementProps;
pub use HtmlDialogElementProps::HtmlDialogElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlDialogElementProps;
    type Element = web_sys::HtmlDialogElement;
    pub struct dialog;
);
#[allow(non_snake_case)]
pub mod HtmlEmbedElementProps;
pub use HtmlEmbedElementProps::HtmlEmbedElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlEmbedElementProps;
    type Element = web_sys::HtmlEmbedElement;
    pub struct embed;
);
#[allow(non_snake_case)]
pub mod HtmlFieldSetElementProps;
pub use HtmlFieldSetElementProps::HtmlFieldSetElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlFieldSetElementProps;
    type Element = web_sys::HtmlFieldSetElement;
    pub struct fieldset;
);
#[allow(non_snake_case)]
pub mod HtmlFormElementProps;
pub use HtmlFormElementProps::HtmlFormElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlFormElementProps;
    type Element = web_sys::HtmlFormElement;
    pub struct form;
);
#[allow(non_snake_case)]
pub mod HtmlHtmlElementProps;
pub use HtmlHtmlElementProps::HtmlHtmlElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlHtmlElementProps;
    type Element = web_sys::HtmlHtmlElement;
    pub struct html;
);
#[allow(non_snake_case)]
pub mod HtmlIFrameElementProps;
pub use HtmlIFrameElementProps::HtmlIFrameElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlIFrameElementProps;
    type Element = web_sys::HtmlIFrameElement;
    pub struct iframe;
);
#[allow(non_snake_case)]
pub mod HtmlImageElementProps;
pub use HtmlImageElementProps::HtmlImageElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlImageElementProps;
    type Element = web_sys::HtmlImageElement;
    pub struct img;
);
#[allow(non_snake_case)]
pub mod HtmlInputElementProps;
pub use HtmlInputElementProps::HtmlInputElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlInputElementProps;
    type Element = web_sys::HtmlInputElement;
    pub struct input;
);
#[allow(non_snake_case)]
pub mod HtmlLabelElementProps;
pub use HtmlLabelElementProps::HtmlLabelElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlLabelElementProps;
    type Element = web_sys::HtmlLabelElement;
    pub struct label;
);
#[allow(non_snake_case)]
pub mod HtmlLiElementProps;
pub use HtmlLiElementProps::HtmlLiElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlLiElementProps;
    type Element = web_sys::HtmlLiElement;
    pub struct li;
);
#[allow(non_snake_case)]
pub mod HtmlLinkElementProps;
pub use HtmlLinkElementProps::HtmlLinkElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlLinkElementProps;
    type Element = web_sys::HtmlLinkElement;
    pub struct link;
);
#[allow(non_snake_case)]
pub mod HtmlMapElementProps;
pub use HtmlMapElementProps::HtmlMapElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlMapElementProps;
    type Element = web_sys::HtmlMapElement;
    pub struct map;
);
#[allow(non_snake_case)]
pub mod HtmlMetaElementProps;
pub use HtmlMetaElementProps::HtmlMetaElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlMetaElementProps;
    type Element = web_sys::HtmlMetaElement;
    pub struct meta;
);
#[allow(non_snake_case)]
pub mod HtmlMeterElementProps;
pub use HtmlMeterElementProps::HtmlMeterElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlMeterElementProps;
    type Element = web_sys::HtmlMeterElement;
    pub struct meter;
);
#[allow(non_snake_case)]
pub mod HtmlObjectElementProps;
pub use HtmlObjectElementProps::HtmlObjectElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlObjectElementProps;
    type Element = web_sys::HtmlObjectElement;
    pub struct object;
);
#[allow(non_snake_case)]
pub mod HtmlOListElementProps;
pub use HtmlOListElementProps::HtmlOListElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlOListElementProps;
    type Element = web_sys::HtmlOListElement;
    pub struct ol;
);
#[allow(non_snake_case)]
pub mod HtmlOptGroupElementProps;
pub use HtmlOptGroupElementProps::HtmlOptGroupElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlOptGroupElementProps;
    type Element = web_sys::HtmlOptGroupElement;
    pub struct optgroup;
);
#[allow(non_snake_case)]
pub mod HtmlOptionElementProps;
pub use HtmlOptionElementProps::HtmlOptionElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlOptionElementProps;
    type Element = web_sys::HtmlOptionElement;
    pub struct option;
);
#[allow(non_snake_case)]
pub mod HtmlOutputElementProps;
pub use HtmlOutputElementProps::HtmlOutputElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlOutputElementProps;
    type Element = web_sys::HtmlOutputElement;
    pub struct output;
);
#[allow(non_snake_case)]
pub mod HtmlProgressElementProps;
pub use HtmlProgressElementProps::HtmlProgressElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlProgressElementProps;
    type Element = web_sys::HtmlProgressElement;
    pub struct progress;
);
#[allow(non_snake_case)]
pub mod HtmlScriptElementProps;
pub use HtmlScriptElementProps::HtmlScriptElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlScriptElementProps;
    type Element = web_sys::HtmlScriptElement;
    pub struct script;
);
#[allow(non_snake_case)]
pub mod HtmlSelectElementProps;
pub use HtmlSelectElementProps::HtmlSelectElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlSelectElementProps;
    type Element = web_sys::HtmlSelectElement;
    pub struct select;
);
#[allow(non_snake_case)]
pub mod HtmlSlotElementProps;
pub use HtmlSlotElementProps::HtmlSlotElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlSlotElementProps;
    type Element = web_sys::HtmlSlotElement;
    pub struct slot;
);
#[allow(non_snake_case)]
pub mod HtmlSourceElementProps;
pub use HtmlSourceElementProps::HtmlSourceElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlSourceElementProps;
    type Element = web_sys::HtmlSourceElement;
    pub struct source;
);
#[allow(non_snake_case)]
pub mod HtmlStyleElementProps;
pub use HtmlStyleElementProps::HtmlStyleElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlStyleElementProps;
    type Element = web_sys::HtmlStyleElement;
    pub struct style;
);
#[allow(non_snake_case)]
pub mod HtmlTableElementProps;
pub use HtmlTableElementProps::HtmlTableElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlTableElementProps;
    type Element = web_sys::HtmlTableElement;
    pub struct table;
);
#[allow(non_snake_case)]
pub mod HtmlTableSectionElementProps;
pub use HtmlTableSectionElementProps::HtmlTableSectionElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlTableSectionElementProps;
    type Element = web_sys::HtmlTableSectionElement;
    pub struct tbody;
    pub struct tfoot;
    pub struct thead;
);
#[allow(non_snake_case)]
pub mod HtmlTableRowElementProps;
pub use HtmlTableRowElementProps::HtmlTableRowElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlTableRowElementProps;
    type Element = web_sys::HtmlTableRowElement;
    pub struct tr;
);
#[allow(non_snake_case)]
pub mod HtmlTableColElementProps;
pub use HtmlTableColElementProps::HtmlTableColElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlTableColElementProps;
    type Element = web_sys::HtmlTableColElement;
    pub struct col;
    pub struct colgroup;
);
#[allow(non_snake_case)]
pub mod HtmlTableCellElementProps;
pub use HtmlTableCellElementProps::HtmlTableCellElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlTableCellElementProps;
    type Element = web_sys::HtmlTableCellElement;
    pub struct td;
    pub struct th;
);
#[allow(non_snake_case)]
pub mod HtmlTextAreaElementProps;
pub use HtmlTextAreaElementProps::HtmlTextAreaElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlTextAreaElementProps;
    type Element = web_sys::HtmlTextAreaElement;
    pub struct textarea;
);
#[allow(non_snake_case)]
pub mod HtmlTimeElementProps;
pub use HtmlTimeElementProps::HtmlTimeElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlTimeElementProps;
    type Element = web_sys::HtmlTimeElement;
    pub struct time;
);
#[allow(non_snake_case)]
pub mod HtmlTrackElementProps;
pub use HtmlTrackElementProps::HtmlTrackElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlTrackElementProps;
    type Element = web_sys::HtmlTrackElement;
    pub struct track;
);
#[allow(non_snake_case)]
pub mod HtmlUListElementProps;
pub use HtmlUListElementProps::HtmlUListElementProps;
crate::imports::def_intrinsic_component_simple!(
    type Props = HtmlUListElementProps;
    type Element = web_sys::HtmlUListElement;
    pub struct ul;
);
