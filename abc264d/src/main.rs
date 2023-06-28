use proconio::input;
use proconio::marker::Chars;

fn find_index(c: char, s: &[char]) -> usize {
    let mut ans = 0;
    for (i, si) in s.iter().enumerate() {
        if &c == si {
            ans = i;
        }
    }
    ans
}

fn main() {
    input! {
        mut s: Chars,
    }
    let t = "atcoder".chars().collect::<Vec<char>>();
    let mut ans = 0;
    for (i, ti) in t.iter().enumerate().take(s.len()) {
        let mut si = find_index(*ti, &s);
        let ti = i;
        while si != ti {
            s.swap(si, si - 1);
            si -= 1;
            ans += 1;
        }
    }
    println!("{}", ans);
}
