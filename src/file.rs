use std::io::Write;

use anyhow::Result as AnyRes;
use tracing::error;

pub struct Filer;

impl Filer {
    pub fn read_files(&self, dir_path: &str, extension: &str) -> AnyRes<Vec<(String, String)>> {
        let mut files = Vec::new();

        for entry in std::fs::read_dir(dir_path)? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                let filename = entry.file_name();
                let f_bytes = filename.to_str().unwrap_or_default().as_bytes();

                if f_bytes.len() > extension.len() {
                    let idx = f_bytes.len() - extension.len();
                    if &f_bytes[idx..] == extension.as_bytes() {
                        if let Ok(exist_file) = filename.into_string() {
                            if let Ok(content_file) = std::fs::read_to_string(entry.path()) {
                                files.push((exist_file, content_file));
                            }
                        }
                    }
                }
            }
        }

        Ok(files)
    }

    pub fn insert_to_file(&self, file_path: &str, content: &str) -> AnyRes<()> {
        Ok(std::fs::File::create(file_path)?.write_all(content.as_bytes())?)
    }

    pub fn delete_file(&self, file_path: &str) {
        let _ = std::fs::remove_file(file_path)
            .map_err(|e| error!("unable to delete {}: {}", file_path, e));
    }
}
