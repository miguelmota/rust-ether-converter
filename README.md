# ether-converter

> Ether unit converter library and CLI in Rust

[![License](http://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/miguelmota/rust-ether-converter/master/LICENSE) [![Build status](https://travis-ci.org/miguelmota/rust-ether-converter.svg)](https://travis-ci.org/miguelmota/rust-ether-converter) [![Crates.io](https://img.shields.io/crates/v/ether-converter.svg)](https://crates.io/crates/ether-converter)

## Install

```bash
cargo install ether-converter
```

## Getting started

Using the library:

```rust
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
```

## CLI

```bash
$ ether_converter {value} {unit}
```

Example:

```bash
$ ether_converter 10 ether

wei     10000000000000000000
kwei    10000000000000000
mwei    10000000000000
gwei    10000000000
szabo   10000000
finney  10000
ether   10
kether  0.01
mether  0.00001
gether  0.00000001
tether  0.00000000001
```

Another example:

```bash
$ ether_converter 30 gwei

wei     30000000000
kwei    30000000
mwei    30000
gwei    30
szabo   0.03
finney  0.00003
ether   0.00000003
kether  0.00000000003
mether  0.00000000000003
gether  0.00000000000000003
tether  0.00000000000000000003
```

## Test

```bash
make test
```

## License

[MIT](LICENSE)
