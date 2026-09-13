#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputFormat {
    Text,
    Json,
}

impl OutputFormat {
    pub fn parse(value: Option<&str>) -> Result<Self, String> {
        match value {
            None => Ok(Self::Text),
            Some("text") => Ok(Self::Text),
            Some("json") => Ok(Self::Json),
            Some(value) => Err(format!(
                "unknown output format \"{value}\"; expected text or json"
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
