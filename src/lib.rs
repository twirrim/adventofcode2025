use std::borrow::Cow;
use std::fs;
use std::time::{Duration, Instant};

// Common utilities for use across multiple days

// from https://www.reddit.com/r/rust/comments/skmpnr/output_text_to_console_in_debug_mode_only/hvluai2/
// This macro will insert a println at compile time, if the code is being compiled in debug mode
// This over-optimisation avoids the overhead of thrown away log::debug entries etc.
#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => (if ::std::cfg!(debug_assertions) { ::std::println!($($arg)*); })
}

/// # Panics
///
/// Will panic if it can't read the file
#[inline]
pub fn read_file(source: &str) -> Vec<String> {
    // Reads in provided filename and returns a Vec<String>
    fs::read_to_string(source)
        .expect("Unable to read file")
        .lines()
        .map(std::string::ToString::to_string)
        .collect()
}

/// Inspired by https://stackoverflow.com/questions/26998485/is-it-possible-to-print-a-number-formatted-with-thousand-separator-in-rust#comment136853740_67834588
/// then made generic, and avoiding most of the allocations
/// This method will print out numbers with thousands separators
pub fn print_with_thousands_separator<T: std::fmt::Display>(val: T) -> String {
    let s = val.to_string();
    // Make a note if the number is negative for later use, stripping the prefix if it is.
    let (is_neg, num_str) = if let Some(stripped) = s.strip_prefix('-') {
        (true, stripped)
    } else {
        (false, s.as_str())
    };

    let mut result = String::with_capacity(s.len() + (s.len() / 3));

    // Put the negative symbol back in
    if is_neg {
        result.push('-');
    }

    let offset = num_str.len() % 3;
    if offset > 0 {
        result.push_str(&num_str[..offset]);
    }

    for (i, c) in num_str[offset..].chars().enumerate() {
        if i % 3 == 0 && (offset > 0 || i > 0) {
            result.push(',');
        }
        result.push(c);
    }

    result
}

pub struct Timer {
    start_time: Instant,
    name: Cow<'static, str>,
}

impl Drop for Timer {
    fn drop(&mut self) {
        self.elapsed();
    }
}

impl Timer {
    pub fn start<T>(name: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        let name: Cow<'static, str> = name.into();
        debug_println!("Creating timer called \"{}\"", name);
        Timer {
            start_time: Instant::now(),
            name,
        }
    }

    pub fn duration(&self) -> Duration {
        self.start_time.elapsed()
    }

    pub fn elapsed(&self) {
        println!("[{}] took {:?}", self.name, self.duration());
    }

    pub fn secs_so_far(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(-1234567 as isize, "-1,234,567")]
    #[case(9876543210 as u64, "9,876,543,210")]
    #[case(1000 as i32, "1,000")]
    #[case(255 as u8, "255")]
    #[case(-128 as i16, "-128")] // Make sure we don't get "-,128"
    fn test_thousands_separator<T: std::fmt::Display>(#[case] val: T, #[case] want: String) {
        assert_eq!(print_with_thousands_separator(val), want);
    }
}
