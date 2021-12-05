use indicatif::ProgressBar;
use log::error;
use regex::Regex;
use std::error::Error;
use std::process::Child;

pub fn pooling_command(
    finish_message: String,
    pooling_message: String,
    error_message: String,
    pooling: &mut Child,
) -> Result<(), Box<dyn Error>> {
    let mut done = false;
    let spinner = ProgressBar::new_spinner();
    spinner.set_message(pooling_message);
    while !done {
        match pooling.try_wait() {
            Ok(Some(_status)) => {
                done = true;
            }
            Ok(None) => {
                spinner.tick();
            }
            Err(e) => {
                error!("{}", error_message);
                return Err(e.into());
            }
        };
    }
    spinner.finish_with_message(finish_message);

    Ok(())
}

pub fn is_valid_time(time: &str) -> bool {
    let time_regex = Regex::new(r"^\d{2}:\d{2}:\d{2}$").unwrap();

    return time_regex.is_match(time);
}

#[test]
fn is_not_a_valid_time() {
    assert_eq!(is_valid_time("asdf"), false);
    assert_eq!(is_valid_time("12:3:13"), false);
    assert_eq!(is_valid_time("12:31:8"), false);
    assert_eq!(is_valid_time("1:31:28"), false);
}

#[test]
fn is_a_valid_time() {
    assert_eq!(is_valid_time("12:38:12"), true);
    assert_eq!(is_valid_time("02:37:13"), true);
    assert_eq!(is_valid_time("12:01:32"), true);
    assert_eq!(is_valid_time("12:19:02"), true);
    assert_eq!(is_valid_time("00:00:00"), true);
    assert_eq!(is_valid_time("12:12:12"), true);
}
