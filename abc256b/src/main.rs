use proconio::input;

fn main() {
    input! {
       n: usize,
       a: [i64; n]
    }
    let mut p = 0;
    for i in 0..n {
        let mut cur = 0;
        for j in i..n {
            cur += a[j];
            if cur >= 4 {
                p += 1;
                break;
            }
        }
    }
    println!("{}", p);
}
