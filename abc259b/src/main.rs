use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        mut d: f64
    }
    d = d.to_radians();
    let x = d.cos() * a - d.sin() * b;
    let y = d.sin() * a + d.cos() * b;
    println!("{} {}", x, y);
}
