use image::{imageops::FilterType, ImageOutputFormat};
use lofty::{Picture, PictureType};
use std::{fs, io::Cursor, path::Path};

pub trait Cover {
    fn from_files(folder: &Path, resize: Option<u32>) -> Option<Picture> {
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

                        if let Some(size) = resize {
                            picture.resize(size, size);
                        }

                        return Some(picture);
                    }
                }
            }
        }

        None
    }

    fn resize(&mut self, width: u32, height: u32);
}

impl Cover for Picture {
    fn resize(&mut self, width: u32, height: u32) {
        let image = image::load_from_memory(self.data()).unwrap();
        image.resize(width, height, FilterType::Lanczos3);

        let mut target = Cursor::new(vec![]);

        image.write_to(&mut target, ImageOutputFormat::Png).unwrap();

        target.set_position(0);

        let mut picture = Self::from_reader(&mut target).unwrap();
        picture.set_pic_type(PictureType::CoverFront);

        *self = picture;
    }
}
