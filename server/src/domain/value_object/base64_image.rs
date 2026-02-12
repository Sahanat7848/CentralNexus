use anyhow::Result;
use base64::{Engine, engine::general_purpose};

#[derive(Debug, Clone)]
pub struct Base64Image(String); // tuple struct

impl Base64Image {
    pub fn new(data: String) -> Result<Self> {
        if data.is_empty() {
            return Err(anyhow::anyhow!("Image data cannot be empty !!"));
        }

        let bytes = match general_purpose::STANDARD.decode(&data) {
            Ok(bs) => bs,
            Err(_) => return Err(anyhow::anyhow!("Invalid image data !!")),
        };

        let file_type = match infer::get(&bytes) {
            Some(t) if t.mime_type() == "image/png" || t.mime_type() == "image/jpeg" => {
                t.mime_type()
            }
            _ => return Err(anyhow::anyhow!("Unsupported or invalid image type !!")),
        };

        let base64text = format!("data:{};base64,{}", file_type, data);

        Ok(Self(base64text))
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}