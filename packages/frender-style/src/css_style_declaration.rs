use crate::declaration::{DeclarationName, DeclarationValue};

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
