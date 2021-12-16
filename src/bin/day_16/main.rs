#[allow(unused_imports)]
use advent_of_code_2021::*;
use nom::{bits::complete::take, IResult};
use std::fmt::Debug;

fn main() {
    let input = include_str!("./input.txt").trim_end();

    println!("{:?}", part_1(input));
    println!("{:?}", part_2(input));
}

#[derive(Debug)]
struct Packet {
    version: u8,
    data: Box<PacketData>,
}

#[derive(Debug)]
enum PacketData {
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Min(Vec<Packet>),
    Max(Vec<Packet>),
    Value(u64),
    Gt(Packet, Packet),
    Lt(Packet, Packet),
    Eq(Packet, Packet),
}

type Input<'a> = (&'a [u8], usize);

fn length(from: Input, to: Input) -> usize {
    (from.0.len() * 8 - from.1) - (to.0.len() * 8 - to.1)
}

fn read<T>(i: Input, n: usize) -> IResult<Input, T>
where
    T: From<u8>
        + std::ops::AddAssign
        + std::ops::Shl<usize, Output = T>
        + std::ops::Shr<usize, Output = T>,
{
    take(n)(i)
}

fn parse_value(mut i: Input) -> IResult<Input, u64> {
    let mut value = 0;

    loop {
        let (s, prefix) = read::<u8>(i, 1)?;
        i = s;

        let (s, bits) = read::<u64>(i, 4)?;
        i = s;

        value = value << 4 | bits;

        if prefix == 0 {
            return Ok((i, value));
        }
    }
}

fn parse_operator_subpackets(i: Input) -> IResult<Input, Vec<Packet>> {
    let (mut i, length_type_id) = read::<u8>(i, 1)?;

    let mut packets = Vec::new();

    if length_type_id == 0 {
        let (s, total_length) = read(i, 15)?;
        i = s;

        let mut read = 0;
        while read < total_length {
            let (s, packet) = parse_packet(i)?;

            read += length(i, s);
            i = s;

            packets.push(packet);
        }
    } else {
        let (s, num_packets) = read(i, 11)?;
        i = s;

        for _ in 0..num_packets {
            let (s, packet) = parse_packet(i)?;
            i = s;

            packets.push(packet);
        }
    }

    Ok((i, packets))
}

fn parse_two_operator_subpackets(i: Input) -> IResult<Input, (Packet, Packet)> {
    let (i, packets) = parse_operator_subpackets(i)?;
    let mut packets = packets.into_iter();

    Ok((i, (packets.next().unwrap(), packets.next().unwrap())))
}

fn parse_packet(i: Input) -> IResult<Input, Packet> {
    let (i, version) = read(i, 3)?;
    let (i, type_id) = read(i, 3)?;

    let (i, packet_data) = match type_id {
        0 => {
            let (i, packets) = parse_operator_subpackets(i)?;
            (i, PacketData::Sum(packets))
        }
        1 => {
            let (i, packets) = parse_operator_subpackets(i)?;
            (i, PacketData::Product(packets))
        }
        2 => {
            let (i, packets) = parse_operator_subpackets(i)?;
            (i, PacketData::Min(packets))
        }
        3 => {
            let (i, packets) = parse_operator_subpackets(i)?;
            (i, PacketData::Max(packets))
        }
        4 => {
            let (i, value) = parse_value(i)?;
            (i, PacketData::Value(value))
        }
        5 => {
            let (i, (l, r)) = parse_two_operator_subpackets(i)?;
            (i, PacketData::Gt(l, r))
        }
        6 => {
            let (i, (l, r)) = parse_two_operator_subpackets(i)?;
            (i, PacketData::Lt(l, r))
        }
        7 => {
            let (i, (l, r)) = parse_two_operator_subpackets(i)?;
            (i, PacketData::Eq(l, r))
        }
        _ => unreachable!(),
    };

    Ok((
        i,
        Packet {
            version,
            data: Box::new(packet_data),
        },
    ))
}

fn parse_input(input: &str) -> Vec<u8> {
    let input = input.chars().map(|c| c.to_digit(16).unwrap() as u8);

    input
        .clone()
        .step_by(2)
        .zip(input.skip(1).step_by(2))
        .map(|(a, b)| a << 4 | b)
        .collect::<Vec<_>>()
}

fn part_1(input: &str) -> impl Debug {
    let input = parse_input(input);
    let (_, packet) = parse_packet((&input, 0)).unwrap();

    fn count_versions(packet: &Packet) -> u64 {
        packet.version as u64
            + match &*packet.data {
                PacketData::Sum(packets)
                | PacketData::Product(packets)
                | PacketData::Min(packets)
                | PacketData::Max(packets) => packets.iter().map(count_versions).sum(),
                PacketData::Value(_) => 0,
                PacketData::Gt(l, r) | PacketData::Lt(l, r) | PacketData::Eq(l, r) => {
                    count_versions(&l) + count_versions(&r)
                }
            }
    }

    count_versions(&packet)
}

fn part_2(input: &str) -> impl Debug {
    let input = parse_input(input);
    let (_, packet) = parse_packet((&input, 0)).unwrap();

    fn eval(packet: &Packet) -> u64 {
        match &*packet.data {
            PacketData::Sum(packets) => packets.iter().map(eval).sum(),
            PacketData::Product(packets) => packets.iter().map(eval).product(),
            PacketData::Min(packets) => packets.iter().map(eval).min().unwrap(),
            PacketData::Max(packets) => packets.iter().map(eval).max().unwrap(),
            PacketData::Value(x) => *x,
            PacketData::Gt(l, r) => {
                if eval(l) > eval(r) {
                    1
                } else {
                    0
                }
            }
            PacketData::Lt(l, r) => {
                if eval(l) < eval(r) {
                    1
                } else {
                    0
                }
            }
            PacketData::Eq(l, r) => {
                if eval(l) == eval(r) {
                    1
                } else {
                    0
                }
            }
        }
    }

    eval(&packet)
}
