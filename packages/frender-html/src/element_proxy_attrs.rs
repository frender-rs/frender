pub use event_listener::EventListener;

pub struct ElementProxyAttrs<E: ?Sized>(pub E);

pub(crate) mod proxy_attr {
    use frender_common::impl_many;
    use frender_dom::behaviors;

    pub(crate) trait ProxyAttribute {
        fn proxy_attribute<R: ?Sized>(el: &mut (impl ?Sized + behaviors::Element<R>), renderer: &mut R, name: &str, value: Self);
    }

    impl ProxyAttribute for Option<&str> {
        fn proxy_attribute<R: ?Sized>(el: &mut (impl ?Sized + behaviors::Element<R>), renderer: &mut R, name: &str, value: Self) {
            if let Some(value) = value {
                el.set_attribute(renderer, name, value)
            } else {
                el.remove_attribute(renderer, name)
            }
        }
    }

    impl ProxyAttribute for &str {
        fn proxy_attribute<R: ?Sized>(el: &mut (impl ?Sized + behaviors::Element<R>), renderer: &mut R, name: &str, value: Self) {
            el.set_attribute(renderer, name, value)
        }
    }

    impl_many!(
        impl<__> ProxyAttribute for each_of![i32, u32, f64] {
            fn proxy_attribute<R: ?Sized>(el: &mut (impl ?Sized + behaviors::Element<R>), renderer: &mut R, name: &str, value: Self) {
                el.set_attribute(renderer, name, &value.to_string()) // optimize
            }
        }
    );

    impl ProxyAttribute for bool {
        fn proxy_attribute<R: ?Sized>(el: &mut (impl ?Sized + behaviors::Element<R>), renderer: &mut R, name: &str, value: Self) {
            if value {
                el.set_attribute(renderer, name, "")
            } else {
                el.remove_attribute(renderer, name)
            }
        }
    }
}

mod event_listener {
    use frender_dom::RegisterOrUpdate;

    use super::ElementProxyAttrs;

    pin_project_lite::pin_project!(
        #[derive(Debug, Default)]
        pub struct EventListener<EL> {
            #[pin]
            inner: EL,
        }
    );

    impl<EL, E: ?Sized, R: ?Sized, F> RegisterOrUpdate<ElementProxyAttrs<E>, R, F> for EventListener<EL>
    where
        EL: RegisterOrUpdate<E, R, F>,
    {
        fn register_or_update(self: std::pin::Pin<&mut Self>, node: &mut ElementProxyAttrs<E>, renderer: &mut R, f: F) {
            self.project().inner.register_or_update(&mut node.0, renderer, f)
        }
    }

    impl<R: ?Sized, ET: frender_dom::event_types::EventType, E: ?Sized + frender_dom::OnEvent<R, ET>> frender_dom::OnEvent<R, ET> for ElementProxyAttrs<E> {
        type EventListener<F: frender_dom::HandleEvent<<ET as frender_dom::event_types::EventType>::Event> + 'static> = EventListener<E::EventListener<F>>;
        type EventListenerUnpinned<F: frender_dom::HandleEvent<<ET as frender_dom::event_types::EventType>::Event> + 'static> = EventListener<E::EventListenerUnpinned<F>>;
    }
}

mod dom {
    use frender_dom::behaviors;

    use super::ElementProxyAttrs;

    impl<R: ?Sized, E: ?Sized + behaviors::Node<R>> behaviors::Node<R> for ElementProxyAttrs<E> {
        fn log_self(&self, renderer: &mut R) {
            self.0.log_self(renderer)
        }

        fn warn_self_with_message(&self, renderer: &mut R, message: &str) {
            self.0.warn_self_with_message(renderer, message)
        }

