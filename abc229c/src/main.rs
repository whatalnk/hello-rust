use proconio::input;

fn main() {
    input! {
        n: usize,
        mut w: i128,
    }
    let mut ab: Vec<(i128, i128)> = Vec::new();
    for _ in 0..n {
        input! {
            a: i128,
            b: i128
        }
        ab.push((a, b));
    }
    ab.sort_by_key(|k| -k.0);
    let mut ans = 0;
    for (a, b) in ab {
        if w >= b {
            ans += a * b;
            w -= b;
        } else {
            ans += a * w;
            break;
        }
    }
    println!("{}", ans);
}
