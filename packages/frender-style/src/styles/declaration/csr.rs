use frender_common::ToStaticCache as _;

use crate::{
    csr::{CsrStyle, CssStyleDeclaration, Priority},
    declaration::{
        important::{
            csr::{CsrDeclarationImportant, UpdateStyleWithDeclarationImportant},
            IntoDeclarationImportant,
        },
        name::csr::{CsrDeclarationName, UpdateStyleWithDeclarationName},
        value::csr::{CsrDeclarationValue, UpdateStyleWithDeclarationValue},
        Declaration, DeclarationName, DeclarationValue, IntoDeclaration, IntoDeclarationAsStyle,
    },
};

pub struct State<N, V, I> {
    name: N,
    value: V,
    important: I,
    removed: bool,
}

impl<D: IntoDeclaration> CsrStyle for IntoDeclarationAsStyle<D> {
    type UpdateWithState = Option<
        State<
            <D::Name as CsrDeclarationName>::StaticCache,
            <D::Value as CsrDeclarationValue>::StaticCache,
            <D::Important as CsrDeclarationImportant>::StaticCache,
        >,
    >;

    fn update_with_state(
        this: Self,
        state: &mut Self::UpdateWithState,
        style: &mut impl CssStyleDeclaration,
    ) {
        let Declaration {
            name,
            value,
            important,
        } = this.0.into_declaration();

        let name = <D::Name>::into_cacheable(name);
        let value = <D::Value>::into_cacheable(value);

        if let Some(cache) = state {
            if !cache.removed && name.match_cache(&cache.name) {
                if value.match_cache(&cache.value) {
                    if important.match_cache(&cache.important) {
                        // all cache matched
                        // doesn't nothing
                    } else {
                        StyleWithAllReady::<_, D::Name, D::Value, D::Important> {
                            style,
                            name: &name,
                            value: &value,
                            important: &important,
                        }
                        .update();
                        important.update_into_cache(&mut cache.important);
                    }
                } else {
                    StyleWithAllReady::<_, D::Name, D::Value, D::Important> {
                        style,
                        name: &name,
                        value: &value,
                        important: &important,
                    }
                    .update();
                    value.update_into_static_cache(&mut cache.value);
                    important.update_into_cache(&mut cache.important);
                }
            } else {
                <D::Name>::update_style(
                    &name,
                    StyleWithValueAndImportantReady::<_, D::Value, D::Important> {
                        style,
                        value: &value,
                        important: &important,
                    },
                );

                cache.removed = false;
                name.update_into_static_cache(&mut cache.name);
                value.update_into_static_cache(&mut cache.value);
                important.update_into_cache(&mut cache.important);
            }
        } else {
            StyleWithAllReady::<_, D::Name, D::Value, D::Important> {
                style,
                name: &name,
                value: &value,
                important: &important,
            }
            .update();

            *state = Some(State {
                name: name.into_static_cache(),
                value: value.into_static_cache(),
                important: important.into_static_cache(),
                removed: false,
            })
        }
    }

    fn remove_with_state(state: &mut Self::UpdateWithState, style: &mut impl CssStyleDeclaration) {
        if let Some(state) = state {
            if !state.removed {
                <D::Name>::remove_style(&state.name, style);
                state.removed = true;
            }
        }
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

struct StyleWithNameProvided<'a, S: CssStyleDeclaration, N, I: CsrDeclarationImportant> {
    style: &'a mut S,
    name: N,
    important: &'a I,
}

impl<'a, S: CssStyleDeclaration, N, I: CsrDeclarationImportant> UpdateStyleWithDeclarationValue
    for StyleWithNameProvided<'a, S, N, I>
where
    for<'v> StyleWithNameValueProvided<'a, S, N, DeclarationValue<&'v str>>:
        UpdateStyleWithDeclarationImportant,
    for<'v> StyleWithNameValueProvided<'a, S, N, &'v str>: UpdateStyleWithDeclarationImportant,
{
    fn update_style_with_declaration_value(self, value: DeclarationValue<&str>) {
        I::update_style(
            self.important,
            StyleWithNameValueProvided {
                style: self.style,
                name: self.name,
                value,
            },
        )
    }

    fn update_style_with_declaration_value_str(self, value: &str) {
        I::update_style(
            self.important,
            StyleWithNameValueProvided {
                style: self.style,
                name: self.name,
                value,
            },
        )
    }
}

struct StyleWithValueAndImportantReady<
    'a,
    S: CssStyleDeclaration,
    V: CsrDeclarationValue,
    I: CsrDeclarationImportant,
> {
    style: &'a mut S,
    value: &'a V::Cacheable,
    important: &'a I,
}

impl<'a, S: CssStyleDeclaration, V: CsrDeclarationValue, I: CsrDeclarationImportant>
    UpdateStyleWithDeclarationName for StyleWithValueAndImportantReady<'a, S, V, I>
where
    for<'n> StyleWithNameProvided<'a, S, DeclarationName<&'n str>, I>:
        UpdateStyleWithDeclarationValue,
    for<'n> StyleWithNameProvided<'a, S, &'n str, I>: UpdateStyleWithDeclarationValue,
{
    fn update_style_with_declaration_name(self, name: DeclarationName<&str>) {
        V::update_style(
            self.value,
            StyleWithNameProvided {
                style: self.style,
                name,
                important: self.important,
            },
        )
    }

    fn update_style_with_declaration_name_str(self, name: &str) {
        V::update_style(
            self.value,
            StyleWithNameProvided {
                style: self.style,
                name,
                important: self.important,
            },
        )
    }
}

struct StyleWithAllReady<
    'a,
    S: CssStyleDeclaration,
    N: CsrDeclarationName,
    V: CsrDeclarationValue,
    I: CsrDeclarationImportant,
> {
    style: &'a mut S,
    name: &'a N::Cacheable,
    value: &'a V::Cacheable,
    important: &'a I,
}

impl<
        'a,
        S: CssStyleDeclaration,
        N: CsrDeclarationName,
        V: CsrDeclarationValue,
        I: CsrDeclarationImportant,
    > StyleWithAllReady<'a, S, N, V, I>
{
    fn update(self) {
        N::update_style(
            self.name,
            StyleWithValueAndImportantReady::<S, V, I> {
                style: self.style,
                value: self.value,
                important: self.important,
            },
        );
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
