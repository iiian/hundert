use crate::core::Core;
use crate::instructions::ctrl_flow_instructions::Ctrl;
use crate::instructions::ctrl_flow_instructions::CtrlInstr;
use crate::resource::{DestType, SrcType};

pub struct Jro<'i> {
    src: SrcType,
    index: usize,
    instrs: &'i [&'i Ctrl<'i>],
}

impl<'i> Jro<'i> {
    pub fn new(src: SrcType, index: usize, instrs: &'i [&'i Ctrl<'i>]) -> Self {
        Self { src, index, instrs }
    }
}

impl<'i> CtrlInstr<'i> for Jro<'i> {
    fn execute(&self, core: &mut Core) -> &'i Ctrl<'i> {
        let offset = match self.src {
            SrcType::Resource(DestType::Nil) => return self.instrs[self.index],
            _ => self.src.read(core),
        };

        // This block does the clock math to wrap around the instruction set enough times
        // to end up in the proper location.
        let index = {
            let imaginary = self.index as i16 + offset;
            let imaginary_half = imaginary % self.instrs.len() as i16;
            (imaginary_half + self.instrs.len() as i16) % self.instrs.len() as i16
        } as usize;
        self.instrs[index]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::mut_instructions::Mut;

    #[test]
    fn it_should_provide_the_next_instruction() {
        let mut instrs: Vec<Ctrl> = vec![];
        for i in 0..10 {
            let mov = Mut::new_mov(SrcType::Literal(i), DestType::Acc);
            let mut_wrap = Ctrl::new_mut_wrap(mov, &Ctrl::None);
            instrs.push(mut_wrap);
        }
        let instrs: Vec<&Ctrl> = instrs.iter().collect();

        let mut core = Core::new();

        let jro = Jro::new(SrcType::Literal(4), 0, &instrs[..]);
        jro.execute(&mut core).execute(&mut core);
        assert_eq!(core.acc, 4);

        let jro = Jro::new(SrcType::Literal(4), 4, &instrs[..]);
        jro.execute(&mut core).execute(&mut core);
        assert_eq!(core.acc, 8);
    }

    #[test]
    fn it_should_clock_forward() {
        let mut instrs: Vec<Ctrl> = vec![];
        for i in 0..10 {
            let mov = Mut::new_mov(SrcType::Literal(i), DestType::Acc);
            let mut_wrap = Ctrl::new_mut_wrap(mov, &Ctrl::None);
            instrs.push(mut_wrap);
        }
        let instrs: Vec<&Ctrl> = instrs.iter().collect();
        let mut core = Core::new();

        // The machine should be instructed to jump forward 19 instructions, which will need to
        // wrap around a base of 10.
        let jro = Jro::new(SrcType::Literal(19), 0, &instrs[..]);
        jro.execute(&mut core).execute(&mut core);
        assert_eq!(core.acc, 9);

        let jro = Jro::new(SrcType::Literal(19), 3, &instrs[..]);
        jro.execute(&mut core).execute(&mut core);
        assert_eq!(core.acc, 2);
    }

    #[test]
    fn it_should_clock_backward() {
        let mut instrs: Vec<Ctrl> = vec![];
        for i in 0..10 {
            let mov = Mut::new_mov(SrcType::Literal(i), DestType::Acc);
            let mut_wrap = Ctrl::new_mut_wrap(mov, &Ctrl::None);
            instrs.push(mut_wrap);
        }
        let instrs: Vec<&Ctrl> = instrs.iter().collect();
        let mut core = Core::new();

        // The machine should be instructed to jump forward 19 instructions, which will need to
        // wrap around a base of 10.
        let jro = Jro::new(SrcType::Literal(-19), 0, &instrs[..]);
        jro.execute(&mut core).execute(&mut core);
        assert_eq!(core.acc, 1);

        let jro = Jro::new(SrcType::Literal(-19), 3, &instrs[..]);
        jro.execute(&mut core).execute(&mut core);
        assert_eq!(core.acc, 4);
    }
}
