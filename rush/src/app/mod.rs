mod component;

use std::any::TypeId;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use signal_hook::consts::signal::*;
use signal_hook::flag::register;
pub use component::Component;

pub struct App {
    components: HashMap<TypeId, Box<dyn Component>>,
    on_exit: Option<Box<dyn Fn()>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            on_exit: None
        }
    }
    
    pub fn add_component<T: Component + 'static>(&mut self, component: T) {
        let type_id = TypeId::of::<T>();
        self.components.insert(type_id, Box::new(component));
    }
    
    pub fn on_exit<F>(&mut self, on_exit : F) where F: Fn() + 'static {
        self.on_exit = Some(Box::new(on_exit));
    }
    
    pub fn start_run(&mut self, fps : u32) {
        let tick_duration = Duration::from_secs_f64(1.0 / fps as f64);
        
        // 设置终止标志
        let term = Arc::new(AtomicBool::new(false));
        let term_ref = term.clone();
        // 注册 SIGINT (Ctrl+C) 和 SIGTERM (kill)
        register(SIGINT, term_ref.clone()).expect("app terminal register SIGINT failed");
        register(SIGTERM, term_ref).expect("app terminal register SIGTERM failed");
        // 没关闭 就去执行逻辑
        while !term.load(Ordering::Relaxed) {
            let start = Instant::now();
            // 执行组件逻辑
            for (_, comp) in &mut self.components {
                comp.run();
            }
            // 睡眠直到下一次 tick
            let elapsed = start.elapsed();
            if elapsed < tick_duration {
                thread::sleep(tick_duration - elapsed);
            }
        }
        
        println!("got term signal, begin exit");
        // 执行关闭回调
        if let Some(on_exit) = self.on_exit.take() {
            on_exit();
        }
        println!("app exited, see ya~");
    }
}