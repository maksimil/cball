use clap::clap_app;

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
                (@arg OUTPUT: -o "Folder to where cball will be unpacked (defaults to <CBALL>)unpacked)"))
        )
    .get_matches();
}