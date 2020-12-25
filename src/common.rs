use std::{fs::File, io::Read, mem::size_of, path::PathBuf};

#[derive(Debug)]
pub struct FileData {
    pub path: PathBuf,
    pub data: Vec<u8>,
}

pub const NULL: u8 = 0;
pub const EOT: u8 = 4;

pub type FileLen = u32;

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

    pub fn size(&self) -> FileLen {
        // size in cball (data with size hint)
        (self.data.len() + size_of::<FileLen>()) as FileLen
    }
}
