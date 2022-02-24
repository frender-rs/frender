use crate::HtmlCommonSharedPropsBuilder;

crate::macros::def_intrinsic_component! {
    "map"
    MapComponent(MapComponentProps) {
        MapComponentProps
        : MapComponentPropsBuilder
        : HtmlCommonSharedPropsBuilder[web_sys::HtmlMapElement]
        {
            name: Option<&str>,
        }
    }
}
