#[allow(unused_imports)]
use super::*;
#[allow(non_snake_case)]
pub mod ElementProps;
pub use ElementProps::ElementProps;
#[allow(non_snake_case)]
pub mod HtmlElementProps;
pub use HtmlElementProps::HtmlElementProps;
crate::imports::def_intrinsic_component! {
    pub abbr[address article aside b bdi bdo cite code datalist dd dfn div dl dt em
    figcaption figure footer h1 h2 h3 h4 h5 h6 head header hgroup hr i kbd legend main
    mark menu nav noscript p picture pre rp rt ruby s samp section small span strong sub
    summary sup template title u var wbr] HtmlElementProps web_sys::HtmlElement
}
#[allow(non_snake_case)]
pub mod HtmlAnchorElementProps;
pub use HtmlAnchorElementProps::HtmlAnchorElementProps;
crate::imports::def_intrinsic_component! {
    pub a[] HtmlAnchorElementProps web_sys::HtmlAnchorElement
}
#[allow(non_snake_case)]
pub mod HtmlAreaElementProps;
pub use HtmlAreaElementProps::HtmlAreaElementProps;
crate::imports::def_intrinsic_component! {
    pub area[] HtmlAreaElementProps web_sys::HtmlAreaElement
}
#[allow(non_snake_case)]
pub mod HtmlMediaElementProps;
pub use HtmlMediaElementProps::HtmlMediaElementProps;
#[allow(non_snake_case)]
pub mod HtmlAudioElementProps;
pub use HtmlAudioElementProps::HtmlAudioElementProps;
crate::imports::def_intrinsic_component! {
    pub audio[] HtmlAudioElementProps web_sys::HtmlAudioElement
}
#[allow(non_snake_case)]
pub mod HtmlVideoElementProps;
pub use HtmlVideoElementProps::HtmlVideoElementProps;
crate::imports::def_intrinsic_component! {
    pub video[] HtmlVideoElementProps web_sys::HtmlVideoElement
}
#[allow(non_snake_case)]
pub mod HtmlBaseElementProps;
pub use HtmlBaseElementProps::HtmlBaseElementProps;
crate::imports::def_intrinsic_component! {
    pub base[] HtmlBaseElementProps web_sys::HtmlBaseElement
}
#[allow(non_snake_case)]
pub mod HtmlQuoteElementProps;
pub use HtmlQuoteElementProps::HtmlQuoteElementProps;
crate::imports::def_intrinsic_component! {
    pub blockquote[q] HtmlQuoteElementProps web_sys::HtmlQuoteElement
}
#[allow(non_snake_case)]
pub mod HtmlBodyElementProps;
pub use HtmlBodyElementProps::HtmlBodyElementProps;
crate::imports::def_intrinsic_component! {
    pub body[] HtmlBodyElementProps web_sys::HtmlBodyElement
}
#[allow(non_snake_case)]
pub mod HtmlBrElementProps;
pub use HtmlBrElementProps::HtmlBrElementProps;
crate::imports::def_intrinsic_component! {
    pub br[] HtmlBrElementProps web_sys::HtmlBrElement
}
#[allow(non_snake_case)]
pub mod HtmlButtonElementProps;
pub use HtmlButtonElementProps::HtmlButtonElementProps;
crate::imports::def_intrinsic_component! {
    pub button[] HtmlButtonElementProps web_sys::HtmlButtonElement
}
#[allow(non_snake_case)]
pub mod HtmlCanvasElementProps;
pub use HtmlCanvasElementProps::HtmlCanvasElementProps;
crate::imports::def_intrinsic_component! {
    pub canvas[] HtmlCanvasElementProps web_sys::HtmlCanvasElement
}
#[allow(non_snake_case)]
pub mod HtmlTableCaptionElementProps;
pub use HtmlTableCaptionElementProps::HtmlTableCaptionElementProps;
crate::imports::def_intrinsic_component! {
    pub caption[] HtmlTableCaptionElementProps web_sys::HtmlTableCaptionElement
}
#[allow(non_snake_case)]
pub mod HtmlDataElementProps;
pub use HtmlDataElementProps::HtmlDataElementProps;
crate::imports::def_intrinsic_component! {
    pub data[] HtmlDataElementProps web_sys::HtmlDataElement
}
#[allow(non_snake_case)]
pub mod HtmlModElementProps;
pub use HtmlModElementProps::HtmlModElementProps;
crate::imports::def_intrinsic_component! {
    pub del[ins] HtmlModElementProps web_sys::HtmlModElement
}
#[allow(non_snake_case)]
pub mod HtmlDetailsElementProps;
pub use HtmlDetailsElementProps::HtmlDetailsElementProps;
crate::imports::def_intrinsic_component! {
    pub details[] HtmlDetailsElementProps web_sys::HtmlDetailsElement
}
#[allow(non_snake_case)]
pub mod HtmlDialogElementProps;
pub use HtmlDialogElementProps::HtmlDialogElementProps;
crate::imports::def_intrinsic_component! {
    pub dialog[] HtmlDialogElementProps web_sys::HtmlDialogElement
}
#[allow(non_snake_case)]
pub mod HtmlEmbedElementProps;
pub use HtmlEmbedElementProps::HtmlEmbedElementProps;
crate::imports::def_intrinsic_component! {
    pub embed[] HtmlEmbedElementProps web_sys::HtmlEmbedElement
}
#[allow(non_snake_case)]
pub mod HtmlFieldSetElementProps;
pub use HtmlFieldSetElementProps::HtmlFieldSetElementProps;
crate::imports::def_intrinsic_component! {
    pub fieldset[] HtmlFieldSetElementProps web_sys::HtmlFieldSetElement
}
#[allow(non_snake_case)]
pub mod HtmlFormElementProps;
pub use HtmlFormElementProps::HtmlFormElementProps;
crate::imports::def_intrinsic_component! {
    pub form[] HtmlFormElementProps web_sys::HtmlFormElement
}
#[allow(non_snake_case)]
pub mod HtmlHtmlElementProps;
pub use HtmlHtmlElementProps::HtmlHtmlElementProps;
crate::imports::def_intrinsic_component! {
    pub html[] HtmlHtmlElementProps web_sys::HtmlHtmlElement
}
#[allow(non_snake_case)]
pub mod HtmlIFrameElementProps;
pub use HtmlIFrameElementProps::HtmlIFrameElementProps;
crate::imports::def_intrinsic_component! {
    pub iframe[] HtmlIFrameElementProps web_sys::HtmlIFrameElement
}
#[allow(non_snake_case)]
pub mod HtmlImageElementProps;
pub use HtmlImageElementProps::HtmlImageElementProps;
crate::imports::def_intrinsic_component! {
    pub img[] HtmlImageElementProps web_sys::HtmlImageElement
}
#[allow(non_snake_case)]
pub mod HtmlInputElementProps;
pub use HtmlInputElementProps::HtmlInputElementProps;
crate::imports::def_intrinsic_component! {
    pub input[] HtmlInputElementProps web_sys::HtmlInputElement
}
#[allow(non_snake_case)]
pub mod HtmlLabelElementProps;
pub use HtmlLabelElementProps::HtmlLabelElementProps;
crate::imports::def_intrinsic_component! {
    pub label[] HtmlLabelElementProps web_sys::HtmlLabelElement
}
#[allow(non_snake_case)]
pub mod HtmlLiElementProps;
pub use HtmlLiElementProps::HtmlLiElementProps;
crate::imports::def_intrinsic_component! {
    pub li[] HtmlLiElementProps web_sys::HtmlLiElement
}
#[allow(non_snake_case)]
pub mod HtmlLinkElementProps;
pub use HtmlLinkElementProps::HtmlLinkElementProps;
crate::imports::def_intrinsic_component! {
    pub link[] HtmlLinkElementProps web_sys::HtmlLinkElement
}
#[allow(non_snake_case)]
pub mod HtmlMapElementProps;
pub use HtmlMapElementProps::HtmlMapElementProps;
crate::imports::def_intrinsic_component! {
    pub map[] HtmlMapElementProps web_sys::HtmlMapElement
}
#[allow(non_snake_case)]
pub mod HtmlMetaElementProps;
pub use HtmlMetaElementProps::HtmlMetaElementProps;
crate::imports::def_intrinsic_component! {
    pub meta[] HtmlMetaElementProps web_sys::HtmlMetaElement
}
#[allow(non_snake_case)]
pub mod HtmlMeterElementProps;
pub use HtmlMeterElementProps::HtmlMeterElementProps;
crate::imports::def_intrinsic_component! {
    pub meter[] HtmlMeterElementProps web_sys::HtmlMeterElement
}
#[allow(non_snake_case)]
pub mod HtmlObjectElementProps;
pub use HtmlObjectElementProps::HtmlObjectElementProps;
crate::imports::def_intrinsic_component! {
    pub object[] HtmlObjectElementProps web_sys::HtmlObjectElement
}
#[allow(non_snake_case)]
pub mod HtmlOListElementProps;
pub use HtmlOListElementProps::HtmlOListElementProps;
crate::imports::def_intrinsic_component! {
    pub ol[] HtmlOListElementProps web_sys::HtmlOListElement
}
#[allow(non_snake_case)]
pub mod HtmlOptGroupElementProps;
pub use HtmlOptGroupElementProps::HtmlOptGroupElementProps;
crate::imports::def_intrinsic_component! {
    pub optgroup[] HtmlOptGroupElementProps web_sys::HtmlOptGroupElement
}
#[allow(non_snake_case)]
pub mod HtmlOptionElementProps;
pub use HtmlOptionElementProps::HtmlOptionElementProps;
crate::imports::def_intrinsic_component! {
    pub option[] HtmlOptionElementProps web_sys::HtmlOptionElement
}
#[allow(non_snake_case)]
pub mod HtmlOutputElementProps;
pub use HtmlOutputElementProps::HtmlOutputElementProps;
crate::imports::def_intrinsic_component! {
    pub output[] HtmlOutputElementProps web_sys::HtmlOutputElement
}
#[allow(non_snake_case)]
pub mod HtmlProgressElementProps;
pub use HtmlProgressElementProps::HtmlProgressElementProps;
crate::imports::def_intrinsic_component! {
    pub progress[] HtmlProgressElementProps web_sys::HtmlProgressElement
}
#[allow(non_snake_case)]
pub mod HtmlScriptElementProps;
pub use HtmlScriptElementProps::HtmlScriptElementProps;
crate::imports::def_intrinsic_component! {
    pub script[] HtmlScriptElementProps web_sys::HtmlScriptElement
}
#[allow(non_snake_case)]
pub mod HtmlSelectElementProps;
pub use HtmlSelectElementProps::HtmlSelectElementProps;
crate::imports::def_intrinsic_component! {
    pub select[] HtmlSelectElementProps web_sys::HtmlSelectElement
}
#[allow(non_snake_case)]
pub mod HtmlSlotElementProps;
pub use HtmlSlotElementProps::HtmlSlotElementProps;
crate::imports::def_intrinsic_component! {
    pub slot[] HtmlSlotElementProps web_sys::HtmlSlotElement
}
#[allow(non_snake_case)]
pub mod HtmlSourceElementProps;
pub use HtmlSourceElementProps::HtmlSourceElementProps;
crate::imports::def_intrinsic_component! {
    pub source[] HtmlSourceElementProps web_sys::HtmlSourceElement
}
#[allow(non_snake_case)]
pub mod HtmlStyleElementProps;
pub use HtmlStyleElementProps::HtmlStyleElementProps;
crate::imports::def_intrinsic_component! {
    pub style[] HtmlStyleElementProps web_sys::HtmlStyleElement
}
#[allow(non_snake_case)]
pub mod HtmlTableElementProps;
pub use HtmlTableElementProps::HtmlTableElementProps;
crate::imports::def_intrinsic_component! {
    pub table[] HtmlTableElementProps web_sys::HtmlTableElement
}
#[allow(non_snake_case)]
pub mod HtmlTableSectionElementProps;
pub use HtmlTableSectionElementProps::HtmlTableSectionElementProps;
crate::imports::def_intrinsic_component! {
    pub tbody[tfoot thead] HtmlTableSectionElementProps web_sys::HtmlTableSectionElement
}
#[allow(non_snake_case)]
pub mod HtmlTableRowElementProps;
pub use HtmlTableRowElementProps::HtmlTableRowElementProps;
crate::imports::def_intrinsic_component! {
    pub tr[] HtmlTableRowElementProps web_sys::HtmlTableRowElement
}
#[allow(non_snake_case)]
pub mod HtmlTableColElementProps;
pub use HtmlTableColElementProps::HtmlTableColElementProps;
crate::imports::def_intrinsic_component! {
    pub col[colgroup] HtmlTableColElementProps web_sys::HtmlTableColElement
}
#[allow(non_snake_case)]
pub mod HtmlTableCellElementProps;
pub use HtmlTableCellElementProps::HtmlTableCellElementProps;
crate::imports::def_intrinsic_component! {
    pub td[th] HtmlTableCellElementProps web_sys::HtmlTableCellElement
}
#[allow(non_snake_case)]
pub mod HtmlTextAreaElementProps;
pub use HtmlTextAreaElementProps::HtmlTextAreaElementProps;
crate::imports::def_intrinsic_component! {
    pub textarea[] HtmlTextAreaElementProps web_sys::HtmlTextAreaElement
}
#[allow(non_snake_case)]
pub mod HtmlTimeElementProps;
pub use HtmlTimeElementProps::HtmlTimeElementProps;
crate::imports::def_intrinsic_component! {
    pub time[] HtmlTimeElementProps web_sys::HtmlTimeElement
}
#[allow(non_snake_case)]
pub mod HtmlTrackElementProps;
pub use HtmlTrackElementProps::HtmlTrackElementProps;
crate::imports::def_intrinsic_component! {
    pub track[] HtmlTrackElementProps web_sys::HtmlTrackElement
}
#[allow(non_snake_case)]
pub mod HtmlUListElementProps;
pub use HtmlUListElementProps::HtmlUListElementProps;
crate::imports::def_intrinsic_component! {
    pub ul[] HtmlUListElementProps web_sys::HtmlUListElement
}
