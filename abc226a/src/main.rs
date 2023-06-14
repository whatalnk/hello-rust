use proconio::input;

fn main() {
    input! {
        x: String,
    }
    let v: Vec<&str> = x.split_terminator('.').collect();
    let mut a = v[0].parse::<i32>().unwrap();
    let b = v[1].chars().next().unwrap().to_digit(10).unwrap();
    if b > 4 {
        a += 1;
    }
    println!("{}", a);
}
