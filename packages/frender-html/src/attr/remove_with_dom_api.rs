use frender_dom::behaviors::Element;

use super::SetAttributeWithDomApi;

pub(crate) trait RemoveAttributeDomApi<V> {
    fn set(self, value: V);
    fn remove(self);
}

pub(crate) trait RemoveAttributeWithDomApi: SetAttributeWithDomApi {
    fn remove_attribute_with_dom_api(api: impl for<'v> RemoveAttributeDomApi<Self::DomApiValue<'v>>);
}

impl RemoveAttributeWithDomApi for bool {
    fn remove_attribute_with_dom_api(api: impl for<'v> RemoveAttributeDomApi<Self::DomApiValue<'v>>) {
        api.set(false)
    }
}

trait SimpleRemove: SetAttributeWithDomApi {}

impl SimpleRemove for str {}
impl SimpleRemove for i32 {}
impl SimpleRemove for u32 {}
impl SimpleRemove for f64 {}

impl<T: ?Sized + SimpleRemove> RemoveAttributeWithDomApi for T {
    fn remove_attribute_with_dom_api(api: impl for<'v> RemoveAttributeDomApi<Self::DomApiValue<'v>>) {
        api.remove()
    }
}
