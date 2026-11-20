// Probe only (not for a PR): does each whole-unit constructor overflow at runtime?
use near_token::NearToken;
use std::hint::black_box;
use std::panic::catch_unwind;

fn probe(name: &str, unit: u128, f: fn(u128) -> NearToken) {
    let n = black_box(u128::MAX / unit + 1);
    let mode = if cfg!(debug_assertions) { "debug" } else { "release" };
    match catch_unwind(|| f(n).as_yoctonear()) {
        Ok(v) => println!(
            "PROBE {mode} {name}({n}) = {v} yocto; wrapped={} ; smaller than input*unit is impossible without wrap",
            v == n.wrapping_mul(unit)
        ),
        Err(e) => println!(
            "PROBE {mode} {name}({n}) panicked: {}",
            e.downcast_ref::<String>().cloned().or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string())).unwrap_or_default()
        ),
    }
}

#[test]
fn overflow_probe() {
    probe("from_near", 10u128.pow(24), NearToken::from_near);
    probe("from_millinear", 10u128.pow(21), NearToken::from_millinear);
    probe("from_micronear", 10u128.pow(18), NearToken::from_micronear);
}
