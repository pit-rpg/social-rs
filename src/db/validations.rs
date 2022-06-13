use validator::ValidationError;

pub fn validate_trimmed(val: &str) -> Result<(), ValidationError> {
    if val.trim() != val {
        return Err(ValidationError::new("value not trimmed"));
    }

    Ok(())
}
