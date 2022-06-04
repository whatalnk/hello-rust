use proconio::input;

fn main() {
    input! {
        n: usize
    }
    let mut a: Vec<Vec<i64>> = vec![];
    for i in 0..n {
        let mut row: Vec<i64> = vec![];
        for j in 0..(i + 1) {
            if j == 0 || j == i {
                row.push(1);
            } else {
                row.push(a[i - 1][j - 1] + a[i - 1][j]);
            }
        }
        a.push(row);
    }
    for i in 0..n {
        let s: String = a[i]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        println!("{}", s);
    }
}
