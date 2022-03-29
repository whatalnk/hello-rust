use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
       n: usize
    }
    let mut hm = HashMap::new();
    let mut g = Vec::<Vec<i32>>::new();
    for i in 0..n {
        input! {
            l: usize,
            a: [i32; l]
        }
        g.push(a);
        let v = hm.entry(l).or_insert(Vec::<usize>::new());
        v.push(i);
    }
    for i in 0..n {
        println!("{} {:?}", i, g[i]);
    }
    for (k, v) in &hm {
        println!("{} {:?}", k, v);
    }
}
