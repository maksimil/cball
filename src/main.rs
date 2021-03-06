use std::fs::create_dir;

use clap::clap_app;

pub mod common;
pub mod pack;
pub mod unpack;

fn main() {
    let matches = clap_app!(cball =>
            (version: "0.1.0")
            (author: "Maxim Kosterov <maxim.kosterov@gmail.com>")
            (about: "Packs and unpacks a folder of files into a cball")
            (@subcommand pack =>
                (about: "Packs a folder into a CBall")
                (@arg FOLDER: +required "Folder that will be packed")
                (@arg OUTPUT: -o "CBall file name (defaults to <FOLDER>.cball)")
            )
            (@subcommand unpack =>
                (about: "Unpacks a folder from a CBall")
                (@arg CBALL: +required "CBall that will be unpacked")
                (@arg OUTPUT: -o "Folder to where cball will be unpacked (defaults to <CBALL>_unpacked)"))
        )
    .get_matches();

    if let Some(matches) = matches.subcommand_matches("pack") {
        let folder = matches
            .value_of("FOLDER")
            .expect("Failed to get folder name");
        let output = matches
            .value_of("OUTPUT")
            .map(String::from)
            .unwrap_or(format!("{}.cball", folder));
        pack::pack(folder, output);
    } else if let Some(matches) = matches.subcommand_matches("unpack") {
        let cball = matches.value_of("CBALL").expect("Failed to get CBall name");
        let output = match matches.value_of("OUTPUT").map(String::from) {
            Some(s) => s,
            None => {
                let folder = format!("{}_unpacked", cball);
                create_dir(&folder).expect("Failed to create output directory");
                folder
            }
        };

        unpack::unpack(cball, output);
    }
}
