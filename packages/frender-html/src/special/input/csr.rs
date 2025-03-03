use std::{marker::PhantomData, pin::Pin, task::Poll};

use frender_common::convert::IntoMut;
use frender_dom::csr::StateUnmount;
use frender_form_control::{
    csr::{FormControlValue, FormControlValueStateKind},
    input::{CsrInputChecked, CsrInputValue, InputDataModel, InputType, InputValue, InputValueKind, InputValueKindCsr, IntoCsrInputDataModel, IntoInputDataModel},
    KindOfChecked,
};
use frender_reactive_value::{
    non_reactive::{CachedNonReactiveValue, CachedNonReactiveValueRenderInit as _},
    temp_ref::TempRef,
    value_kind::KindOfTempRef,
};

use crate::{
    csr::{
        behavior_type::BehaviorType,
        component::{CsrComponent, RenderStateKindPollRenderWithParent},
        element::{self, PinnedRenderStateKind, UnpinnedRenderStateKind},
        kinds::RenderInitNothing,
    },
    html::{components::input, RenderHtml},
};

enum Never {}
pub struct Kind<ValueKind: ?Sized, ValueStateKind: ?Sized, CheckedStateKind: ?Sized, TypeCache>(
    //
    Never,
    PhantomData<ValueKind>,
    PhantomData<ValueStateKind>,
    PhantomData<CheckedStateKind>,
    PhantomData<TypeCache>,
);

type KindOf<Value, Checked, Type> = Kind<
    //
    <Value as InputValue>::ValueKind,
    <<Value as CsrInputValue>::IntoCsrInputValue as FormControlValue<<Value as InputValue>::ValueKind>>::StateKind,
    <<Checked as CsrInputChecked>::IntoCsrInputChecked as FormControlValue<KindOfChecked>>::StateKind,
    <<Type as InputType>::InputTypeStr as CachedNonReactiveValue<KindOfTempRef<str>>>::Cache,
>;

type StateOf<R, ValueKind, ValueStateKind, CheckedStateKind, TypeCache> = State<
    <ValueStateKind as FormControlValueStateKind<ValueKind>>::UnpinnedState<<ValueKind as InputValueKindCsr>::AsMutFormControlElement<<R as RenderHtml>::input, R>, R>,
    <CheckedStateKind as FormControlValueStateKind<KindOfChecked>>::UnpinnedState<<R as RenderHtml>::input, R>,
    Option<TypeCache>,
>;

pub struct State<ValueState, CheckedState, TypeCache> {
    value: ValueState,
    checked: CheckedState,
    type_cache: TypeCache,
}

impl<ValueState, CheckedState, TypeCache> Unpin for State<ValueState, CheckedState, TypeCache> {}
impl<ValueState: StateUnmount + Unpin, CheckedState: StateUnmount + Unpin, TypeCache> StateUnmount for State<ValueState, CheckedState, TypeCache> {
    fn state_unmount(self: Pin<&mut Self>) {
        let Self { value, checked, type_cache: _ } = self.get_mut();
        Pin::new(value).state_unmount();
        Pin::new(checked).state_unmount();
    }
}

