#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandMode {
    Dev,
    Production,
}

impl CommandMode {
    pub fn parse(value: Option<&str>) -> Result<Self, String> {
        match value {
            Some("dev") => Ok(Self::Dev),
            Some("production") => Ok(Self::Production),
            Some(value) => Err(format!(
                "unknown command \"{value}\"; expected dev or production"
            )),
            None => Err("missing command; expected dev or production".to_string()),
        }
    }

    pub fn assembler_command(self) -> &'static str {
        match self {
            Self::Dev => "dev",
            Self::Production => "production",
        }
    }

    pub fn use_release_assembler(self) -> bool {
        matches!(self, Self::Production)
    }
}
