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
2. **Create a sample log file** (e.g., `inputs/example.log`) in the project root:
    ```text
    [2026-11-06 14:03:38] [INFO] [112.195.105.127] HTTP request GET /healthz completed with status 200 in 2878ms
    [2026-11-06 14:03:54] [ERROR] [175.194.75.87] Disk write error on partition /var/log/4980
    [2026-11-06 14:04:07] [INFO] [233.49.52.91] User session started successfully for user_id=4232
    [2026-11-06 14:05:52] [ERROR] [237.210.2.186] NullPointerException in AuthModule at line 9886
    ```

3. **Run the utility with arguments:**

  * **Count total lines without filtering:**
    ```bash
    cargo run --release -- --file inputs/example.log
    ```

  * **Filter log entries by a specific pattern:**
    ```bash
    cargo run --release -- --file inputs/example.log --filter ERROR
    ```

  * **Filter log entries by a specific date and time range:**
    ```bash
    cargo run --release -- --file inputs/example.log --since "2026-11-06 14:03:38" --until "2026-11-06 14:03:54"
    ```

  * **View built-in help and available options:**
    ```bash
    cargo run -- --help
    ```

    *(Note: The double dash `--` separates `cargo` arguments from the arguments passed directly to your compiled binary).*

4. **Build a release binary (for maximum performance):**
    ```bash
    cargo build --release
    ./target/release/logpulse -file inputs/example.log -filter ERROR
    ```

---

## Static Code Analysis (Linter)

To check the codebase for idiomatic Rust patterns, performance improvements, and potential bugs, run **Clippy**:

```bash
cargo clippy
```

### Running Tests

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
