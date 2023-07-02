use std::{
    ffi::{OsStr, OsString},
    fs, io,
    path::PathBuf,
};

pub enum Resource {
    FromPath(ResourceFromPath),
    InMemory(ResourceInMemory),
}

impl From<ResourceFromPath> for Resource {
    fn from(value: ResourceFromPath) -> Self {
        Resource::FromPath(value)
    }
}

impl From<ResourceInMemory> for Resource {
    fn from(value: ResourceInMemory) -> Self {
        Resource::InMemory(value)
    }
}

impl Resource {
    pub fn get_file_extension(&self) -> &OsStr {
        match self {
            Resource::FromPath(r) => r.get_file_extension(),
            Resource::InMemory(r) => r.get_file_extension(),
        }
    }

    pub fn md5_hash(&self) -> &str {
        match self {
            Resource::FromPath(r) => r.md5_hash(),
            Resource::InMemory(r) => r.md5_hash(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum NewResourceFromPathError {
    #[error("File path had not extension")]
    FilePathHadNoExtension,
    #[error("{0}")]
    IO(#[from] io::Error),
}

/// Required path to have a file extension
pub struct ResourceFromPath {
    path: PathBuf,
    md5_hash: String,
}

impl ResourceFromPath {
    pub fn new(path: PathBuf) -> Result<ResourceFromPath, NewResourceFromPathError> {
        if path.extension().is_none() {
            return Err(NewResourceFromPathError::FilePathHadNoExtension);
        }
        let content = fs::read(&path)?;

        Ok(ResourceFromPath {
            path,
            md5_hash: compute_md5_hash(&content),
        })
    }

    pub fn get_file_extension(&self) -> &OsStr {
        self.path.extension().unwrap()
    }

    pub fn md5_hash(&self) -> &str {
        &self.md5_hash
    }

    pub fn load_into_memory(&self) -> io::Result<ResourceInMemory> {
        let data = fs::read(&self.path)?;
        Ok(ResourceInMemory::new(data, self.get_file_extension()))
    }
}

pub struct ResourceInMemory {
    data: Vec<u8>,
    file_extension: OsString,
    md5_hash: String,
}

impl ResourceInMemory {
    pub fn new<FE>(data: Vec<u8>, file_extension: FE) -> ResourceInMemory
    where
        FE: Into<OsString>,
    {
        let md5_hash = compute_md5_hash(&data);
        ResourceInMemory {
            data,
            file_extension: file_extension.into(),
            md5_hash,
        }
    }

    pub fn get_file_extension(&self) -> &OsStr {
        &self.file_extension
    }

    pub fn md5_hash(&self) -> &str {
        &self.md5_hash
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

fn compute_md5_hash(data: &[u8]) -> String {
    md5::compute(data)
        .0
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
