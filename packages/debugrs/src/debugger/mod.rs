use crate::utils::get_ansi_color_sequence;

use dotenv::dotenv;
use emojis;
use regex::Regex;
use std::env;
use std::io::{self, BufWriter, Write};
use std::time::Instant;

const ANSI_RESET_SEQUENCE: &'static str = "\x1b[0m";
const ANSI_BOLD_SEQUENCE: &'static str = "\x1b[1m";

/// # RsDebugger
/// A simple logger for Rust based on the Node.js [`debug`](https://npmjs.com/package/debug) lightweight logging library.
/// 
/// ## Feature Support
/// 
/// * Env based label filtering 
/// * Emoji support
/// * Timed logging events
/// * ANSI 256 color  support
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
pub struct RsDebugger {
    label: String,
    color: String,
    instant: Option<Instant>,
}

impl RsDebugger {
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
    /// 
    pub fn new(label: String) -> Self {
        Self {
            label,
            color: get_ansi_color_sequence(),
            instant: None
        }
    }

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
        Self {
            label: root_label,
            color: get_ansi_color_sequence(),
            instant: None,
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
    pub fn write(&mut self, logline: String) {
        if !self.is_permitted_to_write() {
            return;
        }

        let label = self.format_label();
        let timestamp = self.format_timestamp();
        let emoji_safe_logline = self.replace_emojis(logline);
        let formatted_logline = format!("{} {} {}\n", label, emoji_safe_logline, timestamp);
        let stdout = io::stdout();
        let mut writer = BufWriter::new(stdout.lock());
        match writer.write_all(formatted_logline.as_bytes()) {
            _ => {}
        };
        match writer.flush() {
            _ => {
                self.instant = Some(Instant::now());
            }
        };
    }

    fn format_label(&mut self) -> String {
        format!(
            "{}{}[{}]{}",
            ANSI_BOLD_SEQUENCE, self.color, self.label, ANSI_RESET_SEQUENCE
        )
    }

    fn format_timestamp(&mut self) -> String {
        match self.instant {
            Some(current_instant) => {
                format!(
                    "{}{} +{}ms{}",
                    ANSI_BOLD_SEQUENCE,
                    self.color,
                    current_instant.elapsed().as_millis(),
                    ANSI_RESET_SEQUENCE
                )
            }
            None => {
                format!(
                    "{}{} +0{}",
                    ANSI_BOLD_SEQUENCE, self.color, ANSI_RESET_SEQUENCE
                )
            }
        }
    }

    fn is_permitted_to_write(&mut self) -> bool {
        dotenv().ok();
        let debug_labels = env::var("DEBUG");
        match debug_labels {
            Ok(debug_labels) => {
                if debug_labels == String::from("*") {
                    return true;
                }

                self.label.contains(debug_labels.as_str())
            }
            _ => false,
        }
    }

    fn replace_emojis(&self, logline: String) -> String {
        let re = Regex::new(r":(\w+):").unwrap(); // Match strings like :emoji_name:
        re.replace_all(&logline, |caps: &regex::Captures| {
            let emoji_name = &caps[1];
            match emojis::get_by_shortcode(emoji_name.replace(":", "").as_str()) {
                Some(emoji) => emoji.to_string(),
                None => caps[0].to_string(), // If no emoji match, leave the original text
            }
        })
        .to_string()
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
        matcher.push_str(emojis::get_by_shortcode("smile").unwrap().to_string().as_str());
        assert_eq!(result, matcher);
    }

    #[test]
    fn test_is_permitted_to_write() {
        env::set_var("DEBUG", "test");
        let mut debugger = RsDebugger::new("test".to_string());
        assert!(debugger.is_permitted_to_write());
        
        env::set_var("DEBUG", "other");
        let mut debugger = RsDebugger::new("test".to_string());
        assert!(!debugger.is_permitted_to_write());
    }

    #[test]
    fn test_extend_label() {
        let mut debugger = RsDebugger::new("test".to_string());
        let extended = debugger.extend("sub".to_string());
        assert_eq!(extended.label, "test:sub");
    }
}
