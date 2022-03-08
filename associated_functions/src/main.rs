struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn add_fuel(&mut self, fuel: f64) {
        self.propellant += fuel;
    }

    fn new(name: &str) -> Shuttle {
        Shuttle {
            name: String::from(name),
            crew_size: 7,
            propellant: 0.0,
        }
    }
}

fn main() {
    let mut vehicle = Shuttle::new("Endeavour");

    let vehicle_name = vehicle.get_name();
    println!("Name: {}", vehicle_name);

    println!("Propellant: {}", vehicle.propellant);
    vehicle.add_fuel(1000.0);
    println!("Propellant: {}", vehicle.propellant);
}
