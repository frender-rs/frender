mod assoc_const {
    use super::super::HasConstAttributes;

    enum HasConstValue {}

    impl_has_const_attributes_for!(
        impl<__> HasConstValue {
            const ATTRS: _ = "";
        }
    );

    const _: () = {
        assert!(HasConstValue::ATTRS.is_empty());

        assert!(HasConstValue::ATTRIBUTES.ssr_str().as_str().is_empty());
        let [] = HasConstValue::ATTRIBUTES.attributes();
    };
}

mod empty {
    use super::super::HasConstAttributes;

    enum HasConstValue {}

    impl_has_const_attributes_for!(
        impl<__> HasConstValue {
            const _: _ = "";
        }
    );

    const _: () = {
        assert!(HasConstValue::ATTRIBUTES.ssr_str().as_str().is_empty());
        let [] = HasConstValue::ATTRIBUTES.attributes();
    };
}
