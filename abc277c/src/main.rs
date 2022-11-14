use proconio::input;
use std::collections::{HashMap, HashSet};

struct Ladder {
    hm: HashMap<usize, Vec<usize>>,
    ans: usize,
}
impl Ladder {
    fn new(ab: &Vec<(usize, usize)>) -> Ladder {
        let mut hm = HashMap::<usize, Vec<usize>>::new();
        for i in 0..ab.len() {
            let (a, b) = ab[i];
            let e = hm.entry(a).or_insert(Vec::<usize>::new());
            e.push(b);
            let e = hm.entry(b).or_insert(Vec::<usize>::new());
            e.push(a);
        }
        Ladder { hm: hm, ans: 1 }
    }
    fn dfs(&mut self, nx: usize, mx: usize, visited: HashSet<usize>) {
        if let Some(e) = self.hm.get(&nx).cloned() {
            for i in 0..e.len() {
                if !visited.contains(&e[i]) {
                    let mut visited_ = visited.clone();
                    visited_.insert(e[i]);
                    self.dfs(e[i], mx.max(e[i]), visited_)
                } else {
                    self.ans = self.ans.max(mx);
                }
            }
        }
    }
}

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }
    let mut l = Ladder::new(&ab);
    let mut hs = HashSet::new();
    hs.insert(1);
    l.dfs(1, 1, hs);
    println!("{}", l.ans);
}
