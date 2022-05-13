use proconio::input;
use proconio::marker::Chars;
use std::cell::RefCell;
use std::collections::HashSet;

struct MyStruct {
    n: usize,
    s: Vec<char>,
    hs: RefCell<HashSet<String>>,
}

impl MyStruct {
    fn f(&self, used: Vec<usize>) {
        if used.len() == self.n {
            let ret: String = used
                .iter()
                .map(|i| self.s[*i].to_string())
                .collect::<Vec<String>>()
                .join("");
            self.hs.borrow_mut().insert(ret);
            return;
        }
        for i in 0..self.n {
            if !used.contains(&i) {
                let mut used_ = used.clone();
                used_.push(i);
                self.f(used_);
            }
        }
    }
}

fn main() {
    input! {
         s: Chars,
         k: usize
    }
    let n = s.len();
    let hs = HashSet::<String>::new();
    let mut mystruct = MyStruct {
        n: n,
        s: s,
        hs: RefCell::new(hs),
    };
    mystruct.f(vec![]);
    let mut v: Vec<String> = mystruct.hs.get_mut().clone().into_iter().collect();
    v.sort();
    println!("{}", v[k - 1]);
}
