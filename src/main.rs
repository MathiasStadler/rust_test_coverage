// How do you test this fn main ? It is good to separate your "domain"
// code from the outside world (side effects).
// The println! is a side effect (printing to stdout)
// and the string we send in is our domain.
// modified example from here
// [FOUND HERE](https://learn-with-tests.github.io/learn-rust-with-tests/)

mod second;



fn println_hello_world() -> String {
    String::from("Hello, world!")
}


pub fn msg_from_second() -> String {
    second::msg()
}

fn main() {

    let msg= second::msg();
    println!("{}", println_hello_world());
    println!("{}", msg);
    println!("msg_from_second => {}", msg_from_second());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_println_hello_world() {
        assert_eq!(println_hello_world(), "Hello, world!");
    }

    #[test]
    fn test_lib_msg() {
        assert_eq!(msg_from_second(), "Hello from second");
    }

}