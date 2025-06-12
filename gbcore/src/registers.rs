pub struct RegisterPair {
    pub high: u8,
    pub low: u8,
}

impl RegisterPair {
    pub fn new() -> Self {
        Self {
            high: 0,
            low: 0,
        }
    }

    pub fn get_pair(&self) -> u16 {
        ((self.high as u16) << 8) & (self.low as u16)
    }

    pub fn set_pair(&mut self, value: u16) {
        self.high = ((value & 0xFF00) >> 8) as u8;
        self.low = (value & 0x00FF) as u8;
    }

    pub fn inc_pair(&mut self) {
        let val = self.get_pair();
        self.set_pair(val.wrapping_add(1));
    }

    pub fn dec_pair(&mut self) {
        let val = self.get_pair();
        self.set_pair(val.wrapping_sub(1));
    }

    pub fn inc_high(&mut self) {
        self.high.wrapping_add(1);
    }

    pub fn dec_high(&mut self) {
        self.high.wrapping_sub(1);
    }

    pub fn inc_low(&mut self) {
        self.low.wrapping_add(1);
    }

    pub fn dec_low(&mut self) {
        self.low.wrapping_sub(1);
    }
}