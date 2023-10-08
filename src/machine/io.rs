use super::port::Port;

pub struct IO {
    pub R1: Port,
    pub R2: Port,
    pub R3: Port,
   
    W2: Port,
    W3: Port,
    W4: Port,
    W5: Port,
    W6: Port,

    pub shift: u16
}

pub enum Actions {
    Credit,
    P2Start,
    P1Start,
    P1Shot,
    P1Left,
    P1Right,
    Tilt,
    P2Shot,
    P2Left,
    P2Right,
    Write2(u8),
    Write4(u8),
}

impl IO {
    pub fn new() -> Self {
        Self { 
            R1: Port::new(),
            R2: Port::new(),
            R3: Port::new(),
            W2: Port::new(),
            W3: Port::new(),
            W4: Port::new(),
            W5: Port::new(),
            W6: Port::new(),
            shift: 0x0000, 
        }
    }

    pub fn io_reset_input(&mut self) {
        self.R1.reset_bit(1);
        self.R1.reset_bit(2);

        self.R1.reset_bit(4);
        self.R1.reset_bit(5);
        self.R1.reset_bit(6);
        
        self.R2.reset_bit(4);
        self.R2.reset_bit(5);
        self.R2.reset_bit(6);
    }
    
    pub fn io_op(&mut self, action: Actions) {
        match action {
            Actions::Credit => self.R1.set_bit(0),
            Actions::P2Start => self.R1.set_bit(1),
            Actions::P1Start => self.R1.set_bit(2),
            Actions::P1Shot => self.R1.set_bit(4),
            Actions::P1Left => self.R1.set_bit(5),
            Actions::P1Right => self.R1.set_bit(6),
            Actions::Tilt => {},
            Actions::P2Shot => self.R2.set_bit(4),
            Actions::P2Left => self.R2.set_bit(5),
            Actions::P2Right => self.R2.set_bit(6),
            Actions::Write2(n) => {
                let offset = n >> 5;
                self.R3.reg = (self.shift >> (8 - offset)) as u8;
            },
            Actions::Write4(n) => {
                self.shift >>= 8;
                self.shift |= (n as u16) << 8;
            },
        }
    }
}
