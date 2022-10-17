use proconio::input;
use std::collections::BTreeMap;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut bts = BTreeMap::new();
    for i in 0..n {
        let e = bts.entry(a[i]).or_insert(0);
        *e += 1;
    }
    let mut hm = HashMap::new();
    let nu = bts.len();
    let mut i = 1;
    for (k, _) in bts.iter() {
        hm.insert(nu - i, k);
        i += 1;
    }
    for k in 0..n {
        if let Some(e) = hm.get(&k) {
            println!("{}", bts.get(&e).unwrap());
        } else {
            println!("0");
        }
    }
}
