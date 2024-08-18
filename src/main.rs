use tokio::time::{Duration, interval};
use chrono::{
    Utc,
    Timelike
};

pub mod camera;
pub mod chlorophyll;
pub mod db;
pub mod photo;
pub mod settings;

#[tokio::main]
async fn main() {
    let settings = settings::Settings::new();
    if !settings.gstreamer_supported {
        println!("WARNING: GStreamer is not supported.");
    }
    
    let mut interval = interval(Duration::from_secs(3600)); // 3600 seconds = 1 hour
    
    loop {
        interval.tick().await;
        let now = Utc::now();
        
        // Water plants at 2pm every day
        if now.hour() == 14 && now.minute() == 0 {
            water_plants().await;
        }
        
        // Take photos at 6am, 12pm, 6pm, and 12am
        if now.hour() == 6 && now.minute() == 0 || 
           now.hour() == 12 && now.minute() == 0 || 
           now.hour() == 18 && now.minute() == 0 || 
           now.hour() == 0 && now.minute() == 0 {
            // camera::take_photo().await;
        }
    }
}

async fn water_plants() {
    println!("Watering plants!");
    // Add your watering logic here (e.g., send a signal to a relay controlling a water pump)
}
