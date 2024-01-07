#[allow(unused)]
pub fn will_panic() {
    panic!("panic for no reason")

    // ❯ cargo run -q
    // thread 'main' panicked at src/panic.rs:2:5:
    // panic for no reason
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
}
#[allow(unused)]
pub fn back_trace_panic() {
    back_trace_fn_1()
}
#[allow(unused)]
fn back_trace_fn_1() {
    back_trace_fn_2()
}
#[allow(unused)]
fn back_trace_fn_2() {
    panic!("panic from fn 4")
}
