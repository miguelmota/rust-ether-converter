# ether_convert

> Ether unit converter CLI in Rust

## Install

```bash
cargo install ether_convert
```

## Getting started

```bash
$ ether_convert {value} {unit}
```

### Examples

```bash
$ ether_convert 10 ether

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

```bash
$ ether_convert 30 gwei

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

# License

[MIT](LICENSE)
