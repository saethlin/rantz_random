std::thread_local! {
    static RNG: () = ();
}

#[inline]
fn with_rng(f: impl FnOnce()) {
    RNG.with(|_| {
        f()
    })
}

#[inline]
pub fn seed() {
    with_rng(|| {});
}

#[inline]
pub fn bool() {
    with_rng(|| {})
}
