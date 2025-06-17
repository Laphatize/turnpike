use anyhow::Result;
use opencv::{prelude::*, videoio};

pub struct Camera {
    cam: videoio::VideoCapture,
}

impl Camera {
    pub fn new(index: i32) -> Result<Self> {
        let mut cam = videoio::VideoCapture::new(index, videoio::CAP_ANY)?;
        let opened = videoio::VideoCapture::is_opened(&cam)?;
        if !opened {
            return Err(anyhow::anyhow!("Unable to open camera"));
        }
        Ok(Self { cam })
    }

    pub fn capture_frame(&mut self) -> Result<Mat> {
        let mut frame = Mat::default();
        self.cam.read(&mut frame)?;
        if frame.size()?.width == 0 {
            return Err(anyhow::anyhow!("Unable to capture frame"));
        }
        Ok(frame)
    }
} 