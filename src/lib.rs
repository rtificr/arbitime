//! # Arbitime
//! 
//! A simple Rust library for timing code execution with convenient macros.
//! 
//! ## Features
//! 
//! - [`time!`] - Time code execution and return both duration and result
//! - [`format_time!`] - Time code execution and format duration as a string
//! - [`log_time!`] - Time code execution with automatic logging to stderr
//! 
//! ## Examples
//! 
//! ### Basic timing with [`time!`]
//! 
//! ```rust
//! use arbitime::time;
//! 
//! let (duration, result) = time! {
//!     // Some expensive computation
//!     (1..=1000000).sum::<u64>()
//! };
//! 
//! println!("Result: {}, took: {:?}", result, duration);
//! ```
//! 
//! ### Formatted timing with [`format_time!`]
//! 
//! ```rust
//! use arbitime::format_time;
//! 
//! let (msg, result) = format_time!("Computing sum" => {
//!     (1..=1000).sum::<u32>()
//! });
//! 
//! println!("{}", msg); // "Computing sum - Execution time: ..."
//! ```
//! 
//! ### Logging timing with [`log_time!`]
//! 
//! ```rust
//! use arbitime::log_time;
//! 
//! // Single operation with custom message
//! let result = log_time!("Computing sum" => {
//!     (1..=1000).sum::<u32>()
//! });
//! 
//! // Simple timing without custom message
//! log_time! {
//!     (1..=100).sum::<u32>()
//! };
//! ```

/// Times the execution of a code block and returns both the duration and result.
/// 
/// This macro measures the time it takes to execute the given code and returns
/// a tuple containing the duration and the result of the code execution.
/// 
/// # Examples
/// 
/// ```rust
/// use arbitime::time;
/// 
/// let (duration, result) = time! {
///     let mut sum = 0;
///     for i in 1..=100 {
///         sum += i;
///     }
///     sum
/// };
/// 
/// assert_eq!(result, 5050);
/// println!("Computation took: {:?}", duration);
/// ```
/// 
/// # Returns
/// 
/// A tuple `(Duration, T)` where:
/// - `Duration` is the time elapsed during execution
/// - `T` is the result of the executed code
#[macro_export]
macro_rules! time {
    ($($body:tt)*) => {{
        let __start = std::time::Instant::now();
        let __result = { $($body)* };
        let __duration = __start.elapsed();
        (__duration, __result)
    }};
}
/// Times the execution of code blocks and formats the duration as a string.
/// 
/// This macro provides several convenient ways to time code execution and format
/// the timing information as a string. It supports single operations, multiple 
/// operations, and operations with custom messages.
/// 
/// # Examples
/// 
/// ## Single operation with message
/// 
/// ```rust
/// use arbitime::format_time;
/// 
/// let (msg, result) = format_time!(
///     "Database query" => {
///         // Simulate some work
///         200
///     }
/// );
/// // msg contains: "Database query - Execution time: ..."
/// assert_eq!(result, 200);
/// ```
/// 
/// ## Multiple operations
/// 
/// ```rust
/// use arbitime::format_time;
/// 
/// let (msg1, result1) = format_time!("Fast calculation" => 2 + 2);
/// let (msg2, result2) = format_time!(
///     "Slow calculation" => {
///         let mut result = 0;
///         for i in 1..=100 {
///             result += i;
///         }
///         result
///     }
/// );
/// // Each call returns a formatted message and the result
/// ```
/// 
/// ## Simple timing without custom message
/// 
/// ```rust
/// use arbitime::format_time;
/// 
/// let (msg, result) = format_time! {
///     (1..=100).sum::<u32>()
/// };
/// // msg contains: "Execution time: ..."
/// ```
/// 
/// # Returns
/// 
/// A tuple `(String, T)` where:
/// - `String` is the formatted timing message
/// - `T` is the result of the executed code
#[macro_export]
macro_rules! format_time {
    ($($msg:expr => { $($body:tt)* }),+ $(,)?) => {
        {
            $(
                {
                    let (duration, result) = $crate::time!({ $($body)* });
                    (format!("{} - Execution time: {:?}", $msg, duration), result)
                }
            );+
        }
    };
    // Multiple message-body pairs without braces
    ($($msg:expr => $body:expr),+ $(,)?) => {
        {
            $(
                {
                    let (duration, result) = $crate::time!($body);
                    (format!("{} - Execution time: {:?}", $msg, duration), result)
                }
            );+
        }
    };
    // Single message-body pair with braces
    // This arm is redundant because the first macro arm already matches ($msg:expr => { $($body:tt)* })
    // and handles the case. You can safely remove this arm.
    // Single message-body pair without braces
    ($msg:expr => $body:expr) => {
        {
            let (duration, result) = $crate::time!($body);
            (format!("{} - Execution time: {:?}", $msg, duration), result)
        }
    };
    // Just body without message
    ($($body:tt)*) => {
        {
            let (duration, result) = $crate::time!($($body)*);
            (format!("Execution time: {:?}", duration), result)
        }
    };
}
/// Times the execution of code and automatically logs the duration to stderr.
/// 
/// This is a convenience macro that combines [`format_time!`] with automatic logging.
/// It times the execution of code and prints the timing information to stderr,
/// returning only the result of the executed code.
/// 
/// # Examples
/// 
/// ## Single operation with message
/// 
/// ```rust
/// use arbitime::log_time;
/// 
/// let result = log_time!("Database query" => {
///     // Simulate some work
///     42
/// });
/// // Prints: "Database query - Execution time: ..."
/// assert_eq!(result, 42);
/// ```
/// 
/// ## Simple timing without custom message
/// 
/// ```rust
/// use arbitime::log_time;
/// 
/// let result = log_time! {
///     (1..=100).sum::<u32>()
/// };
/// // Prints: "Execution time: ..."
/// ```
/// 
/// # Output
/// 
/// All timing information is printed to stderr using `eprintln!`.
/// 
/// # Returns
/// 
/// The result of the executed code (type `T`).
#[macro_export]
macro_rules! log_time {
    ($($expr:tt)*) => {{
        let (msg, result) = $crate::format_time!($($expr)*);
        eprintln!("{}", msg);
        result
    }}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (duration, result) = time! {
            let sum: u32 = (1..=1000).sum();
            sum
        };
        let logged_result = log_time!(
            "Summing numbers" => {
                let sum: u32 = (1..=1000).sum();
                sum
            }
        );
        
        log_time!(
            "First operation" => 5*5,
            "Second operation" => {
                10 + 15
            }
        );
        
        assert_eq!(result, 500500);
        assert_eq!(logged_result, 500500);
        assert!(duration >= std::time::Duration::new(0, 0));
    }
}
