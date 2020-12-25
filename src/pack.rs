use std::{
    fs::{read_dir, ReadDir},
    path::{Path, PathBuf},
};

use crate::common::FileData;

type FileFoldAcc = (Vec<PathBuf>, Vec<FileData>);

fn fold_read_dir<P: AsRef<Path>>(root: P, readdir: ReadDir) -> FileFoldAcc {
    let mut acc = (Vec::new(), Vec::new());
    fold_read_dir_rec(&root, readdir, &mut acc);
    acc
}

fn fold_read_dir_rec<P: AsRef<Path>>(root: &P, readdir: ReadDir, acc: &mut FileFoldAcc) {
    for item in readdir {
        let (folders_acc, files_acc) = acc;

        let item = item.expect("Failed to open file");
        let meta = item.metadata().expect("Failed to get file metadata");

        let path_stripped = item
            .path()
            .strip_prefix(root)
            .expect("Failed to strip file path")
            .to_owned();

        if meta.is_dir() {
            folders_acc.push(path_stripped);
            fold_read_dir_rec(
                root,
                read_dir(item.path()).expect("Failed to open folder"),
                acc,
            );
        } else if meta.is_file() {
            files_acc.push(FileData::new(item.path(), path_stripped));
        }
    }
}

pub fn pack<P0: AsRef<Path>, P1: AsRef<Path>>(folder: P0, _output: P1) {
    // open folder and check if it exists
    let readdir = read_dir(&folder).expect("Failed to open folder");

    // traverse the file tree
    let (folders, files) = fold_read_dir(folder, readdir);

    println!("Folders: {:?}\nFiles: {:?}", folders, files);
}
