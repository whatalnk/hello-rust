use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[i32; w]; h]
    }
    for i in 0..w {
        let mut row = Vec::<String>::new();
        for aj in a.iter().take(h) {
            row.push(aj[i].to_string());
        }
        println!("{}", row.join(" "));
    }
}
