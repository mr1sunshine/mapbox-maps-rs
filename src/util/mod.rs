use num::traits::Float;
use std::sync::atomic::{AtomicUsize, Ordering};

pub(crate) fn wrap<T: Float>(n: T, min: T, max: T) -> T {
    let d = max - min;
    let w = ((n - min) % d + d) % d + min;
    if (w - min).abs() < T::from(std::f64::EPSILON).unwrap() {
        max
    } else {
        w
    }
}

pub(crate) fn unique_id() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}
