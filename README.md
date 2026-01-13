# Rust 101

Rust is a general purpose programming language that was created in 2006 by Graydon Hoare as a personal project while at Mozilla. It was later adopted and sponsored by Mozilla. The 0.1 release was in 2012 but had many intentional changes and breaking changes early on as major design questions were debated. The official 1.0 release was in 2015 with a promise to be backward compatible going forward. There is an RFC process to manage the language similar to Python's PEP. The [Rust Foundation](https://rustfoundation.org/) was formed to take stewardship over the language in 2021.

**The history of Rust**
- [The Untold Story of Rust](https://youtu.be/P5fL1otNsfw)
- [The History of Rust via ACM talk](https://youtu.be/79PSagCD_AY)
- [The Rust I Wanted Had No Future](https://graydon2.dreamwidth.org/307291.html)

**Good Resources**
- The site dedicated to the [Rust language](https://rust-lang.org/)
- The online book [The Rust Programming Language](https://doc.rust-lang.org/stable/book/)

## Installing Rust
The preferred way to install Rust is `rustup` and an overview of the that can be found on the Rust lang site [here](https://rust-lang.org/tools/install/)

From the Terminal/CLI

```shell
 curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
 ```

After it completes, that can be checked as follows:

```shell
$ rustc --version
rustc 1.92.0
```

The main installation directories are `$HOME/.rustup` and `$HOME/.cargo`

### Ecosystem and project setup
Every language has an ecosystem that is often harder to learn than the language syntax itself. As part of that ecosystem are also best practices on how to organize and setup a project. 

The main tool is Cargo. It is a build & package manager installed with `rustup`


```shell
$ cargo --version
cargo 1.92.0
```

It can be used to start a project

```shell
$ cargo new hello_rust
Creating binary (application) `hello_rust` package
```

The project setup for a typical Rust project:

```
hello_rust/
  ├── Cargo.toml
  ├── src/
  │   └── main.rs or lib.rs
  └── tests/
```

### Hello World
The `cargo new` provides a project with a hello world in `main.rs`

We can build it...

```shell
$ cargo build
```
Similar to `mvn` in Java the compiled files are placed in a `target` folder. We can run it...

1) From the executable file
```shell
$ ./target/debug/hello_rust
Hello, world!
```

2) using cargo
```shell
$ cargo run
Hello, world!
```

### Git
By default `cargo` also setup the project for use with SCM. Specifically git. You will have to set up the `user.email` and `user.name` (unless relying in global settings). You will probably want to change the branch from `master` to `main`. Finally you will need to create a repo in Git and setup `git remote`

```shell
$ git checkout -b main
$ git add *
$ git commit -m "Hello Rust via Cargo"
$ git remote add origin https://github.com/CodeSmell/hello_rust.git
```