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
    let mut checked = vec![false; n];
    let mut ans = 0;
    for (_, v) in &hm {
        if v.len() == 1 {
            ans += 1;
            continue;
        }
        for i in 0..v.len() {
            if checked[i] {
                continue;
            } else {
                checked[i] = true;
                ans += 1;
            }
            for j in (i + 1)..v.len() {
                if checked[j] {
                    continue;
                } else {
                    if g[v[i]] == g[v[j]] {
                        checked[j] = true;
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
