struct LocalKey {
    inner: fn(),
}

const RNG: LocalKey = LocalKey { inner: || {} };

fn with(key: &LocalKey, f: impl FnOnce()) {}

fn with_rng() {
    with(&RNG, || {})
}

pub fn seed() {
    with_rng();
}

pub fn bool() {
    with_rng()
}
