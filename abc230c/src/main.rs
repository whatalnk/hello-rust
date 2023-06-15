use proconio::input;

fn main() {
    input! {
        n: i128,
        a: i128,
        b: i128,
        p: i128,
        q: i128,
        r: i128,
        s: i128
    }
    let h = (q - p + 1) as usize;
    let w = (s - r + 1) as usize;
    let mut m = vec![vec!['.'; w]; h];
    let kmin1 = (1 - a).max(1 - b);
    let kmax1 = (n - a).min(n - b);
    let kmin2 = (1 - a).max(b - n);
    let kmax2 = (n - a).min(b - 1);
    for i in p..=q {
        for j in r..=s {
            let ki = i - a;
            let kj1 = j - b;
            let kj2 = b - j;
            if ki == kj1 && ki >= kmin1 && ki <= kmax1 && kj1 >= kmin1 && kj1 <= kmax1 {
                m[(i - p) as usize][(j - r) as usize] = '#';
            }
            if ki == kj2 && ki >= kmin2 && ki <= kmax2 && kj2 >= kmin2 && kj2 <= kmax2 {
                m[(i - p) as usize][(j - r) as usize] = '#';
            }
        }
    }
    for mi in m.iter().take(h) {
        println!("{}", mi.iter().collect::<String>());
    }
}
