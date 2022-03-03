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
    for i in 0..n {
        if b[i] == 3 {
            println!("{}", i + 1);
        }
    }
}
