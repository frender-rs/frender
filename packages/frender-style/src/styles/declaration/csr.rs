use crate::{
    csr::{CsrStyle, CsrStyleStateUnmount},
    css_style_declaration::{CssStyleDeclaration, Priority},
    declaration::{
        important::{
            csr::{IntoCsrDeclarationImportant, UpdateStyleWithDeclarationImportant},
            IntoDeclarationImportant,
        },
        name::csr::{
            CsrDeclarationNameCache, IntoCsrDeclarationName, UpdateStyleWithDeclarationName,
        },
        value::csr::{IntoCsrDeclarationValue, UpdateStyleWithDeclarationValue},
        Declaration, DeclarationName, DeclarationValue, IntoDeclaration,
    },
};

use super::IntoDeclarationAsStyle;

pub struct State<N, V, I>((N, (V, I)));

macro_rules! state_name {
    ($v:expr) => {
        $v.0 .0
    };
}

macro_rules! state_value {
    ($v:expr) => {
        $v.0 .1 .0
    };
}

macro_rules! state_important {
    ($v:expr) => {
        $v.0 .1 .1
    };
}

impl<N: CsrDeclarationNameCache, V, I> CsrStyleStateUnmount for State<N, V, I> {
    fn csr_style_state_unmount(state: &mut Self, style: &mut impl CssStyleDeclaration) {
        state_name!(state).remove_style(style)
    }
}

impl<D: IntoDeclaration> CsrStyle for IntoDeclarationAsStyle<D> {
    type State = State<
        <D::Name as IntoCsrDeclarationName>::Cache,
        <D::Value as IntoCsrDeclarationValue>::Cache,
        <D::Important as IntoCsrDeclarationImportant>::StaticCache,
    >;

    fn csr_style_render_init(this: Self, style: &mut impl CssStyleDeclaration) -> Self::State {
        let Declaration {
            name,
            value,
            important,
        } = this.0.into_declaration();

        State(D::Name::into_cache_and_render(
            name,
            StyleWithNothing {
                style,
                value,
                important,
            },
        ))
    }

    fn csr_style_render_init_with_old_state(
        this: Self,
        style: &mut impl CssStyleDeclaration,
        old_state: &mut Self::State,
    ) {
        let Declaration {
            name,
            value,
            important,
        } = this.0.into_declaration();

        ((), ()) = D::Name::update_into_cache_and_render(
            name,
            &mut old_state.0 .0,
            StyleWithNothing {
                style,
                value: UpdateIntoCache(value, &mut state_value!(old_state)),
                important: UpdateIntoCache(important, &mut state_important!(old_state)),
            },
        );
    }

    fn csr_style_render_update(
        this: Self,
        style: &mut impl CssStyleDeclaration,
        cache: &mut Self::State,
    ) {
        let Declaration {
            name,
            value,
            important,
        } = this.0.into_declaration();

        if D::Name::match_cache(&name, &state_name!(cache)) {
            if D::Value::match_cache(&value, &state_value!(cache)) {
                if important.match_cache(&state_important!(cache)) {
                    // all cache matched
                    // doesn't nothing
                } else {
                    ((), ()) = D::Name::into_render(
                        name,
                        StyleWithNothing {
                            style,
                            important: UpdateIntoCache(important, &mut state_important!(cache)),
                            value: IntoRender(value),
                        },
                    )
                }
            } else {
                if important.match_cache(&state_important!(cache)) {
                    ((), ()) = D::Name::into_render(
                        name,
                        StyleWithNothing {
                            style,
                            important: IntoRender(important),
                            value: UpdateIntoCache(value, &mut state_value!(cache)),
                        },
                    );
                } else {
                    ((), ()) = D::Name::into_render(
                        name,
                        StyleWithNothing {
                            style,
                            important: UpdateIntoCache(important, &mut state_important!(cache)),
                            value: UpdateIntoCache(value, &mut state_value!(cache)),
                        },
                    );
                }
            }
        } else {
            if D::Value::match_cache(&value, &state_value!(cache)) {
                if important.match_cache(&state_important!(cache)) {
                    // all cache matched
                    // doesn't nothing
                } else {
                    ((), ()) = D::Name::update_into_cache_and_render(
                        name,
                        &mut state_name!(cache),
                        StyleWithNothing {
                            style,
                            important: UpdateIntoCache(important, &mut state_important!(cache)),
                            value: IntoRender(value),
                        },
                    )
                }
            } else {
                if important.match_cache(&state_important!(cache)) {
                    ((), ()) = D::Name::update_into_cache_and_render(
                        name,
                        &mut state_name!(cache),
                        StyleWithNothing {
                            style,
                            important: IntoRender(important),
                            value: UpdateIntoCache(value, &mut state_value!(cache)),
                        },
                    );
                } else {
                    ((), ()) = D::Name::update_into_cache_and_render(
                        name,
                        &mut state_name!(cache),
                        StyleWithNothing {
                            style,
                            important: UpdateIntoCache(important, &mut state_important!(cache)),
                            value: UpdateIntoCache(value, &mut state_value!(cache)),
                        },
                    );
                }
            }
        }
    }
}

