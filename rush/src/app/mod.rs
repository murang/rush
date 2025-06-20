use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use signal_hook::consts::signal::*;
use signal_hook::flag::register;

pub struct App {
    exiting: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
    on_exit: Option<Box<dyn FnMut() + Send + 'static>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            exiting: Arc::new(AtomicBool::new(false)),
            handle: None,
            on_exit: None
        }
    }

    pub fn on_exit<F>(&mut self, on_exit : F) where F: FnMut() + Send + 'static {
        self.on_exit = Some(Box::new(on_exit));
    }
    
    pub fn run<F>(&mut self, fps : u32, mut tick : F) where F: FnMut() + Send + 'static{
        let exiting = Arc::clone(&self.exiting);
        exiting.store(false, Ordering::SeqCst);
        
        let tick_duration = Duration::from_secs_f64(1.0 / fps as f64);

        self.handle = Some(thread::spawn(move || {
            while !exiting.load(Ordering::SeqCst) {
                let start = Instant::now();

                // 执行 tick 逻辑
                tick();

                // 睡眠直到下一次 tick
                let elapsed = start.elapsed();
                if elapsed < tick_duration {
                    thread::sleep(tick_duration - elapsed);
                }
            }
        }));

        // 设置终止标志
        let term = Arc::new(AtomicBool::new(false));
        let term_ref = term.clone();
        // 注册 SIGINT (Ctrl+C) 和 SIGTERM (kill)
        register(SIGINT, term_ref.clone()).expect("app terminal register SIGINT failed");
        register(SIGTERM, term_ref).expect("app terminal register SIGTERM failed");
        // 阻塞等待
        while !term.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_millis(100));
        }

        println!("got signal, begin exit");
        self.exiting.store(true, Ordering::SeqCst);
        // 执行关闭回调
        if let Some(mut on_exit) = self.on_exit.take() {
            on_exit();
        }
        thread::sleep(Duration::from_secs(1));
        println!("app exited, see ya~");
    }

    pub fn shutdown(&mut self) {
        self.exiting.store(true, Ordering::SeqCst);

        if let Some(handle) = self.handle.take() {
            let _ = handle.join(); // 等待线程退出
        }
    }

    pub fn is_running(&self) -> bool {
        self.exiting.load(Ordering::SeqCst)
    }
}