extern crate walkdir;
extern crate regex;
extern crate subprocess;

use walkdir::WalkDir;
use regex::Regex;
use std::process::Command;
use std::env;

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Display help information if requested
    if args.len() == 2 && args[1] == "-h" {
        print_help();
        return;
    }

    // Ensure that a directory path is provided
    if args.len() != 2 {
        eprintln!("Error: Please provide a directory path.\n");
        print_help();
        return;
    }

    // Proceed with the conversion operation
    convert_files(&args[1]);
}

/// Prints a help message to the console
fn print_help() {
    println!("meven - Video to Audio converter\n");
    println!("Usage:");
    println!("  meven [directory]\n");
    println!("Options:");
    println!("  -h, --help     Display this help message\n");
    println!("Example:");
    println!("  meven C:\\Users\\perle\\Videos\\Youtube");
}

/// Converts all video files in the specified directory to MP3 format
fn convert_files(folder: &str) {
    println!("Converting files in {}...", folder);

    // Regex to match file extensions of video files
    let re = Regex::new(r"(?i)\.(mp4|mkv|flv|avi|webm)$").unwrap();

    // Walk through each file in the directory
    for entry in WalkDir::new(folder) {
        let entry = entry.unwrap();
        let path = entry.path();
        let filename = path.to_str().unwrap();
        // Remove file extension
        let stripped_filename = filename.split(".").next().unwrap();

        // If the file is a video file, convert it to MP3
        if re.is_match(filename) {
            let output_filename = format!("{}.mp3", stripped_filename);

            // Run the ffmpeg command to convert the video file to MP3
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

            // If the conversion operation failed, print an error message
            if !output.status.success() {
                eprintln!("Error: {:?}", String::from_utf8_lossy(&output.stderr));
            }
        }
    }

    println!("Done!");
}

