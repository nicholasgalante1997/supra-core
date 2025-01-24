use crate::utils::{get_ansi_color_sequence, get_color_support_level, ColorSupportLevel, is_match_with_pattern};

use chrono::Local;
use dotenv::dotenv;
use emojis;
use once_cell::sync::Lazy;
use regex::Regex;
use std::env;
use std::fmt::Display;
use std::io::{self, BufWriter, Write};
use std::sync::{Arc, Mutex};
use std::time::Instant;

const ANSI_RESET_SEQUENCE: &'static str = "\x1b[0m";
const ANSI_BOLD_SEQUENCE: &'static str = "\x1b[1m";

static EMOJI_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r":(\w+):").unwrap());

/// # RsDebugger
/// A simple logger for Rust based on the Node.js [`debug`](https://npmjs.com/package/debug) lightweight logging library.
///
/// ## Feature Parity Benchmarks
///
/// * Env based label filtering
/// * Emoji support
/// * Timed logging events
/// * ANSI 256 color support
/// * Child logger extensions
/// * Granular enabling of loggers via programmatic API
///
/// ## Usage
///
/// ```rust
/// use debugrs::RsDebugger;
///
/// let mut debugger = RsDebugger::new(String::from("app"));
/// debugger.write(String::from("App booting!... :rocket:")); // > [app] App booting!... 🚀 +0ms
///
/// let mut child = debugger.extend(String::from("child-process"));
/// child.write(String::from("Child process booting!... :rocket:")); // > [app:child-process] Child process booting!... 🚀 +221ms
/// ```
/// 
/// _RsDebugger instances need to be mutable to update internal timing state within the `write` method._
/// 
/// ## Environment Variables
///
/// * `DEBUG`: Comma separated list of labels to log or just a single label
/// 
/// ## Thread Safety
/// 
/// * `RsDebugger` is thread safe
/// 
/// Here's an example of using the thread-safe RsDebugger in a multithreaded environment:
/// 
/// ```rust
/// use std::thread;
/// use std::sync::{Arc, Mutex};
/// use debugrs::RsDebugger;
/// 
/// fn main() {
///    let mut debugger = Arc::new(Mutex::new(RsDebugger::new(String::from("app"))));
///
///    let handles: Vec<_> = (0..5).map(|i| {
///        let mut debugger = Arc::clone(&debugger);
///        thread::spawn(move || {
///            let mut debugger = debugger.lock().unwrap();
///            debugger.write(format!("Log from thread {}", i));
///        })
///   }).collect();
///
///    for handle in handles {
///        handle.join().unwrap();
///    }
///}
///
/// ```
///
pub struct RsDebugger {
    label: String,
    color: String,
    instant: Arc<Mutex<Option<Instant>>>,
    support_level: ColorSupportLevel,
    enabled: Arc<Mutex<bool>>,
}

impl RsDebugger {
    pub fn init_env() {
        dotenv().ok();
    }

    fn is_permitted_to_write(label: &String) -> bool {
        if let Ok(debug_labels) = env::var("DEBUG") {
            debug_labels.split(',').any(|pattern| is_match_with_pattern(label, pattern))
        } else {
            false
        }
    }
    
    /// Creates a new `RsDebugger` instance
    ///
    /// # Arguments
    ///
    /// * `label` - The string label for this debugger
    ///
    /// # Examples
    ///
    /// ```rust
    /// use debugrs::RsDebugger;
    ///
    /// let mut debugger = RsDebugger::new(String::from("app"));
    /// debugger.write(String::from("App booting!... :rocket:")); // > [app] App booting!... 🚀 +0ms
    /// ```
    ///
    pub fn new(label: String) -> Self {
        let enabled = Self::is_permitted_to_write(&label);
        let color = get_ansi_color_sequence();
        let support_level = get_color_support_level();

        Self {
            enabled: Arc::new(Mutex::new(enabled)),
            label,
            color,
            support_level,
            instant: Arc::new(Mutex::new(None)),
        }
    }
}

impl RsDebugger {
    /// Creates a new `RsDebugger` instance that is a child of the current debugger
    ///
    /// # Arguments
    ///
    /// * `label` - The string label for this child debugger
    ///
    /// # Examples
    ///
    /// ```rust
    /// use debugrs::RsDebugger;
    ///
    /// let mut debugger = RsDebugger::new(String::from("app"));
    /// debugger.write(String::from("App booting!... :rocket:")); // > [app] App booting!... 🚀 +0ms
    ///
    /// let mut child = debugger.extend(String::from("child-process"));
    /// child.write(String::from("Child process booting!... :rocket:")); // > [app:child-process] Child process booting!... 🚀 +221ms
    /// ```
    ///
    pub fn extend(&mut self, label: String) -> Self {
        let mut root_label = self.label.clone();
        root_label.push_str(":");
        root_label.push_str(label.as_str());

        let enabled = Self::is_permitted_to_write(&root_label);
        let color = get_ansi_color_sequence();

        Self {
            color,
            support_level: self.support_level,
            instant: Arc::new(Mutex::new(None)),
            label: root_label,
            enabled: Arc::new(Mutex::new(enabled)),
        }
    }

