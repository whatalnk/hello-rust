use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [i64; n]
    }
    a.sort();
    for _ in 0..q {
        input! {
            x: i64
        }
        // If there are multiple matches, then any one of the matches could be returned. The index is chosen deterministically, but is subject to change in future versions of Rust.
        let res = a.binary_search(&x);
        let ans = match res {
            Ok(v) => n - v,
            Err(v) => n - v,
        };
        println!("{}", ans);
    }
}
