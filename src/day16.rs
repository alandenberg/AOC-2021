use bitbuffer::{BitReadBuffer, BigEndian, BitReadStream};
use substring::Substring;

mod util;

fn main() {
    let input = util::read_input_file("day16.txt").trim().to_string();
    let packet = parse_packet(&input);

    let version_sum = sum_packet_versions(&packet);
    println!("Part 1: Solution={}", version_sum);

    let operator_result = execute_packet_operators(&packet);
    println!("Part 2: Solution={}", operator_result);
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Operator {
    Sum,
    Product,
    Minimum,
    Maximum,
    Literal,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Clone, Debug)]
struct Packet {
    version: u8,
    operator: Operator,
    sub_packets: Vec<Packet>,
    literal_value: u64,
}

fn hex_to_binary(c: char) -> u8 {
    return match c {
        '0' => 0b0000,
        '1' => 0b0001,
        '2' => 0b0010,
        '3' => 0b0011,
        '4' => 0b0100,
        '5' => 0b0101,
        '6' => 0b0110,
        '7' => 0b0111,
        '8' => 0b1000,
        '9' => 0b1001,
        'A' => 0b1010,
        'B' => 0b1011,
        'C' => 0b1100,
        'D' => 0b1101,
        'E' => 0b1110,
        'F' => 0b1111,
        _ => panic!("illegal character {}", c),
    }
}

fn type_id_to_operator(type_id: u8) -> Operator {
    return match type_id {
        0 => Operator::Sum,
        1 => Operator::Product,
        2 => Operator::Minimum,
        3 => Operator::Maximum,
        4 => Operator::Literal,
        5 => Operator::GreaterThan,
        6 => Operator::LessThan,
        7 => Operator::EqualTo,
        _ => panic!("illegaly type id {}", type_id),
    }
}

#[allow(unused_must_use)]
fn read_next_packet(stream: &mut BitReadStream<BigEndian>) -> Packet {
    let mut sub_packets = vec![];
    let mut literal_value: u64 = 0;

    let packet_version = stream.read_int::<u8>(3).unwrap();
    let packet_type = stream.read_int::<u8>(3).unwrap();
    let operator = type_id_to_operator(packet_type);

    if operator != Operator::Literal {
        let length_type_id = stream.read_int::<u8>(1).unwrap();

        if length_type_id == 0 {
            let total_length = stream.read_int::<usize>(15).unwrap();
            let start_pos = stream.pos();
            while stream.pos() < start_pos + total_length {
                let sub_packet = read_next_packet(stream);
                sub_packets.push(sub_packet);
            }
        } else {
            let num_sub_packets = stream.read_int::<usize>(11).unwrap();
            for _ in 0..num_sub_packets {
                let sub_packet = read_next_packet(stream);
                sub_packets.push(sub_packet);
            }
        }
    } else {
        let mut literal_string = String::new();
        loop {
            if stream.bits_left() < 5
            {
                let pos = stream.pos();
                if pos % 4 > 0 {
                    // Align to the next 4-bit block
                    stream.set_pos(pos + 4 - pos % 4);
                }
                break;
            }

            let is_last_group = stream.read_int::<u8>(1).unwrap() == 0;
            let value = stream.read_int::<u8>(4).unwrap();
            let value_string = &format!("{:#06b}", value);
            literal_string += value_string.substring(2, value_string.len());

            if is_last_group {
                break;
            }
        }

        literal_value = u64::from_str_radix(&literal_string, 2).unwrap();
    }

    return Packet { version: packet_version, operator: operator, sub_packets: sub_packets, literal_value: literal_value };
}

fn parse_packet(line: &String) -> Packet {
    let binary: Vec<u8> = line.chars().map(hex_to_binary).collect();
    assert_eq!(binary.len() % 2, 0);

    let mut bytes: Vec<u8> = vec![];
    for i in (0..binary.len()).step_by(2) {
        bytes.push((binary[i] << 4) + binary[i+1]);
    }

    let buffer = BitReadBuffer::new(&bytes, BigEndian);
    let mut stream = BitReadStream::new(buffer);

    let packet = read_next_packet(&mut stream);

    return packet;
}

fn sum_packet_versions(packet: &Packet) -> u64 {
    let mut sum = packet.version as u64;

    for sub_packet in &packet.sub_packets {
        sum += sum_packet_versions(sub_packet);
    }

    return sum;
}

