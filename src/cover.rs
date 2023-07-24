use lofty::{Picture, PictureType};
use std::{fs, path::Path};

pub trait Cover {
    fn from_files(folder: &Path) -> Option<Picture> {
        let allowed_files = ["cover", "front", "album", ".png", ".jpg"];

        for allowed_file in allowed_files {
            for file in fs::read_dir(folder).unwrap().flatten() {
                let filename = file.file_name();
                let filename = filename.to_str().unwrap();

                let extension = Path::new(filename).extension();

                let is_picture = extension.map_or(false, |ext| {
                    ext.eq_ignore_ascii_case("png")
                        || ext.eq_ignore_ascii_case("jpg")
                        || ext.eq_ignore_ascii_case("jpeg")
                });

                if filename.contains(allowed_file) && is_picture {
                    let mut reader = fs::File::open(file.path()).unwrap();

                    if let Ok(mut picture) = Picture::from_reader(&mut reader) {
                        picture.set_pic_type(PictureType::CoverFront);

                        return Some(picture);
                    }
                }
            }
        }

        None
    }
}

impl Cover for Picture {}
