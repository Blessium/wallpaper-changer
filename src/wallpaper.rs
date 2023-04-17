use eframe::{egui, egui::ColorImage, egui::ImageButton, egui::Sense};
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

    pub fn import_wallpapers(&mut self, file_paths: Vec<PathBuf>) -> Result<(), ()> {
        for file_path in file_paths {
            if !self.known_wallpapers.contains_key(&file_path) {
                let wallpaper = self.load_image(file_path.clone()).unwrap();
                self.known_wallpapers.insert(file_path, wallpaper);
            }
        }
        Ok(())
    }

    pub fn load_wallpapers(&self) -> Option<Vec<Wallpaper>> {
        if self.known_wallpapers.len() == 0 {
            return None;
        }

        Some(
            self.known_wallpapers
                .clone()
                .into_iter()
                .map(|(_, value)| value)
                .collect::<Vec<Wallpaper>>(),
        )
    }

    fn load_image(&mut self, mut file_path: PathBuf) -> Result<Wallpaper, ()> {
        let image = ImageReader::open(&file_path)
            .expect("Could not open the file")
            .decode()
            .expect("Could not decode the file");
        let image = image.resize(192, 108, image::imageops::FilterType::Triangle);
        let size = [image.width() as usize, image.height() as usize];
        let image_buffer = image.to_rgb8();
        let pixels = image_buffer.as_flat_samples();
        let image_data = egui::ColorImage::from_rgb(size, pixels.as_slice());
        Ok(Wallpaper::new(
            file_path.pop().to_string(),
            Size(size[0], size[1]),
            image_data,
            file_path.to_str().unwrap(),
        ))
    }
}

#[derive(Clone)]
pub struct Size(usize, usize);

#[derive(Clone)]
pub struct Wallpaper {
    pub file_name: String,
    pub dimension: Size,
    pub file_path: String,
    pub image_data: ColorImage,
    texture: Option<egui::TextureHandle>,
    image_button: Option<ImageButton>,
}

impl Wallpaper {
    pub fn new(
        file_name: String,
        dimension: Size,
        image_data: ColorImage,
        file_path: &str,
    ) -> Wallpaper {
        Self {
            file_name,
            dimension,
            image_data,
            texture: None,
            file_path: file_path.to_string(),
            image_button: None,
        }
    }

    pub fn check_clicked(&mut self) {}

    pub fn display(&mut self, ui: &mut egui::Ui) {
        if let Some(button) = &self.image_button {
            ui.add(button.clone());
        } else {
            let texture: &egui::TextureHandle = self.texture.get_or_insert_with(|| {
                ui.ctx()
                    .load_texture(&self.file_name, self.image_data.clone(), Default::default())
            });

            let button = ImageButton::new(texture, texture.size_vec2());
            ui.add(button);
        }
    }
}