impl<ValueKind: ?Sized + InputValueKind, ValueStateKind: ?Sized + FormControlValueStateKind<ValueKind>, CheckedStateKind: ?Sized + FormControlValueStateKind<KindOfChecked>, TypeCache> UnpinnedRenderStateKind
    for Kind<ValueKind, ValueStateKind, CheckedStateKind, TypeCache>
{
    type UnpinnedUiHandle<R: RenderHtml + ?Sized> = ();
    type UnpinnedState<R: RenderHtml + ?Sized> = StateOf<R, ValueKind, ValueStateKind, CheckedStateKind, TypeCache>;
}
impl<ValueKind: ?Sized + InputValueKind, ValueStateKind: ?Sized + FormControlValueStateKind<ValueKind>, CheckedStateKind: ?Sized + FormControlValueStateKind<KindOfChecked>, TypeCache> PinnedRenderStateKind
    for Kind<ValueKind, ValueStateKind, CheckedStateKind, TypeCache>
{
    type PinnedUiHandle<R: RenderHtml + ?Sized> = ();
    type PinnedState<R: RenderHtml + ?Sized> = StateOf<R, ValueKind, ValueStateKind, CheckedStateKind, TypeCache>;
}
impl<ValueKind: ?Sized + InputValueKind, ValueStateKind: ?Sized + FormControlValueStateKind<ValueKind>, CheckedStateKind: ?Sized + FormControlValueStateKind<KindOfChecked>, TypeCache>
    RenderStateKindPollRenderWithParent<input::Marker> for Kind<ValueKind, ValueStateKind, CheckedStateKind, TypeCache>
{
    fn pinned_poll_render_with_parent<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        parent: &mut <input::Marker as BehaviorType>::OfBehaviorType<R>,
        state: Pin<&mut element::PinnedStateOfKind<R, Self>>,
        ui_handle: &mut element::PinnedUiHandleOfKind<R, Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        Self::unpinned_poll_render_with_parent(renderer, parent, state.get_mut(), ui_handle, cx)
    }

    fn unpinned_poll_render_with_parent<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        parent: &mut <input::Marker as BehaviorType>::OfBehaviorType<R>,
        State { value, checked, type_cache: _ }: &mut element::UnpinnedStateOfKind<R, Self>,
        ui_handle: &mut element::UnpinnedUiHandleOfKind<R, Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        let a = ValueStateKind::unpinned_poll_render_form_control_value_state(renderer, parent.into_mut(), value, cx);
        let b = CheckedStateKind::unpinned_poll_render_form_control_value_state(renderer, parent, checked, cx);

        match (a, b) {
            (Poll::Ready(()), Poll::Ready(())) => Poll::Ready(()),
            _ => Poll::Pending,
        }
    }
}

impl<DataModel: IntoCsrInputDataModel> CsrComponent<DataModel> for input::Marker {
    type ChildrenRenderStateKind = KindOf<
        //
        DataModel::Value,
        DataModel::Checked,
        DataModel::Type,
    >;

    type ChildrenPinnedRenderInit<R: RenderHtml + ?Sized> = RenderInitNothing;

    fn children_pinned_render_init<R: RenderHtml + ?Sized>(
        //
        self,
        children: DataModel,
        renderer: &mut R,
        parent: &mut Self::OfBehaviorType<R>,
    ) -> (
        //
        element::PinnedStateOfKind<R, Self::ChildrenRenderStateKind>,
        Self::ChildrenPinnedRenderInit<R>,
    ) {
        let (state, ()) = self.children_unpinned_render_init(children, renderer, parent);
        (state, RenderInitNothing)
    }

    fn children_pinned_render_update<R: RenderHtml + ?Sized>(
        //
        self,
        children: DataModel,
        renderer: &mut R,
        parent: &mut Self::OfBehaviorType<R>,
        children_state: Pin<&mut element::PinnedStateOfKind<R, Self::ChildrenRenderStateKind>>,
        children_ui_handle: &mut element::PinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
    ) {
        self.children_unpinned_render_update(children, renderer, parent, children_state.get_mut(), children_ui_handle)
    }

    fn children_pinned_render_init_by_reusing<R: RenderHtml + ?Sized>(
        //
        self,
        children: DataModel,
        renderer: &mut R,
        parent: &mut Self::OfBehaviorType<R>,
        children_reused_state: Pin<&mut element::PinnedStateOfKind<R, Self::ChildrenRenderStateKind>>,
        children_unmounted_ui_handle: element::PinnedUnmountedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
    ) -> element::PinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind> {
        self.children_unpinned_render_init_by_reusing(children, renderer, parent, children_reused_state.get_mut(), children_unmounted_ui_handle)
    }

