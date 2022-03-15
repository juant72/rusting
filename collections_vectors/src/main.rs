fn main() {
    let mut astronauts: Vec<String> = Vec::new();
    astronauts.push("Buzz Aldrin".to_string());
    astronauts.push("Alan Bean".to_string());
    astronauts.push("Alan Shepard".to_string());
    astronauts.push("Edgar Mitchell".to_string());
    astronauts.push("David Scott".to_string());
    astronauts.push("James Lovell".to_string());
    astronauts.push("John Young".to_string());
    astronauts.push("Robert Young".to_string());
    astronauts.push("Charles Duke".to_string());
    astronauts.push("Eugene Cernan".to_string());
    astronauts.push("J. Neil Armstrong".to_string());
    astronauts.push("Pete Conrad".to_string());
    astronauts.push("Alan Bean".to_string());
    println!("astronauts us {:?}", astronauts);

    let last = astronauts.pop();
    println!("last astronaut is {:?}", last);

    // let third = &astronauts[2];
    // println!("third is {:?}", third);

    let third = &astronauts.get(2);
    println!("third is {:?}", third);

    let countdown = vec![5, 4, 3, 2, 1, 0];
    println!("countdown is {:?}", countdown);
}
