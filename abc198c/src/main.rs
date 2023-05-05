use proconio::input;

fn main() {
    input! {
        r: i64,
        x: i64,
        y: i64
    }
    let d2 = x * x + y * y;
    let d = (d2 as f64).sqrt();
    if d2 == r * r {
        println!("1");
    } else if d <= (r + r) as f64 {
        println!("2");
    } else {
        println!("{}", (d / r as f64).ceil());
    }
}
