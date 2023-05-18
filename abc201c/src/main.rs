use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

fn main() {
    input! {
        d: Chars,
    }
    let mut n_must = HashSet::new();
    let mut n_all = vec![];
    for (i, item) in d.iter().enumerate().take(10) {
        if item == &'o' {
            n_must.insert(i);
            n_all.push(i);
        } else if item == &'?' {
            n_all.push(i);
        }
    }
    if n_must.len() > 4 {
        println!("0");
        return;
    }
    let n = n_all.len();
    let mut ans = HashSet::new();
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                for l in 0..n {
                    let sn: HashSet<usize> = [n_all[i], n_all[j], n_all[k], n_all[l]]
                        .iter()
                        .cloned()
                        .collect();
                    if sn.is_superset(&n_must) {
                        let s = format!("{}{}{}{}", n_all[i], n_all[j], n_all[k], n_all[l]);
                        ans.insert(s);
                    }
                }
            }
        }
    }
    println!("{}", ans.len());
}
