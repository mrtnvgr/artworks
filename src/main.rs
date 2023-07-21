use cover::Cover;
use indicatif::{ProgressBar, ProgressStyle};
use lofty::{read_from_path, AudioFile, Picture, PictureType, TaggedFileExt};
use std::{fs, path::PathBuf};
use walkdir::WalkDir;

mod cover;
mod panics;

fn main() {
    let folder = panics::get_folder();
    panics::check_folder(&folder);

    let folders: Vec<PathBuf> = get_directories(folder);

    let bar = get_progress_bar(folders.len() as u64);

    for folder in folders {
        let mut cover: Option<Picture> = Picture::from_files(&folder);
        cover::force_type(&mut cover);

        for path in fs::read_dir(folder).unwrap().flatten().map(|x| x.path()) {
            if let Ok(mut audio) = read_from_path(&path) {
                bar.set_message(path.display().to_string());

                let tag = audio.primary_tag_mut().expect("Failed to read audio tag");

                if tag.picture_count() != 0 {
                    if cover.is_none() {
                        let internal_cover = tag.get_picture_type(PictureType::CoverFront);
                        cover = internal_cover.cloned();
                    }

                    continue;
                }

                if let Some(ref cover) = cover {
                    tag.push_picture(cover.clone());
                } else {
                    continue;
                }

                if let Err(err) = audio.save_to_path(&path) {
                    eprintln!("\nFailed to save audio: {}", path.display());
                    eprintln!("Error: {err}\n");
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
