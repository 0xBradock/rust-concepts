#[allow(dead_code)]
#[derive(Debug)]
pub struct Vehicle {
    brand: String,
    year: u16,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Person {
    name: String,
    age: u8,
    vehicle: Option<Vehicle>,
}

pub fn without_car() -> Person {
    return Person {
        age: 99,
        name: String::from("Brad"),
        vehicle: None,
    };
}

pub fn with_car() -> Person {
    return Person {
        age: 99,
        name: String::from("Brad"),
        vehicle: Some(Vehicle {
            brand: String::from("bmw"),
            year: 2024,
        }),
    };
}
