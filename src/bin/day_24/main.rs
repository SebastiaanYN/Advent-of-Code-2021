#[allow(unused_imports)]
use advent_of_code_2021::*;
use nom::{branch::alt, bytes::complete::tag, character::complete::one_of, IResult};
use std::{collections::HashSet, fmt::Debug};

fn main() {
    let input = include_str!("./input.txt").trim_end();

    println!("{:?}", part_1(input));
    println!("{:?}", part_2(input));
}

#[derive(Copy, Clone, Debug)]
enum Reg {
    W,
    X,
    Y,
    Z,
}

impl From<char> for Reg {
    fn from(c: char) -> Self {
        match c {
            'w' => Reg::W,
            'x' => Reg::X,
            'y' => Reg::Y,
            'z' => Reg::Z,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Val {
    Reg(Reg),
    Num(i64),
}

impl From<&str> for Val {
    fn from(s: &str) -> Self {
        s.parse()
            .map(Val::Num)
            .unwrap_or_else(|_| Val::Reg(s.chars().next().unwrap().into()))
    }
}

#[derive(Copy, Clone, Debug)]
enum Instr {
    Inp(Reg),
    Add(Reg, Val),
    Mul(Reg, Val),
    Div(Reg, Val),
    Mod(Reg, Val),
    Eql(Reg, Val),
}

fn parse_instr(s: &str) -> IResult<&str, Instr> {
    let (s, ins) = alt((
        tag("inp"),
        tag("add"),
        tag("mul"),
        tag("div"),
        tag("mod"),
        tag("eql"),
    ))(s)?;

    let (s, _) = tag(" ")(s)?;
    let (s, reg) = one_of("wxyz")(s)?;

    Ok((
        s,
        match ins {
            "inp" => Instr::Inp(reg.into()),
            "add" => Instr::Add(reg.into(), s[1..].into()),
            "mul" => Instr::Mul(reg.into(), s[1..].into()),
            "div" => Instr::Div(reg.into(), s[1..].into()),
            "mod" => Instr::Mod(reg.into(), s[1..].into()),
            "eql" => Instr::Eql(reg.into(), s[1..].into()),
            _ => unreachable!(),
        },
    ))
}

fn run_instruction(registers: &mut [i64; 4], instr: Instr) {
    let v = |val: Val| match val {
        Val::Reg(r) => registers[r as usize],
        Val::Num(x) => x,
    };

    match instr {
        Instr::Add(a, b) => registers[a as usize] += v(b),
        Instr::Mul(a, b) => registers[a as usize] *= v(b),
        Instr::Div(a, b) => registers[a as usize] /= v(b),
        Instr::Mod(a, b) => registers[a as usize] %= v(b),
        Instr::Eql(a, b) => registers[a as usize] = (registers[a as usize] == v(b)) as i64,
        _ => unreachable!(),
    }
}

fn solve(
    seen: &mut HashSet<(i64, usize)>,
    blocks: &[&[Instr]],
    block: usize,
    z: i64,
    range: &[i64],
) -> Option<i64> {
    if block == blocks.len() {
        return if z == 0 { Some(0) } else { None };
    }

    if seen.contains(&(z, block)) {
        return None;
    }

    for &n in range {
        let mut registers = [n, 0, 0, z];

        for &instr in blocks[block] {
            match instr {
                Instr::Inp(_) => {}
                _ => run_instruction(&mut registers, instr),
            }
        }

        if let Some(x) = solve(seen, blocks, block + 1, registers[Reg::Z as usize], range) {
            return Some(x * 10 + n);
        }
    }

    seen.insert((z, block));
    None
}

fn part_1(input: &str) -> impl Debug {
    let instrs = input
        .lines()
        .map(|line| parse_instr(line).unwrap().1)
        .collect::<Vec<_>>();
    let blocks = instrs.chunks(18).collect::<Vec<_>>();

    solve(
        &mut HashSet::new(),
        &blocks,
        0,
        0,
        &(1..=9).rev().collect::<Vec<_>>(),
    )
    .unwrap()
    .to_string()
    .chars()
    .rev()
    .collect::<String>()
}

fn part_2(input: &str) -> impl Debug {
    let instrs = input
        .lines()
        .map(|line| parse_instr(line).unwrap().1)
        .collect::<Vec<_>>();
    let blocks = instrs.chunks(18).collect::<Vec<_>>();

    solve(
        &mut HashSet::new(),
        &blocks,
        0,
        0,
        &(1..=9).collect::<Vec<_>>(),
    )
    .unwrap()
    .to_string()
    .chars()
    .rev()
    .collect::<String>()
}
