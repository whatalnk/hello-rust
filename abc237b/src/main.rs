use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[i32; w]; h]
    }
    for i in 0..w {
        let mut row = Vec::<String>::new();
        for j in 0..h {
            row.push(a[j][i].to_string());
        }
        println!("{}", row.join(" "));
    }
}
