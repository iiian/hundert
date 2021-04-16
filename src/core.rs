use crate::port::Port;
use std::{mem, sync::mpsc::sync_channel};

pub struct Core {
    acc: i16,
    bak: i16,
    left: Port,
    right: Port,
    up: Port,
    down: Port,

    p_ctr: usize,
    instrs: Vec<Instr>,
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
    Nil,
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

impl Core {
    pub fn new(instrs: Vec<Instr>) -> Self {
        Self {
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

    pub fn load(&mut self, instrs: Vec<Instr>) {
        self.instrs = instrs;
    }

    pub fn cycle(&mut self) {
        let instr = &self.instrs[self.p_ctr];
        let index_opt = match instr {
            Instr::Nop => None,
            Instr::Mov(src, dest) => {
                let val = self.read(src);
                match dest {
                    &Dest::Acc => self.acc = val,
                    &Dest::Up => self.up.write(val),
                    &Dest::Down => self.down.write(val),
                    &Dest::Left => self.left.write(val),
                    &Dest::Right => self.right.write(val),
                    &Dest::Nil => {}
                };
                None
            }
            Instr::Swp => {
                mem::swap(&mut self.acc, &mut self.bak);
                None
            }
            Instr::Sav => {
                self.bak = self.acc;
                None
            }
            Instr::Add(src) => {
                self.acc = self.acc + self.read(src);
                None
            }
            Instr::Sub(src) => {
                self.acc = self.acc - self.read(src);
                None
            }
            Instr::Neg => {
                self.acc = self.acc * -1;
                None
            }
            &Instr::Jez(addr) => {
                if self.acc == 0 {
                    Some(addr)
                } else {
                    None
                }
            }
            &Instr::Jnz(addr) => {
                if self.acc != 0 {
                    Some(addr)
                } else {
                    None
                }
            }
            &Instr::Jgz(addr) => {
                if self.acc > 0 {
                    Some(addr)
                } else {
                    None
                }
            }
            &Instr::Jlz(addr) => {
                if self.acc < 0 {
                    Some(addr)
                } else {
                    None
                }
            }
            &Instr::Jmp(addr) => Some(addr),
            Instr::Jro(src) => {
                let offset = self.read(src);
                let clk = self.instrs.len() as i16;
                Some(clk_index(offset, clk) as usize)
            }
        };
        self.p_ctr = if let Some(index) = index_opt {
            index
        } else {
            self.p_ctr + 1
        };
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
        &self.instrs
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

    #[inline]
    pub fn write(&mut self, val: i16, dest: &Dest) {
        match dest {
            &Dest::Acc => self.acc = val,
            &Dest::Up => self.up.write(val),
            &Dest::Down => self.down.write(val),
            &Dest::Left => self.left.write(val),
            &Dest::Right => self.right.write(val),
            &Dest::Nil => {}
        };
    }
}

pub fn clk_index(offset: i16, clk: i16) -> i16 {
    ((offset % clk) + clk) % clk
}
