# entropy-rs
Simple entropy implementation(s) in pure Rust
* [Shannon Entropy](https://en.wiktionary.org/wiki/Shannon_entropy)

## Overview
This crate is designed to provide simple functionality for computing the entropy of arbitrary data. 

## Example
>Cargo.toml

```
[Dependencies]
entropy-rs = { git = "https://github.com/rustysec/entropy-rs.git" }
```

>main.rs

```rust
use entropy_rs::Shannon;

fn main() {
    let data = vec![0,1,2,3,4,5];
    println!("The entropy of this data is: {}", Shannon::quick(data));
}
```
>Output

```
The entropy for this data is: 2.584962500721156
```

>Large data size support:

```rust
use entropy_rs::{Entropy, Shannon};
fn main() -> std::io::Result<()> {
    const BLOCK_SIZE: usize = 1024;
    let mut file = std::fs::File::open("./path/to/large/file")?;
    let mut entropy = Shannon::new();
    let mut data = vec![0; BLOCK_SIZE];
    while {
        let len = file.read(&mut data).unwrap();
        entropy.input(&data[0..len]);
        len == BLOCK_SIZE
    } {}
    println!("Shannon Entropy: {}", entropy.calculate());
}

```

## Additional Examples
- [example1.rs](examples/example1.rs) - Compute entropy of an arbitrary file
- [example2.rs](examples/example2.rs) - Compute the entropy for a set of strings

## License
This crate is published under an MIT License, do with it what you will.
