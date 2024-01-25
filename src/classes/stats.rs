use memory_stats::memory_stats;
use std::thread;

pub struct Stats {
    memory: usize,
    
    clauses_learned: usize,
    clauses_forgotten: usize,

    virtual_memory: usize,
    physical_memory: usize,

    thread_is_running: bool,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            memory: 0,
            
            clauses_learned: 0,
            clauses_forgotten: 0,

            virtual_memory: 0,
            physical_memory: 0,

            thread_is_running: false,
            stop_thread: false
        }
    }

    pub fn start(&mut self) {
        self.thread_is_running = true;
        self.stop_thread = false;
        thread::spawn(move || {
            loop {

                if let Some(usage) = memory_stats() {
                    self.virtual_memory = max(self.virtual_memory, usage.virtual_mem);
                    self.physical_memory = max(self.physical_memory, usage.physical_mem);
                }

                thread::sleep(Duration::from_millis(500));

                if self.stop_thread {
                    self.thread_is_running = false;
                    break;
                }
            }
        })
    }

    pub fn stop(&mut self) {
        self.stop_thread = true;
        loop {
            if !self.thread_is_running {
                break;
            }
        }
    }
}