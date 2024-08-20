use std::path::Path;

use colored::Colorize;
use subprocess::{Popen, PopenConfig};

fn visit_dir(dir: &Path) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let entry = entry.expect("failed to read directory entry");
        let path = entry.path();

        if which::which("ffmpeg").is_err() {
            println!("ffmpeg is not installed");
            return;
        }

        if path.is_dir() {
            visit_dir(&path);
        } else {
            // convert the file to .wav with 41000 sample rate
            if path.extension().is_none() {
                continue;
            }

            if path.extension().unwrap() == "ogg" || path.extension().unwrap() == "mp3" {
                // change the file's extension from whatever it was to WAV
                let new = path.with_extension("").with_extension("wav");

                println!("{}: converting {} to {}", "wavconv".green(), path.to_str().unwrap(), new.to_str().unwrap());

                // open a new process in this thread
                let mut p = Popen::create(&["ffmpeg", "-i", path.to_str().unwrap(), "-ar", "41000", "-y", new.to_str().unwrap()], PopenConfig {
                    stdout: subprocess::Redirection::Pipe, ..Default::default()
                }).expect("failed to create ffmpeg subprocess");

                let (_, _) = p.communicate(None).expect("failed to communicate with ffmpeg");

                // wait for it to finish
                if p.poll().is_some() {
                } else {
                    p.terminate().expect("failed to terminate ffmpeg");
                }
            }
        }
    }
}

fn main() {
    println!("{}: starting conversion in current directory {}", "wavconv".red(), std::env::current_dir().unwrap().to_str().unwrap());
    let path = std::env::current_dir().unwrap();

    visit_dir(&path);
}
