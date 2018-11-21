extern crate bigdecimal;
extern crate regex;

use std::env;
use std::process;
use std::ops::{Mul};
use std::str::{FromStr};
use std::collections::HashMap;
use regex::Regex;
use bigdecimal::BigDecimal;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() <= 2 {
        println!("two arguments are required");
        process::exit(1);
    }

    let val = &args[1];
    let unit = &args[2].to_lowercase();

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

    let map = convert(val, unit);
    for i in 0..ordermap.len() {
        let unit = ordermap.get(&i).unwrap();
        let value = map.get(unit).unwrap();
        println!("{}\t{}", unit, t(value.to_string()));
    }
}

fn to_ether(value: &str, unit: &str) -> BigDecimal {
    let v = BigDecimal::from_str(&value).unwrap();

    if unit == "wei"    { return m(&v, "0.000000000000000001") }
    if unit == "kwei"   { return m(&v, "0.000000000000001") }
    if unit == "mwei"   { return m(&v, "0.000000000001") }
    if unit == "gwei"   { return m(&v, "0.000000001") }
    if unit == "szabo"  { return m(&v, "0.000001") }
    if unit == "finney" { return m(&v, "0.001") }
    if unit == "ether"  { return m(&v, "1") }
    if unit == "kether" { return m(&v, "1000") }
    if unit == "mether" { return m(&v, "1000000") }
    if unit == "gether" { return m(&v, "1000000000") }
    if unit == "tether" { return m(&v, "1000000000000") }

    println!("unit not supported");
    process::exit(1);
}

fn convert<'a>(value: &str, unit: &'a str) -> HashMap<&'a str, String> {
    let v = to_ether(value, unit);
    let mut map: HashMap<&'a str, String> = HashMap::new();

    map.insert(unit, BigDecimal::from_str(&value).unwrap().to_string());

    if unit != "wei"    { map.insert("wei",    s(&v, "1000000000000000000")); }
    if unit != "kwei"   { map.insert("kwei",   s(&v, "1000000000000000")); }
    if unit != "mwei"   { map.insert("mwei",   s(&v, "1000000000000")); }
    if unit != "gwei"   { map.insert("gwei",   s(&v, "1000000000")); }
    if unit != "szabo"  { map.insert("szabo",  s(&v, "1000000")); }
    if unit != "finney" { map.insert("finney", s(&v, "1000")); }
    if unit != "ether"  { map.insert("ether",  s(&v, "1")); }
    if unit != "kether" { map.insert("kether", s(&v, "0.001")); }
    if unit != "mether" { map.insert("mether", s(&v, "0.000001")); }
    if unit != "gether" { map.insert("gether", s(&v, "0.000000001")); }
    if unit != "tether" { map.insert("tether", s(&v, "0.000000000001")); }

    return map;
}

fn m(v: &BigDecimal, u: &str) -> BigDecimal {
    return v.mul(&BigDecimal::from_str(u).unwrap());
}

fn s(v: &BigDecimal, u: &str) -> String {
    return v.mul(&BigDecimal::from_str(u).unwrap()).to_string();
}

fn t(v: String) -> String {
    let re = Regex::new(r"(.*)\.0+$").unwrap();
    let v = re.replace_all(&v, "$1").to_string();
    let re = Regex::new(r"(.*\.\d+[1-9]+)(0+)$").unwrap();
    return re.replace_all(&v, "$1").to_string();
}