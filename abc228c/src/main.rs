use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize
    }
    let mut a = vec![];
    let mut b = vec![];
    for _ in 0..n {
        input! {
            a1: i64,
            a2: i64,
            a3: i64
        }
        let s = a1 + a2 + a3;
        a.push(s);
        b.push(s);
    }
    a.sort();
    for i in b {
        let s = i + 300;
        let res = a.binary_search(&s);
        let ans = match res {
            Ok(v) => v + 1,
            Err(v) => v,
        };
        let rank = n - ans + 1;
        if rank <= k {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
