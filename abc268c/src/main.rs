use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [i64; n],
    }
    let mut cnt = vec![0; n];
    for (i, pi) in p.iter().enumerate().take(n) {
        for j in 0..3 {
            let ii = (pi - 1 - (i as i64) + (j as i64) + (n as i64)) % (n as i64);
            cnt[ii as usize] += 1;
        }
    }
    let mut ans: i64 = 0;
    for cnti in cnt.iter().take(n) {
        ans = ans.max(*cnti);
    }
    println!("{}", ans);
}
