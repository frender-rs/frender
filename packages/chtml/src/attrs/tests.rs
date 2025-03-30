use super::{parse_str, Attribute, AttributeEqValueSsr, Quote};

// zero
const _: () = {
    assert!(parse_str::<0, 0, 0>("").is_empty());
    assert!(parse_str::<0, 0, 0>(" ").is_empty());
    assert!(parse_str::<0, 0, 0>("\n").is_empty());
    assert!(parse_str::<0, 0, 0>("\r\r\n").is_empty());
};

// one
const _: () = {
    assert!(matches!(
        parse_str::<1, 0, 0>("checked").as_slice(),
        [Attribute {
            name,
            value,
            eq_value
        }] if matches!(name.as_bytes(), b"checked")
            && value.is_empty()
            && matches!(eq_value,AttributeEqValueSsr::Empty)
    ));
    assert!(matches!(
        parse_str::<1, 0, 0>("id=unquoted").as_slice(),
        [Attribute {
            name,
            value,
            eq_value
        }] if matches!(name.as_bytes(), b"id")
            && matches!(value.as_str().as_bytes(), b"unquoted")
            && matches!(
                eq_value,
                AttributeEqValueSsr::Eq { quote: None, ssr }
                if matches!(ssr.as_str().as_bytes(), b"unquoted")
            )
    ));
    assert!(matches!(
        parse_str::<1, 0, 0>("xml:link=\"double' quoted\"").as_slice(),
        [Attribute {
            name,
            value,
            eq_value
        }] if matches!(name.as_bytes(), b"xml:link")
            && matches!(value.as_str().as_bytes(), b"double' quoted")
            && matches!(
                eq_value,
                AttributeEqValueSsr::Eq { quote: Some(Quote::Double), ssr }
                if matches!(ssr.as_str().as_bytes(), b"double' quoted")
            )
    ));
    assert!(matches!(
        parse_str::<1, 0, 0>("data-value='single \" quoted'").as_slice(),
        [Attribute {
            name,
            value,
            eq_value
        }] if matches!(name.as_bytes(), b"data-value")
            && matches!(value.as_str().as_bytes(), b"single \" quoted")
            && matches!(
                eq_value,
                AttributeEqValueSsr::Eq { quote: Some(Quote::Single), ssr }
                if matches!(ssr.as_str().as_bytes(), b"single \" quoted")
            )
    ));
};

// character reference
const _: () = {
    // `&` in attribute name is kept intact.
    assert!(matches!(
        parse_str::<1, 0, 0>("checked&AMP;").as_slice(),
        [Attribute {
            name,
            value,
            eq_value
        }] if matches!(name.as_bytes(), b"checked&AMP;")
            && value.is_empty()
            && matches!(eq_value,AttributeEqValueSsr::Empty)
    ));
    {
        #[allow(unused)]
        mod expected {
            pub const CSR: &[u8] = b"unquoted&value";
            pub const SSR: &[u8] = b"unquoted&AMP;value";
        }
        assert!(matches!(
            parse_str::<1, { expected::CSR.len() }, { expected::SSR.len() }>(
                " id=unquoted&AMP;value "
            ).as_slice(),
            [Attribute {
                name,
                value,
                eq_value
            }] if matches!(name.as_bytes(), b"id")
                && matches!(value.as_str().as_bytes(), expected::CSR)
                && matches!(
                    eq_value,
                    AttributeEqValueSsr::Eq { quote: None, ssr }
                    if matches!(ssr.as_str().as_bytes(), expected::SSR)
                )
        ));
    }
    {
        #[allow(unused)]
        mod expected {
            pub const CSR: &[u8] = b"double quoted &";
            pub const SSR: &[u8] = b"double quoted &amp;";
        }
        assert!(matches!(
            parse_str::<1, { expected::CSR.len() }, { expected::SSR.len() }>(
                "xml:link=\"double quoted &amp;\""
            ).as_slice(),
            [Attribute {
                name,
                value,
                eq_value
            }] if matches!(name.as_bytes(), b"xml:link")
                && matches!(value.as_str().as_bytes(), expected::CSR)
                && matches!(
                    eq_value,
                    AttributeEqValueSsr::Eq { quote: Some(Quote::Double), ssr }
                    if matches!(ssr.as_str().as_bytes(), expected::SSR)
                )
        ));
    }
    {
        #[allow(unused)]
        mod expected {
            pub const CSR: &[u8] = b"&&single quoted&&";
            pub const SSR: &[u8] = b"&#X26;&#x0026;single quoted&#0038;&#38;";
        }
        assert!(matches!(
            parse_str::<1, { expected::CSR.len() }, { expected::SSR.len() }>(
                "data-value=\"&#X26;&#x0026;single quoted&#0038;&#38;\""
            ).as_slice(),
            [Attribute {
                name,
                value,
                eq_value
            }] if matches!(name.as_bytes(), b"data-value")
                && matches!(value.as_str().as_bytes(), expected::CSR)
                && matches!(
                    eq_value,
                    AttributeEqValueSsr::Eq { quote: Some(Quote::Double), ssr }
                    if matches!(ssr.as_str().as_bytes(), expected::SSR)
                )
        ));
    }
};

const _: () = {
    assert!(matches!(
    parse_str::<4, 0, 0>(
        r##"
        href="/app.webmanifest"
        itemscope
        onclick='console.log("hello")'
        rel=preload
        "##
    ).as_slice(),
    [a0, a1, a2, a3] if
    matches!(a0, Attribute {
        name,
        value,
        eq_value: AttributeEqValueSsr::Eq { quote: Some(Quote::Double), ssr }
    } if matches!(name.as_bytes(), b"href")
        && matches!(value.as_str().as_bytes(),b"/app.webmanifest")
        && matches!(ssr.as_str().as_bytes(),b"/app.webmanifest"))
    &&
    matches!(a1, Attribute {
        name,
        value,
        eq_value: AttributeEqValueSsr::Empty
    } if matches!(name.as_bytes(), b"itemscope")
        && matches!(value.as_str().as_bytes(), b""))
    &&
    matches!(a2, Attribute {
        name,
        value,
        eq_value: AttributeEqValueSsr::Eq { quote: Some(Quote::Single), ssr }
    } if matches!(name.as_bytes(), b"onclick")
        && matches!(value.as_str().as_bytes(), b"console.log(\"hello\")")
        && matches!(ssr.as_str().as_bytes(), b"console.log(\"hello\")"))
    &&
    matches!(a3, Attribute {
        name,
        value,
        eq_value: AttributeEqValueSsr::Eq { quote: None, ssr }
    } if matches!(name.as_bytes(), b"rel")
        && matches!(value.as_str().as_bytes(), b"preload")
        && matches!(ssr.as_str().as_bytes(), b"preload"))
    && true
    ))
};
