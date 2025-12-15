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
