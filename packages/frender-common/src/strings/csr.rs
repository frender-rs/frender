use crate::{IntoStaticStrCache, ToAsRefStr};

use super::CsrStr;

// TODO: remove
pub fn update_into_option_cache<S: CsrStr, R>(
    s: S,
    cache: &mut Option<S::StaticStrCache>,
    update: impl FnOnce(&str) -> R,
) -> Option<R> {
    let s = s.into_into_static_str_cache();
    let cache = if let Some(cache) = cache {
        if *cache == s {
            return None;
        }

        s.update_into_static_str_cache(cache);

        cache
    } else {
        cache.insert(s.into_static_str_cache())
    };

    Some(update(cache.to_as_ref_str().as_ref()))
}

pub fn init_cache<S: CsrStr, R>(
    //
    s: S,
    update: impl FnOnce(&str) -> R,
) -> (S::StaticStrCache, R) {
    let cache = s.into_into_static_str_cache().into_static_str_cache();

    let res = update(cache.to_as_ref_str().as_ref());
    (cache, res)
}

pub fn update_into_cache<S: CsrStr, R>(
    //
    s: S,
    update: impl FnOnce(&str) -> R,
    cache: &mut S::StaticStrCache,
) -> Option<R> {
    let s = s.into_into_static_str_cache();

    if *cache == s {
        return None;
    }

    s.update_into_static_str_cache(cache);

    Some(update(cache.to_as_ref_str().as_ref()))
}
