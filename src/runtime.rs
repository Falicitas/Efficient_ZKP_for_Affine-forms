use std::time::Instant;
pub fn print_runtime(now: &mut std::time::Instant, indent: &'static str, mesg: &'static str) {
    let duration = now.elapsed();
    let (s, mut ms, mut us) = (
        duration.as_secs(),
        duration.as_millis(),
        duration.as_micros(),
    );
    us -= ms * 1000;
    ms -= s as u128 * 1000;
    println!(
        "{}Hi! {} running time: {} s {} ms {} us",
        indent, mesg, s, ms, us
    );
    *now = Instant::now();
}
