use memory_stats::memory_stats;
use byte_unit::Byte;

pub struct Stats {
    clauses_learned: usize,
    clauses_forgotten: usize,

    virtual_memory: usize,
    physical_memory: usize,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            clauses_learned: 0,
            clauses_forgotten: 0,

            virtual_memory: 0,
            physical_memory: 0,
        }
    }

    pub fn update(&mut self) {
        if let Some(usage) = memory_stats() {
            self.virtual_memory = usage.virtual_mem;
            self.physical_memory = usage.physical_mem;
        };
    }

    pub fn icrease_learned(&mut self) {
        self.clauses_learned += 1;
    }
    pub fn icrease_forgotten(&mut self, amount: usize) {
        self.clauses_forgotten += amount;
    }

    pub fn get_clauses_learned(&self) -> usize {
        self.clauses_learned
    }
    pub fn get_clauses_forgotten(&self) -> usize {
        self.clauses_forgotten
    }
    pub fn get_virtual_memory(&self) -> String {
        //self.virtual_memory
        let byte = Byte::from_u64(self.virtual_memory as u64);
        format!("{byte:#}")
    }
    pub fn get_physical_memory(&self) -> String {
        //self.physical_memory
        let byte = Byte::from_u64(self.physical_memory as u64);
        format!("{byte:#}")
    }
}