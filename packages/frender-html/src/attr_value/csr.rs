use std::marker::PhantomData;

use frender_attr_value::{
    csr::{CsrAttrValue, UpdateAttrValue},
    html::{AttrKindOfContentEditable, Spellcheck},
    AttrKindOfStr, AttrValueKind,
};
use frender_common::convert::FromMut as _;
use frender_dom::csr::behaviors::Element as _;

use crate::{
    csr::behavior_type::{BehaviorType, UnpinnedNonReactiveRenderStateKind, UnpinnedRenderWithBehavior},
    csr::property_common::RemoveAttrOfBehaviorType,
    has_const_attr_name::HasConstAttrName,
    html::{behavior_type_traits, behaviors, RenderHtml},
};

use super::{HasAttrValueKind, Property};

enum Never {}
pub struct Kind<PM, S>(Never, PhantomData<(PM, S)>);

impl<PM, S> UnpinnedNonReactiveRenderStateKind for Kind<PM, S> {
    type UnpinnedNonReactiveState<R: ?Sized + RenderHtml> = S;
}

pub(crate) trait UpdateAttrValueOfBehaviorType<BT: BehaviorType>: HasAttrValueKind {
    fn update_attr_value_of_behavior_type<R: ?Sized + RenderHtml>(
        //
        b: &mut BT::OfBehaviorType<R>,
        renderer: &mut R,
        value: <Self::AttrValueKind as AttrValueKind>::AttrValue<'_>,
    );
}

pub(crate) trait UseSpecUpdateAttrValueOfBehaviorType: HasAttrValueKind {}
pub(crate) trait HasSpecUpdateAttrValueOfBehaviorType<BT: BehaviorType>: UseSpecUpdateAttrValueOfBehaviorType {
    type SpecUpdateAttrValueOfBehaviorType: UpdateAttrValueOfBehaviorType<BT, AttrValueKind = Self::AttrValueKind>;
}

impl<T, BT> UpdateAttrValueOfBehaviorType<BT> for T
where
    BT: BehaviorType,
    T: ?Sized + HasSpecUpdateAttrValueOfBehaviorType<BT> + HasAttrValueKind + UseSpecUpdateAttrValueOfBehaviorType,
{
    fn update_attr_value_of_behavior_type<R: ?Sized + RenderHtml>(
        //
        b: &mut <BT as BehaviorType>::OfBehaviorType<R>,
        renderer: &mut R,
        value: <Self::AttrValueKind as AttrValueKind>::AttrValue<'_>,
    ) {
        <T::SpecUpdateAttrValueOfBehaviorType>::update_attr_value_of_behavior_type(b, renderer, value)
    }
}

struct Updater<'a, PM: ?Sized, R: ?Sized + RenderHtml, BT: BehaviorType> {
    _prop_marker: PhantomData<PM>,
    renderer: &'a mut R,
    b: &'a mut BT::OfBehaviorType<R>,
}

impl<
        //
        PM: ?Sized + UpdateAttrValueOfBehaviorType<BT> + RemoveAttrOfBehaviorType<BT>,
        R: ?Sized + RenderHtml,
        BT: BehaviorType,
    > UpdateAttrValue for Updater<'_, PM, R, BT>
{
    type Kind = PM::AttrValueKind;

    fn set(self, value: <Self::Kind as AttrValueKind>::AttrValue<'_>) {
        PM::update_attr_value_of_behavior_type(self.b, self.renderer, value)
    }

    fn remove(self) {
        PM::remove_attr_of_behavior_type(self.b, self.renderer);
    }
}

impl<
        //
        BT: BehaviorType,
        PM: UpdateAttrValueOfBehaviorType<BT> + RemoveAttrOfBehaviorType<BT>,
        V: CsrAttrValue<PM::AttrValueKind>,
    > UnpinnedRenderWithBehavior<BT> for Property<PM, V>
{
    type UnpinnedRenderStateKind = Kind<PM, V::State>;

    fn unpinned_render_init_with_behavior<R: ?Sized + RenderHtml>(
        //
        Self { _prop_marker, value }: Self,
        renderer: &mut R,
        b: &mut <BT as BehaviorType>::OfBehaviorType<R>,
    ) -> V::State {
        V::update_absent_attribute_value_into_state(value, Updater { _prop_marker, renderer, b })
    }

    fn unpinned_render_update_with_behavior<R: ?Sized + RenderHtml>(
        //
        Self { _prop_marker, value }: Self,
        renderer: &mut R,
        b: &mut <BT as BehaviorType>::OfBehaviorType<R>,
        state: &mut <Self::UnpinnedRenderStateKind as UnpinnedNonReactiveRenderStateKind>::UnpinnedNonReactiveState<R>,
    ) {
        V::update_attribute_value_with_state(value, Updater { _prop_marker, renderer, b }, state)
    }
}

// region: HasDomApi
pub(crate) trait HasDomApi<BT: BehaviorType>: HasAttrValueKind {
    type DomApiValue<'a>;

