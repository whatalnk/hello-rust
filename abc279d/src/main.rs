use proconio::input;

fn main() {
    input! {
        a: f64,
        b: i64
    }
    for i in 0..=10 {
        let g = i as f64 + 1.0;
        println!("{}", (b * i) as f64 + a / g.sqrt());
    }
}
