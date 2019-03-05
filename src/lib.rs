extern crate bigdecimal;
extern crate regex;

use std::process;
use std::ops::{Mul};
use std::str::{FromStr};
use std::collections::HashMap;
use regex::Regex;
use bigdecimal::BigDecimal;

pub fn convert<'a>(value: &str, unit: &'a str) -> HashMap<&'a str, String> {
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

pub fn to_ether(value: &str, unit: &str) -> BigDecimal {
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

pub fn to_wei(value: &str, unit: &str) -> String {
    return convert(&value, &unit).get("wei").unwrap().to_string();
}

pub fn to_kwei(value: &str, unit: &str) -> String {
    return convert(&value, &unit).get("kwei").unwrap().to_string();
}

pub fn to_mwei(value: &str, unit: &str) -> String {
    return convert(&value, &unit).get("mwei").unwrap().to_string();
}

pub fn to_gwei(value: &str, unit: &str) -> String {
    return convert(&value, &unit).get("gwei").unwrap().to_string();
}

pub fn to_szabo(value: &str, unit: &str) -> String {
    return convert(&value, &unit).get("szabo").unwrap().to_string();
}

pub fn to_finney(value: &str, unit: &str) -> String {
    return convert(&value, &unit).get("finney").unwrap().to_string();
}

pub fn to_kether(value: &str, unit: &str) -> String {
    return convert(&value, &unit).get("kether").unwrap().to_string();
}

pub fn to_mether(value: &str, unit: &str) -> String {
    return convert(&value, &unit).get("mether").unwrap().to_string();
}

pub fn to_gether(value: &str, unit: &str) -> String {
    return convert(&value, &unit).get("gether").unwrap().to_string();
}

pub fn to_tether(value: &str, unit: &str) -> String {
    return convert(&value, &unit).get("tether").unwrap().to_string();
}

fn m(v: &BigDecimal, u: &str) -> BigDecimal {
    return v.mul(&BigDecimal::from_str(u).unwrap());
}

fn s(v: &BigDecimal, u: &str) -> String {
    return t(v.mul(&BigDecimal::from_str(u).unwrap()).to_string());
}

// normalize decimal places
// TODO: better way
fn t(v: String) -> String {
    let re = Regex::new(r"(.*)\.0+$").unwrap();
    let v = re.replace_all(&v, "$1").to_string();
    let re = Regex::new(r"(.*\.\d+[1-9]+)(0+)$").unwrap();
    return re.replace_all(&v, "$1").to_string();
}
