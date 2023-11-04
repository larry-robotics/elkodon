use core::{sync::atomic::AtomicBool, time::Duration};
use std::{
    sync::{atomic::Ordering, Arc},
    thread,
    time::Instant,
};

pub struct Watchdog {
    termination_thread: Option<thread::JoinHandle<()>>,
    keep_running: Arc<AtomicBool>,
}

impl Drop for Watchdog {
    fn drop(&mut self) {
        self.keep_running.store(false, Ordering::Relaxed);
        let handle = self.termination_thread.take();
        handle.unwrap().join().unwrap();
    }
}

impl Watchdog {
    pub fn new(timeout: Duration) -> Self {
        let keep_running = Arc::new(AtomicBool::new(true));

        Self {
            keep_running: keep_running.clone(),
            termination_thread: Some(thread::spawn(move || {
                let now = Instant::now();
                while keep_running.load(Ordering::Relaxed) {
                    std::thread::yield_now();
                    std::thread::sleep(Duration::from_millis(10));
                    std::thread::yield_now();

                    if now.elapsed() > timeout {
                        eprintln!("Killing test since timeout of {:?} was hit.", timeout);
                        std::process::exit(1);
                    }
                }
            })),
        }
    }
}
