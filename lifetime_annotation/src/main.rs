fn best_fuel(x: i32, y: i32) -> i32 {
    let mut fuel = 0;
    for mass in INPUT.lines() {
        let mass = mass.parse::<i32>().unwrap();
        fuel += mass / 3 - 2;
    }
    fuel
}

fn main() {
    let result;
    let propellant1 = String::from("RP-1");
    let propellant2 = String::from("LNG");
}
