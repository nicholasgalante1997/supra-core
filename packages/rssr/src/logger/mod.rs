pub mod debugger_lite {
    use dotenv::dotenv;
    use emojis;
    use rand::{thread_rng, Rng};
    use regex::Regex;
    use std::env;
    use std::io::{self, BufWriter, Write};
    use std::time::Instant;

    const ANSI_RESET_SEQUENCE: &'static str = "\x1b[0m";
    const ANSI_BOLD_SEQUENCE: &'static str = "\x1b[1m";

    fn get_ansi_color_sequence() -> String {
        let mut rng = thread_rng();
        let color = rng.gen_range(1..=16) * rng.gen_range(1..=16);
        format!("\x1b[38;5;{}m", color)
    }

    pub struct RsDebugger {
        label: String,
        color: String,
        instant: Option<Instant>,
    }

    impl RsDebugger {
        pub fn new(label: String) -> Self {
            Self {
                label,
                color: get_ansi_color_sequence(),
                instant: None,
            }
        }

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
}
