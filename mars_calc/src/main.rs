use std::io;

fn main() {
    println!("Enter your weight:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let weight = input.trim().parse::<f32>().unwrap();
    let mut mars_weight: f32 = calc_weight(weight);
    mars_weight = mars_weight * 1000.0;
    println!("Weight on Mars: {} g", mars_weight);

}


fn calc_weight(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}