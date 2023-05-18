use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
        q: usize,
        tab: [(i32, usize, usize); q]
    }
    let mut v: Vec<usize> = (0..(2 * n)).collect();
    let mut flipped = false;
    for item in tab.iter() {
        let (t, a, b) = item;
        if t == &1 {
            if flipped {
                let a_: usize;
                let b_: usize;
                if a > &n {
                    a_ = a - n;
                } else {
                    a_ = a + n;
                }
                if b > &n {
                    b_ = b - n;
                } else {
                    b_ = b + n;
                }
                v.swap(a_ - 1, b_ - 1);
            } else {
                v.swap(a - 1, b - 1);
            }
        } else if flipped {
            flipped = false;
        } else {
            flipped = true;
        }
    }
    let left: String = v[0..n].iter().map(|i| s[*i]).collect();
    let right: String = v[n..(2 * n)].iter().map(|i| s[*i]).collect();
    if flipped {
        println!("{}{}", right, left);
    } else {
        println!("{}{}", left, right);
    }
}
