use chtml::encode::attribute_value::AttributeValueForRendering;

use crate::{
    attr_value,
    values::{
        r#const::{ConstAttrValue, HasConstAttrValue},
        EitherAttrValue,
    },
};

const fn assert_empty<M: HasConstAttrValue<AttrValue = AttributeValueForRendering<'static, 0>>>(
    _: ConstAttrValue<M>,
) {
    const {
        assert!(M::ATTR_VALUE.as_str().is_empty());
        assert!(matches!(
            M::ATTR_VALUE.as_eq_value_str().as_str().as_bytes(),
            b"=\"\""
        ));
    }
}

const fn get_csr_and_ssr<
    M: HasConstAttrValue<AttrValue = AttributeValueForRendering<'static, CAP>>,
    const CAP: usize,
>(
    _: ConstAttrValue<M>,
) -> (&'static str, &'static str) {
    (
        M::ATTR_VALUE.as_str(),
        M::ATTR_VALUE.as_eq_value_str().as_str(),
    )
}

#[test]
fn const_macro() {
    const {
        assert_empty(attr_value::r#const!(const { "" }));
        assert_empty(attr_value::r#const!(const { "" } as _));
        assert_empty(attr_value::r#const!(const { "" } as &'static str));
        assert_empty(attr_value::r#const!(""));
        assert_empty(attr_value::r#const!("" as _));
        assert_empty(attr_value::r#const!("" as &'static str));
        const EMPTY: &str = "";
        assert_empty(attr_value::r#const!(EMPTY));
    }
}

#[test]
fn inferred_const() {
    const {
        assert_empty(attr_value!(const { "" }));
        assert_empty(attr_value!(const { "" } as _));
        assert_empty(attr_value!(const { "" } as &'static str));
        assert_empty(attr_value!(""));
        assert_empty(attr_value!("" as _));
        assert_empty(attr_value!("" as &'static str));
        const EMPTY: &str = "";
        assert_empty(attr_value!(const { EMPTY }));
    }
}

#[test]
fn verbatim() {
    #[expect(deprecated)]
    let v = attr_value!(verbatim!("aaa"));
    assert_eq!(v, "aaa");
}

#[test]
fn nesting_attr_value() {
    #[expect(deprecated)]
    assert_empty(attr_value!(attr_value!("")));
    #[expect(deprecated)]
    assert_empty(attr_value!(attr_value!(attr_value!(""))));
}

/// MANUAL TEST: rust-analyzer and rustc should report errors friendly
const _: () = {
    // _ = attr_value!(if (true) {
    //     ()
    // });

    // _ = attr_value!(());
};

#[test]
fn option() {
    const fn opt(
        condition: bool,
    ) -> Option<
        ConstAttrValue<impl HasConstAttrValue<AttrValue = AttributeValueForRendering<'static, 0>>>,
    > {
        attr_value!(if (condition) {
            ""
        })
    }

    const {
        assert_empty(opt(true).unwrap());
        assert!(opt(false).is_none());
    }
}

#[test]
fn either() {
    const fn either_2(
        condition: bool,
    ) -> EitherAttrValue<
        ConstAttrValue<impl HasConstAttrValue<AttrValue = AttributeValueForRendering<'static, 0>>>,
        ConstAttrValue<
            impl HasConstAttrValue<AttrValue = AttributeValueForRendering<'static, { "=value".len() }>>,
        >,
    > {
        attr_value!(if (condition) { "" } else { "value" })
    }

    const {
        assert_empty(match either_2(true) {
            EitherAttrValue::A(v) => v,
            EitherAttrValue::B(_) => panic!(),
        });
    }

    match either_2(false) {
        EitherAttrValue::A(_) => panic!(),
        EitherAttrValue::B(v) => assert_eq!(get_csr_and_ssr(v), ("value", "=value")),
    }
}
