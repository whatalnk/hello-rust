use proconio::input;

fn main() {
    input! {
        a: f32,
        b: f32,
        c: f32,
        x: f32
    }
    if x <= a {
        println!("1");
    } else if x > b {
        println!("0")
    } else {
        println!("{}", c / (b - a));
    }
}
