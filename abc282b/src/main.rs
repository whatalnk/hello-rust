use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }
    let mut ans = 0;
    for x in 0..n {
        for y in (x + 1)..n {
            let mut ok = true;
            for j in 0..m {
                if s[x][j] == 'x' && s[y][j] == 'x' {
                    ok = false;
                    break;
                }
            }
            if ok {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
