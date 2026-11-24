# Lab 01: Hello Cargo

## Goal

Set up the Rust toolchain and get familiar with Cargo: create, build, run, format and test a project.

## Key concepts

- `rustup` manages toolchain versions; `cargo` builds, runs and tests
- `cargo new`, `cargo build`, `cargo run`, `cargo test`
- `cargo fmt` formats code to the community standard
- Tests live next to the code in a `#[cfg(test)]` module

## Coming from Java

| Java | Rust |
|------|------|
| Maven / Gradle | Cargo |
| `pom.xml` / `build.gradle` | `Cargo.toml` |
| SDKMAN or manual JDK installs | `rustup` |
| JUnit in `src/test/java` | `#[test]` in the same file, or `tests/` for integration tests |
| Checkstyle / Spotless | `cargo fmt` and `cargo clippy`, built in |

Cargo is build tool, dependency manager and test runner in one, with no plugins to configure.

## Run it

```bash
cargo run -p lab-01-hello-cargo
cargo test -p lab-01-hello-cargo
```

## What I learned

- _To be completed as I work through the lab._
