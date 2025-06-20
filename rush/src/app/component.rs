use std::any::Any;
use std::error::Error;

pub trait Component : Any {
    fn init(&mut self) -> Result<(), Box<dyn Error>>;
    fn run(&mut self);
    fn on_exit(&mut self);
    fn as_any(&mut self) -> &mut dyn Any;
} 