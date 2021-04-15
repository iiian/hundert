use std::{mem, sync::mpsc::sync_channel};
use crate::port::Port;

pub struct Core<'i> {
    name: &'i str,
    acc: i16,
    bak: i16,
    left: Port,
    right: Port,
    up: Port,
    down: Port,

    p_ctr: usize,
    instrs: &'i [Instr],
}

pub enum Src {
    Literal(i16),
    Acc,
    Up,
    Down,
    Left,
    Right,
}

pub enum Dest {
    Acc,
    Up,
    Down,
    Left,
    Right,
}

pub enum Instr {
    // Control-flow instructions
    Jez(usize),
    Jnz(usize),
    Jgz(usize),
    Jlz(usize),
    Jro(Src),
    Jmp(usize),

    // Register instructions
    Mov(Src, Dest),
    Add(Src),
    Sub(Src),
    Neg,
    Swp,
    Sav,
    Nop,
}

impl<'i> Core<'i> {
    pub fn new(name: &'i str, instrs: &'i [Instr]) -> Self {
        Self {
            name,
            acc: 0,
            bak: 0,
            left: Port::Dsc,
            right: Port::Dsc,
            up: Port::Dsc,
            down: Port::Dsc,
            p_ctr: 0,
            instrs,
        }
    }

    pub fn get_acc(&self) -> i16 {
        self.acc
    }

    pub fn get_bak(&self) -> i16 {
        self.bak
    }

    pub fn load(&mut self, instrs: &'i [Instr]) {
        self.instrs = instrs;
    }

    pub fn cycle(&mut self) {
        let instr = &self.instrs[self.p_ctr];
        self.p_ctr += self.execute(instr);
        self.p_ctr = clk_index(self.p_ctr as i16, self.instrs.len() as i16) as usize;
    }

    pub fn bind_up(&mut self, above: &mut Core) {
        match (&above.down, &self.up) {
            (Port::Dsc, Port::Dsc) => {}
            _ => panic!("Cannot rebind ports"),
        };
        let (tx_a, rx_a) = sync_channel(1);
        let (tx_b, rx_b) = sync_channel(1);
        above.down = Port::new(rx_a, tx_b);
        self.up = Port::new(rx_b, tx_a);
    }

    pub fn bind_down(&mut self, under: &mut Core) {
        match (&self.down, &under.up) {
            (Port::Dsc, Port::Dsc) => {}
            _ => panic!("Cannot rebind ports"),
        };
        let (tx_a, rx_a) = sync_channel(1);
        let (tx_b, rx_b) = sync_channel(1);
        self.down = Port::new(rx_a, tx_b);
        under.up = Port::new(rx_b, tx_a);
    }

    pub fn bind_left(&mut self, to_left: &mut Core) {
        match (&to_left.right, &self.left) {
            (Port::Dsc, Port::Dsc) => {}
            _ => panic!("Cannot rebind ports"),
        };
        let (tx_a, rx_a) = sync_channel(1);
        let (tx_b, rx_b) = sync_channel(1);
        to_left.right = Port::new(rx_a, tx_b);
        self.left = Port::new(rx_b, tx_a);
    }

    pub fn bind_right(&mut self, to_right: &mut Core) {
        match (&to_right.left, &self.right) {
            (Port::Dsc, Port::Dsc) => {}
            _ => panic!("Cannot rebind ports"),
        };
        let (tx_a, rx_a) = sync_channel(1);
        let (tx_b, rx_b) = sync_channel(1);
        self.right = Port::new(rx_a, tx_b);
        to_right.left = Port::new(rx_b, tx_a);
    }

    fn get_instrs(&self) -> &[Instr] {
        self.instrs
    }

    fn read(&self, src: &Src) -> i16 {
        match src {
            Src::Acc => self.acc,
            Src::Up => self.up.read(),
            Src::Down => self.down.read(),
            Src::Left => self.left.read(),
            Src::Right => self.right.read(),
            &Src::Literal(val) => val,
        }
    }

    pub fn write(&mut self, val: i16, dest: &Dest) {
        match dest {
            Dest::Acc => self.acc = val,
            Dest::Up => self.up.write(val),
            Dest::Down => self.down.write(val),
            Dest::Left => self.left.write(val),
            Dest::Right => self.right.write(val),
        };
    }

    fn execute(&mut self, instr: &Instr) -> usize {
        match instr {
            Instr::Nop => 1,
            Instr::Mov(src, dest) => {
                let val = self.read(src);
                self.write(val, dest);
                1
            }
            Instr::Swp => {
                mem::swap(&mut self.acc, &mut self.bak);
                1
            }
            Instr::Sav => {
                self.bak = self.acc;
                1
            }
            Instr::Add(src) => {
                self.acc = self.acc + self.read(src);
                1
            }
            Instr::Sub(src) => {
                self.acc = self.acc - self.read(src);
                1
            }
            Instr::Neg => {
                self.acc = self.acc * -1;
                1
            }
            &Instr::Jez(offset) => {
                if self.acc == 0 {
                    offset
                } else {
                    1
                }
            }
            &Instr::Jnz(offset) => {
                if self.acc != 0 {
                    offset
                } else {
                    1
                }
            }
            &Instr::Jgz(offset) => {
                if self.acc > 0 {
                    offset
                } else {
                    1
                }
            }
            &Instr::Jlz(offset) => {
                if self.acc < 0 {
                    offset
                } else {
                    1
                }
            }
            &Instr::Jmp(offset) => offset,
            Instr::Jro(src) => {
                let offset = self.read(src);
                let clk = self.instrs.len() as i16;
                clk_index(offset, clk) as usize
            }
        }
    }
}

fn clk_index(offset: i16, clk: i16) -> i16 {
    ((offset % clk) + clk) % clk
}
