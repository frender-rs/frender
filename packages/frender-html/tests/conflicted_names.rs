use frender_html::cs;

#[test]
fn height() {
    _ = cs::canvas().height(1);
    _ = cs::td().height("1");
}

#[test]
fn value() {
    // HtmlLiElement
    _ = cs::li().value(1);

    // ElementWithValueF64Attribute
    _ = cs::meter().value(1.0);
    _ = cs::progress().value(1.0);

    // ElementWithValueStrAttribute
    _ = cs::button().value("");
    _ = cs::data().value("");
    _ = cs::option().value("");

    // HtmlInputElement
    _ = cs::input().value("");
}
