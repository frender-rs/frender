use bg::Maybe;
use wasm_bindgen::JsCast;

pub trait ElementTypeTag {
    const TAG: &'static str;
}

pub trait UpdateElementProperties<Element> {
    fn update_property_checked(element: &Element, value: impl Maybe<bool>);
    fn update_property_default_checked(element: &Element, value: impl Maybe<bool>);
}

pub trait ElementTypeMarker: ElementTypeTag + UpdateElementProperties<Self::Element> {
    type Element: JsCast + AsRef<web_sys::Element>;

    fn create_element(document: &web_sys::Document) -> Self::Element {
        let element = document.create_element(Self::TAG).unwrap();
        element.dyn_into::<Self::Element>().unwrap()
    }

    fn mount_element_and_update(
        node_and_mounted: &mut Option<(Self::Element, bool)>,
        ctx: &mut frender_dom::Dom,
        update: impl FnOnce(&mut Self::Element, &mut frender_dom::Dom),
    ) {
        crate::utils::dom::insert_element_and_update(node_and_mounted, ctx, Self::TAG, update)
    }
}

pub enum HtmlElement {}
