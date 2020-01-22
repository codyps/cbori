// examine `in` and determine details about 
fn cbor_len(b: &[u8]) -> usize {

}

fn load_n(mut b: &[u8], mut n: u8) -> Result<(u64, &[u8]), &[u8]> {
    match n {
        8 | 4 | 2 | 1 => {
            // consider optization of this check in optimizing this function.
            if b.len() < n {
                return Err(b)
            }
        },
        _ => panic!(),
    }

    let mut r = 0;
    if n == 8 {
        // TODO: consider the order of conditionals (largest first) and it's effect on perf & code
        // size. in particular, consider the constants being encoded depending on the shifts
        // selected (or lack of constants)
        r = b[0] as u64 << 56 | b[1] as u64 << 48 | b[2] as u64 << 40 | b[3] as u64 << 32;
        b = &b[4..];
        // TODO: consider use of a `fallthrough` bool and it's effect on perf
        n = 4;
    }
    if n == 4 {
        r |= b[0] as u64 << 24 | b[1] as u64 << 16;
        b = &b[2..];
        n = 2;
    }
    if n == 2 {
        r |= b[0] as u64 << 8;
        b = &b[1..];
    }

    r |= b[0] as u64;
    b = &b[1..];

    Ok((r, b))
}

fn l_int(b: &[u8]) -> usize {
    unimplemented!()
}

fn l_int1(b: &[u8]) -> usize {
    unimplemented!()
}

fn l_int2(b: &[u8]) -> usize {
    unimplemented!()
}

fn l_int4(b: &[u8]) -> usize {
    unimplemented!()
}

fn l_int8(b: &[u8]) -> usize {
    unimplemented!()
}

// TODO: consider allowing segmented args
// TODO: consider allowing incrimental parsing?
pub fn cbor(b: &[u8]) -> Value {
    let f = b.take(1)?;

    let l = match f {
        // unsigned int
        0x00 ..= 0x17 => {
            println!("{:<1$}{:02x}{: pos-int{{{}}}", "", i * 2, f, f)
        },
        0x18 => l_int1,
        0x19 => l_int2,
        0x1a => l_int4,
        0x1b => l_int8,
        // neg int
        0x20 ..= 0x37 => l_int,
        0x38 => l_int1,
        0x39 => l_int2,
        0x3a => l_int4,
        0x3b => l_int8,
        // byte string
        0x40 ..= 0x57 => l_bytes,
        0x58 => l_bytes1,
        0x59 => l_bytes2,
        0x5a => l_bytes4,
        0x5b => l_bytes8,
        // multiple byte strings with terminator
        0x5f => unimplemented!(),
        // utf-8
        0x60 ..= 0x77 => l_bytes,
        0x78 => l_bytes1,
        0x79 => l_bytes2,
        0x7a => l_bytes4,
        0x7b => l_bytes8,
        // array
        0x80 ..= 0x97 => l_array,
        0x98 => l_array1,
        0x99 => l_array2,
        0x9a => l_array4,
        0x9b => l_array8,
        // array with terminator
        0x9f => l_array_ul,
        // map
        0xa0 ..= 0xb7 => l_array,
        0xb8 => l_array1,
        0xb9 => l_array2,
        0xba => l_array4,
        0xbb => l_array8,
        0xbf => l_array_ul,

        0xff => term,
    };
}
