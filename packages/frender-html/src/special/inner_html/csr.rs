use frender_common::{
    reactive_value::{ProvideValueOfKind, ReactiveValueWithKind},
    value_kind::ValueKind,
};
use frender_dom::{
    csr::render_from::str::{ValueForStr, ValueKindForStr},
    special::DangerousInnerHtml,
};

use crate::{
    html::behavior_type_traits,
    special::parent_only::{impl_parent_only, RenderKind},
    BehaviorType, CsrComponent, CsrComponentNormalElement, RenderHtml,
};

pub enum RenderInnerHtmlKind {}

impl<ET: behavior_type_traits::Element, VK: ?Sized + ValueKindForStr> RenderKind<ET, VK> for RenderInnerHtmlKind {
    fn render<R: RenderHtml + ?Sized>(renderer: &mut R, parent: &mut <ET as BehaviorType>::OfBehaviorType<R>, value: <VK as ValueKind>::Value<'_>) {
        update_inner_html::<ET, R, _>(renderer, parent, value)
    }

    fn reuse<R: RenderHtml + ?Sized>(renderer: &mut R, parent: &mut <ET as BehaviorType>::OfBehaviorType<R>, provide_value: impl ProvideValueOfKind<VK>) {
        // TODO: check when debug_assertions
        let _ = (renderer, parent, provide_value);
    }
}

fn update_inner_html<ET: ?Sized + behavior_type_traits::Element, R: RenderHtml + ?Sized, V: ValueForStr>(
    //
    renderer: &mut R,
    parent: &mut ET::OfBehaviorType<R>,
    value: V,
) {
    use frender_common::convert::IntoMut as _;
    use frender_dom::csr::behaviors::SetInnerHtmlFromStr as _;

    let parent: &mut ET::Element<R> = parent.into_mut();
    parent.set_inner_html_from_str(renderer, value);
}

impl<ET: CsrComponentNormalElement, S: ReactiveValueWithKind> CsrComponent<DangerousInnerHtml<S>> for ET
where
    S::ReactiveValueKind: ValueKindForStr,
{
    impl_parent_only!(
        type Children = DangerousInnerHtml<S>;

        type ValueKind = S::ReactiveValueKind;
        type RenderKind = RenderInnerHtmlKind;

        const into_reactive_value: S = |DangerousInnerHtml(children)| children;
    );
}
