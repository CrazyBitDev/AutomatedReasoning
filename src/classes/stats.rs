use byte_unit::Byte;
use memory_stats::memory_stats;

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

    /// Updates the memory usage
    pub fn update(&mut self) {
        if let Some(usage) = memory_stats() {
            self.virtual_memory = usage.virtual_mem;
            self.physical_memory = usage.physical_mem;
        };
    }

    /// Increases the number of learned clauses
    pub fn increase_learned(&mut self) {
        self.clauses_learned += 1;
    }

    /// Increases the number of forgotten clauses
    /// 
    /// # Arguments
    /// 
    /// * `amount` - The amount to increase
    /// 
    pub fn increase_forgotten(&mut self, amount: usize) {
        self.clauses_forgotten += amount;
    }

    /// Returns the number of learned clauses
    /// 
    /// # Returns
    /// 
    /// * `usize` - The number of learned clauses
    /// 
    pub fn get_clauses_learned(&self) -> usize {
        self.clauses_learned
    }

    /// Returns the number of forgotten clauses
    /// 
    /// # Returns
    /// 
    /// * `usize` - The number of forgotten clauses
    /// 
    pub fn get_clauses_forgotten(&self) -> usize {
        self.clauses_forgotten
    }

    /// Returns the virtual memory usage
    /// 
    /// # Returns
    /// 
    /// * `String` - The virtual memory usage
    /// 
    pub fn get_virtual_memory(&self) -> String {
        //self.virtual_memory
        let byte = Byte::from_u64(self.virtual_memory as u64);
        format!("{byte:#}")
    }

    /// Returns the physical memory usage
    /// 
    /// # Returns
    /// 
    /// * `String` - The physical memory usage
    /// 
    pub fn get_physical_memory(&self) -> String {
        //self.physical_memory
        let byte = Byte::from_u64(self.physical_memory as u64);
        format!("{byte:#}")
    }
}