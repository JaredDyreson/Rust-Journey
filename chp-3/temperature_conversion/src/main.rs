/*
 * Conversion functions from F -> C and C -> F
*/
fn to_celsius(temp: i32) -> f32 {
    return temp as f32 * (9. / 5.);
}

fn to_fahrenheit(temp: i32) -> f32 {
    return (temp as f32 - 32.) * (9. / 5.);
}

fn main() {
    println!("Hello, world!");
}
