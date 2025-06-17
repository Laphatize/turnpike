use crate::robot::traits::Controllable;
use anyhow::Result;
use opencv::core::Mat;

pub struct Driver<R: Controllable> {
    robot: R,
}

impl<R: Controllable> Driver<R> {
    pub fn new(robot: R) -> Self {
        Self { robot }
    }

    pub fn drive(&mut self, vision_output: &Mat) -> Result<()> {
        // Placeholder for driving logic based on vision output
        self.robot.forward(0.1)?;
        Ok(())
    }
} 