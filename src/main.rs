use anyhow::Result;
use indicatif::ProgressBar;
use log::{error, info};
use std::error::Error;
use std::process::Command;
use std::str;
use structopt::StructOpt;

extern crate youtrmr;

use youtrmr::is_valid_time;

/// Download a trimmed video from Youtube
#[derive(StructOpt)]
struct Cli {
    /// The url of the video to look download (e.g.:https://www.youtube.com/watch?v=dQw4w9WgXcQ)
    url: String,
    /// The start of the video in the format HH:mm:ss (e.g.: 01:15:00)
    start: String,
    /// The duration of the cut in the format HH:mm:ss (e.g.: 01:20:00)
    duration: String,
    /// Output filename
    filename: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    info!("Starting...");
    let args = Cli::from_args();

    for time in vec![&args.start, &args.duration] {
        match is_valid_time(time) {
            true => (),
            false => {
                error!("{} should be in the following format: HH:mm:ss", time);
                return Err("Invalid format".into());
            }
        }
    }

    let youtube_dl_command = Command::new("youtube-dl")
        .args(["-g", &args.url])
        .output()?;

    let youtube_url = str::from_utf8(&youtube_dl_command.stdout).unwrap();

    let mut urls: Vec<&str> = vec![];
    for line in youtube_url.lines() {
        urls.push(line);
    }
    let filename_with_format = format!("{}.mp4", &args.filename);

    let ffmpeg = &mut Command::new("ffmpeg")
        .args([
            "-loglevel",
            "quiet",
            "-ss",
            &args.start,
            "-i",
            urls[0],
            "-ss",
            &args.start,
            "-i",
            urls[1],
            "-t",
            &args.duration,
            "-map",
            "0:v",
            "-map",
            "1:a",
            &filename_with_format,
        ])
        .spawn()
        .unwrap();

    let mut done = false;
    let spinner = ProgressBar::new_spinner();
    spinner.set_message("Trimming video...");
    while !done {
        match ffmpeg.try_wait() {
            Ok(Some(_status)) => {
                done = true;
            }
            Ok(None) => {
                spinner.tick();
            }
            Err(e) => {
                error!("{}: {}", "Error trimming video", e);
                return Err(e.into());
            }
        };
    }
    spinner.finish_with_message("Done trimming!");

    Ok(())
}
