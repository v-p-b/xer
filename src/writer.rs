use std::io::Write;

use crate::util::*;

pub fn write_0x_hex(v: Vec<u8>, stream: &mut impl Write) {
    let mut str_vec = Vec::new();
    for b in v.iter() {
        str_vec.push(format!("0x{b:02x}"))
    }

    let _ = stream.write_all(str_vec.join(",").as_bytes());
}

pub fn write_signed_0x_hex(v: Vec<u8>, stream: &mut impl Write) {
    let mut str_vec = Vec::new();
    for b in v.iter() {
        if *b < 0x80 {
            str_vec.push(format!("0x{b:02x}"))
        } else {
            let twos = twos(*b);
            str_vec.push(format!("-0x{twos:02x}"))
        }
    }

    let _ = stream.write_all(str_vec.join(",").as_bytes());
}

pub fn write_signed_dec(v: Vec<u8>, stream: &mut impl Write) {
    let mut str_vec = Vec::new();
    for b in v.iter() {
        if *b < 0x80 {
            str_vec.push(format!("{b}"))
        } else {
            let twos = twos(*b);
            str_vec.push(format!("-{twos}"))
        }
    }

    let _ = stream.write_all(str_vec.join(",").as_bytes());
}

pub fn write_dec(v: Vec<u8>, stream: &mut impl Write) {
    let mut str_vec = Vec::new();
    for b in v.iter() {
        str_vec.push(format!("{b}"))
    }

    let _ = stream.write_all(str_vec.join(",").as_bytes());
}

pub fn write_esc_hex(v: Vec<u8>, stream: &mut impl Write) {
    let mut str_vec = Vec::new();
    for b in v.iter() {
        str_vec.push(format!("\\x{b:02x}"))
    }

    let _ = stream.write_all(str_vec.join("").as_bytes());
}

pub fn write_hex(v: Vec<u8>, stream: &mut impl Write) {
    let mut str_vec = Vec::new();
    for b in v.iter() {
        str_vec.push(format!("{b:02x}"))
    }
    let _ = stream.write_all(str_vec.join(" ").as_bytes());
}

pub fn write_bin(v: Vec<u8>, stream: &mut impl Write) {
    let mut str_vec = Vec::new();
    for b in v.iter() {
        str_vec.push(format!("0b{b:08b}"))
    }
    let _ = stream.write_all(str_vec.join(",").as_bytes());
}

pub fn write_raw(v: Vec<u8>, stream: &mut impl Write) {
    let _ = stream.write_all(&v);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writer_tests() {
        let mut out_vec = Vec::new();

        write_raw(vec![0x80u8], &mut out_vec);
        assert_eq!(out_vec.pop(), Some(0x80u8));
        out_vec.clear();

        write_hex(vec![0x80u8, 0x7fu8], &mut out_vec);
        assert_eq!(out_vec, "80 7f".as_bytes());
        out_vec.clear();

        write_esc_hex(vec![0x80u8, 0x7fu8], &mut out_vec);
        assert_eq!(out_vec, "\\x80\\x7f".as_bytes());
        out_vec.clear();

        write_bin(vec![0x80u8, 0x7fu8], &mut out_vec);
        assert_eq!(out_vec, "0b10000000,0b01111111".as_bytes());
        out_vec.clear();

        write_0x_hex(vec![0x80u8, 0x7fu8], &mut out_vec);
        assert_eq!(out_vec, "0x80,0x7f".as_bytes());
        out_vec.clear();

        write_signed_0x_hex(vec![0x80u8, 0x7fu8, 0xff], &mut out_vec);
        assert_eq!(out_vec, "-0x80,0x7f,-0x01".as_bytes());
        out_vec.clear();

        write_dec(vec![0x80u8, 0x7fu8], &mut out_vec);
        assert_eq!(out_vec, "128,127".as_bytes());
        out_vec.clear();

        write_signed_dec(vec![0x80u8, 0x7fu8, 0xff], &mut out_vec);
        assert_eq!(out_vec, "-128,127,-1".as_bytes());
        out_vec.clear();
    }
}
