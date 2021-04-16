use crate::Core;
use crate::Instr;
use core::array::IntoIter;

pub fn sandbox(instrs: [Vec<Instr>; 12]) -> [Core; 12] {
    let mut c00 = new();
    let mut c01 = new();
    let mut c02 = new();
    let mut c03 = new();

    let mut c10 = new();
    let mut c11 = new();
    let mut c12 = new();
    let mut c13 = new();

    let mut c20 = new();
    let mut c21 = new();
    let mut c22 = new();
    let mut c23 = new();

    c00.bind_right(&mut c01);
    c01.bind_right(&mut c02);
    c02.bind_right(&mut c03);

    c10.bind_right(&mut c11);
    c11.bind_right(&mut c12);
    c12.bind_right(&mut c13);

    c20.bind_right(&mut c21);
    c21.bind_right(&mut c22);
    c22.bind_right(&mut c23);

    c00.bind_down(&mut c10);
    c01.bind_down(&mut c11);
    c02.bind_down(&mut c12);
    c03.bind_down(&mut c13);

    c10.bind_down(&mut c20);
    c11.bind_down(&mut c21);
    c12.bind_down(&mut c22);
    c13.bind_down(&mut c23);

    let mut cores = [c00, c01, c02, c03, c10, c11, c12, c13, c20, c21, c22, c23];

    for (index, (core, instrs)) in cores.iter_mut().zip(IntoIter::new(instrs)).enumerate() {
        core.load(instrs);
    }

    cores
}

fn new() -> Core {
    Core::new(vec![])
}
