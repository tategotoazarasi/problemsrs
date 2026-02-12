# problemsrs

A collection of competitive programming solutions written in Rust. This repository serves as a personal log of solved problems, demonstrating idiomatic Rust for algorithm implementation.

## Project Structure

The project is structured as a Rust library crate, organized by problem-solving style.

- `Cargo.toml`: The package manifest that defines project metadata and dependencies.
- `src/lib.rs`: The library root, which declares the `leetcode` and `standard_io` modules.
- `src/leetcode.rs`: Contains solutions that follow the class/struct method signature style typical of LeetCode.
- `src/standard_io.rs`: Contains solutions that use standard input and output, common in competitive programming platforms like Kattis or Codeforces.
- **Tests**: Unit tests are co-located within their respective source files (`.rs`) under a `#[cfg(test)]` block, which is a common practice in Rust.

## Getting Started

### Prerequisites

- The Rust toolchain (including `rustc` and `cargo`). You can install it from [rustup.rs](https://rustup.rs/).

### Setup

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/tategotoazarasi/problemsrs.git
    cd problemsrs
    ```

## Build

The project is managed by Cargo, Rust's build system and package manager.

To build the project in debug mode:

```bash
cargo build
```

For an optimized release build:

```bash
cargo build --release
```

## Testing

The project uses Rust's integrated testing framework. Tests are defined within the source files.

To run all tests:

```bash
cargo test
```

## Documentation

The project uses `rustdoc`, Rust's built-in documentation generator, which creates documentation from comments in the source code.

To generate and open the documentation in your web browser:

```bash
cargo doc --open
```
