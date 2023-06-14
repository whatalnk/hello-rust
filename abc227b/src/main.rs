use proconio::input;

fn main() {
    input! {
       n: usize,
       s: [usize; n]
    }
    let mut v = vec![0; 10_000_000];
    for i in 1..=250 {
        for j in 1..=250 {
            let ss = 4 * i * j + 3 * i + 3 * j;
            v[ss] += 1;
        }
    }
    let mut ans = 0;
    for i in s {
        if v[i] == 0 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
