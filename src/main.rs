pub mod camera;
pub mod control;
pub mod robot;
pub mod vision;

use anyhow::Result;
use camera::capture::Camera;
use control::driver::Driver;
use opencv::highgui;
use robot::dummy::DummyRobot;
use vision::lanes::detect_lanes;

fn main() -> Result<()> {
    let mut camera = Camera::new(0)?;
    let robot = DummyRobot;
    let mut driver = Driver::new(robot);

    let window = "video";
    highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;

    loop {
        let frame = camera.capture_frame()?;
        let lane_frame = detect_lanes(&frame)?;

        driver.drive(&lane_frame)?;

        highgui::imshow(window, &lane_frame)?;
        let key = highgui::wait_key(1)?;
        if key == 113 {
            // 'q'
            break;
        }
    }

    Ok(())
} 