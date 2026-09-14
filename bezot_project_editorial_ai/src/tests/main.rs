use crate::required_option;

#[test]
fn finds_a_required_option_value() {
    let options = vec!["--model".to_string(), "qwen2.5:14b".to_string()];

    assert_eq!(required_option(&options, "--model").unwrap(), "qwen2.5:14b");
}

#[test]
fn errors_when_the_option_is_missing() {
    let options = vec!["--topic".to_string(), "Rust async".to_string()];

    assert!(required_option(&options, "--model").is_err());
}

#[test]
fn errors_when_the_option_has_no_value() {
    let options = vec!["--model".to_string()];

    assert!(required_option(&options, "--model").is_err());
}
