extern crate walkdir;
extern crate regex;
extern crate subprocess;

use walkdir::WalkDir;
use regex::Regex;
use std::process::Command;
use std::env;

fn main() {
    // Check if user provided directory path
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please provide a directory path.");
        return;
    }

    let folder = &args[1]; // User-provided directory
    println!("Converting files in {}...", folder);
    let re = Regex::new(r"(?i)\.(mp4|mkv|flv|avi|webm)$").unwrap();

    for entry in WalkDir::new(folder) {
        let entry = entry.unwrap();
        let path = entry.path();
        let filename = path.to_str().unwrap();
        // Remove file extension
        let stripped_filename = filename.split(".").next().unwrap();

        if re.is_match(filename) {
            let output_filename = format!("{}.mp3", stripped_filename);

            let output = Command::new("ffmpeg")
                .arg("-i")
                .arg(filename)
                .arg("-vn")
                .arg("-acodec")
                .arg("libmp3lame")
                .arg("-y")
                .arg(output_filename)
                .output()
                .expect("Failed to execute command");
            println!("Converted {} to {}.mp3", filename, stripped_filename);
            if !output.status.success() {
                eprintln!("Error: {:?}", String::from_utf8_lossy(&output.stderr));
            }
        }
    }
println!("Done!")
}
