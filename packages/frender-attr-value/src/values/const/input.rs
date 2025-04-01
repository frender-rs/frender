pub enum ForAttrValue {}

mod str {
    use std::marker::PhantomData;

    use chtml::encode::{
        self,
        attribute_value::{AttributeValueForRendering, Output},
    };
    use const_core::convert::{ConstFrom, ConstInto, MarkerOfConstValue};
    use frender_const_value::input::{Info, Input, InputWithInfo};

    use super::ForAttrValue;

    pub struct Temp<'a>(Output<'a, 0>);

    impl<'a> Input<ForAttrValue> for &'a str {
        type Temp = Temp<'a>;
    }

    impl<'a, const CAP: usize> InputWithInfo<ForAttrValue, CAP> for &'a str {
        type Output = AttributeValueForRendering<'a, CAP>;
    }

    impl<'a> ConstInto<Info> for Temp<'a> {
        type Into<This: ?Sized + const_core::convert::MarkerOfConstValue<Self>> =
            TempIntoInfo<This>;
    }

    impl<'a> ConstFrom<&'a str> for Temp<'a> {
        type From<Value: ?Sized + MarkerOfConstValue<&'a str>> = TempFromValue<Value>;
    }

    impl<'a, const CAP: usize> ConstInto<AttributeValueForRendering<'a, CAP>> for Temp<'a> {
        type Into<This: ?Sized + MarkerOfConstValue<Self>> = TempIntoOutput<This>;
    }

    enum Never {}
    pub struct TempIntoInfo<M: ?Sized>(Never, PhantomData<M>);
    pub struct TempFromValue<M: ?Sized>(Never, PhantomData<M>);
    pub struct TempIntoOutput<M: ?Sized>(Never, PhantomData<M>);

    impl<'a, M: ?Sized + MarkerOfConstValue<Temp<'a>>> MarkerOfConstValue<Info> for TempIntoInfo<M> {
        const VALUE: Info = Info::new_1(M::VALUE.0.min_required_cap_of_eq_value());
    }

    impl<'a, M: ?Sized + MarkerOfConstValue<&'a str>> MarkerOfConstValue<Temp<'a>>
        for TempFromValue<M>
    {
        const VALUE: Temp<'a> = Temp(encode::attribute_value(M::VALUE));
    }

    impl<'a, const CAP: usize, M: ?Sized + MarkerOfConstValue<Temp<'a>>>
        MarkerOfConstValue<AttributeValueForRendering<'a, CAP>> for TempIntoOutput<M>
    {
        const VALUE: AttributeValueForRendering<'a, CAP> = M::VALUE.0.re_encode_for_rendering();
    }
}
