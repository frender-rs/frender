use frender_dom_tokens::DomTokenList;

#[derive(Default)]
pub struct DomTokenListAddRemove {
    pub tokens: Vec<String>,
}

impl DomTokenList for DomTokenListAddRemove {
    fn set_value(&mut self, _: &str) {
        unreachable!()
    }

    fn add_1(&mut self, token: frender_dom_tokens::DomToken) {
        assert!(self.tokens.iter().all(|t| *t != *token));
        self.tokens.push(token.as_str().to_owned())
    }

    fn remove_1(&mut self, token: frender_dom_tokens::DomToken) {
        let index = self
            .tokens
            .iter()
            .enumerate()
            .find_map(|(i, t)| (*t == *token).then_some(i));

        let index = if let Some(i) = index {
            i
        } else {
            panic!("token \"{}\" doesn't exist", token.as_str());
        };

        self.tokens.remove(index);
    }

    fn replace(&mut self, _: frender_dom_tokens::DomToken, _: frender_dom_tokens::DomToken) {
        unreachable!()
    }
}

pub struct DomTokenListNever;

impl DomTokenList for DomTokenListNever {
    fn set_value(&mut self, _: &str) {
        panic!()
    }

    fn add_1(&mut self, _: frender_dom_tokens::DomToken) {
        panic!()
    }

    fn remove_1(&mut self, _: frender_dom_tokens::DomToken) {
        panic!()
    }

    fn replace(&mut self, _: frender_dom_tokens::DomToken, _: frender_dom_tokens::DomToken) {
        panic!()
    }
}
