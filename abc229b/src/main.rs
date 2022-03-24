use proconio::input;

fn main() {
    input! {
        a: String,
        b: String
    }
    let la = a.len();
    let lb = b.len();
    let l = la.min(lb);
    let va: Vec<u32> = a.chars().map(|x| x.to_digit(10).unwrap()).collect();
    let vb: Vec<u32> = b.chars().map(|x| x.to_digit(10).unwrap()).collect();
    for i in 1..=l {
        if va[la - i] + vb[lb - i] >= 10 {
            println!("Hard");
            return;
        }
    }
    println!("Easy");
}
