use clap::Parser;
use cover::Cover;
use indicatif::{ProgressBar, ProgressStyle};
use lofty::{read_from_path, AudioFile, Picture, PictureType, TaggedFileExt};
use std::{fs, path::PathBuf};
use walkdir::WalkDir;

mod cover;
mod panics;

#[derive(Parser)]
struct Args {
    pub folder: PathBuf,

    #[arg(long)]
    pub pretend: bool,

    #[arg(long)]
    pub resize: Option<u32>,
}

fn main() {
    let args = Args::parse();
    panics::check_folder(&args.folder);

    let folders: Vec<PathBuf> = get_directories(args.folder);

    let bar = get_progress_bar(folders.len() as u64);

    for folder in folders {
        let mut cover: Option<Picture> = Picture::from_files(&folder, args.resize);

        for path in fs::read_dir(folder).unwrap().flatten().map(|x| x.path()) {
            if let Ok(mut audio) = read_from_path(&path) {
                bar.set_message(path.display().to_string());

                let tag = audio.primary_tag_mut().expect("Failed to read audio tag");

                if tag.picture_count() != 0 {
                    if cover.is_none() {
                        if let Some(int_cover) = tag.get_picture_type(PictureType::CoverFront) {
                            let mut int_cover = int_cover.clone();

                            if let Some(size) = args.resize {
                                int_cover.resize(size, size);
                            }

                            cover = Some(int_cover);
                        }
                    }

                    continue;
                }

                if let Some(ref cover) = cover {
                    tag.push_picture(cover.clone());
                } else {
                    continue;
                }

                if args.pretend {
                    continue;
                }

                if let Err(err) = audio.save_to_path(&path) {
                    eprintln!("Failed to save audio: {err}");
                }
            }
        }

        bar.inc(1);
    }

    bar.finish_with_message("Done!");
}

fn get_progress_bar(len: u64) -> ProgressBar {
    let style = ProgressStyle::with_template("[{pos}/{len}] {wide_msg}").unwrap();

    let bar = ProgressBar::new(len);
    bar.set_style(style);
    bar.tick();

    bar
}

fn get_directories(folder: PathBuf) -> Vec<PathBuf> {
    WalkDir::new(folder)
        .into_iter()
        .flatten()
        .map(|x| x.path().to_path_buf())
        .filter(|x| x.is_dir())
        .collect()
}
