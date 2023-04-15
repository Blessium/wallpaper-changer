use eframe::{egui, egui::ColorImage};
use image::io::Reader as ImageReader;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Default)]
pub struct WallpaperManager {
    known_wallpapers: HashMap<PathBuf, Wallpaper>,
}

impl WallpaperManager {
    pub fn new() -> WallpaperManager {
        Self {
            known_wallpapers: HashMap::new(),
        }
    }

    pub fn import_files(&mut self, file_paths: Vec<PathBuf>) -> Result<(), ()> {
        for file_path in file_paths {
            if !self.known_wallpapers.contains_key(&file_path) {
                let wallpaper = self.load_image(file_path.clone()).unwrap();
                self.known_wallpapers.insert(file_path, wallpaper);
            }
        }
        Ok(())
    }

    fn load_image(&mut self, mut file_path: PathBuf) -> Result<Wallpaper, ()> {
        let image = ImageReader::open(&file_path)
            .expect("Could not open the file")
            .decode()
            .expect("Could not decode the file");
        let size = [image.width() as usize, image.height() as usize];
        let image_buffer = image.to_rgb8();
        let pixels = image_buffer.as_flat_samples();
        let image_data = egui::ColorImage::from_rgb(size, pixels.as_slice());
        Ok(Wallpaper::new(
            file_path.pop().to_string(),
            Size(size[0], size[1]),
            image_data,
        ))
    }
}

pub struct Size(usize, usize);

pub struct Wallpaper {
    pub file_name: String,
    pub dimension: Size,
    pub image_data: ColorImage,
}

impl Wallpaper {
    pub fn new(file_name: String, dimension: Size, image_data: ColorImage) -> Wallpaper {
        Self {
            file_name,
            dimension,
            image_data,
        }
    }
}
