#![allow(warnings, unused)]

mod core;
mod parse;
mod port;
mod sandbox;

use parse::parse_subroutine;
use rand::Rng;
use sandbox::sandbox;
use std::array::IntoIter;

use crate::core::{Core, Dest, Instr, Src};
use std::{
    sync::mpsc::channel,
    thread::{self, JoinHandle},
};

fn main() {
    let (tx_src, rx_src) = channel();
    let (tx_snk, rx_snk) = channel();

    let mut sandbox = get_sandbox();

    let mut source = Core::new(vec![]);
    source.bind_down(&mut sandbox[1]);
    let mut sink = Core::new(parse_subroutine("MOV UP ACC"));
    sink.bind_up(&mut sandbox[10]);
    let values = vec![
        0, 32, 30, 27, 24, 28, 37, 33, 24, 13, 9, 13, 9, 13, 12, 14, 23, 21, 23, 19, 9, 18, 8, -3,
        6, 3, 14, 25, 15, 14, 3, 1, 2, -1, 1, -10, -7, -7, -11,
    ];
    let src_t = thread::spawn(move || {
        for i in values.iter() {
            tx_src.send(*i);
            source.write(*i, &Dest::Down);
        }
    });

    let sandbox_ts: Vec<_> = IntoIter::new(sandbox)
        .enumerate()
        .map(|(index, mut core)| {
            thread::spawn(move || loop {
                core.cycle();
            })
        })
        .collect();

    let sink_t = thread::spawn(move || loop {
        sink.cycle();
        tx_snk.send(sink.get_acc());
    });

    let log_t = thread::spawn(move || loop {
        if let Ok(src) = rx_src.recv() {
            if let Ok(snk) = rx_snk.recv() {
                println!("In={}, Out={}", src, snk);
            }
        }
    });

    log_t.join();
    sink_t.join();
    src_t.join();
    for thread in sandbox_ts {
        thread.join();
    }
}

fn get_sandbox() -> [Core; 12] {
    sandbox([
        parse_subroutine(""),
        parse_subroutine(
            "MOV UP ACC
MOV ACC DOWN
MOV ACC DOWN",
        ),
        parse_subroutine(""),
        parse_subroutine(""),
        parse_subroutine(""),
        parse_subroutine(
            "START:SUB UP
JGZ POS
NEG:ADD 10
MOV ACC RIGHT
MOV -1 DOWN
MOV UP ACC
JMP START
POS:SUB 10
MOV ACC DOWN
MOV 1 RIGHT
MOV UP ACC
JMP START",
        ),
        parse_subroutine(
            "START:MOV LEFT ACC
JGZ SEND0
SEND1:MOV 1 DOWN
JMP START
SEND0:MOV 0 DOWN",
        ),
        parse_subroutine(""),
        parse_subroutine(""),
        parse_subroutine(
            "START:MOV UP ACC
JLZ SEND0
SEND1:MOV 1 RIGHT
JMP START
SEND0:MOV 0 RIGHT
JMP START",
        ),
        parse_subroutine(
            "MOV UP ACC
ADD LEFT
MOV ACC DOWN",
        ),
        parse_subroutine(""),
    ])
}
