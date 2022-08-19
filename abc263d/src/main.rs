use proconio::input;

fn main() {
    input! {
        n: usize,
        l: i64,
        r: i64,
        a: [i64; n],
    }
    let mut f = vec![0i64; n + 1];
    let mut g = vec![0i64; n + 1];
    for i in 1..=n {
        f[i] = (f[i - 1] + a[i - 1]).min(l * (i as i64));
    }
    for i in 1..=n {
        g[n - i] = (g[n - i + 1] + a[n - i]).min(r * (i as i64));
    }
    let mut ans = f[0] + g[0];
    for i in 0..=n {
        ans = ans.min(f[i] + g[i]);
    }
    println!("{}", ans);
}
