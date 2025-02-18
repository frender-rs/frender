use crate::{has_const_attr_name::HasConstAttrName, html::behavior_type_traits, BehaviorType, RenderHtml};

use frender_common::convert::FromMut as _;
use frender_dom::behaviors::Element as _;

pub(crate) trait RemoveAttrOfBehaviorType<BT: BehaviorType> {
    fn remove_attr_of_behavior_type<R: ?Sized + RenderHtml>(
        //
        b: &mut BT::OfBehaviorType<R>,
        renderer: &mut R,
    );
}

pub(crate) trait UseSpecRemoveAttrOfBehaviorType {}
pub(crate) trait HasSpecRemoveAttrOfBehaviorType<BT: BehaviorType>: UseSpecRemoveAttrOfBehaviorType {
    type SpecRemoveAttrOfBehaviorType: RemoveAttrOfBehaviorType<BT>;
}

impl<T, BT> RemoveAttrOfBehaviorType<BT> for T
where
    T: ?Sized + HasSpecRemoveAttrOfBehaviorType<BT> + UseSpecRemoveAttrOfBehaviorType,
    BT: BehaviorType,
{
    fn remove_attr_of_behavior_type<R: ?Sized + RenderHtml>(
        //
        b: &mut <BT as BehaviorType>::OfBehaviorType<R>,
        renderer: &mut R,
    ) {
        <T::SpecRemoveAttrOfBehaviorType>::remove_attr_of_behavior_type(b, renderer)
    }
}

pub(crate) struct SpecRemoveAttrOfElementTypeWithAttrName<T: ?Sized + HasConstAttrName>(T);

impl<ET: behavior_type_traits::Element, T: ?Sized + HasConstAttrName> RemoveAttrOfBehaviorType<ET> for SpecRemoveAttrOfElementTypeWithAttrName<T> {
    fn remove_attr_of_behavior_type<R: ?Sized + RenderHtml>(
        //
        b: &mut <ET as BehaviorType>::OfBehaviorType<R>,
        renderer: &mut R,
    ) {
        <ET::Element<R>>::from_mut(b).remove_attribute(renderer, T::ATTR_NAME)
    }
}
