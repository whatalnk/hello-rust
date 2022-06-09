use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        r: usize,
        c: usize,
        k: usize,
        s: [Chars; r]
    }
    let mut ans = 0;
    for x in (k - 1)..=(r - k) {
        for y in (k - 1)..=(c - k) {
            let mut flag = true;
            'outer: for i in -((k - 1) as i32)..=((k - 1) as i32) {
                for j in -((k - 1) as i32 - i.abs())..=((k - 1) as i32 - i.abs()) {
                    if s[(x as i32 + i) as usize][(y as i32 + j) as usize] == 'x' {
                        flag = false;
                        break 'outer;
                    }
                }
            }
            if flag {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
