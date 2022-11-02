use proconio::input;
use std::collections::{BTreeSet, HashMap};

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
    let mut hmc_ = HashMap::<usize, BTreeSet<usize>>::new();
    let mut hmr_ = HashMap::<usize, BTreeSet<usize>>::new();
    for i in 0..n {
        let (r, c) = rc[i];
        let e = hmc_.entry(c).or_insert(BTreeSet::new());
        if e.len() == 0 {
            e.insert(0);
            e.insert(h + 1);
        }
        e.insert(r);
        let e = hmr_.entry(r).or_insert(BTreeSet::new());
        if e.len() == 0 {
            e.insert(0);
            e.insert(w + 1);
        }
        e.insert(c);
    }

    let mut hmc = HashMap::<&usize, Vec<&usize>>::new();
    let mut hmr = HashMap::<&usize, Vec<&usize>>::new();
    for (k, v) in &hmc_ {
        hmc.insert(k, v.iter().collect());
    }

    for (k, v) in &hmr_ {
        hmr.insert(k, v.iter().collect());
    }
    for i in 0..q {
        let (d, l) = dl[i];
        let mut re = rs;
        let mut ce = cs;
        if d == 'L' {
            if let Some(v) = hmr.get(&rs) {
                let j = v.binary_search(&&cs).unwrap_err();
                ce = (ce as isize - l as isize).max(1) as usize;
                ce = ce.max(v[j - 1] + 1);
            } else {
                ce = (ce as isize - l as isize).max(1) as usize;
            }
        } else if d == 'R' {
            if let Some(v) = hmr.get(&rs) {
                let j = v.binary_search(&&cs).unwrap_err();
                ce = (ce + l).min(w);
                ce = ce.min(v[j] - 1);
            } else {
                ce = (ce as isize + l as isize).min(w as isize) as usize;
            }
        } else if d == 'U' {
            if let Some(v) = hmc.get(&cs) {
                let j = v.binary_search(&&rs).unwrap_err();
                re = (re as isize - l as isize).max(1) as usize;
                re = re.max(v[j - 1] + 1);
            } else {
                re = (re as isize - l as isize).max(1) as usize;
            }
        } else {
            if let Some(v) = hmc.get(&cs) {
                let j = v.binary_search(&&rs).unwrap_err();
                re = (re + l).min(h);
                re = re.min(v[j] - 1);
            } else {
                re = (re as isize + l as isize).min(h as isize) as usize;
            }
        }
        println!("{} {}", re, ce);

        rs = re;
        cs = ce;
    }
}
