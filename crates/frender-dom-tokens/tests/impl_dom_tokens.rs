use frender_dom_tokens::impl_dom_tokens_for;

struct ConstDomTokens;

impl_dom_tokens_for!(|this: ConstDomTokens| dom_tokens!["asf", "asf", "asdgf"]);
