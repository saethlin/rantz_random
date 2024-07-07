struct LocalKey {
    inner: fn(),
}

impl LocalKey {
    fn with(&self, f: impl FnOnce()) {}
}

const RNG: LocalKey = {
    LocalKey {
        inner: || {},
    }
};

fn with_rng(f: impl FnOnce()) {
    RNG.with(|| f())
}

#[inline]
pub fn seed() {
    with_rng(|| {});
}

#[inline]
pub fn bool() {
    with_rng(|| {})
}
