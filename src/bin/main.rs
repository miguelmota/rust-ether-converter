extern crate bigdecimal;
extern crate regex;
extern crate ether_converter;

use std::env;
use std::process;
use std::collections::HashMap;
use ether_converter::convert;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() <= 2 {
        println!("two arguments are required");
        process::exit(1);
    }

    let val = &args[1];
    let mut unit = args[2].to_lowercase();

    if unit == "eth" {
        unit = "ether".to_string();
    }

    let mut ordermap: HashMap<usize, &str> = HashMap::new();
    ordermap.insert(0, "wei");
    ordermap.insert(1, "kwei");
    ordermap.insert(2, "mwei");
    ordermap.insert(3, "gwei");
    ordermap.insert(4, "szabo");
    ordermap.insert(5, "finney");
    ordermap.insert(6, "ether");
    ordermap.insert(7, "kether");
    ordermap.insert(8, "mether");
    ordermap.insert(9, "gether");
    ordermap.insert(10, "tether");

    let map = convert(&val, &unit);
    for i in 0..ordermap.len() {
        let unit = ordermap.get(&i).unwrap();
        let value = map.get(unit).unwrap();
        println!("{}\t{}", unit, value.to_string());
    }
}
