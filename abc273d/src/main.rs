use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut rs: usize,
        mut cs: usize,
        n: usize,
        rc: [(usize, usize); n],
        q: usize,
        dl: [(char, usize); q],
    }
    let mut hmc = HashMap::new();
    let mut hmr = HashMap::new();
    for i in 0..n {
        let (r, c) = rc[i];
        let e = hmc.entry(c).or_insert(vec![]);
        e.push(r);
        let e = hmr.entry(r).or_insert(vec![]);
        e.push(c);
    }
    for i in 0..q {
        let (d, l) = dl[i];
        let mut re = rs;
        let mut ce = cs;
        if d == 'L' {
            if cs > l {
                ce -= l;
            } else {
                ce = 1;
            }
            if let Some(v) = hmr.get(&rs) {
                let x1 = v.binary_search(&cs).unwrap_();
                let x2 = v.binary_search(&ce).unwrap();
                if x1 != x2 {
                    ce = ce.max(v[x1] + 1);
                }
            }
        } else if d == 'R' {
            if cs + l <= w {
                ce += l;
            } else {
                ce = w;
            }
            if let Some(v) = hmr.get(&rs) {
                let x1 = v.binary_search(&cs).unwrap();
                let x2 = v.binary_search(&ce).unwrap();
                if x1 != x2 {
                    ce = ce.min(v[x1] - 1);
                }
            }
        } else if d == 'U' {
            if rs > l {
                re -= l;
            } else {
                re = 1;
            }
            if let Some(v) = hmc.get(&cs) {
                let x1 = v.binary_search(&rs).unwrap();
                let x2 = v.binary_search(&re).unwrap();
                if x1 != x2 {
                    re = re.max(v[x1] + 1);
                }
            }
        } else {
            if rs + l <= h {
                re += l;
            } else {
                re = h;
            }
            if let Some(v) = hmc.get(&cs) {
                let x1 = v.binary_search(&rs).unwrap();
                let x2 = v.binary_search(&re).unwrap();
                if x1 != x2 {
                    re = re.min(v[x1] - 1);
                }
            }
        }
        println!("{:?} -> {:?}", (rs, cs), (re, ce));
        rs = re;
        cs = ce;
    }
}
