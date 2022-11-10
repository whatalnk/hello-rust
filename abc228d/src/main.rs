use proconio::input;

fn main() {
    input! {
        q: usize,
        tx: [(i64, usize); q],
    }
    let n = 2_usize.pow(20);
    let mut a: Vec<i64> = vec![-1; n];
    for i in 0..q {
        let (t, x) = tx[i];
        if t == 1 {
            let mut h = x;
            while a[h % n] != -1 {
                h += 1;
            }
            a[h % n] = x as i64;
        } else {
            println!("{}", a[x % n]);
        }
    }
}
