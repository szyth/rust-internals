// 3.7 — The ? operator & error conversion via From
// Exercise: Secure Config Loader
// Spec: see §4 of "3.7 The question mark operator and error conversion via From.md" in the vault.

use std::{collections::HashMap, path::Path};

fn read_config_file(path: impl AsRef<Path>) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

#[derive(Debug)]
enum ParseError {
    MalformedLine(usize),
}

// Config pattern: key=value
// get a line in a raw string, and check if each line matches the above pattern
fn parse_config(raw: &str) -> Result<HashMap<String, String>, ParseError> {
    let mut config_map: HashMap<String, String> = HashMap::new();
    let lines: Vec<&str> = raw.split('\n').collect(); // or raw.lines()
    for (index, line) in lines.iter().enumerate() {
        let parts = line.split('=').collect::<Vec<&str>>();
        if line.is_empty() {
            continue;
        }
        if parts.len() != 2 {
            // Err: config line has more than 1 '='
            return Err(ParseError::MalformedLine(index + 1));
        }
        let key = parts
            .first()
            .ok_or_else(|| ParseError::MalformedLine(index + 1))?;
        let value = parts
            .last()
            .ok_or_else(|| ParseError::MalformedLine(index + 1))?;

        config_map.insert(key.to_string(), value.to_string());
    }
    Ok(config_map)
}

enum ValidationError {
    MissingField(String),
    InvalidPort(String),
}

fn validate_port(config: &HashMap<String, String>) -> Result<u16, ValidationError> {
    let result = config
        .get("port")
        .ok_or_else(|| ValidationError::MissingField("port".to_string()))
        .and_then(|raw_port| {
            let port = raw_port
                .parse::<u16>()
                .map_err(|e| ValidationError::InvalidPort(raw_port.to_string()))?;

            if port <= 1024 {
                // no need of port > 65536 aka u16::MAX as u16 parsing above already
                // checks it
                return Err(ValidationError::InvalidPort(raw_port.to_string()));
            }

            Ok(port)
        });

    result
}

enum ConfigError {
    Io(std::io::Error),
    Parse(ParseError),
    Validation(ValidationError),
}

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        ConfigError::Io(err)
    }
}
impl From<ParseError> for ConfigError {
    fn from(err: ParseError) -> Self {
        ConfigError::Parse(err)
    }
}

impl From<ValidationError> for ConfigError {
    fn from(err: ValidationError) -> Self {
        ConfigError::Validation(err)
    }
}

fn load_and_validate_port(path: impl AsRef<Path>) -> Result<u16, ConfigError> {
    let raw = read_config_file(path)?;
    let config = parse_config(&raw)?;
    let port = validate_port(&config)?;

    Ok(port)
}

fn main() {
    // Err: Wrong file
    let result = load_and_validate_port("nonexistent/config.txt");
    assert!(result.is_err());
    assert!(matches!(result, Err(ConfigError::Io(_))));

    // Err: Wrong Config syntax (key=value)
    let _ = std::fs::write("bad_config_syntax.txt", "port 8080\n").unwrap();
    let result = load_and_validate_port("bad_config_syntax.txt");
    assert!(result.is_err());
    assert!(matches!(result, Err(ConfigError::Parse(_))));

    // Err: Port missing in Config
    let _ = std::fs::write("missing_port.txt", "hostname=localhost\n").unwrap();
    let result = load_and_validate_port("missing_port.txt");
    assert!(result.is_err());
    assert!(matches!(
        result,
        Err(ConfigError::Validation(ValidationError::MissingField(_)))
    ));

    // Ok: Valid config file
    let _ = std::fs::write(
        "valid_config.txt",
        "host=localhost\nport=8080\nlog_level=debug",
    )
    .unwrap();
    let result = load_and_validate_port("valid_config.txt");
    assert!(result.is_ok());
    assert!(matches!(result, Ok(8080)));

    println!("all assertions passed");
}
