use std::io::{Read, Cursor};
use std::error::Error;
use fmt_extra::{AsciiStr, Hs};
use std::env;
use std::fmt;

fn uint<V: AsRef<[u8]>>(a: V) -> u64 {
    let mut b = a.as_ref();
    let mut n = b.len();
    match n {
        8|4|2|1 => {},
        _ => panic!("bad uint len: {}", n),
    }

    let mut v = 0;
    if n == 8 {
        v = (b[0] as u64) << 56 | (b[1] as u64) << 48 | (b[2] as u64) << 40 | (b[3] as u64) << 32;
        b = &b[4..];
        n = 4;
    }
    if n == 4 {
        v |= ((b[0] as u32) << 24 | (b[1] as u32) << 16) as u64;
        b = &b[2..];
        n = 2;
    }
    if n == 2 {
        v |= ((b[0] as u16) << 8) as u64;
        b = &b[1..];
        // n = 1
    }

    // n = 1
    v |= b[0] as u64;

    v
}

fn take<R: Read>(r: &mut R, n: usize) -> Vec<u8> {
    let mut b = Vec::with_capacity(n);
    unsafe { b.set_len(n) };

    let c = r.read(&mut b).expect("read error");
    if c != n {
        panic!("need {} bytes, got {}", n, c);
    }

    b
}

fn pd(depth: usize) {
    print!("{:1$}", "", depth * 2);
}

struct Mt(u8);

impl fmt::Display for Mt {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            0 => write!(fmt, "uint"),
            1 => write!(fmt, "nint"),
            2 => write!(fmt, "bytes"),
            3 => write!(fmt, "text"),
            4 => write!(fmt, "array"),
            5 => write!(fmt, "map"),
            6 => write!(fmt, "tag"),
            7 => write!(fmt, "misc/float"),
            _ => panic!(),
        }
    }
}

fn well_formed<R: Read>(r: &mut R, depth: usize, breakable: bool) -> Result<Option<u8>, Box<dyn Error>> {
    // process initial bytes
    let ib = uint(take(r, 1)) as u8;
    let mt = ib >> 5;
    let ai = ib & 0x1f;
    let mut val = ai as u64;
    pd(depth);
    println!("{:02x} # mt={}, ai={}", ib, Mt(mt), ai);
    match ai {
        24 => {
            let v = take(r, 1);
            val = uint(&v);
            pd(depth + 1);
            println!("{} # val={}", Hs(&v), val);
        },
        25 => {
            let v = take(r, 2);
            val = uint(&v);
            pd(depth + 1);
            println!("{} # val={}", Hs(&v), val);
        },
        26 => {
            let v = take(r, 4);
            val = uint(&v);
            pd(depth + 1);
            println!("{} # val={}", Hs(&v), val);
        },
        27 => {
            let v = take(r, 8);
            val = uint(&v);
            pd(depth + 1);
            println!("{} # val={}", Hs(&v), val);
        },
        28|29|30 => Err(format!("bad additional info: {}", ai))?,
        31 => return well_formed_indefinite(r, depth + 1, mt, breakable),
        _ => {},
    }
    // process content
    match mt {
        // case 0, 1, 7 do not have content; just use val
        2|3 => {
            // bytes/UTF-8
            let v = take(r, val as usize);
            pd(depth + 1);
            if mt == 3 {
                println!("{} # s={}", Hs(&v), AsciiStr(&v));
            } else {
                println!("{}", Hs(&v));
            }
        },
        // array
        4 => {
            for _ in 0..val {
                well_formed(r, depth + 1, false).unwrap();
            }
        },
        // map
        5 => {
            for _ in 0..(val * 2) {
                well_formed(r, depth + 1, false).unwrap();
            }
        },
        // tag
        6 => {
            // 1 embedded data item
            well_formed(r, depth + 1, false).unwrap();
        },

        _ => {
        },
    }

    // finite data item
    Ok(Some(mt))
}

fn well_formed_indefinite<R: Read>(r: &mut R, depth: usize, mt: u8, breakable: bool) -> Result<Option<u8>, Box<dyn Error>> {
    match mt {
        2|3 => {
            loop {
                match well_formed(r, depth, true)? {
                    None => break,
                    Some(it) => {
                        // need finite embedded
                        //    of same type
                        if it != mt {
                            panic!();
                        }
                    },
                }
            }
        },
        // array
        4 => {
            loop {
                if let None = well_formed(r, depth, true)? {
                    break;
                }
            }
        },
        // map
        5 => {
            loop {
                pd(depth);
                println!("# K");
                let v = well_formed(r, depth, true)?;
                match v {
                    None => break,
                    Some(_) => {
                        pd(depth);
                        println!("# V");
                        well_formed(r, depth, false)?;
                    },
                }
            }
        },
        7 => {
            if breakable {
                return Ok(None);
            } else {
                Err("found break in non-breakable")?;
            }
        }
        // wrong mt
        _ => panic!(),
    }

    // no break out
    Ok(Some(0))
}

fn main() {
    for arg in env::args().skip(1) {
        println!("{}", arg);
        let h = hex::decode(arg).unwrap();
        let mut c = Cursor::new(h);
        well_formed(&mut c, 1, false).unwrap();
    }
}
