use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [i64; n],
        mut b: [i64; m]
    }
    a.sort();
    b.sort();
    for i in 0..n {
        for j in 0..m {
            println!("{} {} {}", (a[i] - b[j]).abs(), a[i], b[j]);
        }
    }
}