    /// Writes a line of output to the console, prefixed with the label and time elapsed since last log event
    ///
    /// # Examples
    ///
    /// ```rust
    /// use debugrs::RsDebugger;
    ///
    /// let mut debugger = RsDebugger::new(String::from("app"));
    /// debugger.write(String::from("App booting!... :rocket:")); // > [app] App booting!... 🚀 +0ms
    /// ```
    ///
    pub fn write<T: Display>(&mut self, logline: T) {
        if !self.is_enabled() {
            return;
        }

        let label = self.format_label();
        let timestamp = self.format_timestamp();
        let emoji_safe_logline = self.replace_emojis(format!("{}", logline));
        let formatted_logline = format!("{} {} {}\n", label, emoji_safe_logline, timestamp);
        let stdout = io::stdout();
        let mut writer = BufWriter::new(stdout.lock());
        match writer.write_all(formatted_logline.as_bytes()) {
            _ => {}
        };
        match writer.flush() {
            _ => {
                match self.instant.lock() {
                    Ok(mut instant) => {
                        *instant = Some(Instant::now());
                    }
                    Err(_) => {},
                }
            }
        };
    }

    fn format_label(&mut self) -> String {
        match self.support_level {
            ColorSupportLevel::NoColor => {
                let now = Local::now();
                let formatted_timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();
                format!("{} [{}]", formatted_timestamp, self.label)
            }
            _ => {
                format!(
                    "{}{}[{}]{}",
                    ANSI_BOLD_SEQUENCE, self.color, self.label, ANSI_RESET_SEQUENCE
                )
            }
        }
    }

    fn format_timestamp(&mut self) -> String {
        if self.support_level == ColorSupportLevel::NoColor {
            return String::from("");
        }

        match self.instant.lock() {
            Ok(instant) => {
                match *instant {
                    Some(instant) => {
                        format!(
                            "{}{} +{}ms{}",
                            ANSI_BOLD_SEQUENCE,
                            self.color,
                            instant.elapsed().as_millis(),
                            ANSI_RESET_SEQUENCE
                        )
                    },
                    None => {
                        format!(
                            "{}{} +0{}",
                            ANSI_BOLD_SEQUENCE, self.color, ANSI_RESET_SEQUENCE
                        )
                    }
                }
            },
            Err(_error) => {
                String::from("-ms")
            }
        }
    }

    fn replace_emojis(&self, logline: String) -> String {
        EMOJI_REGEX
            .replace_all(&logline, |caps: &regex::Captures| {
                let emoji_name = &caps[1];
                match emojis::get_by_shortcode(emoji_name.replace(":", "").as_str()) {
                    Some(emoji) => emoji.to_string(),
                    None => caps[0].to_string(), // If no emoji match, leave the original text
                }
            })
            .to_string()
    }

    pub fn enable(&mut self) {
        match self.enabled.lock() {
            Ok(mut enabled) => *enabled = true,
            Err(_) => {},
        }
    }

    pub fn disable(&mut self) {
        match self.enabled.lock() {
            Ok(mut enabled) => *enabled = false,
            Err(_) => {},
        }
    }

    pub fn is_enabled(&self) -> bool {
        match self.enabled.lock() {
            Ok(enabled) => *enabled,
            Err(_) => {
                false
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_emojis() {
        let debugger = RsDebugger::new(String::from("test"));
        let result = debugger.replace_emojis(String::from("Hello :smile:"));
        let mut matcher = String::from("Hello ");
        matcher.push_str(
            emojis::get_by_shortcode("smile")
                .unwrap()
                .to_string()
                .as_str(),
        );
        assert_eq!(result, matcher);
    }

    #[test]
    fn test_is_permitted_to_write() {
        env::set_var("DEBUG", "test");
        let debugger = RsDebugger::new("test".to_string());
        assert!(debugger.is_enabled());

        env::set_var("DEBUG", "other");
        let debugger = RsDebugger::new("test".to_string());
        assert!(!debugger.is_enabled());
    }

    #[test]
    fn test_is_permitted_to_write_extended() {
        env::set_var("DEBUG", "test:*");
        let debugger = RsDebugger::new("test:extended:child-process".to_string());
        assert!(debugger.is_enabled());
    }

    #[test]
    fn test_extend_label() {
        let mut debugger = RsDebugger::new("test".to_string());
        let extended = debugger.extend("sub".to_string());
        assert_eq!(extended.label, "test:sub");
    }

    #[test]
    fn test_write() {
        env::set_var("DEBUG", "test");
        env::set_var("COLORTERM", "truecolor");

        let mut debugger = RsDebugger::new("test".to_string());
        debugger.write("Hello, world! :rocket: :crab: :fire: ".to_string());
    }

    #[test]
    fn test_disables_enabled_logger() {
        env::set_var("DEBUG", "test");
        let mut debugger = RsDebugger::new("test".to_string());
        assert!(debugger.is_enabled());
        debugger.disable();
        assert!(!debugger.is_enabled());
        debugger.enable();
        assert!(debugger.is_enabled());
    }
}
