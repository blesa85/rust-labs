# rust-labs

![CI](https://github.com/blesa85/rust-labs/actions/workflows/ci.yml/badge.svg)

Hands-on Rust labs, written and documented by me as I learn the language coming from 8+ years of Java backend development.

One new lab every week. Each lab is a small, self-contained crate with its own notes, tests and a **"Coming from Java"** section comparing the Rust approach with what I would write in Java and Spring Boot.

## Why Rust

I build microservices and event-driven systems where reliability matters. Rust interests me because the compiler enforces at build time what Java leaves to discipline and code review: memory safety, no null references, explicit error handling and data-race-free concurrency.

## Learning path

The labs follow the structure of the course *Learn to Code with Rust* (Boris Paskhaver) as a guide. All code and notes in this repository are my own.

After the language fundamentals, the plan continues with async Rust and backend development, ending in a capstone service.

## Labs

| # | Lab | Topics | Status |
|---|-----|--------|--------|
| 01 | [Hello Cargo](labs/lab-01-hello-cargo) | toolchain, Cargo, rustfmt, first test | 🟡 In progress |
| 02 | Variables and types | mutability, shadowing, scalar and compound types | ⬜ Planned |
| 03 | Functions and control flow | expressions vs statements, `if`, `loop`, `match` basics | ⬜ Planned |
| 04 | Ownership | moves, copies, drop, the stack and the heap | ⬜ Planned |
| 05 | References and borrowing | `&` and `&mut`, borrowing rules | ⬜ Planned |
| 06 | Slices | string slices, array slices | ⬜ Planned |
| 07 | Structs | methods, associated functions | ⬜ Planned |
| 08 | Enums and pattern matching | `enum`, `match`, `if let` | ⬜ Planned |
| 09 | Option and Result | no nulls, explicit failure | ⬜ Planned |
| 10 | Vectors | `Vec<T>`, iteration, ownership in collections | ⬜ Planned |
| 11 | Project structure | crates, modules, visibility, dependencies | ⬜ Planned |
| 12 | Strings | `String` vs `&str`, UTF-8 | ⬜ Planned |
| 13 | HashMaps | `HashMap`, the entry API | ⬜ Planned |
| 14 | Error handling | `?`, custom error types | ⬜ Planned |
| 15 | Generics | generic functions and types | ⬜ Planned |
| 16 | Traits | trait bounds, default methods, `impl Trait` | ⬜ Planned |
| 17 | Lifetimes | lifetime annotations, elision rules | ⬜ Planned |
| 18 | Closures and iterators | `Fn` traits, iterator adapters | ⬜ Planned |
| 19 | Testing | unit and integration tests | ⬜ Planned |

## Capstone: claims-service

A small insurance claims backend, built after the fundamentals:

- REST API with **Axum** and **Tokio**
- **PostgreSQL** persistence with migrations
- Domain events (`ClaimOpened`, `ClaimResolved`) published to a message broker
- Unit and integration tests, **Docker** and CI

Status: ⬜ Planned

## Running the labs

```bash
# run one lab
cargo run -p lab-01-hello-cargo

# run every test in the workspace
cargo test --workspace

# format and lint, as CI does
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
```

Every push runs formatting, linting and tests on GitHub Actions.

## Structure

```
rust-labs/
├── Cargo.toml          # workspace: every lab is a member crate
├── labs/
│   └── lab-NN-name/
│       ├── Cargo.toml
│       ├── README.md   # goal, concepts, "Coming from Java", lessons learned
│       └── src/
├── templates/
│   └── LAB_TEMPLATE.md
└── .github/workflows/ci.yml
```

## Related

- [kubernetes-labs](https://github.com/blesa85/kubernetes-labs): hands-on Kubernetes labs, same weekly format.
