fn main() {
    let outer_planet ;
    {
        let mut inner_planet= String::from("Mercury");
        println!("Inner planet is {}",inner_planet);
        outer_planet=inner_planet.clone();
        inner_planet.clear();
        println!("Inner planet is {}",inner_planet);
    }
    println!("Outer planet is {}",outer_planet);
}