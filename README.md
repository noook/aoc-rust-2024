<img src="./.assets/christmas_ferris.png" width="164">

# 🎄 Advent of Code 2024 - Rust Learning Journey

> **A portfolio project showcasing systematic learning of Rust, algorithm optimization, and AI-assisted development**

This repository documents my approach to mastering Rust through [Advent of Code](https://adventofcode.com/) challenges, emphasizing performance optimization, memory efficiency, and modern development practices with AI assistance.

<!--- advent_readme_stars table --->

## 🎯 Learning Objectives

- **Rust Mastery**: Deep dive into Rust's ownership model, type system, and zero-cost abstractions
- **Algorithm Optimization**: Focus on computational efficiency and performance benchmarking
- **Memory Management**: Explore heap allocation patterns and memory optimization techniques
- **AI-Enhanced Learning**: Use AI as a learning partner for code review, refactoring guidance, and optimization insights after independent problem solving

## 🚀 Performance Benchmarks

All solutions are optimized for performance with comprehensive benchmarking. Execution times measured on optimized builds:

<!--- benchmarking table --->
## Benchmarks

| Day | Part 1 | Part 2 |
| :---: | :---: | :---:  |
| [Day 1](./src/bin/01.rs) | `4.1µs` | `40.9µs` |
| [Day 2](./src/bin/02.rs) | `407.6µs` | `492.8µs` |
| [Day 3](./src/bin/03.rs) | `258.7µs` | `243.7µs` |
| [Day 4](./src/bin/04.rs) | `72.4µs` | `48.6µs` |
| [Day 5](./src/bin/05.rs) | `232.7µs` | `347.8µs` |
| [Day 6](./src/bin/06.rs) | `49.1µs` | `50.5ms` |
| [Day 7](./src/bin/07.rs) | `1.1ms` | `24.0ms` |
| [Day 8](./src/bin/08.rs) | `15.5µs` | `33.9µs` |
| [Day 9](./src/bin/09.rs) | `259.0µs` | `4.6ms` |
| [Day 10](./src/bin/10.rs) | `304.7µs` | `29.6µs` |
| [Day 11](./src/bin/11.rs) | `104.6µs` | `6.9ms` |
| [Day 12](./src/bin/12.rs) | `4.5ms` | `8.2ms` |
| [Day 13](./src/bin/13.rs) | `37.2µs` | `36.9µs` |

**Total: 102.82ms**
<!--- benchmarking table --->

## 🧠 Development Approach

### Independent Problem Solving First
- **Self-Reliant Analysis**: Initially solve each problem independently, focusing on understanding constraints and developing algorithmic solutions
- **Original Implementation**: Write first-pass solutions using my own reasoning and Rust knowledge
- **Personal Learning**: Build problem-solving intuition and strengthen core programming skills through independent work

### AI-Enhanced Learning & Optimization
- **Code Review & Learning**: Use AI to analyze my solutions and learn about alternative approaches or Rust idioms I might have missed
- **Refactoring Guidance**: Leverage AI for suggesting optimizations while maintaining code readability and safety
- **Performance Analysis**: Apply AI to interpret DHAT profiling results and understand memory allocation patterns
- **Knowledge Expansion**: Use AI as a learning partner to deepen understanding of algorithms and Rust-specific optimizations

### Performance-First Methodology
1. **Benchmark-Driven Development**: Every solution includes comprehensive performance measurements
2. **Memory Profiling**: Regular heap allocation analysis using DHAT integration
3. **Algorithmic Optimization**: Focus on optimal time/space complexity for each problem
4. **Iterative Refinement**: Multiple optimization passes guided by profiling data

### Rust Learning Focus Areas
- **Zero-Cost Abstractions**: Leveraging Rust's performance guarantees
- **Memory Safety**: Exploring ownership, borrowing, and lifetime management
- **Type System**: Utilizing advanced type features for compile-time guarantees

### Iterative Development Process
- **Version History Documentation**: Most solution's evolution is preserved in git history, showing the journey from initial implementation to optimized final version
- **Learning Trail**: Commit history demonstrates iterative improvements, refactoring decisions, and optimization discoveries
- **Transparent Development**: Full development process visible through file history - from first working solution through performance optimizations and code quality improvements

## 🛠️ Technical Implementation

### Code Architecture
- **Modular Design**: Each day's solution is a standalone binary with dedicated input/example files
- **Test-Driven Development**: Comprehensive unit tests against example inputs for all solutions
- **Performance Monitoring**: Integrated benchmarking with automatic README updates
- **Memory Profiling**: DHAT integration for heap allocation analysis

### Rust-Specific Techniques
- **Iterator Chains**: Extensive use of functional programming patterns for clean, efficient code
- **Pattern Matching**: Leveraging Rust's powerful pattern matching for elegant problem decomposition
- **Type Safety**: Using strong typing to prevent runtime errors and encode invariants
- **Zero-Copy Operations**: Minimizing allocations through strategic use of slices and references

### Development Tools & Workflow
- **Cargo Integration**: Custom commands for scaffolding, testing, and benchmarking
- **CI/CD**: Automated testing, linting, and performance tracking
- **Profiling**: Integration with DHAT for detailed memory analysis
- **Version Control**: Systematic commit patterns documenting optimization journey with full development history preserved for learning reference

---

## 📚 Getting Started

### Quick Start
```bash
# Clone and setup
git clone <repository-url>
cd advent-of-code-rust

# Run a specific day's solution
cargo solve 01

# Benchmark performance
cargo time 01 --store

# Run all tests
cargo test

# View development history for any solution
git log --oneline src/bin/01.rs
```

### Development Environment Setup

**Prerequisites:**
1. Install the [Rust toolchain](https://www.rust-lang.org/tools/install)
2. (Recommended) Install [rust-analyzer](https://rust-analyzer.github.io/manual.html) for your editor
3. (Optional) Install [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) for debugging

This project supports all major platforms (macOS, Linux, Windows) and includes comprehensive tooling for performance analysis and development workflow optimization.

---

## 🌟 Portfolio Highlights

### Key Achievements
- **Performance Optimization**: Solutions consistently achieve microsecond-level execution times
- **Memory Efficiency**: Zero-allocation solutions where possible, with detailed profiling of necessary allocations
- **Code Quality**: Comprehensive test coverage, documentation, and adherence to Rust best practices
- **AI-Enhanced Learning**: Strategic use of AI for post-solution analysis, refactoring guidance, and learning acceleration

### Skills Demonstrated
- **Systems Programming**: Low-level optimization and memory management in Rust
- **Algorithm Design**: Implementation of efficient algorithms for complex computational problems
- **Performance Engineering**: Systematic benchmarking and optimization methodology
- **Modern Development**: Strategic AI integration for learning, automated testing, and continuous performance monitoring

### Learning Outcomes
This project demonstrates my commitment to:
- **Continuous Learning**: Systematic skill development through challenging programming exercises
- **Quality Engineering**: Focus on performance, safety, and maintainability
- **Tool Mastery**: Effective use of modern development tools and strategic AI collaboration for learning
- **Documentation**: Clear communication of technical concepts and methodologies

---

## 📖 Development Guide

The following sections provide detailed documentation for using this template and development environment.

## Usage

### ➡️ Scaffold a day

```sh
# example: `cargo scaffold 1`
cargo scaffold <day>

# output:
# Created module file "src/bin/01.rs"
# Created empty input file "data/inputs/01.txt"
# Created empty example file "data/examples/01.txt"
# ---
# 🎄 Type `cargo solve 01` to run your solution.
```

Individual solutions live in the `./src/bin/` directory as separate binaries. _Inputs_ and _examples_ live in the the `./data` directory.

Every [solution](https://github.com/fspoettel/advent-of-code-rust/blob/main/src/template.txt) has _tests_ referencing its _example_ file in `./data/examples`. Use these tests to develop and debug your solutions against the example input. In VS Code, `rust-analyzer` will display buttons for running / debugging these unit tests above the unit test blocks.

> [!TIP]
> If a day has multiple example inputs, you can use the `read_file_part()` helper in your tests instead of `read_file()`. If this e.g. applies to day 1, you can create a second example file `01-2.txt` and invoke the helper like `let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));`. This supports an arbitrary number of example files.

### ➡️ Download input for a day

> [!IMPORTANT] 
> This requires [installing the aoc-cli crate](#configure-aoc-cli-integration).

You can automatically download puzzle input and description by either appending the `--download` flag to `scaffold` (e.g. `cargo scaffold 4 --download`) or with the separate `download` command:

```sh
# example: `cargo download 1`
cargo download <day>

# output:
# [INFO  aoc] 🎄 aoc-cli - Advent of Code command-line tool
# [INFO  aoc_client] 🎅 Saved puzzle to 'data/puzzles/01.md'
# [INFO  aoc_client] 🎅 Saved input to 'data/inputs/01.txt'
# ---
# 🎄 Successfully wrote input to "data/inputs/01.txt".
# 🎄 Successfully wrote puzzle to "data/puzzles/01.md".
```

### ➡️ Run solutions for a day

```sh
# example: `cargo solve 01`
cargo solve <day>

# output:
#     Finished dev [unoptimized + debuginfo] target(s) in 0.13s
#     Running `target/debug/01`
# Part 1: 42 (166.0ns)
# Part 2: 42 (41.0ns)
```

The `solve` command runs your solution against real puzzle inputs. To run an optimized build of your code, append the `--release` flag as with any other rust program.

#### Submitting solutions

> [!IMPORTANT]
> This requires [installing the aoc-cli crate](#configure-aoc-cli-integration).

Append the `--submit <part>` option to the `solve` command to submit your solution for checking.

### ➡️ Run all solutions

```sh
cargo all

# output:
#     Running `target/release/advent_of_code`
# ----------
# | Day 01 |
# ----------
# Part 1: 42 (19.0ns)
# Part 2: 42 (19.0ns)
# <...other days...>
# Total: 0.20ms
```

This runs all solutions sequentially and prints output to the command-line. Same as for the `solve` command, the `--release` flag runs an optimized build.

### ➡️ Benchmark your solutions

```sh
# example: `cargo time 8 --store`
cargo time <day> [--all] [--store]

# output:
# Day 08
# ------
# Part 1: 1 (39.0ns @ 10000 samples)
# Part 2: 2 (39.0ns @ 10000 samples)
#
# Total (Run): 0.00ms
#
# Stored updated benchmarks.
```

The `cargo time` command allows you to benchmark your code and store timings in the readme. When benching, the runner will run your code between `10` and `10.000` times, depending on execution time of first execution, and print the average execution time.

`cargo time` has three modes of execution:

 1. `cargo time` without arguments incrementally benches solutions that do not have been stored in the readme yet and skips the rest.
 2. `cargo time <day>` benches a single solution.
 3. `cargo time --all` benches all solutions.

By default, `cargo time` does not write to the readme. In order to do so, append the `--store` flag: `cargo time --store`.

> Please note that these are not _scientific_ benchmarks, understand them as a fun approximation. 😉 Timings, especially in the microseconds range, might change a bit between invocations.

### ➡️ Run all tests

```sh
cargo test
```

To run tests for a specific day, append `--bin <day>`, e.g. `cargo test --bin 01`. You can further scope it down to a specific part, e.g. `cargo test --bin 01 part_one`.

### ➡️ Read puzzle description

> [!IMPORTANT]
> This command requires [installing the aoc-cli crate](#configure-aoc-cli-integration).

```sh
# example: `cargo read 1`
cargo read <day>

# output:
# Loaded session cookie from "/Users/<snip>/.adventofcode.session".
# Fetching puzzle for day 1, 2022...
# ...the input...
```

### ➡️ Scaffold, download & read the current aoc day

> [!IMPORTANT]
> This command requires [installing the aoc-cli crate](#configure-aoc-cli-integration).

During december, the `today` shorthand command can be used to:

 - scaffold a solution for the current day
 - download its input
 - and read the puzzle

in one go.

```sh
# example: `cargo today` on December 1st
cargo today

# output:
# Created module file "src/bin/01.rs"
# Created empty input file "data/inputs/01.txt"
# Created empty example file "data/examples/01.txt"
# ---
# 🎄 Type `cargo solve 01` to run your solution.
# [INFO  aoc] 🎄 aoc-cli - Advent of Code command-line tool
# [INFO  aoc_client] 🎅 Saved puzzle to 'data/puzzles/01.md'
# [INFO  aoc_client] 🎅 Saved input to 'data/inputs/01.txt'
# ---
# 🎄 Successfully wrote input to "data/inputs/01.txt".
# 🎄 Successfully wrote puzzle to "data/puzzles/01.md".
#
# Loaded session cookie from "/Users/<snip>/.adventofcode.session".
# Fetching puzzle for day 1, 2022...
# ...the input...
```

### ➡️ Format code

```sh
cargo fmt
```

### ➡️ Lint code

```sh
cargo clippy
```

## Optional template features

### Configure aoc-cli integration

1. Install [`aoc-cli`](https://github.com/scarvalhojr/aoc-cli/) via cargo: `cargo install aoc-cli --version 0.12.0`
2. Create the file `<home_directory>/.adventofcode.session` and paste your session cookie into it. To retrieve the session cookie, press F12 anywhere on the Advent of Code website to open your browser developer tools. Look in _Cookies_ under the _Application_ or _Storage_ tab, and copy out the `session` cookie value. [^1]

Once installed, you can use the [download command](#download-input--description-for-a-day), the read command, and automatically submit solutions via the [`--submit` flag](#submitting-solutions).

### Automatically track ⭐️ progress in the readme

This template includes [a Github action](https://github.com/k2bd/advent-readme-stars) that automatically updates the readme with your advent of code progress.

To enable it, complete the following steps:

#### 1. Create a private leaderboard

Go to the leaderboard page of the year you want to track and click _Private Leaderboard_. If you have not created a leaderboard yet, create one by clicking _Create It_. Your leaderboard should be accessible under `https://adventofcode.com/{year}/leaderboard/private/view/{aoc_user_id}`.

#### 2. Set repository secrets

Go to the _Secrets_ tab in your repository settings and create the following secrets:

-   `AOC_USER_ID`: Go to [this page](https://adventofcode.com/settings) and copy your user id. It's the number behind the `#` symbol in the first name option. Example: `3031`.
-   `AOC_YEAR`: the year you want to track. Example: `2021`.
-   `AOC_SESSION`: an active session[^2] for the advent of code website. To get this, press F12 anywhere on the Advent of Code website to open your browser developer tools. Look in your Cookies under the Application or Storage tab, and copy out the `session` cookie.

Go to the _Variables_ tab in your repository settings and create the following variable:

-   `AOC_ENABLED`: This variable controls whether the workflow is enabled. Set it to `true` to enable the progress tracker. After you complete AoC or no longer work on it, you can set this to `false` to disable the CI.

✨ You can now run this action manually via the _Run workflow_ button on the workflow page. If you want the workflow to run automatically, uncomment the `schedule` section in the `readme-stars.yml` workflow file or add a `push` trigger.

### Enable code formatting / clippy checks in the CI

Uncomment the respective sections in the `ci.yml` workflow.

### Use DHAT to profile heap allocations

If you are not only interested in the runtime of your solution, but also its memory allocation profile, you can use the template's [DHAT](https://valgrind.org/docs/manual/dh-manual.html) integration to analyze it. In order to activate DHAT, call the `solve` command with the `--dhat` flag.

```sh
cargo solve 1 --dhat

# output:
#     Running `target/dhat/1`
# dhat: Total:     276 bytes in 3 blocks
# dhat: At t-gmax: 232 bytes in 2 blocks
# dhat: At t-end:  0 bytes in 0 blocks
# dhat: The data has been saved to dhat-heap.json, and is viewable with dhat/dh_view.html
# Part 1: 9001 (4.1ms)
```

The command will output some basic stats to the command-line and generate a `dhat-heap.json` report in the repo root directory.

You can pass the report a tool like [dh-view](https://nnethercote.github.io/dh_view/dh_view.html) to view a detailed breakdown of heap allocations.

### Use VS Code to debug your code

1.  Install [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) and [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb).
2.  Set breakpoints in your code. [^3]
3.  Click _Debug_ next to the unit test or the _main_ function. [^4]
4.  The debugger will halt your program at the specific line and allow you to inspect the local stack. [^5]

## Useful crates

-   [itertools](https://crates.io/crates/itertools): Extends iterators with extra methods and adaptors. Frequently useful for aoc puzzles.
-   [regex](https://crates.io/crates/regex): Official regular expressions implementation for Rust.

A curated list of popular crates can be found on [blessed.rs](https://blessed.rs/crates).

Do you have aoc-specific crate recommendations? [Share them!](https://github.com/fspoettel/advent-of-code-rust/edit/main/README.md)

## Footnotes

[^1]: The session cookie might expire after a while (~1 month) which causes the downloads to fail. To fix this issue, refresh the `.adventofcode.session` file.
[^2]: The session cookie might expire after a while (~1 month) which causes the automated workflow to fail. To fix this issue, refresh the AOC_SESSION secret.
[^3]:
    <img src="https://user-images.githubusercontent.com/1682504/198838369-453dc22c-c645-4803-afe0-fc50d5a3f00c.png" alt="Set a breakpoint" width="450" />

[^4]:
    <img alt="Run debugger" src="https://user-images.githubusercontent.com/1682504/198838372-c89369f6-0d05-462e-a4c7-8cd97b0912e6.png" width="450" />

[^5]:
    <img alt="Inspect debugger state" src="https://user-images.githubusercontent.com/1682504/198838373-36df6996-23bf-4757-9335-0bc4c1db0276.png" width="450" />
