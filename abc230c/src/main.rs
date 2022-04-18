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
    let kmin = (1 - a).max(1 - b);
    let kmax = (n - a).min(n - b);
    println!("{} {}", kmin, kmax);
    for i in p..=q {
        for j in r..=s {
            let ki = (i as i128) - a;
            let kj = (j as i128) - b;
            if ki > kmin && ki < kmax && kj > kmin && kj < kmax {
                println!("{} {}, {} {}", i - p, j - r, ki, kj);
                m[(i - p) as usize][(j - r) as usize] = '#';
            }
        }
    }
    // let kmin = (1 - a).max(b - n);
    // let kmax = (n - a).min(b - 1);
    // for i in 0..h {
    //     for j in 0..w {
    //         let ki = (i as i128) - a + p;
    //         let kj = b - r - (j as i128);
    //         if ki >= kmin && ki <= kmax && kj >= kmin && kj <= kmax {
    //             m[i][j] = '#';
    //         }
    //     }
    // }
    for i in 0..h {
        println!("{}", m[i].iter().collect::<String>());
    }
}
