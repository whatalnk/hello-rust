use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: i64,
        a: [i64; n],
    }
    let mut hm = HashMap::new();
    let mut asum = 0;
    for i in 0..n {
        let e = hm.entry(a[i]).or_insert(0);
        *e += 1;
        asum += a[i];
    }
    let mut v = Vec::<(i64, i64)>::new();
    for (key, val) in hm.iter() {
        v.push((*key, *val));
    }
    let k = v.len();
    if k == m as usize {
        println!("0");
        return;
    }
    let mut p = 0;
    for i in 0..k {
        if v[(i + 1) % (k as usize)].0 != (v[i].0 + 1) % m {
            p = i as i64;
            break;
        }
    }
    let mut s = vec![0; 200005];
    for i in 0..k {
        let j = (p as usize - i + k) % k;
        s[j] = asum;
        if v[(j + 1) % k].0 == (v[j].0 + 1) % m {
            s[j] = s[(j + 1) % k];
        }
        s[j] -= v[j].0 * v[j].1;
    }

    let mut ans = asum;
    for i in 0..k {
        ans = ans.min(s[i]);
    }
    println!("{}", ans);
}
