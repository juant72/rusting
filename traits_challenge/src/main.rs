use std::fmt;

struct Satellite {
    name: String,
    velocity: f64,
}

impl fmt::Display for Satellite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} Satellite flying at {}", self.name, self.velocity)
    }
}

fn main() {
    let hubble = Satellite {
        name: "Hubble".to_string(),
        velocity: 4.72,
    };
    println!("hubble {}", hubble);
}
