# [Advanced Rust testing](https://rust-exercises.com/advanced-testing/00_intro/00_welcome.html)
<!--
# [Jupyter Notebook RUST Kernel](https://developers.stellar.org/docs/tools/developer-tools/jupyter-notebooks)

> [!NOTE] - Install jupyter rust kernel
>
> - Install Visual Studio Code (VSCode)
> - Install the Jupyter Notebook extension in VSCode
> - Install the evcxr Rust Jupyter kernel with:
> - cargo install --locked evcxr_jupyter
> - evcxr_jupyter --install
> - Run the Create: New Jupyter Notebook command in VSCode
> - Click the Select Kernel button in the top right
> - Select Jupyter Kernel...
> - Select Rust by searching for Rust
>
&nbsp;
-->

## Date - start this repo

- command

```bash
date
```

- output

```text
Fri May  2 04:55:45 PM CEST 2025
```

## OS environment

- command

```bash
cat /etc/os-release
```

- output
  
```text
PRETTY_NAME="Debian GNU/Linux 12 (bookworm)"
NAME="Debian GNU/Linux"
VERSION_ID="12"
VERSION="12 (bookworm)"
VERSION_CODENAME=bookworm
ID=debian
HOME_URL="https://www.debian.org/"
SUPPORT_URL="https://www.debian.org/support"
BUG_REPORT_URL="https://bugs.debian.org/
TODO
FIXME
```

> [!IMPORTANT]
> All following steps of this tutorial are carried out as a normal user in his own home directory

## Install rust on linux via command line - bash shell [Link](https://www.rust-lang.org/tools/install)

- command

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- output

> [!NOTE]
> Follow the large output dialog - Not display here

## Rust installed version

- command

```bash
rustup show
```

- output

```text
Default host: x86_64-unknown-linux-gnu
rustup home:  /home/trapapa/.rustup

installed toolchains
--------------------
stable-x86_64-unknown-linux-gnu (active, default)
nightly-x86_64-unknown-linux-gnu

active toolchain
----------------
name: stable-x86_64-unknown-linux-gnu
active because: it's the default toolchain
installed targets:
  x86_64-unknown-linux-gnu
```

## Get version number

- command

```bash
rustc --version
```

- output

```txt
rustc 1.86.0 (05f9846f8 2025-03-31)
```

## Check / list only installed toolchain - Check for updates to Rust toolchains and rustup

- command

```bash
rustup check
```

- output

```text
stable-x86_64-unknown-linux-gnu - Up to date : 1.86.0 (05f9846f8 2025-03-31)
nightly-x86_64-unknown-linux-gnu - Update available : 1.88.0-nightly (3350c1eb3 2025-05-01) -> 1.88.0-nightly (d6a325d93 2025-05-03)
rustup - Up to date : 1.28.1
```

## Update rust to the last stable version

- command

```bash
rustup update stable
```

- output

```text
info: syncing channel updates for 'stable-x86_64-unknown-linux-gnu'

  stable-x86_64-unknown-linux-gnu unchanged - rustc 1.86.0 (05f9846f8 2025-03-31)

info: checking for self-update
```

## Install to the last ***nightly*** version

- command

```bash
rustup toolchain install nightly
```

- output

```text
 rustup toolchain install nightly
info: syncing channel updates for 'nightly-x86_64-unknown-linux-gnu'
info: latest update on 2025-05-02, rust version 1.88.0-nightly (3350c1eb3 2025-05-01)
info: downloading component 'cargo'
info: downloading component 'clippy'
info: downloading component 'rust-docs'
 20.1 MiB /  20.1 MiB (100 %)  11.6 MiB/s in  1s         
info: downloading component 'rust-std'
 27.2 MiB /  27.2 MiB (100 %)  11.7 MiB/s in  2s         
info: downloading component 'rustc'
 76.1 MiB /  76.1 MiB (100 %)  11.6 MiB/s in  6s         
info: downloading component 'rustfmt'
info: installing component 'cargo'
info: installing component 'clippy'
info: installing component 'rust-docs'
 20.1 MiB /  20.1 MiB (100 %)   9.9 MiB/s in  2s         
info: installing component 'rust-std'
 27.2 MiB /  27.2 MiB (100 %)  12.7 MiB/s in  2s         
info: installing component 'rustc'
 76.1 MiB /  76.1 MiB (100 %)  13.5 MiB/s in  5s         
info: installing component 'rustfmt
```

