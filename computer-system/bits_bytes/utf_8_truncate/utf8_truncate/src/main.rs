use std::io::{self, Read, Write};
fn main() {
    // cd utf8_truncate && cargo build -q && cd .. && diff expected.txt <(cat cases | ./utf8_truncate/target/debug/utf8_truncate)
    let mut read_buffer = Vec::new();
    let mut write_buffer = Vec::new();

    io::stdin()
        .read_to_end(&mut read_buffer)
        .expect("Failed to read from stdin");

    for line in read_buffer.split(|b| *b == 0x0A) {
        if line.is_empty() {
            continue;
        }
        let mut n = line[0] as usize;
        write_buffer.extend_from_slice(truncate(&line[1..], &mut n));
        write_buffer.push(0x0A);
    }
    io::stdout()
        .write_all(&write_buffer)
        .expect("Failed to write to stdout");
}

fn truncate<'a>(bytes: &'a [u8], num: &mut usize) -> &'a [u8] {
    if *num >= bytes.len() {
        bytes
    } else {
        // If byte at truncation point starts with 10 specifically then we move pointer down
        // Finding if byte starts with 10
        //     Mask out the first two bits - by bitwise and on 0b11000000 (or 0xC0)
        //     Check if they are equivalent to 0b10000000 (or 0x80)
        // This works b/c only continuation bytes will have 0xc0
        while *num > 0 && ((bytes[*num] & 0xc0) == 0x80) {
            *num -= 1;
        }
        &bytes[..*num]
    }
}
