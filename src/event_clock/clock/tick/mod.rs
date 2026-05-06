#[derive(Default, PartialEq, Eq, Hash, PartialOrd, Clone)]
pub struct Tick {
    accumulator: usize
}

impl Tick {
    pub fn increment(&mut self) -> Option<Self> {
        let old = Tick { accumulator: self.accumulator };
        
        *self = self.checked_add(&Self { accumulator: 1 })?;
    
        Some(old)
    }
    
    pub fn checked_add(&self, rhs: &Self) -> Option<Self> {
        self.accumulator.checked_add(rhs.accumulator).map(|tick| Self { accumulator: tick })
    }
}