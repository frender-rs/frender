use std::marker::PhantomData;

use const_core::convert::MarkerOfConstValue;

use crate::{
    csr::{CsrAttrValue, CsrAttrValueState, UpdateAttrValue},
    AttrValueKind,
};

use super::{value::ConstAttrValueValueOfKind, ConstAttrValue, HasConstAttrValue};

pub trait ConstAttrValueCsrValue<AK: AttrValueKind> {
    fn render_init_on_absent_attribute<M: ?Sized + HasConstAttrValue<AttrValue = Self>>(
        updater: impl UpdateAttrValue<Kind = AK>,
    );
    fn render_init<M: ?Sized + HasConstAttrValue<AttrValue = Self>>(
        updater: impl UpdateAttrValue<Kind = AK>,
    );

    type AttributeIsKnownAsAbsent<This: ?Sized + HasConstAttrValue<AttrValue = Self>>: MarkerOfConstValue<bool>;
}

pub struct State<M: ?Sized + HasConstAttrValue, AK: AttrValueKind>(PhantomData<M>, PhantomData<AK>);

impl<
        M: ?Sized + HasConstAttrValue<AttrValue: ConstAttrValueValueOfKind<AK>>,
        AK: AttrValueKind,
    > CsrAttrValueState for State<M, AK>
{
    fn attribute_is_known_as_absent(&self) -> bool {
        <<M::AttrValue as ConstAttrValueCsrValue<AK>>::AttributeIsKnownAsAbsent<M> as MarkerOfConstValue<bool>>::VALUE
    }
}

impl<
        M: ?Sized + HasConstAttrValue<AttrValue: ConstAttrValueValueOfKind<AK>>,
        AK: AttrValueKind,
    > CsrAttrValue<AK> for ConstAttrValue<M>
{
    type State = State<M, AK>;

    fn render_init_on_absent_attribute(
        _: Self,
        updater: impl UpdateAttrValue<Kind = AK>,
    ) -> Self::State {
        <M::AttrValue as ConstAttrValueCsrValue<AK>>::render_init_on_absent_attribute::<M>(updater);
        State(PhantomData, PhantomData)
    }

    fn render_init(_: Self, updater: impl UpdateAttrValue<Kind = AK>) -> Self::State {
        <M::AttrValue as ConstAttrValueCsrValue<AK>>::render_init::<M>(updater);
        State(PhantomData, PhantomData)
    }

    fn render_init_by_reusing_on_absent_attribute(
        this: Self,
        updater: impl UpdateAttrValue<Kind = AK>,
        state: &mut Self::State,
    ) {
        *state = Self::render_init_on_absent_attribute(this, updater)
    }

    fn render_init_by_reusing(
        this: Self,
        updater: impl UpdateAttrValue<Kind = AK>,
        state: &mut Self::State,
    ) {
        *state = Self::render_init(this, updater)
    }

    fn render_update(_: Self, _: impl UpdateAttrValue<Kind = AK>, _: &mut Self::State) {}
}
