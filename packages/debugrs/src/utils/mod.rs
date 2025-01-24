use rand::{thread_rng, Rng};
use std::env;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColorSupportLevel {
    NoColor,
    BasicColor,
    Color256,
    Color16,
    TrueColor,
}

pub fn get_ansi_color_sequence() -> String {
    let mut rng = thread_rng();

    let support_level = get_color_support_level();

    match support_level {
        ColorSupportLevel::TrueColor => {
            let r = rng.gen_range(0..=255);
            let g = rng.gen_range(0..=255);
            let b = rng.gen_range(0..=255);
            format!("\x1b[38;2;{};{};{}m", r, g, b)
        }
        ColorSupportLevel::Color256 => {
            let color = rng.gen_range(1..=16) * rng.gen_range(1..=16);
            format!("\x1b[38;5;{}m", color)
        }
        ColorSupportLevel::Color16 => {
            let color = rng.gen_range(1..=7);
            format!("\x1b[3{}m", color)
        }
        ColorSupportLevel::BasicColor => {
            let color = rng.gen_range(1..=7);
            format!("\x1b[3{}m", color)
        }
        ColorSupportLevel::NoColor => String::from(""),
    }
}

/// Checks if the terminal supports colors by inspecting environment variables.
fn supports_colors() -> bool {
    let term = env::var("TERM").unwrap_or_else(|_error| {
        "dumb".to_string()
    });

    term != "dumb" && (term.contains("color") || env::var("COLORTERM").is_ok())
}

/// Determines if the terminal supports 256 colors.
fn supports_256_colors() -> bool {
    let term = env::var("TERM").unwrap_or_else(|_error| {
        "dumb".to_string()
    });

    term.contains("256color")
}

/// Determines if the terminal supports 16 colors.
fn supports_16_colors() -> bool {
    let term = env::var("TERM").unwrap_or_else(|_error| {
        "dumb".to_string()
    });

    term.contains("16color")
}

/// Determines if the terminal supports 24-bit true color.
fn supports_24bit_colors() -> bool {
    env::var("COLORTERM")
        .map(|v| v == "truecolor" || v == "24bit")
        .unwrap_or_else(|_error| {
            false
        })
}

pub fn get_color_support_level() -> ColorSupportLevel {
    if supports_24bit_colors() {
        ColorSupportLevel::TrueColor
    } else if supports_256_colors() {
        ColorSupportLevel::Color256
    } else if supports_16_colors() {
        ColorSupportLevel::Color16
    } else if supports_colors() {
        ColorSupportLevel::BasicColor
    } else {
        ColorSupportLevel::NoColor
    }
}

pub fn is_match_with_pattern(label: &str, pattern: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    if pattern.ends_with(":*") {
        return label.starts_with(&pattern[..pattern.len() - 1]);
    }
    label.contains(pattern)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_color_support_level_in_colorterm_env() {
        env::set_var("COLORTERM", "truecolor");

        println!("COLORTERM: {}", env::var("COLORTERM").unwrap());

        let level = get_color_support_level();
        assert_eq!(level, ColorSupportLevel::TrueColor);
    }

    #[test]
    fn test_get_color_support_level_in_256color_term_env() {
        env::remove_var("COLORTERM");
        env::set_var("TERM", "xterm-256color");

        println!("TERM: {}", env::var("TERM").unwrap());

        let level = get_color_support_level();
        assert_eq!(level, ColorSupportLevel::Color256);
    }

    #[test]
    fn test_get_color_support_level_in_16color_term_env() {
        env::remove_var("COLORTERM");
        env::set_var("TERM", "xterm-16color");

        println!("TERM: {}", env::var("TERM").unwrap());

        let level = get_color_support_level();
        assert_eq!(level, ColorSupportLevel::Color16);
    }

    #[test]
    fn test_get_color_support_level_in_color_term_env() {
        env::remove_var("COLORTERM");
        env::set_var("TERM", "xterm-color");

        println!("TERM: {}", env::var("TERM").unwrap());

        let level = get_color_support_level();
        assert_eq!(level, ColorSupportLevel::BasicColor);
    }

    #[test]
    fn test_get_color_support_level_in_no_color_term_env() {
        env::remove_var("COLORTERM");
        env::set_var("TERM", "dumb");

        println!("TERM: {}", env::var("TERM").unwrap());

        let level = get_color_support_level();
        assert_eq!(level, ColorSupportLevel::NoColor);
    }

    #[test]
    fn test_get_color_support_level_in_missing_term_env() {
        env::remove_var("COLORTERM");
        env::remove_var("TERM");
        let level = get_color_support_level();
        assert_eq!(level, ColorSupportLevel::NoColor);
    }
}
