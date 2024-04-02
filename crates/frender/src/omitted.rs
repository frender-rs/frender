use frender_html_common::MaybeUpdateValueWithState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Omitted;

impl MaybeUpdateValueWithState<bool> for Omitted {
    type State = ();

    fn maybe_as(_: &Self) -> Option<&bool> {
        Some(&true)
    }

    fn initialize_state_and_update(
        _: Self,
        update: impl FnOnce(&bool),
        _: impl FnOnce(),
    ) -> Self::State {
        update(&true)
    }

    fn maybe_update_value_with_state(
        _: Self,
        _: &mut Self::State,
        _: impl FnOnce(&bool),
        _: impl FnOnce(),
    ) {
    }

    type HtmlAttributeEqValueOrEmpty = async_str_iter::empty::Empty;

    fn maybe_into_html_attribute_eq_value_or_empty(
        _: Self,
    ) -> Option<Self::HtmlAttributeEqValueOrEmpty> {
        Some(async_str_iter::empty::Empty)
    }

    // whether initialized
    type UpdateWithState = bool;

    fn update_with_state(
        _: Self,
        state: &mut Self::UpdateWithState,
        updater: impl frender_html_common::ValueUpdater<bool>,
    ) {
        if !*state {
            updater.update(&true)
        }
    }
}
