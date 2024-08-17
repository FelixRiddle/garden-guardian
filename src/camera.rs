use image::RgbImage;
use chrono::{Local, Utc};

/// Define a public struct representing a camera
/// 
/// 
pub struct Camera {
    pub name: String,
    pub model: String,
    pub resolution: (u32, u32), // width, height
    pub image: RgbImage,
}

impl Camera {
    /// Create a new camera instance
    /// 
    /// 
    pub fn new(name: String, model: String, resolution: (u32, u32)) -> Self {
        Self {
            name,
            model,
            resolution,
            image: RgbImage::new(resolution.0, resolution.1),
        }
    }

    /// Take a photo using the camera
    /// 
    /// 
    pub async fn take_photo(&mut self) {
        // TODO: implement device detection and photo taking
    }

    /// Save the photo to a file
    /// 
    /// 
    pub fn save_photo(&self) -> String {
        let local_date = Local::now();
        let utc_date = local_date.with_timezone(&Utc);
        let file_name = format!("{}.jpg", utc_date.format("%Y-%m-%d-%H-%M-%S").to_string());
        let path = format!("images/{}", file_name);
        self.image.save(path.clone()).unwrap();
        path
    }
}
