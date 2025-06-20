use std::any::Any;

pub trait Component : Any {
    fn init(&mut self);
    fn run(&mut self);
    fn on_exit(&mut self);
    fn as_any(&mut self) -> &mut dyn Any;
} 