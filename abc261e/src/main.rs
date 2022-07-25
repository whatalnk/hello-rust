use proconio::input;

fn bit(x: i64, i: i64) -> i64 {
    return (x >> i) & 1;
}

fn main() {
    input! {
        n: usize,
        mut c: i64,
        ta: [(i64, i64); n]
    }
    let mut ans = vec![0; n];
    for k in 0..30 {
        let mut func: Vec<i64> = vec![0, 1];
        let mut crr = bit(c, k);
        for i in 0..n {
            let mut f = Vec::<i64>::new();
            let x = bit(ta[i].1, k);
            if ta[i].0 == 1 {
                f = vec![0 & x, 1 & x];
            } else if ta[i].0 == 2 {
                f = vec![0 | x, 1 | x];
            } else {
                f = vec![0 ^ x, 1 ^ x];
            }
            func = vec![f[func[0] as usize], f[func[1] as usize]];
            crr = func[crr as usize];
            ans[i] |= crr << k;
        }
    }
    for i in 0..n {
        println!("{}", ans[i]);
    }
}
