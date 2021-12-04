use anyhow::Result;
use indicatif::ProgressBar;
use log::{error, info};
use regex::Regex;
use std::error::Error;
use std::process::Command;
use std::str::from_utf8;
use structopt::StructOpt;

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
    let time_regex = Regex::new(r"^\d{2}:\d{2}:\d{2}$").unwrap();
    let args = Cli::from_args();

    let has_valid_times = time_regex.is_match(&args.start) && time_regex.is_match(&args.end);
    match has_valid_times {
        true => (),
        false => return Err("Remember start and end should be in the format HH:mm:ss".into()),
    };

    let video_output = Command::new("youtube-dl")
        .args([
            "-f",
            "bestvideo[ext=mp4]+bestaudio",
            "-o",
            "file",
            &args.url,
        ])
        .output()?;

    if !video_output.status.success() {
        error!(
            "stderr youtube-dl: {}",
            from_utf8(&video_output.stderr).unwrap()
        );
    }

    info!(
        "Downloaded video! {}",
        from_utf8(&video_output.stdout).unwrap()
    );

    let ffmpeg_output = Command::new("ffmpeg")
        .args([
            "-i",
            "file.mp4",
            "-ss",
            &args.start,
            "-to",
            &args.end,
            "cut.mp4",
        ])
        .output()?;

    if !ffmpeg_output.status.success() {
        error!(
            "stderr ffmpeg: {}",
            from_utf8(&ffmpeg_output.stderr).unwrap()
        );
    }

    Ok(())
}
