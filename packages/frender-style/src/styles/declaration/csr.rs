use frender_common::IntoStaticStrCache;

use crate::{
    csr::{CsrStyle, CsrStyleStateUnmount, CssStyleDeclaration, Priority},
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
}

impl<N: CsrStyleStateUnmount, V, I> CsrStyleStateUnmount for State<N, V, I> {
    fn csr_style_state_unmount(state: &mut Self, style: &mut impl CssStyleDeclaration) {
        N::csr_style_state_unmount(&mut state.name, style);
    }
}

impl<D: IntoDeclaration> CsrStyle for IntoDeclarationAsStyle<D> {
    type State = State<
        <D::Name as CsrDeclarationName>::StaticCache,
        <D::Value as CsrDeclarationValue>::StaticCache,
        <D::Important as CsrDeclarationImportant>::StaticCache,
    >;

    fn csr_style_render_init(this: Self, style: &mut impl CssStyleDeclaration) -> Self::State {
        let Declaration {
            name,
            value,
            important,
        } = this.0.into_declaration();

        let name = <D::Name>::into_cacheable(name).into_static_str_cache();
        let value = <D::Value>::into_cacheable(value).into_static_str_cache();

        StyleWithAllReady::<_, D::Name, D::Value, D::Important> {
            style,
            name: &name,
            value: &value,
            important: &important,
        }
        .update();

        State {
            name,
            value,
            important: important.into_static_cache(),
        }
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

        <D::Name>::into_cacheable(name).update_into_static_str_cache(&mut old_state.name);
        <D::Value>::into_cacheable(value).update_into_static_str_cache(&mut old_state.value);

        StyleWithAllReady::<_, D::Name, D::Value, D::Important> {
            style,
            name: &old_state.name,
            value: &old_state.value,
            important: &important,
        }
        .update();

        important.update_into_cache(&mut old_state.important);
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

        if <D::Name>::match_cache(&name, &cache.name) {
            if <D::Value>::match_cache(&value, &cache.value) {
                if important.match_cache(&cache.important) {
                    // all cache matched
                    // doesn't nothing
                } else {
                    StyleWithAllReady::<_, D::Name, D::Value, D::Important> {
                        style,
                        name: &cache.name,
                        value: &cache.value,
                        important: &important,
                    }
                    .update();
                    important.update_into_cache(&mut cache.important);
                }
            } else {
                <D::Value>::into_cacheable(value).update_into_static_str_cache(&mut cache.value);

                StyleWithAllReady::<_, D::Name, D::Value, D::Important> {
                    style,
                    name: &cache.name,
                    value: &cache.value,
                    important: &important,
                }
                .update();
                important.update_into_cache(&mut cache.important);
            }
        } else {
            <D::Name>::into_cacheable(name).update_into_static_str_cache(&mut cache.name);
            <D::Value>::into_cacheable(value).update_into_static_str_cache(&mut cache.value);

            <D::Name>::update_style(
                &cache.name,
                StyleWithValueAndImportantReady::<_, D::Value, D::Important> {
                    style,
                    value: &cache.value,
                    important: &important,
                },
            );

            important.update_into_cache(&mut cache.important);
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
    value: &'a V::StaticCache,
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
    name: &'a N::StaticCache,
    value: &'a V::StaticCache,
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