struct StyleWithNothing<'a, S, V, I> {
    style: &'a mut S,
    important: I,
    value: V,
}

struct UpdateIntoCache<'a, T, Cache>(T, &'a mut Cache);
struct IntoRender<T>(T);

trait ImplValue {
    type ImplCache;

    fn impl_into_cache_and_render<Out>(
        this: Self,
        style: impl UpdateStyleWithDeclarationValue<Output = Out>,
    ) -> (Self::ImplCache, Out);
}

impl<T: IntoCsrDeclarationValue> ImplValue for T {
    type ImplCache = T::Cache;

    fn impl_into_cache_and_render<Out>(
        this: Self,
        style: impl UpdateStyleWithDeclarationValue<Output = Out>,
    ) -> (Self::ImplCache, Out) {
        T::into_cache_and_render(this, style)
    }
}

impl<T: IntoCsrDeclarationValue> ImplValue for UpdateIntoCache<'_, T, T::Cache> {
    type ImplCache = ();

    fn impl_into_cache_and_render<Out>(
        this: Self,
        style: impl UpdateStyleWithDeclarationValue<Output = Out>,
    ) -> (Self::ImplCache, Out) {
        ((), T::update_into_cache_and_render(this.0, style, this.1))
    }
}

impl<T: IntoCsrDeclarationValue> ImplValue for IntoRender<T> {
    type ImplCache = ();

    fn impl_into_cache_and_render<Out>(
        this: Self,
        style: impl UpdateStyleWithDeclarationValue<Output = Out>,
    ) -> (Self::ImplCache, Out) {
        ((), T::into_render(this.0, style))
    }
}

trait ImplImportant {
    type ImplCache;

    fn impl_into_cache_and_render(
        this: Self,
        style: impl UpdateStyleWithDeclarationImportant,
    ) -> Self::ImplCache;
}

impl<T: IntoCsrDeclarationImportant> ImplImportant for T {
    type ImplCache = T::StaticCache;

    fn impl_into_cache_and_render(
        this: Self,
        style: impl UpdateStyleWithDeclarationImportant,
    ) -> Self::ImplCache {
        this.update_style(style);
        T::into_static_cache(this)
    }
}

impl<T: IntoCsrDeclarationImportant> ImplImportant for UpdateIntoCache<'_, T, T::StaticCache> {
    type ImplCache = ();

    fn impl_into_cache_and_render(
        this: Self,
        style: impl UpdateStyleWithDeclarationImportant,
    ) -> Self::ImplCache {
        this.0.update_style(style);
        T::update_into_cache(this.0, this.1)
    }
}

impl<T: IntoCsrDeclarationImportant> ImplImportant for IntoRender<T> {
    type ImplCache = ();

    fn impl_into_cache_and_render(
        this: Self,
        style: impl UpdateStyleWithDeclarationImportant,
    ) -> Self::ImplCache {
        T::update_style(&this.0, style)
    }
}

