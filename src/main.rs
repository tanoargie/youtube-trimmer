use anyhow::Result;
use indicatif::ProgressBar;
use log::{error, info};
use std::error::Error;
use std::fs;
use std::process::Command;
use structopt::StructOpt;

extern crate youtrmr;

/// Download a trimmed video from Youtube
#[derive(StructOpt)]
struct Cli {
    /// The url of the video to look download (e.g.:https://www.youtube.com/watch?v=dQw4w9WgXcQ)
    url: String,
    /// The start of the video in the format HH:mm:ss (e.g.: 01:15:00)
    start: String,
    /// The end of the video in the format HH:mm:ss (e.g.: 01:20:00)
    end: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    info!("Starting...");
    let args = Cli::from_args();

    for time in vec![&args.start, &args.end] {
        match youtrmr::is_valid_time(time) {
            true => (),
            false => {
                error!("{} should be in the following format: HH:mm:ss", time);
                return Err("Format invalid".into());
            }
        }
    }

    let mut video = Command::new("youtube-dl")
        .args([
            "-q",
            "-f",
            "bestvideo[ext=mp4]+bestaudio",
            "-o",
            "file",
            &args.url,
        ])
        .spawn()
        .unwrap();

    let mut done = false;

    let spinner_youtube = ProgressBar::new_spinner();
    while !done {
        match video.try_wait() {
            Ok(Some(_status)) => {
                done = true;
            }
            Ok(None) => {
                spinner_youtube.set_message("Downloading youtube video...");
            }
            Err(e) => {
                error!("Error trying to download video: {}", e);
                return Err(e.into());
            }
        };
    }
    spinner_youtube.finish_with_message("Done!");

    let mut ffmpeg = Command::new("ffmpeg")
        .args([
            "-loglevel",
            "quiet",
            "-i",
            "file.mp4",
            "-ss",
            &args.start,
            "-to",
            &args.end,
            "cut.mp4",
        ])
        .spawn()
        .unwrap();

    let spinner_ffmpeg = ProgressBar::new_spinner();
    done = false;
    while !done {
        match ffmpeg.try_wait() {
            Ok(Some(_status)) => {
                done = true;
            }
            Ok(None) => {
                spinner_ffmpeg.set_message("Trimming video...");
            }
            Err(e) => {
                error!("Error trying to trim video: {}", e);
                return Err(e.into());
            }
        };
    }

    fs::remove_file("file.mp4")?;

    Ok(())
}
