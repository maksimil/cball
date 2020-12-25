use std::{fs::File, io::Read, path::PathBuf};

#[derive(Debug)]
pub struct FileData {
    pub path: PathBuf,
    pub data: Vec<u8>,
}

impl FileData {
    pub fn new(path: PathBuf, path_stripped: PathBuf) -> FileData {
        let mut data = Vec::new();
        File::open(&path)
            .expect("Failed to open file")
            .read_to_end(&mut data)
            .expect("Failed to read file contents");
        FileData {
            path: path_stripped,
            data,
        }
    }

    pub fn size(&self) -> u32 {
        // size in cball (data with size hint)
        self.data.len() as u32 + 4
    }
}
