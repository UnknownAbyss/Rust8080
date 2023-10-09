use super::port::Port;

pub struct IO {
    pub R1: Port,
    pub R2: Port,
    pub R3: Port,
   
    pub W2: Port,
    pub W3: Port,
    pub W4: Port,
    pub W5: Port,
    pub W6: Port,

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
            R1: Port::new(0b10001000),
            R2: Port::new(0b00000001),
            R3: Port::new(0),
            W2: Port::new(0),
            W3: Port::new(0),
            W4: Port::new(0),
            W5: Port::new(0),
            W6: Port::new(0),
            shift: 0x0000, 
        }
    }

    pub fn io_reset_input(&mut self) {
        self.R1.reset_bit(0);
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
                self.W2.reg = n & 0b111;
                let offset = n & 0b111;
                self.R3.reg = (self.shift >> (8 - offset)) as u8;
            },
            Actions::Write4(n) => {
                self.W4.reg = n;
                self.shift >>= 8;
                self.shift |= (n as u16) << 8;
                self.R3.reg = n;
            },
        }
    }

    pub fn machine_in(&mut self, port: u8) -> u8 {
        match port {
            1 => self.R1.reg,
            2 => self.R2.reg,
            3 => {
                let offset = self.W2.reg;
                self.R3.reg = (self.shift >> (8 - offset)) as u8;
                self.R3.reg
            },
            _ => self.R1.reg
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
