use proconio::input;

fn main() {
    input! {
        _w: usize
    }
    println!("297");
    let mut ans: Vec<usize> = vec![];
    for i in 1..=99 {
        ans.push(i);
        ans.push(i * 100);
        ans.push(i * 10000);
    }
    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
