use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        s: Chars,
        q: usize,
        tk: [(usize, usize); q]
    }
    let mut hm = HashMap::new();
    hm.insert('A', ['B', 'C']);
    hm.insert('B', ['C', 'A']);
    hm.insert('C', ['A', 'B']);
    for i in 0..q {
        let (t, mut k) = tk[i];
        k -= 1;
        let mut cnt = 0;
        let mut lr = vec![];
        while cnt < t {
            if k % 2 == 0 {
                lr.push(0);
            } else {
                lr.push(1);
            }
            k /= 2;
            cnt += 1;
            if k == 0 {
                break;
            }
        }
        lr.reverse();
        let mut ans = s[k];
        for j in 0..cnt {
            ans = hm.get(&s[k]).unwrap()[lr[j]];
        }

        println!("{}", ans);
    }
}
