use rand::{thread_rng, Rng};

pub fn get_ansi_color_sequence() -> String {
    let mut rng = thread_rng();
    let color = rng.gen_range(1..=16) * rng.gen_range(1..=16);
    format!("\x1b[38;5;{}m", color)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ansi_color_sequence() {
        let color_sequence = get_ansi_color_sequence();
        assert!(color_sequence.starts_with("\x1b[38;5;"));
        assert!(color_sequence.ends_with("m"));
    }       
    
}