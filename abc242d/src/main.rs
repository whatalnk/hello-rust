use proconio::input;
use proconio::marker::Chars;

struct S {
    s: Vec<char>,
}

impl S {
    fn g(&self, s: char, add: i64) -> char {
        let a = b'A';
        let s_ = (s as u8) as i64;
        let offset = ((s_ - (a as i64) + add) % 3) as u8;
        (a + offset) as char
    }

    fn f(&self, t: i64, k: i64) -> char {
        if t == 0 {
            return self.s[k as usize];
        }
        if k == 0 {
            return self.g(self.s[0], t);
        }
        self.g(self.f(t - 1, k / 2), k % 2 + 1)
    }
}

fn main() {
    input! {
        s: Chars,
        q: usize,
        tk: [(i64, i64); q]
    }
    let ss = S { s };
    for tki in tk.iter().take(q) {
        let (t, k) = tki;
        println!("{}", ss.f(*t, k - 1));
    }
}
