use docopt::Docopt;
use std::path::Path;
use serde::{Serialize, Deserialize};
use walkdir::WalkDir;
use std::{fs, env};
use std::fs::File;
use std::io::Write;

extern crate tera;
extern crate base64;
extern crate image_base64;
use base64::{encode};
use tera::{Tera, Context};
use std::process::exit;
use std::ffi::OsStr;

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
    --ignore-first-dir   Don't include the first directory path
    --pn=<pn>            Give a custom name to the rust static file structure 'StaticFiles{PROJECT_NAME}<a'>'
"#;

const TEMPLATE: &'static str = r#"
use std::collections::HashMap;

pub struct StaticFiles{{struct_name}}<'a> {
    pub(crate) map: HashMap<&'a str, &'a str>
}

impl<'a> StaticFiles{{struct_name}}<'a> {
    pub fn new() -> StaticFiles{{struct_name}}<'a> {
        let mut files = StaticFiles {
            map: HashMap::new()
        };
        {% for file in static_files %}
        files.map.insert("{{file.name}}", "{{file.content}}");{% endfor %}

        files
    }
}
"#;

#[derive(Debug, Deserialize)]
struct Args {
    arg_in: String,
    arg_out: String,
    flag_version: bool,
    flag_ignore_first_dir: bool,
    flag_pn: String
}

#[derive(Debug, Serialize)]
struct SFile {
    name: String,
    content: String
}

fn version() -> String {
    format!(
        "cargo-brust64 {}.{}.{}{}",
        option_env!("CARGO_PKG_VERSION_MAJOR").unwrap_or("X"),
        option_env!("CARGO_PKG_VERSION_MINOR").unwrap_or("X"),
        option_env!("CARGO_PKG_VERSION_PATCH").unwrap_or("X"),
        option_env!("CARGO_PKG_VERSION_PRE").unwrap_or("")
    )
}

fn main() {
    if env::var("CARGO").is_err() {
        eprintln!("This binary may only be called via `cargo brust64`.");
        exit(1);
    }

    let mut files: Vec<SFile> = vec![];
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.version(Some(version())).deserialize())
        .unwrap_or_else(|e| e.exit());
    //println!("{:?}", args);

    let mut tera_renderer = Tera::default();
    match tera_renderer.add_raw_template("template", TEMPLATE) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    if Path::new(&args.arg_in).exists() {
        for entry in WalkDir::new(args.arg_in) {
            let entry = entry.unwrap();
            if entry.path().is_file() {
                if !entry.path().ends_with(".DS_Store") {
                    println!("{}", entry.path().display());

                    if entry.path().extension() == Some(OsStr::new("png")) {
                        let encoded_image_content =
                            image_base64::to_base64(&*entry.path().to_string_lossy());
                        let file = SFile {
                            name: entry.path().display().to_string(),
                            content: encoded_image_content
                        };
                        files.push(file);
                    } else {
                        let contents = fs::read_to_string(entry.path())
                            .expect("Something went wrong reading the file");
                        let file = SFile {
                            name: entry.path().display().to_string(),
                            content: encode(contents)
                        };
                        files.push(file);
                    }

                }
            }
        }
    }

    let mut context = Context::new();
    context.insert("static_files", &files);
    context.insert("struct_name", &args.flag_pn);

    let out = tera_renderer.render("template", &context).unwrap();
    let mut output = File::create(args.arg_out).unwrap();
    write!(output, "{}", out).unwrap();
}