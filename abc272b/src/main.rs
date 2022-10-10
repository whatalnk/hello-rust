use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,

    }
    let mut v = vec![HashSet::new(); m];
    for i in 0..m {
        input!(k: usize);
        input!(x: [usize; k]);
        for j in 0..k {
            v[i].insert(x[j] - 1);
        }
    }
    for i in 0..n {
        for j in (i + 1)..n {
            let mut flag = false;
            for k in 0..m {
                if v[k].contains(&i) && v[k].contains(&j) {
                    flag = true;
                    break;
                }
            }
            if !flag {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
