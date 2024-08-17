// use image::{RgbImage, Rgb};
// use num::traits::Float;
// use std::fs;
// use std::io::{Write, BufWriter};
// use chrono::{DateTime, Local};

// /// Take a photo
// /// 
// /// 
// pub async fn take_photo() {
//     // Save image to file
//     let date = Local::now().format("%Y-%m-%d").to_string();
//     let file_name = format!("{} {}.jpg", date, Local::now().format("%H-%M-%S").to_string());
//     let path = format!("images/{}", file_name);
//     image.save(path).unwrap();
    
//     // Save data to CSV file
//     let data = format!("{}, {}, {}\n", date, Local::now().format("%H:%M:%S").to_string(), chlorophyll_index);
//     let mut file = BufWriter::new(fs::OpenOptions::new().append(true).open("data.csv").unwrap());
//     file.write_all(data.as_bytes()).unwrap();
// }
