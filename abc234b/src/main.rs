use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(f32, f32); n]
    }
    let mut ans = 0.0;
    for (x1, y1) in &xy {
        for (x2, y2) in &xy {
            let d = ((x1 - x2).powf(2.0) + (y1 - y2).powf(2.0)).sqrt();
            ans = d.max(ans);
        }
    }
    println!("{}", ans);
}
