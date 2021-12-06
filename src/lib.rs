use regex::Regex;

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
