use image::RgbImage;
use std::iter::Iterator;

/// Calculate chlorophyll index
/// 
/// 
pub fn calculate_chlorophyll_index(image: &RgbImage) -> f32 {
    let (width, height) = image.dimensions();
    let mut exg_values: Vec<f32> = Vec::new();
    
    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            let (r, g, b) = (pixel[0] as f32, pixel[1] as f32, pixel[2] as f32);
            let exg = 2.0 * g - r - b;
            exg_values.push(exg);
        }
    }
    
    let chl_values: Vec<f32> = exg_values.iter().zip(image.pixels()).map(|(exg, pixel)| {
        let (r, _g, b) = (pixel[0] as f32, pixel[1] as f32, pixel[2] as f32);
        *exg / (r + b)
    }).collect();
    
    let min_chl = *chl_values.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_chl = *chl_values.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    
    let chl_norm = (chl_values.iter().sum::<f32>() / chl_values.len() as f32 - min_chl) / (max_chl - min_chl);
    
    chl_norm
}
