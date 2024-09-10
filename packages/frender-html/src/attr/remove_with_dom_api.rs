use frender_dom::behaviors::Element;

use super::SetAttributeWithDomApi;

pub(crate) trait RemoveAttributeDomApi<V> {
    fn set(self, value: V);
    fn remove(self);
}

pub(crate) struct DomApi<'a, E: ?Sized, R: ?Sized, ApiSet> {
    pub(crate) element: &'a mut E,
    pub(crate) renderer: &'a mut R,
    pub(crate) attr_name: &'a str,
    /// the api to set this attribute
    pub(crate) api_set: ApiSet,
}

impl<E: ?Sized + Element<R>, R: ?Sized, ApiSet: FnOnce(&mut E, &mut R, V), V> RemoveAttributeDomApi<V> for DomApi<'_, E, R, ApiSet> {
    fn set(self, value: V) {
        (self.api_set)(self.element, self.renderer, value)
    }

    fn remove(self) {
        self.element.remove_attribute(self.renderer, self.attr_name)
    }
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
