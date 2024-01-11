use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{Read, Write},
};

pub trait FileSystemOperations {
    fn read_file(&self, file_path: &str) -> Result<String, std::io::Error>;
    fn write_file(&mut self, file_path: &str, content: &str) -> Result<(), std::io::Error>;
}

pub struct FileSystem;

impl FileSystemOperations for FileSystem {
    fn read_file(&self, file_path: &str) -> Result<String, std::io::Error> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    fn write_file(&mut self, file_path: &str, content: &str) -> Result<(), std::io::Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)?;

        file.write_all(content.as_bytes())?;
        Ok(())
    }
}

pub struct FakeFileSystem {
    pub file_contents: HashMap<String, String>,
}

impl FileSystemOperations for FakeFileSystem {
    fn read_file(&self, file_path: &str) -> Result<String, std::io::Error> {
        match self.file_contents.get(file_path) {
            Some(content) => Ok(content.to_string()),
            None => Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "File not found",
            )),
        }
    }

    fn write_file(&mut self, file_path: &str, content: &str) -> Result<(), std::io::Error> {
        self.file_contents
            .insert(file_path.to_string(), content.to_string());
        Ok(())
    }
}

impl Default for FakeFileSystem {
    fn default() -> Self {
        FakeFileSystem {
            file_contents: HashMap::new(),
        }
    }
}
