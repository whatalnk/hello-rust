use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut al = vec![BTreeSet::<usize>::new(); n + 1];
    for abi in ab.iter().take(m) {
        let (a, b) = abi;
        al[*a].insert(*b);
        al[*b].insert(*a);
    }
    for ali in al.iter().take(n + 1).skip(1) {
        let d = ali.len();
        let v = ali.iter().collect::<Vec<&usize>>();
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
