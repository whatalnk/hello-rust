use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let max_n = 1_000_001;
    let mut p = vec![true; max_n];
    p[0] = false;
    p[1] = false;
    for i in 2..=1000 {
        if p[i] {
            let mut j = i + i;
            while j < max_n {
                p[j] = false;
                j += i;
            }
        }
    }
    // let cnt = p.iter().filter(|x| **x).collect::<Vec<_>>().len();
    let pp = ((0i128)..(p.len() as i128))
        .filter(|i| p[*i as usize])
        .collect::<Vec<i128>>();
    let mut ans = 0;
    for i in 0..(pp.len()) {
        for j in (i + 1)..(pp.len()) {
            if pp[i] * pp[j] * pp[j] * pp[j] <= n as i128 {
                ans += 1;
            } else {
                break;
            }
        }
    }
    println!("{}", ans);
}
