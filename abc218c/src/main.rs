use proconio::input;
use proconio::marker::Chars;

fn rot(s: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut ss = vec![];
    let n = s.len();
    for j in 0..n {
        let mut r = vec![];
        for i in 0..n {
            r.push(s[n - i - 1][j]);
        }
        ss.push(r);
    }
    return ss;
}

fn find_left_top(s: &Vec<Vec<char>>) -> (usize, usize) {
    let n = s.len();
    let mut ans = (0, 0);
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == '#' {
                ans = (i, j);
                break;
            }
        }
    }
    return ans;
}

fn is_same(s: &Vec<Vec<char>>, t: &Vec<Vec<char>>) -> bool {
    let n = s.len();
    let (si, sj) = find_left_top(&s);
    let (ti, tj) = find_left_top(&t);
    let offset_i = ti as i64 - si as i64;
    let offset_j = tj as i64 - sj as i64;
    for i in 0..n {
        for j in 0..n {
            let ii = i as i64 + offset_i;
            let jj = j as i64 + offset_j;
            if ii >= 0 && ii < n as i64 && jj >= 0 && jj < n as i64 {
                if s[i][j] != t[ii as usize][jj as usize] {
                    return false;
                }
            } else {
                if s[i][j] == '#' {
                    return false;
                }
            }
        }
    }
    return true;
}

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
        t : [Chars; n]
    }
    let cnt_s = s.iter().flatten().filter(|x| x == &&'#').count();
    let cnt_t = t.iter().flatten().filter(|x| x == &&'#').count();
    if cnt_s != cnt_t {
        println!("No");
        return;
    }
    let mut s_ = s.clone();
    for _ in 0..4 {
        if is_same(&s_, &t) {
            println!("Yes");
            return;
        }
        s_ = rot(&s_);
    }
    println!("No");
    return;
}
