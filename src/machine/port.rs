pub struct Port {
    pub reg: u8,
}

impl Port {
    pub fn new() -> Self {
        Port { reg: 0xff }
    }

    pub fn bit(&self, n: u8) -> u8 {
        if n > 7 { return 0 }
        self.reg >> n & 0b1
    }

    pub fn set_bit(&mut self, n: u8) {
        if n > 7 { return }
        self.reg |= 1 << n
    }
    
    pub fn reset_bit(&mut self, n: u8) {
        if n > 7 { return }
        self.reg &= !(1 << n)
    }
}
