extern crate entropy_rs;

use entropy_rs::Shannon;

fn main() {
    let tests = vec!["hello world", "a", "", "i ❤ rust"];

    for test in tests {
        println!(
            "Entropy of \"{}\" is {}",
            test,
            Shannon::quick(test.as_bytes())
        );
    }
}
