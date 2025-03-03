use frender_reactive_value::{
    non_reactive::{CacheCanProvideValue, CacheProvideValue, CachedNonReactiveValue},
    temp_ref::TempRef,
    value_kind::KindOfTempRef,
    ProvideValueOfKind,
};

use crate::css_style_declaration::CssStyleDeclaration;

use super::DeclarationName;

/// A trait alias.
pub trait CsrStr:
    CachedNonReactiveValue<KindOfTempRef<str>, CacheCanProvideValue = CacheCanProvideValue>
{
}

impl<
        T: ?Sized
            + CachedNonReactiveValue<KindOfTempRef<str>, CacheCanProvideValue = CacheCanProvideValue>,
    > CsrStr for T
{
}

pub trait UpdateStyleWithDeclarationName {
    type Output;
    fn update_style_with_declaration_name(self, name: DeclarationName<&str>) -> Self::Output;
    fn update_style_with_declaration_name_str(self, name: &str) -> Self::Output;
}

pub trait CsrDeclarationNameCache {
    /// Takes `&mut self` instead of `self` to match [`CsrStyleStateUnmount::csr_style_state_unmount()`](crate::csr::CsrStyleStateUnmount::csr_style_state_unmount).
    fn remove_style(&mut self, style: &mut impl CssStyleDeclaration);
}

pub trait IntoCsrDeclarationName {
    type Cache: CsrDeclarationNameCache;

    fn match_cache(this: &Self, cache: &Self::Cache) -> bool;
    fn not_match_cache(this: &Self, cache: &Self::Cache) -> bool;

    fn into_cache_and_render<Out>(
        this: Self,
        style: impl UpdateStyleWithDeclarationName<Output = Out>,
    ) -> (Self::Cache, Out);
    fn update_into_cache_and_render<Out>(
        this: Self,
        cache: &mut Self::Cache,
        style: impl UpdateStyleWithDeclarationName<Output = Out>,
    ) -> Out;

    fn into_render<Out>(
        this: Self,
        style: impl UpdateStyleWithDeclarationName<Output = Out>,
    ) -> Out;
}

pub struct DeclarationNameStrCache<S: CacheProvideValue<KindOfTempRef<str>>>(pub S);

impl<S: CacheProvideValue<KindOfTempRef<str>>> CsrDeclarationNameCache
    for DeclarationNameStrCache<S>
{
    fn remove_style(&mut self, style: &mut impl CssStyleDeclaration) {
        self.0
            .cache_provide_value(|TempRef(value)| style.remove_property_str(value))
    }
}

impl<S: CsrStr> IntoCsrDeclarationName for S {
    type Cache = DeclarationNameStrCache<<S as CachedNonReactiveValue<KindOfTempRef<str>>>::Cache>;

    fn match_cache(this: &Self, cache: &Self::Cache) -> bool {
        S::match_cache(this, &cache.0)
    }

    fn not_match_cache(this: &Self, cache: &Self::Cache) -> bool {
        S::not_match_cache(this, &cache.0)
    }

    fn into_cache_and_render<Out>(
        this: Self,
        style: impl UpdateStyleWithDeclarationName<Output = Out>,
    ) -> (Self::Cache, Out) {
        let (cache, out) = S::into_cache_and_render(this, render_str(style));
        (DeclarationNameStrCache(cache), out)
    }

    fn update_into_cache_and_render<Out>(
        this: Self,
        cache: &mut Self::Cache,
        style: impl UpdateStyleWithDeclarationName<Output = Out>,
    ) -> Out {
        S::update_into_cache_and_render(this, render_str(style), &mut cache.0)
    }

    fn into_render<Out>(
        this: Self,
        style: impl UpdateStyleWithDeclarationName<Output = Out>,
    ) -> Out {
        this.cached_into_provide_value()
            .provide_value_of_kind(render_str(style))
    }
}

pub struct DeclarationNameCache<S: CacheProvideValue<KindOfTempRef<str>>>(S);

impl<S: CacheProvideValue<KindOfTempRef<str>>> CsrDeclarationNameCache for DeclarationNameCache<S> {
    fn remove_style(&mut self, style: &mut impl CssStyleDeclaration) {
        self.0
            .cache_provide_value(|TempRef(value)| style.remove_property(DeclarationName(value)))
    }
}

/// This assumes [`CachedNonReactiveValue`] is implemented in the way that the string value doesn't change.
impl<S: CsrStr> IntoCsrDeclarationName for DeclarationName<S> {
    type Cache = DeclarationNameCache<S::Cache>;

    fn match_cache(this: &Self, cache: &Self::Cache) -> bool {
        this.0.match_cache(&cache.0)
    }

    fn not_match_cache(this: &Self, cache: &Self::Cache) -> bool {
        this.0.not_match_cache(&cache.0)
    }

    fn into_cache_and_render<Out>(
        this: Self,
        style: impl UpdateStyleWithDeclarationName<Output = Out>,
    ) -> (Self::Cache, Out) {
        let (cache, out) = this.0.into_cache_and_render(render_parsable(style));
        (DeclarationNameCache(cache), out)
    }

    fn update_into_cache_and_render<Out>(
        this: Self,
        cache: &mut Self::Cache,
        style: impl UpdateStyleWithDeclarationName<Output = Out>,
    ) -> Out {
        this.0
            .update_into_cache_and_render(render_parsable(style), &mut cache.0)
    }

    fn into_render<Out>(
        this: Self,
        style: impl UpdateStyleWithDeclarationName<Output = Out>,
    ) -> Out {
        this.0
            .cached_into_provide_value()
            .provide_value_of_kind(render_parsable(style))
    }
}

fn render_str<S: UpdateStyleWithDeclarationName>(
    style: S,
) -> impl FnOnce(TempRef<'_, str>) -> S::Output {
    |TempRef(value)| style.update_style_with_declaration_name_str(value)
}

/// Assumes `value` to be able to be parsed as [`DeclarationName`] without panic.
fn render_parsable<S: UpdateStyleWithDeclarationName>(
    style: S,
) -> impl FnOnce(TempRef<'_, str>) -> S::Output {
    |TempRef(value)| style.update_style_with_declaration_name(DeclarationName(value))
}
