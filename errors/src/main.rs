mod option;
mod panic;

fn main() {
    // panic::will_panic();
    // panic::back_trace_panic();

    let w_car = option::with_car();
    println!("{:?}", w_car);

    let wo_car = option::without_car();
    println!("{:?}", wo_car);
}
