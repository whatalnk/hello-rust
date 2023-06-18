use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; 4*n - 1]
    }
    let mut b = vec![0; n];
    for i in &a {
        b[*i - 1] += 1;
    }
    for (i, bi) in b.iter().enumerate().take(n) {
        if bi == &3 {
            println!("{}", i + 1);
        }
    }
}
