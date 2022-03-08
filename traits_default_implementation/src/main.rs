struct Satellite {
    name: String,
    velocity: f64,
}

struct SpaceStation {
    name: String,
    crew_size: u8,
    altitude: u64,
}

trait Description {
    fn describe(&self) -> String {
        String::from("an object flying through space")
    }
}

impl Description for Satellite {}

impl Description for SpaceStation {
    fn describe(&self) -> String {
        format!(
            "{} is a space station with a crew of {} people and an altitude of {} km",
            self.name, self.crew_size, self.altitude
        )
    }
}

fn main() {
    let hubble = Satellite {
        name: "Hubble".to_string(),
        velocity: 4.72,
    };

    let iss = SpaceStation {
        name: "ISS".to_string(),
        crew_size: 6,
        altitude: 254,
    };
    println!("hubble is {}", hubble.describe());
    println!("iss is {}", iss.describe());
}
