use proconio::input;

fn main() {
    input! {
        mut n: usize,
        k: usize,
        a: [usize; k]
    }
    let mut ans = 0;
    while n > 0 {
        let s = a.binary_search(&n);
        match s {
            Ok(v) => {
                n -= a[v];
                ans += a[v];
            }
            Err(e) => {
                if e == 0 {
                    break;
                } else {
                    n -= a[e - 1];
                    ans += a[e - 1];
                }
            }
        }

        let s = a.binary_search(&n);
        match s {
            Ok(v) => {
                n -= a[v];
            }
            Err(e) => {
                if e == 0 {
                    break;
                } else {
                    n -= a[e - 1];
                }
            }
        }
    }
    println!("{}", ans);
}