    fn children_unpinned_render_init<R: RenderHtml + ?Sized>(
        //
        self,
        children: DataModel,
        renderer: &mut R,
        element: &mut Self::OfBehaviorType<R>,
    ) -> (
        //
        element::UnpinnedStateOfKind<R, Self::ChildrenRenderStateKind>,
        element::UnpinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
    ) {
        let InputDataModel { r#type, value, checked } = children.into_input_data_model();

        // type should be updated before value is updated
        let state_type = {
            let input_type = <DataModel::Type>::maybe_into_input_type_str(r#type);

            if let Some(input_type) = input_type {
                let (mut cache, render_init) = CachedNonReactiveValue::into_cache_and_render_init(input_type);
                render_init.cached_non_reactive_value_render_init(renderer_update_type(renderer, element), &mut cache);

                Some(cache)
            } else {
                // TODO: is this needed?
                // use frender_dom::behaviors::Element;
                // element.remove_attribute(renderer, "type");

                None
            }
        };

        // value should be updated after type is updated

        let value = CsrInputValue::into_csr_input_value(value);
        let state_value = <<DataModel::Value as CsrInputValue>::IntoCsrInputValue as FormControlValue<<DataModel::Value as InputValue>::ValueKind>>::render_init(value, renderer, element.into_mut());

        let checked = CsrInputChecked::into_csr_input_checked(checked);
        let state_checked = <<DataModel::Checked as CsrInputChecked>::IntoCsrInputChecked as FormControlValue<KindOfChecked>>::render_init(checked, renderer, element);

        (
            State {
                value: state_value,
                checked: state_checked,
                type_cache: state_type,
            },
            (),
        )
    }

    fn children_unpinned_render_init_by_reusing<R: RenderHtml + ?Sized>(
        //
        self,
        children: DataModel,
        renderer: &mut R,
        parent: &mut Self::OfBehaviorType<R>,
        children_reused_state: &mut element::UnpinnedStateOfKind<R, Self::ChildrenRenderStateKind>,
        (): element::UnpinnedUnmountedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
    ) -> element::UnpinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind> {
        // TODO: render_init or render_update?
        // render_init is correct but render_update might avoid unnecessary rendering
        (*children_reused_state, ()) = self.children_unpinned_render_init(children, renderer, parent);
        ()
    }

    fn children_unpinned_render_update<R: RenderHtml + ?Sized>(
        //
        self,
        children: DataModel,
        renderer: &mut R,
        element: &mut Self::OfBehaviorType<R>,
        children_state: &mut element::UnpinnedStateOfKind<R, Self::ChildrenRenderStateKind>,
        (): &mut element::UnpinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
    ) {
        let InputDataModel { r#type, value, checked } = children.into_input_data_model();

        let State {
            type_cache: state_type,
            value: state_value,
            checked: state_checked,
        } = children_state;

        // type should be updated before value is updated
        {
            let input_type = <DataModel::Type>::maybe_into_input_type_str(r#type);

            match (state_type, input_type) {
                (None, None) => {}
                (Some(cache), Some(input_type)) => {
                    _ = CachedNonReactiveValue::maybe_update_into_cache_and_render(input_type, renderer_update_type(renderer, element), cache);
                }
                (state_type, Some(input_type)) => {
                    let (cache, render_init) = CachedNonReactiveValue::into_cache_and_render_init(input_type);
                    let cache = state_type.insert(cache);
                    render_init.cached_non_reactive_value_render_init(renderer_update_type(renderer, element), cache);
                }
                (state_type, None) => {
                    use frender_dom::csr::behaviors::Element as _;
                    element.remove_attribute(renderer, "type");
                    *state_type = None;
                }
            }
        }

        // value should be updated after type is updated
        let value = CsrInputValue::into_csr_input_value(value);
        <<DataModel::Value as CsrInputValue>::IntoCsrInputValue as FormControlValue<<DataModel::Value as InputValue>::ValueKind>>::render_update(value, renderer, element.into_mut(), state_value);

        let checked = CsrInputChecked::into_csr_input_checked(checked);
        <<DataModel::Checked as CsrInputChecked>::IntoCsrInputChecked as FormControlValue<KindOfChecked>>::render_update(checked, renderer, element, state_checked);
    }
}

fn renderer_update_type<'a, R: ?Sized + RenderHtml>(renderer: &'a mut R, element: &'a mut R::input) -> impl 'a + FnOnce(TempRef<str>) {
    |TempRef(input_type_str)| {
        use crate::html::behaviors::ElementWithTypeAttribute as _;
        element.set_type(renderer, input_type_str);
    }
}