impl<S: CssStyleDeclaration, V: ImplValue, I: ImplImportant> UpdateStyleWithDeclarationName
    for StyleWithNothing<'_, S, V, I>
{
    type Output = (V::ImplCache, I::ImplCache);
    fn update_style_with_declaration_name(self, name: DeclarationName<&str>) -> Self::Output {
        V::impl_into_cache_and_render(
            self.value,
            StyleWithNameProvided {
                style: self.style,
                important: self.important,
                name_provided: name,
            },
        )
    }

    fn update_style_with_declaration_name_str(self, name: &str) -> Self::Output {
        V::impl_into_cache_and_render(
            self.value,
            StyleWithNameProvided {
                style: self.style,
                important: self.important,
                name_provided: name,
            },
        )
    }
}

struct StyleWithNameValueProvided<'a, S: CssStyleDeclaration, N, V> {
    style: &'a mut S,
    name: N,
    value: V,
}

impl<S: CssStyleDeclaration> UpdateStyleWithDeclarationImportant
    for StyleWithNameValueProvided<'_, S, DeclarationName<&str>, DeclarationValue<&str>>
{
    fn update_not_important(self) {
        self.style.set_property(self.name, self.value)
    }

    fn update_with_priority(self, priority: Priority) {
        self.style
            .set_property_with_priority(self.name, self.value, priority)
    }
}

impl<S: CssStyleDeclaration> UpdateStyleWithDeclarationImportant
    for StyleWithNameValueProvided<'_, S, DeclarationName<&str>, &str>
{
    fn update_not_important(self) {
        self.style
            .set_property_with_value_str(self.name, self.value)
    }

    fn update_with_priority(self, priority: Priority) {
        self.style
            .set_property_with_value_str_and_priority(self.name, self.value, priority)
    }
}

impl<S: CssStyleDeclaration> UpdateStyleWithDeclarationImportant
    for StyleWithNameValueProvided<'_, S, &str, DeclarationValue<&str>>
{
    fn update_not_important(self) {
        self.style.set_property_str(self.name, self.value)
    }

    fn update_with_priority(self, priority: Priority) {
        self.style
            .set_property_str_with_priority(self.name, self.value, priority)
    }
}

impl<S: CssStyleDeclaration> UpdateStyleWithDeclarationImportant
    for StyleWithNameValueProvided<'_, S, &str, &str>
{
    fn update_not_important(self) {
        self.style
            .set_property_str_with_value_str(self.name, self.value)
    }

    fn update_with_priority(self, priority: Priority) {
        self.style
            .set_property_str_with_value_str_and_priority(self.name, self.value, priority)
    }
}

struct StyleWithNameProvided<'a, S, N, I> {
    style: &'a mut S,
    important: I,
    name_provided: N,
}

impl<'a, S: CssStyleDeclaration, N, I: ImplImportant> UpdateStyleWithDeclarationValue
    for StyleWithNameProvided<'a, S, N, I>
where
    for<'v> StyleWithNameValueProvided<'a, S, N, &'v str>: UpdateStyleWithDeclarationImportant,
    for<'v> StyleWithNameValueProvided<'a, S, N, DeclarationValue<&'v str>>:
        UpdateStyleWithDeclarationImportant,
{
    type Output = I::ImplCache;

    fn update_style_with_declaration_value(self, value: DeclarationValue<&str>) -> Self::Output {
        I::impl_into_cache_and_render(
            self.important,
            StyleWithNameValueProvided {
                style: self.style,
                name: self.name_provided,
                value,
            },
        )
    }

    fn update_style_with_declaration_value_str(self, value: &str) -> Self::Output {
        I::impl_into_cache_and_render(
            self.important,
            StyleWithNameValueProvided {
                style: self.style,
                name: self.name_provided,
                value,
            },
        )
    }
}

pub(crate) fn update_style<I: IntoDeclarationImportant>(
    style: &mut impl CssStyleDeclaration,
    name: DeclarationName<&str>,
    value: DeclarationValue<&str>,
    important: I,
) {
    important.update_style(StyleWithNameValueProvided { style, name, value });
}
