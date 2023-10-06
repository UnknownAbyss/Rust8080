pub struct Flag {
    pub reg: u8,
}

pub enum FlagType {
    Z,
    S,
    P,
    CY,
    AC,
    PAD,
}

impl Flag {
    pub fn new() -> Self {
        Flag { reg: 0 }
    }

    pub fn get(&self, f: FlagType) -> u8 {
        use FlagType::*;
        match f {
            Z => (self.reg >> 7) & 0b1,
            S => (self.reg >> 6) & 0b1,
            P => (self.reg >> 5) & 0b1,
            CY => (self.reg >> 4) & 0b1,
            AC => (self.reg >> 3) & 0b1,
            PAD => (self.reg) & 0b111,
        }
    }

    pub fn set(&mut self, f: FlagType) {
        use FlagType::*;
        match f {
            Z => self.reg |= 1 << 7,
            S => self.reg |= 1 << 6,
            P => self.reg |= 1 << 5,
            CY => self.reg |= 1 << 4,
            AC => self.reg |= 1 << 3,
            PAD => self.reg |= 0b111,
        }
    }

    pub fn unset(&mut self, f: FlagType) {
        use FlagType::*;
        match f {
            Z => self.reg &= !(1 << 7),
            S => self.reg &= !(1 << 6),
            P => self.reg &= !(1 << 5),
            CY => self.reg &= !(1 << 4),
            AC => self.reg &= !(1 << 3),
            PAD => self.reg &= 0b000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use FlagType::*;

    #[test]
    fn z() {
        let mut flag = Flag::new();

        assert_eq!(flag.get(Z), 0);
        assert_eq!(flag.get(S), 0);
        assert_eq!(flag.get(P), 0);
        assert_eq!(flag.get(CY), 0);
        assert_eq!(flag.get(AC), 0);

        flag.set(Z);
        assert_eq!(flag.get(Z), 1);
        assert_eq!(flag.get(S), 0);
        assert_eq!(flag.get(P), 0);
        assert_eq!(flag.get(CY), 0);
        assert_eq!(flag.get(AC), 0);
        flag.unset(Z);

        flag.set(S);
        assert_eq!(flag.get(Z), 0);
        assert_eq!(flag.get(S), 1);
        assert_eq!(flag.get(P), 0);
        assert_eq!(flag.get(CY), 0);
        assert_eq!(flag.get(AC), 0);
        flag.unset(S);

        flag.set(P);
        assert_eq!(flag.get(Z), 0);
        assert_eq!(flag.get(S), 0);
        assert_eq!(flag.get(P), 1);
        assert_eq!(flag.get(CY), 0);
        assert_eq!(flag.get(AC), 0);
        flag.unset(P);

        flag.set(CY);
        assert_eq!(flag.get(Z), 0);
        assert_eq!(flag.get(S), 0);
        assert_eq!(flag.get(P), 0);
        assert_eq!(flag.get(CY), 1);
        assert_eq!(flag.get(AC), 0);
        flag.unset(CY);

        flag.set(AC);
        assert_eq!(flag.get(Z), 0);
        assert_eq!(flag.get(S), 0);
        assert_eq!(flag.get(P), 0);
        assert_eq!(flag.get(CY), 0);
        assert_eq!(flag.get(AC), 1);
        flag.unset(AC);

        assert_eq!(flag.get(Z), 0);
        assert_eq!(flag.get(S), 0);
        assert_eq!(flag.get(P), 0);
        assert_eq!(flag.get(CY), 0);
        assert_eq!(flag.get(AC), 0);
    }
}
