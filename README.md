# rust_test

    - explore rust test inline the same files and separate directory further as module 
    - a rocky road
  :smiley:

> [!NOTE]
> Code block is marked by [backticks](https://commonmark.org/help/tutorial/09-code.html)
>
&nbsp;
> [!NOTE]
> Extension markdownlint: Disable problem message inside MS VSCode for code block e.g. bash code block
>
> - MD046/code-block-style: Code block style [Expected: indented; Actual: fenced]
>
> ```` <!-- markdownlint-disable-line code-block-style -->
> ```bash <!-- markdownlint-disable-line code-block-style -->
> # bash code HERE
> ```
> ````
>
&nbsp;
> [!NOTE]
> [e.g](https://simple.wikipedia.org/wiki/Exempli_gratia) Exempli gratia means "for example" in Latin
>
> - Exempli gratia is a linking word \
> mostly known in English-speaking countries \
> by its abbreviation \
> e.g., means "for example" in Latin. \
>
&nbsp;

## create project folder on linux console e.g. inside your own home directory

```bash <!-- markdownlint-disable-line code-block-style -->
# cd && mkdir <project_name> && cd $_
cd && mkdir rust_test_coverage && cd $_ 
```

## init

```bash <!-- markdownlint-disable-line code-block-style -->
touch README.md \
&& ln -s README.md README \
&& cargo init "." \
&& cargo add rustfmt \
&& rustup component add rustfmt \
&& mkdir examples \
&& cp src/main.rs examples/example.rs \
&& sed -i -e 's/world/example/g' examples/example.rs \
&& rustup  show \
&& rustup  check \
&& rustup toolchain uninstall stable \
&& rustup toolchain install stable \
&& rustup update  --force \
&& rustup show \
&& mkdir tests
```

## source

- [Functional Testing Vs. Unit Testing Vs. Integration Testing](https://www.headspin.io/blog/unit-integration-and-functional-testing-4-main-points-of-difference#:~:text=Purpose%3A%20Unit%20testing%20checks%20the,it%20functions%20as%20a%20whole.)

- [Should unit tests really be put in the same file as the source?](https://users.rust-lang.org/t/should-unit-tests-really-be-put-in-the-same-file-as-the-source/62153/2)
  
  - **REASON** A nice advantage of this separation is that it speeds up compilation and thus saves time, since the compiler does not have to analyze/compile the tests. :smiley:
  
- [Test Organization](https://doc.rust-lang.org/book/ch11-03-test-organization.html)

- [Complete Guide To Testing Code In Rust](https://zerotomastery.io/blog/complete-guide-to-testing-code-in-rust/)

- [cargo-nextest HIGH ACTIVITY](https://crates.io/crates/cargo-nextest)

> [!TIP]
> [Markdownlint: Inline rule ignore](https://github.com/DavidAnson/markdownlint/blob/main/README.md#configuration)
>
&nbsp;
> [!TIP]
> Show all already installed components
>
> ```bash <!-- markdownlint-disable-line code-block-style -->
>cargo install --list
>```
>
&nbsp;
> [!TIP]
> Or show all Rust binary
>
> ```bash <!-- markdownlint-disable-line code-block-style -->
> ls -la ~/.cargo/bin/
> ```

## Install cargo-nextest and check

```bash <!-- markdownlint-disable-line code-block-style -->
cargo install cargo-nextest 
```

> [!NOTE]
> cargo add vs cargo install
>
> - cargo add — Add dependencies to a Cargo.toml manifest file
> - cargo install - Build and install a Rust binary to the local rust installation
>
> ```bash <!-- markdownlint-disable-line code-block-style -->
> cargo add color-eyre
> cargo install cargo-nextest
>```
>
> list installed Rust binary e.g. cargo-nextest
>
> ```bash <!-- markdownlint-disable-line code-block-style -->
> cargo install --list |grep cargo-nextest
> ```

## Rust supports three types of tests

- unit
- doc
- integration

### unit test  - inside the same file

- [cargo-test Rust Book](https://doc.rust-lang.org/cargo/commands/cargo-test.html)

- [create HelloWorld with test](https://learn-with-tests.github.io/learn-rust-with-tests/)

```bash <!-- markdownlint-disable-line code-block-style -->

```

## [How write first simple test - main.rs)](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)

- rust test code
  
```rust <!-- markdownlint-disable-line code-block-style -->
//src/main.rs
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
```

- run single testcase by name  inside the project folder
  
```bash <!-- markdownlint-disable-line code-block-style -->
# cargo test <test case name>

cargo test test_println_hello_world

Compiling rust_test_coverage v0.1.0 (/home/trapapa/rust_test_coverage)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.28s
     Running unittests src/lib.rs (target/debug/deps/rust_test_coverage-45de6f6799a3e60f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/rust_test_coverage-c53a497db42afdea)

running 1 test
test tests::test_println_hello_world ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


```

## [follow the tutorial](https://zerotomastery.io/blog/complete-guide-to-testing-code-in-rust/)

## buffer

    - color-eyre v0.6.3
    - async-scoped v0.9.0
  