        fn cursor_is_at_self(&self, render_context: &<R>::RenderContext<'_>) -> bool
        where
            R: frender_dom::render::RenderWithContext,
        {
            self.0.cursor_is_at_self(render_context)
        }

        fn check_and_move_cursor_after_self(&self, render_context: &mut <R>::RenderContext<'_>)
        where
            R: frender_dom::render::RenderWithContext,
        {
            self.0.check_and_move_cursor_after_self(render_context)
        }

        fn readd_self(&mut self, render_context: &mut <R>::RenderContext<'_>, force_reposition: bool)
        where
            R: frender_dom::render::RenderWithContext,
        {
            E::readd_self(&mut self.0, render_context, force_reposition)
        }

        fn remove_self(&mut self, renderer: &mut R) {
            self.0.remove_self(renderer)
        }
    }

    impl<R: ?Sized, E: ?Sized + behaviors::Element<R>> behaviors::Element<R> for ElementProxyAttrs<E> {
        fn set_attribute(&mut self, renderer: &mut R, name: &str, value: &str) {
            self.0.set_attribute(renderer, name, value)
        }

        fn remove_attribute(&mut self, renderer: &mut R, name: &str) {
            self.0.remove_attribute(renderer, name)
        }

        fn set_inner_html(&mut self, renderer: &mut R, value: &str) {
            self.0.set_inner_html(renderer, value)
        }

        fn as_node_ref(&self) -> &(dyn 'static + frender_dom::node_ref::traits::Element) {
            self.0.as_node_ref()
        }
    }

    impl<R: ?Sized, E: ?Sized + behaviors::HtmlElement<R>> behaviors::HtmlElement<R> for ElementProxyAttrs<E> {
        fn set_inner_text(&mut self, renderer: &mut R, value: &str) {
            self.0.set_inner_text(renderer, value)
        }

        fn as_node_ref(&self) -> &(dyn 'static + frender_dom::node_ref::traits::HtmlElement) {
            <E as behaviors::HtmlElement<_>>::as_node_ref(&self.0)
        }
    }

    impl<R: ?Sized, E: ?Sized + behaviors::ElementWithClassList<R>> behaviors::ElementWithClassList<R> for ElementProxyAttrs<E> {
        type ClassList<'a> = E::ClassList<'a>
        where
            Self: 'a,
            R: 'a;

        fn class_list<'a>(&'a mut self, renderer: &'a mut R) -> Self::ClassList<'a> {
            self.0.class_list(renderer)
        }
    }

    impl<R: ?Sized, E: ?Sized + behaviors::ElementWithRelList<R>> behaviors::ElementWithRelList<R> for ElementProxyAttrs<E> {
        type RelList<'a> = E::RelList<'a>
        where
            Self: 'a,
            R: 'a;

        fn rel_list<'a>(&'a mut self, renderer: &'a mut R) -> Self::RelList<'a> {
            self.0.rel_list(renderer)
        }
    }

    impl<R: ?Sized, E: ?Sized + behaviors::ElementWithStyle<R>> behaviors::ElementWithStyle<R> for ElementProxyAttrs<E> {
        type Style<'a> = E::Style<'a>
        where
            Self: 'a,
            R: 'a;

        fn style<'a>(&'a mut self, renderer: &'a mut R) -> Self::Style<'a> {
            self.0.style(renderer)
        }
    }

    impl<R: ?Sized, E: ?Sized + behaviors::ElementWithChildren<R>> behaviors::ElementWithChildren<R> for ElementProxyAttrs<E> {
        fn with_render_context_at_first_child_of_self<Res>(&mut self, renderer: &mut R, f: impl FnOnce(&mut R::RenderContext<'_>) -> Res) -> Res
        where
            R: frender_dom::render::RenderWithContext,
        {
            self.0.with_render_context_at_first_child_of_self(renderer, f)
        }
    }
}

mod form_control {
    use frender_form_control::value::FormControlValueKind;

    use frender_form_control::element::FormControlElement;

    use super::ElementProxyAttrs;
    impl<
            //
            E: ?Sized + FormControlElement<V, Renderer>,
            V: ?Sized + FormControlValueKind,
            Renderer: ?Sized,
        > FormControlElement<V, Renderer> for ElementProxyAttrs<E>
    {
        fn set_default_value(&mut self, renderer: &mut Renderer, value: &V) {
            self.0.set_default_value(renderer, value)
        }

        fn set_value(&mut self, renderer: &mut Renderer, value: &V) {
            self.0.set_value(renderer, value)
        }

        fn remove_value(&mut self, renderer: &mut Renderer) {
            self.0.remove_value(renderer)
        }

        type OnValueChangeEventListener<F: frender_form_control::value::HandleFormControlValue<V> + 'static> = E::OnValueChangeEventListener<F>;

        fn on_value_change<F: frender_form_control::value::HandleFormControlValue<V> + 'static>(&mut self, renderer: &mut Renderer, state: &mut Self::OnValueChangeEventListener<F>, f: F) {
            self.0.on_value_change(renderer, state, f)
        }
    }
}

pub(crate) mod macros {
    macro_rules! impl_behavior_fn {
        ($fn_name:ident ($value:ident : event![
            $event_trait_name:ident,
            $event_type_name:literal,
            $event_type_ident:ident,
            $event_type_listener_ident:ident $(,)?
        ]); $trait_name:tt) => {
        };
        ($fn_name:ident ($value:ident : attr_value![$maybe_ty:ty]) {
            $(alias! $alias:tt;)?
            $(attr_name! $attr_name:tt;)?
            $(update_with! $update_with:tt;)?
        } $trait_name:tt) => {
            crate::element_proxy_attrs::macros::impl_behavior_fn_update_with! {
                $(update_with $update_with)?
                value($value)
                type($maybe_ty)
                trait_name $trait_name
                attr_name(::frender_common::expand!({$($attr_name)?} or (stringify!($fn_name))))
            }
        };
        ($fn_name:ident $fn_args:tt $fn_body_or_semi:tt $trait_name:tt) => {};
    }

    macro_rules! impl_behavior_fn_update_with {
        (
            // no update_with
            value $($rest:tt)*
        ) => {};
        (
            update_with($set_attribute_ident:ident $(, $(web_sys_name = $web_sys_name:ident $(,)?)?)? )
            value($value:ident)
            type($maybe_ty:ty)
            trait_name($trait_name:ident $($only_for_types:tt)?)
            attr_name($attr_name:expr)
        ) => {
            fn $set_attribute_ident(&mut self, renderer: &mut Renderer, $value: $maybe_ty) {
                crate::element_proxy_attrs::proxy_attr::ProxyAttribute::proxy_attribute(self, renderer, $attr_name, $value)
            }
        };
        (
            update_with($set_attribute_ident:ident, custom_type!($custom_type:ty), impl_with! $impl_with:tt $(,)?)
            value($value:ident)
            type($maybe_ty:ty)
            trait_name($trait_name:ident $($only_for_types:tt)?)
            attr_name($attr_name:expr)
        ) => {
            fn $set_attribute_ident(&mut self, renderer: &mut Renderer, $value: $custom_type) {
                crate::element_proxy_attrs::proxy_attr::ProxyAttribute::proxy_attribute(self, renderer, $attr_name, $value)
            }
        };
    }

    pub(crate) use impl_behavior_fn;
    pub(crate) use impl_behavior_fn_update_with;
}
