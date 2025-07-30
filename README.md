# Arbitime

A simple Rust library for timing code execution with convenient macros.

## Features

- `time!` - Time code execution and return both duration and result
- `format_time!` - Time code execution and format duration as a string
- `log_time!` - Time code execution with automatic logging to stderr

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
arbitime = "0.1.0"
```

## Usage

### Basic timing with `time!`

```rust
use arbitime::time;

let (duration, result) = time! {
    // Some expensive computation
    (1..=1000000).sum::<u64>()
};

println!("Result: {}, took: {:?}", result, duration);
```

### Formatted timing with `format_time!`

```rust
use arbitime::format_time;

// Single operation with custom message
let (msg, result) = format_time!("Computing sum" => {
    (1..=1000).sum::<u32>()
});
println!("{}", msg); // "Computing sum - Execution time: 42.123µs"

// Simple timing without custom message
let (msg, result) = format_time! {
    expensive_operation()
};
println!("{}", msg); // "Execution time: 1.234ms"
```

### Automatic logging with `log_time!`

```rust
use arbitime::log_time;

// Single operation with custom message
let result = log_time!("Computing sum" => {
    (1..=1000).sum::<u32>()
});
// Prints: "Computing sum - Execution time: 42.123µs"

// Simple timing without custom message
let result = log_time! {
    expensive_operation()
};
// Prints: "Execution time: 1.234ms"
```

## API Reference

### `time!`

Times the execution of a code block and returns both the duration and result as a tuple `(Duration, T)`.

### `format_time!`

Times the execution of a code block and returns a formatted timing message along with the result as a tuple `(String, T)`. The string contains a human-readable timing message.

### `log_time!`

Times the execution of code and automatically logs the duration to stderr, returning only the result. This is a convenience wrapper around `format_time!` that handles the logging automatically.

All timing information is printed to stderr using `eprintln!`.

## License

This project is licensed under the MIT License.