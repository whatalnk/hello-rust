use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }
    let mut ans: i64 = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            let ii = (i + 1) as i64;
            let jj = (j + 1) as i64;
            if (ii, jj) == (a[i], a[j]) || (ii, jj) == (a[j], a[i]) {
                println!("{} {}", ii, jj);
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
