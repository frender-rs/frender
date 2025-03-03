use std::{marker::PhantomData, task::Poll};

use async_str_iter::AsyncStrIterator;

use crate::ssr::{SsrDeclarationList, SsrStyle};

use super::{ConstDeclarationList, HasConstDeclarationList};

pub struct ConstDeclarationListIntoSsr<T: ?Sized + HasConstDeclarationList> {
    yielded: bool,
    __: PhantomData<T>,
}

impl<T: ?Sized + HasConstDeclarationList> Unpin for ConstDeclarationListIntoSsr<T> {}

impl<T: ?Sized + HasConstDeclarationList> AsyncStrIterator for ConstDeclarationListIntoSsr<T> {
    fn poll_next_str(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> Poll<Option<&str>> {
        let this = self.get_mut();
        if this.yielded {
            Poll::Ready(None)
        } else {
            this.yielded = true;
            Poll::Ready(Some(
                T::DECLARATION_LIST_PREFIX_SEMICOLON.to_str_without_prefix_semicolon(),
            ))
        }
    }
}

pub struct ConstDeclarationListIntoSsrPrefixSemicolon<T: ?Sized + HasConstDeclarationList> {
    yielded: bool,
    __: PhantomData<T>,
}

impl<T: ?Sized + HasConstDeclarationList> Unpin for ConstDeclarationListIntoSsrPrefixSemicolon<T> {}
impl<T: ?Sized + HasConstDeclarationList> AsyncStrIterator
    for ConstDeclarationListIntoSsrPrefixSemicolon<T>
{
    fn poll_next_str(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> Poll<Option<&str>> {
        // TODO: ASSERT match csr
        let this = self.get_mut();
        if this.yielded {
            Poll::Ready(None)
        } else {
            this.yielded = true;
            Poll::Ready(Some(T::DECLARATION_LIST_PREFIX_SEMICOLON.to_str()))
        }
    }
}

impl<T: ?Sized + HasConstDeclarationList> SsrDeclarationList for ConstDeclarationList<T> {
    type IntoDeclarationList = ConstDeclarationListIntoSsr<T>;

    type IntoDeclarationListPrefixSemicolon = ConstDeclarationListIntoSsrPrefixSemicolon<T>;

    fn into_declaration_list(_: Self) -> Self::IntoDeclarationList {
        ConstDeclarationListIntoSsr {
            yielded: false,
            __: PhantomData,
        }
    }

    fn into_declaration_list_prefix_semicolon(_: Self) -> Self::IntoDeclarationListPrefixSemicolon {
        ConstDeclarationListIntoSsrPrefixSemicolon {
            yielded: false,
            __: PhantomData,
        }
    }
}

impl<T: ?Sized + HasConstDeclarationList> SsrStyle for ConstDeclarationList<T> {
    type IntoSsrDeclarationList = Self;

    fn into_ssr_declaration_list(this: Self) -> Self::IntoSsrDeclarationList {
        this
    }
}
