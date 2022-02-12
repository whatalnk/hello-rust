use proconio::input;

fn main(){
    input!{
        n: f32,
    }
    if n > 2.0 * n.log2() {
        println!("Yes");
    } else {
        println!("No");
    }
}