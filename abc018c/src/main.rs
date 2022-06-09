use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        r: usize,
        c: usize,
        k: usize,
        s: [Chars; r]
    }
    let mut u = vec![];
    let mut l = vec![];
    for i in 0..r {
        let mut ru = vec![];
        let mut rl = vec![];
        for j in 0..c {
            let mut cnt = 0;
            for di in 0..=(i as i32) {
                if s[(i as i32 - di) as usize][j] == 'o' {
                    cnt += 1;
                } else {
                    break;
                }
            }
            ru.push(cnt);
            let mut cnt = 0;
            for di in 0..((r - i) as i32) {
                if s[(i as i32 + di) as usize][j] == 'o' {
                    cnt += 1;
                } else {
                    break;
                }
            }
            rl.push(cnt);
        }
        u.push(ru);
        l.push(rl);
    }
    let mut ans = 0;
    for x in (k - 1)..=(r - k) {
        for y in (k - 1)..=(c - k) {
            let mut flag = true;
            for dy in -((k - 1) as i32)..=((k - 1) as i32) {
                if u[x][(y as i32 + dy) as usize] < k as i32 - dy.abs()
                    || l[x][(y as i32 + dy) as usize] < k as i32 - dy.abs()
                {
                    flag = false;
                    break;
                }
            }
            if flag {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
