use proconio::input;
use proconio::marker::Chars;

fn rot(s: Vec<Vec<char>>) -> Vec<Vec<char>> {
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

fn find_left_top(s: Vec<Vec<char>>) -> (usize, usize) {
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

fn is_same(s: Vec<Vec<char>>, t: Vec<Vec<char>>) -> bool {
    let n = s.len();
    let (si, sj) = find_left_top(s.clone());
    let (ti, tj) = find_left_top(t.clone());
    let offset_i = ti - si;
    let offset_j = tj - sj;
    for i in 0..n {
        for j in 0..n {
            let ii = i + offset_i;
            let jj = j + offset_j;
            if ii >= 0 && ii < n && jj >= 0 && jj < n {
                if s[i][j] != t[ii][jj] {
                    return false;
                } else {
                    if s[i][j] == '#' {
                        return false;
                    }
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
    let mut cnt_s = 0;
    let mut cnt_t = 0;
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == '#' {
                cnt_s += 1;
            }
            if t[i][j] == '#' {
                cnt_t += 1;
            }
        }
    }
    if cnt_s != cnt_t {
        println!("No");
        return;
    }
    let mut s_ = s.clone();
    for _ in 0..4 {
        if is_same(s_.clone(), t.clone()) {
            println!("Yes");
            return;
        }
        s_ = rot(s_);
    }
    println!("No");
    return;
}
