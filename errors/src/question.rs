use std::num::ParseIntError;

// Option
pub fn question_option(s: &str, l: char) -> Option<usize> {
    let pos = s.find(l)?;

    Some(pos)
}

pub fn question_result() -> Result<u32, ParseIntError> {
    let val = "j".parse::<u32>()?;

    Ok(val)
}
// // Result
// pub fn inner_result() -> Result<(), String> {
//     Err(String::from("Error"))
// }

// pub fn result() -> Result<(), String> {
//     inner_result()?
// }
