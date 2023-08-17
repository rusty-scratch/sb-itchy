use std::{fs, io, path::Path};
#[derive(Debug, thiserror::Error)]
pub enum NewResourceError {
    #[error("File path had no extension")]
    FilePathHadNoExtension,
    #[error("{0}")]
    IO(#[from] io::Error),
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Resource {
    content: Vec<u8>,
    file_extension: String,
    md5_hash: String,
}

impl Resource {
    pub fn new<FE>(content: Vec<u8>, file_extension: FE) -> Resource
    where
        FE: Into<String>,
    {
        let md5_hash = compute_md5_hash(&content);
        Resource {
            content,
            file_extension: file_extension.into(),
            md5_hash,
        }
    }

    pub fn load_from_path<P: AsRef<Path>>(path: P) -> Result<Resource, NewResourceError> {
        let path = path.as_ref();
        let Some(file_extension)= path.extension() else {
            return Err(NewResourceError::FilePathHadNoExtension);
        };
        let content = fs::read(path)?;
        let md5_hash = compute_md5_hash(&content);

        Ok(Resource {
            content,
            file_extension: file_extension.to_string_lossy().to_string(),
            md5_hash,
        })
    }

    pub fn file_extension(&self) -> &str {
        &self.file_extension
    }

    pub fn md5_hash(&self) -> &str {
        &self.md5_hash
    }

    pub fn data(&self) -> &[u8] {
        &self.content
    }
}

fn compute_md5_hash(data: &[u8]) -> String {
    md5::compute(data)
        .0
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
