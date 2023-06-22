use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        k: i32,
        s: [Chars; n]
    }
    let mut ans = 0;
    let a = b'a';
    for i in 0..(1 << n) {
        let mut sum = vec![0; 26];
        for j in 0..n {
            if (i >> j) & 1 > 0 {
                for x in 0..(s[j].len()) {
                    sum[(s[j][x] as u8 - a) as usize] += 1;
                }
            }
        }
        let mut now = 0;
        for sumj in sum.iter().take(26) {
            if sumj == &k {
                now += 1;
            }
        }
        ans = ans.max(now);
    }
    println!("{}", ans);
}
