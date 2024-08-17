use std::fs;
use std::io::{Write, BufWriter};
use chrono::{DateTime, Utc};
use std::path::PathBuf;

/// Define a trait for saving data to CSV
/// 
/// 
pub trait SaveToCsv {
    fn save_to_csv(&self, path: &str) -> std::io::Result<()>;
}

/// Implement the trait for a struct holding the data
/// 
/// 
pub struct PhotoData {
    pub date: String,
    pub chlorophyll_index: f64,
}

impl SaveToCsv for PhotoData {
    fn save_to_csv(&self, path: &str) -> std::io::Result<()> {
        let data = format!("{}, {}\n", self.date, self.chlorophyll_index);
        let mut file = BufWriter::new(fs::OpenOptions::new().append(true).open(path).unwrap());
        file.write_all(data.as_bytes())
    }
}

/// Define a public struct representing a photo
/// 
/// 
pub struct Photo {
    pub path: PathBuf,
    pub date_taken: DateTime<Utc>,
    pub chlorophyll_index: f64,
}

impl Photo {
    /// Create a new photo instance
    /// 
    /// 
    pub fn new(path: PathBuf, date_taken: DateTime<Utc>, chlorophyll_index: f64) -> Self {
        Self {
            path,
            date_taken,
            chlorophyll_index,
        }
    }

    /// Save photo data to a CSV file
    /// 
    /// 
    pub fn save_to_csv(&self, path: &str) -> std::io::Result<()> {
        let data = format!("{}, {}, {}\n", self.date_taken.format("%Y-%m-%d %H:%M:%S").to_string(), self.chlorophyll_index, self.path.display());
        let mut file = std::fs::OpenOptions::new().append(true).open(path)?;
        std::io::Write::write_all(&mut file, data.as_bytes())?;
        Ok(())
    }
}
