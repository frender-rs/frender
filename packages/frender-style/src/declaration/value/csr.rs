use frender_reactive_value::{
    non_reactive::{CacheCanProvideValue, CacheProvideValue, CachedNonReactiveValue},
    temp_ref::TempRef,
    value_kind::KindOfTempRef,
    ProvideValueOfKind,
};

use super::DeclarationValue;

pub trait UpdateStyleWithDeclarationValue {
    type Output;
    fn update_style_with_declaration_value(self, value: DeclarationValue<&str>) -> Self::Output;
    fn update_style_with_declaration_value_str(self, value: &str) -> Self::Output;
}

pub trait IntoCsrDeclarationValue {
    type Cache;

    fn match_cache(this: &Self, cache: &Self::Cache) -> bool;
    fn not_match_cache(this: &Self, cache: &Self::Cache) -> bool;

    fn into_cache_and_render<Out>(
        this: Self,
        style: impl UpdateStyleWithDeclarationValue<Output = Out>,
    ) -> (Self::Cache, Out);

    fn update_into_cache_and_render<Out>(
        this: Self,
        style: impl UpdateStyleWithDeclarationValue<Output = Out>,
        cache: &mut Self::Cache,
    ) -> Out;

    fn into_render<Out>(
        this: Self,
        style: impl UpdateStyleWithDeclarationValue<Output = Out>,
    ) -> Out;
}

pub trait CsrDeclarationValueStr:
    CachedNonReactiveValue<KindOfTempRef<str>, CacheCanProvideValue = CacheCanProvideValue>
{
}

impl<S: CachedNonReactiveValue<KindOfTempRef<str>>> IntoCsrDeclarationValue for S {
    type Cache = S::Cache;

    fn match_cache(this: &Self, cache: &Self::Cache) -> bool {
        this.match_cache(cache)
    }

    fn not_match_cache(this: &Self, cache: &Self::Cache) -> bool {
        this.not_match_cache(cache)
    }

    fn into_cache_and_render<Out>(
        this: Self,
        style: impl UpdateStyleWithDeclarationValue<Output = Out>,
    ) -> (Self::Cache, Out) {
        this.into_cache_and_render(render_str(style))
    }

    fn update_into_cache_and_render<Out>(
        this: Self,
        style: impl UpdateStyleWithDeclarationValue<Output = Out>,
        cache: &mut Self::Cache,
    ) -> Out {
        this.update_into_cache_and_render(render_str(style), cache)
    }

    fn into_render<Out>(
        this: Self,
        style: impl UpdateStyleWithDeclarationValue<Output = Out>,
    ) -> Out {
        this.cached_into_provide_value()
            .provide_value_of_kind(render_str(style))
    }
}

pub struct DeclarationValueCache<T>(T);

impl<S: CachedNonReactiveValue<KindOfTempRef<str>>> IntoCsrDeclarationValue
    for DeclarationValue<S>
{
    type Cache = DeclarationValueCache<S::Cache>;

    fn match_cache(this: &Self, cache: &Self::Cache) -> bool {
        this.unparsed.match_cache(&cache.0)
    }

    fn not_match_cache(this: &Self, cache: &Self::Cache) -> bool {
        this.unparsed.not_match_cache(&cache.0)
    }

    fn into_cache_and_render<Out>(
        this: Self,
        style: impl UpdateStyleWithDeclarationValue<Output = Out>,
    ) -> (Self::Cache, Out) {
        let (cache, out) = this.unparsed.into_cache_and_render(render_parsable(style));
        (DeclarationValueCache(cache), out)
    }

    fn update_into_cache_and_render<Out>(
        this: Self,
        style: impl UpdateStyleWithDeclarationValue<Output = Out>,
        cache: &mut Self::Cache,
    ) -> Out {
        this.unparsed
            .update_into_cache_and_render(render_parsable(style), &mut cache.0)
    }

    fn into_render<Out>(
        this: Self,
        style: impl UpdateStyleWithDeclarationValue<Output = Out>,
    ) -> Out {
        this.unparsed
            .cached_into_provide_value()
            .provide_value_of_kind(render_parsable(style))
    }
}

fn render_str<S: UpdateStyleWithDeclarationValue>(
    style: S,
) -> impl FnOnce(TempRef<str>) -> S::Output {
    |TempRef(value)| style.update_style_with_declaration_value_str(value)
}

/// Assumes `value` to be able to be parsed as [`DeclarationValue`] without panic.
fn render_parsable<S: UpdateStyleWithDeclarationValue>(
    style: S,
) -> impl FnOnce(TempRef<str>) -> S::Output {
    |TempRef(value)| style.update_style_with_declaration_value(DeclarationValue { unparsed: value })
}
