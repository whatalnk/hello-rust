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
            if let Some(v) = hmr.get(&rs) {
                let j = v.binary_search(&cs).unwrap_err();
                match j {
                    0 if ce > l => ce = ce - l,
                    0 => ce = 1,
                    _ if j == v.len() => ce = (ce - l).max(v[j - 1] + 1),
                    _ => ce = (ce - l).max(v[j] + 1),
                }
            } else {
                if ce > l {
                    ce = ce - l;
                } else {
                    ce = 1;
                }
            }
        } else if d == 'R' {
            if let Some(v) = hmr.get(&rs) {
                let j = v.binary_search(&cs).unwrap_err();
                match j {
                    0 => ce = (ce + l).min(v[j] - 1),
                    _ if j == v.len() => ce = (ce + l).min(w),
                    _ => ce = (ce + l).min(v[j + 1] - 1),
                }
            } else {
                ce = (ce + l).min(w);
            }
        } else if d == 'U' {
            if let Some(v) = hmc.get(&cs) {
                let j = v.binary_search(&rs).unwrap_err();
                match j {
                    0 if re > l => re = re - l,
                    0 => re = 1,
                    _ if j == v.len() => re = (re - l).max(v[j - 1] + 1),
                    _ => re = (re - l).max(v[j] + 1),
                }
            } else {
                if re > l {
                    re = re - l;
                } else {
                    re = 1;
                }
            }
        } else {
            if let Some(v) = hmc.get(&cs) {
                let j = v.binary_search(&rs).unwrap_err();
                match j {
                    0 => re = (re + l).min(v[j] - 1),
                    _ if j == v.len() => re = (re + l).min(h),
                    _ => re = (re + l).min(v[j + 1] - 1),
                }
            } else {
                re = (re + l).min(h);
            }
        }
        println!("{} {}", re, ce);

        rs = re;
        cs = ce;
    }
}
