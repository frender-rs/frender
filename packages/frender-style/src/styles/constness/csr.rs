use std::marker::PhantomData;

use crate::{
    csr::{CsrStyle, CsrStyleStateUnmount},
    css_style_declaration::CssStyleDeclaration,
};

use super::{ConstDeclarationList, HasConstDeclarationList};

pub struct State<T: ?Sized + HasConstDeclarationList>(PhantomData<T>);

impl<T: ?Sized + HasConstDeclarationList> CsrStyleStateUnmount for State<T> {
    fn csr_style_state_unmount(_: &mut Self, style: &mut impl CssStyleDeclaration) {
        T::DECLARATION_LIST
            .into_iter()
            .for_each(|d| style.remove_property(d.name.as_borrowed()));
    }
}

impl<T: ?Sized + HasConstDeclarationList> CsrStyle for ConstDeclarationList<T> {
    type State = State<T>;

    fn csr_style_render_init(_: Self, style: &mut impl CssStyleDeclaration) -> Self::State {
        T::DECLARATION_LIST.into_iter().for_each(|d| {
            let name = d.name;
            let value = d.value;
            crate::styles::declaration::csr::update_style(
                style,
                name.as_borrowed(),
                value.as_borrowed(),
                d.important,
            );
        });
        State(PhantomData)
    }

    fn csr_style_render_update(_: Self, _: &mut impl CssStyleDeclaration, _: &mut Self::State) {
        return;
    }
}
