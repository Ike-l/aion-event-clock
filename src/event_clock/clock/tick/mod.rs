#[derive(Default)]
pub struct Tick {
    accumulator: usize
}

impl Tick {
    pub fn increment(&mut self) -> Option<Tick> {
        let old = Tick { accumulator: self.accumulator };
        
        self.accumulator = self.accumulator.checked_add(1)?;

        Some(old)
    }
}