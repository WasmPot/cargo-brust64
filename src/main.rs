use docopt::Docopt;
use std::path::Path;
use serde::Deserialize;
use walkdir::WalkDir;
use std::fs;

extern crate base64;
use base64::{encode};

const USAGE: &'static str = r#"
Extract the content of static files like html, js, css in a given directory
and convert it into base64 to be stored in a rust file as Hashmap.

Usage:
    cargo brust64 [options] <in> <out>
    cargo brust64 -h | --help
    cargo brust64 --version

The first version read in a directory <in> all the files with the extension html, css, js
and encode the content of each file that will be stored in hashmap in a generated rust file <out>.

Or <in> can be a file that will be read and encoded content into a rust function.

Options:
    -h, --help           Show this help message and exit.
    --version            Show the version.
    --ignore-extension   Disable the extension verification
"#;

#[derive(Debug, Deserialize)]
struct Args {
    arg_dir: String,
    arg_out: String,
    flag_version: bool,
    flag_ignore_extension: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);

    if Path::new(&args.arg_dir).exists() {
        for entry in WalkDir::new(args.arg_dir) {
            let entry = entry.unwrap();
            if entry.path().is_file() {
                println!("{}", entry.path().display());
                let contents = fs::read_to_string(entry.path())
                    .expect("Something went wrong reading the file");
                println!("{}", encode(contents));
            }
        }
    }
}