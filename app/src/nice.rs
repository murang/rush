use std::any::Any;
use std::error::Error;
use rush::app::Component;

pub struct Nice {
    name: String
}

impl Nice {
    pub fn new() -> Self {
        Self {
            name: "niceman".to_string()
        }
    }
}

impl Component for Nice {
    fn init(&mut self) -> Result<(), Box<dyn Error>> {
        println!("nice init");
        Ok(())
    }

    fn run(&mut self) {
        println!("helloe {}", self.name);
    }

    fn on_exit(&mut self) {
        println!("nice on_exit");
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}