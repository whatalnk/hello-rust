use proconio::input;

fn main() {
    input! {
        r: f64,
        x: f64,
        y: f64
    }
    let d = (x * x + y * y).sqrt();
    println!("{}", (d / r).ceil());
}
