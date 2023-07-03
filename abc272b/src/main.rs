use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,

    }
    let mut v = vec![HashSet::new(); m];
    for vi in v.iter_mut().take(m) {
        input!(k: usize);
        input!(x: [usize; k]);
        for xj in x.iter().take(k) {
            vi.insert(xj - 1);
        }
    }
    for i in 0..n {
        for j in (i + 1)..n {
            let mut flag = false;
            for vk in v.iter().take(m) {
                if vk.contains(&i) && vk.contains(&j) {
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
