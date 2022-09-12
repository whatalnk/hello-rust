use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [i64; n],
    }
    let mut cnt = vec![0; n];
    for i in 0..n {
        for j in 0..3 {
            let ii = (p[i] - 1 - (i as i64) + (j as i64) + (n as i64)) % (n as i64);
            cnt[ii as usize] += 1;
        }
    }
    let mut ans: i64 = 0;
    for i in 0..n {
        ans = ans.max(cnt[i]);
    }
    println!("{}", ans);
}
