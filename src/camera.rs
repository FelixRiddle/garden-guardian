use gstreamer::prelude::*;
use gstreamer::{ElementFactory, Pipeline};
use image::RgbImage;
use std::error::Error;
use chrono::{Local, Utc};

/// Represents a camera.
pub struct Camera {
    /// The name of the camera.
    pub name: String,
    /// The model of the camera.
    pub model: String,
    /// The resolution of the camera.
    pub resolution: (u32, u32), // width, height
    /// The captured image.
    pub image: RgbImage,
    /// The GStreamer pipeline.
    pub pipeline: Pipeline,
}

impl Camera {
    /// Creates a new camera instance.
    ///
    /// # Arguments
    ///
    /// * `name`: The name of the camera.
    /// * `model`: The model of the camera.
    /// * `resolution`: The resolution of the camera.
    ///
    /// # Returns
    ///
    /// A new `Camera` instance.
    pub fn new(name: String, model: String, resolution: (u32, u32)) -> Result<Self, Box<dyn Error>> {
        // Create a new pipeline
        let pipeline = Pipeline::new();
        
        // Create elements
        let source = ElementFactory::make("v4l2src").build()?;
        let converter = ElementFactory::make("videoconvert").build()?;
        let sink = ElementFactory::make("appsink").build()?;
        
        // Set properties
        source.set_property("device", "/dev/video0");
        sink.set_property("emit-signal", true);
        sink.set_property("signal-handoff", true);
        
        // Add elements to pipeline
        pipeline.add(&source)?;
        pipeline.add(&converter)?;
        pipeline.add(&sink)?;
        
        // Link elements
        source.link(&converter)?;
        converter.link(&sink)?;
        
        // Set up signal handler for appsink
        let sink = sink.clone();
        sink.connect("handoff", false, move |buffer| {
            let buffer = match buffer[0].get::<gstreamer::Buffer>() {
                Ok(buffer) => buffer,
                Err(_) => return None,
            };
            let _image = match buffer.map_readable() {
                Ok(map) => map,
                Err(_) => return None,
            };
            
            // TODO: Convert buffer to RgbImage (not implemented yet)
            // self.image = ...
            
            None
        });
        
        // Start pipeline
        pipeline.set_state(gstreamer::State::Playing)?;
        
        Ok(Self {
            name,
            model,
            resolution,
            image: RgbImage::new(resolution.0, resolution.1),
            pipeline,
        })
    }
    
    /// Takes a photo using the camera.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the photo was taken successfully.
    /// 
    /// TODO: Return a Photo instance
    pub fn take_photo(&mut self) -> Result<bool, Box<dyn Error>> {
        // Send an EOS (End Of Stream) event to the pipeline to capture a single frame
        self.pipeline.send_event(gstreamer::event::Eos::new());

        // Wait for the frame to be captured
        let bus = match self.pipeline.bus() {
            Some(bus) => bus,
            None => return Err(String::from("Failed to get bus from pipeline").into())
        };
        let msg = bus.timed_pop_filtered(gstreamer::ClockTime::NONE, &[gstreamer::MessageType::Error, gstreamer::MessageType::Eos]);
        
        let result = match msg {
            Some(msg) => {
                if let gstreamer::MessageView::Error(err) = msg.view() {
                    eprintln!("Error capturing photo: {}", err.error());
                    return Err(format!("Error capturing photo: {}", err).into())
                } else if let gstreamer::MessageView::Eos(_) = msg.view() {
                    true
                } else {
                    return Err(format!("Error capturing").into());
                }
            }
            None => return Err(format!("No message received").into())
        };
        
        Ok(result)
    }
    
    /// Saves the photo to a file.
    ///
    /// # Returns
    ///
    /// The path to the saved photo file.
    pub fn save_photo(&self) -> String {
        let local_date = Local::now();
        let utc_date = local_date.with_timezone(&Utc);
        let file_name = format!("{}.jpg", utc_date.format("%Y-%m-%d-%H-%M-%S").to_string());
        let path = format!("images/{}", file_name);
        self.image.save(path.clone()).unwrap();
        path
    }
}
