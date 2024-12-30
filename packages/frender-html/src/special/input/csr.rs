use std::task::Poll;

use frender_common::{convert::IntoMut, strings::CsrStr};
use frender_form_control::{
    input::{InputChecked, InputDataModel, InputType, InputValue, InputValueKind, IntoInputDataModel},
    value::{FormControlValue, FormControlValueStateKind},
};

use crate::{
    element::{PinMutRenderInitStates, PinnedRenderStateKind, RenderStates, UnpinnedRenderStateKind},
    element_types::RenderStateKindPollRenderWithParent,
    html::components::input,
    kinds::UiHandleWithNonReactiveState,
    CsrComponent, RenderHtml,
};

enum Never {}
pub struct Kind<Value, Checked, TypeCache>(Never, std::marker::PhantomData<(Value, Checked, TypeCache)>);

type NonReactiveState<R, Value, Checked, TypeCache> = (
    Option<TypeCache>,
    <<Value as FormControlValue<<Value as InputValue>::ValueKind>>::StateKind as FormControlValueStateKind<<Value as InputValue>::ValueKind>>::UnpinnedNonReactiveState<
        <<Value as InputValue>::ValueKind as InputValueKind>::AsMutFormControlElement<<R as RenderHtml>::input, R>,
        R,
    >,
    <<Checked as FormControlValue<bool>>::StateKind as FormControlValueStateKind<bool>>::UnpinnedNonReactiveState<<R as RenderHtml>::input, R>,
);

type ReactiveState<Value, Checked> = (
    <<Value as FormControlValue<<Value as InputValue>::ValueKind>>::StateKind as FormControlValueStateKind<<Value as InputValue>::ValueKind>>::UnpinnedReactiveState,
    <<Checked as FormControlValue<bool>>::StateKind as FormControlValueStateKind<bool>>::UnpinnedReactiveState,
);

impl<Value: InputValue, Checked: InputChecked, TypeCache> UnpinnedRenderStateKind for Kind<Value, Checked, TypeCache> {
    type UnpinnedUiHandle<R: RenderHtml + ?Sized> = ();
    type UnpinnedNonReactiveState<R: RenderHtml + ?Sized> = NonReactiveState<R, Value, Checked, TypeCache>;
    type UnpinnedReactiveState = ReactiveState<Value, Checked>;
}
impl<Value: InputValue, Checked: InputChecked, TypeCache> PinnedRenderStateKind for Kind<Value, Checked, TypeCache> {
    type PinnedUiHandle<R: RenderHtml + ?Sized> = UiHandleWithNonReactiveState<(), NonReactiveState<R, Value, Checked, TypeCache>>;
    type PinnedNonReactiveState<R: RenderHtml + ?Sized> = ();
    type PinnedReactiveState = ReactiveState<Value, Checked>;
}
impl<Value: InputValue, Checked: InputChecked, TypeCache> RenderStateKindPollRenderWithParent<input::Marker> for Kind<Value, Checked, TypeCache> {
    fn pinned_poll_render_with_parent<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        parent: &mut <input::Marker as crate::BehaviorType>::OfBehaviorType<R>,
        RenderStates {
            ui_handle: UiHandleWithNonReactiveState { ui_handle, non_reactive_state },
            non_reactive_state: _,
            reactive_state,
        }: crate::element::PinnedMutRenderStatesOfKind<Self, R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        Self::unpinned_poll_render_with_parent(
            renderer,
            parent,
            RenderStates {
                ui_handle,
                non_reactive_state,
                reactive_state: reactive_state.get_mut(),
            },
            cx,
        )
    }

    fn unpinned_poll_render_with_parent<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        parent: &mut <input::Marker as crate::BehaviorType>::OfBehaviorType<R>,
        RenderStates {
            ui_handle: (),
            non_reactive_state: (_, nrs_value, nrs_checked),
            reactive_state: (rs_value, rs_checked),
        }: crate::element::UnpinnedMutRenderStatesOfKind<Self, R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        let a = <<Value as FormControlValue<_>>::StateKind>::unpinned_poll_render_form_control_value_state(renderer, parent.into_mut(), nrs_value, rs_value, cx);
        let b = <<Checked as FormControlValue<bool>>::StateKind>::unpinned_poll_render_form_control_value_state(renderer, parent, nrs_checked, rs_checked, cx);

        match (a, b) {
            (Poll::Ready(()), Poll::Ready(())) => Poll::Ready(()),
            _ => Poll::Pending,
        }
    }
}

impl<DataModel: IntoInputDataModel> CsrComponent<DataModel> for input::Marker {
    type ChildrenRenderStateKind = Kind<DataModel::Value, DataModel::Checked, <<DataModel::Type as InputType>::InputTypeStr as CsrStr>::StaticStrCache>;

