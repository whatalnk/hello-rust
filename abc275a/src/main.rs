use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut ans = 1;
    let mut amax = 0;
    for i in 0..n {
        if a[i] > amax {
            ans = i + 1;
            amax = a[i];
        }
    }
    println!("{}", ans);
}
