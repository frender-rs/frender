#[allow(non_snake_case)]
#[inline(always)]
pub fn HtmlElementProps() -> Building {
    Building(Default::default())
}
pub mod data_struct {
    #[allow(unused_imports)]
    use super::super::*;
    #[derive(Debug, Clone, Copy, Default)]
    #[repr(transparent)]
    pub struct HtmlElementProps<
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
    pub struct HtmlElementProps<
        Children = crate::imports::frender_html_simple::AllowChildren,
        Props = (),
    >(pub super::Data<Children, Props>);
}
pub use building_struct::HtmlElementProps as Building;
pub use data_struct::HtmlElementProps as Data;
pub type DataInitial = data_struct::HtmlElementProps;
pub mod prelude {}
#[inline(always)]
pub fn build<Children, Props>(building: Building<Children, Props>) -> Data<Children, Props> {
    building.0
}
pub use build as valid;
pub mod props {
    super::inherit_props_from!(ElementProps);
    super::def_props!(
        access_key,
        auto_capitalize,
        auto_focus,
        context_menu,
        dir,
        draggable,
        enter_key_hint,
        hidden,
        inert,
        input_mode,
        is,
        item_id,
        item_prop,
        item_ref,
        item_scope,
        item_type,
        lang,
        nonce,
        role,
        slot,
        spellcheck,
        style,
        tab_index,
        title,
        translate,
        virtual_keyboard_policy,
        on_invalid,
        on_animation_cancel,
        on_animation_end,
        on_animation_iteration,
        on_animation_start,
        on_before_input,
        on_input,
        on_change,
        on_got_pointer_capture,
        on_lost_pointer_capture,
        on_pointer_cancel,
        on_pointer_down,
        on_pointer_enter,
        on_pointer_leave,
        on_pointer_move,
        on_pointer_out,
        on_pointer_over,
        on_pointer_up,
        on_transition_cancel,
        on_transition_end,
        on_transition_run,
        on_transition_start,
        on_drag,
        on_drag_end,
        on_drag_enter,
        on_drag_leave,
        on_drag_over,
        on_drag_start,
        on_drop,
    );
}
#[cfg(feature = "dom")]
mod props_impl_dom {
    #[allow(unused_imports)]
    use super::super::*;
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::access_key<V>
    {
        type State = super::props::access_key<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::access_key(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_access_key(v),
                    || dom_element.remove_attribute("accesskey").unwrap(),
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
                |v| element.set_access_key(v),
                || dom_element.remove_attribute("accesskey").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::auto_capitalize<V>
    {
        type State = super::props::auto_capitalize<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::auto_capitalize(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "autocapitalize",
                    ),
                    || dom_element.remove_attribute("autocapitalize").unwrap(),
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
                    "autocapitalize",
                ),
                || dom_element.remove_attribute("autocapitalize").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::auto_focus<V>
    {
        type State = super::props::auto_focus<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::auto_focus(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        *v,
                        dom_element,
                        "autofocus",
                    ),
                    || dom_element.remove_attribute("autofocus").unwrap(),
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
                bool,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                    *v,
                    dom_element,
                    "autofocus",
                ),
                || dom_element.remove_attribute("autofocus").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::context_menu<V>
    {
        type State = super::props::context_menu<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::context_menu(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "contextmenu",
                    ),
                    || dom_element.remove_attribute("contextmenu").unwrap(),
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
                    "contextmenu",
                ),
                || dom_element.remove_attribute("contextmenu").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E> for super::props::dir<V>
    {
        type State = super::props::dir<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::dir(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_dir(v),
                    || dom_element.remove_attribute("dir").unwrap(),
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
                |v| element.set_dir(v),
                || dom_element.remove_attribute("dir").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::draggable<V>
    {
        type State = super::props::draggable<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::draggable(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_draggable(*v),
                    || dom_element.remove_attribute("draggable").unwrap(),
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
                bool,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| element.set_draggable(*v),
                || dom_element.remove_attribute("draggable").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::enter_key_hint<V>
    {
        type State = super::props::enter_key_hint<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::enter_key_hint(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "enterkeyhint",
                    ),
                    || dom_element.remove_attribute("enterkeyhint").unwrap(),
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
                    "enterkeyhint",
                ),
                || dom_element.remove_attribute("enterkeyhint").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::hidden<V>
    {
        type State = super::props::hidden<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::hidden(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_hidden(*v),
                    || dom_element.remove_attribute("hidden").unwrap(),
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
                bool,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| element.set_hidden(*v),
                || dom_element.remove_attribute("hidden").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::inert<V>
    {
        type State = super::props::inert<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::inert(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        *v,
                        dom_element,
                        "inert",
                    ),
                    || dom_element.remove_attribute("inert").unwrap(),
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
                bool,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                    *v,
                    dom_element,
                    "inert",
                ),
                || dom_element.remove_attribute("inert").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::input_mode<V>
    {
        type State = super::props::input_mode<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::input_mode(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "inputmode",
                    ),
                    || dom_element.remove_attribute("inputmode").unwrap(),
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
                    "inputmode",
                ),
                || dom_element.remove_attribute("inputmode").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E> for super::props::is<V>
    {
        type State = super::props::is<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::is(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "is",
                    ),
                    || dom_element.remove_attribute("is").unwrap(),
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
                    "is",
                ),
                || dom_element.remove_attribute("is").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::item_id<V>
    {
        type State = super::props::item_id<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::item_id(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "itemid",
                    ),
                    || dom_element.remove_attribute("itemid").unwrap(),
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
                    "itemid",
                ),
                || dom_element.remove_attribute("itemid").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::item_prop<V>
    {
        type State = super::props::item_prop<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::item_prop(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "itemprop",
                    ),
                    || dom_element.remove_attribute("itemprop").unwrap(),
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
                    "itemprop",
                ),
                || dom_element.remove_attribute("itemprop").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::item_ref<V>
    {
        type State = super::props::item_ref<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::item_ref(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "itemref",
                    ),
                    || dom_element.remove_attribute("itemref").unwrap(),
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
                    "itemref",
                ),
                || dom_element.remove_attribute("itemref").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::item_scope<V>
    {
        type State = super::props::item_scope<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::item_scope(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "itemscope",
                    ),
                    || dom_element.remove_attribute("itemscope").unwrap(),
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
                    "itemscope",
                ),
                || dom_element.remove_attribute("itemscope").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::item_type<V>
    {
        type State = super::props::item_type<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::item_type(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "itemtype",
                    ),
                    || dom_element.remove_attribute("itemtype").unwrap(),
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
                    "itemtype",
                ),
                || dom_element.remove_attribute("itemtype").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::lang<V>
    {
        type State = super::props::lang<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::lang(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_lang(v),
                    || dom_element.remove_attribute("lang").unwrap(),
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
                |v| element.set_lang(v),
                || dom_element.remove_attribute("lang").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::nonce<V>
    {
        type State = super::props::nonce<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::nonce(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "nonce",
                    ),
                    || dom_element.remove_attribute("nonce").unwrap(),
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
                    "nonce",
                ),
                || dom_element.remove_attribute("nonce").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::role<V>
    {
        type State = super::props::role<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::role(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "role",
                    ),
                    || dom_element.remove_attribute("role").unwrap(),
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
                    "role",
                ),
                || dom_element.remove_attribute("role").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::slot<V>
    {
        type State = super::props::slot<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::slot(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "slot",
                    ),
                    || dom_element.remove_attribute("slot").unwrap(),
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
                    "slot",
                ),
                || dom_element.remove_attribute("slot").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::spellcheck<V>
    {
        type State = super::props::spellcheck<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::spellcheck(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_spellcheck(*v),
                    || dom_element.remove_attribute("spellcheck").unwrap(),
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
                bool,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| element.set_spellcheck(*v),
                || dom_element.remove_attribute("spellcheck").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::style<V>
    {
        type State = super::props::style<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::style(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "style",
                    ),
                    || dom_element.remove_attribute("style").unwrap(),
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
                    "style",
                ),
                || dom_element.remove_attribute("style").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<i32>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::tab_index<V>
    {
        type State = super::props::tab_index<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::tab_index(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    i32,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_tab_index(*v),
                    || dom_element.remove_attribute("tabindex").unwrap(),
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
                i32,
            >>::maybe_update_value_with_state(
                this.0,
                &mut state.get_mut().0,
                |v| element.set_tab_index(*v),
                || dom_element.remove_attribute("tabindex").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::title<V>
    {
        type State = super::props::title<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::title(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| element.set_title(v),
                    || dom_element.remove_attribute("title").unwrap(),
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
                |v| element.set_title(v),
                || dom_element.remove_attribute("title").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::translate<V>
    {
        type State = super::props::translate<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::translate(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "translate",
                    ),
                    || dom_element.remove_attribute("translate").unwrap(),
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
                    "translate",
                ),
                || dom_element.remove_attribute("translate").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::virtual_keyboard_policy<V>
    {
        type State = super::props::virtual_keyboard_policy<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            let element = dom_element;
            super::props::virtual_keyboard_policy(
                <V as crate::imports::frender_html::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.0,
                    |v| crate::imports::frender_dom::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "virtualkeyboardpolicy",
                    ),
                    || dom_element.remove_attribute("virtualkeyboardpolicy").unwrap(),
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
                    "virtualkeyboardpolicy",
                ),
                || dom_element.remove_attribute("virtualkeyboardpolicy").unwrap(),
            );
        }
    }
    impl<
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::Invalid>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_invalid<V>
    {
        type State = super::props::on_invalid<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_invalid(V::initialize_dom_event_listener_state(this.0, dom_element))
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
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::AnimationCancel>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_animation_cancel<V>
    {
        type State = super::props::on_animation_cancel<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_animation_cancel(V::initialize_dom_event_listener_state(
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
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::AnimationEnd>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_animation_end<V>
    {
        type State = super::props::on_animation_end<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_animation_end(V::initialize_dom_event_listener_state(
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
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::AnimationIteration>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_animation_iteration<V>
    {
        type State = super::props::on_animation_iteration<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_animation_iteration(V::initialize_dom_event_listener_state(
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
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::AnimationStart>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_animation_start<V>
    {
        type State = super::props::on_animation_start<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_animation_start(V::initialize_dom_event_listener_state(
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
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::BeforeInput>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_before_input<V>
    {
        type State = super::props::on_before_input<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_before_input(V::initialize_dom_event_listener_state(
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
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::Input>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_input<V>
    {
        type State = super::props::on_input<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_input(V::initialize_dom_event_listener_state(this.0, dom_element))
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
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::Change>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_change<V>
    {
        type State = super::props::on_change<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_change(V::initialize_dom_event_listener_state(this.0, dom_element))
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
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::GotPointerCapture>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_got_pointer_capture<V>
    {
        type State = super::props::on_got_pointer_capture<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_got_pointer_capture(V::initialize_dom_event_listener_state(
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
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::LostPointerCapture>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_lost_pointer_capture<V>
    {
        type State = super::props::on_lost_pointer_capture<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_lost_pointer_capture(V::initialize_dom_event_listener_state(
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
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::PointerCancel>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_pointer_cancel<V>
    {
        type State = super::props::on_pointer_cancel<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_pointer_cancel(V::initialize_dom_event_listener_state(
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
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::PointerDown>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_pointer_down<V>
    {
        type State = super::props::on_pointer_down<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_pointer_down(V::initialize_dom_event_listener_state(
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
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::PointerEnter>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_pointer_enter<V>
    {
        type State = super::props::on_pointer_enter<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_pointer_enter(V::initialize_dom_event_listener_state(
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
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::PointerLeave>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_pointer_leave<V>
    {
        type State = super::props::on_pointer_leave<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_pointer_leave(V::initialize_dom_event_listener_state(
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
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::PointerMove>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_pointer_move<V>
    {
        type State = super::props::on_pointer_move<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_pointer_move(V::initialize_dom_event_listener_state(
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
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::PointerOut>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_pointer_out<V>
    {
        type State = super::props::on_pointer_out<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_pointer_out(V::initialize_dom_event_listener_state(
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
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::PointerOver>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_pointer_over<V>
    {
        type State = super::props::on_pointer_over<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_pointer_over(V::initialize_dom_event_listener_state(
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
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::PointerUp>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_pointer_up<V>
    {
        type State = super::props::on_pointer_up<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_pointer_up(V::initialize_dom_event_listener_state(this.0, dom_element))
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
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::TransitionCancel>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_transition_cancel<V>
    {
        type State = super::props::on_transition_cancel<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_transition_cancel(V::initialize_dom_event_listener_state(
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
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::TransitionEnd>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_transition_end<V>
    {
        type State = super::props::on_transition_end<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_transition_end(V::initialize_dom_event_listener_state(
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
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::TransitionRun>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_transition_run<V>
    {
        type State = super::props::on_transition_run<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_transition_run(V::initialize_dom_event_listener_state(
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
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::TransitionStart>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_transition_start<V>
    {
        type State = super::props::on_transition_start<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_transition_start(V::initialize_dom_event_listener_state(
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
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::Drag>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_drag<V>
    {
        type State = super::props::on_drag<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_drag(V::initialize_dom_event_listener_state(this.0, dom_element))
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
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::DragEnd>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_drag_end<V>
    {
        type State = super::props::on_drag_end<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_drag_end(V::initialize_dom_event_listener_state(this.0, dom_element))
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
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::DragEnter>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_drag_enter<V>
    {
        type State = super::props::on_drag_enter<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_drag_enter(V::initialize_dom_event_listener_state(this.0, dom_element))
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
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::DragLeave>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_drag_leave<V>
    {
        type State = super::props::on_drag_leave<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_drag_leave(V::initialize_dom_event_listener_state(this.0, dom_element))
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
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::DragOver>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_drag_over<V>
    {
        type State = super::props::on_drag_over<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_drag_over(V::initialize_dom_event_listener_state(this.0, dom_element))
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
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::DragStart>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_drag_start<V>
    {
        type State = super::props::on_drag_start<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_drag_start(V::initialize_dom_event_listener_state(this.0, dom_element))
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
            V: crate::imports::frender_html::props::UpdateDomEventListener<events::Drop>,
            E: ::core::convert::AsRef<web_sys::HtmlElement>,
        > crate::imports::frender_dom::props::UpdateElementNonReactive<E>
        for super::props::on_drop<V>
    {
        type State = super::props::on_drop<V::State>;
        fn initialize_state_non_reactive(
            this: Self,
            element: &E,
            children_ctx: &mut crate::imports::frender_dom::Dom,
        ) -> Self::State {
            let dom_element = element.as_ref();
            super::props::on_drop(V::initialize_dom_event_listener_state(this.0, dom_element))
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
impl_prop_children!(children);
mod builder_and_replacer {
    #[allow(unused_imports)]
    use super::super::*;
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
        #[inline(always)]
        pub fn access_key<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
        >(
            self,
            access_key: V,
        ) -> super::Building<Children, (Props, super::props::access_key<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::access_key(access_key)),
            })
        }
        #[inline(always)]
        pub fn auto_capitalize<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
        >(
            self,
            auto_capitalize: V,
        ) -> super::Building<Children, (Props, super::props::auto_capitalize<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::auto_capitalize(auto_capitalize)),
            })
        }
        #[inline(always)]
        pub fn auto_focus<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
        >(
            self,
            auto_focus: V,
        ) -> super::Building<Children, (Props, super::props::auto_focus<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::auto_focus(auto_focus)),
            })
        }
        #[inline(always)]
        pub fn context_menu<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
        >(
            self,
            context_menu: V,
        ) -> super::Building<Children, (Props, super::props::context_menu<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::context_menu(context_menu)),
            })
        }
        #[inline(always)]
        pub fn dir<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            dir: V,
        ) -> super::Building<Children, (Props, super::props::dir<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::props::dir(dir)),
            })
        }
        #[inline(always)]
        pub fn draggable<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
        >(
            self,
            draggable: V,
        ) -> super::Building<Children, (Props, super::props::draggable<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::props::draggable(draggable)),
            })
        }
        #[inline(always)]
        pub fn enter_key_hint<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
        >(
            self,
            enter_key_hint: V,
        ) -> super::Building<Children, (Props, super::props::enter_key_hint<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::enter_key_hint(enter_key_hint)),
            })
        }
        #[inline(always)]
        pub fn hidden<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>>(
            self,
            hidden: V,
        ) -> super::Building<Children, (Props, super::props::hidden<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::props::hidden(hidden)),
            })
        }
        #[inline(always)]
        pub fn inert<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>>(
            self,
            inert: V,
        ) -> super::Building<Children, (Props, super::props::inert<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::props::inert(inert)),
            })
        }
        #[inline(always)]
        pub fn input_mode<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
        >(
            self,
            input_mode: V,
        ) -> super::Building<Children, (Props, super::props::input_mode<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::input_mode(input_mode)),
            })
        }
        #[inline(always)]
        pub fn is<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            is: V,
        ) -> super::Building<Children, (Props, super::props::is<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::props::is(is)),
            })
        }
        #[inline(always)]
        pub fn item_id<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            item_id: V,
        ) -> super::Building<Children, (Props, super::props::item_id<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::props::item_id(item_id)),
            })
        }
        #[inline(always)]
        pub fn item_prop<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            item_prop: V,
        ) -> super::Building<Children, (Props, super::props::item_prop<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::props::item_prop(item_prop)),
            })
        }
        #[inline(always)]
        pub fn item_ref<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            item_ref: V,
        ) -> super::Building<Children, (Props, super::props::item_ref<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::props::item_ref(item_ref)),
            })
        }
        #[inline(always)]
        pub fn item_scope<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
        >(
            self,
            item_scope: V,
        ) -> super::Building<Children, (Props, super::props::item_scope<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::item_scope(item_scope)),
            })
        }
        #[inline(always)]
        pub fn item_type<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            item_type: V,
        ) -> super::Building<Children, (Props, super::props::item_type<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::props::item_type(item_type)),
            })
        }
        #[inline(always)]
        pub fn lang<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            lang: V,
        ) -> super::Building<Children, (Props, super::props::lang<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::props::lang(lang)),
            })
        }
        #[inline(always)]
        pub fn nonce<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            nonce: V,
        ) -> super::Building<Children, (Props, super::props::nonce<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::props::nonce(nonce)),
            })
        }
        #[inline(always)]
        pub fn role<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            role: V,
        ) -> super::Building<Children, (Props, super::props::role<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::props::role(role)),
            })
        }
        #[inline(always)]
        pub fn slot<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            slot: V,
        ) -> super::Building<Children, (Props, super::props::slot<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::props::slot(slot)),
            })
        }
        #[inline(always)]
        pub fn spellcheck<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<bool>,
        >(
            self,
            spellcheck: V,
        ) -> super::Building<Children, (Props, super::props::spellcheck<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::spellcheck(spellcheck)),
            })
        }
        #[inline(always)]
        pub fn style<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            style: V,
        ) -> super::Building<Children, (Props, super::props::style<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::props::style(style)),
            })
        }
        #[inline(always)]
        pub fn tab_index<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<i32>>(
            self,
            tab_index: V,
        ) -> super::Building<Children, (Props, super::props::tab_index<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::props::tab_index(tab_index)),
            })
        }
        #[inline(always)]
        pub fn title<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            title: V,
        ) -> super::Building<Children, (Props, super::props::title<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::props::title(title)),
            })
        }
        #[inline(always)]
        pub fn translate<V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>>(
            self,
            translate: V,
        ) -> super::Building<Children, (Props, super::props::translate<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::props::translate(translate)),
            })
        }
        #[inline(always)]
        pub fn virtual_keyboard_policy<
            V: crate::imports::frender_html::props::MaybeUpdateValueWithState<str>,
        >(
            self,
            virtual_keyboard_policy: V,
        ) -> super::Building<Children, (Props, super::props::virtual_keyboard_policy<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::virtual_keyboard_policy(
                        virtual_keyboard_policy,
                    )),
            })
        }
        #[inline(always)]
        pub fn on_invalid<V>(
            self,
            on_invalid: V,
        ) -> super::Building<Children, (Props, super::props::on_invalid<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_invalid(on_invalid)),
            })
        }
        #[inline(always)]
        pub fn on_animation_cancel<V>(
            self,
            on_animation_cancel: V,
        ) -> super::Building<Children, (Props, super::props::on_animation_cancel<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_animation_cancel(on_animation_cancel)),
            })
        }
        #[inline(always)]
        pub fn on_animation_end<V>(
            self,
            on_animation_end: V,
        ) -> super::Building<Children, (Props, super::props::on_animation_end<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_animation_end(on_animation_end)),
            })
        }
        #[inline(always)]
        pub fn on_animation_iteration<V>(
            self,
            on_animation_iteration: V,
        ) -> super::Building<Children, (Props, super::props::on_animation_iteration<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_animation_iteration(on_animation_iteration)),
            })
        }
        #[inline(always)]
        pub fn on_animation_start<V>(
            self,
            on_animation_start: V,
        ) -> super::Building<Children, (Props, super::props::on_animation_start<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_animation_start(on_animation_start)),
            })
        }
        #[inline(always)]
        pub fn on_before_input<V>(
            self,
            on_before_input: V,
        ) -> super::Building<Children, (Props, super::props::on_before_input<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_before_input(on_before_input)),
            })
        }
        #[inline(always)]
        pub fn on_input<V>(
            self,
            on_input: V,
        ) -> super::Building<Children, (Props, super::props::on_input<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::props::on_input(on_input)),
            })
        }
        #[inline(always)]
        pub fn on_change<V>(
            self,
            on_change: V,
        ) -> super::Building<Children, (Props, super::props::on_change<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::props::on_change(on_change)),
            })
        }
        #[inline(always)]
        pub fn on_got_pointer_capture<V>(
            self,
            on_got_pointer_capture: V,
        ) -> super::Building<Children, (Props, super::props::on_got_pointer_capture<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_got_pointer_capture(on_got_pointer_capture)),
            })
        }
        #[inline(always)]
        pub fn on_lost_pointer_capture<V>(
            self,
            on_lost_pointer_capture: V,
        ) -> super::Building<Children, (Props, super::props::on_lost_pointer_capture<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_lost_pointer_capture(
                        on_lost_pointer_capture,
                    )),
            })
        }
        #[inline(always)]
        pub fn on_pointer_cancel<V>(
            self,
            on_pointer_cancel: V,
        ) -> super::Building<Children, (Props, super::props::on_pointer_cancel<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_pointer_cancel(on_pointer_cancel)),
            })
        }
        #[inline(always)]
        pub fn on_pointer_down<V>(
            self,
            on_pointer_down: V,
        ) -> super::Building<Children, (Props, super::props::on_pointer_down<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_pointer_down(on_pointer_down)),
            })
        }
        #[inline(always)]
        pub fn on_pointer_enter<V>(
            self,
            on_pointer_enter: V,
        ) -> super::Building<Children, (Props, super::props::on_pointer_enter<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_pointer_enter(on_pointer_enter)),
            })
        }
        #[inline(always)]
        pub fn on_pointer_leave<V>(
            self,
            on_pointer_leave: V,
        ) -> super::Building<Children, (Props, super::props::on_pointer_leave<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_pointer_leave(on_pointer_leave)),
            })
        }
        #[inline(always)]
        pub fn on_pointer_move<V>(
            self,
            on_pointer_move: V,
        ) -> super::Building<Children, (Props, super::props::on_pointer_move<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_pointer_move(on_pointer_move)),
            })
        }
        #[inline(always)]
        pub fn on_pointer_out<V>(
            self,
            on_pointer_out: V,
        ) -> super::Building<Children, (Props, super::props::on_pointer_out<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_pointer_out(on_pointer_out)),
            })
        }
        #[inline(always)]
        pub fn on_pointer_over<V>(
            self,
            on_pointer_over: V,
        ) -> super::Building<Children, (Props, super::props::on_pointer_over<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_pointer_over(on_pointer_over)),
            })
        }
        #[inline(always)]
        pub fn on_pointer_up<V>(
            self,
            on_pointer_up: V,
        ) -> super::Building<Children, (Props, super::props::on_pointer_up<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_pointer_up(on_pointer_up)),
            })
        }
        #[inline(always)]
        pub fn on_transition_cancel<V>(
            self,
            on_transition_cancel: V,
        ) -> super::Building<Children, (Props, super::props::on_transition_cancel<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_transition_cancel(on_transition_cancel)),
            })
        }
        #[inline(always)]
        pub fn on_transition_end<V>(
            self,
            on_transition_end: V,
        ) -> super::Building<Children, (Props, super::props::on_transition_end<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_transition_end(on_transition_end)),
            })
        }
        #[inline(always)]
        pub fn on_transition_run<V>(
            self,
            on_transition_run: V,
        ) -> super::Building<Children, (Props, super::props::on_transition_run<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_transition_run(on_transition_run)),
            })
        }
        #[inline(always)]
        pub fn on_transition_start<V>(
            self,
            on_transition_start: V,
        ) -> super::Building<Children, (Props, super::props::on_transition_start<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_transition_start(on_transition_start)),
            })
        }
        #[inline(always)]
        pub fn on_drag<V>(
            self,
            on_drag: V,
        ) -> super::Building<Children, (Props, super::props::on_drag<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::props::on_drag(on_drag)),
            })
        }
        #[inline(always)]
        pub fn on_drag_end<V>(
            self,
            on_drag_end: V,
        ) -> super::Building<Children, (Props, super::props::on_drag_end<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_drag_end(on_drag_end)),
            })
        }
        #[inline(always)]
        pub fn on_drag_enter<V>(
            self,
            on_drag_enter: V,
        ) -> super::Building<Children, (Props, super::props::on_drag_enter<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_drag_enter(on_drag_enter)),
            })
        }
        #[inline(always)]
        pub fn on_drag_leave<V>(
            self,
            on_drag_leave: V,
        ) -> super::Building<Children, (Props, super::props::on_drag_leave<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_drag_leave(on_drag_leave)),
            })
        }
        #[inline(always)]
        pub fn on_drag_over<V>(
            self,
            on_drag_over: V,
        ) -> super::Building<Children, (Props, super::props::on_drag_over<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_drag_over(on_drag_over)),
            })
        }
        #[inline(always)]
        pub fn on_drag_start<V>(
            self,
            on_drag_start: V,
        ) -> super::Building<Children, (Props, super::props::on_drag_start<V>)> {
            super::Building(super::Data {
                props: self
                    .0
                    .props
                    .chain_prop(super::props::on_drag_start(on_drag_start)),
            })
        }
        #[inline(always)]
        pub fn on_drop<V>(
            self,
            on_drop: V,
        ) -> super::Building<Children, (Props, super::props::on_drop<V>)> {
            super::Building(super::Data {
                props: self.0.props.chain_prop(super::props::on_drop(on_drop)),
            })
        }
    }
}
#[cfg(feature = "dom")]
impl_dom! {}
#[cfg(feature = "ssr")]
impl_ssr! {}
mod imports {
    use super::super::*;
    pub(super) use crate::imports::frender_html_simple::{
        def_props, impl_dom, impl_prop_children, impl_ssr, inherit_props_from,
    };
}
use imports::{def_props, impl_dom, impl_prop_children, impl_ssr, inherit_props_from};
