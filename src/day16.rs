use std::num;

fn parse_input_lines(raw_input_lines: &[String]) -> Result<String, num::ParseIntError> {
    let input_lines = raw_input_lines.iter().collect::<Vec<&String>>();
    assert!(input_lines.len() == 1);
    Ok(input_lines[0].clone())
}

fn convert_char_to_binary(char: &char) -> String {
    let digit = u8::from_str_radix(&char.to_string(), 16).unwrap();
    let binary = format!("{:b}", digit);
    let binary_with_padding = "0".repeat(4 - binary.len()).to_string() + &binary;
    binary_with_padding
}

fn convert_string_to_binary(data: &String) -> String {
    let binary_nums: Vec<String> = data
        .chars()
        .map(|x| convert_char_to_binary(&x))
        .collect::<_>();
    binary_nums.join("")
}

#[derive(Debug, Clone)]
pub struct Packet {
    version: u32,
    packet_type: u32,
    literal_data: Option<u64>,
    sub_packets: Option<Vec<Packet>>,
}
impl Packet {
    fn sum_total_versions(&self) -> u32 {
        let mut sum = self.version;
        if let Some(packets_to_operate) = &self.sub_packets {
            sum += packets_to_operate.iter().fold(0, |sub_packet_sum, packet| {
                sub_packet_sum + packet.sum_total_versions()
            });
        }
        sum
    }

    fn calc_value(&self) -> u64 {
        if let Some(literal_data) = self.literal_data {
            return literal_data;
        } else {
            return self.calc_operator_value();
        }
    }
    fn calc_operator_value(&self) -> u64 {
        let sub_packets = (self.sub_packets.as_ref()).unwrap();
        match self.packet_type {
            0 => sub_packets
                .iter()
                .fold(0, |sum, packet| sum + packet.calc_value()),
            1 => sub_packets
                .iter()
                .fold(1, |product, packet| product * packet.calc_value()),
            2 => sub_packets
                .iter()
                .min_by(|x, y| x.calc_value().cmp(&y.calc_value()))
                .unwrap()
                .calc_value(),
            3 => sub_packets
                .iter()
                .max_by(|x, y| x.calc_value().cmp(&y.calc_value()))
                .unwrap()
                .calc_value(),
            5 => {
                assert!(sub_packets.len() == 2);
                if sub_packets[0].calc_value() > sub_packets[1].calc_value() {
                    1
                } else {
                    0
                }
            }
            6 => {
                assert!(sub_packets.len() == 2);
                if sub_packets[0].calc_value() < sub_packets[1].calc_value() {
                    1
                } else {
                    0
                }
            }
            7 => {
                assert!(sub_packets.len() == 2);
                if sub_packets[0].calc_value() == sub_packets[1].calc_value() {
                    1
                } else {
                    0
                }
            }

            _ => panic!("invalid"),
        }
    }
}

fn get_data_from_buffer(buffer: &mut String, size: usize) -> String {
    let data = buffer[..size].to_string();
    *buffer = buffer[size..buffer.len()].to_string();
    data
}

fn get_literal_packet(transmission: &mut String) -> Packet {
    let version = u32::from_str_radix(&get_data_from_buffer(transmission, 3), 2).unwrap();
    let packet_type = u32::from_str_radix(&get_data_from_buffer(transmission, 3), 2).unwrap();
    assert!(packet_type == 4);

    let mut binary_literal_data = String::new();
    let mut processed_last_literal_data_group = false;
    while !processed_last_literal_data_group {
        // Work out if last group
        let literal_data_group_type = get_data_from_buffer(transmission, 1);
        if literal_data_group_type == "0" {
            processed_last_literal_data_group = true
        }

        // Read the literal data
        binary_literal_data += &get_data_from_buffer(transmission, 4)
    }

    let literal_data = u64::from_str_radix(&binary_literal_data, 2).unwrap();

    Packet {
        version,
        packet_type,
        literal_data: Some(literal_data),
        sub_packets: None,
    }
}

fn get_operator_packet(transmission: &mut String) -> Packet {
    let version = u32::from_str_radix(&get_data_from_buffer(transmission, 3), 2).unwrap();
    let packet_type = u32::from_str_radix(&get_data_from_buffer(transmission, 3), 2).unwrap();
    assert!(packet_type != 4);

    // Workout length type
    let length_type_id = u32::from_str_radix(&get_data_from_buffer(transmission, 1), 2).unwrap();

    let mut packets_to_operate = vec![];

    if length_type_id == 1 {
        let num_sub_packets_bit_length = 11;

        let num_sub_packets =
            usize::from_str_radix(&transmission[..num_sub_packets_bit_length], 2).unwrap();
        *transmission = transmission[num_sub_packets_bit_length..transmission.len()].to_string();

        for _ in 0..num_sub_packets {
            packets_to_operate.push(get_next_packet(transmission));
        }
    } else {
        let num_bits_in_sub_packets_length = 15;

        let sub_packets_bit_length =
            usize::from_str_radix(&transmission[..num_bits_in_sub_packets_length], 2).unwrap();
        *transmission =
            transmission[num_bits_in_sub_packets_length..transmission.len()].to_string();

        // Get sub packets to operate on
        let mut sub_packets_bits = transmission[..sub_packets_bit_length].to_string();
        *transmission = transmission[sub_packets_bit_length..transmission.len()].to_string();

        while sub_packets_bits.contains("1") {
            packets_to_operate.push(get_next_packet(&mut sub_packets_bits));
        }
    }

    Packet {
        version,
        packet_type,
        literal_data: None,
        sub_packets: Some(packets_to_operate),
    }
}

pub fn get_next_packet(transmission: &mut String) -> Packet {
    let packet_type = u32::from_str_radix(&transmission[3..6], 2).unwrap();

    if packet_type == 4 {
        let packet = get_literal_packet(transmission);
        packet
    } else {
        let packet = get_operator_packet(transmission);
        packet
    }
}

pub fn part_1(encoded_data: &String) -> i64 {
    let encoded_data = encoded_data.clone();
    let mut transmission = convert_string_to_binary(&encoded_data);
    let packet = get_next_packet(&mut transmission);

    packet.sum_total_versions() as i64
}

pub fn part_2(encoded_data: &String) -> i64 {
    let encoded_data = encoded_data.clone();
    let mut transmission = convert_string_to_binary(&encoded_data);
    let packet = get_next_packet(&mut transmission);

    packet.calc_value() as i64
}

pub fn day16(input_lines: &[String]) -> (u64, u64) {
    let encoded_data = parse_input_lines(input_lines).unwrap_or_else(|err| {
        panic!("Got error : {} , when trying to parse the input lines", err);
    });
    (part_1(&encoded_data) as u64, part_2(&encoded_data) as u64)
}
