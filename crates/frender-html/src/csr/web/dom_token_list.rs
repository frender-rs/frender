use frender_common::try_behavior::{TryBehavior, TryWithTryBehavior};

pub struct DomTokenList<TB: TryBehavior>(pub(crate) web_sys::DomTokenList, pub(crate) TB);

impl<TB: TryBehavior> crate::DomTokenList for DomTokenList<TB> {
    fn set_value(&mut self, value: &str) {
        self.0.set_value(value)
    }

    fn add_1(&mut self, token: &str) {
        self.0.add_1(token).unwrap_with_behavior(&mut self.1)
    }

    fn remove_1(&mut self, token: &str) {
        self.0.remove_1(token).unwrap_with_behavior(&mut self.1)
    }

    fn replace(&mut self, old_token: &str, new_token: &str) {
        _ = self.0.replace(old_token, new_token).unwrap_with_behavior(&mut self.1)
    }
}
