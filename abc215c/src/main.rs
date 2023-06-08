use proconio::input;
use proconio::marker::Chars;
use std::cell::RefCell;
use std::collections::HashSet;

struct MyStruct {
    n: usize,
    s: Vec<char>,
    v: RefCell<Vec<String>>,
}

impl MyStruct {
    fn f(&self, used: i32, ret: String) {
        if used == (1 << self.n) - 1 {
            self.v.borrow_mut().push(ret);
            return;
        }
        for i in 0..self.n {
            if (used & (1 << i)) == 0 {
                self.f(used | (1 << i), ret.clone() + &self.s[i].to_string());
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
    let v = Vec::<String>::new();
    let mut mystruct = MyStruct {
        n,
        s,
        v: RefCell::new(v),
    };
    mystruct.f(0, "".to_string());
    let mut vv: Vec<String> = mystruct
        .v
        .get_mut()
        .clone()
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    vv.sort();
    println!("{}", vv[k - 1]);
}
