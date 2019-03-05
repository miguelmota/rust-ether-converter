extern crate ether_converter;

fn main() {
    let amt = "1";
    let amt_unit = "ether";
    let to_unit = "wei";
    let map = ether_converter::convert(&amt, &amt_unit);
    let val = map.get(to_unit).unwrap();

    println!("{} {} = {} {}", amt, amt_unit, val, to_unit);
    // 1 ether = 1000000000000000000 wei
}
