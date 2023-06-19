use proconio::input;

fn main() {
    input! {
        h: f32,
    }
    let ans = (h * (12_800_000.0 + h)).sqrt();
    println!("{}", ans);
}
