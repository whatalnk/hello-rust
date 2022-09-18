use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    let mut m = vec![0i64; 61];
    m[0] = 1;
    for i in 1..=60 {
        m[i] = m[i - 1] * 2;
    }
    let mut nb = vec![];
    for i in 0..=60 {
        if (n & (1i64 << i)) > 0 {
            nb.push(i as usize);
        }
    }
    for b in 0..(1 << nb.len()) {
        let mut v = vec![];
        for i in 0..=60 {
            if (b & (1i64 << i)) > 0 {
                v.push(i as usize);
            }
        }
        let mut ans = 0i64;
        for i in 0..v.len() {
            ans += m[nb[v[i]]];
        }
        println!("{}", ans);
    }
}
