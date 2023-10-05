use super::flag::Flag;


pub struct State<'a> {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
    pub mem: &'a mut [u8],
    pub flags: Flag,
    pub enable: u8,
} 

impl State<'_>{
    pub fn new<'a>(mem: &'a mut Vec<u8>) -> State<'a> {
        State { 
            a: 0, 
            b: 0, 
            c: 0, 
            d: 0, 
            e: 0, 
            h: 0, 
            l: 0, 
            sp: 0xf000, 
            pc: 0, 
            mem, 
            flags: Flag::new(), 
            enable: 0 
        }
    }
}