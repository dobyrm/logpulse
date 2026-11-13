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
regex = "1.10"
chrono = "0.4"
```

---

## How to Run

1. **Navigate to the project root** and ensure you have the Rust toolchain installed (`rustc` and `cargo`).
2. **Create a sample log file** (e.g., `inputs/example.log`) in the project root:
   ```text
   INFO: Server started on port 8080
   ERROR: Connection timed out on IP 192.168.1.10
   INFO: User logged in successfully
   ERROR: Database connection failed
   ```

3. **Run the utility with arguments:**

   * **Count total lines without filtering (long flag):**
     ```bash
     cargo run -- --file inputs/example.log
     ```

   * **Count total lines (short flag):**
     ```bash
     cargo run -- -f inputs/example.log
     ```

   * **Filter log entries by a specific pattern (long flags):**
     ```bash
     cargo run -- --file inputs/example.log --filter ERROR
     ```

   * **Filter log entries using short flags (`-f` for file, `-t` for text filter):**
     ```bash
     cargo run -- -f inputs/example.log -t ERROR
     ```

   * **View built-in help and available options:**
     ```bash
     cargo run -- --help
     ```

   *(Note: The double dash `--` separates `cargo` arguments from the arguments passed directly to your compiled binary).*

4. **Build a release binary (for maximum performance):**
   ```bash
   cargo build --release
   ./target/release/logpulse -f inputs/example.log -t ERROR
   ```

---

## Static Code Analysis (Linter)

To check the codebase for idiomatic Rust patterns, performance improvements, and potential bugs, run **Clippy**:

```bash
cargo clippy
```

If no warnings are emitted, your code adheres to Rust's best practices!
