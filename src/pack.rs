use std::{
    fs::{read_dir, File, ReadDir},
    io::Write,
    mem,
    path::{Path, PathBuf},
    slice,
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

fn dataof<T>(e: &T) -> &[u8] {
    unsafe { slice::from_raw_parts(mem::transmute(e), mem::size_of::<T>()) }
}

const FWE: &str = "File writing error";

pub fn pack<P0: AsRef<Path>, P1: AsRef<Path>>(folder: P0, output: P1) {
    // open folder and check if it exists
    let readdir = read_dir(&folder).expect("Failed to open folder");

    // traverse the file tree
    let (folders, files) = fold_read_dir(folder, readdir);

    // write cball

    // write folders header
    let mut file = File::create(output).expect("Failed to create output file");
    for pb in folders {
        file.write(pb.to_string_lossy().as_bytes()).expect(FWE);
        file.write(&[0]).expect(FWE);
    }
    file.write(&[0]).expect(FWE);

    // write files header
    for (filedata, pos) in files.iter().zip(
        // computing pos in data
        files.iter().scan(0, |acc, item| {
            *acc += item.size();
            Some(*acc - item.size())
        }),
    ) {
        file.write(filedata.path.to_string_lossy().as_bytes())
            .expect(FWE);
        file.write(&[0]).expect(FWE);
        file.write(dataof(&pos)).expect(FWE);
    }
}
