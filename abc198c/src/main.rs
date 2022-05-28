use proconio::input;

fn main() {
    input! {
        r: f64,
        x: f64,
        y: f64
    }
    let d = (x * x + y * y).sqrt();
    if d == r {
        println!("1");
    } else if d <= r + r {
        println!("2");
    } else {
        println!("{}", (d / r).ceil());
    }
}
