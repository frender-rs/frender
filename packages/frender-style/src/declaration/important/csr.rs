use frender_reactive_value::non_reactive::Uncached;

use crate::css_style_declaration::Priority;

pub trait IntoCsrDeclarationImportant {
    type StaticCache: 'static;

    fn match_cache(&self, cache: &Self::StaticCache) -> bool;

    fn into_static_cache(self) -> Self::StaticCache;
    fn update_into_cache(self, cache: &mut Self::StaticCache);

    fn update_style(&self, style: impl UpdateStyleWithDeclarationImportant);
}

impl IntoCsrDeclarationImportant for frender_common::Empty {
    type StaticCache = ();

    fn match_cache(&self, (): &Self::StaticCache) -> bool {
        true
    }

    fn into_static_cache(self) -> Self::StaticCache {}

    fn update_into_cache(self, (): &mut Self::StaticCache) {}

    fn update_style(&self, style: impl UpdateStyleWithDeclarationImportant) {
        style.update_not_important()
    }
}

impl IntoCsrDeclarationImportant for bool {
    type StaticCache = Self;

    fn match_cache(&self, cache: &Self::StaticCache) -> bool {
        self == cache
    }

    fn into_static_cache(self) -> Self::StaticCache {
        self
    }

    fn update_into_cache(self, cache: &mut Self::StaticCache) {
        *cache = self
    }

    fn update_style(&self, style: impl UpdateStyleWithDeclarationImportant) {
        style.update_with_priority(Priority::from_bool(*self))
    }
}

impl IntoCsrDeclarationImportant for Uncached<bool> {
    type StaticCache = ();

    fn match_cache(&self, (): &Self::StaticCache) -> bool {
        false
    }

    fn into_static_cache(self) -> Self::StaticCache {}

    fn update_into_cache(self, _: &mut Self::StaticCache) {}

    fn update_style(&self, style: impl UpdateStyleWithDeclarationImportant) {
        self.0.update_style(style)
    }
}

pub trait UpdateStyleWithDeclarationImportant {
    fn update_not_important(self);
    fn update_with_priority(self, priority: Priority);
}
