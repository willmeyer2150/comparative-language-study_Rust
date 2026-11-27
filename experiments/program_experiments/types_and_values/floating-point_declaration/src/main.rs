fn main() {
    let x = 2.0; // f64 (default)
    let y: f32 = 3.0; // f32 (user specified)
    println!("{:.1}", x + y as f64);
}