fn execute_packet_operators(packet: &Packet) -> u64 {
    if packet.operator == Operator::Literal {
        return packet.literal_value;
    }

    let sub_packets: Vec<u64> = packet.sub_packets.iter().map(execute_packet_operators).collect();

    return match packet.operator {
        Operator::Sum => sub_packets.iter().sum(),
        Operator::Product => sub_packets.iter().product(),
        Operator::Minimum => *sub_packets.iter().min().unwrap(),
        Operator::Maximum => *sub_packets.iter().max().unwrap(),
        Operator::Literal => packet.literal_value,
        Operator::GreaterThan => if sub_packets[0] > sub_packets[1] { 1 } else { 0 },
        Operator::LessThan => if sub_packets[0] < sub_packets[1] { 1 } else { 0 },
        Operator::EqualTo => if sub_packets[0] == sub_packets[1] { 1 } else { 0 },
    }
}

#[test]
fn test_day16_part1_example1() {
    let input = String::from("8A004A801A8002F478");
    let packet = parse_packet(&input);
    let version_sum = sum_packet_versions(&packet);

    assert_eq!(version_sum, 16);
}

#[test]
fn test_day16_part1_example2() {
    let input = String::from("620080001611562C8802118E34");
    let packet = parse_packet(&input);
    let version_sum = sum_packet_versions(&packet);

    assert_eq!(version_sum, 12);
}

#[test]
fn test_day16_part1_example3() {
    let input = String::from("C0015000016115A2E0802F182340");
    let packet = parse_packet(&input);
    let version_sum = sum_packet_versions(&packet);

    assert_eq!(version_sum, 23);
}

#[test]
fn test_day16_part1_example4() {
    let input = String::from("A0016C880162017C3686B18A3D4780");
    let packet = parse_packet(&input);
    let version_sum = sum_packet_versions(&packet);

    assert_eq!(version_sum, 31);
}

#[test]
fn test_day16_part1_solution() {
    let input = util::read_input_file("day16.txt").trim().to_string();
    let packet = parse_packet(&input);
    let version_sum = sum_packet_versions(&packet);

    assert_eq!(version_sum, 923);
}

#[test]
fn test_day16_part2_example1() {
    let input = String::from("C200B40A82");
    let packet = parse_packet(&input);
    let operator_result = execute_packet_operators(&packet);

    assert_eq!(operator_result, 3);
}

#[test]
fn test_day16_part2_example2() {
    let input = String::from("04005AC33890");
    let packet = parse_packet(&input);
    let operator_result = execute_packet_operators(&packet);

    assert_eq!(operator_result, 54);
}

#[test]
fn test_day16_part2_example3() {
    let input = String::from("880086C3E88112");
    let packet = parse_packet(&input);
    let operator_result = execute_packet_operators(&packet);

    assert_eq!(operator_result, 7);
}

#[test]
fn test_day16_part2_example4() {
    let input = String::from("CE00C43D881120");
    let packet = parse_packet(&input);
    let operator_result = execute_packet_operators(&packet);

    assert_eq!(operator_result, 9);
}

#[test]
fn test_day16_part2_example5() {
    let input = String::from("D8005AC2A8F0");
    let packet = parse_packet(&input);
    let operator_result = execute_packet_operators(&packet);

    assert_eq!(operator_result, 1);
}

#[test]
fn test_day16_part2_example6() {
    let input = String::from("F600BC2D8F");
    let packet = parse_packet(&input);
    let operator_result = execute_packet_operators(&packet);

    assert_eq!(operator_result, 0);
}

#[test]
fn test_day16_part2_example7() {
    let input = String::from("9C005AC2F8F0");
    let packet = parse_packet(&input);
    let operator_result = execute_packet_operators(&packet);

    assert_eq!(operator_result, 0);
}

#[test]
fn test_day16_part2_example8() {
    let input = String::from("9C0141080250320F1802104A08");
    let packet = parse_packet(&input);
    let operator_result = execute_packet_operators(&packet);

    assert_eq!(operator_result, 1);
}

#[test]
fn test_day16_part2_solution() {
    let input = util::read_input_file("day16.txt").trim().to_string();
    let packet = parse_packet(&input);
    let operator_result = execute_packet_operators(&packet);

    assert_eq!(operator_result, 258888628940);
}