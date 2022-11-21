use proconio::input;

fn main() {
    input! {
        h: i64,
        m: i64
    }
    let h1 = h / 10;
    let h2 = h % 10;
    let m1 = m / 10;
    let m2 = m % 10;
    println!("{} {}: {} {}", h1, h2, m1, m2);
    for dh in 0..24 {
        for dm in 0..60 {
            let hh1 = ((h + dh) % 24) / 10;
            let hh2 = ((h + dh) % 24) % 10;
            let mm1 = ((m + dm) % 60) / 10;
            let mm2 = ((m + dm) % 60) % 10;
            println!("{}{}: {}{}", hh1, hh2, mm1, mm2);
        }
    }
}
