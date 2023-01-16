use crate::element::str::RenderingStr;

pub trait MaybeRenderingStr {
    type State: Default;

    fn maybe_update_rendering_str_with_cache(
        this: Self,
        state: &mut Self::State,
        update: impl FnOnce(&str),
        remove: impl FnOnce(),
    );
}

impl<T: RenderingStr> MaybeRenderingStr for T {
    type State = Option<T::Cache>;

    fn maybe_update_rendering_str_with_cache(
        this: Self,
        cache: &mut Self::State,
        update: impl FnOnce(&str),
        _: impl FnOnce(),
    ) {
        match cache {
            Some(cache) => {
                if T::not_match_cache(&this, cache) {
                    update(&this);
                    T::update_cache(cache, this);
                }
            }
            None => {
                update(&this);
                *cache = Some(T::create_cache(this));
            }
        }
    }
}

impl MaybeRenderingStr for () {
    type State = ();

    #[inline]
    fn maybe_update_rendering_str_with_cache(
        _: Self,
        _: &mut Self::State,
        _: impl FnOnce(&str),
        _: impl FnOnce(),
    ) {
    }
}

impl<T: RenderingStr> MaybeRenderingStr for Option<T> {
    type State = Option<T::Cache>;

    fn maybe_update_rendering_str_with_cache(
        this: Self,
        cache: &mut Self::State,
        update: impl FnOnce(&str),
        remove: impl FnOnce(),
    ) {
        match this {
            Some(this) => T::maybe_update_rendering_str_with_cache(this, cache, update, remove),
            None => {
                remove();
                *cache = None;
            }
        }
    }
}
