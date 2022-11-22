use proconio::input;

fn main() {
    input! {
        h: i64,
        m: i64
    }
    let dmax = 24 * 60;
    for d in 0..dmax {
        let md = m + d;
        let hh = (h + md / 60) % 24;
        let mm = md % 60;
        let a = hh / 10;
        let b = hh % 10;
        let c = mm / 10;
        let d = mm % 10;
        let ac = a * 10 + c;
        let bd = b * 10 + d;
        if ac >= 0 && ac < 24 && bd >= 0 && bd < 60 {
            println!("{:02} {:02}", hh, mm);
            return;
        }
    }
}
