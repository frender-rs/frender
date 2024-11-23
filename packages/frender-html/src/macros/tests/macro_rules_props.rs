use super::super::define_props_macro::define;

define! {
    Node
    main_ancestors()
    other_ancestors()
    {}
}

define! {
    Element
    main_ancestors(Node)
    other_ancestors()
    {}
}

define! {
    HtmlElement
    main_ancestors(Element)
    other_ancestors()
    {}
}

define! {
    ElementWithTypeAttribute
    main_ancestors(Element)
    other_ancestors()
    {}
}

define! {
    HtmlAnchorElement
    main_ancestors(HtmlElement)
    other_ancestors(ElementWithTypeAttribute)
    {}
}

const _: () = {
    struct Node;
    struct Element;
    struct HtmlElement;
    struct ElementWithTypeAttribute;

    macro_rules! chain_braced {
        ($({$name:ident})*) => {
            ($($name,)*)
        };
    }

    macro_rules! expand {
        ($name:ident . $branch:tt) => {
            $name! {
                $branch
                {
                    wrap {}
                    prepend(chain_braced!)
                }
            }
        };
    }

    let (Element, Node) = expand!(HtmlElement.for_all_ancestors);
    let (Element, Node) = expand!(HtmlElement.for_all_main_ancestors);
    let () = expand!(HtmlElement.for_all_other_ancestors);

    let (HtmlElement, Element, Node, ElementWithTypeAttribute) = expand!(HtmlAnchorElement.for_all_ancestors);
    let (HtmlElement, Element, Node) = expand!(HtmlAnchorElement.for_all_main_ancestors);
    let (ElementWithTypeAttribute,) = expand!(HtmlAnchorElement.for_all_other_ancestors);
};

#[test]
fn compile_only() {}
