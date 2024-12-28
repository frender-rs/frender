macro_rules! HtmlElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { Element Node } $($append)* }
    };
}
pub(crate) use HtmlElement;
macro_rules! HtmlDataListElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node } $($append)* }
    };
}
pub(crate) use HtmlDataListElement;
macro_rules! HtmlDivElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node } $($append)* }
    };
}
pub(crate) use HtmlDivElement;
macro_rules! HtmlDListElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node } $($append)* }
    };
}
pub(crate) use HtmlDListElement;
macro_rules! HtmlHeadingElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node } $($append)* }
    };
}
pub(crate) use HtmlHeadingElement;
macro_rules! HtmlHeadElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node } $($append)* }
    };
}
pub(crate) use HtmlHeadElement;
macro_rules! HtmlHrElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node } $($append)* }
    };
}
pub(crate) use HtmlHrElement;
macro_rules! HtmlLegendElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node } $($append)* }
    };
}
pub(crate) use HtmlLegendElement;
macro_rules! HtmlMenuElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node } $($append)* }
    };
}
pub(crate) use HtmlMenuElement;
macro_rules! HtmlParagraphElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node } $($append)* }
    };
}
pub(crate) use HtmlParagraphElement;
macro_rules! HtmlPictureElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node } $($append)* }
    };
}
pub(crate) use HtmlPictureElement;
macro_rules! HtmlPreElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node } $($append)* }
    };
}
pub(crate) use HtmlPreElement;
macro_rules! HtmlSpanElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node } $($append)* }
    };
}
pub(crate) use HtmlSpanElement;
macro_rules! HtmlTemplateElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node } $($append)* }
    };
}
pub(crate) use HtmlTemplateElement;
macro_rules! HtmlTitleElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node } $($append)* }
    };
}
pub(crate) use HtmlTitleElement;
macro_rules! HtmlAnchorElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node HtmlElementWithHref
        ElementWithTypeAttribute ElementWithHrefLangAttribute ElementWithHrefAttribute
        ElementWithTargetAttribute ElementWithReferrerPolicyAttribute
        ElementWithRelAttribute } $($append)* }
    };
}
pub(crate) use HtmlAnchorElement;
macro_rules! HtmlAreaElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node HtmlElementWithHref
        ElementWithAltAttribute ElementWithHrefAttribute ElementWithTargetAttribute
        ElementWithReferrerPolicyAttribute ElementWithRelAttribute } $($append)* }
    };
}
pub(crate) use HtmlAreaElement;
macro_rules! HtmlBaseElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node ElementWithHrefAttribute
        ElementWithTargetAttribute } $($append)* }
    };
}
pub(crate) use HtmlBaseElement;
macro_rules! HtmlQuoteElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node ElementWithCiteAttribute }
        $($append)* }
    };
}
pub(crate) use HtmlQuoteElement;
macro_rules! HtmlBodyElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node } $($append)* }
    };
}
pub(crate) use HtmlBodyElement;
macro_rules! HtmlBrElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node } $($append)* }
    };
}
pub(crate) use HtmlBrElement;
macro_rules! HtmlButtonElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node ElementWithTypeAttribute
        ElementWithFormAttributes ElementWithDisabledAttribute ElementWithNameAttribute
        ElementWithValueStrAttribute ElementWithFormAttribute } $($append)* }
    };
}
pub(crate) use HtmlButtonElement;
macro_rules! HtmlCanvasElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node
        ElementWithHeightWidthU32Attributes } $($append)* }
    };
}
pub(crate) use HtmlCanvasElement;
macro_rules! HtmlTableCaptionElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node ElementWithAlignAttribute }
        $($append)* }
    };
}
pub(crate) use HtmlTableCaptionElement;
macro_rules! HtmlDataElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node ElementWithValueStrAttribute
        } $($append)* }
    };
}
pub(crate) use HtmlDataElement;
macro_rules! HtmlModElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node ElementWithCiteAttribute
        ElementWithDateTimeAttribute } $($append)* }
    };
}
pub(crate) use HtmlModElement;
macro_rules! HtmlDetailsElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node ElementWithOpenAttribute }
        $($append)* }
    };
}
pub(crate) use HtmlDetailsElement;
macro_rules! HtmlDialogElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node ElementWithOpenAttribute }
        $($append)* }
    };
}
pub(crate) use HtmlDialogElement;
macro_rules! HtmlEmbedElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node ElementWithTypeAttribute
        ElementWithSrcAttribute ElementWithHeightWidthStrAttributes } $($append)* }
    };
}
pub(crate) use HtmlEmbedElement;
macro_rules! HtmlFieldSetElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node ElementWithFormAttribute
        ElementWithDisabledAttribute ElementWithNameAttribute } $($append)* }
    };
}
pub(crate) use HtmlFieldSetElement;
macro_rules! HtmlFormElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node ElementWithTargetAttribute
        ElementWithAutoCompleteAttribute ElementWithAcceptAttribute
        ElementWithRelAttribute ElementWithNameAttribute } $($append)* }
    };
}
pub(crate) use HtmlFormElement;
macro_rules! HtmlHtmlElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node } $($append)* }
    };
}
pub(crate) use HtmlHtmlElement;
macro_rules! HtmlIFrameElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node ElementWithSrcAttribute
        ElementWithFetchPriorityAttribute ElementWithLoadingAttribute
        ElementWithReferrerPolicyAttribute ElementWithNameAttribute
        ElementWithHeightWidthStrAttributes } $($append)* }
    };
}
pub(crate) use HtmlIFrameElement;
macro_rules! HtmlImageElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node ElementWithSrcsetAttribute
        ElementWithUseMapAttribute ElementWithSizesAttribute ElementWithLoadingAttribute
        ElementWithAltAttribute ElementWithReferrerPolicyAttribute
        ElementWithCrossOriginAttribute ElementWithHeightWidthU32Attributes
        ElementWithSrcAttribute } $($append)* }
    };
}
pub(crate) use HtmlImageElement;
macro_rules! HtmlInputElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node ElementWithReadOnlyAttribute
        ElementWithPlaceHolderAttribute ElementWithMaxMinLengthAttributes
        ElementWithSrcAttribute ElementWithSizeU32Attribute ElementWithRequiredAttribute
        ElementWithMultipleAttribute ElementWithFormAttributes
        ElementWithAutoCompleteAttribute ElementWithAutoCorrectAttribute
        ElementWithAcceptAttribute ElementWithAltAttribute ElementWithDisabledAttribute
        ElementWithNameAttribute ElementWithHeightWidthU32Attributes
        ElementWithFormAttribute } $($append)* }
    };
}
pub(crate) use HtmlInputElement;
macro_rules! HtmlLabelElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node ElementWithForAttribute }
        $($append)* }
    };
}
pub(crate) use HtmlLabelElement;
macro_rules! HtmlLiElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node } $($append)* }
    };
}
pub(crate) use HtmlLiElement;
macro_rules! HtmlLinkElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node ElementWithHrefAttribute
        ElementWithTypeAttribute ElementWithMediaAttribute ElementWithBlockingAttribute
        ElementWithIntegrityAttribute ElementWithSizesAttribute
        ElementWithHrefLangAttribute ElementWithFetchPriorityAttribute
        ElementWithReferrerPolicyAttribute ElementWithRelAttribute
        ElementWithCrossOriginAttribute } $($append)* }
    };
}
pub(crate) use HtmlLinkElement;
macro_rules! HtmlMapElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node ElementWithNameAttribute }
        $($append)* }
    };
}
pub(crate) use HtmlMapElement;
macro_rules! HtmlMetaElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node ElementWithNameAttribute }
        $($append)* }
    };
}
pub(crate) use HtmlMetaElement;
macro_rules! HtmlMeterElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node ElementWithMaxF64Attribute
        ElementWithValueF64Attribute } $($append)* }
    };
}
pub(crate) use HtmlMeterElement;
macro_rules! HtmlObjectElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node ElementWithTypeAttribute
        ElementWithUseMapAttribute ElementWithFormAttribute ElementWithNameAttribute
        ElementWithHeightWidthStrAttributes } $($append)* }
    };
}
pub(crate) use HtmlObjectElement;
macro_rules! HtmlOListElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node ElementWithTypeAttribute }
        $($append)* }
    };
}
pub(crate) use HtmlOListElement;
macro_rules! HtmlOptGroupElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node ElementWithLabelAttribute
        ElementWithDisabledAttribute } $($append)* }
    };
}
pub(crate) use HtmlOptGroupElement;
macro_rules! HtmlOptionElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node ElementWithLabelAttribute
        ElementWithDisabledAttribute ElementWithValueStrAttribute } $($append)* }
    };
}
pub(crate) use HtmlOptionElement;
macro_rules! HtmlOutputElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node ElementWithForAttribute
        ElementWithFormAttribute ElementWithNameAttribute } $($append)* }
    };
}
pub(crate) use HtmlOutputElement;
macro_rules! HtmlProgressElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node ElementWithMaxF64Attribute
        ElementWithValueF64Attribute } $($append)* }
    };
}
pub(crate) use HtmlProgressElement;
macro_rules! HtmlScriptElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node ElementWithTypeAttribute
        ElementWithSrcAttribute ElementWithBlockingAttribute
        ElementWithIntegrityAttribute ElementWithFetchPriorityAttribute
        ElementWithReferrerPolicyAttribute ElementWithCrossOriginAttribute } $($append)*
        }
    };
}
pub(crate) use HtmlScriptElement;
macro_rules! HtmlSelectElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node ElementWithSizeU32Attribute
        ElementWithRequiredAttribute ElementWithMultipleAttribute
        ElementWithFormAttribute ElementWithAutoCompleteAttribute
        ElementWithDisabledAttribute ElementWithNameAttribute } $($append)* }
    };
}
pub(crate) use HtmlSelectElement;
macro_rules! HtmlSlotElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node ElementWithNameAttribute }
        $($append)* }
    };
}
pub(crate) use HtmlSlotElement;
macro_rules! HtmlSourceElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node ElementWithTypeAttribute
        ElementWithMediaAttribute ElementWithSrcsetAttribute ElementWithSizesAttribute
        ElementWithHeightWidthU32Attributes ElementWithSrcAttribute } $($append)* }
    };
}
pub(crate) use HtmlSourceElement;
macro_rules! HtmlStyleElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node ElementWithTypeAttribute
        ElementWithMediaAttribute ElementWithBlockingAttribute } $($append)* }
    };
}
pub(crate) use HtmlStyleElement;
macro_rules! HtmlTableElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node ElementWithAlignAttribute
        ElementWithBgColorAttribute } $($append)* }
    };
}
pub(crate) use HtmlTableElement;
macro_rules! HtmlTableSectionElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node HtmlTableChildElement
        ElementWithAlignAttribute ElementWithBgColorAttribute } $($append)* }
    };
}
pub(crate) use HtmlTableSectionElement;
macro_rules! HtmlTableRowElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node HtmlTableChildElement
        ElementWithAlignAttribute ElementWithBgColorAttribute } $($append)* }
    };
}
pub(crate) use HtmlTableRowElement;
macro_rules! HtmlTableColElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node HtmlTableChildElement
        ElementWithAlignAttribute ElementWithBgColorAttribute } $($append)* }
    };
}
pub(crate) use HtmlTableColElement;
macro_rules! HtmlTableCellElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node
        ElementWithHeightWidthStrAttributes HtmlTableChildElement
        ElementWithAlignAttribute ElementWithBgColorAttribute } $($append)* }
    };
}
pub(crate) use HtmlTableCellElement;
macro_rules! HtmlTextAreaElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node ElementWithReadOnlyAttribute
        ElementWithPlaceHolderAttribute ElementWithMaxMinLengthAttributes
        ElementWithRequiredAttribute ElementWithFormAttribute
        ElementWithAutoCompleteAttribute ElementWithAutoCorrectAttribute
        ElementWithDisabledAttribute ElementWithNameAttribute } $($append)* }
    };
}
pub(crate) use HtmlTextAreaElement;
macro_rules! HtmlTimeElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node ElementWithDateTimeAttribute
        } $($append)* }
    };
}
pub(crate) use HtmlTimeElement;
macro_rules! HtmlTrackElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node ElementWithSrcAttribute
        ElementWithLabelAttribute } $($append)* }
    };
}
pub(crate) use HtmlTrackElement;
macro_rules! HtmlUListElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlElement Element Node ElementWithTypeAttribute }
        $($append)* }
    };
}
pub(crate) use HtmlUListElement;
macro_rules! HtmlAudioElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlMediaElement HtmlElement Element Node
        ElementWithSrcAttribute ElementWithCrossOriginAttribute } $($append)* }
    };
}
pub(crate) use HtmlAudioElement;
macro_rules! HtmlVideoElement {
    ([$($call:tt)+] { $($prepend:tt)* } { $($append:tt)* }) => {
        $($call)+ { $($prepend)* { HtmlMediaElement HtmlElement Element Node
        ElementWithHeightWidthU32Attributes ElementWithSrcAttribute
        ElementWithCrossOriginAttribute } $($append)* }
    };
}
pub(crate) use HtmlVideoElement;
