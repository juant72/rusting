fn main() {
    let numbers = [1,9,-2,0, 23,20,-7,13,37,20,56,-18,20,3];
    let mut max:i32 = numbers[0];
    let mut min:i32 = numbers[0];
    let mut mean:f64 = 0.0 as f64;

    for i in 0..numbers.len() {
        if numbers[i] > max {
            max = numbers[i];
        }
        if numbers[i] < min {
            min = numbers[i];
        }
        mean += numbers[i] as f64;
    }
    mean/=numbers.len() as f64;


    assert_eq!(max,56 as i32);
    assert_eq!(min, -18 as i32);
    assert_eq!(mean, 12.5 as f64);
    println!("Test passed!");

}
