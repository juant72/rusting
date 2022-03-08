use std::mem;

struct Shuttle {
    name: String,
    crew_size: u8,
    passengers: f64,
}

fn main() {
    let vehicle = Shuttle {
        name: String::from("Atlantis"),
        crew_size: 7,
        passengers: 835958.0,
    };
    println!("vehicle size on stack {}", mem::size_of_val(&vehicle));

    let boxed_vehicle: Box<Shuttle> = Box::new(vehicle);
    println!("vehicle size on stack {}", mem::size_of_val(&boxed_vehicle));
    println!("vehicle size on heap {}", mem::size_of_val(&*boxed_vehicle));

    let unboxed_vehicle = *boxed_vehicle;
    println!(
        "unboxed vehicle size on stack {}",
        mem::size_of_val(&unboxed_vehicle)
    );
}
