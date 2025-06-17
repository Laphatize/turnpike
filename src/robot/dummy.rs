use crate::robot::traits::Controllable;
use anyhow::Result;

pub struct DummyRobot;

impl Controllable for DummyRobot {
    fn forward(&mut self, speed: f64) -> Result<()> {
        println!("Moving forward with speed: {}", speed);
        Ok(())
    }

    fn backward(&mut self, speed: f64) -> Result<()> {
        println!("Moving backward with speed: {}", speed);
        Ok(())
    }

    fn turn_left(&mut self, speed: f64) -> Result<()> {
        println!("Turning left with speed: {}", speed);
        Ok(())
    }

    fn turn_right(&mut self, speed: f64) -> Result<()> {
        println!("Turning right with speed: {}", speed);
        Ok(())
    }

    fn stop(&mut self) -> Result<()> {
        println!("Stopping");
        Ok(())
    }
} 