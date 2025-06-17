use anyhow::Result;

pub trait Controllable {
    fn forward(&mut self, speed: f64) -> Result<()>;
    fn backward(&mut self, speed: f64) -> Result<()>;
    fn turn_left(&mut self, speed: f64) -> Result<()>;
    fn turn_right(&mut self, speed: f64) -> Result<()>;
    fn stop(&mut self) -> Result<()>;
} 