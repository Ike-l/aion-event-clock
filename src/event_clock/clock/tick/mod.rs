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

    pub fn checked_sub(&self, rhs: &Self) -> Option<Self> {
        self.accumulator.checked_sub(rhs.accumulator).map(|tick| Self { accumulator: tick })
    }

    pub fn rem_euclid(&self, rhs: &Self) -> Option<Self> {
        if rhs.accumulator == 0 {
            return None;
        }
        
        Some(Self { accumulator: self.accumulator.rem_euclid(rhs.accumulator) })
    }
}