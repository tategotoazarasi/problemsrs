// src/standard_io.rs

use std::io::{BufRead, Write};
use std::str::FromStr;

/// # A + B Problem (Standard I/O Version)
///
/// Reads two integers from input and writes their sum to output.
/// This mimics competitive programming environments (like Codeforces, Luogu).
///
/// ## Arguments
///
/// * `input` - A mutable reference to any type implementing `BufRead`.
/// * `output` - A mutable reference to any type implementing `Write`.
///
/// ## Errors
///
/// Returns `std::io::Result<()>` if I/O operations fail.
pub fn solve_a_plus_b<R, W>(mut input: R, mut output: W) -> std::io::Result<()>
where
    R: BufRead,
    W: Write,
{
    // Read all content into a string buffer
    let mut buffer = String::new();
    input.read_to_string(&mut buffer)?;

    // Use split_whitespace for efficient tokenization
    let mut tokens = buffer.split_whitespace();

    // Parse integers safely
    // In a real contest, you might unwrap() for speed, but here we handle potential errors gently.
    let a_str = tokens.next().unwrap_or("0");
    let b_str = tokens.next().unwrap_or("0");

    let a = i32::from_str(a_str).unwrap_or(0);
    let b = i32::from_str(b_str).unwrap_or(0);

    // Write the result
    writeln!(output, "{}", a + b)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_a_plus_b_normal() {
        // Mock input
        let input_data = "10 20";
        let input = Cursor::new(input_data);

        // Mock output buffer
        let mut output = Vec::new();

        // Run solution
        solve_a_plus_b(input, &mut output).expect("Should not fail");

        // Verify output
        let result = String::from_utf8(output).expect("Output should be valid UTF-8");
        assert_eq!(result, "30\n");
    }

    #[test]
    fn test_a_plus_b_multiline() {
        // Test handling of newlines
        let input_data = "5\n15";
        let input = Cursor::new(input_data);
        let mut output = Vec::new();

        solve_a_plus_b(input, &mut output).expect("Should not fail");

        let result = String::from_utf8(output).expect("Output should be valid UTF-8");
        assert_eq!(result, "20\n");
    }

    #[test]
    fn test_a_plus_b_negative() {
        let input_data = "-5 10";
        let input = Cursor::new(input_data);
        let mut output = Vec::new();

        solve_a_plus_b(input, &mut output).expect("Should not fail");

        let result = String::from_utf8(output).expect("Output should be valid UTF-8");
        assert_eq!(result, "5\n");
    }
}
