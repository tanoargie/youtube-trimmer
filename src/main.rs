use structopt::StructOpt;
use std::process::Command;
use std::str::from_utf8;
use anyhow::{Result};
use log::{info, error};

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("Starting");
    let args = Cli::from_args();
    let video_output = Command::new("youtube-dl")
        .arg("-f")
        .arg("bestvideo[ext=mp4]+bestaudio")
        .arg("-o")
        .arg("file")
        .arg(&args.url)
        .output()?;

    if !video_output.status.success() {
        error!("stderr youtube-dl: {}", from_utf8(&video_output.stderr).unwrap());
    }

    info!("Downloaded video! {}", from_utf8(&video_output.stdout).unwrap());

    let ffmpeg_output = Command::new("ffmpeg")
        .arg("-i")
        .arg("file.mp4")
        .arg("-ss")
        .arg(&args.start)
        .arg("-t")
        .arg(&args.end)
        .arg("cut.mp4")
        .output()?;

    if !ffmpeg_output.status.success() {
        error!("stderr ffmpeg: {}", from_utf8(&ffmpeg_output.stderr).unwrap());
    }

    Ok(())
}
