use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize
    }
    let mut idx = vec![vec![]; n + 1];
    for i in 0..n {
        idx[a[i]].push(i);
    }
    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
            x: usize
        }
        let right = match idx[x].binary_search(&r) {
            Ok(v) => v,
            Err(v) => v,
        };
        let left = match idx[x].binary_search(&(l - 1)) {
            Ok(v) => v,
            Err(v) => v,
        };
        println!("{}", right - left);
    }
}
