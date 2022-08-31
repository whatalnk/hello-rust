use proconio::input;

fn distance2(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    let dx = p1.0 - p2.0;
    let dy = p1.1 - p2.1;
    return dx * dx + dy * dy;
}

fn main() {
    input! {
        (ax, ay): (f64, f64),
        (bx, by): (f64, f64),
        (cx, cy): (f64, f64),
        (dx, dy): (f64, f64),
    }
    let ab = distance2((ax, ay), (bx, by));
    let bc = distance2((bx, by), (cx, cy));
    let ca = distance2((cx, cy), (ax, ay));
    let cd = distance2((cx, cy), (dx, dy));
    let da = distance2((dx, dy), (ax, ay));
    // abc
    let cab = ((ab + ca - bc) / (2.0 * ab.sqrt() * ca.sqrt()))
        .acos()
        .to_degrees();
    let abc = ((ab + bc - ca) / (2.0 * ab.sqrt() * bc.sqrt()))
        .acos()
        .to_degrees();
    let bca = ((bc + ca - ab) / (2.0 * bc.sqrt() * ca.sqrt()))
        .acos()
        .to_degrees();
    // adc
    let dac = ((da + ca - cd) / (2.0 * da.sqrt() * ca.sqrt()))
        .acos()
        .to_degrees();
    let acd = ((ca + cd - da) / (2.0 * ca.sqrt() * cd.sqrt()))
        .acos()
        .to_degrees();
    let cda = ((cd + da - ca) / (2.0 * cd.sqrt() * da.sqrt()))
        .acos()
        .to_degrees();
    let dab = dac + cab;
    let bcd = bca + acd;
    if abc > 180.0 || bcd > 180.0 || cda > 180.0 || dab > 180.0 {
        println!("No");
    } else {
        println!("Yes");
    }
}
