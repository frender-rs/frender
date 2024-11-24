use frender_html::cs;

#[test]
fn height_weight() {
    // ElementWithHeightWidthU32Attributes
    _ = cs::video().width(100).height(50);
    _ = cs::canvas().height(1).width(2);
    _ = cs::img().height(100).width(250);
    _ = cs::input().width(40).height(10);
    _ = cs::source().width(40).height(10);

    // ElementWithHeightWidthStrAttributes
    _ = cs::embed().height("10").width("40");
    _ = cs::iframe().height("10").width("40");
    _ = cs::object().height("10").width("40");
    _ = cs::td().width("2").height("1");
    _ = cs::th().height("2").width("1");
}

#[test]
fn max_min() {
    // ElementWithMaxF64Attribute
    _ = cs::progress().max(1.);
    _ = cs::meter().max(1.).min(0.);

    // HtmlInputElement
    _ = cs::input().max("1").min("0");
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
