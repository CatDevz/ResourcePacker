mod zip_utils;

use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

use walkdir::WalkDir;

use zip_utils::zip_dir;

use serde_json::Value;

fn main() {
    // Creating our output directory
    let out_path = Path::new("out/");
    if out_path.exists() {
        fs::remove_dir_all(out_path).unwrap();
    }
    fs::create_dir_all(&out_path).unwrap();

    // Walking our input directory
    for entry in WalkDir::new("pack") {
        let entry = entry.unwrap();
        let path: &Path = entry.path();
        let new_file_path = out_path.join(path);
        
        if path.is_file() {
            match path.extension().unwrap().to_str().unwrap() {
                "json" | "mcmeta" => {
                    let mut ifile = File::open(path).unwrap();
                    let mut ofile = File::create(new_file_path).unwrap();
        
                    let mut contents = String::new();
                    ifile.read_to_string(&mut contents).unwrap();

                    let json_obj: Value = match serde_json::from_str(&contents) {
                        Ok(v) => v,
                        Err(_) => {
                            println!("Resource pack compilation failed. File '{}' contains invalid JSON.", path.to_str().unwrap());
                            std::process::exit(1);
                        },
                    };
                    
                    ofile.write_all(json_obj.to_string().as_bytes()).unwrap();
                },
                _ => {
                    fs::copy(&path, &new_file_path).unwrap();
                },
            }
        } else if path.is_dir() {
            fs::create_dir_all(&new_file_path).unwrap();
            continue;
        }
    }

    // Compressing the pack to pack.zip
    let srcpath = out_path.join("pack");
    let srcdir = srcpath.to_str().unwrap();

    let walkdir = WalkDir::new(&srcpath);
    let outfile = File::create(&out_path.join("pack.zip")).unwrap();

    zip_dir(&mut walkdir.into_iter().filter_map(|e| e.ok()), srcdir, outfile, zip::CompressionMethod::Deflated).unwrap();
}
