#[inline]
pub fn map_or_insert_with_ctx<T, C, R>(
    this: &mut Option<T>,
    ctx: C,
    map: impl FnOnce(&mut T, C) -> R,
    insert: impl FnOnce(C) -> T,
) -> (Option<R>, &mut T) {
    match this {
        Some(this) => (Some(map(this, ctx)), this),
        None => {
            let this = this.insert(insert(ctx));
            (None, this)
        }
    }
}
