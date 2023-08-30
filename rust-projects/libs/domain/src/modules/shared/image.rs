use uuid::Uuid;

use super::errors::ValidationError;

pub struct Image {
    id: String,
    pub bytes: Vec<u8>,
    file_type: String,
}

impl Image {
    pub fn new(bytes: Vec<u8>, file_type: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            bytes,
            file_type,
        }
    }

    pub fn file_name(&self) -> String {
        let file_extension = self.file_type.split("/").last().unwrap();
        format!("{}.{}", self.id, file_extension)
    }
}

pub fn validate(image: &Image) -> Result<(), ValidationError> {
    validate_size(&image.bytes)?;
    validate_extension(&image.file_type)?;
    Ok(())
}

const TEN_MEGABYTES_IN_BYTES: usize = 10 * 1024 * 1024;
fn validate_size(bytes: &[u8]) -> Result<(), ValidationError> {
    if bytes.len() > TEN_MEGABYTES_IN_BYTES {
        Err(ValidationError::new(
            "image must be less than or equal to 10 MB".to_string(),
        ))
    } else {
        Ok(())
    }
}

const ALLOWED_EXTENSIONS: [&'static str; 3] = ["image/png", "image/jpg", "image/jpeg"];
fn validate_extension(file_type: &str) -> Result<(), ValidationError> {
    if !ALLOWED_EXTENSIONS.contains(&file_type) {
        Err(ValidationError::new(format!(
            "allowed file types are {}",
            ALLOWED_EXTENSIONS.join(",")
        )))
    } else {
        Ok(())
    }
}
