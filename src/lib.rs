use std::fs;
use std::time::Instant;

// Common utilities for use across multiple days

// from https://www.reddit.com/r/rust/comments/skmpnr/output_text_to_console_in_debug_mode_only/hvluai2/
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

pub struct Timer {
    start_time: Instant,
    name: String,
}

impl Timer {
    pub fn start(name: String) -> Self {
        Timer {
            start_time: Instant::now(),
            name,
        }
    }
    pub fn elapsed(&self) {
        println!("{} took {:?}", self.name, self.start_time.elapsed());
    }
    pub fn secs_so_far(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }
}
