# logpulse

**logpulse** is an ultra-fast CLI utility for analyzing and filtering log files, written in **Rust**. The project is built with a strong focus on maximum performance and efficient system resource utilization using memory-mapped files (`memmap2`).

---

## What Has Been Implemented

1. **High-Speed File Reading (`memmap2`):** Instead of traditional byte-by-byte or buffered I/O, files are mapped directly into the OS virtual memory space, enabling blazing-fast processing of large log files.
2. **Ergonomic CLI (`clap`):** Declarative command-line argument parsing supporting both short and long flags, along with auto-generated help menus.
3. **Zero-Allocation Pattern Filtering:** Searches for specified text patterns directly across byte slices (`&[u8]`) without allocating new strings on the heap for every line.
4. **Code Quality Assurance (`Clippy`):** Fully verified with Rust's built-in linter to ensure idiomatic code and prevent common pitfalls.

---

## Dependencies (`Cargo.toml`)

The project relies on the following crates:

```toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }
rayon = "1.8"
memmap2 = "0.9"
chrono = "0.4"
```

---

## How to Run

1. **Navigate to the project root** and ensure you have the Rust toolchain installed (`rustc` and `cargo`).

2. **Run the utility with arguments using the repository sample logs (`inputs/example_syslog.log` and `inputs/example_git_commit.log`):**

    * **Count total lines without filtering:**

      ```bash
      cargo run --release -- --sys-file inputs/example_syslog.log
      ```

    * **Filter log entries by a specific pattern:**

      ```bash
      cargo run --release -- --sys-file inputs/example_syslog.log --filter ERROR
      ```

    * **Filter log entries by a specific date and time range:**

      ```bash
      cargo run --release -- --sys-file inputs/example_syslog.log --since "2026-11-13 11:00:00" --until "2026-11-13 12:00:00"
      ```

    * **Run Incident Forensics (correlating Syslog errors with preceding Git commits):**

      ```bash
      cargo run --release -- --sys-file inputs/example_syslog.log --git-file inputs/example_git_commit.log --window-minutes 15
      ```

    * **View built-in help and available options:**
      ```bash
      cargo run -- --help
      ```

    *(Note: The double dash `--` separates `cargo` arguments from the arguments passed directly to your compiled binary).*

3. **Build and run a release binary (for maximum performance):**

    ```bash
    cargo build --release
    ./target/release/logpulse --sys-file inputs/example_syslog.log --filter ERROR
    ./target/release/logpulse --sys-file inputs/example_syslog.log --git-file inputs/example_git_commit.log --window-minutes 15
    ```

---

## Code Formatting

To automatically format the codebase according to the official Rust style guidelines, run **rustfmt**:

```bash
cargo fmt
```

To check if the code is properly formatted without making any changes (useful for CI/CD pipelines):

```bash
cargo fmt --check
```

## Static Code Analysis 

To check the codebase for idiomatic Rust patterns, performance improvements, and potential bugs, run **Clippy**:

```bash
cargo clippy
```

## Tests

Execute the unit test suite:

```bash
cargo test
```

## Development Tools (`tools/`)

Helper scripts for testing, benchmarking, and generating large test data sets.

**Usage:**
```bash
python3 tools/generate_syslog.py
python3 tools/generate_git_commit.py
```

* Creates `inputs/syslog.log` (~10 MB).
* Creates `inputs/git_commit.log` (~10 MB).
