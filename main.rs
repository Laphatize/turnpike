use anyhow::Result;
use delivery_robot_cv::camera::capture::Camera;
use delivery_robot_cv::control::driver::Driver;
use delivery_robot_cv::robot::dummy::DummyRobot;
use delivery_robot_cv::vision::lanes::detect_lanes;
use opencv::highgui;

fn main() -> Result<()> {
    let mut camera = Camera::new(0)?;

    // no robo, so trigger directional functions in robot class
    let robot = DummyRobot;
    let mut driver = Driver::new(robot);

    let window = "video";
    highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;


    // trigger frame for directional CV
    // trigger lane frame for lane detection
    loop {
        let frame = camera.capture_frame()?;
        let lane_frame = detect_lanes(&frame)?;

        driver.drive(&lane_frame)?;

        highgui::imshow(window, &lane_frame)?;
        let key = highgui::wait_key(1)?;
        if key == 113 { // 'q'
            break;
        }
    }

    Ok(())
} 