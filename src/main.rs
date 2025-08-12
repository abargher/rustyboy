pub struct CPURegs {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
}

impl CPURegs {
    fn readAF(&self) -> u16 {
        (self.a as u16) << 8 | self.f as u16
    }
    fn readBC(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }
    fn readDE(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }
    fn readHL(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    fn writeAF(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = (value & 0x00FF) as u8;
    }
    fn writeBC(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }
    fn writeDE(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0x00FF) as u8;
    }
    fn writeHL(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0x00FF) as u8;
    }
}

pub struct CPUState {
    num_cycles: u32,
    regs: CPURegs,
    sp: u16,
    pc: u16,
}

impl CPUState {}

fn main() {
    println!("Hello, world!");

    println!("Fetch stage");
    println!("Decode stage");
    println!("Execute stage");
}
