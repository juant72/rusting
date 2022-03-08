#[derive(Debug, Clone)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}
fn main() {
    let mut vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 835958.0,
    };
    println!("Name: {}", vehicle.name);

    let vehicle2 = Shuttle {
        //        name: String::from("Voyager"),
        ..vehicle.clone()
    };

    vehicle.name = String::from("Atlantis");
    vehicle.crew_size = 6;
    println!("vehicle: {:?}", vehicle);
    println!("vehicle2: {:?}", vehicle2);
}
