const RNG: fn() = || {};

fn with_rng() {
    let _x = &RNG;
}

pub fn seed() {
    with_rng();
}

pub fn bool() {
    with_rng();
}
