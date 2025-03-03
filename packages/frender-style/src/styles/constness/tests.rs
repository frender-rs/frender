use async_str_iter::ext::AsyncStrIteratorExt;
use futures_lite::future::block_on;

use crate::{ssr::SsrDeclarationList, styles::constness::HasConstDeclarationList};

enum Demo {}

impl_has_const_declaration_list_for!(
    impl<__> Demo {
        const _: _ = r"
                font-size: large;
                animation: 3s infinite alternate slidein;
            ";
    }
);

#[test]
fn test() {
    let v = super::ConstDeclarationList::<Demo>();

    assert_eq!(
        Demo::DECLARATION_LIST_PREFIX_SEMICOLON.to_str(),
        ";font-size:large;animation:3s infinite alternate slidein"
    );

    assert_eq!(
        Demo::DECLARATION_LIST_PREFIX_SEMICOLON.to_str_without_prefix_semicolon(),
        "font-size:large;animation:3s infinite alternate slidein"
    );

    {
        let s = block_on(SsrDeclarationList::into_declaration_list(v).collect::<String>());
        assert_eq!(
            s,
            Demo::DECLARATION_LIST_PREFIX_SEMICOLON.to_str_without_prefix_semicolon()
        )
    }

    {
        let s = block_on(
            SsrDeclarationList::into_declaration_list_prefix_semicolon(v).collect::<String>(),
        );
        assert_eq!(s, Demo::DECLARATION_LIST_PREFIX_SEMICOLON.to_str())
    }
}
