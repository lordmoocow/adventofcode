pub type Version = u8;
pub type Type = u8;

const LITERAL: u8 = 4;
const MODE_LEN: usize = 1;
const MODE_0_LEN: usize = 15;
const MODE_1_LEN: usize = 11;
const VERSION_LEN: usize = 3;
const TYPE_LEN: usize = 3;
const HEADER_LEN: usize = VERSION_LEN + TYPE_LEN;
const LITERAL_DIGIT_LEN: usize = 4;
const LITERAL_CHUNK_LEN: usize = 1 + LITERAL_DIGIT_LEN;

#[derive(Debug)]
pub struct Packet {
    version: Version,
    kind: Type,
    value: Option<usize>,
    packets: Option<Vec<Packet>>,
}

impl Packet {
    pub fn from_hex(hex: &str) -> Self {
        let bin = hex_to_bin(&hex);
        decode(&bin).0
    }

    pub fn version_sum(&self) -> usize {
        if let Some(packets) = &self.packets {
            packets
                .iter()
                .fold(self.version as usize, |sum, p| sum + p.version_sum())
        } else {
            self.version as usize
        }
    }

    pub fn value(&self) -> usize {
        // convenience...
        let packets = || self.packets.as_ref().unwrap().iter().map(|p| p.value());

        match self.kind {
            0 => packets().fold(0, |v, p| v + p),
            1 => packets().fold(1, |v, p| v * p),
            2 => packets().min().unwrap_or(0),
            3 => packets().max().unwrap_or(0),
            4 => self.value.unwrap_or(0),
            5 | 6 | 7 => {
                let packets: Vec<_> = packets().collect();
                if match self.kind {
                    5 => packets[0] > packets[1],
                    6 => packets[0] < packets[1],
                    7 => packets[0] == packets[1],
                    _ => false,
                } { 1 } else { 0 }
            },
            _ => 0,
        }
    }
}

fn decode(binary: &[u8]) -> (Packet, usize) {
    let (v, t) = decode_header(&binary[..HEADER_LEN]);
    let mut p = Packet {
        kind: t,
        version: v,
        packets: None,
        value: None,
    };

    // decode the payload for this packet
    let mut pos = HEADER_LEN;
    let buf = &binary[pos..];
    pos += match p.kind {
        LITERAL => {
            let (value, len) = decode_literal(buf);
            p.value.replace(value);
            len
        }
        _ => {
            let (packets, len) = decode_sub_packets(buf);
            p.packets.replace(packets);
            len
        }
    };

    // return the position we read up to in the buffer
    // this allows us to know where the end of this packet
    // was and therefore the start of the next
    (p, pos)
}

fn decode_header(buf: &[u8]) -> (Version, Type) {
    // header is comprised of two 3-bit chunks
    // our buffer is actually comprised of 8-bit values
    // representing each 1-bit value -- which is a bit silly really

    // the first 3-bits is the version
    let v = bits_to_usize(&buf[..VERSION_LEN]) as u8;

    // next 3-bits is the type
    let t = bits_to_usize(&buf[VERSION_LEN..HEADER_LEN]) as u8;

    (v, t)
}

fn decode_literal(buf: &[u8]) -> (usize, usize) {
    let mut len = 0;

    let mut digit = Vec::default();
    for chunk in buf.chunks(LITERAL_CHUNK_LEN) {
        digit.extend(&chunk[1..LITERAL_CHUNK_LEN]);
        len += LITERAL_CHUNK_LEN;
        if chunk[0] == 0 {
            break;
        }
    }

    let value = bits_to_usize(&digit);
    (value, len)
}

fn decode_sub_packets(buf: &[u8]) -> (Vec<Packet>, usize) {
    // determine size of payload and length mode
    let (len, i) = decode_packet_len(buf);
    let mode_1 = i < MODE_0_LEN;

    // remaining payload after length header
    let mut buf = &buf[i..];
    let mut packets = Vec::default();
    let mut pos = 0;
    while if mode_1 {
        packets.len() < len
    } else {
        pos < len
    } {
        let (p, len) = decode(buf);
        packets.push(p);

        // move buffer along and update position tracker
        buf = &buf[len..];
        pos += len;
    }
    (packets, pos + i)
}

fn decode_packet_len(buf: &[u8]) -> (usize, usize) {
    // packet length is determined by the first bit
    // and either the following 11 or 15 bits contain the length

    // 0 = 15, 1 = 11
    let bit_len = if buf[0] == 0 { MODE_0_LEN } else { MODE_1_LEN };

    let len = bits_to_usize(&buf[MODE_LEN..bit_len + 1]) as usize;

    // return the number of bits we have read too
    (len, bit_len + 1)
}

fn bits_to_usize(bits: &[u8]) -> usize {
    let mut result = 0;

    for (i, bit) in bits.iter().rev().enumerate() {
        if bit != &0 {
            result |= 1 << i;
        }
    }

    result
}

fn hex_to_bin(hex: &str) -> Vec<u8> {
    hex.chars()
        .flat_map(|c| {
            format!("{:04b}", c.to_digit(16).unwrap_or(0))
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packet_version() {
        let inputs = [
            ("110100101111111000101000", 6),
            (
                "11101110000000001101010000001100100000100011000001100000",
                7,
            ),
            (
                "00111000000000000110111101000101001010010001001000000000",
                1,
            ),
        ];

        for (input, version) in inputs {
            let buf: Vec<u8> = input
                .chars()
                .map(|c| c.to_digit(10).unwrap_or(0) as u8)
                .collect();
            let (v, _) = decode_header(&buf);
            assert_eq!(v, version);
        }
    }

    #[test]
    fn packet_type() {
        let inputs = [
            ("110100101111111000101000", 4),
            (
                "11101110000000001101010000001100100000100011000001100000",
                3,
            ),
            (
                "00111000000000000110111101000101001010010001001000000000",
                6,
            ),
        ];

        for (input, kind) in inputs {
            let buf: Vec<u8> = input
                .chars()
                .map(|c| c.to_digit(10).unwrap_or(0) as u8)
                .collect();
            let (_, t) = decode_header(&buf);
            assert_eq!(t, kind);
        }
    }

    #[test]
    fn version_sum() {
        let inputs = [
            ("8A004A801A8002F478", 16),
            ("620080001611562C8802118E34", 12),
            ("C0015000016115A2E0802F182340", 23),
            ("A0016C880162017C3686B18A3D4780", 31),
        ];

        for (input, v) in inputs {
            let packet = Packet::from_hex(&input);
            println!("{:?}", input);
            println!("{:?}", packet);
            assert_eq!(packet.version_sum(), v);
        }
    }
}
