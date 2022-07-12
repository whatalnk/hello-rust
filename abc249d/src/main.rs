use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }
    let n_max = 200000;
    let mut cnt = vec![0; n_max + 1];
    for i in 0..n {
        cnt[a[i] as usize] += 1;
    }
    let mut ans: i64 = 0;
    for i in 1..=n_max {
        let mut j = 1;
        while i * j <= n_max {
            ans += cnt[i] * cnt[j] * cnt[i * j];
            j += 1;
        }
    }
    println!("{}", ans);
}
