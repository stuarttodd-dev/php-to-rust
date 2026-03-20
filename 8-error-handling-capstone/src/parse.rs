use crate::error::ConfigLineError;

pub fn split_assignment(line: &str) -> Result<(String, String), ConfigLineError> {
    let line = line.trim();
    let Some((k, v)) = line.split_once('=') else {
        return Err(ConfigLineError::MissingEquals);
    };
    let key = k.trim();
    if key.is_empty() {
        return Err(ConfigLineError::EmptyKey);
    }
    Ok((key.to_string(), v.trim().to_string()))
}

pub fn parse_value_as_i32(value: &str) -> Result<i32, ConfigLineError> {
    value
        .trim()
        .parse::<i32>()
        .map_err(|source| ConfigLineError::InvalidInteger {
            value: value.to_string(),
            source,
        })
}
