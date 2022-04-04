use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64
    }
    let d = (a * a + b * b).sqrt();
    println!("{} {}", 1.0 / d * a, 1.0 / d * b);
}
