use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        st: [(String, String); n]
    }
    let mut h = HashMap::<String, HashMap<String, i32>>::new();
    for (s, t) in st {
        let v = h.entry(s).or_insert_with(HashMap::new);
        let vv = v.entry(t).or_insert(0);
        *vv += 1;
        if *vv == 2 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
