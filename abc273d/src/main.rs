use proconio::input;
use std::collections::{BinaryHeap, HashMap};

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
    let mut hmc_ = HashMap::new();
    let mut hmr_ = HashMap::new();
    for i in 0..n {
        let (r, c) = rc[i];
        let e = hmc_.entry(c).or_insert(BinaryHeap::new());
        e.push(r);
        let e = hmr_.entry(r).or_insert(BinaryHeap::new());
        e.push(c);
    }
    let mut hmc = HashMap::new();
    let mut hmr = HashMap::new();
    for (k, v) in &hmc_ {
        hmc.insert(k, v.clone().into_sorted_vec());
    }
    for (k, v) in &hmr_ {
        hmr.insert(k, v.clone().into_sorted_vec());
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
                let x1 = match v.binary_search(&cs) {
                    Ok(vv) => vv,
                    Err(vv) => vv,
                };
                let x2 = match v.binary_search(&ce) {
                    Ok(vv) => vv,
                    Err(vv) => vv,
                };
                if x1 != x2 {
                    ce = ce.max(v[x1.min(v.len() - 1)] + 1);
                }
            }
        } else if d == 'R' {
            if cs + l <= w {
                ce += l;
            } else {
                ce = w;
            }
            if let Some(v) = hmr.get(&rs) {
                let x1 = match v.binary_search(&cs) {
                    Ok(vv) => vv,
                    Err(vv) => vv,
                };
                let x2 = match v.binary_search(&ce) {
                    Ok(vv) => vv,
                    Err(vv) => vv,
                };
                if x1 != x2 {
                    ce = ce.min(v[x1.min(v.len() - 1)] - 1);
                }
            }
        } else if d == 'U' {
            if rs > l {
                re -= l;
            } else {
                re = 1;
            }
            if let Some(v) = hmc.get(&cs) {
                let x1 = match v.binary_search(&rs) {
                    Ok(vv) => vv,
                    Err(vv) => vv,
                };
                let x2 = match v.binary_search(&re) {
                    Ok(vv) => vv,
                    Err(vv) => vv,
                };
                if x1 != x2 {
                    re = re.max(v[x1.min(v.len() - 1)] + 1);
                }
            }
        } else {
            if rs + l <= h {
                re += l;
            } else {
                re = h;
            }
            if let Some(v) = hmc.get(&cs) {
                let x1 = match v.binary_search(&rs) {
                    Ok(vv) => vv,
                    Err(vv) => vv,
                };
                let x2 = match v.binary_search(&re) {
                    Ok(vv) => vv,
                    Err(vv) => vv,
                };
                if x1 != x2 {
                    re = re.min(v[x1.min(v.len() - 1)] - 1);
                }
            }
        }
        println!("{} {}", re, ce);
        rs = re;
        cs = ce;
    }
}
