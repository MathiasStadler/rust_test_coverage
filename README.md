# rust_test

    - explore rust test inline the same files and separate directory further as module 
    - :smiley: a rocky road

## create project folder on linux console inside home directory

    ```bash <!-- markdownlint-disable-line code-block-style -->
    # cd && mkdir <project_name> && cd $_
    cd && mkdir rust_test_coverage && cd $_ 
    ```

> [!NOTE]
> Code block is marked by [backticks](https://commonmark.org/help/tutorial/09-code.html)
>
&nbsp;
> [!NOTE]
> Extension markdownlint: disable problem message inside MS VSCode for code block e.g. bash code block
>
> - MD046/code-block-style: Code block style [Expected: indented; Actual: fenced]
>
> ```` <!-- markdownlint-disable-line code-block-style -->
> ```bash <!-- markdownlint-disable-line code-block-style -->
> # bash code HERE
> ```
> ````
>

## test code block

```bash <!-- markdownlint-disable-line code-block-style -->
# point stands for  current directory
cargo init ,
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
>
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

## install cargo-nextest and check

> [!INFO]
> [e.g](https://simple.wikipedia.org/wiki/Exempli_gratia)
>
> - Exempli gratia is a linking word
> mostly known in English-speaking countries
> by its abbreviation e.g., means "for example" in Latin.
>
&nbsp;
> [!NOTE]
> cargo add vs cargo install
>
> - cargo-add — Add dependencies to a Cargo.toml manifest file
> - cargo install - Build and install a Rust binary to the local rust installation
>
> ```bash <!-- markdownlint-disable-line code-block-style -->
> cargo add color-eyre
> cargo install nextest
>```
>
> list installed Rust binary e.g. cargo-nextest
>
> ```bash <!-- markdownlint-disable-line code-block-style -->
> cargo install --list |grep cargo-nextest
> ```

## buffer

    - color-eyre v0.6.3
    - async-scoped v0.9.0
  