use proconio::input;

fn main() {
    input! {
       h1: i64,
       h2: i64,
       h3: i64,
       w1: i64,
       w2: i64,
       w3: i64
    }
    if (h1 + h2 + h3) != (w1 + w2 + w3) {
        println!("0");
        return;
    }
    let mut ans = 0;
    for a11 in 1..(h1.min(w1)) {
        for a12 in 1..(h1.min(w2)) {
            for a21 in 1..(h2.min(w1)) {
                for a22 in 1..(h2.min(w2)) {
                    let a13 = h1 - (a11 + a12);
                    let a23 = h2 - (a21 + a22);
                    let a31 = w1 - (a11 + a21);
                    let a32 = w2 - (a12 + a22);
                    let a33 = w3 - (a13 + a23);
                    if a13 > 0 && a23 > 0 && a31 > 0 && a32 > 0 && a33 > 0 {
                        ans += 1;
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