> [!TIP]
> Show all already installed components
>
> ```bash <!-- markdownlint-disable-line code-block-style -->
>cargo install --list
>```
>
&nbsp;
> [!TIP]
> Or show all installed Rust binary
>
> ```bash
> ls -la ~/.cargo/bin/
> ```

## Install cargo-nextest and check - A next-generation test runner for Rust [Link](https://crates.io/crates/cargo-nextest)

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

- unit - unit tests is to test each unit of code in isolation from the rest of the code [Link](https://doc.rust-lang.org/book/ch11-03-test-organization.html)
- doc - supports executing your documentation examples as tests. This makes sure that examples within your documentation are up to date and working [Link](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html)
- integration - test the library in the same way any other code would, which means they can only call functions that are part of your library’s public API. [Link](https://doc.rust-lang.org/book/ch11-03-test-organization.html)

&nbsp;
> [!TIP]
> Test Organization [Link}[https://doc.rust-lang.org/book/ch11-03-test-organization.html]
&nbsp;
> [!TIP]
> Use rust cargo to run tests in workspace root [Link](https://stackoverflow.com/questions/71460402/use-rust-cargo-to-run-tests-in-workspace-root)
&nbsp;
> [!TIP]
> Want a more interactive learning experience?
> Try out a different version of the Rust Book,
> featuring: quizzes, highlighting, visualizations, and more:[LINK](https://rust-book.cs.brown.edu/)
&nbsp;
> [!Tip]
> [Should unit tests really be put in the same file as the source?](https://users.rust-lang.org/t/should-unit-tests-really-be-put-in-the-same-file-as-the-source/62153/2)
>
>> ***REASON*** A nice advantage of this separation is that it speeds up compilation and thus saves time, since the compiler does not have to analyze/compile the tests. :smiley:
&nbsp;

## Simple testcase inside main.rs

```bash
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

> [!INFO]
> call method from library in another file [Rust lib vs main](https://www.youtube.com/watch?v=5F6pHtkWMxg&t=28s)
>
>```rust
> //lib.rs
> pub fn some_function(){
>  println("Hello from lib.rs);
> )
>```
>
> ```rust
> //main.rs call the lib function
> mod lib
> pub fn main(){
> lib.some_function();
> }
>

&nbsp;

## Simple testcase inside lib.rs

```bash
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lib_it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

&nbsp;
> [!NOTE]
> Comment inside TOML File
>
> ```toml
> # A hash symbol marks the rest of the line as a comment, except when inside a string.
> ```
>
&nbsp;

## Simple testcase inside a separate directory tests parallel  to src directory

> [!important]
> For this case must the testcase declare inside the project cargo.toml with follow line
>
> ```text
> [[test]]
> # name = name of test
> name = "it_works"
> # path relative path to the testcase 
> path = "tests/test_2,rs"
>```
>
&nbsp;

```bash
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

## install rust-grov

```bash
# as root
sudo apt update
sudo apt install grcov
sudo apt install librust-grcov-dev
```

## [How to do code coverage in Rust](https://blog.rng0.io/how-to-do-code-coverage-in-rust/)

```bash
CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test
```

## create folder for html output - my way doesn't know another

```bash
mkdir target/coverage
mkdir target/coverage/html
```

## Output as HTML

```bash
# change into project directory
# cd  <project folder>

# command to create 
grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/html

# output can you find into  <project folder>/target/coverage/html/index.html

```

## Inline coverage with VSCode

- you can do that with the **coverage gutters** extension - install first

- [Coverage-Gutter extension for visual studio code is not showing line coverage - .NET](https://stackoverflow.com/questions/74983417/coverage-gutter-extension-for-visual-studio-code-is-not-showing-line-coverage)

## GITHUB Marker

> [!NOTE]
> Useful information that users should know, even when skimming content.
<!-- -->
> [!TIP]
> Helpful advice for doing things better or more easily.
<!-- -->
> [!IMPORTANT]
> Key information users need to know to achieve their goal.
<!-- -->
> [!WARNING]
> Urgent info that needs immediate user attention to avoid problems.
<!-- -->
> [!CAUTION]
> Advises about risks or negative outcomes of certain actions.
