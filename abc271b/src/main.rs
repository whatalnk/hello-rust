use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut aa = vec![];
    for _ in 0..n {
        input!(l: usize);
        input!(a: [i64; l]);
        aa.push(a);
    }
    for _ in 0..q {
        input!(s: usize, t: usize);
        println!("{}", aa[s - 1][t - 1]);
    }
}
