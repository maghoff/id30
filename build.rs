use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

const ALT_FLAG: u8 = 1 << 5;
const ERR_FLAG: u8 = 1 << 6;

static CODE: [&[u8]; 32] = [
    &[b'0', b'o', b'O'],
    &[b'1', b'i', b'I', b'l', b'L'],
    &[b'2'],
    &[b'3'],
    &[b'4'],
    &[b'5'],
    &[b'6'],
    &[b'7'],
    &[b'8'],
    &[b'9'],
    &[b'a', b'A'],
    &[b'b', b'B'],
    &[b'c', b'C'],
    &[b'd', b'D'],
    &[b'e', b'E'],
    &[b'f', b'F'],
    &[b'g', b'G'],
    &[b'h', b'H'],
    &[b'j', b'J'],
    &[b'k', b'K'],
    &[b'm', b'M'],
    &[b'n', b'N'],
    &[b'p', b'P'],
    &[b'q', b'Q'],
    &[b'r', b'R'],
    &[b's', b'S'],
    &[b't', b'T'],
    &[b'v', b'V'],
    &[b'w', b'W'],
    &[b'x', b'X'],
    &[b'y', b'Y'],
    &[b'z', b'Z'],
];

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("codec_tables.rs");

    let encode_table: Vec<_> = CODE.iter().map(|encodings| encodings[0]).collect();

    let mut decode_table = [ERR_FLAG; 256];
    for (symbol, encoded) in CODE.iter().enumerate() {
        for (index, encoded) in encoded.iter().enumerate() {
            let flags = if index == 0 { 0 } else { ALT_FLAG };
            decode_table[*encoded as usize] = symbol as u8 | flags;
        }
    }

    let mut out = fs::File::create(dest_path).unwrap();

    write!(&mut out, "pub(crate) const ENCODE: [u8; 32] = *b\"").unwrap();
    for x in &encode_table {
        write!(&mut out, "{}", char::from(*x)).unwrap();
    }
    writeln!(&mut out, "\";\n").unwrap();

    writeln!(&mut out, "pub(crate) const DECODE: [u8; 256] = [").unwrap();
    for x in decode_table {
        if x == ERR_FLAG {
            write!(&mut out, "    ERR_FLAG").unwrap();
        } else {
            write!(&mut out, "    {}", x & 0b1_1111).unwrap();
            if x & ALT_FLAG != 0 {
                write!(&mut out, " | ALT_FLAG").unwrap();
            }
        }
        writeln!(&mut out, ",").unwrap();
    }
    writeln!(&mut out, "];").unwrap();

    writeln!(&mut out, "pub(crate) const DECODE_LOW: [u8; 64] = [").unwrap();
    for &x in &decode_table[0..64] {
        if x == ERR_FLAG {
            write!(&mut out, "    ERR_FLAG").unwrap();
        } else {
            write!(&mut out, "    {}", x & 0b1_1111).unwrap();
            if x & ALT_FLAG != 0 {
                write!(&mut out, " | ALT_FLAG").unwrap();
            }
        }
        writeln!(&mut out, ",").unwrap();
    }
    writeln!(&mut out, "];").unwrap();

    writeln!(&mut out, "pub(crate) const DECODE_HIGH: [u8; 64] = [").unwrap();
    for &x in &decode_table[64..128] {
        if x == ERR_FLAG {
            write!(&mut out, "    ERR_FLAG").unwrap();
        } else {
            write!(&mut out, "    {}", x & 0b1_1111).unwrap();
            if x & ALT_FLAG != 0 {
                write!(&mut out, " | ALT_FLAG").unwrap();
            }
        }
        writeln!(&mut out, ",").unwrap();
    }
    writeln!(&mut out, "];").unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}
