use core::panic;
use std::{collections::HashMap, usize};

use crate::core::{Dest, Instr, Src, clk_index};

pub fn parse_subroutine(input: &str) -> Vec<Instr> {
    let mut instrs: Vec<Instr> = vec![];
    if input.len() == 0 {
        return instrs;
    }
    let mut label_map = HashMap::<&str, usize>::new();
    for (index, line) in input.split("\n").enumerate() {
        let maybe_label: Vec<_> = line.split(":").collect();
        if maybe_label.len() == 2 {
            if label_map.contains_key(maybe_label[0]) {
                panic!()
            }
            label_map.insert(maybe_label[0], index);
        }
    }

    for (index, line) in input.split("\n").enumerate() {
        let maybe_label: Vec<_> = line.split(":").collect();
        let instr_body = if maybe_label.len() == 2 {
            maybe_label[1]
        } else {
            line
        };

        instrs.push(parse_instr(instr_body, &label_map, &index));
    }

    instrs
}

fn parse_instr(instr_body: &str, label_map: &HashMap<&str, usize>, index: &usize) -> Instr {
    let instr_id = &instr_body[0..3];
    let instr_body = &instr_body[3..];
    match instr_id {
        "NOP" => Instr::Nop,
        "MOV" => parse_mov_instr(instr_body),
        "SWP" => Instr::Swp,
        "SAV" => Instr::Sav,
        "ADD" => parse_add_instr(instr_body),
        "SUB" => parse_sub_instr(instr_body),
        "NEG" => Instr::Neg,
        "JMP" => parse_jmp_instr(instr_body, &label_map),
        "JEZ" => parse_jez_instr(instr_body, &label_map),
        "JNZ" => parse_jnz_instr(instr_body, &label_map),
        "JGZ" => parse_jgz_instr(instr_body, &label_map),
        "JLZ" => parse_jlz_instr(instr_body, &label_map),
        "JRO" => parse_jro_instr(instr_body),
        unk => panic!("Unrecognized instruction: {} with body {}", unk, instr_body),
    }
}

fn parse_jro_instr(instr_body: &str) -> Instr {
    Instr::Jro(match_src(instr_body.trim()))
}

fn parse_jlz_instr(
    instr_body: &str,
    label_map: &HashMap<&str, usize>,
) -> Instr {
    Instr::Jlz(*label_map.get(instr_body.trim()).unwrap())
}

fn parse_jgz_instr(
    instr_body: &str,
    label_map: &HashMap<&str, usize>,
) -> Instr {
    Instr::Jgz(*label_map.get(instr_body.trim()).unwrap())
}

fn parse_jnz_instr(
    instr_body: &str,
    label_map: &HashMap<&str, usize>,
) -> Instr {
    Instr::Jnz(*label_map.get(instr_body.trim()).unwrap())
}

fn parse_jez_instr(
    instr_body: &str,
    label_map: &HashMap<&str, usize>,
) -> Instr {
    Instr::Jez(*label_map.get(instr_body.trim()).unwrap())
}

fn parse_jmp_instr(
    instr_body: &str,
    label_map: &HashMap<&str, usize>,
) -> Instr {
    Instr::Jmp(*label_map.get(instr_body.trim()).unwrap())
}

fn parse_sub_instr(instr_body: &str) -> Instr {
    Instr::Sub(match_src(instr_body.trim()))
}

fn parse_add_instr(instr_body: &str) -> Instr {
    Instr::Add(match_src(instr_body.trim()))
}

fn parse_mov_instr(instr_body: &str) -> Instr {
    let instrs: Vec<_> = instr_body.trim().split(" ").collect();
    let src = match_src(instrs[0]);
    let dest = match_dest(instrs[1]);
    Instr::Mov(src, dest)
}

fn match_src(possible_src: &str) -> Src {
    match possible_src {
        "ACC" => Src::Acc,
        "UP" => Src::Up,
        "DOWN" => Src::Down,
        "LEFT" => Src::Left,
        "RIGHT" => Src::Right,
        "NIL" => Src::Literal(0),
        otherwise => Src::Literal(otherwise.parse().unwrap()),
    }
}

fn match_dest(possible_dest: &str) -> Dest {
    match possible_dest {
        "ACC" => Dest::Acc,
        "UP" => Dest::Up,
        "DOWN" => Dest::Down,
        "LEFT" => Dest::Left,
        "RIGHT" => Dest::Right,
        "NIL" => Dest::Nil,
        unk => panic!("Unknown destination {} was specified", unk),
    }
}
