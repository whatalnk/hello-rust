use proconio::input;
use proconio::marker::Chars;

fn find_index(c: char, s: &Vec<char>) -> usize {
    let mut ans = 0;
    for i in 0..s.len() {
        if c == s[i] {
            ans = i;
        }
    }
    return ans;
}

fn main() {
    input! {
        mut s: Chars,
    }
    let t = "atcoder".chars().collect::<Vec<char>>();
    let mut ans = 0;
    for i in 0..s.len() {
        let mut si = find_index(t[i], &s);
        let ti = i;
        while si != ti {
            s.swap(si, si - 1);
            si -= 1;
            ans += 1;
        }
    }
    println!("{}", ans);
}
