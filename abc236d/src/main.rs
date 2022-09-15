use itertools::Itertools;
use proconio::input;

fn main() {
    input!(n: usize);
    println!("{}", n);
    let mut a = vec![vec![0; 2 * n - 1]; 2 * n - 1];
    for i in 1..=(2 * n - 1) {
        input!(aa: [i64; 2 * n - i]);
        for j in 0..(2 * n - i) {
            a[i - 1][j] = aa[j];
        }
    }
}
