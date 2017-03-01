# entropy-rs
Simple [Shannon Entropy](https://en.wiktionary.org/wiki/Shannon_entropy) implementation in pure Rust!

## Overview
This crate is designed to provide simple functionality for computing the entropy of arbitrary data. 

## Example
>Cargo.toml
```
[Dependencies]
entropy = { git = "https://github.com/rustysec/entropy-rs.git" }
```

>main.rs

```
extern crate entropy_rs;

use entropy_rs::calculate;

fn main() {
    let data = vec![0,1,2,3,4,5];
    println!("The entropy of this data is: {}", calculate(data));
}
```
>Output

```
The entropy for this data is: 2.584962500721156
```

## Additional Examples
- [example1.rs](examples/example1.rs) - Compute entropy of an arbitrary file
- [example2.rs](examples/example2.rs) - Compute the entropy for a set of strings

## License
This crate is published under an MIT License, do with it what you will.
