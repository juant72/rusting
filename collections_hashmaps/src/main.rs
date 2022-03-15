use std::collections::HashMap;

fn main() {
    let mut mission_flown = HashMap::new();
    mission_flown.insert("Hardfield", 3);
    mission_flown.insert("Hurley", 3);
    mission_flown.insert("Barron", 0);
    mission_flown.insert("Barron", 1);
    // mission_flown.entry("Barron").or_insert(2);
    mission_flown.entry("Stone").or_insert(2);

    let kayla = mission_flown.entry("Barron").or_insert(0);
    *kayla += 1;
    // println!("kayla mission_flown is {:?}", kayla);
    println!("mission_flown is {:?}", mission_flown);
    let barron_mission = mission_flown.get("Barron");
    println!("barron_mission is {:?}", barron_mission);
    let hardfield_mission = mission_flown.get("Hardfield");
    println!("hardfield_mission is {:?}", hardfield_mission);
}
