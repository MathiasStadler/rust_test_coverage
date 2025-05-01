// How do you test this fn main ? It is good to separate your "domain"
// code from the outside world (side effects).
// The println! is a side effect (printing to stdout)
// and the string we send in is our domain.
// modified example from here
// [FOUND HERE](https://learn-with-tests.github.io/learn-rust-with-tests/)

fn println_hello_world() -> String {
    String::from("Hello, world!")
}

fn main() {
    println!("{}",println_hello_world());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_println_hello_world() {
        assert_eq!(println_hello_world(), "Hello, world!");
    }

}