use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut al = vec![BTreeSet::<usize>::new(); n + 1];
    for i in 0..m {
        let (a, b) = ab[i];
        al[a].insert(b);
        al[b].insert(a);
    }
    for i in 1..=n {
        let d = al[i].len();
        let v = al[i].iter().collect::<Vec<&usize>>();
        println!(
            "{} {}",
            d,
            v.iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}
