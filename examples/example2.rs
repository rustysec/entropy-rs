extern crate entropy_rs;

use entropy_rs::calculate;

fn main() {
    let tests = vec![
            "hello world",
            "a",
            "",
            "i ❤ rust"
        ];

    for test in tests {
        println!("Entropy of \"{}\" is {}", test, calculate(test.as_bytes()));
    }
}
