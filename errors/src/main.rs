use question::question_result;

mod option;
mod panic;
mod question;

fn main() {
    // Option
    let w_car = option::with_car();
    println!("{:?}", w_car);

    let wo_car = option::without_car();
    println!("{:?}", wo_car);

    // Question mark ?
    match question::question_option("some string", 'a') {
        Some(x) => println!("The position is: {}", x),
        None => println!("Not found"),
    }

    match question_result() {
        Ok(v) => println!("Value is: {}", v),
        Err(e) => println!("Returned error: {}", e),
    }

    // Panic
    panic::will_panic();
    panic::back_trace_panic();
}
