use std::io::Read;

use actix_multipart::form::tempfile::TempFile;
use domain::modules::shared::{errors::ValidationError, image::Image};

pub fn parse_form_image(temp_file: &mut TempFile) -> Result<Image, ValidationError> {
    if let Some(ref content_type) = temp_file.content_type {
        let mut buffer = Vec::new();
        temp_file.file.read_to_end(&mut buffer).unwrap();
        Ok(Image::new(buffer, content_type.to_string()))
    } else {
        Err(ValidationError::new(
            "image has not content type".to_string(),
        ))
    }
}
