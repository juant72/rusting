#[derive(PartialEq, PartialOrd)]
struct Satellite {
    name: String,
    velocity: f64,
}

fn main() {
    let hubble = Satellite {
        name: "Hubble Telescope.".to_string(),
        velocity: 4.72,
    };

    let gps = Satellite {
        name: "GPS".to_string(),
        velocity: 2.42,
    };

    println!("hubble == gps {}", hubble == gps);
    println!("hubble > gps {}", hubble > gps);
}
