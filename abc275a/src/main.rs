use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut ans = 1;
    let mut amax = 0;
    for (i, ai) in a.iter().enumerate().take(n) {
        if ai > &amax {
            ans = i + 1;
            amax = *ai;
        }
    }
    println!("{}", ans);
}
