use std::marker::PhantomData;

use chtml::attrs::{parse_str, parse_str_with_ssr, AttributesForRendering};
use frender_const_value::input::{
    ConstFrom, ConstInto, Info, Input, InputWithInfo, MarkerOfConstValue,
};

use super::ForAttributes;

pub struct Temp<'a> {
    input: &'a str,
}

pub struct Output<
    'a,
    const ATTRS: usize,
    const CSR: usize,
    const SSR: usize,
    const SSR_STRING_CAP: usize,
>(pub AttributesForRendering<'a, ATTRS, SSR_STRING_CAP>);

impl<'a> Input<ForAttributes> for &'a str {
    type Temp = Temp<'a>;
}

impl<'a, const ATTRS: usize, const CSR: usize, const SSR: usize, const SSR_STRING_CAP: usize>
    InputWithInfo<ForAttributes, ATTRS, CSR, SSR, SSR_STRING_CAP> for &'a str
{
    type Output = Output<'a, ATTRS, CSR, SSR, SSR_STRING_CAP>;
}

impl<'a> ConstInto<Info> for Temp<'a> {
    type Into<This: ?Sized + MarkerOfConstValue<Self>> = TempIntoInfo<This>;
}

impl<'a> ConstFrom<&'a str> for Temp<'a> {
    type From<Value: ?Sized + MarkerOfConstValue<&'a str>> = TempFromValue<Value>;
}

impl<'a, const ATTRS: usize, const CSR: usize, const SSR: usize, const SSR_STRING_CAP: usize>
    ConstInto<Output<'a, ATTRS, CSR, SSR, SSR_STRING_CAP>> for Temp<'a>
{
    type Into<This: ?Sized + MarkerOfConstValue<Self>> = TempIntoOutput<This>;
}

enum Never {}
pub struct TempIntoInfo<M: ?Sized>(Never, PhantomData<M>);
pub struct TempFromValue<M: ?Sized>(Never, PhantomData<M>);
pub struct TempIntoOutput<M: ?Sized>(Never, PhantomData<M>);

impl<'a, M: ?Sized + MarkerOfConstValue<Temp<'a>>> MarkerOfConstValue<Info> for TempIntoInfo<M> {
    const VALUE: Info = {
        let s: &str = M::VALUE.input;
        let parsed = parse_str::<0, 0, 0>(s);

        Info::new_4([
            parsed.len(),
            parsed.max_csr_cap(),
            parsed.max_ssr_cap(),
            parsed.ssr_string_len(),
        ])
    };
}

impl<'a, M: ?Sized + MarkerOfConstValue<&'a str>> MarkerOfConstValue<Temp<'a>>
    for TempFromValue<M>
{
    const VALUE: Temp<'a> = Temp { input: M::VALUE };
}

impl<
        'a,
        const ATTRS: usize,
        const CSR: usize,
        const SSR: usize,
        const SSR_STRING_CAP: usize,
        M: ?Sized + MarkerOfConstValue<Temp<'a>>,
    > MarkerOfConstValue<Output<'a, ATTRS, CSR, SSR, SSR_STRING_CAP>> for TempIntoOutput<M>
{
    const VALUE: Output<'a, ATTRS, CSR, SSR, SSR_STRING_CAP> = Output(
        parse_str_with_ssr::<ATTRS, CSR, SSR, SSR_STRING_CAP>(M::VALUE.input)
            .to_attributes_for_rendering(),
    );
}
