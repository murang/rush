use std::any::Any;
use rush::app::Component;

pub(crate) struct Nice {
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
    fn init(&mut self) {
        println!("nice init");
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