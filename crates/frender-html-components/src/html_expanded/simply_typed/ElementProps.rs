#[allow(non_snake_case)]
#[inline(always)]
pub fn ElementProps() -> Building {
    Building(Default::default())
}
pub mod data_struct {
    #[allow(unused_imports)]
    use super::super::*;
    #[derive(Debug, Clone, Copy, Default)]
    #[repr(transparent)]
    pub struct ElementProps<
        Children = crate::imports::frender_html_simple::AllowChildren,
        Props = (),
    > {
        pub props: crate::imports::frender_html_simple::ElementProps<Children, Props>,
    }
}
pub mod building_struct {
    #[allow(unused_imports)]
    use super::super::*;
    #[repr(transparent)]
    pub struct ElementProps<Children = crate::imports::frender_html_simple::AllowChildren, Props = ()>(
        pub super::Data<Children, Props>,
    );
}
pub use building_struct::ElementProps as Building;
pub use data_struct::ElementProps as Data;
pub type DataInitial = data_struct::ElementProps;
pub mod prelude {}
#[inline(always)]
pub fn build<Children, Props>(building: Building<Children, Props>) -> Data<Children, Props> {
    building.0
}
pub use build as valid;
pub mod props {
    super::def_props!(
        children,
        class,
        id,
        part,
        on_cancel,
        on_error,
        on_scroll,
        on_security_policy_violation,
        on_select,
        on_wheel,
        on_copy,
        on_cut,
        on_paste,
        on_composition_end,
        on_composition_start,
        on_composition_update,
        on_blur,
        on_focus,
        on_focus_in,
        on_focus_out,
        on_fullscreen_change,
        on_fullscreen_error,
        on_key_down,
        on_key_up,
        on_aux_click,
        on_click,
        on_context_menu,
        on_double_click,
        on_mouse_down,
        on_mouse_enter,
        on_mouse_leave,
        on_mouse_move,
        on_mouse_out,
        on_mouse_over,
        on_mouse_up,
        on_touch_cancel,
        on_touch_end,
        on_touch_move,
        on_touch_start,
    );
}
#[cfg(feature = "dom")]
mod props_impl_dom {
    #[allow(unused_imports)]
    use super::super::*;
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::class<V>
    {
        type State = super::props::class<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::class(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "class",
                    ),
                    || dom_element.remove_attribute("class").unwrap(),
                ),
            )
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "class",
                ),
                || dom_element.remove_attribute("class").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E> for super::props::id<V>
    {
        type State = super::props::id<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::id(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_id(v),
                    || dom_element.remove_attribute("id").unwrap(),
                ),
            )
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| element.set_id(v),
                || dom_element.remove_attribute("id").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::part<V>
    {
        type State = super::props::part<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::part(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "part",
                    ),
                    || dom_element.remove_attribute("part").unwrap(),
                ),
            )
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            let element = dom_element;
            <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "part",
                ),
                || dom_element.remove_attribute("part").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::Cancel>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_cancel<V>
    {
        type State = super::props::on_cancel<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_cancel(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::Error>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_error<V>
    {
        type State = super::props::on_error<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_error(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::Scroll>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_scroll<V>
    {
        type State = super::props::on_scroll<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_scroll(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<
                events::SecurityPolicyViolation,
            >,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_security_policy_violation<V>
    {
        type State = super::props::on_security_policy_violation<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_security_policy_violation(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::Select>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_select<V>
    {
        type State = super::props::on_select<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_select(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::Wheel>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_wheel<V>
    {
        type State = super::props::on_wheel<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_wheel(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::Copy>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_copy<V>
    {
        type State = super::props::on_copy<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_copy(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::Cut>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_cut<V>
    {
        type State = super::props::on_cut<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_cut(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::Paste>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_paste<V>
    {
        type State = super::props::on_paste<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_paste(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::CompositionEnd>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_composition_end<V>
    {
        type State = super::props::on_composition_end<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_composition_end(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::CompositionStart>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_composition_start<V>
    {
        type State = super::props::on_composition_start<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_composition_start(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::CompositionUpdate>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_composition_update<V>
    {
        type State = super::props::on_composition_update<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_composition_update(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::Blur>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_blur<V>
    {
        type State = super::props::on_blur<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_blur(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::Focus>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_focus<V>
    {
        type State = super::props::on_focus<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_focus(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::FocusIn>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_focus_in<V>
    {
        type State = super::props::on_focus_in<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_focus_in(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::FocusOut>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_focus_out<V>
    {
        type State = super::props::on_focus_out<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_focus_out(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::FullscreenChange>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_fullscreen_change<V>
    {
        type State = super::props::on_fullscreen_change<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_fullscreen_change(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::FullscreenError>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_fullscreen_error<V>
    {
        type State = super::props::on_fullscreen_error<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_fullscreen_error(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::KeyDown>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_key_down<V>
    {
        type State = super::props::on_key_down<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_key_down(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::KeyUp>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_key_up<V>
    {
        type State = super::props::on_key_up<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_key_up(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::AuxClick>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_aux_click<V>
    {
        type State = super::props::on_aux_click<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_aux_click(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::Click>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_click<V>
    {
        type State = super::props::on_click<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_click(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::ContextMenu>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_context_menu<V>
    {
        type State = super::props::on_context_menu<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_context_menu(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::DoubleClick>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_double_click<V>
    {
        type State = super::props::on_double_click<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_double_click(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::MouseDown>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_mouse_down<V>
    {
        type State = super::props::on_mouse_down<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_mouse_down(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::MouseEnter>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_mouse_enter<V>
    {
        type State = super::props::on_mouse_enter<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_mouse_enter(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::MouseLeave>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_mouse_leave<V>
    {
        type State = super::props::on_mouse_leave<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_mouse_leave(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::MouseMove>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_mouse_move<V>
    {
        type State = super::props::on_mouse_move<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_mouse_move(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::MouseOut>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_mouse_out<V>
    {
        type State = super::props::on_mouse_out<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_mouse_out(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::MouseOver>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_mouse_over<V>
    {
        type State = super::props::on_mouse_over<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_mouse_over(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::MouseUp>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_mouse_up<V>
    {
        type State = super::props::on_mouse_up<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_mouse_up(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::TouchCancel>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_touch_cancel<V>
    {
        type State = super::props::on_touch_cancel<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_touch_cancel(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::TouchEnd>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_touch_end<V>
    {
        type State = super::props::on_touch_end<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_touch_end(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::TouchMove>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_touch_move<V>
    {
        type State = super::props::on_touch_move<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_touch_move(V::initialize_dom_event_listener_state(this.0, dom_element))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::TouchStart>,
            E: ::core::convert::AsRef<web_sys::Element>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_touch_start<V>
    {
        type State = super::props::on_touch_start<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_touch_start(V::initialize_dom_event_listener_state(
                this.0,
                dom_element,
            ))
        }
        fn update_element_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element = element.as_ref();
            V::update_dom_event_listener(this.0, dom_element, &mut state.get_mut().0)
        }
    }
}
#[cfg(feature = "ssr")]
mod props_impl_ssr {
    #[allow(unused_imports)]
    use super::super::*;
}
mod builder_and_replacer {
    #[allow(unused_imports)]
    use super::super::*;
    impl<Props> super::Building<crate::imports::frender_html_simple::AllowChildren, Props> {
        #[inline(always)]
        pub fn children<Children>(self, children: Children) -> super::Building<Children, Props> {
            super::Building(super::Data {
                props: self.0.props.children(children),
            })
        }
    }
    impl<Children, Props> super::Building<Children, Props> {
        #[inline(always)]
        pub fn class<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            class: V,
        ) -> super::Building<Children, (Props, super::props::class<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::props::class(class)),
            })
        }
        #[inline(always)]
        pub fn id<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            id: V,
        ) -> super::Building<Children, (Props, super::props::id<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::props::id(id)),
            })
        }
        #[inline(always)]
        pub fn part<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            part: V,
        ) -> super::Building<Children, (Props, super::props::part<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::props::part(part)),
            })
        }
        #[inline(always)]
        pub fn on_cancel<V>(
            self,
            on_cancel: V,
        ) -> super::Building<Children, (Props, super::props::on_cancel<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::props::on_cancel(on_cancel)),
            })
        }
        #[inline(always)]
        pub fn on_error<V>(
            self,
            on_error: V,
        ) -> super::Building<Children, (Props, super::props::on_error<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::props::on_error(on_error)),
            })
        }
        #[inline(always)]
        pub fn on_scroll<V>(
            self,
            on_scroll: V,
        ) -> super::Building<Children, (Props, super::props::on_scroll<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::props::on_scroll(on_scroll)),
            })
        }
        #[inline(always)]
        pub fn on_security_policy_violation<V>(
            self,
            on_security_policy_violation: V,
        ) -> super::Building<Children, (Props, super::props::on_security_policy_violation<V>)>
        {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_security_policy_violation(
                        on_security_policy_violation,
                    )),
            })
        }
        #[inline(always)]
        pub fn on_select<V>(
            self,
            on_select: V,
        ) -> super::Building<Children, (Props, super::props::on_select<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::props::on_select(on_select)),
            })
        }
        #[inline(always)]
        pub fn on_wheel<V>(
            self,
            on_wheel: V,
        ) -> super::Building<Children, (Props, super::props::on_wheel<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::props::on_wheel(on_wheel)),
            })
        }
        #[inline(always)]
        pub fn on_copy<V>(
            self,
            on_copy: V,
        ) -> super::Building<Children, (Props, super::props::on_copy<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::props::on_copy(on_copy)),
            })
        }
        #[inline(always)]
        pub fn on_cut<V>(
            self,
            on_cut: V,
        ) -> super::Building<Children, (Props, super::props::on_cut<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::props::on_cut(on_cut)),
            })
        }
        #[inline(always)]
        pub fn on_paste<V>(
            self,
            on_paste: V,
        ) -> super::Building<Children, (Props, super::props::on_paste<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::props::on_paste(on_paste)),
            })
        }
        #[inline(always)]
        pub fn on_composition_end<V>(
            self,
            on_composition_end: V,
        ) -> super::Building<Children, (Props, super::props::on_composition_end<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_composition_end(on_composition_end)),
            })
        }
        #[inline(always)]
        pub fn on_composition_start<V>(
            self,
            on_composition_start: V,
        ) -> super::Building<Children, (Props, super::props::on_composition_start<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_composition_start(on_composition_start)),
            })
        }
        #[inline(always)]
        pub fn on_composition_update<V>(
            self,
            on_composition_update: V,
        ) -> super::Building<Children, (Props, super::props::on_composition_update<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_composition_update(on_composition_update)),
            })
        }
        #[inline(always)]
        pub fn on_blur<V>(
            self,
            on_blur: V,
        ) -> super::Building<Children, (Props, super::props::on_blur<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::props::on_blur(on_blur)),
            })
        }
        #[inline(always)]
        pub fn on_focus<V>(
            self,
            on_focus: V,
        ) -> super::Building<Children, (Props, super::props::on_focus<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::props::on_focus(on_focus)),
            })
        }
        #[inline(always)]
        pub fn on_focus_in<V>(
            self,
            on_focus_in: V,
        ) -> super::Building<Children, (Props, super::props::on_focus_in<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_focus_in(on_focus_in)),
            })
        }
        #[inline(always)]
        pub fn on_focus_out<V>(
            self,
            on_focus_out: V,
        ) -> super::Building<Children, (Props, super::props::on_focus_out<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_focus_out(on_focus_out)),
            })
        }
        #[inline(always)]
        pub fn on_fullscreen_change<V>(
            self,
            on_fullscreen_change: V,
        ) -> super::Building<Children, (Props, super::props::on_fullscreen_change<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_fullscreen_change(on_fullscreen_change)),
            })
        }
        #[inline(always)]
        pub fn on_fullscreen_error<V>(
            self,
            on_fullscreen_error: V,
        ) -> super::Building<Children, (Props, super::props::on_fullscreen_error<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_fullscreen_error(on_fullscreen_error)),
            })
        }
        #[inline(always)]
        pub fn on_key_down<V>(
            self,
            on_key_down: V,
        ) -> super::Building<Children, (Props, super::props::on_key_down<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_key_down(on_key_down)),
            })
        }
        #[inline(always)]
        pub fn on_key_up<V>(
            self,
            on_key_up: V,
        ) -> super::Building<Children, (Props, super::props::on_key_up<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::props::on_key_up(on_key_up)),
            })
        }
        #[inline(always)]
        pub fn on_aux_click<V>(
            self,
            on_aux_click: V,
        ) -> super::Building<Children, (Props, super::props::on_aux_click<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_aux_click(on_aux_click)),
            })
        }
        #[inline(always)]
        pub fn on_click<V>(
            self,
            on_click: V,
        ) -> super::Building<Children, (Props, super::props::on_click<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::props::on_click(on_click)),
            })
        }
        #[inline(always)]
        pub fn on_context_menu<V>(
            self,
            on_context_menu: V,
        ) -> super::Building<Children, (Props, super::props::on_context_menu<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_context_menu(on_context_menu)),
            })
        }
        #[inline(always)]
        pub fn on_double_click<V>(
            self,
            on_double_click: V,
        ) -> super::Building<Children, (Props, super::props::on_double_click<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_double_click(on_double_click)),
            })
        }
        #[inline(always)]
        pub fn on_mouse_down<V>(
            self,
            on_mouse_down: V,
        ) -> super::Building<Children, (Props, super::props::on_mouse_down<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_mouse_down(on_mouse_down)),
            })
        }
        #[inline(always)]
        pub fn on_mouse_enter<V>(
            self,
            on_mouse_enter: V,
        ) -> super::Building<Children, (Props, super::props::on_mouse_enter<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_mouse_enter(on_mouse_enter)),
            })
        }
        #[inline(always)]
        pub fn on_mouse_leave<V>(
            self,
            on_mouse_leave: V,
        ) -> super::Building<Children, (Props, super::props::on_mouse_leave<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_mouse_leave(on_mouse_leave)),
            })
        }
        #[inline(always)]
        pub fn on_mouse_move<V>(
            self,
            on_mouse_move: V,
        ) -> super::Building<Children, (Props, super::props::on_mouse_move<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_mouse_move(on_mouse_move)),
            })
        }
        #[inline(always)]
        pub fn on_mouse_out<V>(
            self,
            on_mouse_out: V,
        ) -> super::Building<Children, (Props, super::props::on_mouse_out<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_mouse_out(on_mouse_out)),
            })
        }
        #[inline(always)]
        pub fn on_mouse_over<V>(
            self,
            on_mouse_over: V,
        ) -> super::Building<Children, (Props, super::props::on_mouse_over<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_mouse_over(on_mouse_over)),
            })
        }
        #[inline(always)]
        pub fn on_mouse_up<V>(
            self,
            on_mouse_up: V,
        ) -> super::Building<Children, (Props, super::props::on_mouse_up<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_mouse_up(on_mouse_up)),
            })
        }
        #[inline(always)]
        pub fn on_touch_cancel<V>(
            self,
            on_touch_cancel: V,
        ) -> super::Building<Children, (Props, super::props::on_touch_cancel<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_touch_cancel(on_touch_cancel)),
            })
        }
        #[inline(always)]
        pub fn on_touch_end<V>(
            self,
            on_touch_end: V,
        ) -> super::Building<Children, (Props, super::props::on_touch_end<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_touch_end(on_touch_end)),
            })
        }
        #[inline(always)]
        pub fn on_touch_move<V>(
            self,
            on_touch_move: V,
        ) -> super::Building<Children, (Props, super::props::on_touch_move<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_touch_move(on_touch_move)),
            })
        }
        #[inline(always)]
        pub fn on_touch_start<V>(
            self,
            on_touch_start: V,
        ) -> super::Building<Children, (Props, super::props::on_touch_start<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_touch_start(on_touch_start)),
            })
        }
    }
}
#[cfg(feature = "dom")]
mod impl_update_element {
    #[allow(unused_imports)]
    use super::super::*;
    impl<E, Children, Props> crate::imports::frender_dom::props::UpdateElement<E>
        for super::Data<Children, Props>
    where
        crate::imports::frender_html_simple::ElementProps<Children, Props>:
            crate::imports::frender_dom::props::UpdateElement<E>,
    {
        type State = <crate::imports::frender_html_simple::ElementProps<
            Children,
            Props,
        > as crate::imports::frender_dom::props::UpdateElement<E>>::State;
        #[inline(always)]
        fn initialize_state(
            this: Self,
            element: &E,
            children_ctx: &mut ::frender_dom::Dom,
        ) -> Self::State {
            crate::imports::frender_html_simple::ElementProps::<Children, Props>::initialize_state(
                this.props,
                element,
                children_ctx,
            )
        }
        #[inline(always)]
        fn update_element(
            this: Self,
            element: &E,
            children_ctx: &mut ::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            crate::imports::frender_html_simple::ElementProps::<Children, Props>::update_element(
                this.props,
                element,
                children_ctx,
                state,
            )
        }
    }
}
#[cfg(feature = "ssr")]
mod impl_into_ssr_data {
    #[allow(unused_imports)]
    use super::super::*;
    impl<W: ::frender_ssr::AsyncWrite + ::core::marker::Unpin, Children, Props>
        crate::imports::frender_ssr::IntoSsrData<W> for super::Data<Children, Props>
    where
        crate::imports::frender_html_simple::ElementProps<Children, Props>:
            crate::imports::frender_ssr::IntoSsrData<W>,
    {
        type Children = <crate::imports::frender_html_simple::ElementProps<
            Children,
            Props,
        > as crate::imports::frender_ssr::IntoSsrData<W>>::Children;
        type ChildrenRenderState = <crate::imports::frender_html_simple::ElementProps<
            Children,
            Props,
        > as crate::imports::frender_ssr::IntoSsrData<W>>::ChildrenRenderState;
        type Attrs = <crate::imports::frender_html_simple::ElementProps<
            Children,
            Props,
        > as crate::imports::frender_ssr::IntoSsrData<W>>::Attrs;
        fn into_ssr_data(this: Self) -> (Self::Children, Self::Attrs) {
            crate::imports::frender_ssr::IntoSsrData::<W>::into_ssr_data(this.props)
        }
    }
}
mod imports {
    use super::super::*;
    pub(super) use crate::imports::frender_html_simple::{def_props, inherit_props_from};
}
use imports::{def_props, inherit_props_from};
