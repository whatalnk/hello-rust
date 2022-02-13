use proconio::input;

fn main(){
    input!{
        n: i64,
    }
    if n >= -2147483648 &&  n < 2147483648 {
        println!("Yes");
    } else {
        println!("No");
    }
}