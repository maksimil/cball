use std::{
    fs::{create_dir, File},
    io::Read,
    mem,
    path::Path,
    ptr,
};

use crate::common::{FileLen, EOT, NULL};

fn find<T: PartialEq>(slice: &[T], mut k: usize, v: &T) -> usize {
    while k < slice.len() && &slice[k] != v {
        k += 1;
    }
    k
}

fn fromdata<T>(bytes: &[u8]) -> T {
    unsafe {
        let mut output = mem::zeroed::<T>();
        ptr::copy(&bytes[0], mem::transmute(&mut output), mem::size_of::<T>());
        output
    }
}

pub fn unpack<P0: AsRef<Path>, P1: AsRef<Path>>(cball: P0, output: P1) {
    let buff = {
        let mut file = File::open(cball).expect("Failed to open file");
        let mut buff = Vec::new();
        file.read_to_end(&mut buff)
            .expect("Failed to read file contents");
        buff
    };

    // reading header

    // reading folder header
    let mut fend = find(&buff, 0, &EOT);
    let folders = {
        let mut folders = Vec::new();
        let mut s = 0;
        let mut f = find(&buff, 0, &NULL);
        while s < fend {
            folders.push(
                std::str::from_utf8(&buff[s..f])
                    .expect("Failed to read non-utf8 string")
                    .as_ref() as &Path,
            );
            s = f + 1;
            f = find(&buff, s, &NULL);
        }
        folders
    };

    // reading files header
    let files = {
        let mut files = Vec::new();

        let mut s = fend + 1;
        let mut f = find(&buff, s, &NULL);

        fend = find(&buff, fend + 1, &EOT);
        while s < fend {
            let name = std::str::from_utf8(&buff[s..f])
                .expect("Failed to read non-utf8 string")
                .as_ref() as &Path;
            let pos = {
                s = f + 1;
                f = s + mem::size_of::<FileLen>();
                fromdata::<FileLen>(&buff[s..f])
            };
            files.push((name, pos));
            s = f;
            f = find(&buff, s, &NULL);
        }
        files
    };

    let _filesdata = &buff[fend + 1..];

    // folder creation
    let output = output.as_ref().to_owned();
    for folder in folders {
        let mut path = output.clone();
        path.push(folder);
        create_dir(path).expect("Failed to create folder");
    }

    todo!()
}
