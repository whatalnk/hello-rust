use proconio::input;

fn main() {
    input! {
        (ax, ay): (f64, f64),
        (bx, by): (f64, f64),
        (cx, cy): (f64, f64),
        (dx, dy): (f64, f64),
    }
    println!("{} {}", ax, ay);
    println!("{} {}", bx, by);
    println!("{} {}", cx, cy);
    println!("{} {}", dx, dy);
}
