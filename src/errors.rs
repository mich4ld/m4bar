pub enum ParserError {
    ReadFileError(std::io::Error),
    ParseTomlError(toml::de::Error),
}

impl From<std::io::Error> for ParserError {
    fn from(err: std::io::Error) -> Self {
        ParserError::ReadFileError(err)
    }
}

impl From<toml::de::Error> for ParserError {
    fn from(err: toml::de::Error) -> Self {
        ParserError::ParseTomlError(err)
    }
}