use std::io::Write;

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
        }else{
            let mut twos = !b + 1;
            twos &= !(1<<7);
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
        }else{
            let mut twos = !b + 1;
            twos &= !(1<<7);
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
