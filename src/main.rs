#![allow(warnings, unused)]

mod core;
mod parse;
mod port;
use parse::parse_subroutine;

use crate::core::{Core, Dest, Instr, Src};
use std::{
    sync::mpsc::channel,
    thread::{self, JoinHandle},
};

fn main() {
    let mut src = Core::new(vec![]);
    let mut c1 = Core::new(parse_subroutine("MOV UP DOWN"));
    src.bind_down(&mut c1);
    let mut c2 = Core::new(parse_subroutine(
        "MOV UP ACC
ADD ACC
MOV ACC DOWN",
    ));
    c1.bind_down(&mut c2);
    let mut sink = Core::new(parse_subroutine("MOV UP ACC"));
    c2.bind_down(&mut sink);
    let (src_tx, src_rx) = channel::<i16>();
    let (sink_tx, sink_rx) = channel::<i16>();

    let mut i: i16 = 0;
    let src_t = thread::spawn(move || loop {
        src_tx.send(i);
        src.write(i, &Dest::Down);
        i += 1;
    });

    let c1_t = thread::spawn(move || loop {
        c1.cycle();
    });

    let c2_t = thread::spawn(move || loop {
        c2.cycle();
    });

    let sink_t = thread::spawn(move || loop {
        sink_tx.send(sink.get_acc());
        sink.cycle();
        thread::sleep_ms(1);
    });

    let log_t = thread::spawn(move || loop {
        if let Ok(src) = src_rx.recv() {
            if let Ok(sink) = sink_rx.recv() {
                println!("In={}, Out={}", src, sink);
            }
        }
    });

    src_t.join();
    c1_t.join();
    c2_t.join();
    sink_t.join();
    log_t.join();
}
