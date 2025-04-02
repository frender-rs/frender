mod assoc_const {
    use super::super::HasConstAttributes;

    enum HasConstValue {}

    impl_HasConstAttributes_for!(
        impl<__> HasConstValue {
            const ATTRS: _ = "";
        }
    );

    const _: () = {
        assert!(HasConstValue::ATTRS.is_empty());

        assert!(HasConstValue::ATTRIBUTES.0.ssr_str().as_str().is_empty());
        let [] = HasConstValue::ATTRIBUTES.0.attributes();
    };
}

mod empty {
    use super::super::HasConstAttributes;

    enum HasConstValue {}

    impl_HasConstAttributes_for!(
        impl<__> HasConstValue {
            const _: _ = "";
        }
    );

    const _: () = {
        assert!(HasConstValue::ATTRIBUTES.0.ssr_str().as_str().is_empty());
        let [] = HasConstValue::ATTRIBUTES.0.attributes();
    };
}
