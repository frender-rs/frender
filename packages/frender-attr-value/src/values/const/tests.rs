const _: () = {
    enum ConstAttrValue {}

    impl_HasConstAttrValue_for!(
        impl<__> ConstAttrValue {
            const _: _ = "";
        }
    );
};
