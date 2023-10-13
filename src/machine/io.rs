use super::port::Port;

pub struct IO {
    pub r1: Port,
    pub r2: Port,
    pub r3: Port,
   
    pub w2: Port,
    pub w3: Port,
    pub w4: Port,
    pub w5: Port,
    pub w6: Port,

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
            r1: Port::new(0b10001000),
            r2: Port::new(0b00000001),
            r3: Port::new(0),
            w2: Port::new(0),
            w3: Port::new(0),
            w4: Port::new(0),
            w5: Port::new(0),
            w6: Port::new(0),
            shift: 0x0000, 
        }
    }

    pub fn io_reset_input(&mut self) {
        self.r1.reset_bit(0);
        self.r1.reset_bit(1);
        self.r1.reset_bit(2);

        self.r1.reset_bit(4);
        self.r1.reset_bit(5);
        self.r1.reset_bit(6);
        
        self.r2.reset_bit(4);
        self.r2.reset_bit(5);
        self.r2.reset_bit(6);
    }
    
    pub fn io_op(&mut self, action: Actions) {
        match action {
            Actions::Credit => self.r1.set_bit(0),
            Actions::P2Start => self.r1.set_bit(1),
            Actions::P1Start => self.r1.set_bit(2),
            Actions::P1Shot => self.r1.set_bit(4),
            Actions::P1Left => self.r1.set_bit(5),
            Actions::P1Right => self.r1.set_bit(6),
            Actions::Tilt => {},
            Actions::P2Shot => self.r2.set_bit(4),
            Actions::P2Left => self.r2.set_bit(5),
            Actions::P2Right => self.r2.set_bit(6),
            Actions::Write2(n) => {
                self.w2.reg = n & 0b111;
                let offset = n & 0b111;
                self.r3.reg = (self.shift >> (8 - offset)) as u8;
            },
            Actions::Write4(n) => {
                self.w4.reg = n;
                self.shift >>= 8;
                self.shift |= (n as u16) << 8;
                self.r3.reg = n;
            },
        }
    }

    pub fn machine_in(&mut self, port: u8) -> u8 {
        match port {
            1 => self.r1.reg,
            2 => self.r2.reg,
            3 => {
                let offset = self.w2.reg;
                self.r3.reg = (self.shift >> (8 - offset)) as u8;
                self.r3.reg
            },
            _ => self.r1.reg
        }
    }

    pub fn machine_out(&mut self, port: u8, a: u8) {
        match port {
            2 => self.io_op(Actions::Write2(a)),
            3 => {},
            4 => self.io_op(Actions::Write4(a)),
            5 => {},
            _ => {}
        };
    }
}
