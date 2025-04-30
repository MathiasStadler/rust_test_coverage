# rust_test

    - explore rust test inline the same files and separate directory further as module 
    - :smiley: a rocky road

## create project folder on linux console inside home directory

    ```bash
    # cd && mkdir <project_name> && cd $_
    cd && mkdir rust_test && cd $_ 
    ```

> [!NOTE]
> Code block is marked by single quotation marks

## init

'''bash
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
'''

## source

- [Functional Testing Vs. Unit Testing Vs. Integration Testing](https://www.headspin.io/blog/unit-integration-and-functional-testing-4-main-points-of-difference#:~:text=Purpose%3A%20Unit%20testing%20checks%20the,it%20functions%20as%20a%20whole.)

- [Should unit tests really be put in the same file as the source?](https://users.rust-lang.org/t/should-unit-tests-really-be-put-in-the-same-file-as-the-source/62153/2)
  
  - **REASON** A nice benefit of that is that it speeds up compilation times, because the compiler doesn't have to parse the tests :smiley:
  
- [Test Organization](https://doc.rust-lang.org/book/ch11-03-test-organization.html)

- [Complete Guide To Testing Code In Rust](https://zerotomastery.io/blog/complete-guide-to-testing-code-in-rust/)

- [cargo-nextest HIGH ACTIVITY](https://crates.io/crates/cargo-nextest)
