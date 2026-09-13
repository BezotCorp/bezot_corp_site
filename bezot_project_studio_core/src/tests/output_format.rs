use crate::output_format::OutputFormat;

#[test]
fn parses_output_formats() {
    assert_eq!(OutputFormat::parse(None).unwrap(), OutputFormat::Text);
    assert_eq!(
        OutputFormat::parse(Some("text")).unwrap(),
        OutputFormat::Text
    );
    assert_eq!(
        OutputFormat::parse(Some("json")).unwrap(),
        OutputFormat::Json
    );
    assert!(OutputFormat::parse(Some("xml")).is_err());
}
