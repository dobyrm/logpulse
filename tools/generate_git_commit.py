import os
import random
import time

COMMIT_TYPES = ["feat", "fix", "refactor", "perf", "docs", "test", "chore"]
SCOPES = ["parser", "filter", "ui", "models", "cli", "mmap", "rayon", "deps", "docs"]

MESSAGES = {
    "feat": [
        "add time-based filtering with --since and --until flags",
        "implement parallel log splitting using rayon",
        "support zero-copy memory mapping via memmap2",
        "add ANSI color formatting for log levels",
        "introduce log level filtering support"
    ],
    "fix": [
        "resolve thread safety issue with stdout writing using mutex",
        "fix incompatible types in conditional branches for string formatting",
        "correct timestamp parsing format string for chrono NaiveDateTime",
        "fix unused import warning for io::Write",
        "prevent race conditions during parallel scan execution"
    ],
    "refactor": [
        "flatten nested if statements to satisfy clippy lints",
        "modularize codebase into models, filter, parser, and ui modules",
        "optimize byte-level search performance in filter module",
        "clean up unused regex dependency",
        "simplify time parsing logic with is_some_and guards"
    ],
    "perf": [
        "optimize memory mapping overhead for large log files",
        "speed up line iteration using parallel chunks",
        "reduce allocations during string slicing in parser",
        "improve atomic counter performance with Relaxed ordering"
    ],
    "docs": [
        "update README with benchmarking instructions and usage examples",
        "add code documentation comments for public functions",
        "document multithreading safety considerations",
        "add instructions for compiling with release optimizations"
    ],
    "test": [
        "add unit tests for valid and invalid log line parsing",
        "add test cases for timestamp boundaries",
        "verify byte-level filter matches correctly",
        "expand test coverage for edge case log formats"
    ],
    "chore": [
        "update cargo dependencies and lockfile",
        "configure clippy lints and warnings",
        "setup gitignore for build artifacts and input logs",
        "bump project version to 0.1.0",
        "remove redundant note about code adherence in README"
    ]
}

AUTHORS = [
    ("clever_fox", "fox@forest.org"),
    ("wild_wolf", "wolf@pack.net"),
    ("fast_badger", "badger@burrow.io"),
    ("wise_owl", "owl@night.com"),
    ("brave_bear", "bear@taiga.org"),
    ("silent_lynx", "lynx@mountain.dev")
]

TIMEZONES = ["-0500", "-0400", "+0000", "+0100", "+0200"]

def generate_sha1():
    return f"{random.randint(0, 2**160 - 1):040x}"

def generate_conventional_commit():
    commit_type = random.choice(COMMIT_TYPES)
    scope = random.choice(SCOPES)
    msg = random.choice(MESSAGES[commit_type])
    return f"{commit_type}({scope}): {msg}"

def generate_git_commit_file(filename="inputs/git_commit.log", target_size_mb=10):
    os.makedirs("inputs", exist_ok=True)
    print(f"Generating git commit file {filename} (~{target_size_mb} MB)...")
    
    target_size_bytes = target_size_mb * 1024 * 1024
    written_bytes = 0
    start_time = time.time()
    
    prev_hash = generate_sha1()
    
    with open(filename, "w", encoding="utf-8") as f:
        base_timestamp = 1794570000 
        
        while written_bytes < target_size_bytes:
            base_timestamp += random.randint(60, 300)
            
            current_hash = generate_sha1()
            parent_hash = prev_hash
            prev_hash = current_hash
            
            author, email = random.choice(AUTHORS)
            timezone = random.choice(TIMEZONES)
            commit_msg = generate_conventional_commit()
            
            line = f"{current_hash} {parent_hash} {author} <{email}> {base_timestamp} {timezone}\tcommit: {commit_msg}\n"
            
            f.write(line)
            written_bytes += len(line.encode('utf-8'))
            
    elapsed = time.time() - start_time
    print(f"Done! File successfully created in {elapsed:.2f} sec. Path: {filename}")

if __name__ == "__main__":
    generate_git_commit_file()
