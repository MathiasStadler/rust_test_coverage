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
```

## [!INFO] - All following steps of the tutorial are carried out as a normal user in his own home directory

## Install rust via linux command line [Link](https://www.rust-lang.org/tools/install)

- command

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- output

> [!NOTE]
> Follow the large output dialog

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

## Only a list about installed toolchain

- command
TODO Fix with the current command

```bash
#
# cargo install --list
```

- output

```text
stable-x86_64-unknown-linux-gnu (active, default)
nightly-x86_64-unknown-linux-gnu
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

## Install to the last NIGHTLY VERSION

- command

```bash
rustup toolchain install nightly
```

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
> Or show all Rust binary
>
> ```bash
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

## First test inside main.rs

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
