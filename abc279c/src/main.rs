use proconio::{input, marker::Chars};

fn transpose(v: &Vec<Vec<char>>) -> Vec<String> {
    let h = v.len();
    let w = v[0].len();
    let mut ret = vec![];
    for i in 0..w {
        let mut c = vec![];
        for j in 0..h {
            c.push(v[j][i]);
        }
        ret.push(c.iter().collect::<String>());
    }
    return ret;
}

fn main() {
    input! {
        h: usize,
        _w: usize,
        s: [Chars; h],
        t: [Chars; h],
    }
    let mut ss = transpose(&s);
    ss.sort();
    let mut tt = transpose(&t);
    tt.sort();
    if ss == tt {
        println!("Yes");
    } else {
        println!("No");
    }
}