    fn children_pinned_render_init<R: RenderHtml + ?Sized>(
        //
        self,
        children: DataModel,
        renderer: &mut R,
        parent: &mut Self::OfBehaviorType<R>,
        PinMutRenderInitStates { non_reactive_state, reactive_state }: crate::element::PinMutRenderInitStatesOfKind<Self::ChildrenRenderStateKind, R>,
    ) -> crate::element::PinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind> {
        let () = non_reactive_state.get_mut();
        let non_reactive_state;
        RenderStates {
            ui_handle: (),
            non_reactive_state,
            reactive_state: *reactive_state.get_mut(),
        } = self.children_unpinned_render_init(children, renderer, parent);

        UiHandleWithNonReactiveState { ui_handle: (), non_reactive_state }
    }

    fn children_pinned_render_update<R: RenderHtml + ?Sized>(
        //
        self,
        children: DataModel,
        renderer: &mut R,
        parent: &mut Self::OfBehaviorType<R>,
        RenderStates {
            ui_handle: UiHandleWithNonReactiveState { ui_handle, non_reactive_state },
            non_reactive_state: _,
            reactive_state,
        }: crate::element::PinnedMutRenderStatesOfKind<Self::ChildrenRenderStateKind, R>,
    ) {
        self.children_unpinned_render_update(
            children,
            renderer,
            parent,
            RenderStates {
                ui_handle,
                non_reactive_state,
                reactive_state: reactive_state.get_mut(),
            },
        );
    }

    fn children_unpinned_render_init<R: RenderHtml + ?Sized>(
        //
        self,
        children: DataModel,
        renderer: &mut R,
        element: &mut Self::OfBehaviorType<R>,
    ) -> crate::element::UnpinnedRenderStatesOfKind<Self::ChildrenRenderStateKind, R> {
        let InputDataModel { r#type, value, checked } = children.into_input_data_model();

        // type should be updated before value is updated
        let state_type = {
            let input_type = <DataModel::Type>::maybe_into_input_type_str(r#type);

            if let Some(input_type) = input_type {
                let (cache, ()) = frender_common::strings::csr::init_cache(input_type, |input_type_str| {
                    use crate::html::behaviors::ElementWithTypeAttribute;
                    element.set_type(renderer, input_type_str);
                });

                Some(cache)
            } else {
                // TODO: is this needed?
                // use frender_dom::behaviors::Element;
                // element.remove_attribute(renderer, "type");

                None
            }
        };

        // value should be updated after type is updated

        let (nrs_value, rs_value) = <DataModel::Value as FormControlValue<<DataModel::Value as InputValue>::ValueKind>>::render_init(value, renderer, element.into_mut());

        let (nrs_checked, rs_checked) = <DataModel::Checked as FormControlValue<bool>>::render_init(checked, renderer, element);

        RenderStates {
            ui_handle: (),
            non_reactive_state: (state_type, nrs_value, nrs_checked),
            reactive_state: (rs_value, rs_checked),
        }
    }

    fn children_unpinned_render_update<R: RenderHtml + ?Sized>(
        //
        self,
        children: DataModel,
        renderer: &mut R,
        element: &mut Self::OfBehaviorType<R>,
        children_states: crate::element::UnpinnedMutRenderStatesOfKind<Self::ChildrenRenderStateKind, R>,
    ) {
        let InputDataModel { r#type, value, checked } = children.into_input_data_model();

        let RenderStates {
            ui_handle: (),
            non_reactive_state: (state_type, state_value, state_checked),
            reactive_state: (rs_value, rs_checked),
        } = children_states;

        // type should be updated before value is updated
        {
            let input_type = <DataModel::Type>::maybe_into_input_type_str(r#type);

            match (state_type, input_type) {
                (None, None) => {}
                (state_type, Some(input_type)) => {
                    _ = frender_common::strings::csr::update_with_option_cache(input_type, state_type, |input_type_str| {
                        use crate::html::behaviors::ElementWithTypeAttribute;
                        element.set_type(renderer, input_type_str);
                    })
                }
                (state_type, None) => {
                    use frender_dom::behaviors::Element;
                    element.remove_attribute(renderer, "type");
                    *state_type = None;
                }
            }
        }

        // value should be updated after type is updated
        <DataModel::Value as FormControlValue<<DataModel::Value as InputValue>::ValueKind>>::render_update(value, renderer, element.into_mut(), state_value, rs_value);

        <DataModel::Checked as FormControlValue<bool>>::render_update(checked, renderer, element, state_checked, rs_checked);
    }
}