    fn dom_api_value_from_value(value: <Self::AttrValueKind as AttrValueKind>::AttrValue<'_>) -> Self::DomApiValue<'_>;

    fn set_attribute_value<R: ?Sized + RenderHtml>(b: &mut BT::OfBehaviorType<R>, renderer: &mut R, value: Self::DomApiValue<'_>);
}
// endregion
// region: remove attr with dom api
pub(crate) struct SpecRemoveAttrWithDomApi<U: ?Sized>(U);

pub(crate) trait AttrValueKindRemoveAttrWithDomApi<BT: BehaviorType, U: ?Sized + HasDomApi<BT, AttrValueKind = Self>>: AttrValueKind {
    fn remove_attr_with_dom_api<R: ?Sized + RenderHtml>(b: &mut BT::OfBehaviorType<R>, renderer: &mut R);
}

impl<U: ?Sized, BT: BehaviorType> RemoveAttrOfBehaviorType<BT> for SpecRemoveAttrWithDomApi<U>
where
    U: HasDomApi<BT>,
    U::AttrValueKind: AttrValueKindRemoveAttrWithDomApi<BT, U>,
{
    fn remove_attr_of_behavior_type<R: ?Sized + RenderHtml>(
        //
        b: &mut <BT as BehaviorType>::OfBehaviorType<R>,
        renderer: &mut R,
    ) {
        <U::AttrValueKind>::remove_attr_with_dom_api(b, renderer)
    }
}

/// For bool, to remove attr is to set attr as false
impl<BT: BehaviorType, U: ?Sized + HasDomApi<BT, AttrValueKind = Self>> AttrValueKindRemoveAttrWithDomApi<BT, U> for bool
where
    for<'a> U: HasDomApi<BT, DomApiValue<'a> = bool>,
{
    fn remove_attr_with_dom_api<R: ?Sized + RenderHtml>(b: &mut <BT as BehaviorType>::OfBehaviorType<R>, renderer: &mut R) {
        U::set_attribute_value(b, renderer, false)
    }
}

trait SimpleAttrValueKindRemoveAttr: AttrValueKind {}
impl SimpleAttrValueKindRemoveAttr for Spellcheck {}
impl SimpleAttrValueKindRemoveAttr for AttrKindOfContentEditable {}
impl SimpleAttrValueKindRemoveAttr for AttrKindOfStr {}
impl SimpleAttrValueKindRemoveAttr for i32 {}
impl SimpleAttrValueKindRemoveAttr for u32 {}
impl SimpleAttrValueKindRemoveAttr for f64 {}

/// For simple kinds, to remove attr is to remove attr by attr name.
impl<VK: ?Sized, ET, U: ?Sized + HasDomApi<ET, AttrValueKind = Self>> AttrValueKindRemoveAttrWithDomApi<ET, U> for VK
where
    VK: SimpleAttrValueKindRemoveAttr,
    ET: behavior_type_traits::Element,
    U: HasConstAttrName,
{
    fn remove_attr_with_dom_api<R: ?Sized + RenderHtml>(b: &mut <ET as BehaviorType>::OfBehaviorType<R>, renderer: &mut R) {
        <ET::Element<R>>::from_mut(b).remove_attribute(renderer, U::ATTR_NAME)
    }
}

// endregion
// region: update attr with dom api
pub(crate) struct SpecUpdateAttrWithDomApi<U: ?Sized>(U);

impl<U: ?Sized + HasAttrValueKind> HasAttrValueKind for SpecUpdateAttrWithDomApi<U> {
    type AttrValueKind = U::AttrValueKind;
}

impl<U: ?Sized, BT: BehaviorType> UpdateAttrValueOfBehaviorType<BT> for SpecUpdateAttrWithDomApi<U>
where
    U: HasDomApi<BT>,
{
    fn update_attr_value_of_behavior_type<R: ?Sized + RenderHtml>(
        //
        b: &mut <BT as BehaviorType>::OfBehaviorType<R>,
        renderer: &mut R,
        value: <Self::AttrValueKind as AttrValueKind>::AttrValue<'_>,
    ) {
        U::set_attribute_value(b, renderer, U::dom_api_value_from_value(value))
    }
}
// endregion
// region: update attr with attr name
pub(crate) trait SetAttribute: AttrValueKind {
    fn set_attribute<E: ?Sized + behaviors::Element<RR>, RR: ?Sized>(element: &mut E, renderer: &mut RR, attr_name: &str, value: Self::AttrValue<'_>);
}

impl SetAttribute for AttrKindOfStr {
    fn set_attribute<E: ?Sized + behaviors::Element<RR>, RR: ?Sized>(element: &mut E, renderer: &mut RR, attr_name: &str, value: &str) {
        element.set_attribute(renderer, attr_name, value)
    }
}

impl SetAttribute for bool {
    fn set_attribute<E: ?Sized + behaviors::Element<RR>, RR: ?Sized>(element: &mut E, renderer: &mut RR, attr_name: &str, (): ()) {
        element.set_attribute(renderer, attr_name, "")
    }
}

pub(crate) struct SpecUpdateAttrValueOfElementWithAttrName<T: ?Sized>(T);

impl<T: ?Sized + HasAttrValueKind> HasAttrValueKind for SpecUpdateAttrValueOfElementWithAttrName<T> {
    type AttrValueKind = T::AttrValueKind;
}

impl<T, ET> UpdateAttrValueOfBehaviorType<ET> for SpecUpdateAttrValueOfElementWithAttrName<T>
where
    ET: behavior_type_traits::Element,
    T: ?Sized + HasConstAttrName + HasAttrValueKind,
    T::AttrValueKind: SetAttribute,
{
    fn update_attr_value_of_behavior_type<R: ?Sized + RenderHtml>(
        //
        b: &mut <ET as BehaviorType>::OfBehaviorType<R>,
        renderer: &mut R,
        value: <Self::AttrValueKind as AttrValueKind>::AttrValue<'_>,
    ) {
        <T::AttrValueKind>::set_attribute(<ET::Element<R>>::from_mut(b), renderer, T::ATTR_NAME, value)
    }
}
// endregion
