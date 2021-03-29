use crate::consts::FILE_STORAGE_KEY_FOLDER;
use sqlx::types::Uuid;
use std::path::Path;

pub struct NewFile {
    pub id: Uuid,
    pub name: String,
    pub url: String,
}

impl NewFile {
    pub fn file_url(&self, base_url: &str) -> String {
        format!("{}/{}/{}", base_url, FILE_STORAGE_KEY_FOLDER, &self.id)
    }
    pub fn parse_file_path(&self) -> Result<String, String> {
        let file_path = format!("{}/{}", FILE_STORAGE_KEY_FOLDER, &self.id);
        let file_path = Path::new(".").join(file_path);
        match file_path.to_str() {
            None => Err(format!("new path is not a valid UTF-8 sequence")),
            Some(s) => Ok(s.to_string()),
        }
    }
}
