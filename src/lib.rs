//! This is the library for Duel.  Yay!

extern crate impose;
extern crate rusty_sword_arena;

pub mod audio;
pub mod player;

pub use audio::audio_loop;

/// `parse_args()` is a fancy function that parses arguments for you.  It expects the an argument
/// for name and host.
///
/// # Examples
///
/// ```
/// let args = vec!["nathan".to_string(), "localhost".to_string()];
/// let (name, host) = duel::parse_args(args);
/// assert_eq!(name, "nathan".to_string());
/// assert_eq!(host, "localhost".to_string());
/// ```
pub fn parse_args(mut args: Vec<String>) -> (String, String) {
    if args.len() != 2 {
        println!("Usage: duel NAME HOST");
        std::process::exit(2);
    }
    let host = args.pop().unwrap();
    let name = args.pop().unwrap();
    (name, host)
}

#[cfg(test)]
mod test {
    use super::parse_args;

    #[test]
    fn normal_conditions_work() {
        let input = vec![
            "nathan".to_string(),
            "localhost".to_string(),
        ];
        let output = ("nathan".to_string(), "localhost".to_string());
        assert_eq!(parse_args(input), output);
    }
}
