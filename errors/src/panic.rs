pub fn will_panic() {
    panic!("panic for no reason")

    // ❯ cargo run -q
    // thread 'main' panicked at src/panic.rs:2:5:
    // panic for no reason
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
}
