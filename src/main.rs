use anyhow::Result;
use log::{error, info};
use std::error::Error;
use std::fs;
use std::process::Command;
use structopt::StructOpt;

extern crate youtrmr;

use youtrmr::{is_valid_time, pooling_command};

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
        match is_valid_time(time) {
            true => (),
            false => {
                error!("{} should be in the following format: HH:mm:ss", time);
                return Err("Invalid format".into());
            }
        }
    }

    let video = &mut Command::new("youtube-dl")
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

    pooling_command(
        String::from("Done downloading video!"),
        String::from("Downloading youtube video..."),
        String::from("Error trying to download video"),
        video,
    )?;

    let ffmpeg = &mut Command::new("ffmpeg")
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

    pooling_command(
        String::from("Done trimming!"),
        String::from("Trimming video..."),
        String::from("Error trimming video"),
        ffmpeg,
    )?;

    fs::remove_file("file.mp4")?;

    Ok(())
}
