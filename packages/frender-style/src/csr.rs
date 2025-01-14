use crate::{
    declaration::{DeclarationName, DeclarationValue},
    IntoStyle,
};

pub enum Priority {
    Empty,
    Important,
}

impl Priority {
    pub(crate) fn from_bool(important: bool) -> Priority {
        if important {
            Self::Important
        } else {
            Self::Empty
        }
    }

    /// Returns `true` if the priority is [`Important`].
    ///
    /// [`Important`]: Priority::Important
    #[must_use]
    pub fn is_important(&self) -> bool {
        matches!(self, Self::Important)
    }
}

/// https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration
pub trait CssStyleDeclaration {
    fn remove_property_str(&mut self, property: &str);
    fn remove_property(&mut self, property: DeclarationName<&str>) {
        self.remove_property_str(property.unparsed())
    }

    /// Note: value must not contain "!important" or will panic. That should be set using the priority parameter.
    fn set_property_str_with_value_str_and_priority(
        &mut self,
        property_name: &str,
        value: &str,
        priority: Priority,
    );

    fn set_property_str_with_priority(
        &mut self,
        property_name: &str,
        value: DeclarationValue<&str>,
        priority: Priority,
    ) {
        self.set_property_str_with_value_str_and_priority(
            property_name,
            value.into_unparsed(),
            priority,
        )
    }

    // Note: value must not contain "!important", that should be set using the priority parameter.
    fn set_property_str_with_value_str(&mut self, property_name: &str, value: &str) {
        self.set_property_str_with_value_str_and_priority(property_name, value, Priority::Empty)
    }

    fn set_property_str(&mut self, property_name: &str, value: DeclarationValue<&str>) {
        self.set_property_str_with_value_str(property_name, value.into_unparsed())
    }

    /// Note: value must not contain "!important" or will panic. That should be set using the priority parameter.
    fn set_property_with_value_str_and_priority(
        &mut self,
        property_name: DeclarationName<&str>,
        value: &str,
        priority: Priority,
    ) {
        self.set_property_str_with_value_str_and_priority(
            property_name.into_unparsed(),
            value,
            priority,
        )
    }

    fn set_property_with_priority(
        &mut self,
        property_name: DeclarationName<&str>,
        value: DeclarationValue<&str>,
        priority: Priority,
    ) {
        self.set_property_str_with_priority(property_name.into_unparsed(), value, priority)
    }

    // Note: value must not contain "!important", that should be set using the priority parameter.
    fn set_property_with_value_str(&mut self, property_name: DeclarationName<&str>, value: &str) {
        self.set_property_with_value_str_and_priority(property_name, value, Priority::Empty)
    }

    fn set_property(
        &mut self,
        property_name: DeclarationName<&str>,
        value: DeclarationValue<&str>,
    ) {
        self.set_property_with_value_str(property_name, value.into_unparsed())
    }
}

pub trait CsrStyleStateUnmount {
    /// Takes `&mut Self` instead of `Self` so that:
    /// - [`EitherStyle`](crate::styles::EitherStyle) can ensure old state is unmounted before new state is initialized,
    ///   avoiding the case where they share same style declaration names.
    /// - `Preserved` can be implemented without additional traits.
    fn csr_style_state_unmount(state: &mut Self, style: &mut impl CssStyleDeclaration);
}

impl CsrStyleStateUnmount for () {
    fn csr_style_state_unmount((): &mut Self, _: &mut impl CssStyleDeclaration) {}
}

pub trait CsrStyle {
    type State: CsrStyleStateUnmount;

    fn csr_style_render_init(this: Self, style: &mut impl CssStyleDeclaration) -> Self::State;

    /// The `old_state` was [unmounted](CsrStyleStateUnmount::csr_style_state_unmount).
    fn csr_style_render_init_with_old_state(
        this: Self,
        style: &mut impl CssStyleDeclaration,
        old_state: &mut Self::State,
    ) where
        Self: Sized,
    {
        *old_state = Self::csr_style_render_init(this, style)
    }

    fn csr_style_render_update(
        this: Self,
        style: &mut impl CssStyleDeclaration,
        state: &mut Self::State,
    );
}

impl<S: IntoStyle> CsrStyle for S
where
    S::IntoStyle: CsrStyle,
{
    type State = <S::IntoStyle as CsrStyle>::State;

    fn csr_style_render_init(this: Self, style: &mut impl CssStyleDeclaration) -> Self::State {
        <S::IntoStyle>::csr_style_render_init(this.into_style(), style)
    }

    fn csr_style_render_init_with_old_state(
        this: Self,
        style: &mut impl CssStyleDeclaration,
        old_state: &mut Self::State,
    ) {
        <S::IntoStyle>::csr_style_render_init_with_old_state(this.into_style(), style, old_state)
    }

    fn csr_style_render_update(
        this: Self,
        style: &mut impl CssStyleDeclaration,
        state: &mut Self::State,
    ) {
        <S::IntoStyle>::csr_style_render_update(this.into_style(), style, state)
    }
}
