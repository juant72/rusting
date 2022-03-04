fn main() {
    let mut rocket_fuel = String::from("RP-1");
    let lenght = process_fuel(&mut rocket_fuel);
    println!(
        "rocket fuel is {} and the lenght is {}",
        rocket_fuel, lenght
    );
}

fn process_fuel(propellant: &mut String) -> usize {
    println!("processing propellant {}", propellant);
    propellant.push_str(" is highly flamable");
    let lenght = propellant.len();
    lenght
}
